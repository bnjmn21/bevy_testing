//! [![Crates.io Size](https://img.shields.io/crates/size/bevy_testing?label=size)](https://crates.io/crates/bevy_testing)
//! [![GitHub Issues or Pull Requests](https://img.shields.io/github/issues-pr/bnjmn21/bevy_testing)](https://github.com/bnjmn21/bevy_testing/issues?q=is%3Aissue+is%3Aopen)
//! [![MIT License](https://img.shields.io/crates/l/bevy_testing)](https://github.com/bnjmn21/bevy_testing/blob/master/LICENSE)
//! [![Bevy version 0.14.1](https://img.shields.io/badge/bevy-0.14.1-green)](https://docs.rs/bevy/0.14.1/bevy/index.html)
//!
//! # Test things [bevy](https://bevyengine.org/)!
//!
//! ```rust
//! # mod my_lib {
//! #     use bevy_testing::p::*;
//! #
//! #     #[derive(Component, Debug, PartialEq)]
//! #     pub struct Countdown(pub u32);
//! #
//! #     pub struct CountdownPlugin;
//! #
//! #     impl Plugin for CountdownPlugin {
//! #         fn build(&self, app: &mut App) {
//! #             app.add_systems(Update, countdown_sys);
//! #         }
//! #     }
//! #
//! #     fn countdown_sys(mut query: Query<&mut Countdown>) {
//! #         for mut countdown in &mut query {
//! #             countdown.0 -= 1;
//! #         }
//! #     }
//! # }
//!
//! use my_lib::{Countdown, CountdownPlugin};
//!
//! // import bevy's and bevy_testing's prelude
//! use bevy_testing::p::*;
//!
//! // create your app as usual
//! let mut app = App::new();
//! app.add_plugins(CountdownPlugin);
//!
//! // useful world methods are exposed
//! app.spawn(Countdown(10));
//!
//! // run schedules once
//! app.update_once();
//!
//! // assert that a query returns some values
//! app.query::<&Countdown>()
//!     .matches(vec![&Countdown(9)]);
//! ```
//!
//! ## Usage
//!
//! just [`use bevy_testing::TestApp` (view docs)](https://docs.rs/bevy_testing/latest/bevy_testing/trait.TestApp.html)!
//!
//! In cases where you need more control, you can always get the world via
//! `App::world()` and `App::world_mut()`.
//!
//! This library also exports `bevy_testing::p`, short for prelude, which contains the entire bevy prelude as well as `TestApp`.
//!
//! ## Query Matching
//!
//! Use `App::query()` to check...
//!
//! method name      | description
//! -----------------|--
//! `.matches()`     | if the query matches the given bundles
//! `.has()`         | if the query contains the given bundle
//! `.has_all()`     | if the query contains all given bundles
//! `.has_any()`     | if the query contains any of the given bundles
//! `.all()`         | if all bundles match the given predicate
//! `.any()`         | if any bundle matches the given predicate
//! `.length()`      | if the query matches the given length
//! `.not()` ...     | to invert the test
//!
//! ## Bevy versions
//!
//! bevy   | bevy_testing
//! -------|--
//! `0.14` | `0.1.1`
//!

mod query;

use std::{any::type_name, fmt::Debug};

use bevy::{
    ecs::{
        query::{QueryFilter, ReadOnlyQueryData},
        world::SpawnBatchIter,
    },
    prelude::*,
};
use colored::Colorize;
use query::AssertQuery;
use sealed::sealed;

