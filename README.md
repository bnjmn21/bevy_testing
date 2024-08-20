[![Crates.io Size](https://img.shields.io/crates/size/bevy_testing?label=size)](https://crates.io/crates/bevy_testing)
[![GitHub Issues or Pull Requests](https://img.shields.io/github/issues-pr/bnjmn21/bevy_testing)](https://github.com/bnjmn21/bevy_testing/issues?q=is%3Aissue+is%3Aopen)
[![MIT License](https://img.shields.io/crates/l/bevy_testing)](https://github.com/bnjmn21/bevy_testing/blob/master/LICENSE)
[![Bevy version 0.14.1](https://img.shields.io/badge/bevy-0.14.1-green)](https://docs.rs/bevy/0.14.1/bevy/index.html)

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

just [`use bevy_testing::TestApp` (view docs)](https://docs.rs/bevy_testing/latest/bevy_testing/trait.TestApp.html)!

In cases where you need more control, you can always get the world via
`App::world()` and `App::world_mut()`.

This library also exports `bevy_testing::p`, short for prelude, which contains the entire bevy prelude as well as `TestApp`.

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

## Bevy versions

bevy   | bevy_testing
-------|--
`0.14` | `0.1.1`