---
id: cylinder-wall-pair-meeting-in-an-interior-loop-while-crossings-exist-elsewhere
kind: issue
title: Two cylinder walls meeting in a saddle loop interior to both faces, while crossings exist elsewhere, come back as a valid wrong body under every op
status: closed
refs: [torus-face-meeting-a-partner-only-in-an-interior-loop-while-crossings-exist-elsewhere]
opened: 2026-09-28
priority: P0
cost: M
closed: 2026-09-28
---


## What

The interior-loop class, measured on the cylinder. **On main every op
returns a body that is valid at tiers 1–3 and wrong.**
`ops::interior_loop_verdict` (`crates/topo/src/boolean/ops.rs:703`)
gates only torus and sphere faces. `cylinder_extent_gate`
(`ops.rs:2337`) guards only the no-crossings fallback.

**The fixture** (GERM measurement lane, 2026-09-28, on `378f66744`):

- **A** = `common::germ_pair::cyl(1.0, 2.0)`: radius 1 about `z`,
  `z ∈ [−2, 2]`, `vol = 4π`.
- **B** = an arc prism ∪ a planar bracket:
  - **The arc prism.** A profile in the `yz` plane (sketch
    `Mat3::from_cols(y, z, x)` at `x = −3`), extruded 6 along `x`.
    It is a 240° arc of the circle centred `(y, z) = (1.3, 0)`,
    `r = 0.5`, from `(1.55, +0.433)` through `y = 0.8` to
    `(1.55, −0.433)` (bulge `√3`), closed by the rectangle to `y = 2`.
    Its cylinder face is PARTIAL. It meets A's wall in one saddle loop
    (`y ∈ [0.8, 1]`, `|x| ≤ 0.954`, `|z| ≤ 0.5`) that touches no edge
    of either body: the arc's line edges sit at `y = 1.55`, and A's
    seam and rims are clear of it.
  - **The bracket.** The polygon `(−0.1,1.8) (0.1,1.8) (0.1,2.3)
    (1.65,2.3) (1.65,0.2) (1.85,0.2) (1.85,2.5) (−0.1,2.5)` in `(y, z)`,
    extruded over `x ∈ [−0.1, 0.1]`. Its pin (`x, y ∈ [−0.1, 0.1]`,
    `z ∈ [1.8, 2.3]`) pierces A's top cap, and those four pierces are
    the op's only crossings.
- **Closed form:** `vol(A ∩ B) = 0.008 + lens`, where
  `lens = ∬ 2√(1−y²)` over the arc disc with `y < 1` `= 0.0820944`
  (scipy `dblquad`). So `vol(A ∩ B) = 0.0900944`.

| op | returned | volume | true volume |
|---|---|---|---|
| A ∪ B | `Ok(Seamed)`, valid | 18.860430 = vA + vB − 0.008 | 18.778336 |
| A ∩ B | `Ok(Seamed)`, valid | 0.008 (the pin alone) | 0.0900944 |
| A ∖ B | `Ok(Seamed)`, valid | 12.558371 | 12.476276 |
| B ∖ A | `Ok(Seamed)`, valid | 6.294060 | 6.211965 |

- **`point_in_solid` witnesses.** `(0, 0.9, 0)` and `(0.3, 0.88, 0.1)`
  are `In` both operands. They read `Out` of ∩ and `In` of both
  differences.
- **Grid.** A 14³ grid over `[−1.2, 1.2]² × [−2.2, 2.2]` gave 10 wrong
  points of 2747 per op. On ∪ the wrong points are B points outside
  A, read `Out`: the result carries A's whole wall and B's whole arc
  face, which cross in the loop. The shell is self-intersecting and
  still passes the tiers.
- **Tilted, the same.** B rotated `0.15` rad about `y`, so the axes
  cross at an angle: all four ops again come back `Seamed` and valid,
  with 10 wrong points each. ∩ = `0.0089085` (the pin alone), and
  ∪ = `vA + vB − ∩`.
