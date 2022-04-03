use std::cmp::Ordering;

/// A struct to order values by a custom function `F`
pub struct OrderBy<V, F> {
    val: V,
    cmp_fn: F,
}

impl<V, F> OrderBy<V, F>
where
    F: Fn(&V, &V) -> Ordering,
{
    /// Create a new OrderBy
    #[inline]
    pub fn new(val: V, cmp_fn: F) -> Self {
        Self { val, cmp_fn }
    }

    /// Convert back into `V`
    #[inline]
    pub fn into_inner(self) -> V {
        self.val
    }
}

impl<V, F> PartialEq for OrderBy<V, F>
where
    F: Fn(&V, &V) -> Ordering,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(&other)
            .map(|i| i == Ordering::Equal)
            .unwrap_or(false)
    }
}

impl<V, F> PartialOrd for OrderBy<V, F>
where
    F: Fn(&V, &V) -> Ordering,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.cmp_fn)(&self.val, &other.val))
    }
}

impl<V, F> Eq for OrderBy<V, F>
where
    F: Fn(&V, &V) -> Ordering,
{
    fn assert_receiver_is_total_eq(&self) {}
}

impl<V, F> Ord for OrderBy<V, F>
where
    F: Fn(&V, &V) -> Ordering,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl<V: Clone, F> Clone for OrderBy<V, F>
where
    F: Fn(&V, &V) -> Ordering + Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            val: self.val.clone(),
            cmp_fn: self.cmp_fn.clone(),
        }
    }
}

impl<V: Copy, F> Copy for OrderBy<V, F> where F: Fn(&V, &V) -> Ordering + Copy {}
