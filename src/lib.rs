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
//! # Installation
//!
//! This crate is available [on crates.io][crate] and can be used by adding the
//! following to your project's [`Cargo.toml`]:
//!
//! ```toml
//! [dependencies]
//! mods = "1.0.0"
//! ```
//!
//! ## Minimum Supported Rust Version (MSRV)
//!
//! This library requires Rust 1.9.0 as the minimum version and will work with
//! all subsequent versions.
//!
//! This is because  previous versions can't have `mod x;` declarations within
//! submodules [When testing this](https://github.com/nvzqz/mods/runs/508242550)
//! we get an error complaining that the module file is not inside the directory
//! "src".
//!
//! ## Rust 2015
//!
//! If you're not using [Rust 2018], add this to your crate root (`main.rs` or
//! `lib.rs`):
//!
//! ```rust,ignore
//! #[macro_use]
//! extern crate mods;
//! ```
//!
//! You can then use the macro directly from anywhere:
//!
//! ```rust,ignore
//! mods! {
//!     pub puppy, kitty;
//! }
//! ```
//!
//! # Wishful Thinking
//!
//! It would be wonderful if we could instead have:
//!
//! ```rust,ignore
//! pub mod puppy, kitty;
//! ```
//!
//! Or a syntax that matches `use` imports:
//!
//! ```rust,ignore
//! pub mod {puppy, kitty};
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
//! [`Cargo.toml`]:    https://doc.rust-lang.org/cargo/reference/manifest.html
//! [`CHANGELOG.md`]:  https://github.com/nvzqz/mods/blob/master/CHANGELOG.md
//! [crate]:           https://crates.io/crates/mods
//! [Rust 2018]:       https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html#rust-2018

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
///
/// # Wishful Thinking
///
/// It would be wonderful if we could instead have:
///
/// ```rust,ignore
/// pub mod puppy, kitty;
/// ```
///
/// Or a syntax that matches `use` imports:
///
/// ```rust,ignore
/// pub mod {puppy, kitty};
/// ```
#[macro_export]
macro_rules! mods {
    () => {};

    // Trailing comma
    (pub $($module:ident,)+; $($rest:tt)*) => {
        $(pub mod $module;)+
        mods! { $($rest)* }
    };
    (pub($vis:ident) $($module:ident,)+; $($rest:tt)*) => {
        $(pub($vis) mod $module;)+
        mods! { $($rest)* }
    };
    (pub(in $vis:path) $($module:ident,)+; $($rest:tt)*) => {
        $(pub(in $vis) mod $module;)+
        mods! { $($rest)* }
    };
    ($($module:ident,)+; $($rest:tt)*) => {
        $(mod $module;)+
        mods! { $($rest)* }
    };

    // No trailing comma
    (pub $($module:ident),+; $($rest:tt)*) => {
        $(pub mod $module;)+
        mods! { $($rest)* }
    };
    (pub($vis:ident) $($module:ident),+; $($rest:tt)*) => {
        $(pub($vis) mod $module;)+
        mods! { $($rest)* }
    };
    (pub(in $vis:path) $($module:ident),+; $($rest:tt)*) => {
        $(pub(in $vis) mod $module;)+
        mods! { $($rest)* }
    };
    ($($module:ident),+; $($rest:tt)*) => {
        $(mod $module;)+
        mods! { $($rest)* }
    };
}

#[cfg(test)]
mod tests;
