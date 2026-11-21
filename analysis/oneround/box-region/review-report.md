# Review r2 of PR 7225, `oneround/box-region`, frozen at `5c3bb9b9`

Reviewed: commit `5c3bb9b933bab6e715ac253840dbc9c1ba7f90d4` against its
merge base `1d320404`. Hosted CI: run 44859392555.

## Verdict: APPROVE-WITH-FIXES

The predicate is sound as far as I could attack it: across 365 placements
of curved bodies that truly reach past the box, none was admitted, where
the merge base admitted 82 of the same placements. No body inside the box
was called past it, and the placing door agreed with the gate on the box
in every case. The fixes owed are to what the PR tells Ev and to what its
rows pin, not to the fold:

1. The report back to Ev understates which valid bodies now refuse
   (MAJOR 1). The class is not "spline nets": sphere faces through their
   poles, torus faces with no window, and every spindle face refuse as a
   limit inside the box wherever their whole carrier pokes out of it.
2. Hosted CI ran none of the slow rows this PR re-asserts (MINOR 2).
3. Seven mutants survive the box rows, two of them verdict-changing on
   valid bodies (MINOR 3).

## What I executed, and what I only read

EXECUTED, in a detached worktree of the frozen head, one target dir of
my own (and a second worktree and target dir at the merge base):

- A differential of the box against a sampled oracle
  (`probes/sweep_tests_zz_review_r2_probe.rs`,
  `review_r2_differential_of_the_box_against_a_sampled_oracle`): 14
  revolved profiles (ball, three tori, torus parts with a cylinder, a
  lens, a dome, a spool, two spindles, a cone, a frustum, a fat torus of
  quarter arcs), each revolved fully and by 1.1, −2.3 and 4.4 rad, each
  under 14 rigid motions that stand its far coordinate within 40 m of a
  box face or corner. 756 placements. At each: the gate on the moved
  image, the placing door on the unmoved body, and the true largest
  `|coordinate|` sampled off the profile (under a millimetre short).
  Run at the frozen head and at the merge base.
- Boolean-trimmed sphere faces near the box's top (a dimple, a boss;
  the ball poled on `z` and on `y`), at the gate and the placing door.
- How the limit is said, at the gate and the door.
- 16 mutants planted in one build and chosen at run time
  (`probes/mutants-by-env.patch`), each run over 228 rows of `topo`,
  `sweep` and `editor-core` (`probes/run-mutant-rows.sh`: every
  session-box row, the two `tcost_k3_certificate` rows, the modules that
  hold the tests the PR body names under (a), slow set included).
- The f64 cost of the ask and of the structural gate, head and base.
- `scripts/gates/cover-kind-match.sh` with `GATE_BASE` at the merge base.
- The CI run resolved to steps, and the kernel job's log read at the
  slow-set step.

ONLY READ: the Interval and `Sym<f64>` lanes of the fold; the editor-core
row; `pncad-py`'s tags and census; the plate ledger; the PR's whole-suite
differential (three hours a row here; not re-run); the slow set at
ε = 1e-6 and 1e-12 (not run); every claim about ε other than 1e-9.

Everything of mine ran at ε = 1e-9 only.

## Findings

### MAJOR 1. The valid bodies that newly refuse are a far wider class than the PR reports. DEMONSTRATED, sure

The PR body's (c) lists two new limits on bodies inside the box, both
spline nets, and files one row for them
(`work/boxes/a-spline-faces-cover-is-its-whole-control-net.md`). That is
what the suites hold, because suite fixtures stand near the origin. It is
not the class.

Measured, same 756 placements at both commits:

| true state | merge base | frozen head |
|---|---|---|
| past the box | 82 admitted, 283 refused past | 0 admitted, 329 refused past, 36 refused as a limit |
| inside the box | 391 box-admitted | 306 box-admitted, **85 refused as a limit** |

Of the 85, 77 were fully admitted at the merge base (the other 8 are
refused by tier 3 at both commits). By shape:

| shape | in-box limit refusals | why the cover is loose |
|---|---|---|
| lemon spindle | 26 | a spindle's cover is its whole tube, and it has no inside read |
| lens of two torus parts | 18 | torus faces with no window: the cover is the whole tube |
| ball wedge (a ball revolved part-way) | 15 | a sphere face through both poles reads no window: the cover is the whole ball |
| apple spindle | 14 | as the lemon |
| torus outer part on a cylinder | 12 | as the lens |

The simplest one: a ball of radius 300 revolved 1.1 rad, turned to face
`−x` and stood with its axis at `x = 900`. Every point of it is within
900 m. The gate says: "the region the body's faces may occupy, as the
kernel bounds it, reaches 200 m past the 1e3 m session box, and no point
known to lie on the body does: the body may lie inside the box, but a
tighter bound of its faces is not built yet." The merge base admits it.