#[sealed]
pub trait TestApp {
    /// Spawns a new [`Entity`] and returns a corresponding [`EntityWorldMut`], which can be used
    /// to add components to the entity or retrieve its id.
    ///
    /// ```
    /// use bevy_testing::p::*;
    ///
    /// #[derive(Component)]
    /// struct Position {
    ///   x: f32,
    ///   y: f32,
    /// }
    /// #[derive(Component)]
    /// struct Label(&'static str);
    /// #[derive(Component)]
    /// struct Num(u32);
    ///
    /// let mut app = App::new();
    /// let entity = app.spawn_empty()
    ///     .insert(Position { x: 0.0, y: 0.0 }) // add a single component
    ///     .insert((Num(1), Label("hello"))) // add a bundle of components
    ///     .id();
    ///
    /// let position = app.component::<Position>(entity);
    /// assert_eq!(position.x, 0.0);
    /// ```
    fn spawn_empty(&mut self) -> EntityWorldMut;

    /// Spawns a new [`Entity`] with a given [`Bundle`] of [components](`Component`) and returns
    /// a corresponding [`EntityWorldMut`], which can be used to add components to the entity or
    /// retrieve its id.
    ///
    /// ```
    /// use bevy_testing::p::*;
    ///
    /// #[derive(Component)]
    /// struct Position {
    ///   x: f32,
    ///   y: f32,
    /// }
    ///
    /// #[derive(Component)]
    /// struct Velocity {
    ///     x: f32,
    ///     y: f32,
    /// };
    ///
    /// #[derive(Component)]
    /// struct Name(&'static str);
    ///
    /// #[derive(Bundle)]
    /// struct PhysicsBundle {
    ///     position: Position,
    ///     velocity: Velocity,
    /// }
    ///
    /// let mut app = App::new();
    ///
    /// // `spawn` can accept a single component:
    /// app.spawn(Position { x: 0.0, y: 0.0 });
    ///
    /// // It can also accept a tuple of components:
    /// app.spawn((
    ///     Position { x: 0.0, y: 0.0 },
    ///     Velocity { x: 1.0, y: 1.0 },
    /// ));
    ///
    /// // Or it can accept a pre-defined Bundle of components:
    /// app.spawn(PhysicsBundle {
    ///     position: Position { x: 2.0, y: 2.0 },
    ///     velocity: Velocity { x: 0.0, y: 4.0 },
    /// });
    ///
    /// let entity = app
    ///     // Tuples can also mix Bundles and Components
    ///     .spawn((
    ///         PhysicsBundle {
    ///             position: Position { x: 2.0, y: 2.0 },
    ///             velocity: Velocity { x: 0.0, y: 4.0 },
    ///         },
    ///         Name("Elaina Proctor"),
    ///     ))
    ///     // Calling id() will return the unique identifier for the spawned entity
    ///     .id();
    ///
    /// let position = app.component::<Position>(entity);
    /// assert_eq!(position.x, 2.0);
    /// ```
    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut;

    /// Spawns a batch of entities with the same component [`Bundle`] type. Takes a given
    /// [`Bundle`] iterator and returns a corresponding [`Entity`] iterator.
    /// This is more efficient than spawning entities and adding components to them individually,
    /// but it is limited to spawning entities with the same [`Bundle`] type, whereas spawning
    /// individually is more flexible.
    ///
    /// ```
    /// use bevy_testing::p::*;
    ///
    /// #[derive(Component)]
    /// struct Str(&'static str);
    /// #[derive(Component)]
    /// struct Num(u32);
    ///
    /// let mut app = App::new();
    /// let entities = app.spawn_batch(vec![
    ///   (Str("a"), Num(0)), // the first entity
    ///   (Str("b"), Num(1)), // the second entity
    /// ]).collect::<Vec<Entity>>();
    ///
    /// assert_eq!(entities.len(), 2);
    /// ```
    fn spawn_batch<I>(&mut self, iter: I) -> SpawnBatchIter<'_, I::IntoIter>
    where
        I: IntoIterator,
        I::Item: Bundle;

