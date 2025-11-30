/// The result of a division
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Quotient<T>
where
    T: Copy + Clone,
{
    /// Result from division by 0
    Nan,
    /// Numeric value of the quotient
    Number(T),
}

impl<T> Quotient<T>
where
    T: Copy + Clone,
{
    /// Map a `Quotient<T>` to `Quotient<U>` by applying a function to a
    /// contained value.
    pub fn map<U, F>(self, f: F) -> Quotient<U>
    where
        F: FnOnce(T) -> U,
        U: Copy + Clone,
    {
        match self {
            Self::Nan => Quotient::Nan,
            Self::Number(n) => Quotient::Number(f(n)),
        }
    }

    /// Convert from `Quotient<T>` to `Option<T>`.
    pub const fn number(self) -> Option<T> {
        let Self::Number(number) = self else {
            return None;
        };

        Some(number)
    }

    /// Convert from `Quotient<T>` to `Option<T>`.
    pub const fn number_mut(&mut self) -> Option<&mut T> {
        let Self::Number(number) = self else {
            return None;
        };

        Some(number)
    }

    /// Return true if the quotient is a [`Number`](Self::Number) value.
    pub const fn is_number(self) -> bool {
        matches!(self, Self::Number(_))
    }

    /// Return true if the quotient is a [`Nan`](Self::Nan) value.
    pub const fn is_nan(self) -> bool {
        matches!(self, Self::Nan)
    }

    /// Transform the `Quotient<T>` into a [`Result<T, E>`], mapping `Number(v)`
    /// to `Ok(v)` and `Nan` to `Err(err)`.
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        self.number().ok_or(err)
    }

    /// Transform the `Quotient<T>` into a [`Result<T, E>`], mapping `Number(v)`
    /// to `Ok(v)` and `Nan` to `Err(err)`.
    pub fn ok_or_else<E, F>(self, f: F) -> Result<T, E>
    where
        F: FnOnce() -> E,
    {
        self.number().ok_or_else(f)
    }

    /// Return the number if not [`Nan`](Self::Nan), otherwise returns `other`.
    pub fn or(self, other: Self) -> Self {
        let Some(result) = self.number().or(other.number()) else {
            return Quotient::Nan;
        };

        Quotient::Number(result)
    }

    /// Return the number if not [`Nan`](Self::Nan), otherwise call `f` and
    /// return the result.
    pub fn or_else<F>(self, f: F) -> Self
    where
        F: FnOnce() -> Self,
    {
        let Some(number) = self.number().or_else(|| f().number()) else {
            return Quotient::Nan;
        };

        Quotient::Number(number)
    }

    /// Return [`Nan`](Self::Nan) if `self` is [`Nan`](Self::Nan), otherwise
    /// return `other`.
    pub fn and<U>(self, other: Quotient<U>) -> Quotient<U>
    where
        U: Copy + Clone,
    {
        let Some(result) = self.number().and(other.number()) else {
            return Quotient::Nan;
        };

        Quotient::Number(result)
    }

    /// Return [`Nan`](Self::Nan) if `self` is [`Nan`](Self::Nan), otherwise
    /// call `f` with the number and return the result.
    pub fn and_then<U, F>(self, f: F) -> Quotient<U>
    where
        F: FnOnce(T) -> Quotient<U>,
        U: Copy + Clone,
    {
        let Some(number) = self.number().and_then(|n| f(n).number()) else {
            return Quotient::Nan;
        };

        Quotient::Number(number)
    }

    /// Return [`Number`](Self::Number) if exactly one of `self` and `other` are
    /// [`Number`](Self::Number), otherwise return [`Nan`](Self::Nan).
    pub fn xor(self, other: Self) -> Self {
        let Some(result) = self.number().xor(other.number()) else {
            return Quotient::Nan;
        };

        Quotient::Number(result)
    }
}
