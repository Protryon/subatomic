use core::{
    marker::PhantomData,
    mem::transmute_copy,
    sync::atomic::{AtomicU64, Ordering},
};

/// Wraps an 8-byte length T in an atomic, all operations are Ordering::SeqCst.
pub struct Subatomic64<T: Copy + 'static> {
    inner: AtomicU64,
    _phantom: PhantomData<T>,
}

impl<T: Copy + 'static> Subatomic64<T> {
    /// Create a new atomic wrapper
    pub fn new(item: T) -> Self {
        assert!(core::mem::size_of::<T>() == 8);
        Self {
            inner: AtomicU64::new(unsafe { transmute_copy(&item) }),
            _phantom: PhantomData,
        }
    }

    /// Update the interior value of the atomic wrapper
    pub fn store(&self, item: T) {
        self.inner
            .store(unsafe { transmute_copy(&item) }, Ordering::SeqCst);
    }

    /// Swap the interior value of the atomic wrapper, returning the previous value
    pub fn swap(&self, item: T) -> T {
        let out = self
            .inner
            .swap(unsafe { transmute_copy(&item) }, Ordering::SeqCst);
        unsafe { transmute_copy(&out) }
    }

    /// Loads the interior value of the atomic wrapper.
    pub fn load(&self) -> T {
        let out = self.inner.load(Ordering::SeqCst);
        unsafe { transmute_copy(&out) }
    }

    /// Stores a value into the atomic integer if the current value is the same as the current value.
    /// The return value is a result indicating whether the new value was written and containing the previous value.
    /// On success this value is guaranteed to be equal to current.
    pub fn compare_exchange(&self, current: T, new: T) -> Result<T, T> {
        self.inner
            .compare_exchange(
                unsafe { transmute_copy(&current) },
                unsafe { transmute_copy(&new) },
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .map(|x| unsafe { transmute_copy(&x) })
            .map_err(|x| unsafe { transmute_copy(&x) })
    }

    /// Stores a value into the atomic integer if the current value is the same as the current value.
    /// Unlike compare_exchange, this function is allowed to spuriously fail even when the comparison succeeds, which can result in more efficient code on some platforms.
    /// The return value is a result indicating whether the new value was written and containing the previous value.
    pub fn compare_exchange_weak(&self, current: T, new: T) -> Result<T, T> {
        self.inner
            .compare_exchange_weak(
                unsafe { transmute_copy(&current) },
                unsafe { transmute_copy(&new) },
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .map(|x| unsafe { transmute_copy(&x) })
            .map_err(|x| unsafe { transmute_copy(&x) })
    }
}
