//! Error handling.
//!
//! This crate uses a [`failure`]-based error handling scheme. Errors generated by this library use
//! the [`Error`] structure.
//!
//! [`failure`]: https://docs.rs/failure/0.1/failure/
//! [`Error`]: ./struct.Error.html

use derive_more::{Display, From};
use failure::{Backtrace, Context, Fail};

/// A specialized [`std::result::Result`] type for operations originating from within the library.
///
/// This type is generally used to avoid writing out [`bsa::Error`] directly and is otherwise a
/// direct mapping to [`std::result::Result`].
///
/// [`std::result::Result`]: https://doc.rust-lang.org/std/result/enum.Result.html
/// [`bsa::Error`]: ./struct.Error.html
pub type Result<T> = std::result::Result<T, Error>;

/// The error type for operations originating from within the library.
#[derive(Debug, Display)]
pub struct Error(Context<Kind>);

impl Fail for Error {
    fn cause(&self) -> Option<&dyn Fail> {
        self.0.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.0.backtrace()
    }
}

impl<T> From<T> for Error
where
    T: Into<Kind>,
{
    fn from(kind: T) -> Self {
        Self(Context::new(kind.into()))
    }
}

/// A list specifying general categories of errors, used by the [`Error`] type.
///
/// This list is intended to grow over time and it is not recommended to exhaustively match against
/// it.
///
/// [`Error`]: ./struct.Error.html
#[derive(Debug, Display, Fail, From)]
pub enum Kind {
    #[doc(hidden)]
    __Nonexhaustive,
}
