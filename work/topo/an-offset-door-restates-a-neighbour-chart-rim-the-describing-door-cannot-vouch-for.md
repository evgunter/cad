---
id: an-offset-door-restates-a-neighbour-chart-rim-the-describing-door-cannot-vouch-for
kind: issue
title: an offset door restates a rim described in a held neighbour's chart verbatim, and the describing door refuses the sound curved move it bounds
status: closed
opened: 2026-10-05
priority: P2
cost: M
refs: [a-listed-spec-cannot-name-a-fresh-chart-a-neighbour-keeps-the-old-key-of, boundary-on-the-new-chart-has-two-homes-in-the-attach-doors, validate-tier3-curved-boundary-containment, the-planar-offset-door-keeps-a-held-neighbours-chart-image-the-moved-edge-has-left]
pr: 4060
branch: topo/moved-boundary-one-home
closed: 2026-10-05
---

## What

Found in review of `boundary-on-the-new-chart-has-two-homes-in-the-attach-doors`
(PR 4060), which made `Body::set_face_surfaces_describing` refuse an
edge on a moved face whose description names no key the face wears
after the move, onto a curved chart
(`RechartUnvouched { door: SetFaceSurfacesDescribing, .. }`,
`Body::unvouched` in `crates/topo/src/attach.rs`).

The offset doors list a spec for every boundary edge and reach that
door through `replace_face::move_points_then_rechart`. Two restate arms
keep a chart image on the chart it names:

- `crates/topo/src/offset_axial.rs` `restate`,
  `EdgeDescription::Chart(c) => EdgeDescriptionSpec::Chart { surface: c.surface, image: None, .. }`
  (`offset_charts_together`);
- `crates/topo/src/replace_face.rs`, the `EdgeDescription::Chart(ref c)`
  arm of the per-edge plan (`replace_face_offset` /
  `replace_faces_offset`), which also keeps the image. No witness goes
  through this arm: a kept image may fail certification before the
  vouch is asked.

Where that chart is a NEIGHBOUR's that holds still while the face moves
onto a fresh curved chart, the listed spec names only the neighbour:
`Sides::repoint` maps the moving face's old key, and this spec never
names it. So no key vouches for the moving side, no curved residual is
read, and the move is refused, although it is sound: the offset door
checks the moved edge against both moved surfaces itself. The caller is
the offset door, not a user, so the refusal's lever ("list a
re-description of each on the new chart") is not one anybody can pull.

Witness: `crates/sweep/tests/offset_restates_a_neighbour_chart_rim.rs`
`a_rim_in_the_caps_chart_refuses_the_walls_offset`. A revolved drum
whose bottom rim is re-described (public `Body::set_edge_curve`) as an
image in the bottom cap's chart, with the wall offset inward and the
caps held by `offset_charts_together`, is refused
`ReplaceFaceError::Op { error: RechartUnvouched { door: SetFaceSurfacesDescribing, edges: [rim], .. } }`.
The same test on `origin/main` before PR 4060 returned `Ok(())`.

How else it is reached: `crates/step-import/src/adopt.rs`'s mapped-curve
rung can adopt an edge on `fs_plus`, which may be the neighbour of an
analytic curved wall. None of the 57 step fixtures, and none of the
native constructors the PR 4060 review probed, produce one: their
analytic curved faces' edges are own-chart or intersections, and
neighbour-chart sides appear only on NURBS loft walls, where the fit
lane refuses first (`FittedBoundaryUnsupported`).

This is a different shape from
`a-listed-spec-cannot-name-a-fresh-chart-a-neighbour-keeps-the-old-key-of`,
which is a spec naming a key the neighbour keeps. Here the spec names
the neighbour's own chart, and is right to.

## Shapes a fix could take

- **Restate it as the intersection it now is.** Where the face a chart
  image does not name moves, the offset door could state the edge as the
  intersection of the two post-move surfaces (the door already checks
  the carrier on both), or add the moving face's chart to what it lists.
  That is local to the two restate arms.
- **#638's curved residual**
  (`work/restfront/validate-tier3-curved-boundary-containment`): read
  curved residuals where a band is in hand, as the plane arm does. That
  closes this and the sibling row together.

## Closed 2026-10-05

**The first shape was taken, in both arms, inside PR 4060.** Where the
chart an image names holds and the edge's other face is re-minted, the
door states the edge as `Intersection(moving, held)`; its moving key is
one `Sides::repoint` maps to the minted chart, so the describing door
vouches for it by key and certifies the section. A declaration rides
only a chart image, so a declared edge moves into the moving face's own
chart instead, its declaration re-authored (`offset_axial.rs`) or
translated (`replace_face.rs`) as before.

- `offset_axial.rs` `restate` (with `beside_reminted`, and
  `MovedChart::rekeyed`, which the mutation phase's "a chart asked to
  move nothing keeps its chart" now shares).
- `replace_face.rs` `plan_edge`'s neighbour-chart arm. That arm kept the
  image verbatim, so certification refused the move (`ChartResidual`)
  before the vouch was asked, on `origin/main` too; it now goes through
  the intersection arm's own route and pose gates
  (`neighbour_section`).

Witnesses, `crates/sweep/tests/offset_restates_a_neighbour_chart_rim.rs`:
the drum through `offset_charts_together`, undeclared (the rim comes
back `Intersection(minted wall, cap)`, the revolve's own form, tier-3
valid) and declared (an image in the minted wall's chart, its
`RevolvedPoint` re-authored at the moved corner); the drum through
`replace_faces_offset` at ±1/64; and a cube's top edge declared in a
side's chart through `replace_faces_offset` (the declaration translated
by `d`).

Residue: the planar door's own restate keeps a held neighbour's image
the same way and refuses at certification, on main as on head. Filed as
`the-planar-offset-door-keeps-a-held-neighbours-chart-image-the-moved-edge-has-left`.
