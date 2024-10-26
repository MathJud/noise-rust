# noise-rust-crypto 0.6.3

* Update x25519-dalek dependency to 2.0.1

# noise-protocol 0.2.1

* Update arrayvec dependency to 0.7.6

# noise-protocol 0.1.4

* Update arrayvec dependency to 0.7.2

# noise-rust-crypto 0.4.0

* Change output type of `Blake2s` to use zeroing wrapping as well.

* Update dependencies to newer version.

# noise-rust-crypto 0.3.0

* Zero sensitive material on drop. (#20)

# noise-protocol 0.1.3

* Support `Vec` based APIs in no-std via the `alloc` crate. (#19)

# noise-rust-crypto 0.2.1

* Support for no-std. (#16, #18)

# noise-rust-crypto 0.2.0

* Use x25519-dalek and RustCrypto crates instead of rust-crypto.

* There is a feature for each primitive, you can pick exactly what you need.

# noise-protocol 0.1.1

* Update dependency arrayvec to 0.5

# noise-sodiumoxide 0.1.1

* Add `Eq` and `PartialEq` implementations.

# 0.1.0

## `noise-protocol`

No API change.

## `noise-sodiumoxide`

* Update to use sodiumoxide 0.2.

* Add wrapper for AES-256-GCM.

* Fix alignment of Blake2b state.

## `noise-ring`

Removed.

## `noise-rust-crypto`

No API change.

* Update to use rand 0.6.

* No longer depends on byteorder.