    /// Retrieves an [`EntityRef`] that exposes read-only operations for the given `entity`.
    /// This will panic if the `entity` does not exist. Use [`TestApp::get_entity`] if you want
    /// to check for entity existence instead of implicitly panic-ing.
    ///
    /// If you want to get a specific component from an entity, use [`TestApp::component`].
    ///
    /// ```
    /// use bevy_testing::p::*;
    ///
    /// #[derive(Component)]
    /// struct Position {
    ///   x: f32,
    ///   y: f32,
    /// }
    ///
    /// let mut app = App::new();
    /// let entity = app.spawn(Position { x: 0.0, y: 0.0 }).id();
    /// let position = app.entity(entity).get::<Position>().unwrap();
    /// // preferred:
    /// // let position = world.component::<Position>(entity)
    ///
    /// assert_eq!(position.x, 0.0);
    /// ```
    fn entity(&self, entity: Entity) -> EntityRef;

    /// Retrieves an [`EntityWorldMut`] that exposes read and write operations for the given `entity`.
    /// This will panic if the `entity` does not exist. Use [`TestApp::get_entity_mut`] if you want
    /// to check for entity existence instead of implicitly panic-ing.
    ///
    /// ```
    /// use bevy_testing::p::*;
    ///
    /// #[derive(Component)]
    /// struct Position {
    ///   x: f32,
    ///   y: f32,
    /// }
    ///
    /// let mut app = App::new();
    /// let entity = app.spawn(Position { x: 0.0, y: 0.0 }).id();
    ///
    /// let mut entity_mut = app.entity_mut(entity);
    /// let mut position = entity_mut.get_mut::<Position>().unwrap();
    /// position.x = 1.0;
    ///
    /// let new_position = app.component::<Position>(entity);
    /// assert_eq!(new_position.x, 1.0);
    ///
    /// ```
    fn entity_mut(&mut self, entity: Entity) -> EntityWorldMut;

    /// Retrieves an [`EntityRef`] that exposes read-only operations for the given `entity`.
    /// Returns [`None`] if the `entity` does not exist.
    /// Instead of unwrapping the value returned from this function, prefer [`TestApp::entity`].
    ///
    /// If you want to get a specific component from an entity, use [`TestApp::get_component`].
    ///
    /// ```
    /// use bevy_testing::p::*;
    ///
    /// #[derive(Component)]
    /// struct Position {
    ///   x: f32,
    ///   y: f32,
    /// }
    ///
    /// let mut app = App::new();
    /// let entity = app.spawn(Position { x: 0.0, y: 0.0 }).id();
    /// let entity_ref = app.get_entity(entity).unwrap();
    /// let position = entity_ref.get::<Position>().unwrap();
    /// // preferred:
    /// // let position = world.component::<Position>(entity)
    /// assert_eq!(position.x, 0.0);
    /// ```
    fn get_entity(&self, entity: Entity) -> Option<EntityRef>;

    /// Retrieves an [`EntityWorldMut`] that exposes read and write operations for the given `entity`.
    /// Returns [`None`] if the `entity` does not exist.
    /// Instead of unwrapping the value returned from this function, prefer [`TestApp::entity_mut`].
    ///
    /// ```
    /// use bevy_testing::p::*;
    ///
    /// #[derive(Component)]
    /// struct Position {
    ///   x: f32,
    ///   y: f32,
    /// }
    ///
    /// let mut app = App::new();
    /// let entity = app.spawn(Position { x: 0.0, y: 0.0 }).id();
    /// let mut entity_mut = app.get_entity_mut(entity).unwrap();
    /// let mut position = entity_mut.get_mut::<Position>().unwrap();
    /// position.x = 1.0;
    ///
    /// let new_position = app.component::<Position>(entity);
    /// assert_eq!(new_position.x, 1.0);
    /// ```
    fn get_entity_mut(&mut self, entity: Entity) -> Option<EntityWorldMut>;

