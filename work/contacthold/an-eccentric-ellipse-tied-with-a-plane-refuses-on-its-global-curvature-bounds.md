---
id: an-eccentric-ellipse-tied-with-a-plane-refuses-on-its-global-curvature-bounds
kind: issue
title: An ellipse tied with a plane face refuses ConicBend on its carrier's global curvature bounds even where it clearly rises off the plane
status: open
opened: 2026-10-10
priority: P3
cost: M
refs: [boolean-conic-side-code-zero-is-first-order, 4575]
---


## Finding

From the dual review of PR 4575 (R1 and R2, sampled). `sectors::conic_side`
bounds an ellipse's lift with its carrier's GLOBAL curvature bounds
(`Bend::of`: `κ ∈ [b/a², a/b²]`) and caps the witness at
`l = min(extent, π/(2·κ_max))`. For an eccentric ellipse the global
`κ_max = a/b²` is huge, so `l` collapses and `κ_min = b/a²` is tiny,
and the certified lift is far below the arc's real one: an ellipse with
`b/a = 0.01` rising `5e-5` off the plane at its witness refuses
`ConicBend`.

The bounds that make the reading sound need only hold over the stretch
the reading reads, from the base vertex to `l`. Bounding `κ` over that
parameter interval (the ellipse's curvature is monotone between axis
ends) would certify the same arcs as a circle of comparable bend.
Reachability by a modelled body is unshown.

**Both lanes now.** The splitting lane reads an in-plane conic
departure through the same `geom_brep::conic_bend_side`
(`two-second-order-conic-readers-certify-one-fact-differently`), so
an eccentric ellipse departing in a split plane refuses `SliverSector`
on `conic_bend` where the old bare sagitta answered. A semi-axis ratio
of 5 still reads its bend in the split
(`crates/sweep/tests/a_plane_along_an_elliptic_cut.rs`,
`a_plane_tangent_along_the_ruling_reads_the_elliptic_edges_by_their_bend`).
The bounds live once, in `ConicBend::of`
(`crates/geom-brep/src/conic_bend.rs`).

## Merged in: `a-spiric-arc-tied-with-a-plane-always-refuses` (2026-11-25)

Folded in at the CONTACTHOLD cleanup (`docs/doc-ledger/contacthold-leaves-the-tracker.md`): the same mechanism, fixed by the same change. Its title was: A spiric arc tied with a plane face always refuses ConicBend, even lying in the face's plane. Its body follows, headings demoted.

### Finding

From the dual review of PR 4575. `Bend::of` gives a spiric carrier
`BendBounds::Unbounded`, so `sectors::conic_side` takes its lift as zero
and refuses `ConicBend` at every first-order tie with a plane face. That
includes a spiric arc lying in the face's plane, which a height bound
would read `On`, and one that clearly bends off. No modelled body is
known to reach it (a torus's plane section tangent to a partner plane at
a boolean vertex). It has no simple cure: `geom_brep::edge_extent`'s
map with derivative at least 1 bounds distances, not curvature, and a
spiric section can inflect, so it has no positive lower curvature bound
to give it the circle's reading. A cure reads the section's departure
some other way (e.g. a height bound for the in-plane case alone).

**The split cannot reach it.** The splitting lane reads an in-plane
conic departure through the same `geom_brep::conic_bend_side`
(`splitting/neighborhood.rs`, `split_conic_departure`'s Zero), but no
spiric edge gets there: the gate (`splitting/classify.rs`) refuses
`CurvedEdgeUnsupported` for a spiric edge unless `edge_clears` puts its
whole locus, or a face of it, off the plane's band, and `sector_face`'s
`Torus` arm refuses `CurvedBooleanUnsupported` behind it. The finding is
the boolean's alone; a cure lands once, in
`crates/geom-brep/src/conic_bend.rs`.

### The edge extent's new home (OFFSET, 2026-10-27)

`geom_brep::edge_extent` no longer exists (OFFSET, PR 5875). Its home is `geom_brep::extent`, beside the reach family, as three functions: `edge_extent_inf` (≤ the edge's diameter `D`: the chord, the minor-axis folds and the sampled pairs; the lever of a reading whose Zero only refuses), `edge_extent_sup` (≥ `D`; a reading whose Zero is acted on and whose definite signs only refuse) and `edge_reach_from` (the edge's farthest distance from a point of it; a reading that acts on every sign, read from that point). `ConicDeparture`, which this row's citation reads, levers at `edge_extent_inf`, which now also reads the spiric's sampled pairs.
