#![allow(missing_docs)]

mods! {
    pub puppy, kitty;
    a, b, c;
}

#[cfg(not(no_pub_special))]
mods! {
    pub(crate) x;
    pub(in super::tests) y;
}

mods! {}

// Check that the module is actually declared.
pub use self::puppy::Dog;
