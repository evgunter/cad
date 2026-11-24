---
id: the-mesh-cures-a-side-two-faces-can-both-make-three-ways
kind: unit
title: A side two faces both make is split where the patches join, once, in every patch that has it inside, at a point of that face's own surface; the three present rules retire
status: dispatched
opened: 2026-11-22
priority: P1
cost: H
branch: tesslane/join-split
---

Filed at the closing of unit 4c (PR 7096) as a design question, weighed by three designers over four rounds (2026-11-22 and 23), and decided by Ev on PR 7485 (2026-11-23: "allowing zero volume sounds annoying; this plan sounds good"). This file is the unit that answer specifies.

## The defect

Two faces that share two boundary chains can each make, as an inside edge of its own triangulation, the same straight side between a point of one chain and a point of the other. The joined mesh then has an edge of four triangles.

The mesh answers it today in three places, each by predicting what the face across will do:

1. **The triangulating arms of the trimmed lane** (`TrimmedFace::emit`, PR 7198). A kept triangle may not have a side joining two polygon points that are not loop neighbours and that one face across the loop's edges holds both of (`TrimPolygon::faces_holding_side`). Where it has one, a point is inserted and the face rebuilt; past the retry budget it refuses `UnclearedSharedSide`. It reads which faces hold the two points, not whether the other face makes the side.
2. **The walk** (`trimmed::walk_face`, PR 7096). The same test, then a second reading: the side is cut at its chart midpoint only where the face across could reach it (`trimmed::may_make`, by the side's distance from that face's surface). It reads where the other face's surface lies, not how that face is meshed.
3. **The rectangle lane** (`curved.rs`). No rule; the census at the join (`tessellate::joined`, `OverusedSide`) refuses the mesh.

Each lane predicts its neighbour, and each prediction is wrong somewhere: the first inserts a point for a side the other face never makes (997 points in 103 rebuilds over the `mesh` suite where about 502 sides are in fact made by both), the second cuts where the neighbour lays a row between the rims (256 to 440 of about 2,640 cells denser than main), and the third refuses a valid body (a ball of radius 1.05 bored at radius 1, unsplit, on seven cells). And a face's patch depends on which faces lie across its edges, which the patch memo's key has to carry (PR 7241).

## What is built

**No lane reads anything about the faces across its edges.** Whether two faces both made a side is a fact that exists only where the patches join, and that is where it is acted on.

1. **Detected at the join.** The census in `join::census`, on shared ids before any patch is placed, finds each straight side between two shared ids that a patch has inside (two of its triangles on it, not a chord of its own boundary) and that it splits:
   - a side used past twice per chord carrying it, which joined is an edge of four triangles; and
   - **the pair trigger** (ruled 2026-11-23): a side of a triangle all of whose corners are chord points the face F shares with one face G, where G has such a triangle toward F. Both faces then lay nothing but the points they share over one region (a cap thinner than δ against its plane), and their patches can cover it with the same points, a mesh enclosing nothing there.
2. **Split once, in every such patch.** Each patch that splits the side splits it at the foot, on ITS OWN surface, of the side's straight midpoint, read back to its chart (`join::point_at`; one rule on cylinder, cone, sphere and ring torus, in place of the chart midpoint, the cone's development and the singular-end rule; a spindle sheet and a spline face, which have no foot in closed form, keep the chart midpoint). Its two triangles on that side become four. Each half is held to that face's certificate, on an analytic face to its fold, and to the winding read (`[mesh.winding]`), as any triangle of the face is; one that fails refuses the face, typed, as it would have from its lane.
3. **Nothing is meshed again.** No lane is re-entered, there are no rounds, no patch is chosen to keep the side, and nothing reads the order faces are stored in.
4. **A boundary does not give way.** Where one face's boundary chord carries the side, that face keeps it, and every patch that has it inside splits.
5. **Two things beside it, needed under any cure:**
   - **A chord floor.** Two edges joining one vertex pair are never chorded to one segment. The seven refusing cells of the unsplit bored ball hold this second defect: the bore's two seam-side edges both chord to the segment between the rims' vertices, which the census passes and `check_mesh` refuses.
   - **A fold's own cure.** A face whose triangulation folds gets a point inserted for the fold, not for a shared side (two needle-cone cells lean on today's shared-side insert for this).
