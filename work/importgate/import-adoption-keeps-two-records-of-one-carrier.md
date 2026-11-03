---
id: import-adoption-keeps-two-records-of-one-carrier
kind: issue
title: Import adoption reads two coincident surface records one carrier and keeps both
status: open
opened: 2026-11-03
---


`crates/step-import/src/adopt.rs`, the conventional rung of edge
adoption, describes an edge between two coincident surface records in
the first record's chart through `topo::rest_locus` (the carrier-equality
ladder over the faces' region). That is a verdict that two records are
one carrier, and DESIGN `[coincidence.door]` (ratified on PR 6427) has
it end in one record, the act that decided it re-pointing the other's
faces and recording why. Import keeps both records and records no row.

The primitive exists (`topo::Body::unify_surfaces`, crate-private;
`topo::Body::unify_across` is the public door that reads an edge's two
records, unifies them where its rest reads one chart, and returns the
row with `RowCell::Result` cells). What the import side lacks is a
settled home for the row: an import's cells are neither an operation's
inputs nor (by `RowCell::Result`'s current wording) a census decision,
and the document layer names neither for an import.
