# Second delta read r2 of PR 7225, `oneround/box-region`, frozen at `2e9fa0b6`

Read: the delta `1e81f303..2e9fa0b6` (the unit's own commits; the three
merges of main by `git show --remerge-diff`, of which `7c661b1c` and
`2e9fa0b6` have hand resolutions). Hosted CI: run 45165602738. Everything
of mine ran in a detached worktree of `2e9fa0b6`, my own target dirs,
at ε = 1e-9 unless a line says otherwise. I did not fetch or read any
other review lane's branch.

## Verdict: APPROVE-WITH-FIXES

Six of the seven claims hold under what I ran. The first does not: the
class table's sphere line is contradicted by a body a kernel door
builds (E1). That table is the report to Ev, so it is the fix owed.
Nothing I found is in the fold or admits a body past the box.

## Findings

### E1 (claim 1). MINOR. The sphere line omits an arm: a ball cut by a plane tilted to its poles reads the whole ball. DEMONSTRATED, sure

The table: a sphere face's cover is tight "when its boundary says which
side is the face (domes, caps and partial wedges probed all do)", and
its one row is the pole-to-pole wedge.

`sphere_window` takes the window only where the side is encoded **and**
`solid_contain::sphere_chart_trim` answers, and that trim's class is
latitude rims, meridian arcs and circles whose axis is square to the
polar axis. A boundary circle tilted to the poles is outside it.

Built by `topo::intersect`: a ball of radius 50 poled on `y`, cut by a
slab face turned 0.4 rad about `x` at 10 m from the centre. The kept
part's top stands 28.29 m above the centre. Lifted 960 m its far
coordinate is 988.3 m and its ball's is 1010 m. The gate: "the kernel's
bound on the body's faces reaches 10 m past the 1e3 m session box … the
body may lie inside the box or past it". Lifted 940 m it is admitted;
lifted 985 m it is past by 13.288 m, which is its true reach. The same
ball cut by a `z`-plane (a circle square to the poles) is admitted at
all three, its window read
(`probes/sweep_tests_zz_review_r2_probe.rs`,
`review_r2_delta2_cone_cut_along_its_axis_and_a_tilted_ball_cut`).

So the line should read: tight where the side is encoded and every
boundary edge is a latitude rim, a meridian arc with no pole inside it,
or a circle square to the polar axis; otherwise the whole ball. A
boolean's oblique cut of a ball is the common member, not the wedge.
The filed row (`a-sphere-face-through-both-poles-reads-its-whole-ball`)
names the wedge alone. The doc of `sphere_window` also lists a lone
vertex loop, null scaffolding, an escalated trim, an axis not decided
unit and a radius not positive; the table has none of them (by reading;
I built none).

Smaller omissions on the torus line, by reading, likely: the window also
needs the ring test to decide at the band and each row's certified
envelope to be within half the tube and half the ring's inner gap
(`torus_window_slack`), and a loop with a missing lift row reads none.

The rest of the table I could not contradict. The cut ring torus reads
its tube (50 m past, as on the last head) and now has its row and its
P2. Cuts of a slender cone ring by planes along its axis are refused by
the boolean before a body exists (`GermSectionOpenConic`, hyperbola),
so I found no door that leaves a cone face undecided either. The
differential's 85 are spindle and pole-to-pole sphere faces, as said.

### E2. NOTE. On this head hosted CI ran no rustfmt, clippy or rustdoc. DEMONSTRATED, sure

The lint job's gates step failed (on main's `NodeErrorClass` alone), and
the steps after it, `rustfmt`, `clippy (--all-features)` and `rustdoc`,
were skipped. The PR body lists the gates step as red and does not say
those three did not run. Its "clippy `-D warnings` … locally: clean" is
the only evidence for them on this head. I did not run them.

### E3 (claim 2). NOTE. One sentence is loose. BY-INSPECTION, likely

A spline net with an infinite control point (finite check fails, no
poison) has no poisoned datum to name and is told its surface "computes
a point that is not a finite number"; it stores one. Not built.
Everything else I read of `UncoveredCause::of` is taken off the body
and true of it.

## The claims

1. **The class.** E1. Every other line agrees with `topo::cover` and
   `geom::cover` as I read them; all eight surface kinds and the edge
   kinds are present (a periodic cubic rides the spline edge line).
