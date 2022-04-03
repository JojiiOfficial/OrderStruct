use std::cmp::Ordering;

/// Custom struct to be able to compare items wich don't necessarily implement `Ord` themselves or are required
/// to be sorted by an u32 instead of the `Ord` implementation
pub struct OrderVal<T> {
    ord: u32,
    val: T,
}

impl<T: PartialEq + Eq> OrderVal<T> {
    /// Create a new OrderVal
    #[inline]
    pub fn new(val: T, ord: u32) -> Self {
        Self { ord, val }
    }

    /// Convert back to `T`
    #[inline]
    pub fn into_inner(self) -> T {
        self.val
    }

    /// Get assigned score
    #[inline]
    pub fn ord(&self) -> u32 {
        self.ord
    }
}

impl<T> PartialOrd for OrderVal<T> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.ord.cmp(&other.ord))
    }
}

impl<T> Ord for OrderVal<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ord.cmp(&other.ord)
    }
}

impl<T> PartialEq for OrderVal<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ord.cmp(&other.ord) == Ordering::Equal
    }
}
impl<T> Eq for OrderVal<T> {}
