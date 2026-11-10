---
id: the-torus-floor-is-read-by-doors-no-ring-gate-guards
kind: issue
title: The circle and ellipse × torus doors read conic_torus_harmonics' floor without the ring decision that makes it a floor: subnormal on radii near 1e-100 m, and not positive off the ring
status: closed
opened: 2026-11-10
priority: P2
closed: 2026-11-10
branch: onecopy/torus-floor-ring-gate
pr: 6777
---

Filed from PR 6761's review (the torus harmonics charge), on its one finding that matters.

## What

`conic_torus_harmonics`' floor `f_per_metre_lo`, `2r(R − r)(R + r)` padded down (`crates/geom-brep/src/implicit.rs`, `ConicTorusHarmonics::f_per_metre_lo`), is a floor on two premises:

- **A ring:** `R > r`. Off the ring the product is zero or negative.
- **A product in f64's normal range.** With radii near `1e-100` m the product goes subnormal, its rounding is no longer relative, and the floor can stand above the exact value or read zero. The review measured this in 1365 of 13000 draws, many of them zero.

A ring the band certifies meets both. `geom::RingTorus::of` decides `r` and `R − r` against a band whose ε is at least `COORDINATE_FLOOR` (about 1.1e-13 m), so the floor there is at least `4ε³`.

Two doors read the floor without that decision:

- `crates/geom-brep/src/curve_face/ellipse_torus.rs:152`, `torus_walk`: `f_per_metre: h.f_per_metre_lo` in its `SubdivisionFrame`, reached from `ellipse_torus_roots` on any `Surface::Torus`.
- `crates/geom-brep/src/curve_face/circle_torus.rs:368`, the ladder walk: `f_per_metre: h.f_per_metre_lo`. Its `geom::ring_torus` test at `:291` gates only the meridian's special pose, and a torus off D3's convention "takes the ladder".

**Reachable, by the public API.** `circle_torus_roots` and `ellipse_torus_roots` are `pub`, and step-import's `TOROIDAL_SURFACE` carries `R ≤ r` verbatim (`crates/geom/src/surfaces.rs:223-231`). That was found in PR 6761's review. Production callers run on rings that tier 3 has validated. **Check first:** tori minted mid-operation, before at-rest validation, may not be such rings. The candidates are `offset_axial`, and blend tori that reach `reduce`.

A floor that stands above the exact one understates `noise / f_per_metre`, so a root can be placed tighter than its rounding allows. A zero floor divides by zero.

## The question

Should these doors decide the ring (`RingTorus::of`) before reading the floor, and refuse or escalate off it? Or should the floor itself report when its premises fail (a zero floor with a refusal behind it, or an interval)? Off-convention tori take the ladder today, so the gate changes what those tori answer, which is why it is not done in the PR that found it.

## Resolved (2026-11-10)

**The named reach is refused before any door.**
- `geom_brep::offset::offset_surface` decides `geom::ring_torus` on the realized minor radius and refuses a non-ring (`OffsetError::TorusRing`, `crates/geom-brep/src/offset.rs:420-431`). Every `offset_axial` torus is minted through it.
- A blend torus with `0 < s ≤ r` is refused `SpineIrregular` at predicate 3, and one with `s² < 0` escalates on poison, both before its surface is used (`crates/sweep/src/blend/arms.rs:443-455`).
- `reduce` mints no torus; it only matches them.

**The two doors ask the convention's one home.** `DoorRefusal::ring` (`crates/geom-brep/src/curve_face/circle_roots.rs`) reads `TorusRefusal::require`, which is `geom::RingTorus::of`:
- an undecided half becomes `Escalated`, under the convention's own row;
- a torus decided outside the convention becomes `Invariant`, a torus the door does not read.

The circle door asks before its meridian pose. That pose had decided `geom::ring_torus` again on its own, and that check is gone. The ellipse door asks at the head of its walk. The coaxial poses read no floor and keep answering first.

**Measured, before and after**, on a scratch probe that is not kept: 1200 draws of circles and ellipses against spindle, horn, horn-within-band, no-tube, negative-minor and tiny (`1e-100` m) tori, with ring controls, checked against a 20000-sample sign-change oracle.
- Before, the non-ring tori certified or missed, with no count the oracle contradicted, and a negative minor radius read `Unsettled`.
- After, every decided non-ring reads `Invariant` at both doors. The ring controls are unchanged (certified or miss, as before).
- The tiny torus's circles still read `On`, from the coaxial pose, which reads no floor.

The row `curve_face::ellipse_torus::torus_rows::the_torus_doors_read_the_conventions_ring` pins each case, at both doors.
