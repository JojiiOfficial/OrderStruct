use std::{cmp::Ordering, marker::PhantomData};

/// A struct to order values by a custom key function `F`
#[derive(Debug)]
pub struct OrderByKey<V, F, O: Ord> {
    val: V,
    key_fn: F,
    pd: PhantomData<O>,
}

impl<V, F, O: Ord> OrderByKey<V, F, O>
where
    F: Fn(&V) -> O,
{
    /// Create a new OrderBy
    #[inline]
    pub fn new(val: V, key_fn: F) -> Self {
        Self {
            val,
            key_fn,
            pd: PhantomData,
        }
    }

    /// Convert back into `V`
    #[inline]
    pub fn into_inner(self) -> V {
        self.val
    }
}

impl<V, F, O: Ord> PartialEq for OrderByKey<V, F, O>
where
    F: Fn(&V) -> O,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(&other)
            .map(|i| i == Ordering::Equal)
            .unwrap_or(false)
    }
}

impl<V, F, O: Ord> Eq for OrderByKey<V, F, O>
where
    F: Fn(&V) -> O,
{
    fn assert_receiver_is_total_eq(&self) {}
}

impl<V, F, O: Ord> PartialOrd for OrderByKey<V, F, O>
where
    F: Fn(&V) -> O,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let self_k = (self.key_fn)(&self.val);
        let other_k = (other.key_fn)(&other.val);
        Some(self_k.cmp(&other_k))
    }
}

impl<V, F, O: Ord> Ord for OrderByKey<V, F, O>
where
    F: Fn(&V) -> O,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl<V: Clone, F, O: Ord + Clone> Clone for OrderByKey<V, F, O>
where
    F: Fn(&V) -> O + Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            val: self.val.clone(),
            pd: self.pd.clone(),
            key_fn: self.key_fn.clone(),
        }
    }
}

impl<V: Copy, F, O: Ord + Copy> Copy for OrderByKey<V, F, O> where F: Fn(&V) -> O + Copy {}