    /// Gets access to the component of type `T` for the given `entity`.
    /// Panics if the entity doesn't have a component of type `T` or
    /// if the `entity` doesn't exist.
    ///
    /// This is effectively a shortcut for `App::entity(entity).get::<T>().unwrap()`.
    ///
    /// ```
    /// use bevy_testing::p::*;
    ///
    /// #[derive(Component)]
    /// struct Position {
    ///   x: f32,
    ///   y: f32,
    /// }
    ///
    /// let mut app = App::new();
    /// let entity = app.spawn(Position { x: 0.0, y: 0.0 }).id();
    /// let position = app.component::<Position>(entity);
    /// assert_eq!(position.x, 0.0);
    /// ```
    fn component<T: Component>(&self, entity: Entity) -> &T;

    /// Gets access to the component of type `T` for the given `entity`.
    /// Returns [`None`] if the entity doesn't have a component of type `T`.
    /// Panics if the `entity` doesn't exist.
    ///
    /// This is effectively a shortcut for `App::entity(entity).get::<T>()`.
    ///
    /// Instead of unwrapping the value returned from this function, prefer [`TestApp::entity`].
    ///
    /// ```
    /// use bevy_testing::p::*;
    ///
    /// #[derive(Component)]
    /// struct Position {
    ///   x: f32,
    ///   y: f32,
    /// }
    ///
    /// let mut app = App::new();
    /// let entity = app.spawn(Position { x: 0.0, y: 0.0 }).id();
    /// let position = app.get_component::<Position>(entity).unwrap();
    /// // preferred:
    /// // let position = world.component::<Position>(entity)
    /// assert_eq!(position.x, 0.0);
    /// ```
    fn get_component<T: Component>(&self, entity: Entity) -> Option<&T>;

    // where is `component_mut` and `get_component_mut` you may ask.
    // I specifically left them out because they are a huge pain to implement for some reason.

