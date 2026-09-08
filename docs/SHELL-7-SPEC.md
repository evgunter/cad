# SHELL-7 — the axial door takes a one-surface corner

**Status: BINDING at dispatch (SHELL orchestrator, 2026-09-08).** Unit
`SHELL-7`, branch `shell/7-seam-corner`, block SHELL-B2 slot 1. Closes
`work/shell/axial-door-refuses-a-one-surface-seam-corner.md`. Deleted
at merge per `docs/DOC-LEDGER.md`; the item file is the record that
survives. Read `docs/prompts/implementer-discipline.md` in full first.
Survey against main at SHELL-6's merge (2026-09-08); citations by
symbol; a line number is a hint, not a claim.

## 0. The defect, stated as the missing fact

`topo::offset_charts_together` (`crates/topo/src/offset_axial.rs`)
solves a corner in the meridian half-plane `(ρ, h)` from the PROFILE
constraints of the distinct moved surfaces meeting it — a line or a
circle each — and refuses `TogetherAxialCorner` when fewer than two
meet and the vertex is not an axis pole ("one profile constraint meets
here and the vertex is not on the axis, so its station is determined
but its radius is not"). The full-period torus from
`sweep::tube_along_arc(.., TubeWindow::Full, ..)` has exactly such
vertices: its seam vertex is where the ONE torus surface meets itself
along two seam edges, so one profile circle meets it, and `shell` of
the solid torus and of its hollow twin both refuse there
(`crates/sweep/tests/verbs_shell.rs`,
`the_full_period_torus_refuses_at_the_axial_doors_seam_corner_solid_and_hollow_alike`).

The refusal's premise is wrong for this shape. A vertex ALL of whose
incident faces lie on one surface of revolution `S` is a point OF `S`,
and the offset of `S` by `d` moves every point of `S` by `d` along
`S`'s own normal — for a point on a profile circle (sphere, torus)
that is the concentric move about the circle's centre to the moved
radius; for a point on a profile line (cylinder, cone, a plane normal
to the axis) it is the perpendicular foot onto the moved line. Nothing
is undetermined: the door already USES this fact for the sphere seam
(`mint_carrier`: "a sphere's seam is a GREAT circle about the sphere's
own centre, and the offset is concentric") and for the carried-datum
rim corner (the concentric move when ONE moved cap fixes the azimuth),
and refuses only because that arm demands exactly one meridian plane
to solve the azimuth. At a seam vertex no plane contains the axis, and
the azimuth is what the module docs already say it is in that case:
"carried from the old vertex when no plane contains the axis at this
corner (the seam's own conventional datum)".

## 1. The change

In `solve_corner`'s single-profile branch (`profiles.len() < 2`, not a
pole): when `meridians` is EMPTY, the corner's meridian point is the
old point's image under the one profile's own offset —

- `Profile::Circle { rho_c, h_c, r }` (the MOVED circle): the
  concentric move `centre + r · unit(p − centre)`, with the same
  `offset_axial_datum_arm` decide the carried-datum arm makes (a
  corner standing at the circle's centre fixes no direction);
- `Profile::Line { n, c }` (the MOVED line): the perpendicular foot
  `p − (n·p − c)·n`. Say in the code which operand family reaches
  this arm (measure: does any body in the corpus have a vertex whose
  only surface is a cylinder, cone or plane with no second surface? a
  lamina or a degenerate; if none, the arm is written for
  completeness and its row is a hand-made operand, disclosed);

and the azimuth is carried from the old vertex through the existing
`meridians.is_empty()` path. The `what:` string of the refusal that
remains ("fewer than two … no point in the meridian half-plane is
determined") must then be TRUE of every case that still reaches it —
re-read it; a Line profile with no meridian is determined by this
unit, so what is left is a corner with NO profile constraint at all.

**Factor, do not duplicate**: the carried-datum arm and the new arm
compute the same concentric move; give `Profile` one method
(`image_of(ρ, h)` — the foot on a line, the concentric point on a
circle, refusing typed at a circle's centre) and have both arms call
it. Verification stays as it is: every corner is metered against every
moved surface at the end of the solve.

**The seam edges** of the full torus are two circles: the MERIDIAN
seam (centre on the tube-centre circle; `mint_carrier`'s torus seam
arm already moves it concentrically) and the LATITUDE seam (centre on
the axis, radius `R + r·cos v`; after the offset `R + (r + d)·cos v`
at the corner's own station — the rule every latitude circle follows
here, "keeps its normal and `u_ref` and takes the corner's own station
and radius"). The seam branch of `mint_carrier` currently certifies a
torus seam by its centre standing on the tube-centre circle, which the
latitude seam fails; extend the branch so a torus seam is EITHER a
meridian circle (concentric about the tube centre) or a latitude
circle (the standard latitude rule), decided by where its centre
stands (on the axis, or on the tube-centre circle; anything else
refuses `TogetherAxialEdge` as today). Measure first: build the
full torus, list its edges and their carriers, and put the list in the
PR body before writing the arm.

## 2. Acceptance

1. **The solid full torus shells.** `tube_along_arc(R = 2, r = 0.5,
   Full)` at `t = 0.05`: tier 3 green, 2 shells, volume
   `2π²R[r² − (r − t)²]` within `1e-9 + pad`; the cavity's minor radius
   read off its surface is `r − t` exactly.
2. **The hollow full torus shells** (SHELL-5's row, flipped): two thin
   tori, `2π²R[(r² − (r−t)²) + ((r−w+t)² − (r−w)²)]`, 2 solids, 4
   shells, per-solid `[Outer, Void]`. The SHELL-5 row's name and doc
   change to say what it asserts now; the issue file closes.
3. **The seam vertex moved exactly.** On row 1 the seam vertex's
   image is at meridian distance `r − t` from the tube centre `(R, h_c)`
   and at the OLD azimuth (bitwise: the old azimuth direction dotted
   with the new radial is 1 within 1 ulp), and both seam edges'
   carriers are the circles §1 names, with their endpoints on them.
4. **The old corpus is byte-identical**: every fixture in
   `torax_axial.rs`, `sf2b_axial*.rs`, `verbs_shell.rs`, the teapot and
   torus-vessel scenes — the same bodies (the SHELL-6 dump instrument,
   `shell5_r1_dump.rs`'s corpus, or a differential of your own at the
   merge base and head, cited in the PR).
5. **The refusal that remains is reachable and true**: one row that
   still refuses `TogetherAxialCorner` on the "no profile constraint"
   shape with the corrected `what`, built by hand if no door builds
   it, and said to be hand-made.
6. **The Line arm** (§1): a row per its measured reachability.
7. **A torus seam vertex at the tube's inner equator** (`v = π`, the
   seam placed by `u_ref`): if the tube door places its seam at
   another azimuth of the minor circle, rotate `u_ref` so a fixture
   puts the seam vertex at each of the four cardinal minor angles;
   every one shells to row 1's closed form.

## 3. Stops

STOP and report if a corpus body moves under this change (row 4), if
the latitude seam's carrier cannot be certified by the standard
latitude rule, or if the tube's seam vertex is incident to a surface
other than the torus (then the premise of §0 is wrong and the corner
is a two-surface corner refused for another reason — measure and say
which).

## 4. Docs and owed

`offset_axial.rs`'s module header: the corner table gains the row
`full torus  2 corners  1 [torus seam ∩ torus seam]` and the reduction
paragraph's step 1 gains the one-surface sentence; the "which refusals
a fixture reaches" section is re-read against what now reaches.
`docs/KERNEL-VERBS.md`'s shell row: one sentence ("a full torus
shells"). `crates/topo/README.md` unchanged unless its offset row
names the corner shapes. `offset_axial.rs` is SHELL's file; the tour
scenes are no program's (demo-purpose rule); nothing here is a seam.
Lane rules as every SHELL brief: own worktree, own `CARGO_TARGET_DIR`,
narrow builds, one heavy cargo job, no `Co-Authored-By` trailer in
lane commits, push after every coherent step, hosted CI is the gate
(nothing narrowed), report ≤ 150 lines.
