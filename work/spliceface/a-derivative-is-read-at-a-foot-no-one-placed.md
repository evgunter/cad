---
id: a-derivative-is-read-at-a-foot-no-one-placed
kind: issue
title: A spline chart's derivative is read at a projected foot that may lie on another sheet, at an irregular point, or across a crease
status: open
opened: 2026-10-26
priority: P2
cost: M
refs: [the-side-of-a-spline-face-is-read-by-no-dihedral, a-point-has-no-place-in-a-nurbs-faces-trim, nurbs-surface-project-accepts-a-zero-speed-stationary-point, the-plane-nurbs-transversality-reads-a-jet-on-a-crease]
---



Filed by the SPLICE lane that built `geom_brep::dihedral_side` (PR
5764), on its re-review: the class its counterexamples belong to.

`NurbsSurface::project` returns the carrier's nearest stationary point.
That foot is not placed in any face, its chart may be irregular there,
and it may sit on a C⁰ line. A derivative read at it (a normal, a
curvature, a transversality sine) is the face's only where all three
are ruled out. The sweep: `.project(` / `project_from_seed(` /
`chart_foot(` / `surface_foot(` followed, at the foot's parameters, by
`.ders(` or `ders_in_span(`, in `crates/geom-brep/src` and
`crates/topo/src`, fuzz and row modules excluded.

| hit | reads | disposition |
|---|---|---|
| `geom-brep/src/dihedral_side.rs` `spline_normal` (~260-310) | the normal and κ at the foot | the irregular point and the crease are guarded (`irregular_reach`, `on_a_crease`). The other sheet is not: the door's contract says the foot is the carrier's, and the caller places it (`a-point-has-no-place-in-a-nurbs-faces-trim`) |
| `geom-brep/src/edge_nurbs.rs` `plane_nurbs_limbs` (~646-656, feet from `chart_image` ~1966) | the wall's normal sine at each schedule foot | unguarded on all three; the crease half is filed as `select/the-plane-nurbs-transversality-reads-a-jet-on-a-crease` |
| `geom-brep/src/edge_nurbs.rs` `nurbs_pair_limbs` (~1281-1297) | both walls' normals at their feet | as above |
| `geom-brep/src/pcurve_cache.rs` `chart_foot_lane` (~2970), `topo/src/pcurves.rs` `derive_chart_foot` (~1214, ~1336) | the foot's position only, as an iso column | not this class (no derivative). A wrong-sheet foot places a wrong column, and the image's own certificate then refuses it |
| `geom-brep/src/ssi/handle.rs` `foot` (~66) | a Newton seed | not this class: the march certifies what it converges to |

**The first-order narrowing** (`dihedral_side`'s module docs, "The
foot's offset is charged to first order"): the normal at the foot
stands for the normal at the point, a distance inside the band away,
with κ read pointwise at the foot. A curvature rising steeply across
that gap is not covered. A bound on the patch cell's curvature
(`patch_bound`'s `s_uu`…`s_vv` enclosures over the cell holding both)
would turn it into a statement over the gap.

**Blind spot:** a derivative read at a foot under another name (a jet
cached from the projection, `SurfaceProjection::foot` reused as a
seed). A second pass on `\.foot\b` and `jet` near a projection is owed.

Moved from SPLICE to SPLICEFACE on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).
