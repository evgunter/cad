---
id: a-split-circle-fixture-sits-inside-the-1e-6-escalation-band
kind: issue
title: A pane::profile fixture's chord-side margin lands in the 1e-6 escalation band and the row fails at that eps
status: dispatched
opened: 2026-09-21
priority: P2
cost: E
branch: chrome/split-circle-eps
---


## Finding

`crates/viewer/src/pane/profile.rs`,
`tests::drawing_a_locked_split_circle_above_the_cap_leaves_it_alone`.
It builds a `circle_split` of radius `0.01 m` with
`MAX_CIRCLE_SPLIT + 1` segments and `expect`s the document to admit
it — *"the document admits a split circle above the form's cap"*.

Under `CAD_TOLERANCE_EPS=1e-6` the document does not admit it. The
`chord_side` predicate answers a margin of `1.1272608421844756e-6`
against `Band { zero: 1e-6, escalate: 9.999999999999999e-6 }`, so the
pair escalates as `Indeterminate` and `apply` refuses. The row then
panics on its own `expect`.

Passes at the default eps and at `1e-12`. Executed on `origin/main`,
twice — `cargo test -p viewer --features app --lib` and
`cargo nextest run -p viewer --features app -E
'test(drawing_a_locked_split_circle)'`.

**The row is right and the fixture is the problem.** Nothing about
"drawing an over-cap split circle leaves it alone" is a claim about
tolerance; the fixture just happens to sit a tenth of a decade inside
one eps row's escalation band, because a `0.01 m` circle cut into
`MAX_CIRCLE_SPLIT + 1` pieces puts its chords there. A fixture whose
scale is near a gated eps row's is a fixture about that row. The fix
is a radius (or a count) that clears all three bands, and saying in
the row why the number was chosen.

## Why nobody had seen it

The viewer's `#[cfg(feature = "app")]` rows run in exactly one CI step
and that step carries no `CAD_TOLERANCE_EPS`, so they are gated at the
default row only —
`work/ciw/the-viewer-app-feature-rows-gate-one-eps-of-three.md`. This
row is live on `main` today and the gate is green.

## Fence

`crates/viewer/src/pane/profile.rs` — chrome's and view's. Filed from
`vgeom/sketch-infinity`, which found it by running the viewer suite at
all three eps rows after CI reded one of its own rows at `1e-6`.
