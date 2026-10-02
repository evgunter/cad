---
id: an-ellipse-stored-minor-over-major-passes-tier-3
kind: issue
title: An Ellipse carrier stored with minor > major mints and passes tier 3; only the constructor decides the ordering
status: open
opened: 2026-09-25
priority: P4
cost: E
refs: [3238]
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

## Consumer sweep (REACH, PR 3805)

The readers that took a stored semi-axis as ordered or signed now read
magnitudes (`min(|major|, |minor|)` for a floor, `max` for a reach),
which is `minor`/`major` exactly for a frame in the ordinary order:

- `geom_brep::certify::edge_extent` (the second reader above) — fixed;
- `geom_brep::certify`'s ellipse span meter (`InfSpeed::new(minor)`) and
  `geom_brep::pcurve_cache::param_rate` — fixed;
- `editor_core::eval::measure::curve_reach` (`from(center) + major`) —
  fixed, with a row over every stored order and sign;
- `topo::replace_face::pose_reach` and `geom_brep::implicit`'s harmonics
  and bounds (`Conic::speed_lo`/`speed_hi`) — fixed earlier in the PR.

The ordering itself is still decided only by the constructor; this
item's question (whether tier 3 should decide it) is unchanged.

### More readers (REACH, PR 3805 fix pass 4)

- `topo::split` (the split's interiority meter) and
  `topo::splitting::classify::conic_plane_crossing_roots` (the crossing
  root's end meter) took `InfSpeed::new(minor)`: with `minor > major`
  stored that OVER-states the speed floor, so a root 5e-10 m of arc from
  an end read 5e-7 m and was certified interior
  (`a_crossing_at_an_end_is_metered_at_the_smaller_semi_axis`, red on
  the old read). Both now read `min(|major|, |minor|)`.
- `mesh::sizing::ellipse_step` assumed `major > minor`: swapped, its
  `R_eff` was far below the true bound (7776× for semi-axes 3 and 0.5,
  a step √7776 ≈ 88× too long before the angular cap), and a negative
  `major` gave `NaN` (taken as the angular cap). It now orders the magnitudes
  (`the_ellipse_step_reads_its_semi_axes_in_any_stored_frame`).
- **The certify gate is not widened.** Before PR 3805 its span meter was
  `InfSpeed::new(minor)`, so an ellipse stored with a negative `minor`
  was refused `IntervalNotForward` (incidentally); reading
  `min(|major|, |minor|)` admitted it (measured: the plane ∩ leaning
  cylinder ellipse with `minor = −1` certified). Ruled in review: the
  meter is now the smaller SIGNED semi-axis, so either order meters and
  a non-positive semi-axis — `minor` or `major` — is refused
  (`ellipse_signed_semi_axis_gate`). Whether a swapped or signed frame
  should be normalised at the mint stays this item's question.