6. **What retires:** PR 7198's rule on the triangulating arms with `faces_holding_side` and `UnclearedSharedSide`; the walk's cut and `may_make`; the memo key's entry for the faces across a face; `OverusedSide` as a refusal (after the split a side used past twice is a defect of the join, reported as `[failure.taxonomy]` reports a kernel defect).

No clause of `crates/mesh/README.md` or `docs/DESIGN.md` changes. One sentence of `crates/mesh/src/lib.rs` (the per-face purity of a patch, written by PR 7241) is rewritten: a patch is a function of its face and δ, and the join adds the split points.

## Why a point of the face's own surface, and in every patch

**What the split is for** is the mesh's shape where two patches meet: an edge of four triangles where two patches make one side, and, with the pair trigger, two patches covering one region with the points their faces share. It promises no volume: where the faces are closer than δ, the mesh encloses whatever their own points make of it, held to the certificate, not to the body's volume (`mesh::validate::check_volume`'s bound is δ times the area, vacuous there).

The other consistent answer is the side's straight midpoint, split in every patch but one. Its halves are pieces of triangles already certified, so it needs no certificate and cannot refuse, but one patch must keep the side, chosen by storage order. A straight midpoint in every patch is not an answer: the two new points share a position and the same two corners, which is the edge of four triangles again for any reader that joins vertices by position.

The two were measured against each other where they differ, on a body thinner than δ: the ball of radius 1.05 bored at radius 1 (a shell 0.05 thick, volume about 0.137), both arc signs, seam split and unsplit, δ 3 to 0.1. There both faces lay the same chords, so the mesh is one sheet counted twice.

| | signed volume of the returned mesh | written as STL, read by `scripts/check_admesh.sh` |
|---|---|---|
| today (PR 7096's head) | −0.0715, −0.039, +0.028, +0.0715 on the split body; the unsplit body refuses on seven cells | 4 of the 21 meshes returned fail: every facet reversed |
| straight midpoint, all but one patch | zero to rounding on the split body (±10⁻¹⁶); +0.010 to +0.015 unsplit | 4 of 28 fail: the sign of a zero volume is rounding's |
| own-surface point, every patch | +0.078 to +0.092 (58 to 67% of the body's) | 0 of 28 fail |

A straight split leaves the two sheets on one set of chords; a point of each face's own surface moves each sheet toward its own face. On these cells that gave the wall a positive volume; it is what was measured, not a theorem, and no page promises it. With the foot and the pair trigger the shell's row reads 0.523 to 0.897 of the body's volume on 84 cells in three placements (`split_rulings::a_shell_thinner_than_delta_meshes_split_or_unsplit`, its margin 0.45).

**Why the foot.** Over every side the `mesh` suite's patches could be asked to split (the filed row on pieces that would fold), the chart midpoint folds a piece on 687 cone sides (the walk's and the trimmed lane's), 10 on the rectangle lane's cone, 75 on the trimmed sphere and 8 on the rectangle sphere; the foot folds none of them, 16 against 46 on a torus, and the same 6 on a cylinder, where the two are one point. At an apex or a pole the foot is the singular-end rule's point, and it reads no frame, so it does not depend on where a face's chart puts its pole. It is not what recovers the lost bodies: with the pair trigger on, the chart midpoint loses no body of the thin-cap class either (0 refusals, 0 flat, 126 of 636 cells within 5% of the foot's volume).

Over the `mesh` suite at the default ε, with the three present rules off and the chord floor and fold cure on, the designers' probes counted 1,416 such splits over 59 meshes; the stage-1 count, with every pin of today's cut counts softened so that each row meshes all its bodies rather than stopping at the first moved pin, is 1,846 over 73 (below). No half folded against its parent, none farther than δ from its surface by sampled distance (worst 0.905 δ, none worse than its parent), no side left overused; two new points at one position only on the lens below, whose two faces are one sheet; six rows red, each a pin of today's cut counts or of a refusal's name.

## What the measurements did not cover

- The stage-1 halves were held to sampled true distance, not to each lane's own certificate; the build holds every piece to its face's certificate, and nothing measured refuses there.
- Two spline faces can both make a side (`faces_that_bound_no_region::a_lens_of_two_short_arcs_meshes_at_a_coarse_delta`). A spline surface has no closed-form foot, so its split point is the chart midpoint, and its pieces are held to the certificate alone (no outward normal in closed form to read a fold against).
- **The thin-cap class** (2026-11-23, the review of 24ee9a28fb): a face thinner than δ whose boundary is one closed chain shared with one neighbour. Measured on caps of a ball and of a ball less a box (heights 10⁻³ to 0.1, latitudes ±90° to 0, four azimuths), lenses of two spheres, a cylinder segment and a cone's tip, at δ ten times the thickness down to a tenth, δ 1 and 10⁹, three ε: 636 cells, none flat or negative under the pair trigger, the least at 0.363 of main's volume. A torus sliver did not build. Caps of other surfaces, faces bounded by two chains, and a class face against a spline neighbour were not measured.
- **The three-face cap.** A cap that also meets a third face (the ball less a box, cut toward latitude ±45°, azimuth 300°, height 0.03) is outside what the trigger reaches: at δ 0.3 the sphere's triangles take a point of the third face's arc, so neither face lays a triangle of their shared points alone. It meshes positive at 0.238 of the body's volume (main's 0.656), pinned as measured in `sweep`'s `snowman::a_cap_thinner_than_delta_encloses_its_volume`.
- **Trigger triangles with no inside side** (a patch of one triangle, all its corners shared with one face): a point at the foot of its centroid was measured behind a switch and fired on nothing in the `mesh` suite (13,998 calls), the class table or the bored ball; it was not run over `sweep`. It does not land; such a triangle keeps its three corners.
- What `tessellate` owes a body thinner than δ is before the owner (PR 7592); this unit answers only where the patches join.

## Stage 1: measured (2026-11-23)

The split at the midpoint in each face's own chart, each half held to that lane's own certificate, its fold where the lane reads one, and the winding read; the three present rules off, the chord floor and a fold cure on. Every `tessellate` call was logged and diffed against the same suite on main.

| pair (patches with the side inside) | splits at ε 1e-9 / 1e-6 / 1e-12 | halves refused (certificate, fold, winding) | worst half |
|---|---|---|---|
| plane + trimmed cylinder | 802 / 802 / 802 | 0 | 0.780 δ |
| rectangle sphere + walked cylinder | 224 / 224 / 224 | 0 | 0.905 δ |
| rectangle cylinder + rectangle sphere | 184 / 184 / 184 | 0 | 0.905 δ |
| rectangle torus + walked cylinder | 184 / 184 / 184 | 0 | 0.345 δ |
| rectangle cylinder + rectangle torus | 180 / 180 / 180 | 0 | 0.345 δ |
| plane + trimmed cone | 138 / 52 / 138 | 0 | 0.404 δ |
| trimmed cylinder + trimmed cylinder | 128 / 136 / 132 | 0 | 0.471 δ |
| spline + spline (the lens) | 4 / 4 / 4 | 0 | 0 |
| plane + trimmed sphere | 2 / 2 / 2 | 0 | 0.128 δ |

1,846 / 1,768 / 1,850 splits over 73 / 57 / 73 meshes. `sweep`'s 131 meshing modules at the default ε: 276 splits (trimmed cylinder pairs, plane + trimmed sphere), none refused. No body that meshes on main refuses under the split; the unsplit bored ball (refused `OverusedSide` on main) and the rod sliver of `trimmed::tests` (refused `UnclearedSharedSide`) mesh.

- **The apex.** No side split in the suite has an apex end. Of every side a cone or sphere face could be asked to split, the apex ones (about 30,000) all certify; in a cone's development the apex is the origin, so the development's midpoint of a side from it is the chart rule.
- **The development.** On a cone the chart midpoint and the development's midpoint differ (by up to 5.5·10³ δ on a wide side of the rectangle lane), and the pieces certify at either. Ruled by measurement: the one whose pieces fold on fewer of the sides a cone face could split, the development's (walked 685 against 1,675, rectangle 10 against 57, trimmed 0 against 6).
- **Sides that could be split and never were.** Over every side a lane could be asked to split, no piece fails its certificate, but some would fold or fall under winding resolution; none of them is made by a second patch in any body measured. Ruled: the face refuses typed if one ever is, and the class is filed (`a-side-the-join-could-split-has-a-piece-that-would-fold-or-fall-under-winding-resolution`).
- **The lens** (`faces_that_bound_no_region::a_lens_of_two_short_arcs_meshes_at_a_coarse_delta`): its two spline faces are one sheet, so each one's own-surface point is the same position. Joined by position, main's mesh of it is already a sheet counted twice (6 coincident pairs of triangles, 7 edges of more than two); the split's has 4 and 4. By vertex id both are closed manifolds. Nothing a reader can see changes; recorded on `work/ciw/the-admesh-gate-cannot-see-an-edge-of-four-triangles.md`.
- **The pick memo** (`editor_core`'s `PickMemo`) serves a patch's pick table under the patch memo's entry id, which promises the placed corners are the entry's. A patch the join split is not: it reports no entry id (`PatchKeys::unstore`), and its table is built for the picture. Ruled.
- **A triangle split twice.** Two sides of one triangle are both split on the walked strips of the bored ball (308 triangles); a split certified per side would hold pieces that are never made. Ruled by that: the join splits on demand from the chart corners the patch keeps, as a pure function of the face, δ and the side, and reads the pieces it actually makes.
- **The fold cure.** The shared-side insert also cured folds: on the near-pole circle of `sweep`'s `tilted_sphere_pair` (δ 0.04), 133 of the 134 sides it inserted for are folds of a triangle with no lattice corner, and a cure that inserts only for those and keeps dropping lattice corners for the rest loses that body. Two cures were measured against main with the rules off: (a) a point inside every folded triangle, the corner drop gone; (b) the drop kept, an inserted point never dropped. At the trimmed lane's headroom neither loses a body at any ε or in `sweep`'s meshing modules; (a) meshes four cells that refuse on main (the near-pole tube three, a needle funnel one), (b) three; triangles −0.001 % on `mesh`, −0.014 % and −0.012 % on `sweep`. At δ itself (`UNPROVED_HEADROOM` 1.0) both mesh the tilted pair in one round and the near-pole tube at 5·10⁻³, and both lose one cell to `CertificateExceeded` with the insert retired (`reach_probes::crossing_rods_mesh_manifold`, a certificate refusal at that target). Against (a), at δ: (b) meshes one needle-funnel cell (a) does not, and (a) spends more rounds there (two faces reach its last round and refuse `FoldedTriangle`, as they refuse on main under another name). Ruled: (a), one rule. Its rounds are its own named budget, `trimmed::MAX_FOLD_ROUNDS` (6), beside refinement's `MAX_GRID_RETRIES` (6): a face that meshes takes at most three fold rounds at the headroom and five at δ. Six faces of `sweep`'s `spheres_crossing_off_every_edge` beside its pole spend all six and refuse `FoldedTriangle`, as on main.

2026-11-23: at δ itself (`UNPROVED_HEADROOM` 1.0, the sizing unit's target) `reach_probes::crossing_rods_mesh_manifold` refuses `CertificateExceeded` once the shared-side insert retires; it meshes at the headroom this unit ships at. A cell the sizing unit must clear by its correction.

## Unit

**Letter H. Tier: dual review.** It moves a rule out of three lanes into the join, changes what a patch is a function of and what the memo key holds, retires another program's rule (TESSLIE's PR 7198 and PR 7241), and the split's safety on every chart rests on measurement the build has to complete.

Stage 1 measures what the list above names, on the lanes' own certificates and with the chart midpoint, over the `mesh` suite at the three ε, and stops if any half refuses on a body that meshes today. Stage 2 builds it.

Neighbouring rows: `a-strip-against-a-neighbour-holding-both-rims-makes-an-edge-of-four-triangles` (main's unsplit bore: closed by the chord floor here); `a-walked-sides-reach-exclusion-is-read-only-by-the-walk` (moot once the walk's rule retires); `work/tesslie/the-shared-side-rule-on-the-spline-arm-has-no-witness.md` (the lens above is a candidate witness); `work/ciw/the-admesh-gate-cannot-see-an-edge-of-four-triangles.md`; unit 6 (`rectangle-faces-move-to-the-chart-lane`).
