---
id: a-pinch-no-kept-face-can-cross-refuses
kind: issue
title: A boolean whose pinch only a face's outer loop could cross refuses PinchUncrossed (cube minus a reflex corner on a cube face, the holed block's intersection, a staircase's second pinch)
status: closed
opened: 2026-10-04
priority: P0
cost: H
design: true
refs: [a-pierce-whose-wide-run-pinches-its-intersection-refuses, a-pierce-whose-difference-pinches-at-two-edge-runs-refuses, a-hole-weld-cannot-tell-a-figure-eight-hole-from-an-island-face, three-corners-alternating-round-a-corner-refuse-at-the-join]
closed: 2026-10-06
---


## What

Found by `join/pierce-pinch-families` (PR 4038), which taught the seam
zips to pass a pinch a second time (`boolean/zip.rs` `cross_pinches`).

**The class.** A boolean pinches where two cones of the result's
boundary meet at one point. Where both operands keep the point as one
vertex, the zips would fuse it to itself. One vertex holds two cones
only where a face's boundary crosses from one cone to the other there,
so `cross_pinches` first splits the vertex across two corners of kept
faces (`split_across`): two corners of one ring (`kemr`, two holes
meeting at the point), or the corners of two faces of one surface and
sense, one of them ringless (`kef`, one face). Where neither offers,
the op refuses `BooleanError::PinchUncrossed`.

**What is left is one shape.** On every remaining line the only face
through the point twice passes it on its outer loop, round a notch the
other operand cuts there: a hole touching the outer boundary at the
point (the row's old "nested" sub-family). Crossing that loop (`mev` +
`kemr` on it, an instrumented build) splits it into an outer loop and
a ring that meet at the point, and the gate refuses
`ResultInvalid { RingMeetsOuter }`.

**There is no bow-tie sub-family.** The lines the old row called
bow-tie (`LoopRoleInverted` under that build) are the same nested
shape: `kemr` left the outer role on the hole-shaped half, so both
loops read inverted, and the list goes on to `RingMeetsOuter` at the
point (`outcome` cuts error text at 110 characters, which hid it).
Measured on every such line: main's 48 (staircase 43, near-tangent 4,
notch327 1, see below), and review r2's 184 on main before PR 4036
(`cced486c`, its cube set: 184/184 carry both inversions and the
meeting). A true bow-tie, one face's outer loop passing the point twice
round two separate regions, cannot reach the pre-pass: its two corners
at the point would each join one region's edges, so the face's
boundary there is two loops already, not one.

