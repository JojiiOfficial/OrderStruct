use std::cmp::Ordering;

/// Custom struct to be able to compare items using another types `Ord` implementation
pub struct OrderVal<T, O: Ord> {
    ord: O,
    val: T,
}

impl<T, O: Ord> OrderVal<T, O> {
    /// Create a new OrderVal
    #[inline]
    pub fn new(val: T, ord: O) -> Self {
        Self { ord, val }
    }

    /// Convert back to `T`
    #[inline]
    pub fn into_inner(self) -> T {
        self.val
    }

    /// Get assigned score
    #[inline]
    pub fn ord(&self) -> &O {
        &self.ord
    }
}

impl<T, O: Ord> PartialOrd for OrderVal<T, O> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.ord.cmp(&other.ord))
    }
}

impl<T, O: Ord> Ord for OrderVal<T, O> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ord.cmp(&other.ord)
    }
}

impl<T, O: Ord> PartialEq for OrderVal<T, O> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ord.cmp(&other.ord) == Ordering::Equal
    }
}

impl<T, O: Ord> Eq for OrderVal<T, O> {}
