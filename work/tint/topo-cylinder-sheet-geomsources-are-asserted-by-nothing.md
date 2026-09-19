---
id: topo-cylinder-sheet-geomsources-are-asserted-by-nothing
kind: issue
title: mate5's cross-instance GeomSource fingerprint — node, minted index and pairwise distinctness — is documented at three suites and pinned by no row
status: open
opened: 2026-09-19
---


## Finding

Three `crates/topo/tests` suites build cylinder-wall sheets whose
cylinder key is given a distinct `GeomSource` per sheet, and each says
in its own prose that this is the thing under test — the mate5 suite's
builder doc calls it *"the cross-instance fingerprint (`same_chart`'s
'distinct GeomSources' arm)"*.

**Three mutations, three greens.** Measured 2026-09-19 at merge base
`5b4979ef2`, `cargo test -p topo --features interval`, over the shared
builder `topo::test_support::cyl_wall_sheet`:

| mutation | integration rows red |
| --- | --- |
| the source write deleted — no sheet's cylinder key carries one at all | **0 of 617** |
| `GeomSource::minted(source, 0)` → `minted(source, 7)` — every sheet's minted INDEX changed | **0 of 617** |
| (both leave `744` lib rows green too, apart from the builder's own) | |

So neither field is evidence at any call site. The sheets refuse for
some other reason — each is built in its own `Body`, so no structural
chart identity can exist between two of them in the first place — and
the `src_id` argument at every call site is inert.

**Three claims, none of them asserted**, and a row about only the first
would be a half-fix:

1. the `node` field carries the id the caller named;
2. the `expr` field is `Minted { index: 0 }` — the index has never been
   read by anything, anywhere;
3. **two sheets in a pair carry DISTINCT sources**, which is the
   property the prose actually names and which no single-sheet row can
   reach at all.

Sites, all reaching the builder through `topo::test_support::cyl_wall_sheet`:
`crates/topo/tests/mate5_cyl_eps_rung.rs` (6 f64 sites + 2 in
`interval_lane`), `crates/topo/tests/r1_mate5_probe.rs` (2),
`crates/topo/tests/r2_probes.rs` (8).
`crates/topo/tests/split_edge_pcurve_rows.rs` passes `None` and claims
nothing.

## What is now pinned, and what is not

The builder's own row
(`test_support_fixtures::tests::a_sheet_is_three_solids_with_a_shared_key_and_a_reversed_top_rim`)
now asserts the WHOLE `GeomSource` the door mints —
`assert_eq!(body.surface_source(cyl), Some(&GeomSource::minted(11, 0)))`
— so claims 1 and 2 are pinned at the door, and both mutations above
red there. It builds ONE sheet, so claim 3 is untouched by it and
remains asserted by nothing in the tree.

The deliverable here is claim 3: a row that builds a pair, removes or
equalises the fingerprint, and shows the verdict CHANGE. If no such row
can be written — because the separate arenas already decide it — then
the prose at those three suites is wrong about what it is testing, and
deleting the prose is the fix.

## Why this row is on this slate

`crates/topo/tests/*` is this program's territory with S-TCOST's, and
a suite whose stated subject no assertion can reach is S-TINT's
charter. Found by the S-DUP unit
`the-cylindrical-patch-rim-builder-is-written-nine-times`, whose
mutation probe it is.
