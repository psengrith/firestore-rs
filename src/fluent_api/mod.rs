//! Provides a fluent, chainable API for constructing and executing Firestore operations.
//!
//! This module is the entry point for the fluent API, which allows for a more declarative
//! and type-safe way to interact with Firestore compared to using the direct methods on
//! [`FirestoreDb`](crate::FirestoreDb) with [`FirestoreQueryParams`](crate::FirestoreQueryParams).
//!
//! The main way to access this API is via the [`FirestoreDb::fluent()`](crate::FirestoreDb::fluent) method,
//! which returns a [`FirestoreExprBuilder`]. From there, you can chain calls to build
//! `select`, `insert`, `update`, `delete`, or `list` operations.
//!
//! Each operation type has its own dedicated builder module:
//! - [`delete_builder`]: For constructing delete operations.
//! - [`document_transform_builder`]: For specifying field transformations in update operations.
//! - [`insert_builder`]: For constructing insert/create operations.
//! - [`listing_builder`]: For listing documents or collection IDs.
//! - [`select_aggregation_builder`]: For building aggregation queries (e.g., count, sum, avg).
//! - [`select_builder`]: For constructing query/select operations.
//! - [`select_filter_builder`]: For building complex filter conditions for queries.
//! - [`update_builder`]: For constructing update operations.
//! ```

// Linter allowance for functions that might have many arguments,
// often seen in builder patterns or comprehensive configuration methods.
#![allow(clippy::too_many_arguments)]

pub mod delete_builder;
pub mod document_transform_builder;
pub mod insert_builder;
pub mod listing_builder;
pub mod select_aggregation_builder;
pub mod select_builder;
pub mod select_filter_builder;
pub mod update_builder;

use crate::delete_builder::FirestoreDeleteInitialBuilder;
use crate::fluent_api::select_builder::FirestoreSelectInitialBuilder;
use crate::insert_builder::FirestoreInsertInitialBuilder;
use crate::listing_builder::FirestoreListingInitialBuilder;
use crate::update_builder::FirestoreUpdateInitialBuilder;
use crate::{
    FirestoreAggregatedQuerySupport, FirestoreCreateSupport, FirestoreDb, FirestoreDeleteSupport,
    FirestoreGetByIdSupport, FirestoreListenSupport, FirestoreListingSupport,
    FirestoreQuerySupport, FirestoreUpdateSupport,
};

/// The entry point for building fluent Firestore expressions.
///
/// Obtain an instance of this builder by calling [`FirestoreDb::fluent()`](crate::FirestoreDb::fluent).
/// From this builder, you can chain methods to specify the type of operation
/// (select, insert, update, delete, list) and then further configure and execute it.
///
/// The type parameter `D` represents the underlying database client type, which
/// must implement various support traits (like [`FirestoreQuerySupport`], [`FirestoreCreateSupport`], etc.).
/// This is typically [`FirestoreDb`](crate::FirestoreDb).
#[derive(Clone, Debug)]
pub struct FirestoreExprBuilder<'a, D> {
    db: &'a D,
}

impl<'a, D> FirestoreExprBuilder<'a, D>
where
    D: FirestoreQuerySupport
        + FirestoreCreateSupport
        + FirestoreDeleteSupport
        + FirestoreUpdateSupport
        + FirestoreListingSupport
        + FirestoreGetByIdSupport
        + FirestoreListenSupport
        + FirestoreAggregatedQuerySupport
        + Clone
        + Send
        + Sync
        + 'static,
{
    /// Creates a new `FirestoreExprBuilder` with a reference to the database client.
    /// This is typically called by [`FirestoreDb::fluent()`](crate::FirestoreDb::fluent).
    pub(crate) fn new(db: &'a D) -> Self {
        Self { db }
    }

    /// Begins building a Firestore select/query operation.
    ///
    /// Returns a [`FirestoreSelectInitialBuilder`] to further configure the query.
    #[inline]
    pub fn select(self) -> FirestoreSelectInitialBuilder<'a, D> {
        FirestoreSelectInitialBuilder::new(self.db)
    }

    /// Begins building a Firestore insert/create operation.
    ///
    /// Returns a [`FirestoreInsertInitialBuilder`] to further configure the insertion.
    #[inline]
    pub fn insert(self) -> FirestoreInsertInitialBuilder<'a, D> {
        FirestoreInsertInitialBuilder::new(self.db)
    }

    /// Begins building a Firestore update operation.
    ///
    /// Returns a [`FirestoreUpdateInitialBuilder`] to further configure the update.
    #[inline]
    pub fn update(self) -> FirestoreUpdateInitialBuilder<'a, D> {
        FirestoreUpdateInitialBuilder::new(self.db)
    }

    /// Begins building a Firestore delete operation.
    ///
    /// Returns a [`FirestoreDeleteInitialBuilder`] to further configure the deletion.
    #[inline]
    pub fn delete(self) -> FirestoreDeleteInitialBuilder<'a, D> {
        FirestoreDeleteInitialBuilder::new(self.db)
    }

    /// Begins building a Firestore list operation (e.g., listing documents in a collection
    /// or listing collection IDs).
    ///
    /// Returns a [`FirestoreListingInitialBuilder`] to further configure the listing operation.
    #[inline]
    pub fn list(self) -> FirestoreListingInitialBuilder<'a, D> {
        FirestoreListingInitialBuilder::new(self.db)
    }
}

impl FirestoreDb {
    /// Provides access to the fluent API for building Firestore operations.
    ///
    /// This is the main entry point for using the chainable builder pattern.
    #[inline]
    pub fn fluent(&self) -> FirestoreExprBuilder<FirestoreDb> {
        FirestoreExprBuilder::new(self)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    pub mod mockdb;

    // Test structure used in fluent API examples and tests.
    pub struct TestStructure {
        pub some_id: String,
        pub one_more_string: String,
        pub some_num: u64,
    }
}