So the rule a user meets is: a body with a sphere, torus, spindle or
spline face refuses as a limit wherever that face's **whole carrier**
(ball, tube, control net) reaches past the box, unless the face has a
proven window. The overshoot is up to the carrier's diameter, not a
fraction of a net. Sphere zones between latitudes and boolean dimples do
have windows and are admitted (measured: the dome, and a dimple whose
ball tops out 55 m past the box).

What is owed before Ev reads it:
- (c) in the PR body states the class, not the two fixtures.
- The filed row names splines only. Sphere faces with no window, torus
  faces with no window and spindles are the same refusal and have no row
  that says the session box now refuses on them. (BOXES holds rows on
  some of those covers' looseness; none says a finished body refuses.)
- P3 looks low for a refusal of valid bodies that main admits, once the
  class is stated. That is the orchestrator's call; I flag it.

A wrong premise in my brief, on the same point: "a valid body with a
rational-arc face farther than about 707 m from the origin now refuses".
The net of a rational arc overshoots the arc by at most `(√2 − 1)` of
the **arc's own radius**, along the diagonal of its end tangents, and by
nothing along them. Distance from the origin enters only because the
suite fixture is scaled about the origin. A lofted arc wall refuses where
the wall stands within that overshoot of a box face, wherever it is.

### MINOR 2. Hosted CI ran none of the slow rows this PR re-asserts. DEMONSTRATED, sure

Run 44859392555, job `test (kernel)`, step "test (the slow-set tests this
diff adds or edits)": the log reads "Starting 0 tests across 0 binaries
(12 binaries skipped) … Summary 0 tests run". The step selects by test
names spelled in the diff's added lines; the PR edits the bodies of
`tcost_k3_certificate`'s two rows and their helper, and spells neither
name. The same step in `test (app)` took one second, so
`m10_sym_profile_interval`'s re-baselined plate ledger very likely did
not run either (not resolved to its log).

So the green run does not vouch for the two rows that assert the new
limit, nor for the re-baselined ledger. I ran the two `tcost_k3` rows at
ε = 1e-9: they pass, and go red under mutants 1, 3 and 4. I did not run
the ledger.

Jobs and steps of that run. Ran green: `change filter`; `lint` (work
tracker lint, three self-tests, ruff, actionlint, `scripts/gates`,
rustfmt, clippy `--all-features`, rustdoc); `test (kernel)` and
`test (app)` (build, default ε, ε 1e-6 and 1e-12 for the touched files,
the empty slow-set step above, doc-tests, slow-set name check);
`python suite`; `mesh budget meter`; `corrupt input`; `gate ok`. Skipped
whole: `interval`, `tools (k-lint, tess-lint, tess-meter)`, `demos`,
`step import (freecad)`, `viewer`, `prime the build cache`. Skipped
steps: the full slow set (both test jobs), `change filter self-test`.
The PR's claims about k-lint baselines and the Interval lane therefore
rest on the author's local runs alone.

### MINOR 3. Seven mutants survive the box rows. DEMONSTRATED, sure (on the 228 rows; the whole fast set was not run under them)

The author's five mutants each go red on the rows the PR body names
(confirmed: 1 face read dropped, 2 inside read dropped, 3 cover as a
point, 4 limit refused as past, 5 placed carrier unmapped). Two more of
mine are caught: the door reading stages 2 and 3 off the unmoved body
(12 rows red), the torus extreme's sign flipped (the torus row).

These survive all 228 rows:

| mutant | what it does | shown to change a verdict |
|---|---|---|
| 12: the placement filter of `face_extreme_points` dropped (`cover.rs:520`) | every candidate pole counts as a point of the face | yes: the in-box ball wedge above is then refused as **past** the box by 200 m, a false finding; a dimple past the box by 5 m reads 65 m |
| 6: stage 2 reads the stage-1 bound again | `face_cover` never admits | yes: every dimple whose ball pokes out is refused as a limit |
| 8: the gate drops `SessionBoxUncovered` | a body with an uncovered face that tier 3 passes is admitted | no test reaches `validate.rs:6669` or `:11038` |
| 15: an uncovered face counts as within at stage 1 | the move and placing doors admit it | not shown |
| 7: a Zero margin read as past | a body within the band of the box's edge refuses | not shown |
| 10: the cone's apex dropped from the stage-1 bound | | not shown |
| 11: the torus's eight compass points dropped | | not shown |

Mutant 12 is the one I would not leave: the PR's own soundness argument
for "past the box is read only off points on the body" rests on that
filter, and no row has a candidate that is past the box and off the face.
The wedge and the dimple are such rows.

