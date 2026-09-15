---
id: rate-pair-in-geom-core
kind: unit
title: SupSpeed and InfSpeed beside Margin: metered takes the inf, a sup door for overshoot metering, the three blurred sites typed
status: open
opened: 2026-09-15
branch: scalar/rate-pair
pr: 2657
---


## What

`D283`'s ruling, route A (PR 2457), first unit. Two types in
`crates/geom-core/src/predicate.rs` beside `Margin`: `SupSpeed` and
`InfSpeed`, meters per parameter unit, one operation each way
(`span · s` to meters, `m / s` to parameter units — bit-identical to the
bare arithmetic, D9), a surface being a pair. `Margin::metered` takes
`InfSpeed` (its doc's promise made a type) and gains the sup-side
sibling for overshoot and escape metering; the three sites that pass a
sup through `metered` today (`pcurve_cache::trim_containment`, the
`pcurve_iso_*` slack meters, `topo::pcurves::pcurve_loop_continuity`'s
v-channel — PROPS' row
`metered-margin-doc-promises-an-inf-bound-three-sites-pass-a-sup` and
TRIM's `loop-continuity-meters-u-through-levered-and-v-through-metered`)
move to it; `PatchRegularity`'s speeds and `plane_nurbs_ssi`'s local
speed become `SupSpeed`. The type is a dimension-and-direction tag, not
a positivity witness: the 0/∞ guards stay at their sites. Out of scope
by the ruling: second-order and param→param rates (mesh sizing,
chords), pointwise jet speeds (the march, Newton acceptance),
`levered_inv`'s curvature uses, `gap_is_noise`'s zero lever. Evidence:
`work/scalar/log.md`, "The rate census". PROPS' and TRIM's ground;
announce. Full v6 dual.

## Digest receipt (taken before the change)

The D9 pin for "nothing's bits move". Taken on this branch at the
merge with `origin/main`, with the tree still at the pre-change state,
so the hashes are in history before a line of the change was written.

`cd demos/tour && cargo run --release -- <out>` (release, default
features, default eps), then
`find . -type f | LC_ALL=C sort | xargs sha256sum`:

- **1766 emitted files** (`*.pncad`, `*.stl`, `*.step`, `uv/*.svg`,
  `uv.json`, `scenes.json`); digest of the sorted per-file digest
  listing: `87be4dd9df4cc3af9bd44593a6b981608c8e73721c746413a00322ff61e4a892`
- the tour's own **narration** (the census, genus, validation tiers and
  exact-vs-meshed mass properties it prints per stop), from the first
  scene line to the end:
  `0450e8ba80a473c12adfb9efc768e6cc0b09bcebf31c0149156487b93824a33e`

The same two hashes are re-taken after the change in the PR body. The
listing itself is a one-shot comparison artefact and is not committed
(`memories/test-suite-cost.md`): the aggregate hash is what a second
run has to reproduce.

**Correction to the narration recipe, at the re-take.** The narration
hash above was cut at a fixed line number and kept the absolute output
path, so it digests the run's own `cargo` preamble and its outdir name
as well as the kernel's words — neither of which is a statement about
the kernel, and both of which differ between two runs whatever the
code does. The recipe that reproduces is: take the log from the line
after cargo's `Running` line, drop the harness's own trailing lines,
and rewrite the outdir name to a constant. The **pre-change log, the
one captured above and unmodified**, re-digests under that recipe to
**728 lines**,
`8522886427381545d312aafd1ff6a1a6757ea4d6500c47278036c2d847ab6712` —
and so does the post-change run. The file-listing hash needed no
correction and is unchanged.
