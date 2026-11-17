---
id: a-run-of-ruling-points-the-development-merges-is-re-split-after-the-triangulation
kind: unit
title: The chart lane's cone arm gives a run of ruling points its development merges one CDT vertex and re-splits it after the triangulation
status: dispatched
opened: 2026-11-17
priority: P1
cost: M
branch: tesslane/merged-run
---

## Finding

A cone face on the chart lane refuses `SphereTrimShape { DevelopedMerge }` when two points of one ruling of its boundary lie closer than the development resolves. `trimmed::developed` cuts a point's slant radius to 27 significant bits and its direction to 26, so that every product is exact and all points of one ruling lie on one exact line through the apex; that rule is what keeps a triangle off one ruling. Two points of one ruling whose slant radii agree to within 2⁻²⁷ of the radius therefore develop to one point, and `trimmed::developed_merge` refuses the face.

The resolution is 2⁻²⁷ of the SLANT radius `r / sin α`, so it is coarse on a slender cone: about 4·10⁻⁵ on a cone of half-angle 0.01° and radius 2. PR 7049 routes every cone face whose ruling is split to this lane, and its delta review measured 408 cells that mesh on that PR's base and refuse on its head at the DEFAULT ε, at δ 0.1, 0.01 and 0.001:

- bodies: `revolve` of `(1, 0), (2 + tan α, 0), (2, 1), (1, 1)`, and the solid `(0, 0), (2 + tan α, 0), (2, 1), (0, 1)`, full and half turn, one cone ruling split, placed obliquely;
- α = 0.01°: a split 10⁻⁵ of the ruling's range from either end, or two splits 10⁻⁵ apart mid-ruling, and every finer gap `split_edge` admits;
- α = 0.1°: 10⁻⁶; α = 1° and 3°: 10⁻⁷; α = 26.6°: two splits 3·10⁻⁸ apart.