2. **The refusal sentences.** Hold. The row
   `a_face_with_no_cover_refuses_at_the_gate_alone_by_its_cause` is red
   under mutant 8 (the gate dropping the uncovered refusal), 23 (only
   the first face named) and 24 (geom's cylinder axis guard dropped). The lost subjects are asserted again where the PR body
   says (read, and their rows ran green in my 321).
3. **One home.** Holds. `convex()` on a boundary is written only in
   geom's `face_region`; topo's cone arm passes `Apex` and builds no
   hull; step-import reads the same door. The deleted topo guard caught
   nothing geom's does not: with geom's guard out (mutant 24) the
   NaN-axis row is red.
4. **The cone's apex.** The argument is sound, and stronger than
   written: on a face across both nappes every ruling leaves at the
   boundary both ways, so each point lies between two boundary points.
   A cap (one winding loop) keeps its apex: mutants 20 (one winding
   loop read as clear of it) and 22 (the apex always off) are red on
   the cap rows; 19 (two winding loops read as missing it) on the band
   row. I could not build
   a cone face whose read under-covers it.
5. **The counts.** The differential on this head: 289 admitted, 17
   refused by tier 3, 85 a limit inside the box; 329 past and 36 a
   limit past it. The same as my two earlier runs. The ledger: at main
   `dd8e7019` the plate row fails, reading `Plain/Assertion` 1051 calls
   12370 forms, `Early/Assertion` 1051 / 12820, `Door/Decision` 554 /
   12512, and `Plain/Decision`'s new digest; the head pins 12382, 12832
   and 12524 with the same calls and that digest. So the pin is main's
   move plus this unit's 12 forms on three lines, judged against main's
   own reading. I did not build `56a16cc2`.
6. **The merge.** The two hand resolutions keep both sides: `place(…)`
   on a placeholder's point and the injected `edge_cover` closure from
   the unit, the edge views from main. All 67 gate scripts print OK on
   this head against base `dd8e7019`.
7. **Cost and text.** The cost lines name `validate_geometric_structural`
   and quote both ranges. The text items read true.

**The reds, each checked on main `dd8e7019` here:**
`payload-rung-sweep.py --check` prints the one `NodeErrorClass` line on
main and on the head, and nothing about `UncoveredCause`; the slow-set
entry `e6_drive_over_a_loft_at_interval::an_arc_loft_bisects…` is in
main's `.config/nextest.toml` and names no test on main;
`recourse_roster` fails on main with the same six lines hosted CI
printed (`live.rs:2032`, `chain_parity.rs`, two in
`pcurves/collapsed_side_rows.rs`, `pcurves/decision.rs`,
`spline_trim.rs`), none in this PR's diff; `square_search` at seed
`0x9c64e8fd5d2bf505` gives 4 mismatches on main. The plate ledger is red
on main and green here.

**Hosted run 45165602738, by step.** Failed: lint's gates step; the app
job's slow-set-names step; the kernel job's slow-set step (78 ran, 2
failed: the fuzzer and the roster). Ran green in the slow-set steps: all
four `tcost_k3_certificate` rows, the two plate ledger rows, one
`intent_e` row. Skipped: lint's rustfmt, clippy and rustdoc (E2); jobs
`interval`, `tools`, `demos`, `step import (freecad)`, `viewer`.

## Mutants

Re-planted on this head (`probes/delta2-mutants-by-env.patch`), each
over 321 rows of `topo`, `sweep`, `editor-core` and `step-import`
(`probes/delta-run-mutant-rows.sh`): 16 of 16 are red.
They are 1, 2, 4, 5, 6, 7, 8, 10, 12, 13, 14 as before, and 19, 20, 22,
23, 24 named above. Last read's survivor (the nappe guard) has no
mutant now: the guard is gone, on the argument under claim 4. The whole
fast set was not run under any of them.

## Style

Exercised: Q1, Q3, Q5, Q6.

1. Q5, sure. E1: the table's sphere line is a summary of
   `sphere_window`'s doc that dropped half of it.
2. Q6, likely. "No door I found" appears three times in the table as
   the reason a line has no member at rest. It is a search result with
   no guard; the doc of `face_extreme_points` now says the like of its
   torus claim plainly ("no check holds a new door to that"), and the
   table could say the same.
3. Q3, sure. No row can go red on E1's arm: no fixture has a sphere
   face with a tilted rim near the box.
4. Q1, sure. Nothing left: the ruled rule has one statement.

Ratings: idiom and structure 5 (one rule, one fold, the guard gone on an
argument). Test quality 4. Doc and comment honesty 4 (E1, E2).
