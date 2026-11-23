# Second delta review — PR 7219, `oneround/lever-sites-brep`, frozen at `bb12af18`

Delta read: `c65b9292..bb12af18`: the fix commit `8da54bb5` whole, and
the hand-resolved hunks of the merges `1de746ae` and `bb12af18`
(`git show --remerge-diff`). Merge base with `main`: `91e24317`. Earlier
reports: `review-report.md`, `delta-review-report.md`, beside this file.

## Verdict: APPROVE

Every claim checked holds. E1 is right for every curve kind; the merge's
one hand-made line is the same arithmetic; the text says what the tree
does; no decision moves, base against head, at either ε. What remains is
NOTE-level: the halves of the two-sided values this pass built that
nothing can tell apart (N1, N2), and one of main's lines that no fast row
pins (N3).

## What I executed, and what I only read

EXECUTED (probes under `probes/delta2/`):

- Every gate as CI runs them (`scripts/run-gates.sh` with
  `GATE_BASE=91e24317`, `payload-rung-sweep.py --check`,
  `work.py lint`) on a pristine checkout of the head: all green
  (`gates.log`). `lever-doors` reads 46 pins, judged against the new
  base's 58 (main dropped one of its own since the last head).
- Nine mutants planted on the frozen head, each over its crate's whole
  fast set (step-import 424 tests, geom-brep 2 506).
- A probe of every apex read `choose` makes over step-import's fast set.
- The differential: base `91e24317` against head `bb12af18`, my logger
  (counts and margin hashes), CI's build profile, one fuzz seed,
  **geom-brep and step-import with their slow sets**, ε = 1e-9 and
  1e-12. Not topo, sweep, mesh or the app crates this time.

ONLY READ: `8da54bb5`'s diff; both merges' remerge diffs; main's
`placement_of`; `coaxial_tangent_circle` and `plane_torus_read`; the
rows cited in E5.

**Hosted CI: run 45092389517 on `bb12af18` was still `queued` when I
finished** (created 01:03:55Z, no job started). Nothing of it is read
here; the gates, mutants and differential above are the verification.

## The claims

**E1. Holds.** BY-INSPECTION, `sure`; and by probe that it moves nothing
in the suite.

`edge_reach_bounds(carrier, span, apex, region.lever_from(apex))` is
`Exact(above)` on a line or a circle arc, whose cover's far distance is
the edge's own, and otherwise `Between { below: the farthest of the
edge's seventeen samples from the apex, above }`. Each sample is a point
of the edge, so that half is from below on every kind; the cover's far
distance is from above on every kind. `choose` then takes, per side, the
larger of the faces' run and the apex reach. The larger of two bounds
from below of two lengths is a bound from below of the larger length,
and the same from above. So `below` is from below and `above` from above
for every curve kind.

The implementer's account of why no row can go red is true as far as I
can test it. The mutant (below handed the apex reach from above)
survives step-import's 424 fast tests. The probe says why: of the 121
apex reads `choose` makes over those tests, 115 are on a circle and 6 on
a line, and in none do the two bounds differ. A fixture in reach? I do
not see one. Among analytic pairs the only readers of `below` that have a
cone arm are the cone's coaxial rows (`section_closed`'s cone branch),
whose edges are parallels; plane×cone reads the edge's region and never
this lever. So the two halves can differ only where a file prints a
coaxial pair's parallel as something other than a circle, and then only
matter where that reach passes the faces' run. That is the fixture the
implementer describes and did not build. `likely`.

