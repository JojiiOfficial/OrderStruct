use std::cmp::Ordering;

/// A struct to order values by a custom function
pub struct OrderBy<T, U> {
    val: T,
    cmp_fn: U,
}

impl<T, U> OrderBy<T, U>
where
    U: Fn(&T, &T) -> Ordering,
{
    /// Create a new OrderBy
    #[inline]
    pub fn new(val: T, cmp_fn: U) -> Self {
        Self { val, cmp_fn }
    }

    /// Convert back into `T`
    #[inline]
    pub fn into_inner(self) -> T {
        self.val
    }
}

impl<T, U> PartialEq for OrderBy<T, U>
where
    U: Fn(&T, &T) -> Ordering,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(&other)
            .map(|i| i == Ordering::Equal)
            .unwrap_or(false)
    }
}

impl<T, U> PartialOrd for OrderBy<T, U>
where
    U: Fn(&T, &T) -> Ordering,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.cmp_fn)(&self.val, &other.val))
    }
}

impl<T, U> Eq for OrderBy<T, U>
where
    U: Fn(&T, &T) -> Ordering,
{
    fn assert_receiver_is_total_eq(&self) {}
}

impl<T, U> Ord for OrderBy<T, U>
where
    U: Fn(&T, &T) -> Ordering,
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}
