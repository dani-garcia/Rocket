#![recursion_limit="512"]

#![cfg_attr(nightly, feature(doc_cfg))]

#![warn(rust_2018_idioms)]
#![warn(missing_docs)]

//! Types that map to concepts in HTTP.
//!
//! This module exports types that map to HTTP concepts or to the underlying
//! HTTP library when needed. Because the underlying HTTP library is likely to
//! change (see [#17]), types in [`hyper`] should be considered unstable.
//!
//! [#17]: https://github.com/SergioBenitez/Rocket/issues/17

#[macro_use]
extern crate pear;

pub mod hyper;
pub mod uri;
pub mod ext;

#[macro_use]
mod docify;

#[doc(hidden)]
#[cfg(feature = "tls")]
pub mod tls;

#[macro_use]
mod header;
mod cookies;
mod method;
mod status;
mod raw_str;
mod parse;
mod listener;

/// Case-preserving, ASCII case-insensitive string types.
///
/// An _uncased_ string is case-preserving. That is, the string itself contains
/// cased characters, but comparison (including ordering, equality, and hashing)
/// is ASCII case-insensitive. **Note:** the `alloc` feature _is_ enabled.
pub mod uncased {
    #[doc(inline)] pub use uncased::*;
}

// Types that we expose for use by core.
#[doc(hidden)]
pub mod private {
    pub use crate::parse::Indexed;
    pub use smallvec::{SmallVec, Array};

    pub mod cookie {
        pub use cookie::*;
        pub use crate::cookies::Key;
    }

    pub use crate::listener::{Incoming, Listener, Connection, bind_tcp};
}

pub use crate::method::Method;
pub use crate::status::{Status, StatusClass};
pub use crate::raw_str::RawStr;
pub use crate::cookies::{Cookie, CookieJar, SameSite};
pub use crate::header::*;
