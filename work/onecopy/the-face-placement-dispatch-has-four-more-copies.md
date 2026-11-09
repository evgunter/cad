---
id: the-face-placement-dispatch-has-four-more-copies
kind: issue
title: five more sites spell 'where does this face hold a point on its carrier' (plane -> contfp, curved -> curved_face_containment or _placement) beside face_placement, and a class splits the decided door pair (contfp_decided / curved_face_placement_decided) that face_placement has no form of
status: open
opened: 2026-11-09
priority: P3
cost: M
---


## Finding

Filed by `onecopy/face-holds`, from the sweep behind
`a-face-holds-a-point-has-no-public-door`. That row's two copies now read
`boolean::contain::face_placement`, the one dispatch. A sweep for the same
shape (a `Surface::Plane` or planar-normal arm calling `contfp` or
`contfp_decided`, beside an arm calling `curved_face_containment` or
`curved_face_placement`) found five more. Each reads a point it already
holds on the face's carrier, so it skips the on-plane decision that
`face_placement` makes:

- `census.rs` `placed` (~1929): the planar arm reads the census's cached
  plane (`planar_face(geo)` → `contain`); the curved arm goes through
  `read_containment`.
- `boolean/fate.rs`, a fate cut's end in a divided face's pieces (~1146):
  the arm is chosen by `face_outward_normal` (sense-folded), and refusals
  map to `contained(.., "a fate cut's end is not placed in its face")`.
- `boolean/fate.rs`, the ring-vertex ladder (~1366): the same dispatch.
  Refusals go by their standing (`ContainError::refusal`, `rung_or_abort`),
  and `StaleFace` goes to `driver_face_stale`.
- `cell_region.rs` `place` (~186): every refusal is dropped with `.ok()`.
- `boolean/carrier_touch.rs` `ball_off_face` (~325): `Surface::Plane` →
  `contfp_decided` (its margin dropped), else `curved_face_placement`
  (PR 6599's review).

**The deeper copy is a decided placement dispatch.** `face_placement`
has no form that keeps the boundary verdict's margin (`contfp_decided`,
`curved_face_placement_decided`). So the sites that need the margin
split plane from curved over that door pair, in separate functions
rather than one match. Sweep: callers of `contfp_decided(` and
`curved_face_placement_decided(` outside `contain.rs`.

- `boolean/vtxfac.rs` (~1921 curved, ~1971 plane);
- `boolean/reduce.rs` (~5723 curved, ~5906 plane; and the planar-only
  ~1329, ~1472, ~1779; the curved-only ~3346, ~5161).

**The fix that retires both kinds** is a decided form of
`face_placement`, which the plain door projects, read by every site
above.

- **Its result is the curved door's triple**, `ChartedPlacement`
  (`boolean::contain`: the placement, the margin that decided a boundary
  verdict, and a spline face's in-trim chart foot, which
  `boolean::reduce` ~5723 reads). A pair would drop that foot.
- **On a plane it decides the on-plane margin itself and returns it**,
  so no caller decides it again: today the planar halves (`reduce.rs`
  ~5906, `vtxfac.rs` ~1971) are handed a margin the caller already
  decided, and the decided door would take that decision's place.

Each of the five keeps its own error policy, and that may be right: the
census reports findings, the fate ladder ranks refusals, and a cell
station refusing is no reading. What is copied is the dispatch.

**A carrier-held point should be a type, not a convention.** A point
held on the face's carrier by construction refuses on `OffCarrier`
rather than reading it as `Out` (`CurvedPlacement`'s doc), as
`rim_wedge` does: it returns `ClassificationInvariant`, through its
`RimUnread::Fault`. Left as a per-caller rule, five sites would each
spell that again. The proposal is a point type that says it is held on
its carrier, e.g. `OnCarrier<T>(Point3<T>)`, minted only where the
construction holds it (a root, a locus point on the face's carrier, a
vertex of the face). `face_placement` reads it and still decides the
on-plane margin once (decide and fault, not trust and skip), with a
definite `OffCarrier` mapped to the typed fault in one place, carrying
that decided margin. A
point not so held keeps today's door, where `OffCarrier` is a reading.

Check first that a fate site's sense-folded normal and the plane's
stored normal give `contfp` the same reading.
