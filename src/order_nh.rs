use std::{
    borrow::Borrow,
    cmp::Ordering,
    hash::Hash,
    ops::{AddAssign, DivAssign, MulAssign, SubAssign},
};

/// Custom struct to be able to compare items using another types `Ord` implementation
/// This implements Hash if T implements hash and only uses T for hashing
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

    /// Borrow inner value
    #[inline]
    pub fn inner(&self) -> &T {
        &self.val
    }

    /// Get assigned score
    #[inline]
    pub fn ord(&self) -> &O {
        &self.ord
    }

    /// Sets the order
    #[inline]
    pub fn set_ord(&mut self, ord: O) {
        self.ord = ord;
    }
}

impl<T, O: Ord> PartialOrd for OrderVal<T, O> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.ord.cmp(&other.ord))
    }
}

impl<T, O: Ord> Ord for OrderVal<T, O> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
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

impl<T: Clone, O: Ord + Clone> Clone for OrderVal<T, O> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            ord: self.ord.clone(),
            val: self.val.clone(),
        }
    }
}

impl<T: Copy, O: Ord + Copy> Copy for OrderVal<T, O> {}

impl<T, O: Ord> Borrow<T> for OrderVal<T, O> {
    #[inline]
    fn borrow(&self) -> &T {
        self.inner()
    }
}

impl<T, O: Ord> AsRef<T> for OrderVal<T, O> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.inner()
    }
}

impl<T, O: Ord + AddAssign<O>> AddAssign<O> for OrderVal<T, O> {
    #[inline]
    fn add_assign(&mut self, rhs: O) {
        self.ord.add_assign(rhs)
    }
}

impl<T, O: Ord + SubAssign<O>> SubAssign<O> for OrderVal<T, O> {
    #[inline]
    fn sub_assign(&mut self, rhs: O) {
        self.ord.sub_assign(rhs)
    }
}

impl<T, O: Ord + DivAssign<O>> DivAssign<O> for OrderVal<T, O> {
    #[inline]
    fn div_assign(&mut self, rhs: O) {
        self.ord.div_assign(rhs)
    }
}

impl<T, O: Ord + MulAssign<O>> MulAssign<O> for OrderVal<T, O> {
    #[inline]
    fn mul_assign(&mut self, rhs: O) {
        self.ord.mul_assign(rhs)
    }
}

impl<T: Hash, O: Hash + Ord> Hash for OrderVal<T, O> {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}
