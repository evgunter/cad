# Delta read — PR 7228, `oneround/lever-sites-topo`, `ad1b9352..fdaacef8`

Frozen head `fdaacef89cc891e72c6037c2cfaf4cd665280d8c`. The PR's base at that head is
`61403c47390f61545a24c739312e12ce8be6a87f` (main as merged at `f282042c`); the unit's diff against
it is 27 files, +1021 −304. First report: `review-report.md` beside this one.

## Verdict

**APPROVE-WITH-FIXES.**

The three MAJORs are closed. M1's frame now holds a region and each consumer takes a bound at its
own datum's pivot; M2's two respelled sites are pinned again; M3's roster line is listed. Every row
the fix pass claims goes red under the mutant it names, and my differential finds no verdict moved
outside the unit's own rows.

What is left is smaller than what was fixed, and none of it is a wrong answer I can produce:

- **d1.** Two consumers were handed one bound on the ground that "any other sign refuses". At one
  of `latitude_posture`'s two call sites, and for every `classify` sine as the *gate* reads it,
  the other sign is acted on. The bound chosen is the safe one of the two; the stated reason is
  false, and the README's rule for a verdict acted on at every sign is both.
- **d2.** `plane_torus_section`'s lever is described as "the bound it took before". Against main it
  is the opposite bound about a different point.
- **d3.** Three of the six consumers' bounds, and four of their pivots, have no row.
- **d4.** "The one row that reaches `corner_arms`' fallback arm" is the fourth.

The fixes I would ask for before merge are textual (d1's two reasons, d2's sentence, d4's
sentence) plus a ruling on d1: read both at those two consumers, or say in the code why the bound
from above is the one to keep there.

## What I executed, and what I only read

**Executed**, base `61403c47` and head `fdaacef8` each in its own worktree and target directory,
fuzz seed `0x00c0ffee00000001`:

- `lever-doors.sh`, `cover-kind-match.sh`, `lever-sources.sh` with `GATE_BASE=61403c47`, and
  `work.py lint`, on the head: all pass. 48 pinned sites.
- `recourse_roster::every_decided_name_has_its_door_and_every_site_is_read` alone on each side.
- Twenty mutants behind an environment switch in one build of the head
  (`probes/delta-mutant-switches.patch`): seventeen over the 29 rows of `offset_axial`,
  `separating::tests`, `straight_corner_rows` and `spline_wall_rows`; the three that survived
  there over the per-PR topo suite (4032 tests) and over the sweep and step-import axial filter
  (617 tests); and a panic in the axial `corner_arms` fallback over that same filter.
- The differential (below).
- Hosted run 44889521003 resolved to step level.

**Only read**: the delta's code in `offset_axial.rs` (every reader of `frame.region`),
`shell.rs::offset_door`, `separating.rs`, `cover.rs::face_extent_below`, `fate.rs`,
`offset_together.rs`, `solid_contain.rs::selection_reach` and its six callers, `spline_wall.rs`,
the roster, the three rows, the reader lines, the PR body. I did not rebuild the implementer's
per-decision logger, so its counts (1,687 / 784 / 17,055 / 28,736 logged, 3 moved) are not
re-measured; my differential is at the level of test verdicts. I did not run `mesh` or
`editor-core`.

One accident to report: a mutant build filled this container's disk part-way through. Two runs
wrote empty logs; I freed my own incremental build directories and re-ran both. Every number below
is from a run that completed.

## The differential

| Leg | ε | Base | Head | Difference in the pass/fail sets |
|---|---|---|---|---|
| `topo`, `--profile ci` | 1e-9 | 4020 pass | 4032 pass | the head's 12 rows, all passing; nothing else |
| `topo`, `--profile ci` | 1e-12 | 4020 pass | 4032 pass | the same 12; nothing else |
| `sweep` + `step-import`, the shell/offset/hollow/axial filter, slow rows included | 1e-9 | 616 pass | 617 pass | the new torax row; nothing else |
| `topo`, whole suite, slow set included | 1e-9 | 4325 run, 4321 pass, 4 fail | 4337 run, 4333 pass, 4 fail | the same 12 rows; the same four fail on both sides (`offer_rows`, two `a_corner_reads_only_its_own_sectors` rows, `recourse_roster`) with identical panic text but the roster's line number |

