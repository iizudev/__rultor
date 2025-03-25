# bevy_actify_derive

[`bevy_actify`](https://github.com/bevious/bevy_actify)'s derive macros.

## How to use

By default `bevy_actify` already re-exports this crates' derive
macros for its traits and normaly one would never want to use this
crate directly, though if you have a reason to, here is how you
can add it to your project:

```toml
[dependencies]
bevy_actify_derive = "*" // refer to crates.io for the actual version.
```

### Usage

```rust
use bevy_actify::InputAction;

#[derive(InputAction, PartialEq, Clone)]
struct Jump;
```

## How to contribute

Fork repository, make changes, send us a pull request. We will review your changes and apply them to the master branch shortly, provided they don't violate our quality standards.
