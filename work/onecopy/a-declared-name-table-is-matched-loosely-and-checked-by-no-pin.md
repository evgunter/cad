---
id: a-declared-name-table-is-matched-loosely-and-checked-by-no-pin
kind: issue
title: A declared name table is read loosely and pinned in one crate: any impl naming the token matches, a non-literal arm is dropped silently, and CLOSED_TYPE reasons are not checked against the declared tables
status: open
opened: 2026-11-16
priority: P3
cost: M
---


Filed from the review of #6793 (census-tables): findings S5, S6 and S8. That PR made a closed type's name table a carrier its roster declares (`NameCarrier::Table`, `crates/test-utils/src/source.rs`). Three looseness gaps remain, none of which reds today:

- **S5. `table_bodies` matches any `impl` whose head names the token** (`crates/test-utils/src/source.rs`, `table_bodies`). `Table("Row")` reads the `fn name/predicate/straddle/floor/row` bodies of `impl Row`, but also of `impl From<Row> for X`, `impl Display for RowLabel` (whole-word, so not that one), or any trait impl mentioning `Row` in its head. A label table in such an impl is read as names, which is the over-hold the declaration was meant to end. The head should name the type as the impl's self type (`impl Row`, `impl<T> Row<T>`, `impl Trait for Row`), not as any word of it.
- **S6. A declared table's non-literal arm is dropped silently** (`variant_arms` reads `Variant => "lit"` only). An arm spelling a const (`Self::LinkBend => geom_brep::TANGENT_SECOND_ORDER`) or an expression adds nothing, and nothing says so. Only sweep pins its tables against the types' `ALL`s, and its `GEOM_BREP_ARMS` list is that pin's answer to this (`crates/sweep/tests/recourse_roster.rs`, `the_census_reads_the_decision_tables_as_the_types_do`). profile pins its gate table (`the_census_reads_the_gate_table_as_the_type_does`). geom-brep's five tables, topo's `TangentRow` and editor-core's `decisions!` have no such pin, so a new non-literal arm there passes unseen. Either the table reader reports a non-literal arm (as `unreadable`, or as a const site held to a declared carrier), or each declaring roster pins its tables against the type's `ALL`.
- **S8. `CLOSED_TYPE` reasons are not checked against the declared tables.** editor-core's, geom-brep's and topo's rosters list `CLOSED_TYPE` sites (`decision.name()`, `row.predicate()`, …) whose reason says the census reads the names at the type's declared table. Nothing ties a listed site to a declared `Table`, so a site whose type is not declared, or whose table was dropped, still reads as answered.

**Done** when a table's impl match is by its self type, a declared table's non-literal arm is reported or pinned in every declaring roster, and each `CLOSED_TYPE` entry names the declared table that answers it, held by `roster_violations`.

**Also (6793 delta review):** a call handed two separate top-level tuples, `two_as(("tup_lit", m), (name, n))`, reads `tup_lit` and drops `name` with no site. That is the cost of "a value pair beside a literal is no batch". No such call exists today; every funnel takes one tuple or an array.
