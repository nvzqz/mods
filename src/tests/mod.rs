#![allow(missing_docs)]

mods! {
    pub puppy, kitty;
    pub(crate) x;
    pub(in super::tests) y;
    a, b, c;
}

// Check that the module is actually declared.
pub use self::puppy::Dog;
