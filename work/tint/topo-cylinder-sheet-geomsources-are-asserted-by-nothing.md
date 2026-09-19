---
id: topo-cylinder-sheet-geomsources-are-asserted-by-nothing
kind: issue
title: mate5's cross-instance GeomSource fingerprint is documented at three suites and pinned by no row
status: open
opened: 2026-09-19
---


## Finding

Three `crates/topo/tests` suites build cylinder-wall sheets whose
cylinder key is given a distinct `GeomSource` per sheet, and each says
in its own prose that this is the thing under test — the mate5 suite's
builder doc calls it *"the cross-instance fingerprint (`same_chart`'s
'distinct GeomSources' arm)"*.

**Nothing reds when it is not recorded.** Measured 2026-09-19 at merge
base `5b4979ef2`: with `set_face_surface`'s source write deleted from
the shared builder — so every sheet's cylinder key carries no source at
all — `cargo test -p topo --features interval` stayed **fully green at
617 integration rows and 744 lib rows**. The sheets refuse for some
other reason (each is built in its own `Body`, so no structural chart
identity can exist between two of them in the first place), and the
`src_id` argument at every call site is inert as evidence.

Sites, all reaching the builder through `topo::test_support::cyl_wall_sheet`:
`crates/topo/tests/mate5_cyl_eps_rung.rs` (6 f64 sites + 2 in
`interval_lane`), `crates/topo/tests/r1_mate5_probe.rs` (2),
`crates/topo/tests/r2_probes.rs` (8).
`crates/topo/tests/split_edge_pcurve_rows.rs` passes `None` and claims
nothing.

## What is now pinned, and what is not

The builder's own row
(`test_support_fixtures::tests::a_sheet_is_three_solids_with_a_shared_key_and_a_reversed_top_rim`)
asserts that the door records the source it is handed, so the
mutation above is no longer silent **at the door**. What is still
unasserted is the suites' claim: that the distinct sources are what
makes `same_chart` refuse these pairs. A row that removes the
fingerprint and shows the verdict CHANGE is the deliverable; if no
such row can be written, the prose is wrong and the fix is to delete
it.

## Why this row is on this slate

`crates/topo/tests/*` is this program's territory with S-TCOST's, and
a suite whose stated subject no assertion can reach is S-TINT's
charter. Found by the S-DUP unit
`the-cylindrical-patch-rim-builder-is-written-nine-times`, whose
mutation probe it is.
