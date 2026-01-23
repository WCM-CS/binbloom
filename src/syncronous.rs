use std::{sync::atomic::{AtomicPtr, AtomicUsize, Ordering}, thread::{self}};

pub struct AtomicBits {
    data: AtomicPtr<SnapShot>, // current up to date atomic copy
}

#[derive(Debug)]
pub struct SnapShot {
    data: Vec<u64>,  // Bitmap
    rc: AtomicUsize, // number of readers this snapshot has currently (green thread readers)
    mercury: thread::Thread,
}

impl Clone for SnapShot {
    fn clone(&self) -> Self {
        SnapShot {
            data: self.data.clone(),
            rc: AtomicUsize::new(0), // reset rc, thus need to ensure all threads rc of old snapshot is 0 before drop
            mercury: thread::current(),
        }
    }
}

impl AtomicBits {
    // Init
    pub fn new() -> Self {
        let snapshot = Box::new(SnapShot {
            data: vec![0],
            rc: AtomicUsize::new(0),
            mercury:thread::current(),
        });

        Self {
            data: AtomicPtr::new(Box::into_raw(snapshot)),
        }
    }

    // Reader
    pub fn read(&self, idx: usize) -> bool {
        let bucket = get_bucket(idx); // which integer slot/bucket we reside in for the given input index
        let offset = get_offset(idx); // what the actual offset is for the index within it's bucket

        // loop until we ensure snapshot validity
        let snapshot: &SnapShot = loop {
            // Get a ref to the current snapshot
            let ptr = self.data.load(Ordering::Acquire); // current instance, raw ptr

            // increment the rc
            let rc_ref: &AtomicUsize = unsafe { &(*ptr).rc };
            rc_ref.fetch_add(1, Ordering::AcqRel);

            if ptr == self.data.load(Ordering::Acquire) {
                break unsafe { &*ptr };
            } // return valid snapshot, under the single writer assumption this should only ever need to run once, and wont actually resolve certain UB derived from multi writer corruption, but can mitigate segfault risk if that does occur

            rc_ref.fetch_sub(1, Ordering::Release);
        };

        let result = if bucket >= snapshot.data.len() {
            false
        } else {
            snapshot.data[bucket] & (1 << offset) != 0
        }; // handle out of bounds inputs, else reads data

        if snapshot.rc.fetch_sub(1, Ordering::Release) == 1 {
            // decrement reader count before we return the result
            //snapshot.mercury.notify_one();
            snapshot.mercury.unpark();
        } // notify waiting threads that the final reader has concluded its work

        result
    }

    // Writers
    pub fn set(&self, idx: usize) {
        let bucket = get_bucket(idx);
        let offset = get_offset(idx);

        // Aquire atomic clone of snapshot
        let old_ptr: *mut SnapShot = self.data.load(Ordering::Acquire);
        debug_assert!(!old_ptr.is_null());
        //let mut snapshot_copy: SnapShot = unsafe { (*old_ptr).clone() };
        let snapshot: &SnapShot = unsafe { &*old_ptr };

        if bucket < snapshot.data.len() && (snapshot.data[bucket] & (1 << offset)) != 0 {
            return ();
        }

        let mut snapshot_copy: SnapShot = snapshot.clone();

        if bucket >= snapshot_copy.data.len() {
            snapshot_copy.data.resize(bucket + 1, 0);
        } // capacity regulator

        // set bit
        snapshot_copy.data[bucket] |= 1 << offset;

        // swap snapshot ptr
        let new_ptr = Box::into_raw(Box::new(snapshot_copy));
        let old_ptr = self.data.swap(new_ptr, Ordering::AcqRel);
        //let old_snapshot = unsafe { &*old_ptr };

        while unsafe { &*old_ptr }.rc.load(Ordering::Acquire) != 0 {
            //unsafe { &*old_ptr }.mercury.notified().await;
            thread::park();
        } // once last thread notifies the vallet this work continues

        /*
        while unsafe {&*old_ptr}.rc.load(Ordering::Acquire) > 0 {
            std::thread::yield_now();
        } // Listen for active reader of current/old to hit zero, yeilding thread to allow for multitasking in the meantime


         */

        // can this starve aka writes starve from heavy red load?

        unsafe {
            let old_data = Box::from_raw(old_ptr);
            drop(old_data);
        }
    }

    pub fn clear(&self, idx: usize) {
        let bucket = get_bucket(idx);
        let offset = get_offset(idx);

        // Aquire atomic clone of snapshot
        let old_ptr: *mut SnapShot = self.data.load(Ordering::Acquire);
        debug_assert!(!old_ptr.is_null());

        let snapshot: &SnapShot = unsafe { &*old_ptr };

        if bucket >= snapshot.data.len() || (snapshot.data[bucket] & (1 << offset)) == 0 {
            return ();
        } // check bounds and current state before mutating from 1 to zero

        // set bit
        let mut snapshot_copy: SnapShot = snapshot.clone();
        snapshot_copy.data[bucket] &= !(1 << offset);

        {
            while let Some(bucket) = snapshot_copy.data.last() {
                if *bucket != 0 {
                    break;
                }

                snapshot_copy.data.pop();
            }
        } // reclamation

        // swap snapshot ptr
        let new_ptr = Box::into_raw(Box::new(snapshot_copy));
        let old_ptr = self.data.swap(new_ptr, Ordering::AcqRel);
        //let old_snapshot = unsafe { &*old_ptr };

        /*
        while unsafe { &*old_ptr }.rc.load(Ordering::Acquire) > 0 {
                    std::thread::yield_now();
                } // wait for thread safe guarentee
         */
        while unsafe { &*old_ptr }.rc.load(Ordering::Acquire) != 0 {
            //unsafe { &*old_ptr }.mercury.notified().await;
            thread::park();
        } // once last thread notifies the vallet this work continues

        unsafe {
            let old_data = Box::from_raw(old_ptr);
            drop(old_data);
        }
    }

    pub fn len(&self) -> usize {
        let snap_ptr = self.data.load(Ordering::Relaxed);
        let snapshot: &SnapShot = unsafe { &*snap_ptr };

        snapshot.data.len()
    }
}

fn get_bucket(idx: usize) -> usize {
    idx / 64
}

fn get_offset(idx: usize) -> usize {
    idx & 63 // equivalent of idx % 64 // bit op, only works when denominator is an integer power of two
}
