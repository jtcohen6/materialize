// Copyright 2019 Materialize, Inc. All rights reserved.
//
// This file is part of Materialize. Materialize may not be used or
// distributed without the express permission of Materialize, Inc.

//! SQL-dataflow translation.

#![deny(missing_debug_implementations)]

use dataflow_types::{Dataflow, PeekWhen, RowSetFinishing, Sink, Source, View};
use failure::bail;

use repr::{Datum, RelationType};
pub use session::Session;
use sqlparser::ast::ObjectName;

use store::DataflowStore;

mod expr;
mod query;
mod scope;
mod session;
mod statement;
pub mod store;
mod transform;

// this is used by sqllogictest to turn sql values into `Datum`
pub use query::scalar_type_from_sql;

/// Instructions for executing a SQL query.
#[derive(Debug)]
pub enum Plan {
    CreateSource(Source),
    CreateSources(Vec<Source>),
    CreateSink(Sink),
    CreateView(View),
    DropSources(Vec<String>),
    DropViews(Vec<String>),
    EmptyQuery,
    DidSetVariable,
    Parsed {
        name: String,
    },
    Peek {
        source: ::expr::RelationExpr,
        when: PeekWhen,
        transform: RowSetFinishing,
    },
    Tail(Dataflow),
    SendRows {
        typ: RelationType,
        rows: Vec<Vec<Datum>>,
    },
    ExplainPlan {
        typ: RelationType,
        relation_expr: ::expr::RelationExpr,
    },
}

fn extract_sql_object_name(n: &ObjectName) -> Result<String, failure::Error> {
    if n.0.len() != 1 {
        bail!("qualified names are not yet supported: {}", n.to_string())
    }
    Ok(n.to_string())
}

/// Holds all previously planned dataflows, and is the owner of many methods for creating `Plan`s.
#[derive(Debug)]
pub struct Planner {
    pub dataflows: DataflowStore,
}