Not run: the whole topo suite at 1e-12, the sweep filter at 1e-12, ε = 1e-6.

## The adjudicated findings, one by one

### F1 — the axial frame — closed, with d1, d2 and d3

`Frame { axis, region: ScopeRegion { cover, vertices } }`, one reader
`reach_from(pivot) -> LeverBounds::Between { below, above }` (`offset_axial.rs:327-357`). It reuses
the existing vocabulary (`LeverBounds`, `validate::decide_levered`, `k_stats::straddle`); it is not
a second both-bounds spelling. Sign by sign, in the consumer:

| Consumer | What the code does on each sign | Bound handed | |
|---|---|---|---|
| plane alignment, `:1177-1189`, `:1195` | Zero → `Station`; other → the axis-parallel read | both, `decide_levered` | ✓ |
| plane `axis_parallel`, `:1203-1210` | Zero → `AxisParallel`; other → `TogetherNotAxial` | above | ✓ in the door; **d1** in the gate |
| cylinder, cone, sphere, torus sines, `:1224`-`:1266` | Zero → goes on to `on_axis`; other → `TogetherNotAxial` | above | ✓ in the door; **d1** in the gate |
| `latitude_posture`, `:2794-2805` | Zero → `Ok(None)`; other → `Ok(Some(text))` | above | ✓ at `:2607` (refuses); **d1** at `:2229` |
| `pole_station`, `:2743-2750`, used `:1562` | Positive → `c / n.1`; other → refuses | below | ✓ |
| `generator_sense`, `:2415-2431` | Positive / Negative pick a sense; Zero refuses | below | ✓ |
| `rim_sense`, `:2553-2570` | the same | below | ✓ |
| `plane_torus_section`, `:2483-2488` | several decisions | above only | disclosed; **d2** |

Rows claimed red, each confirmed: the bound from above handed to `pole_station`
(`a_pole_station_is_decided_at_the_scopes_reach_from_below`); the bound from below handed to the
sine (`an_alignment_is_decided_at_the_scopes_reach_from_above`); the plane read above-only and
below-only (`a_planes_alignment_straddling_its_reach_escalates`, red on each).

### F2 — the datum's pivot — closed

Each chart's sine is levered about its stored datum. The row
`a_charts_alignment_is_levered_about_its_datum` goes red when the sine is levered from the frame's
axis point instead. `pole_station`'s pivot (the frame's axis point) and the two senses' (the old
edge's middle) I read and did not test: `likely` right, no row (d3).

### F3 — the count — closed; twelve is right

The first head removed 15 references. The fix pass put three back: `extent_from`'s vertex term
*and* its zero seed (containment 2 → 4), and the ring segment (ring_path 1 → 2). That leaves twelve
removed, and `ScopeRegion::reach_from` adds one `ExtentBall::point` (the vertices from below), so
the gate falls by eleven: 59 → 48, 39 → 28. I counted thirteen because I had left `extent_from`'s
seed among the honest falls; with the seed restored beside the vertices, twelve.

### F4 — the roster — closed

Run alone, both sides print the same two lines (`chain_parity.rs: decide: name`,
`spline_trim.rs: decide: name`). The iso row's line is gone.

### F5 — `planar_axis` — closed

`cover::face_extent_below` is the largest `edge_extent_inf` over the face's certified boundary
edges: each edge is part of the face, so it is a bound from below, the one a definite-sign verdict
is owed. The row is red on the cover ball's diameter and on half the extent. The arm is shorter
than before on every face (a square reads its side, not its diagonal); the differential shows no
suite verdict moved by it.

### F6 — `IsoPiece::travel` — the argument holds (likely)

The predicate asks whether the arriving piece is "of some length". A piece whose ends meet round a
closed wall has one, and a chord says it has none. The doc, the audit table, the roster's reason and
the locals now say extent; the row adds the whole-turn piece and is red on the chord and on
`edge_extent_sup`.

### F7 — rows and refusals — closed, with d4

- `straight_corner_rows`: red on the spans swapped. The axial whole-body and uncovered-face-skipped
  rows: each red on its mutant, and also red on the scope's *vertices* widened to the whole body.
