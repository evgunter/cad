---
id: import-adoption-keeps-two-records-of-one-carrier
kind: issue
title: Import adoption reads two coincident surface records one carrier and keeps both
status: open
opened: 2026-11-03
---


`crates/step-import/src/adopt.rs`, the conventional rung of edge
adoption, describes an edge between two coincident surface records
where `topo::rest_locus` says it rests. That reads keys only now, so the
edge rests on the two records' pair: two records at rest are two
surfaces (DESIGN `[coincidence.door]`). Where the file's two faces are
one carrier, import keeps both records and records no row; a verdict
that they are one would end in one record (`crates/topo/README.md`
`[topo.one-record]`).

The act exists (`topo::Body::one_record`, crate-private, consuming a
token its deciding reading mints; `topo::Body::one_record_across` is the
public door a sweep takes for a smooth join it read, returning the row
with `RowCell::Result` cells). What the import side lacks is a
settled home for the row: an import's cells are neither an operation's
inputs nor (by `RowCell::Result`'s current wording) a census decision,
and the document layer names neither for an import.

