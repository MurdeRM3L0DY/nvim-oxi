use std::marker::PhantomData;
use std::mem::ManuallyDrop;

/// A non-owning value for lifetime `'a`.
///
/// Used for FFI functions that accept data by value, but don't destroy or move
/// out of it. This is guaranteed to have the same layout as `T`.
#[repr(transparent)]
pub struct NonOwning<'a, T> {
    inner: ManuallyDrop<T>,
    lt: PhantomData<&'a ()>,
}

impl<'a, T> NonOwning<'a, T> {
    pub const fn new(value: T) -> Self {
        Self { inner: ManuallyDrop::new(value), lt: PhantomData }
    }
}

impl<'a, T> core::fmt::Debug for NonOwning<'a, T>
where
    T: core::fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.inner.fmt(f)
    }
}

impl<'a, T> Default for NonOwning<'a, T>
where
    T: Default,
{
    #[inline]
    fn default() -> Self {
        Self { inner: ManuallyDrop::new(T::default()), lt: PhantomData }
    }
}
