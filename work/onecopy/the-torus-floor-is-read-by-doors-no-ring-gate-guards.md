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

**The type holds the premise.** The floor on `|F| / |res|` is now `geom_brep::ring_floor(ring: geom::RingTorus)`, readable only with the convention's witness. `ConicTorusHarmonics` carries no floor. The harmonics stay total (`conic_form_sup` reads them on any torus, for the form's sup), and the floor is the one reading that leans on the ring. So "the floor reports when its premises fail", one option this row's question named, became moot: a floor with no witness cannot be read.

**The doors ask the convention's home and carry its refusal.**
- Both doors call `TorusRefusal::require`, which reads `geom::RingTorus::of`, before any pose. The circle door's coaxial and parallel poses used to answer first: the parallel pose answered a negative-minor torus `Miss`, wrongly, and `r = 0` escalated `Invalid` there.
- A torus outside the convention is `DoorRefusal::Torus(TorusRefusal)`, carried whole as the plane × torus section carries it. Its standing is the convention's own (`TorusRefusal::refusal`: undecided ranked by its margin, decided out invalid input, which DESIGN's `[failure.taxonomy]` row 1 types).
- topo ends it at the torus face: `BooleanError::of_torus_refusal` gives `DegenerateTorus` or `Escalated` under `BooleanDecision::Torus`, which is the carrier ladder's existing ending, lifted into one place. The trace ends it as `TraceFault::Torus`, and editor-core says it in the convention's words, ended by the half's lever.

**The reach, corrected.**
- `offset_surface` decides the ring on the realized minor radius and refuses `TorusRing` (`crates/geom-brep/src/offset.rs:420-431`).
- The blend's coaxial arm **can** mint a torus of negative major radius that passes predicate 3. That is filed as `a-coaxial-blend-arm-mints-a-negative-major-torus-predicate-3-passes`. An earlier version of this note said blend tori were all refused before use, and that was wrong.
- Whatever else reaches the doors, the type now holds the floor's premise there.

**Measured**, on the first version of this change, with a scratch probe that is not kept: 1200 circles and ellipses against spindle, horn, horn-within-band, no-tube, negative-minor and tiny (`1e-100` m) tori, with ring controls. The non-ring tori had certified or missed, and none of those counts contradicted a 20000-sample oracle, but the probe's poses missed the parallel-pose `Miss` the review found.

Pinned by `curve_face::ellipse_torus::torus_rows::the_torus_doors_read_the_conventions_ring`: every non-ring torus, at the circle door's crossing, coaxial and parallel circles and at the ellipse door, refused with the convention's verdict; an in-gap ring undecided; a ring answering at each pose.
