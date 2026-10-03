---
id: limb-3-tube-banks-a-second-arc-as-accounted
kind: issue
title: limb 3's tube proved one solution per slice, not one arc, and the searches banked a second arc inside it as accounted — a silent lost branch on main's R3 lane
status: open
opened: 2026-10-03
priority: P1
cost: H
branch: ssi/limb3-one-arc
---

## Found (review of PR 3983, Hermite-first; reproduced and widened by the limb-3 lane, 2026-10-03)

C2 says limb 3's tube proves "the solution set in the chain is one
arc", and C3's accounting banks every cell inside a found branch's tube
as accounted (`ssi/exhaust.rs`, `SweepDuty::accounts`). The probe
proved less: a zero-free `(∇f₁ × ∇f₂)·e`, or the chart form
`∇φ·e⊥`, over a box gives at most one solution on each slice of it
(`ssi/certify.rs`, `probe_tube_analytic`, `probe_tube_chart`). A
second arc beside the first along `e`, leaving through the box's sides,
passes that. C2's own text deferred it ("the tube says nothing about a
disjoint component … that is C3's exhaustiveness obligation"), while
C3's accounting counted the tube's cells as already proved. Neither
side checked it.

### The reviewer's fold (plane × NURBS, reachable on Hermite-first)

- **Wall:** `z = c·x + a·x² + 4β·y(L − y)/L²`, exact as a biquadratic
  Bézier.
- **Plane:** `z = 0`.
- **Parameters:** β = ε, c = 80ε, a = 0.28·c²/β, w = β/c.
- **Domain:** `x ∈ [−1.5w, 1.8w]`, `y ∈ [0, L]`, L = 1.2w. SsiDomain
  half extent 1, extent 1.

The locus is two arcs, each from a `v` side to the low `u` side.

- **On `origin/ssi/hermite-first`:** the Hermite from `(0, 0)` to
  `(0, L)` passes all three limbs over one window covering the wall.
  The op returns Ok with the wrong pairing, and 200 of 256 grid zeros
  lie more than 1 mm from any carrier.
- **On main:** the march pairs it correctly. Eight variants of the
  fold (L, a, c and the `x` range swept) all answer correctly or
  refuse loudly.

### Reachable on main: the R3 lane loses a short arc silently

`cylinder_sphere_ssi` takes every branch from the subdivision's seeds
and skips a seed whose Newton landing lies in an existing tube box
(`ssi.rs`, `cylinder_sphere_ssi`).

- **Fixture:** a unit cylinder about `z` and a radius-3 sphere about
  the origin meet in a circle at `z = √8`.
- **Slab:** centre `(cos 5° − 2, sin 11° − 2, 2)`, half extent 2,
  extent 4.8. Its faces cut the circle into a long arc (11° round to
  355°) and a short one (5° to 11°).
- **The short arc lies in the long arc's end box at rung 0.3.** It
  continues the long arc's end tangent, so every slice holds it once.
- **Result on main:** **Ok with one branch**. The short arc's seeds land
  inside the long arc's tube and are skipped, and accounting banks its
  cells. 5°..9° and 3°..11° slabs lose it the same way.

## Fixed (branch `ssi/limb3-one-arc`)

Where a search banks the tube (`certify::Banked`), limb 3 now proves the
chain holds the traced arc and nothing else, over each box cut to the
searched region (the wall's knot rectangle, the ℝ³ slab):

- each piece of the solution set in a box ends on its boundary at two
  points, so a boundary holding exactly two simple solutions holds one
  piece (`certify::boundary_zeros`: chart edges walked in monotone runs
  with a mean-value enclosure; `certify::face_roots`: ℝ³ faces by
  Krawczyk);
- consecutive boxes share a solution in their overlap
  (`certify::one_arc`, `certify::one_arc_r3`).

A rung whose chain is a graph but not one arc gives way to a narrower
one; with none left the certificate refuses `SsiError::TubeNotOneArc`.

**Rows:**
- `ssi_limb3_one_arc::the_fold_answers_its_two_arcs_paired_as_the_locus_pairs_them`;
- `ssi_limb3_one_arc::a_short_arc_in_a_long_arcs_end_box_is_traced`
  (fails on main);
- `ssi::certify::tests::a_graph_window_holding_two_arcs_is_not_one_arc`.

With the fix applied onto `origin/ssi/hermite-first`, every fold variant
answers correctly or refuses: limb 3 refuses the wrong-pairing Hermite,
and the march takes the branch.

At rest (`certify_rung3`) nothing banks the tube, and the probe proves
the graph alone: `limb3-at-rest-proves-the-graph-not-the-arc`.
