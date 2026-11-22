---
id: a-touchs-crossing-or-side-is-re-decided-from-tangents-at-its-one-point
kind: issue
title: A touch's crossing-or-touch and its sides are re-decided from tangents at its one point
status: open
opened: 2026-11-16
priority: P1
---

Found by PR 6570's reader audit (answering Ev, 2026-11-10: "double check that all the ways it's being used are sound even when geometry varies by eps"), on main `0be17a13e`.

`curve_pair::meet` reports a touch as one point of its in-band arc. Most readers read only that the meeting exists, its hit kind, or its position within the band. Any point of the arc serves those, so they are sound when the geometry moves within ε. Three readers instead decide **whether the touch crosses**, or **which side each face lies on**, from the two curves' tangent directions at the reported point.

That angle is 0 at the extremum. It grows to about √(2zκ) at the arc's ends, which is definite at these readers' levers. So each answer is right only while the point is held at the extremum. PR 6768 holds it there on the coplanar paths. Across planes and on the walk it is unheld (`a-touch-point-read-across-planes-or-by-the-walk-carries-an-unheld-spread`). Even where it is held, the readers make a second reading of what `meet` decided (DESIGN "One reading"; [verdicts] "posed so the choice does not arise").

- `crates/topo/src/ring_path.rs:1717-1734`, `spline_crossings`: it decides the sine of the tangents at `(tp, ta)` past the band, levered at the piece's length. Off the extremum a touch reads Positive, counts as a crossing, and flips the parity. Spline arcs are always walk touches. (likely)
- `crates/topo/src/census.rs:5298` (pass 5′) → `confine` `:1307` → `in_overlap_closure` `:2004` → `cones_meet` `:2169` / `corners_meet` `:1797`: for two faces kissing with opposite facing, the edge tangents at q differ by sκ, so `corners_meet` reads a shared wedge. The result is `Some(true)`, and a real `UnrecordedContact` is suppressed. (likely)
- `crates/topo/src/census.rs:12355`, `shared_interior_witness` (the Pair probe → `cones_meet(m.point)`): it gives a false "faces share interior" witness by the same mechanism. (likely)
- Possibly `census.rs:12460` `arc_interiors_meet`, and `curve_face::located/cuts`, `boolean/overlap.rs` and `boolean/mod.rs:8672`. A neighbouring cut just outside the stretch puts a piece's midpoint probe inside the arc. (unsure, not traced to the probe)

**Fix (C, as recommended on PR 6570):** a `Meeting` carries its kind, `Crossing | Touch { arc }`, decided once by the pose or the subdivision. Readers that ask about crossing, touch or side read the kind. For a touch, where they need sides, they read the arc's two ends, never tangents at the point.

**Done when:** a row injects an off-extremum reported point (an arc end) for each reader above and its verdict is unchanged, and no reader outside `meet` decides crossing-or-touch from tangents at a meeting's point.

**Ev's reservation (PR 6570, 2026-11-16):** "i think this works, then. it still seems like it's not 'sound by construction' / has arbitrariness, but there may not be a solution that handles that better". The arbitrariness is which point of the in-band arc `meet` reports. Under C, no verdict may depend on that choice. When this row lands, check whether any reader still can, and say so in its PR. The aim is that the reported point is arbitrary only where nothing reads it.
