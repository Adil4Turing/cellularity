// no imports needed

/// Trait defining neighborhood calculation strategies
pub trait Neighborhood: Send + Sync {
    /// Returns the relative offsets for neighbors
    ///
    /// # Returns
    /// A vector of (dx, dy) offsets relative to a cell
    fn offsets(&self) -> &[(isize, isize)];

    /// Returns the name of the neighborhood type
    fn name(&self) -> &str;
}