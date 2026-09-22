---
id: a-split-circle-fixture-sits-inside-the-1e-6-escalation-band
kind: issue
title: A pane::profile fixture's chord-side margin lands in the 1e-6 escalation band and the row fails at that eps
status: review
branch: chrome/split-circle-eps
opened: 2026-09-21
priority: P2
cost: E
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

## Resolution (on `chrome/split-circle-eps`)

**No constant radius clears all three bands**, and that is measured,
not argued. The figure is conditioned purely relatively: the
`segment_straightness` sagitta `r·(1 − cos(π/n))` and the
`carrier_circles_identity` floating-point residue are both
proportional to `r` and both read against (ε, K·ε), so the admissible
radii are an interval in ε — `[2.13e6·ε, ~1.3e12·ε]`, measured
identically at ε = 1e-6 and ε = 1e-12 by scanning the fixture's
radius. At `n = MAX_CIRCLE_SPLIT + 1` that interval does not contain
a constant: ε = 1e-6 wants `r > 2.13 m` (measured: OK from 2.512 m,
in-band at 1.995 m) and ε = 1e-12 wants `r < 1.5 m` (measured: OK up
to 1.45 m, in-band from 1.50 m). The two windows are disjoint.

Dropping to the chord regime does not help either, and cannot: below
ε the sagitta reads as a chord, and then the nearest rung of the
polygon's own `chord_side` ladder is `8s`, which is still under K·ε
because 8 < K.

So the radius is tied to the run's ε: `1.5e9 · Tol::witness().get().eps`,
which stands ~700× clear of both walls at every ε, with the
derivation written beside it in the fixture. Verified green at
ε ∈ {default, 1e-6, 1e-12} and, because the fixture is now
scale-derived, at 1e-5, 1e-7, 1e-8, 1e-10, 1e-11 and 1e-13 as well.

The sibling found while sweeping is
`work/chrome/the-circle-split-cap-offers-counts-the-document-refuses.md`:
the same two walls, on the product surface rather than a fixture.

