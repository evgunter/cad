---
id: a-kissing-convex-corner-result-ships-an-undeclared-vertex-on-face
kind: issue
title: A convex corner kissing a cube's face ships results with an undeclared vertex-on-face (8 runs, identical on main)
status: closed
opened: 2026-10-04
priority: P1
cost: M
refs: [a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms]
closed: 2026-10-13
pr: 4820
---


## What

Found by PR 4026's review r2 (N1) and re-measured by its fix pass on
main `45dc18f9`, in release. The battery is `join_pierce_r2_probes.rs`
`r2_shapes_battery` on branch `join/pierce-two-out-runs-review-r2`
(one pose: `R2_SHAPE=Lcvx R2_TAG=<tag> … r2_one_pose`).

The shape `Lcvx` puts the L-prism's convex corner on a cube's face,
undeclared, over a grid of directions. Eight results build with tier 2,
the certificate and the exact volume, but tier 3′ refuses
`UndeclaredContact { VertexOnFace }`. For example `g4.6 cp U` at
`(2, 1, 1)`. The runs are `g4.6`, `g4.7`, `g5.7`, `g5.8` and
`nf0_1e-2k0s1` (cp U), and `g16.3`, `g17.2`, `g17.3` (cp S).

The lines are identical on main and on PR 4026's head; nothing in that
PR's diff reaches a single-run corner. The result keeps a vertex of one
operand lying on a face of the other and records nothing for it. The
same battery's ten `StaleContactDeclaration` results are evidence on
`a-boolean-result-ships-contact-records-its-geometry-no-longer-confirms`.

## The shape to give

Read one pose's contact records through `remap_contacts`. Either the
kissing vertex's record is dropped, or none was minted for a contact
the result keeps. Fix it at the record's mint or its remap.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: the tier-3′ UndeclaredContact{VertexOnFace} refusal becomes an unproven-coincidence finding recorded at the stage-4 door. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-09)

E does not reach this. Its glue door records face pairs the carrier ladder decides one carrier, or that the witness lane verifies tangent (`crates/topo/src/boolean/glue.rs:72`–`:83`). A convex corner kissing a face is a vertex on a face, and no face pair there is coincident or tangent, so no new record backs it. The census still raises `UndeclaredContact` for an unrecorded touch (`crates/topo/src/census.rs:1388` and its siblings). Not re-measured (the battery lives on `join/pierce-two-out-runs-review-r2`). The shape to give stands: the kissing vertex's record is dropped at its mint or its remap.

## 2026-10-11, fixed at the remap (FLUSH, `flush/corner-ruling-tier3`)

The cause is this row's: `carry_rows` chased a vertex-on-face record's
face by key, and a division of the face keeps the key on one piece,
whichever it is. FLUSH's
`a-box-and-a-cylinder-through-its-corner-edge-intersect-past-tier-3-prime`
(a cylinder's ruling along a box edge, the same defect on a curved
face, with the plain booleans and no declaration) reads a record's face
through the face's division rows (the join's and the welds' fragment
logs) and lands it on the piece whose interior holds the vertex, or
drops it where no live piece does (then `Descendants::holder`; since
PR 4820 the one carry's placement, `boolean::ops::place_found` over
`records::landings`, which lands it on every piece or chord the census
confirms).
On that branch the line-kiss poses of
`rest_zip_admission.rs` (`a_line_kiss_beside_a_tangent_site_records_its_contact`)
pass tier 3′. The `Lcvx` battery (`join_pierce_r2_probes.rs`, branch
`join/pierce-two-out-runs-review-r2`) was not re-run: whether its eight
runs share the cause is unmeasured. Re-run it before closing.

## 2026-10-13, closed: fixed by PRs 4650 and 4820 (FUSE, carry-rows-after-4820)

Measured on main `0d2117d0` with a temporary probe (never committed)
in `boolean_op_with` that ran `validate_pseudomanifold(body, contacts)`
on every body the door ships, over the topo, sweep and editor-core `ci`
suites (8,390 tests, about 18,000 results). Across all of it there is no
`StaleContactDeclaration`.

- **The battery this row cites** (`join_pierce_r2_probes.rs`, branch
  `join/pierce-two-out-runs-review-r2`) was re-run in a scratch copy
  adapted to the current API. Shape `Lcvx`, its eight poses (`g4.6`,
  `g4.7`, `g5.7`, `g5.8`, `nf0_1e-2k0s1`, `g16.3`, `g17.2`, `g17.3`),
  every op in both orders: **48/48 pass tier 3′**.
- **The line-kiss pin**
  (`rest_zip_admission::a_line_kiss_beside_a_tangent_site_records_its_contact`):
  40/40 pass.

The cause was the one this row's last note named: a vertex-on-face
record whose face the section divided, read by key. FLUSH's PR 4650
(`Descendants::holder`) fixed it, and CONTACTHOLD's PR 4820 (the one
carry: `boolean::ops::place_found` over `records::landings`) carries
the fix.