- `offset_together`'s `CarrierLaneUnsupported` has its own words and its row.
- **Unreachable, each checked.** `fate::piece_beside`: true, `edge_is_line` is
  `edge_line_span(..).is_some()`, which reads the certified curve (`census.rs:2354-2364`).
  `axial_frame`: true, `holds_face(first)` and `faces_in_scope()` apply the same predicate to the
  same map. `selection_reach`: true of its production callers (`point_in_faces` refuses an empty
  selection at `solid_contain.rs:2040`; `aimed_ray` is aimed at a face of the selection). One
  test-support caller is not covered: see n1.
- The fallback arm: **d4**.

### F8 — the rows and the body — closed, with d2's sentence

The pinned-sites row's title, counts, wrappers and caller count are right at the head; the germ row
cites by name and has its band; the turn-lever row moves `stands_on` and `sheets_part` to Zero and
marks the two `must_carry_description` callers unsure; the body's CI section and differential base
are the frozen head's.

### F9 — the three argued style items

- **S3 holds (likely).** One fold is gone with `extent_from`. The two left answer differently for a
  reason that is written down, and the cost of unifying them (a new `ReplaceFaceError` variant
  through an exhaustive match in another crate) is real.
- **S7 holds in part (unsure).** `in_plane::normal` no longer measures one face two ways for one
  purpose, which was the finding. But "a scale, not a lever" is a name: `:196` multiplies a pure
  number by that length and decides the product against the band, which is what a lever is. It is
  older than this unit and I would not hold the unit for it.
- **S10 holds (sure).** Production hands `face_cover`; the row lists the wrapper and what its rows
  hand it.

## New findings

### d1. Two consumers read one bound where the other sign is acted on — BY-INSPECTION, sure of the control flow, likely of its weight

**`latitude_posture` at the seam site.** `offset_axial.rs:2224-2232`:

```rust
if let Curve3::Circle(..) = old
    && latitude_posture(frame, circle.axis_line(), band)?.is_none()
{
    return Ok(latitude_circle(frame, p_start, *axis, *u_ref));
}
return match (&ca.old, old) { .. }
```

A Zero mints the latitude circle. Any other sign does not refuse: it falls through to the seams
"that are NOT latitudes" (the comment at `:2219-2223`), and the sphere arm at `:2264-2287` then mints
a great circle after checking the centre alone. So at this site the tilt is acted on at every sign.
At the other call site, `:2607`, the other sign refuses, and the bound from above is right. The
comment at `:2794` ("A Zero names the posture held, so the scope's reach … is read from above")
is true of one of the two.

**The `classify` sines, as the gate reads them.** `is_axial_in` (`:1041-1049`) turns
`TogetherNotAxial` into `Ok(false)` under the comment "The gate's own definite verdicts: this body
is not axial, and that is an answer", and `shell::offset_door` (`shell.rs:4290-4294`) acts on it by
choosing `OffsetDoor::General`. `offset_door`'s own doc states the contract: "An UNDECIDED axis gate
is not `General`: it escalates typed". A sine that is in band at the scope's true reach and
definite at the hull's is not undecided to this code; it is `General`. The fix pass's reason
(`:1183-1187`, and the PR table: "Zero (names the chart aligned); any other sign refuses") is true
inside the door and false at the gate, which calls the same `classify`.