(Mutant 16, the ruled hull not made convex, also survives, and is
equivalent: a cover's box reads supports, and `Convex`'s support is its
inner node's.)

### MINOR 4. The gate decides tier 3 on a body the box has not shown inside. BY-INSPECTION, sure

`session_box::asked_before_the_checks` (`session_box.rs:264`) hands
`SessionBoxUncovered` back and the gate runs tier 3 first. The README
bullet: "The door at which a region comes to exist asks it, before
anything is decided on that region." A body with an uncovered face and
no known point past the box may be past it, and tier 3's checks then
read margins on it, and report their findings in place of the box's.

The body is refused either way, so nothing is admitted. But the
deferral changes what the ratified sentence decides, the PR body
discloses it and the README does not, and its justification is a
measurement ("every such face the suites reach is a stored-data fault
check 1 names") with no guard: no test reaches the arm that refuses when
tier 3 finds nothing (mutant 8). Whether this is the README's "before
anything is decided" is Ev's to say, not the PR's.

### MINOR 5. `SessionBoxUncovered` ends on a recourse that cannot cure it. BY-INSPECTION, likely

Both new variants take `FindingFamily::SessionBox`'s one recourse,
"place the geometry nearer the origin, or model it at a smaller size"
(`finish.rs:855`). For `SessionBoxUnshown` that is the lever. For a face
with no cover (a poisoned net, a placeholder, an edge with no cover) no
placement gives it one. The PR body argues the recourse for "a loose
bound as for a true reach" and does not address the uncovered case.
Where a body's findings span families, the family heading over a limit
also reads "the body reaches past the session box" (`finish.rs:830`),
over a sentence that says it may lie inside.

### MINOR 6. `tcost_k3_certificate`'s two rows lose their subject at the default ε. BY-INSPECTION, sure

`exhausting_lofts` (`tcost_k3_certificate.rs:215`) stands the exhausting
arm down at ε = 1e-9 and asserts the box's limit instead. The rows'
subject (a certificate split across rounds; a refusing arm returning no
properties) now runs at 1e-12 only, in the nightly. The fixture was left
as it is rather than stood where its net fits the box.

### NOTE 7. Bodies truly past the box that refuse as a limit. DEMONSTRATED, sure

36 of 365. Spindles (21) by design: no inside read. Torus parts (13: the
part on a cylinder, the lens) and two more: the far point is not shown on
the face, so the refusal says "the body may lie inside the box" of a body
up to 37 m past it. Refused, so sound; the inside read of a torus face is
less complete than the PR body's account of it suggests. I did not trace
which placement declines.

### NOTE 8. An undecided box margin admits. BY-INSPECTION, sure

`within` and stage 2 (`session_box.rs:301`, `:355`) admit on anything but
a definite Negative, an escalation included. The merge base did the
same, on points. The README now says "in the box only where its cover
is"; an escalated cover margin is not that. At `f64` no finite body
reaches it. Unexercised at Interval.

### NOTE 9. The predicate decides on the region before it answers. BY-INSPECTION, likely

Stages 2 and 3 run band decisions (a sphere's chart trim, the ring-torus
test, each placement) on a body whose carriers already reach past the
box. I found no way for that to admit a body past the box beyond the
band, and say so only because the README's "before anything is decided"
is being read strictly elsewhere in this PR.

### NOTE 10. Off this unit: a finished body turned rigidly stops being finished. DEMONSTRATED at both commits, sure

In the differential, the gate on the moved image refuses with one
non-box finding where the unmoved body is finished: the apple spindle and
the frustum with a spindle cap, under generic turns (25 in-box cases at
the merge base, 17 at the head, where the box's limit now comes first on
the others). Main's, not this PR's.

## The claims

**C1. No body past the box is admitted.** Held under everything I ran.
By inspection each stage-1 arm contains its face: a plane or cylinder
face lies between boundary points on a line or ruling through it, a cone
face between the boundary and the apex, and the rest read the whole
carrier (the spindle's by `|R|`, a spline's net). Stage 2's torus window
is guarded against rings and wrapping loops; the open clockwise-face row
needs a face tier 3 refuses, and I could not build one that stands. Not
attacked: imported NURBS with non-positive weights (a net is then no
bound), periodic and spiric edges as hull members, the Interval lane.

**C2. The inside read claims only points on the body.** Held: no in-box
body was called past the box in 391 placements plus the dimples, whose
cut-off poles lie past the box. The torus formula is the true extreme
for a general axis. Between "nearly along `e`" and "along `e`" the read
keeps two points of the top circle instead of sixteen; that can only
turn a past-the-box refusal into a limit.

**C3. The two refusals.** Stage 1 Positive, Zero or escalated: admitted.
Negative or uncovered: stage 2. There Positive, Zero or escalated:
admitted; Negative: `Unshown`; a face with no cover: `Uncovered`. Then a
point definitely past: `OutsideSessionBox`; otherwise the limit. No path
admits an uncovered face. See NOTE 8 and MINOR 4.

**C4. The placing door equals asking the image.** Held: 756 of 756 agree
on the box.

**C5. (a)** No uncovered face in 756 gate asks of my shapes; consistent.
**(b)** Re-measured at `f64`, debug, mean of two runs, base → head:
brick ask 31 → 95–142 µs; ball 16–23 → 18–28; torus 29–42 → 50–58;
structural gate unchanged within noise (2–4 ms). The loft, Interval and
`Sym<f64>` figures I did not re-measure, and the cost of stages 2 and 3
(paid by every ask of a body whose carrier pokes out) is not in the PR's
table. **(c)** MAJOR 1. "Nothing moved the other way" holds in the
differential.

**C6.** One `session_box_reach` sample where stage 1 admits: true by
reading. The ledger's call columns are unmoved in the diff; that its
form counts grew because the reach's expression now holds each face's
supports is consistent with the code and unverified (MINOR 2).

**C7.** MINOR 3. On the third example: with the placing door reading
faces, a copy is admitted only where its cover is in the box, so no
later one-input operation can mint an edge past it. I found no door that
places a body without `asked_where_it_stands` (read, not run).

**C8.** `face_cover_unproven`, `boundary_cover_by` and
`face_extreme_points` are registered and the gate is green against the
merge base. They do restate existing arms: see Style 1.

**C9.** The module docs say what the code does. The two variants are in
`at_rest.rs`, `tags.rs`, the tag inventory and the binding census, in
the right family. Recourse: MINOR 5.

**C10.** The torus example's edges do reach 700 m. `curved_face_placement`
does answer `Trim(None)` for a spindle. The rows are true of what they
measured and too narrow (MAJOR 1).

**C11.** I met no red that was not of my own planting.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8 (`session_box.rs`
whole; `cover.rs` whole).

1. **Q1, sure.** `cover::face_cover_unproven` (`cover.rs:414`) is a
   third statement of one rule. `geom`'s `Surface::face_region`
   (`geom/src/cover/readings.rs:99`) already says it for a reader with a
   boundary and no decision: a plane's or cylinder's face in its
   boundary's hull, a cone's with its apex, the rest in the carrier. And
   `face_cover_in` (`cover.rs:307`) says it again as the first step of
   each arm. The new door's doc calls itself "the cover `face_cover`
   meets its proven constraints into", but `face_cover_in` does not call
   it: the two matches agree by hand, and the staging's soundness claim
   ("stage 1 holds stage 2's region") rests on that agreement.
2. **Q1, sure.** The fold over faces is written twice,
   `session_box.rs:209-213` and `:343-347`, and the `bounded` test twice,
   `session_box.rs:197` and `cover.rs:318`.
3. **Q5, sure.** `cover.rs:7` says geom's two readings "hold the only
   kind matches". The file now holds three matches on `Surface`'s kind.
4. **Q6, sure.** The deferral's reason is a measurement with no guard
   and no row (MINOR 4). The staging's cost claim is a table in a PR
   body.
5. **Q3, sure.** No row can go red on the placement filter, on stage 2,
   or on the gate's deferred refusal (MINOR 3).
6. **Q7, likely.** `reach_of` takes four closures; `cover_of_edge` is
   `edge_cover_where` in every caller but one test. `body_reach` is a
   test-only function documented as the file's first item.
7. **Q2, likely.** `face_extreme_points`'s doc carries the whole
   argument for which critical points can be extremes; nothing checks
   it. A row per kind with the extreme off every candidate would.
8. **Q4, unsure.** `OutsideSessionBox`'s doc still ends "Reported alone,
   before any check reads a margin on the geometry"; stages 2 and 3 read
   margins on it first (NOTE 9).

Ratings:
- Idiom and structure: 4. One fold, small functions, typed refusals;
  marked down for the third copy of the ruled-face rule.
- Test quality: 3. The rows pin the two measured examples and the limit
  at both doors; they do not pin the filter, stage 2 or the deferral.
- Doc and comment honesty: 4 for the code's docs; the PR body's report
  to Ev is the weak part (MAJOR 1, MINOR 2).

## On this branch

The remote branch was created at main's head, not the frozen head, and I
did not rewrite it. The probes and the mutant patch under `probes/` apply
to `5c3bb9b9`. I did not fetch or read the other reviewer's branch.