- **Control, without the bracket.** A full rod at `y = 1.3` gives no
  crossings, and every op refuses at `FallbackExtentUnsupported`
  (`cylinder_extent_gate`). The class opens only once a crossing
  elsewhere moves the op off the fallback.

**The replay** is the fixture above, built from `sweep::extrude`,
`profile::test_support::bulge_loop` and `topo::union` (prism ∪
bracket). The full probe file is kept with the measurement lane's
report.

## Home

GERM, beside `interior_loop_verdict`. Two ways to close it:

- **A cylinder half in the guard.** Refuse on reach, as the torus half
  does, or per pair. This fixture's wall pair has NO event, so either
  form refuses it. A per-pair rule that clears a pair WITH an event
  needs its own argument, the sphere half's analogue: two walls' section
  is one loop that encircles neither axis, or two loops that each
  encircle the thinner wall's axis and so cross its seam lines. That
  argument is unmeasured here.
- **The section certificate (b)** on the torus item. It is kind-generic.

## Stopgap (GERM cylinder-guard PR, 2026-09-28)

`ops::interior_loop_verdict` gains a cylinder half, per pair, raised
as `CurvedPairUnsupported { site: InteriorLoopGuard }` like the other
two. Rows: `crates/sweep/tests/germ_interior_saddle.rs`. Both saddle
fixtures (perpendicular and tilted `0.15`) refused every op; on the
base each op answered the valid wrong bodies tabled above.

- **Sphere or torus partner:** left to the halves above. The torus
  half refuses every overlapping cylinder pair on reach, and the
  sphere half owns sphere × cylinder.
- **Plane partner** (`ops::ellipse_misses_a_face`): a pair with an
  event passes, because the section is one ellipse or rulings. With
  no event the pair clears only on a certificate:
  - the ellipse's axial extent `2r|n⊥|/|n·â|` exceeds either face
    box's width along the axis; a plane parallel to the axis passes
    this at `2r`;
  - or one of four ellipse points is outside a face.
- **Wall partner** (`ops::wall_pair_clear`), for axes decided skew.
  Over the thinner wall, the section is one saddle loop when
  `s > r_k − r_t`, else two loops, one per branch, about the thin
  axis:
  - a saddle loop clears on any event of the pair, or one point
    outside a face;
  - each branch clears on an event on it, or one of its points
    outside a face;
  - with equal radii, all four branch-sign quadrants need an event.
  - A reach test clears walls whose boxes keep them off the other
    carrier, at any angle. Parallel axes that fail it refuse.
- **Any other partner kind** refuses on reach.

**What it costs.** Across the whole `topo` and `sweep` suites only
`m9_3_zip`'s tube chain moved. Its union refused at (B wall × A end
cap) until the plane clause tried four ellipse points instead of one:
the first sat exactly on the tangency point `(2, 4, 1)`, on both
faces' boundaries. After that no answered row refuses.

**Still open.** The section certificate (b) remains the proper fix,
the stopgap being removable once it lands. Separately, the sphere
half's with-event clause for a cylinder partner rests on the premise
`cylinder_extent_gate` states: every wall face carries a meridian
boundary edge. A ball the wall passes through meets it in two loops,
and one event does not cover both unless each loop crosses a
meridian. A seamless periodic band would void that. The clauses here
do not rest on it, because the wall-pair clause asks for evidence on
each branch.

Under that premise the two-loop branches (the per-branch and
four-quadrant rules) and the plane certificates are dormant defence.
A loop that encircles an axis cannot lie wholly inside a valid face
without an event, so those clauses decide over-refusal only, and no
row can show them refusing. Only the saddle clause is live.

One tolerance-level hazard is argued but unmeasured. In the band where
the saddle margin is undecided (`|r_k − r_t − s| ≲ tol`), the two
branches meet at a shallow angle, and a contact vertex within
tolerance of both carriers can sit on the wrong side of
`sign(m̂·(p − o_k))`. An event on one branch could then clear an
interior loop on the other. That needs a face boundary within about
tol of the loop that never touches it: a near-tangency, which the
section certificate refuses as R-tan.
