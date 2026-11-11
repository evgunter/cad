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

## Witness: dm1's refusal moved (2026-11-11)

With `rest_locus` reading keys only (PR 6505, fix pass 2), the corpus
file `tests/fixtures/wild/stepcode/dm1-id-214.stp` still refuses, but
earlier: at the pcurve re-mint on the adopted body ("the circle carrier
is on the nurbs chart but is none of its own curves"), where main
refuses at the arc-rim pick ("not an arc rim of this chart"). Red on the
branch, green on main `ab4fc6ca15`: `step-import`'s
`tier_gate::every_corpus_import_passes_the_shared_gate` and
`r1_dm1_probe::{dm1_no_longer_refuses_at_the_instancing_gate,
the_l_bracket_alone_adopts_its_reversed_slit}`. The likely cause, not
traced edge by edge: an edge the adoption used to rest in one record's
chart, reading the two records one carrier, now rests on their pair. Converting adoption to leave such
faces on one record (the act, `[topo.one-record]`) is what restores the
frontier.