Of the two single bounds, above is the safer here, and I think that is why nothing breaks: the
general door is valid for a body at any tilt, where the axial door on a misaligned body is a wrong
answer. So this is not M1 again. But it is the question the brief asks ("a consumer handed one bound
whose verdict acts on the other sign"), the answer is yes at these two, and the plane arm three
lines away got both bounds for the same shape of consumer.

### d2. `plane_torus_section` does not "keep the bound it took" — BY-INSPECTION, sure

The comment (`:2479-2482`), the pinned-sites row and the PR table say the section "takes the bound
it took before" and "keeps the single bound it took". On main it takes `frame.extent`, the scope's
vertices from the frame's axis point: a bound from below. At the head it takes
`frame.region.reach_from(torus_centre(..)).above()`: the hull, from above, about the torus's centre.
That is the first head's bound, not main's. Against the base this PR merges into, the section's
lever changes side and pivot, and the text says it does not. The implementer's logger and my sweep
leg agree no verdict moves.

The deferral names `oneround/turn-lever`. The open row for that unit
(`a-turns-lever-is-read-from-below-for-every-caller.md`) does not list this site.

### d3. Three consumers' bounds and four pivots have no row — DEMONSTRATED, sure

Mutants that survive the 29 targeted rows, the per-PR topo suite (4032) and the sweep and
step-import filter (617):

| Mutant | Topo | Sweep filter |
|---|---|---|
| `latitude_posture` reads the bound from below | survives | survives |
| `latitude_posture` levered from the frame's axis point, not the circle's centre | survives | not run |
| `generator_sense` and `rim_sense` read the bound from above | survives | survives |
| every `classify` sine reads the bound from below (red in topo on its row) | — | survives |

The two senses are cosines of ±1 and no lever can move them; the body does not list them among the
"scalings with no row". `latitude_posture` is one of d1's two, and nothing holds either its bound or
its pivot. `pole_station`'s pivot and the senses' pivot I did not mutate. The last line says the
sweep suite, where the axial door actually runs, is blind to the alignment bound: only the topo
rows built on hand-made frames hold it.

### d4. The fallback arm was already reached, three times over — DEMONSTRATED, sure

The body says `torax_the_klein_elbow_hollow_hollows_again` "is the one row that reaches
`offset_axial::corner_arms`' fallback arm". With a panic in the arm, the sweep filter fails four
rows:

```
shell_cavity_embedded::a_cavity_the_engine_cannot_certify_refuses_uncertified
torax_axial::torax_the_klein_elbow_hollow_hollows_again
verbs_shell::the_klein_elbow_opens_at_one_cap_to_its_measured_volume
verbs_shell::the_klein_wall_pair_seals_to_its_measured_volume_and_opens_to_the_hand_built_tube
```

Three are older than this unit, and `the_klein_elbow_opens_at_one_cap_to_its_measured_volume` is not
in the slow set: it runs on every PR. So the arm was never unreached in the workspace (my first report's "no row reaches
it" was the topo suite's, and said so), and the new slow-set row was added to reach something
already reached. With the old chord put back the new row passes, as disclosed. The row is a real
double hollow and may be worth keeping for itself; its stated purpose is not true.

## NOTE

- **n1 (likely).** `side_reader::beyond_report` (`:805-817`, test-support) calls `selection_reach`
  for every solid of a body. A solid with no face read a zero reach before and now hits the
  `unreachable!`, whose message says every caller holds a face.
- **n2 (sure).** Hosted run 44889521003, by step: every job that ran is green. `lint` ran its gates
  (8 m), fmt, clippy and rustdoc. `test (kernel)` ran default ε (1 m 37 s), the ε step (53 s), "the
  slow-set tests this diff adds or edits" (1 s; the new torax row takes 0.57 s here) and doc-tests.
  `test (app)` ran the same with a one-second ε step. Skipped: the slow set, and the jobs
  `prime the build cache`, `interval`, `python suite`, `demos`, `viewer app feature`,
  `step import` and `tools`. So hosted CI did not run the whole-suite leg that I and the
  implementer ran by hand.
- **n3 (sure).** `axial_frame`'s doc (`:1054-1056`) still says it returns "the scope's faces'
  farthest distance from it". It returns a region.
- **n4 (likely).** `ScopeRegion` (points from below, covers from above, read from a pivot) is
  private to `offset_axial`. It is the shape the turn-lever row asks for at about 41 callers. It is
  one type today; it is where I would look for the second.
- **n5 (sure).** A planar face with no certified boundary edge now has no axis, and
  `vertex_face_side` refuses it as a normal that "does not read as a unit direction over its
  extent" (`reduce.rs:673`). The normal was not read.

## Is anything in the delta a fresh instance of what it fixes?

- A second both-bounds spelling: **no.** The frame builds a `LeverBounds` and the one both-bounds
  consumer goes through `decide_levered`.
- A consumer handed one bound whose verdict acts on the other sign: **yes, two**, d1. Both got the
  safer bound.
- A count stated from memory instead of from a run: **yes, one**, d4.
- A sentence that says "kept" over a change: **yes, one**, d2.
