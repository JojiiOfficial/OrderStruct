use std::{cmp::Ordering, marker::PhantomData};

/// A struct to order values by a custom key function
pub struct OrderByKey<T, U, K: Ord> {
    val: T,
    key_fn: U,
    pd: PhantomData<K>,
}

impl<T, U, K: Ord> OrderByKey<T, U, K>
where
    U: Fn(&T) -> K,
{
    /// Create a new OrderBy
    #[inline]
    pub fn new(val: T, cmp_fn: U) -> Self {
        Self {
            val,
            key_fn: cmp_fn,
            pd: PhantomData,
        }
    }

    /// Convert back into `T`
    #[inline]
    pub fn into_inner(self) -> T {
        self.val
    }
}

impl<T, U, K: Ord> PartialEq for OrderByKey<T, U, K>
where
    U: Fn(&T) -> K,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(&other)
            .map(|i| i == Ordering::Equal)
            .unwrap_or(false)
    }
}

impl<T, U, K: Ord> Eq for OrderByKey<T, U, K>
where
    U: Fn(&T) -> K,
{
    fn assert_receiver_is_total_eq(&self) {}
}

impl<T, U, K: Ord> PartialOrd for OrderByKey<T, U, K>
where
    U: Fn(&T) -> K,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_k = (self.key_fn)(&self.val);
        let other_k = (other.key_fn)(&other.val);
        Some(self_k.cmp(&other_k))
    }
}

impl<T, U, K: Ord> Ord for OrderByKey<T, U, K>
where
    U: Fn(&T) -> K,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}