**E2. Holds.** DEMONSTRATED for the gate, `likely` for the judgement.
`cover-kind-match` is green with one new reader line and **no pin
moved**: `cover-kind-match.counts` is unchanged against the base. The
base's own spelling (`Surface::Cone { frame, .. } =>
lever.max(region.lever_from(frame.origin()))`) was not pinned either, so
nothing left the gate's sight. The reader line's direction, two-sided, is
the right one: on a line or circle the half from below *is* the cover's
far distance, so a looser cover would decide a reading the band holds.
"Reads the reach outside the match" is a real separation. The match now
yields the apex, a datum the cone's form fixes, and nothing in the arm
names a region. The reach is read off the edge's cover and its samples,
which is the reading the gate's rule asks for; no kind-specific
construction of a region exists on either side of the match.

**E3. The two reds are red; both arguments hold, with one narrowing.**
DEMONSTRATED for the mutants, BY-INSPECTION for the arguments, `likely`.

- Red, as claimed: the spline pair's section handed `above` (on the new
  spline-wall row); `faces_run` returning below for both (on that row and
  `a_sphere_cap_is_read_to_its_pole`). Above for both is red on three.
- `coaxial_tangent_circle` handed `below`: survives. The argument is
  right about what reads it: of that function's four kind pairs only
  plane×torus takes the extent, and there only `pt_axis_in_plane`
  (`Margin::levered(a·n, extent)`, structural). A tangent circle needs
  the plane normal to the axis, where `a·n` is ±1 and the row is Positive
  at any extent past the band. **The narrowing:** "no import's outcome
  turns on the bound" is true of what *builds*. Where a plane nearly
  through the torus's axis reaches the tangent arm, the same row can read
  Zero, in band or Positive by the bound, and those are three different
  refusals (no circle; an escalation; no circle). No build turns on it.
- `above` without the apexes' reach: survives, and "no reader of `above`
  has a cone arm" is true: `coaxial_tangent_circle` answers `None` for
  every pair with a cone before it reads anything.

So yes: a two-sided value is being built whose one side almost nothing
reads (N1).

**E4. Holds.** BY-INSPECTION, `sure`; differential agrees.
`placement_of(p, T::zero(), region, band)` and
`placement(Occupant::point(p), region, band)` reach the same body with
the same centre and a radius of `T::zero()`: the same operations on the
same values. Main's `placement_of` had that one caller. Dropping it loses
nothing main pinned, because main pinned nothing there (N3). `bb12af18`'s
remerge diff is empty, as stated.

**E5. Each is true.** `sure` unless marked.

- The IMPORTGATE row's new section says what I found: not through the
  window; yes as a class through the shorter lever, up to √3; 158 tests'
  margins and no verdict (my figure, correctly attributed and scoped).
- `section_closed`'s doc line on a `below` past its `above`: true, and
  placed where the reader of that function will meet it.
- The dated section's `Option<impl NurbsLaneOf<T>>`: corrected.
- `edges_extent_inf`'s doc at 27.6° and 0.886, and why widths (linear
  where the farthest pair is quadratic): matches my measurement.
- `work/nearpoint/rim-arms-holds-a-level-arm-a-cone-never-reads.md`:
  says what the code does and what would close it.
- The 1e-12 sweep red: `work/tesslie/main-red-a-cylinder-wall-through-a-prism-corner-…`
  names the test (the nearpoint row did not). Not re-run by me this
  round (sweep was out of scope).
- The two sweep reds: `work/green/main-red-eleven-…` names both, and its
  new paragraph cites my D5 accurately. Not re-run this round.
- The three `nist_ftc_09` rows: `work/germ/main-red-axis-normal-snap-refuses-nist-ftc-09.md`
  names them with the payload I see (#5130, `step_snap_relation`), and
  they are red here on the base and the head alike at both ε.

## Notes

**N1. The `above` half of the import's run has one reader, and no build
turns on it.** BY-INSPECTION and by two surviving mutants, `likely`.
After E3, `extent.above()` is read by `pt_axis_in_plane` on the tangent
arm and nowhere else, and for a cone pair by nothing. The code says so in
a comment ("Only the half from below is consumed"). It is correct and
costs a cover read per edge. Whether a `LeverBounds` is worth carrying to
a single fork is a question for whoever next touches `tangent`.

**N2. The apex reach's two halves cannot be told apart by any file the
suite holds** (E1). Correct by construction, unexercised. The row being
filed for the fixture is the right schedule.

**N3. Main's cone-pair point placement is pinned by no fast row.**
DEMONSTRATED, `sure`; inherited from main, not this PR's. With
`cones::cone_ruled`'s `held` made always true, geom-brep's 2 506 fast
tests pass. A point given a radius of one is red, on the cylinder pair's
rows and the placement's own, not on any cone row. I did not run the
slow set against it.

## Differential

Base `91e24317`, head `bb12af18`, geom-brep and step-import, slow sets
included.

| ε | tests, base / head | `f64` decisions | `Interval` decisions | tests logging on both | counts that differ | moved margins |
|---|---|---|---|---|---|---|
| 1e-9 | 3 412 / 3 428 | 268 995 237 on both | 1 445 013 on both | 2 623 | **0** | 168 tests |
| 1e-12 | 3 412 / 3 428 | 317 057 031 on both | 1 609 560 on both | 2 605 | **0** | 169 tests |

The head's extra 16 tests are the PR's own rows. The moved margins are
the PR's known ones (`offset_section_planes_parallel` in 159 tests at
1e-9, the cone-coaxial and tangent readings, a few `ssi_*`); no count
moves with any of them. Reds are the same on both sides with the same
messages: at 1e-9 the three `nist_ftc_09` rows and one source-scan
roster; at 1e-12 those and `envelope_lemma_fuzz::every_arm_and_term_is_load_bearing_in_a_pinned_sweep`,
a pinned sweep my seed overrides.

This is the whole PR against its new base, not this delta alone. For the
delta's own change (E1), the probe above shows it alters no value the
fast set reads; for E4, the arithmetic is the same and the cone pair's
suite (`geom-brep::all cone_pair_section`) is in this row.

Mutants (`probes/delta2/mutants.log`), nine: red are below without any
apex reach (7 rows), the run's below for both (2), its above for both
(3), the spline section handed above (1), a point given a radius (4).
Surviving are the three the implementer names (below handed the apex
reach from above; the coaxial circle handed below; above without the
apex reach) and N3's.

## Style

Questions exercised on this delta: Q1, Q2, Q3 (by mutant), Q4, Q6.

- The fix for D2 did not mint a second spelling: it reads through the
  existing `edge_reach_bounds`. `sure`.
- The merge chose one of two parallel spellings main and this branch had
  each minted for the same thing (`placement_of(center, radius)` and
  `Occupant`) and deleted the other. That is the right way round. `sure`.
- The new comment in `tangent` argues from poses the function never
  checks ("a plane the file read tangent along a parallel stands normal
  to that axis"). It is true where a circle is built; as a statement
  about every caller it is the narrowing in E3. `likely`.
- `a_planes_cut_of_a_spline_wall_…` asserts the refusal by a substring of
  its message (`"the spline-wall section refused"`). It can go red on a
  rewording that changes nothing. `unsure` it matters.
