//! ASCII Types
//!
//! This could be an alternate implementation for
//! <https://github.com/rust-lang/rust/issues/110998> using ranged integers.

use core::ascii::EscapeDefault;

use super::*;

/// ASCII uppercase character
pub type Uppercase = RangedU8<0x41, 0x5A>;

impl Uppercase {
    /// Convert to ASCII lowercase.
    ///
    /// ```rust
    /// # use ranch::ascii::{Lowercase, Uppercase};
    /// let uppercase_a = Uppercase::new::<65>();
    ///
    /// assert_eq!(Lowercase::new::<97>(), uppercase_a.to_ascii_lowercase());
    /// ```
    pub const fn to_ascii_lowercase(self) -> Lowercase {
        RangedU8(self.get().to_ascii_lowercase())
    }

    /// Convert to [`char`].
    ///
    /// ```rust
    /// # use ranch::ascii::Uppercase;
    /// let uppercase_a = Uppercase::new::<65>();
    ///
    /// assert_eq!(uppercase_a.to_char(), 'A');
    /// ```
    pub const fn to_char(self) -> char {
        let chr: Char = self.to_ranged_u8();

        chr.to_char()
    }
}

/// ASCII lowercase character
pub type Lowercase = RangedU8<0x61, 0x7A>;

impl Lowercase {
    /// Convert to ASCII uppercase.
    ///
    /// ```rust
    /// # use ranch::ascii::{Lowercase, Uppercase};
    /// let lowercase_a = Lowercase::new::<97>();
    ///
    /// assert_eq!(Uppercase::new::<65>(), lowercase_a.to_ascii_uppercase());
    /// ```
    pub const fn to_ascii_uppercase(self) -> Uppercase {
        RangedU8(self.get().to_ascii_uppercase())
    }

    /// Convert to [`char`].
    ///
    /// ```rust
    /// # use ranch::ascii::Lowercase;
    /// let lowercase_a = Lowercase::new::<97>();
    ///
    /// assert_eq!(lowercase_a.to_char(), 'a');
    /// ```
    pub const fn to_char(self) -> char {
        let chr: Char = self.to_ranged_u8();

        chr.to_char()
    }
}

/// ASCII graphic character
pub type Graphic = RangedU8<0x21, 0x7E>;

impl Graphic {
    /// Convert to ASCII uppercase.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-alphabetic
    /// letters are unchanged.
    ///
    /// ```rust
    /// # use ranch::ascii::Graphic;
    /// let ascii = Graphic::new::<b'a'>();
    /// let non_alphabetic = Graphic::new::<b'1'>();
    ///
    /// assert_eq!(Graphic::new::<b'A'>(), ascii.to_ascii_uppercase());
    /// assert_eq!(Graphic::new::<b'1'>(), non_alphabetic.to_ascii_uppercase());
    /// ```
    pub const fn to_ascii_uppercase(self) -> Self {
        Self(self.get().to_ascii_uppercase())
    }

    /// Convert to ASCII lowercase.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-alphabetic
    /// letters are unchanged.
    ///
    /// ```rust
    /// # use ranch::ascii::Graphic;
    /// let ascii = Graphic::new::<b'A'>();
    /// let non_alphabetic = Graphic::new::<b'1'>();
    ///
    /// assert_eq!(Graphic::new::<b'a'>(), ascii.to_ascii_lowercase());
    /// assert_eq!(Graphic::new::<b'1'>(), non_alphabetic.to_ascii_lowercase());
    /// ```
    pub const fn to_ascii_lowercase(self) -> Self {
        Self(self.get().to_ascii_lowercase())
    }

    /// Convert to [`char`].
    ///
    /// ```rust
    /// # use ranch::ascii::Graphic;
    /// let lowercase_a = Graphic::new::<97>();
    ///
    /// assert_eq!(lowercase_a.to_char(), 'a');
    /// ```
    pub const fn to_char(self) -> char {
        let chr: Char = self.to_ranged_u8();

        chr.to_char()
    }
}

/// ASCII digit character
pub type Digit = RangedU8<0x30, 0x39>;