**The old "none twice" lines were an island.** The holed block's 18
cube ∖ holed lines (and 5 on review r2's cylinder): the plane's section
closes round the hole, so the cube's plane keeps an island face inside
its near face's hole, touching that hole's ring at the point, and one
seam meets the point twice. The island's outer corner and the ring's
corner, one surface and sense, now cross by `kef` (see Built).

**Reach on main `4cf3f8b9`** (release; every line `cube ∖ ·` unless
said), 590 `PinchUncrossed` lines before this lane, 567 after:

| battery | nested | read as bow-tie | island |
|---|---|---|---|
| r2 cube, `v` on a face (vee300 11, asym 10, vee224bot 5) | 26 | 0 | 0 |
| r2 near-tangent, on a face | 24 | 4 | 0 |
| r2 cylinder wall | 30 | 0 | 5 → `RingOnCurvedFace` |
| r1 cube (Uin 30, notch327 18) | 47 | 1 | 0 |
| r1 `u2` staircase, second pinch | 0 | 43 | 0 |
| holed block (∩ both orders; cube ∖ block) | 392 | 0 | 18 → `SOUND` |
| `pierce_runs_battery` | 0 | 0 | 0 |

Batteries: `crates/sweep/examples/r2_pinch_probes.rs` on
`join/pierce-pinch-families-review-r2`,
`crates/sweep/examples/r1b_pinch_probes.rs` on
`join/pierce-pinch-families-review-r1`, and `r2_holed_battery` in
`crates/sweep/tests/join_pierce_r2_probes.rs` on
`join/pierce-two-out-runs-review-r2`. Edge and corner placements left
the class with PR 4036.

**Pinned**, each asserting the refusal and every other run `SOUND`:
`join_pierce_runs_sweep::a_pinch_round_a_notch_on_a_faces_outer_loop_refuses_typed`
(`vee300 fib62 face`) and
`join_pierce_runs_sweep::a_staircases_second_pinch_round_a_notch_refuses_typed`
(`u2 S_tt b594`, the one read as bow-tie).

## The shape to give

**A pinch is one vertex per cone** (Ev, PR 4057, 2026-10-05). At a
pinch the result holds several vertices on one point key, each a
manifold cone, and no face crosses between cones:
- The notched face keeps one outer loop through both vertices.
- An island stays its own face, and a bow-tie is two faces.
- The crossing pre-pass (`zip::split_across`'s `kemr`/`kef`), the
  pinch welds (`finish::weld_pinches`, `weld_pierce_copies`) and
  `BooleanError::PinchUncrossed` retire. PR 4051's island `kef` goes
  with them.
- Check 9 can then refuse every meeting of two loops of one face.

Why: a crossing reads one handle more than the solid has. D1 already
calls touching via two vertices at one point representable.

The construction route is the lane's to choose. The designers named two
routes:
- a zero-length seam edge, minted at each operand's section-face
  corners at the pinch and consumed by the zip's second pass;
- splitting each operand vertex per result cone (the cycles of seam-pair
  fusions `cross_pinches` already computes) before the zips.

This ruling also decides two rows:
- `a-boolean-ships-a-face-whose-loop-passes-two-vertices-on-one-point`:
  its bodies are right, and its check becomes "every corner is a slice
  of its own face".
- `a-hole-weld-cannot-tell-a-figure-eight-hole-from-an-island-face`:
  dissolved.

## Built (branch `join/pinch-uncrossed-residue`)

`zip::split_across`'s two-face crossing no longer asks both corners to
be outer. It asks that one of the two faces be ringless, and the
ringless one dies into the other (`kef` cannot kill a face holding a
ring). That covers an island face pinched to its hole's ring:
- holed block 18 cube ∖ block lines go to `SOUND`;
- review r2's 5 cylinder lines cross, then refuse at FLUX's volume
  lane (`VolumeUncomputable { RingOnCurvedFace }`), refusal to refusal.

PR 4051's review measured further island families going refusal to
`SOUND` (its `r4051_island_probes.rs`, on
`join/pinch-uncrossed-residue-review`), with no other line moved:
- a deeper hole, h = 6: 22 more lines; a shallow one, h = 0.6: 8;
- two holes, a plane through a corner of each, two islands in one op:
  60;
- a U-shaped hole, a plane through both arm tips, one island pinched
  twice: 38, plus 2 at a single tip.

Pinned by
`join_pierce_runs_sweep::an_island_face_pinched_to_its_holes_ring_crosses_and_builds`
and, for the twice-pinched island and the face `kef` kills there,
`join_pierce_runs_sweep::an_island_pinched_twice_to_its_holes_ring_dies_at_each_crossing`
(`u2tip mid side=12 psi=0 th54`).

## Measured (step 0, branch `join/pinch-one-vertex-per-cone`)

The mesher refuses the ratified shape. On main `f1a4a317`,
`mesh::tessellate` panics on every body whose one face passes two
vertices at one point. That is the shape the ruling makes every pinch
build. Thirteen such `SOUND` bodies on review r2's cube poses panic,
`notch307 fib117 edge psi=0.3 pc S` among them. Each passes tier 2,
tier 3′, the certificate and mass properties at the oracle volume
(3.766827644). No other body on those poses panics. The cause is the
planar lane's same-position dedup, filed with the lines as
`work/tess/a-planar-face-through-two-vertices-on-one-point-meshes-under-one-id.md`.

What the zips meet at the pinned pinches (instrumented `cross_pinches`):
- `vee300 fib62 face` and `u2 S_tt b594`: each operand's pinch vertex
  has two section corners on two section faces (two seams). The cones,
  the cycles of the seam pairs' fusions, put one run of each operand
  into each cone. So both operand vertices split, and each split's
  transient edge lies between the two section faces.
- The island poses (`holed c00 side=4 g6.0`,
  `u2tip mid side=12 psi=0 th54`): both section corners lie on one loop
  of one section face (one seam meets the point twice).

Nothing was built past step 0.

## Built (branch `join/pinch-one-vertex-per-cone-build`)

**One vertex per cone, split before the zips** (`zip::split_cones`).
Each operand vertex's section corners start runs. An A run continues
into the B run the zip glues to the section corner it ends at, so the
result's cones are the cycles of `σ_B ∘ σ_A` over the seam pairs. A
vertex whose runs lie in several cones is split per cone by `mev_null`,
on its own point key. The transient edge lies on the section faces the
zips consume and is killed there:
- `kef` between two section faces, which merges their seams;
- `kemr` in one section loop, then `mfkrh` for the split-off loop.

The seams are then re-paired, and the correspondence is rebuilt from the
edges the zips join. A cone that would still fuse a vertex to itself
refuses `ZipCorrespondence` (no battery line reaches it). Lumps that met
only at a pinch become shells of their own (`movefac` after the zips).

Retired:
- the crossing pre-pass (`cross_pinches`, `split_across` and its `kemr`/`kef`
  crossings, PR 4051's island `kef` included);
- the post-zip pierce weld (`weld_pierce_copies`);
- `BooleanError::PinchUncrossed`.

The pre-zip weld stays, as the repair of an operand's coincident
pierces; its vertex is split per cone with the rest
(`the-pre-zip-pinch-weld-retires-once-coincident-pierces-split-per-cone`).
The corner-slice check and check 9's widening are filed
(`a-corner-is-a-slice-of-its-face-tier-3-check`,
`check-9-refuses-only-a-ring-meeting-its-outer-loop`).

**Where this row's 567 lines went** (main `f9bf3bca` vs head, release):

| battery | lines | head |
|---|---|---|
| holed block (∩ both orders; cube ∖ block) | 392 | `SOUND` |
| r1 cube | 48 | `SOUND` |
| r1 `u2` staircase | 43 | `SOUND` |
| r2 cube | 26 | `SOUND` |
| r2 near-tangent | 28 | `SOUND` |
| r2 cylinder | 30 | `BAD`, operand check only (the curved legal-operand class: 12 536 of main's 17 280 cylinder lines) |

Beyond the 567: 86 lines of review r2's shapes battery and 1 251 of
PR 4051's island families also go `PinchUncrossed` → `SOUND`, and 62 on
its cylinder walls → operand-only `BAD`.

Every planar body the batteries newly build (2 648 lines) tessellates at
δ = 0.05 and passes `check_mesh`. Of the 134 newly built cylinder-wall
bodies:
- 30 mesh;
- 60 refuse `CertificateExceeded` and 44 `Triangulation`, the trimmed
  lane's refusals at that δ, which main's cylinder bodies meet as often
  (main: 1 145 and 767 of `isl_cyl`'s lines, head 1 170 and 643).

No line meets the mesher's `PinchWedge`, so no crossing ships.

Pinned by `join_pierce_runs_sweep`:
- `a_pinch_round_a_notch_on_a_faces_outer_loop_is_one_vertex_per_cone`
  (`vee300 fib62 face`: its difference's near face runs through two
  vertices on one point key);
- `a_staircases_second_pinch_round_a_notch_is_one_vertex_per_cone`;
- `the_holed_blocks_intersection_pinched_at_its_hole_corner_builds`;
- `an_island_face_pinched_to_its_holes_ring_stays_its_own_face`;
- `an_island_pinched_twice_to_its_holes_ring_stays_its_own_face`;
- `two_pinches_in_one_op_are_each_split_per_cone`.

Each asserts every run `SOUND` and meshing. In `join_pierce_strut_facing`,
`a_pinch_keeps_one_vertex_per_cone` pins two vertices at `v`.
