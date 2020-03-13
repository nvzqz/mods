//! Simpler module declaration, brought to you by [@NikolaiVazquez]!
//!
//! This library enables you to declare modules in ways the current syntax
//! doesn't allow.
//!
//! # Examples
//!
//! To declare multiple public modules, simply place `pub` before a module list:
//!
//! ```rust,ignore
//! mods::mods! {
//!     pub puppy, kitty;
//! }
//! ```
//!
//! This works for all visibility modifiers:
//!
//! ```rust,ignore
//! mods::mods! {
//!     pub a, b;        // Visible anywhere, even outside the module
//!     pub(crate) c, d; // Visible anywhere within the crate
//!     pub(super) e, f; // Visible to the parent module
//!     g, h;            // Visible to the current module
//! }
//! ```
//!
//! Without the `mods!` macro, the same code is much less succinct. This is what
//! the macro expands out to:
//!
//! ```rust,ignore
//! pub mod a;
//! pub mod b;
//! pub(crate) mod c;
//! pub(crate) mod d;
//! pub(super) mod e;
//! pub(super) mod f;
//! mod g;
//! mod h;
//! ```
//!
//! # Changes
//!
//! See [`CHANGELOG.md`] for an exhaustive list of what has changed from one
//! version to another.
//!
//! # License
//!
//! This project is released under either:
//!
//! - [MIT License](https://github.com/nvzqz/mods/blob/master/LICENSE-MIT)
//! - [Apache License (Version 2.0)](https://github.com/nvzqz/mods/blob/master/LICENSE-APACHE)
//!
//! # Donate
//!
//! This project is made freely available (as in free beer), but unfortunately
//! not all beer is free! So, if you would like to buy me a beer (or coffee or
//! *more*), then consider supporting my work that's benefited your project
//! and thousands of others.
//!
//! <a href="https://www.patreon.com/nvzqz">
//!     <img src="https://c5.patreon.com/external/logo/become_a_patron_button.png" alt="Become a Patron!" height="35">
//! </a>
//! <a href="https://www.paypal.me/nvzqz">
//!     <img src="https://buymecoffee.intm.org/img/button-paypal-white.png" alt="Buy me a coffee" height="35">
//! </a>
//!
//! [@NikolaiVazquez]: https://twitter.com/NikolaiVazquez
//!
//! [`CHANGELOG.md`]: https://github.com/nvzqz/mods/blob/master/CHANGELOG.md

#![no_std]
#![warn(missing_docs)]

/// Declares modules simply.
///
/// # Examples
///
/// To declare multiple public modules, simply place `pub` before a module list:
///
/// ```rust,ignore
/// mods::mods! {
///     pub puppy, kitty;
/// }
/// ```
///
/// This works for all visibility modifiers:
///
/// ```rust,ignore
/// mods::mods! {
///     pub a, b;        // Visible anywhere, even outside the module
///     pub(crate) c, d; // Visible anywhere within the crate
///     pub(super) e, f; // Visible to the parent module
///     g, h;            // Visible to the current module
/// }
/// ```
///
/// Without the `mods!` macro, the same code is much less succinct. This is what
/// the macro expands out to:
///
/// ```rust,ignore
/// pub mod a;
/// pub mod b;
/// pub(crate) mod c;
/// pub(crate) mod d;
/// pub(super) mod e;
/// pub(super) mod f;
/// mod g;
/// mod h;
/// ```
#[macro_export]
macro_rules! mods {
    // Trailing comma
    ($($vis:vis $($mod:ident,)+;)+) => {
        $($($vis mod $mod;)+)+
    };
    // No trailing comma
    ($($vis:vis $($mod:ident),+;)+) => {
        $($($vis mod $mod;)+)+
    };
}

#[cfg(test)]
mod tests;
