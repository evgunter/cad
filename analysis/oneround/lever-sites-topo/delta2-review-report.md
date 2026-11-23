# Second delta read — PR 7228, `oneround/lever-sites-topo`, `fdaacef8..1049847e`

Frozen head `1049847e47f93c87f62f6bb96e6859143f549c9f`; base, its second parent, main at
`37fcdf296c`. The unit's diff against that base is 30 files, +1182 −282. Earlier reports:
`review-report.md`, `delta-review-report.md`.

## Verdict

**APPROVE.**

The four findings of the last read are closed, each of the six claims holds, and nothing in this
delta is a finding above NOTE.

## What I executed, and what I only read

Base and head each in its own worktree and target directory, fuzz seed `0x00c0ffee00000001`.
Hosted CI has not run on this head; everything below is local.

- `scripts/run-gates.sh` with `GATE_BASE=37fcdf296c` on the head: exit 0, every gate OK
  (`lever-doors`: 36 pinned sites, none above the base's).
- `recourse_roster` alone on each side.
- The differential:

| Leg | ε | Base | Head | Difference in the pass/fail sets |
|---|---|---|---|---|
| `topo`, `--profile ci` | 1e-9 | 4026 pass | 4042 pass | the head's 16 rows, all passing; nothing else |
| `topo`, `--profile ci` | 1e-12 | 4026 pass | 4042 pass | the same 16; nothing else |
| `sweep` + `step-import`, shell/offset/hollow/axial/tilted filter, slow rows included | 1e-9 | 739 run, 1 fail | 740 run, 1 fail | the new tilted-cap row; the one failure is the same on both sides |

- Fourteen mutants behind an environment switch in one build of the head
  (`probes/delta2-mutant-switches.patch`), each over the 24 `offset_axial` rows in topo and over
  the sweep and step-import filter under `--profile ci` (646 tests).

**Not run:** the whole topo suite with its slow set (I ran it on `fdaacef8`, not here); the sweep
filter at 1e-12; ε = 1e-6; `mesh`; `editor-core`. The sweep mutant runs exclude the slow set.
**Only read:** the delta's code in `offset_axial.rs`, `reduce.rs`, `side_reader.rs`, the new sweep
row, the two rows under `work/oneround/`, and the unit's files as they stand after each merge.

## The claims

**1. Both bounds at the sines and the latitude posture — holds (sure).** Every `classify` sine
(`ALIGNMENT` for the plane, cylinder, cone, sphere and torus; `AXIS_PARALLEL`) and
`latitude_posture`'s tilt go through `validate::decide_levered` on `reach_from(pivot)`, each with a
straddle name. I read every reader of `frame.region` again: `pole_station` and the two senses take
the bound from below and act only on a definite sign; `plane_torus_section` takes one bound, by
stated deferral (claim 4). No consumer is left on one bound where the other sign is acted on.
A pose the two-bound read newly refuses and no row covers: I found none by running (the
differential), and one by reading that I could not reach, n1.

**2. The rows go red — holds (sure).**

| Mutant | Topo `offset_axial` rows | Sweep filter |
|---|---|---|
| alignment sine from below alone | red: the chart straddle row and the plane straddle row | red: the tilted-cap row |
| alignment sine from above alone | red: the same two | survives |
| axis-parallel sine from below alone | red: the axis-parallel straddle row | survives |
| axis-parallel sine from above alone | red: the same | survives |
| the sines levered from the frame's axis point | red: `a_charts_alignment_is_levered_about_its_datum` | survives |
| latitude tilt from below alone | red: the latitude straddle row | survives |
| latitude tilt from above alone | red: the same | survives |
| latitude tilt levered from the frame's axis point | red: `a_latitudes_tilt_is_levered_about_its_centre` | survives |
| `pole_station` from above | red: its from-below row | survives |
| `pole_station` levered 100 m up the axis | red: its pivot row and its from-below row | survives |
| the two senses from above | **survives** | **survives** |
| the two senses levered from the frame's axis point | **survives** | **survives** |
| `plane_torus_section` from below | **survives** | **survives** |
| `plane_torus_section` levered from the frame's axis point | **survives** | **survives** |

The sines share one closure, so "each `classify` sine" is two mutants by predicate name, not one
per chart kind. The sweep row where the axial door runs is red on the alignment read from below
alone, which is the direction that would hollow a tilted cap through the axial door.

**3. The torax row — holds (sure).** It is gone, and the text now says three older rows reach the
fallback arm.

**4. `plane_torus_section`'s text — holds (sure).** The comment (`offset_axial.rs:2462-2468`) and
the pinned-sites row say what changed against main (a bound from below about the frame's axis point
became the hull from above about the torus's centre). The turn-lever row lists the site and
`ScopeRegion`. A verdict that moves there: none in the differential, and the two mutants that put
main's side and main's pivot back change no test either.

**5. `12d78de097` as new code — sound (sure), with n2.** `vertex_face_side` refuses a face with no
certified boundary edge in its own words before it reads the normal. `beyond_report` answers a
faceless solid with an `Err` string for that solid and does not reach `selection_reach`'s
`unreachable!`.

**6. The merges — holds (sure).** The full gate run passes on the head against the base, so every
`*.counts` file is what its gate prints. The roster prints the same five lines on both sides
(`chain_parity.rs`, two in `pcurves/collapsed_side_rows.rs`, `pcurves/decision.rs`,
`spline_trim.rs`), all main's; the unit's own entries (`ALIGNMENT`, `AXIS_PARALLEL`,
`classify: names`, the iso row) are in. `git diff 37fcdf296c 1049847e47` touches only the unit's
thirty files, and in `join.rs` and the roster the difference from main is the unit's change and
nothing else, so no resolution dropped a line of main's.

## NOTE

- **n1 (unsure).** `reach_from`'s bound from below is the farthest scope *vertex* from the pivot. A
  scope whose only vertices sit at a chart's stored datum reads zero from below, so a definite
  tilt there straddles and escalates where the gate used to answer "not axial" and take the general
  door. I could not build such a scope through a kernel door and no suite reaches one. No row says
  what it should do.
- **n2 (sure).** `reduce.rs`: `vertex_face_side` calls `face_extent_below` to choose its refusal and
  `planar_axis` then calls it again for the lever. One walk of the boundary twice.
- **n3 (sure).** The four surviving mutants: the senses are cosines of ±1, which no lever moves, so
  their bound and pivot are held by reading alone; the torus section's bound and pivot wait on the
  turn-lever row, which now lists it.
- **n4 (sure).** `sweep::all germ_tilted_rod::other_spins_and_steeper_tilts_refuse_at_their_germs`
  fails on the base and on the head alike. It is main's, it entered my run only because I widened
  the filter with `tilted`, and it is not the unit's.