    /// Returns an [`AssertQuery`] which can be used to perform tests on a query.
    /// To invert the test, use [`AssertQuery::not`].
    ///
    /// If you need to use a query filter, use [`App::query_filtered`].
    ///
    /// ```
    /// use bevy_testing::p::*;
    ///
    /// #[derive(Component, Debug, PartialEq)]
    /// struct Position {
    ///   x: f32,
    ///   y: f32,
    /// }
    ///
    /// let mut app = App::new();
    /// app.spawn(Position { x: 0.0, y: 0.0 });
    /// app.spawn(Position { x: 1.0, y: 2.0 });
    /// app.spawn(Position { x: 4.5, y: 1.0 });
    ///
    /// app.query::<&Position>()
    ///     .has(&Position { x: 1.0, y: 2.0 })
    ///     .not().has(&Position { x: 4.0, y: -3.0 })
    ///     .length(3);
    /// ```
    fn query<'w, D: ReadOnlyQueryData>(&'w mut self) -> AssertQuery<'w, D>
    where
        D::Item<'w>: PartialEq + Debug;

    /// Returns an [`AssertQuery`] which can be used to perform tests on a query, with a query filter.
    /// To invert the test, use [`AssertQuery::not`].
    ///
    /// If you don't need the filter, use [`App::query`].
    ///
    /// Note that some filters such as [`Changed`] might behave unexpectedly.
    ///
    /// ```
    /// use bevy_testing::p::*;
    ///
    /// #[derive(Component, Debug, PartialEq)]
    /// struct Position {
    ///   x: f32,
    ///   y: f32,
    /// }
    ///
    /// #[derive(Component, Debug, PartialEq)]
    /// struct Marker;
    ///
    /// let mut app = App::new();
    /// app.spawn((Position { x: 0.0, y: 0.0 }, Marker));
    /// app.spawn((Position { x: 1.0, y: 2.0 }, Marker));
    /// app.spawn(Position { x: 4.5, y: 1.0 });
    ///
    /// app.query_filtered::<&Position, With<Marker>>()
    ///     .has(&Position { x: 1.0, y: 2.0 })
    ///     .not().has(&Position { x: 4.5, y: 1.0 })
    ///     .length(2);
    /// ```
    fn query_filtered<'w, D: ReadOnlyQueryData, F: QueryFilter>(&'w mut self) -> AssertQuery<'w, D>
    where
        D::Item<'w>: PartialEq + Debug;

    /// Updates the app once.
    /// This will run all of the main schedules such as [`Update`] and [`FixedUpdate`],
    /// along with [`Startup`] if it's the first update.
    /// If you want to update the app multiple times, use [`App::update_n_times`].
    ///
    /// ```rust
    /// # mod my_lib {
    /// #    use bevy_testing::p::*;
    /// #
    /// #    #[derive(Component, Debug, PartialEq)]
    /// #    pub struct Countdown(pub u32);
    /// #
    /// #    pub struct CountdownPlugin;
    /// #
    /// #    impl Plugin for CountdownPlugin {
    /// #        fn build(&self, app: &mut App) {
    /// #            app.add_systems(Update, countdown_sys);
    /// #        }
    /// #    }
    /// #
    /// #    fn countdown_sys(mut query: Query<&mut Countdown>) {
    /// #        for mut countdown in &mut query {
    /// #            countdown.0 -= 1;
    /// #        }
    /// #    }
    /// # }
    /// use my_lib::{Countdown, CountdownPlugin};
    /// use bevy_testing::p::*;
    ///
    /// let mut app = App::new();
    /// app.add_plugins(CountdownPlugin);
    ///
    /// app.spawn(Countdown(10));
    /// app.update_once();
    /// app.query::<&Countdown>()
    ///     .matches(vec![&Countdown(9)]);
    /// ```
    fn update_once(&mut self);

    /// Updates the app `amount` times.
    /// This will run all of the main schedules such as [`Update`] and [`FixedUpdate`],
    /// along with [`Startup`] if it's the first update.
    /// If you want to update the app just once, use [`App::update_n_times`].
    ///
    /// ```rust
    /// # mod my_lib {
    /// #    use bevy_testing::p::*;
    /// #
    /// #    #[derive(Component, Debug, PartialEq)]
    /// #    pub struct Countdown(pub u32);
    /// #
    /// #    pub struct CountdownPlugin;
    /// #
    /// #    impl Plugin for CountdownPlugin {
    /// #        fn build(&self, app: &mut App) {
    /// #            app.add_systems(Update, countdown_sys);
    /// #        }
    /// #    }
    /// #
    /// #    fn countdown_sys(mut query: Query<&mut Countdown>) {
    /// #        for mut countdown in &mut query {
    /// #            countdown.0 -= 1;
    /// #        }
    /// #    }
    /// # }
    /// use my_lib::{Countdown, CountdownPlugin};
    /// use bevy_testing::p::*;
    ///
    /// let mut app = App::new();
    /// app.add_plugins(CountdownPlugin);
    ///
    /// app.spawn(Countdown(10));
    /// app.update_n_times(2);
    /// app.query::<&Countdown>()
    ///     .matches(vec![&Countdown(8)]);
    /// ```
    fn update_n_times(&mut self, amount: u32);
}

#[sealed]
impl TestApp for App {
    fn spawn_empty(&mut self) -> EntityWorldMut {
        self.world_mut().spawn_empty()
    }

    fn spawn<B: Bundle>(&mut self, bundle: B) -> EntityWorldMut {
        self.world_mut().spawn(bundle)
    }

    fn spawn_batch<I>(&mut self, iter: I) -> SpawnBatchIter<'_, I::IntoIter>
    where
        I: IntoIterator,
        I::Item: Bundle,
    {
        self.world_mut().spawn_batch(iter)
    }

    fn entity(&self, entity: Entity) -> EntityRef {
        self.world().entity(entity)
    }

    fn entity_mut(&mut self, entity: Entity) -> EntityWorldMut {
        self.world_mut().entity_mut(entity)
    }