At ε 10⁻¹² a split 10⁻⁹ from an end does it on an ordinary frustum (168 cells in that PR's first review). PR 7049 carries all of this as a stated residual (`a-ruling-split-within-the-developments-resolution-merges-two-points`, with its pins in `crates/mesh/tests/split_rulings.rs`); this unit is its cure. The limit is the chart lane's own and older than the route: a trimmed cone face with two boundary points of one ruling that close refuses on main too.

## Unit (2026-11-17, TESSLANE)

**Letter M. Tier: single review, full.** The change is inside one arm of the chart lane (the development's point insertion, boundary constraints and emit pass) and by construction reaches only faces that refuse today; its hazard is the mesh's topology at the re-split vertex, which `check_mesh` and the per-triangle certificate and winding rule read directly.

Branch `tesslane/merged-run`, cut from `tesslane/cone-axis-winding` at `22e7ef6ffd` (PR 7049, in its delta re-review): merge that branch again when its closing head lands, and main after it merges. PR 7041 (`tesslane/route-by-domain`) is in `tessellate.rs` and `curved.rs`; do not merge it.

**The cure.** Where the points the development merges are CONSECUTIVE points of one straight side of the developed polygon (a run on one ruling), the CDT gets one vertex for the run, and the run is re-split after the triangulation. With `a, p₁, …, p_k, b` in order along the ruling and `P` the run's one vertex: the fan triangle on side `a–P` takes `p₁`, every other triangle of `P`'s fan takes `p_k`, and the triangles `(p_i, p_{i+1}, x)` are added, `x` being the third corner of the triangle on `a–P`. Every triangle then goes through the certificate and the winding rule on its true 3-D corners like any other. Any other merge still refuses `DevelopedMerge`.

The analysis this rests on, by PR 7049's implementer from a reading of `trimmed.rs` (nothing was built; check each statement against the code before relying on it):

> Soundness, case by case:
> - **Manifold.** With a, p1, p2, b in order along one polygon side and P the merged vertex, P's triangles inside the face form one fan from side a–P to side P–b. a, P, b are collinear in the development (exactly: the cut keeps one ruling one line), so no single triangle spans the fan. The first triangle (a, P, x) becomes (a, p1, x); every later fan triangle takes p2; (p1, p2, x) is added. Edge uses: a–p1 once (boundary), p1–x twice, p1–p2 once (boundary, matching the neighbour's chord segment, so watertight), p2–x twice. That is a fan about each of p1 and p2. A run of k ≥ 3 points generalises: the first triangle takes p1, the rest take pk, and (p_i, p_{i+1}, x) is added for each i. All the added triangles share x, which is off the ruling: the CDT made (a, P, x), so x cannot be collinear with a and P.
> - **The rule it must keep.** No triangle lies along one ruling: the added triangles have two corners on the ruling and x off it. Every triangle then goes through the certificate and the winding rule on its true 3-D corners. (p1, p2, x) is thin: |p1p2| is the merged gap, about 1e-5 to 1e-9 of the radius, against a lattice step for its height. Its cross product is about gap × step, far above the rounding allowance for any gap the cut can merge (≥ about 1e-9·r against allowance about 1e-16·r). A rare winding flip from substituting p2's true position for P's developed one would refuse typed (FoldedTriangle), never meshing silently.
> - **The apex row: excluded.** If either merged point is a singular (apex) id, it keeps refusing. The apex is already one CDT vertex standing for all its copies, and merging a split point into it would put a non-apex id inside the apex fan the drop rule collapses.
> - **Seam copies: fine.** The seam's two copies are on two different rays, so a merge on one ray is a run on that side alone. The same ids merge again on the other ray and are re-split there independently. ids_at_two_uvs and the census of identified edges see p1 and p2 as ordinary ids, each with its own fan.
> - **Non-consecutive merges: still refuse.** That covers two points of different sides, a point and a lattice node (`lay` keeps nodes clear, so not expected), and points of different loops.
> - **Other bookkeeping.** Crossings and the even-odd classification: side p1–p2 has zero length in the CDT and is skipped as `a == b` already is; a–P and P–b are constrained as sides. The shared-side and ear rule (`faces_across`) sees p1–p2 as a polygon side and the other two sides of (p1, p2, x) as interior.
> - **Where it lands.** `insert_points` stops refusing the run and records it (one handle, ordered ids); `constrain_boundary` skips the zero side; `emit` re-splits P's fan. `developed_merge` narrows to the non-run case.
>
> What it moves: every DevelopedMerge face of the class meshes instead of refusing: the 408 default-ε cells, my 1e-12 cells, and any chart-lane cone face with two points of one ruling closer than 2⁻²⁷·r/sin α (on main too, outside the class). No face that meshes today changes, since a face without a merge never enters the new path. Cost: one extra handle map per face, nothing per triangle.

**Owed by the unit:**

- **Measure before building.** Reproduce the default-ε cells above and the ε 10⁻¹² ones on the branch as cut, and census what else reaches `developed_merge` today: over the `mesh` suite, `sweep`'s meshing modules, `step-import` and the tour, how many faces refuse `DevelopedMerge`, and of which kind (a run on one ruling side; two sides; a point and a lattice node; two loops; a point merged with the apex). Say which the cure reaches and which it leaves.
- **Build the cure** as above, in `trimmed::TrimmedFace` (`insert_points`, `constrain_boundary`, `emit`) and `developed_merge`. The run is recognised from the polygon's own order, not from distances; the rule "all points of one ruling on one exact line" is untouched; a merge that involves a singular (apex) id keeps refusing; a run on the seam is re-split on each of the seam's two rays.
- **What moves, argued.** No face that meshes today may move a byte: a face without a merged run never enters the new path. Show it by digest of every patch of the `mesh` suite and `sweep`'s meshing modules, base against head. Every face that newly meshes passes `check_mesh`, the winding rule and the certificate, and `reach_probes::geometry` or the split-ruling rows' `got_enclosing` (resolved outward winding, sampled deviation within δ, volume). **A face that meshes on the base and refuses or moves on the head is a stop**: report it, do not tune.
- **Rows.** PR 7049's residual pins in `split_rulings.rs` (`a_ruling_split_within_the_developments_resolution_refuses` and the default-ε cells beside it) become meshing rows; a row with a run of three or more; a run on a one-face full-turn wall's seam (both rays); a run next to the apex row that must still refuse (a split merged with the apex), with text true of it; a merge that is no run (two sides) still refusing. Three ε. Mutants, each red on a named row: the re-split removed; the added triangles dropped; every fan triangle given `p₁`; the apex exclusion removed; a non-run merge admitted.
- **The thin triangle.** `(p_i, p_{i+1}, x)` has a base as short as the merged gap. Say what its certificate and its winding verdict read at the smallest gap `split_edge` admits at each ε, on the slenderest cone above, and whether any such triangle is refused `UnresolvedWinding` or `FoldedTriangle`; one that is, is reported with its body.
- **Sweep the class** (implementer-discipline §5): another place where the chart lane holds two mesh vertices at one CDT location and refuses or drops (the apex row's drop rule, the seam's copies, `ids_at_two_uvs`' readers, the sphere arm's pole). One disposition per hit.
- **Tracker.** Close `a-ruling-split-within-the-developments-resolution-merges-two-points` with what the cure reaches and leaves; `a-ruling-split-a-hair-from-its-end-folds-a-triangle` is a neighbour, not this unit's, unless the cure changes its answer, which is then reported.
- **Pre-push**, on the pushed head: fmt, `scripts/run-gates.sh`, the doc gate, clippy (`mesh` and `geom-brep` all features, the viewer with `--features app`, `demos/tour`); `mesh` whole with the default profile (slow set included) and the changed rows at ε 10⁻⁶ and 10⁻¹²; `mesh --features budget`; the three `tools` roots; `step-import`, `stl`, `sweep`'s meshing modules; the viewer with `--ignore-default-filter` for its slow-set rows pinned on mesh output (`display_budget`, `review_pick_r2`); `demos/tour`.

**Done when** every cell above meshes within δ with resolved outward winding at the three ε, no patch that meshes today has moved, and the refusals that remain are named and pinned.

