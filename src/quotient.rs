/// The result of a division
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Quotient<T> {
    /// Result from division by 0
    Nan,
    /// Numeric value of the quotient
    Number(T),
}
