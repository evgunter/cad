---
id: an-ellipse-stored-minor-over-major-passes-tier-3
kind: issue
title: An Ellipse carrier stored with minor > major mints and passes tier 3; only the constructor decides the ordering
status: open
opened: 2026-09-25
priority: P4
cost: E
refs: [ATREST-13]
---


## What

`geom::Curve3::Ellipse`'s convention is `major > minor > 0`, and
`Curve3::ellipse` is the one constructor that decides the ordering
(`EllipseInvalid::CircularAxes` / `AxesSwapped`, on the band). Nothing
at rest does. ATREST-13 gave check 1 the `> 0` halves
(`geom::Curve3::representability_margins`,
`ValidationError::UnrepresentableCurveDatum`) and left the ordering out,
because it relates two datums rather than bounding one — the torus's
`R > r` shape, which check 1 decides through `decide` as a separate
verdict (`DegenerateTorus`).

**Measured (ATREST-13, the D-2 table):** a pillow chord re-minted
through `Body::set_edge_curve` as `Ellipse { major: 0.5, minor: 0.7 }`
(a half-arc between the chord's two vertices, chart-described) mints,
and `validate_geometric` is `Ok(())`.

A swapped pair describes an ellipse — the same locus with `u_ref` along
its minor axis — so this is not a representability refusal. What reads
the ordering: `topo::loop_winding::conic_segment_term`'s perimeter lever
was `|Δ|·major`, an arc-length UPPER bound only when `major` is the
larger semi-axis; ATREST-13 made it `|Δ|·max(major, minor)` because
check 6 now winds ellipse-bounded loops. **A second reader, not fixed**:
`geom_brep::certify::edge_extent` — certification's transversality arm,
re-used verbatim by tier 3's check 4 dihedral pass — takes the fold at
`minor` as a LOWER bound on the arc's diameter because "the ellipse
dominates its minor-radius circle"; with `minor > major` stored, the
circle at `minor` dominates the ellipse instead, the arm can exceed the
honest extent, and a larger arm is the unsafe direction (it can decide
where it should escalate). `geom`'s `Ellipse` docs used to
say tier 3 owns the ordering at rest; ATREST-13 corrected them to what
is checked.

## What must be decided

Whether the ordering is tier 3's to certify (a decided `major − minor`
margin and a variant of its own, D3's one-kind-per-configuration
argument: `major == minor` is a `Circle`), or a mint-only convention
every consumer must be robust to. A consumer sweep for readers that
assume `major ≥ minor` is owed either way.

## Fence

Track P. `crates/topo/src/validate.rs` (check 1); `crates/geom/src/curves.rs`
is `props` ground.