    fn get_entity(&self, entity: Entity) -> Option<EntityRef> {
        self.world().get_entity(entity)
    }

    fn get_entity_mut(&mut self, entity: Entity) -> Option<EntityWorldMut> {
        self.world_mut().get_entity_mut(entity)
    }

    fn component<T: Component>(&self, entity: Entity) -> &T {
        self.get_component(entity).unwrap_or_else(|| {
            panic!(
                "component \"{}\" is not part of the entity",
                type_name::<T>()
            )
        })
    }

    fn get_component<T: Component>(&self, entity: Entity) -> Option<&T> {
        self.world().entity(entity).get::<T>()
    }

    fn query<'w, D: ReadOnlyQueryData>(&'w mut self) -> AssertQuery<'w, D>
    where
        D::Item<'w>: PartialEq + Debug,
    {
        let mut query = self.world_mut().query::<D>();
        let collected = query.iter(self.world()).collect::<Vec<_>>();
        AssertQuery {
            query: collected,
            invert: false,
        }
    }

    fn query_filtered<'w, D: ReadOnlyQueryData, F: QueryFilter>(&'w mut self) -> AssertQuery<'w, D>
    where
        D::Item<'w>: PartialEq + Debug,
    {
        let mut query = self.world_mut().query_filtered::<D, F>();
        let collected = query.iter(self.world()).collect::<Vec<_>>();
        AssertQuery {
            query: collected,
            invert: false,
        }
    }

    fn update_once(&mut self) {
        self.update();
    }

    fn update_n_times(&mut self, amount: u32) {
        for _ in 0..amount {
            self.update_once();
        }
    }
}

const MAX_DEBUG_LEN: usize = 300;

fn mismatch(message: &str, given: impl Debug, found: impl Debug) -> ! {
    let mut given = format!("{:#?}", given);
    if given.len() > MAX_DEBUG_LEN {
        given = given[0..MAX_DEBUG_LEN].to_owned() + &" ...".bright_black();
    }
    let mut found = format!("{:#?}", found);
    if found.len() > MAX_DEBUG_LEN {
        found = found[0..MAX_DEBUG_LEN].to_owned() + &" ...".bright_black();
    }
    eprintln!("{}", message.red());
    if given.contains('\n') {
        eprintln!("{}", "Given:".bright_black());
        eprintln!("{}", given);
    } else {
        eprintln!("{} {}", "Given:".bright_black(), given);
    }
    eprintln!();
    if found.contains('\n') {
        eprintln!("{}", "Found:".bright_black());
        eprintln!("{}", found);
    } else {
        eprintln!("{} {}", "Found:".bright_black(), found);
    }
    panic!("assertion failed");
}

fn unexpected_match(message: &str, matches: impl Debug) -> ! {
    let mut given = format!("{:#?}", matches);
    if given.len() > MAX_DEBUG_LEN {
        given = given[0..MAX_DEBUG_LEN].to_owned() + &" ...".bright_black();
    }
    eprintln!("{}", message.red());
    if given.contains('\n') {
        eprintln!("{}", "Match:".bright_black());
        eprintln!("{}", given);
    } else {
        eprintln!("{} {}", "Match:".bright_black(), given);
    }
    panic!("assertion failed");
}

pub mod p {
    //! A module that re-exports the entire [`bevy::prelude`] as well as [`TestApp`].

    pub use crate::TestApp;
    pub use bevy::prelude::*;
}

/// module for doctests
mod my_lib {
    use crate::p::*;

    #[derive(Component, Debug, PartialEq)]
    pub struct Countdown(pub u32);

    pub struct CountdownPlugin;

    impl Plugin for CountdownPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(Update, countdown_sys);
        }
    }

    fn countdown_sys(mut query: Query<&mut Countdown>) {
        for mut countdown in &mut query {
            countdown.0 -= 1;
        }
    }
}
