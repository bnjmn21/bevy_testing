![Crates.io Size](https://img.shields.io/crates/size/bevy_testing?label=size)
![GitHub Issues or Pull Requests](https://img.shields.io/github/issues-pr/bnjmn21/bevy_testing)
![MIT License](https://img.shields.io/crates/l/bevy_testing)
![Bevy version 0.14.1](https://img.shields.io/badge/bevy-0.14.1-green)

# Test things [bevy](https://bevyengine.org/)!

```rust
use my_lib::{Countdown, CountdownPlugin};

// import bevy's and bevy_testing's prelude
use bevy_testing::p::*;

// create your app as usual
let mut app = App::new();
app.add_plugins(CountdownPlugin);

// useful world methods are exposed
app.spawn(Countdown(10));

// run schedules once
app.update_once();

// assert that a query returns some values
app.query::<&Countdown>()
    .matches(vec![&Countdown(9)]);
```

## Usage

just [`use bevy_testing::TestApp` (view docs)](https://docs.rs/bevy_testing)!

In cases where you need more control, you can always get the world via
`App::world()` and `App::world_mut()`.

This library also exports `bevy_testing::p`, short for prelude, which contains the entire bevy prelude as well as `TestApp`.

**Note:** By default, the entire crate disables itself when `cfg(not(test))` to improve compile times.
You can disable this behaviour with the `always` feature flag.

## Query Matching

Use `App::query()` to check...

method name      | description
-----------------|--
`.matches()`     | if the query matches the given bundles
`.has()`         | if the query contains the given bundle
`.has_all()`     | if the query contains all given bundles
`.has_any()`     | if the query contains any of the given bundles
`.all()`         | if all bundles match the given predicate
`.any()`         | if any bundle matches the given predicate
`.length()`      | if the query matches the given length
`.not()` ...     | to invert the test

## Feature Flags

name     | description
---------|--------
`always` | always enable this crate, see [Usage](#usage)