impl Digit {
    /// Convert from numeric digit to ASCII digit.
    ///
    /// ```rust
    /// # use ranch::{RangedU8, ascii::Digit};
    /// assert_eq!(
    ///     Digit::from_digit(RangedU8::new::<5>()),
    ///     Digit::new::<b'5'>(),
    /// );
    /// ```
    pub const fn from_digit(digit: RangedU8<0, 9>) -> Self {
        digit.add::<0x30, 0x30, 0x39>()
    }

    /// Convert from ASCII digit to numeric digit.
    ///
    /// ```rust
    /// # use ranch::{RangedU8, ascii::Digit};
    /// assert_eq!(
    ///     Digit::new::<b'5'>().to_digit(),
    ///     RangedU8::new::<5>(),
    /// );
    /// ```
    pub const fn to_digit(self) -> RangedU8<0, 9> {
        self.sub::<0x30, 0, 9>()
    }

    /// Convert to [`char`].
    ///
    /// ```rust
    /// # use ranch::ascii::Digit;
    /// let one = Digit::new::<0x31>();
    ///
    /// assert_eq!(one.to_char(), '1');
    /// ```
    pub const fn to_char(self) -> char {
        let chr: Char = self.to_ranged_u8();

        chr.to_char()
    }
}

/// One of the 128 Unicode characters from U+0000 through U+007F, often known as
/// the ASCII subset.
pub type Char = bitwise::U7;

impl Char {
    /// Convert to ASCII uppercase.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-alphabetic
    /// letters are unchanged.
    ///
    /// ```rust
    /// # use ranch::ascii::Char;
    /// let ascii = Char::new::<b'a'>();
    /// let non_alphabetic = Char::new::<b'1'>();
    ///
    /// assert_eq!(Char::new::<b'A'>(), ascii.to_ascii_uppercase());
    /// assert_eq!(Char::new::<b'1'>(), non_alphabetic.to_ascii_uppercase());
    /// ```
    pub const fn to_ascii_uppercase(self) -> Self {
        Self(self.get().to_ascii_uppercase())
    }

    /// Convert to ASCII lowercase.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-alphabetic
    /// letters are unchanged.
    ///
    /// ```rust
    /// # use ranch::ascii::Char;
    /// let ascii = Char::new::<b'A'>();
    /// let non_alphabetic = Char::new::<b'1'>();
    ///
    /// assert_eq!(Char::new::<b'a'>(), ascii.to_ascii_lowercase());
    /// assert_eq!(Char::new::<b'1'>(), non_alphabetic.to_ascii_lowercase());
    /// ```
    pub const fn to_ascii_lowercase(self) -> Self {
        Self(self.get().to_ascii_lowercase())
    }

    /// Convert to [`char`].
    ///
    /// ```rust
    /// # use ranch::ascii::Char;
    /// let one = Char::new::<b'\n'>();
    ///
    /// assert_eq!(one.to_char(), '\n');
    /// ```
    pub const fn to_char(self) -> char {
        self.get() as char
    }
}

impl<const MIN: u8, const MAX: u8> RangedU8<MIN, MAX> {
    /// Return an iterator that produces an escaped version of a `u8`, treating
    /// it as an ASCII character.
    ///
    /// The behavior is identical to [`u8::escape_ascii()`].
    ///
    /// ```rust
    /// # use ranch::ascii::Char;
    /// assert_eq!("0", Char::new::<b'0'>().escape_ascii().to_string());
    /// assert_eq!("\\t", Char::new::<b'\t'>().escape_ascii().to_string());
    /// assert_eq!("\\r", Char::new::<b'\r'>().escape_ascii().to_string());
    /// assert_eq!("\\n", Char::new::<b'\n'>().escape_ascii().to_string());
    /// assert_eq!("\\'", Char::new::<b'\''>().escape_ascii().to_string());
    /// assert_eq!("\\\"", Char::new::<b'"'>().escape_ascii().to_string());
    /// assert_eq!("\\\\", Char::new::<b'\\'>().escape_ascii().to_string());
    /// ```
    ///
    /// Won't compile if out of ASCII range:
    ///
    /// ```rust,compile_fail
    /// assert_eq!("\\x9d", Char::new::<b'\x9d'>().escape_ascii().to_string());
    /// ```
    pub fn escape_ascii(self) -> EscapeDefault {
        self.get().escape_ascii()
    }
}
