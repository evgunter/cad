---
id: teapot-scene-through-node-shell
kind: issue
title: Convert the tour's teapot vessel from the plane scan to Node::Shell (a render-lane change with tess-budget rows)
status: closed
opened: 2026-09-06
closed: 2026-09-08
---

`demos/tour/src/teapot.rs` builds the vessel kernel-direct: a
`revolve` of `vessel_meridian`, then `shell_open` with the mouth's two
faces found by a numeric plane scan (`plane_chart_at`), because no
document could name them. `Node::Shell` now can: the document spelling
is `crates/editor-core/tests/corpus/vessel.rs` — the same meridian,
station for station, then `Node::shell(pot, len(WALL),
[band(pot, SEG_MOUTH), band_pi(pot, SEG_MOUTH)])`, the mouth named by
ROLE and carried through a rebuild.

Converting the scene is a render-lane change with tess-budget rows
(`docs/TESS-BUDGET.md`; the tour's frames are committed evidence and a
changed frame is re-baselined with its reason, never restored), so it
is its own unit rather than a rider on the door. The lid and the spout
were always sayable (`docs/guide/north-star-audit.md` row 27); the
handle is `Node.tube`. After the conversion, rows 27, 44 and 45 of the
audit flip when a Python row executes the scene.

## Closed

**What flipped.** `demos/tour/src/teapot.rs` is ONE
`Doc<ProfileProgram>` and every body the scene ships is a node's
value: the pot's revolve, the sealed hollow and the cup as two
`Node::Shell`s over the same operand and wall (parted only by the open
list), the lid's revolve and its roll, the spout's revolve and
`Node::Transform`, the handle's `Datum::Axis` + `Node::Tube`, and the
two joins the operand gate has no arm for as `Node::Boolean` nodes
that refuse at `evaluate` with the kernel's payload carried unaltered.
The mouth is the two half-discs of the meridian's mouth-disc segment
BY NAME and the lid's three rims are the `BandRim` edges at the
meridian vertices they stand on, so `plane_chart_at` and `rim_at` are
deleted — the role names were checked against the keys the scan found
before it went. `gallery_document` exposes the scene's recipe, and the
audit's rows 27 and 44 flip on `TestTeapot` and `TestTorusvessel` in
`crates/pncad-py/tests/test_north_star.py`.

**What stayed, and why.** The lid's roll is TWO `Node::Fillet`
requests where the kernel door takes one. That is not a choice: the
blend NAME emitter refuses the one-request output, because the flange's
rim and the dome's foot slit one meridian segment's seam and
`RoleSeg::BandSlit` names a slit by the source edge alone. Filed as
`blend-slit-name-collides-when-two-rims-share-a-meridian`; the scene
records it as its sixth finding and the geometry is unchanged by it.
Row 45 stays NO on `Body::merge_coplanar_faces`, which is its own
question and not this unit's.
