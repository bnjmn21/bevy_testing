#[allow(unused_imports)] // used in doc
use super::p::*;

use std::fmt::Debug;

use bevy::ecs::query::ReadOnlyQueryData;

use crate::{mismatch, unexpected_match};

/// A struct to perform tests on a query which is created via [`App::query`].
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
pub struct AssertQuery<'w, D: ReadOnlyQueryData>
where
    D::Item<'w>: Debug + PartialEq,
{
    pub(crate) query: Vec<D::Item<'w>>,
    pub(crate) invert: bool,
}

impl<'w, D: ReadOnlyQueryData> AssertQuery<'w, D>
where
    D::Item<'w>: Debug + PartialEq,
{
    /// Returns an inverted [`AssertQuery`].
    /// When chaining methods,
    /// the inverted state gets reset after every method.
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
    ///     .not().has(&Position { x: 4.0, y: -3.0 })
    ///     .not().length(5);
    /// ```
    #[allow(clippy::should_implement_trait)] // users should not need to import std::ops::Not
    pub fn not(mut self) -> Self {
        self.invert = !self.invert;
        self
    }

    /// Checks if the query contains the given and only the given bundles.
    /// The given bundles do not need to be in order.
    /// If you only need to check if the query *contains* the given bundles use [`Self::has_all`].
    ///
    /// This can be inverted via [`Self::not`].
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
    ///     .matches(vec![
    ///         &Position { x: 0.0, y: 0.0 },
    ///         &Position { x: 4.5, y: 1.0 },
    ///         &Position { x: 1.0, y: 2.0 },
    ///     ])
    ///     .not().matches(vec![
    ///         &Position { x: 0.0, y: 0.0 },
    ///         &Position { x: 1.0, y: 2.0 },
    ///     ]);
    /// ```
    pub fn matches(self, given: Vec<D::Item<'w>>) -> Self {
        if self.invert {
            return self.not_matches(given);
        }

        for bundle in self.query.iter() {
            let is_match = given.iter().any(|v| v == bundle);
            if !is_match {
                mismatch(
                    "One of the given bundles wasn't found in the query.",
                    &given,
                    None::<()>,
                );
            }
        }
        for bundle in given.iter() {
            let is_match = self.query.iter().any(|v| v == bundle);
            if !is_match {
                mismatch(
                    "The query contains an unexpected bundle.",
                    None::<()>,
                    bundle,
                );
            }
        }
        if given.len() != self.query.len() {
            mismatch(
                "The length of the query result and the given result mismatches.",
                given.len(),
                self.query.len(),
            );
        }

        self
    }
    fn not_matches(self, given: Vec<D::Item<'w>>) -> Self {
        for bundle in self.query.iter() {
            let is_match = given.iter().any(|v| v == bundle);
            if !is_match {
                return self.reset_invert();
            }
        }
        for bundle in given.iter() {
            let is_match = self.query.iter().any(|v| v == bundle);
            if !is_match {
                return self.reset_invert();
            }
        }

        if given.len() != self.query.len() {
            return self.reset_invert();
        }

        unexpected_match("The query matches with the given bundles", given);
    }

    /// Checks if the query contains the given bundle.
    /// This can be inverted via [`Self::not`].
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
    ///     .has(&Position { x: 0.0, y: 0.0 })
    ///     .not().has(&Position { x: 3.0, y: -2.0 });
    /// ```
    pub fn has(self, given: D::Item<'w>) -> Self {
        if self.invert {
            return self.not_has(given);
        }

        let is_match = self.query.iter().any(|bundle| bundle == &given);
        if !is_match {
            mismatch(
                "The given bundle wasn't found in the query.",
                &given,
                None::<()>,
            );
        }

        self
    }
    fn not_has(self, given: D::Item<'w>) -> Self {
        let is_match = self.query.iter().any(|bundle| bundle == &given);
        if !is_match {
            return self.reset_invert();
        }

        unexpected_match("The query contains all given bundles", given);
    }

    /// Checks if the query contains all given bundle.
    /// If you want to check for exact equality beetween the query and the given bundles, use [`Self::matches`].
    ///
    /// This can be inverted via [`Self::not`].
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
    ///     .has_all(vec![&Position { x: 0.0, y: 0.0 }, &Position { x: 1.0, y: 2.0 }])
    ///     .not().has_all(vec![&Position { x: 1.0, y: 2.0 }, &Position { x: 3.0, y: -2.0 }]);
    /// ```
    pub fn has_all(self, given: Vec<D::Item<'w>>) -> Self {
        if self.invert {
            return self.not_has_all(given);
        }

        for given in given.iter() {
            let is_match = self.query.iter().any(|bundle| bundle == given);
            if !is_match {
                mismatch(
                    "The given bundle wasn't found in the query.",
                    given,
                    None::<()>,
                );
            }
        }

        self
    }
    fn not_has_all(self, given: Vec<D::Item<'w>>) -> Self {
        for given in given.iter() {
            let is_match = self.query.iter().any(|bundle| bundle == given);
            if !is_match {
                return self.reset_invert();
            }
        }

        unexpected_match("The query has all given bundles.", given);
    }

    /// Checks if the query contains any of the given bundle.
    ///
    /// This can be inverted via [`Self::not`].
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
    ///     .has_any(vec![&Position { x: 3.0, y: -2.0 }, &Position { x: 1.0, y: 2.0 }])
    ///     .not().has_any(vec![&Position { x: 5.0, y: -6.0 }, &Position { x: 0.0, y: 3.0 }]);
    /// ```
    pub fn has_any(self, given: Vec<D::Item<'w>>) -> Self {
        if self.invert {
            return self.not_has_any(given);
        }

        let is_match = self
            .query
            .iter()
            .any(|bundle| given.iter().any(|given| given == bundle));
        if !is_match {
            mismatch(
                "None of the given bundles were found in the query.",
                given,
                None::<()>,
            );
        }

        self
    }
    fn not_has_any(self, given: Vec<D::Item<'w>>) -> Self {
        let is_match = self
            .query
            .iter()
            .any(|bundle| given.iter().any(|given| given == bundle));
        if is_match {
            unexpected_match(
                "Some of the given bundles were found in the query.",
                self.query
                    .iter()
                    .find(|bundle| given.iter().any(|given| given == *bundle)),
            );
        }

        self.reset_invert()
    }

    /// Checks if all bundles of the query match a given predicate.
    /// If you need to check if any bundle matches the predicate, use [`Self::any`].
    ///
    /// This can be inverted via [`Self::not`].
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
    /// app.spawn(Position { x: 1.0, y: -1.0 });
    /// app.spawn(Position { x: 4.5, y: -4.5 });
    ///
    /// app.query::<&Position>()
    ///     .all(|bundle: &&Position| bundle.x + bundle.y == 0.0)
    ///     .not().all(|bundle: &&Position| bundle.x == 0.0);
    /// ```
    pub fn all(self, predicate: impl Fn(&D::Item<'w>) -> bool) -> Self {
        if self.invert {
            return self.not_all(predicate);
        }

        let predicate = &predicate;
        for bundle in self.query.iter() {
            if !predicate(bundle) {
                mismatch(
                    "The predicate fails on one of the bundles",
                    "impl Fn(&D::Item<'w>) -> bool",
                    bundle,
                );
            }
        }

        self
    }
    fn not_all(self, predicate: impl Fn(&D::Item<'w>) -> bool) -> Self {
        let predicate = &predicate;
        for bundle in self.query.iter() {
            if !predicate(bundle) {
                return self.reset_invert();
            }
        }

        unexpected_match("The predicate matches on all of the bundles.", self.query);
    }

    /// Checks if any of the bundles of the query match a given predicate.
    /// If you need to check if all bundles matche the predicate, use [`Self::all`].
    ///
    /// This can be inverted via [`Self::not`].
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
    /// app.spawn(Position { x: 4.5, y: -1.0 });
    ///
    /// app.query::<&Position>()
    ///     .any(|bundle: &&Position| bundle.x == 0.0)
    ///     .not().any(|bundle: &&Position| bundle.y == 1.0);
    /// ```
    pub fn any(self, predicate: impl Fn(&D::Item<'w>) -> bool) -> Self {
        if self.invert {
            return self.not_any(predicate);
        }

        let predicate = &predicate;
        let is_match = self.query.iter().any(predicate);
        if !is_match {
            mismatch(
                "The predicate didn't match on any of the bundles",
                "impl Fn(&D::Item<'w>) -> bool",
                None::<()>,
            );
        }

        self
    }
    fn not_any(self, predicate: impl Fn(&D::Item<'w>) -> bool) -> Self {
        let predicate = &predicate;
        let is_match = self.query.iter().any(predicate);
        if is_match {
            unexpected_match(
                "The predicate matched on one of the bundles",
                self.query.iter().find(|bundle| predicate(bundle)),
            );
        }

        self.reset_invert()
    }

    /// Checks if the amount of bundles in the query matches the given length.
    ///
    /// This can be inverted via [`Self::not`].
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
    /// app.spawn(Position { x: 4.5, y: -1.0 });
    ///
    /// app.query::<&Position>()
    ///     .length(3)
    ///     .not().length(4);
    /// ```
    pub fn length(self, given: usize) -> Self {
        if self.invert {
            return self.not_length(given);
        }

        if self.query.len() != given {
            mismatch(
                "The length of the query result mismatches.",
                given,
                self.query.len(),
            );
        }

        self
    }
    fn not_length(self, given: usize) -> Self {
        if self.query.len() == given {
            unexpected_match("The length of the query result matches.", given);
        }

        self.reset_invert()
    }

    fn reset_invert(mut self) -> Self {
        self.invert = false;
        self
    }
}
