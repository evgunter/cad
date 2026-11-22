# Delta review of PR 7227 (`oneround/unsourced-bits`): `a686ebb5..2e721a928be2`

This reviews the fix pass on the frozen head `2e721a928be25c2865b538a0d0f84a353a550cde`.
That head merges main at `3ef71034a2`; `origin/main` has since moved to
`3900eb46`, which this does not read. The brief named the head as
`2e721a92ff`, which resolves to no object; the branch tip `2e721a928b`
shares its first eight hex digits, and that is the commit reviewed here.
The style lane is `docs/prompts/reviewer-style-lane.md`.

## Verdict: APPROVE-WITH-FIXES

Every adjudicated fix (F1–F6) is in the tree as claimed. The counts move
exactly as claimed, and my own re-run of the fast-set differential gives
the implementer's numbers. Nothing in the delta is MAJOR.

Four MINORs remain:

1. `corner_arms`' new doc says every non-circle carrier reads its chord.
   For an ellipse, a spiric or a spline, `edge_extent_inf` reads more than
   the chord. And the two-bounds-equal argument the PR leans on holds only
   for a line.
2. A fresh instance of the defect F2 closed: the restored `intersect`
   markers still name `levered_inv(size, …)` as "its honest door". The
   stand-ins row now calls that spelling a bare division wearing a
   door's name.
3. The PR body credits the sweep row
   `offer_rows::every_sphere_offer_passes_just_below_it` to a row that
   does not name it.
4. The 1e-12 lily red is owned by a row that names the scene's stop but
   not this test.

## What I executed and what I only read

**Executed**

- **Decision-hash differential**, fast set (geom-brep, topo and sweep;
  9408 rows), my patch applied byte-identically to main `3ef71034a2` and
  to head `2e721a928be2`. Seed `0x5eed`, nextest `ci` profile:

  | ε | rows run | rows with decisions | decisions compared | verdict moves | rows with bit moves |
  |---|---|---|---|---|---|
  | 1e-9 | 9408 / 9408, all pass | 7971 | 198,957,822 | **0** (bar the 4 nondeterministic `validate::tests` rows) | 364 |
  | 1e-12 | 9408 / 9408, all pass | 7952 | 199,848,352 | **0** (bar the same 4) | 363 |

  Bits move only at:
  - `line_quadric_cone_lead`: 230 rows at 1e-12;
  - `pm_census_ee_parallel`: 109, across f64, `Interval` and `Sym<Interval>`;
  - `germ_side` and `germ_order`: 45;
  - `offset_axial_corner`: 18;
  - `offset_together_corner`: 5;
  - `offset_axial_cap_pair` (Interval): 1.

  The `pn_apex_*` and `pm_census_ef_curved_wall_side` predicates no longer
  move. This agrees with the implementer's table (364 / 363). The
  `offset_axial_corner` bits that remain come from the line arm (the chord
  against `|Δt|`) and the fallback.
- `lever-sources.sh`, `lever-doors.sh` and `cover-kind-match.sh` on the
  head: all green, 52 unsourced sites.
- Marker counts: 62 on main and 52 on head.
- CI run 44879067356 resolved to step level through the jobs API.

**Not re-run.** I judged that the delta's code does not need these:

- **The 108 slow rows and the tour.** The delta's code changes are three
  reversions to the base's own text, plus a circle arm restored to the
  base's read. The surviving conversions are the ones I already measured
  at `a686ebb5`, on every suite: no verdict moved there, and the PR's own
  re-take reports 0. A fresh slow or tour run would re-measure main's
  merged-in changes, not this PR's.
- **The torn-body seed reproduction on main** was run, after the rest:
  see "Inherited reds".

**Only read**

- The whole PR diff against its merge base, and each fix commit.
- `replace_face`'s corner loop and `offset_derive::analytic_root`, read on
  each sign.
- Each restored marker, the unit row and the parent row.
- The stand-ins and unions rows.
- The rows the PR body cites for the base reds.

## The adjudicated fixes

### F1: holds. The consumer claim is true.

- **The circle arm** is back on `Lever::unsourced((t1 − t0).abs() * radius)`,
  in its own match arm, with the base's marker text
  (`crates/topo/src/offset_axial.rs:1254-1259`).
- **Every other kind** reads `carrier => geom_brep::edge_extent_inf(carrier, t0, t1)`
  (`:1261`).
- **The doc** makes no refuse-on-Zero claim any more.
- **The pins:** `offset_axial` is at 1 in `lever-sources.counts`, and
  `ExtentBall::point` falls 3 → 2 in `lever-doors.counts`.

**The consumer claim: true, by inspection, `sure`.** It concerns
`edge_extent_inf`'s narrowed sentence, which names `topo`'s `replace_face`
corner root (`crates/geom-brep/src/extent.rs:466-469`).

- `Incident::extent` is `corner_arm(carrier, t0, t1) = edge_extent_inf`
  (`crates/topo/src/replace_face.rs:4212-4220`).
- `analytic_root` (`crates/topo/src/offset_derive.rs:1005-1068`) decides
  `offset_corner_graze` at that arm, on every iterate:
  - **Zero**: returns `Err(CornerVerdict::Graze)`.
  - **Positive**: steps on, and returns `Ok(t)` only from a root it
    certified Positive.
  - **Escalation**: propagates as `Escalated`.
- In the corner loop (`replace_face.rs:4645-4648`), a `Graze` is pushed to
  `grazed` and `continue`s, so no point is taken from it.
- After the loop (`:4664-4705`), a grazed surface that no other candidate
  covers refuses (`CornerSection` with the graze verdict). One that is
  covered yields to the other candidates' points, each from a root decided
  Positive.

So the Zero is a refusal, or a hand-off to roots certified on their own
Positive. It is never a served point. The from-below bound is the README's
rule for that consumer, and the sentence is now true of what it names.

### F2: holds, but the restored marker text is a fresh instance (MINOR 2).

- The five `intersect` quotients are byte-identical to main, and
  `intersect.rs 5` is back in the pins.
- The stand-ins row owns them
  (`work/oneround/unsourced-levers-the-stand-ins-and-the-carrier-terms.md:25`).

### F3: holds.

- `census::edge_in_curved_face`'s wall sides are byte-identical to main:
  `Lever::unsourced(w.arm)` with the base marker
  (`crates/topo/src/census.rs:4718-4720`).
- The "scaled to" comment is gone.

### F4: holds.

- The certify twin is back on `Lever::unsourced(domain.extent)` with the
  base marker (`crates/geom-brep/src/ssi/certify.rs:3889-3891`).
- `lever-fixtures` geom-brep is 484, as on main.
- The argued wrapper hole is real: a helper returning production's lever
  would be a function wrapping a pinned site, the hole `lever-doors.sh`'s
  header names. `likely`.

### F5: holds, with one wrong attribution.

- (a) The PR body says the earlier probe ran over geom-brep, topo and the
  tour, with sweep in none of them.
- (b) It cites the roughly 20 sweep rows that catch a chord-only read.
- (c) The row `a_half_turn_rims_corner_arms_are_its_diameter` is deleted,
  with its reason.
- (d) "stacked" is gone from the title and from the unit row
  (`59e6e11f`), and the base lines are corrected (`join.rs` 8622 and
  2271).
- The base-red list now holds five slow rows. One of them is attributed
  wrongly (MINOR 3).

### F6: holds.

These comments now say that the lean scales the reading and the sector's
metering arm levers it:

- `star::order::leaned` (`crates/topo/src/star/order.rs:36-39`);
- `boolean::insert::walks_after` and `fan_turn`;
- `boolean::sectors::rays_within`.

`leaned`'s doc goes further: "Its caller levers it at the sector's arm,
which is the metering arm". That is true at `order_sine` and `fan_turn`.
For `vtxfac`'s callers I traced the germs' arms to `sectors[t].arm`
(`crates/topo/src/boolean/vtxfac.rs:1379`) but not every caller. `likely`.

### The counts: hold, `sure`.

Against `a686ebb5`, `lever-sources.counts` rises by exactly the 8
reverted references, and no new site is added:

| file | change |
|---|---|
| `intersect.rs` | +5 (line added) |
| `ssi/certify.rs` | 1 → 2 |
| `census.rs` | +1 |
| `offset_axial.rs` | +1 |

`mate/value.rs` falling 4 → 1 is main's. Against the merge base, the PR
removes 10 references:

| file | references removed |
|---|---|
| `line_quadric` | 1 |
| `fan` | 2 |
| `join` | 1 |
| `census` | 1 |
| `chord_rows` | 1 |
| `offset_axial` | 1 |
| `cut` | 1 |
| `order` | 2 |

That gives 62 markers on main and 52 on the head. `lever-fixtures`
geom-brep moves 485 → 484, and topo is 306 (300 on main, plus 6).
`geom-core 10 → 25` is main's. No count was moved by hand away from its
site: each gate is green on the head.

### The tracker

- **The unit row and the parent row's "Read against the tree"** each say
  what this PR takes and where each moved site went. The circle arm goes
  to unions (the row exists: `unsourced-levers-the-sums-the-arc-lengths-and-profile.md:27`,
  with my probe as its row). The quotients go to stand-ins (`…-the-stand-ins-and-the-carrier-terms.md:25`).
  The wedge and the twin go to the records unit.
- **The records unit has no row file.** It is a named unit in
  `work/oneround/plan.md:28`, which the style lane accepts as a schedule.

## Findings

### MINOR 1. `corner_arms`' new doc says every non-circle carrier reads its chord. Only a line does, and the bounds argument covers only a line.
BY-INSPECTION · `sure` on the text, `likely` on the substance.

**The doc** (`crates/topo/src/offset_axial.rs:1227-1229`): "A line reads its
chord ([`geom_brep::edge_extent_inf`], whose two bounds are equal on a
line), as in the planar door; so does every carrier but a circle".

**What the code reads.** For an ellipse, a spiric or a spline,
`edge_extent_inf` is `max(chord, fold at the minor radius, the farthest of
17 sampled points)` (`extent.rs:525-538`), not the chord.

**Why the bounds argument stops at the line.** The PR body repeats it:
"On a line its two bounds are equal, so the verdict's acting on every sign
does not matter here". Only on a line are inf and sup equal. For the other
kinds, this verdict acts on every sign: a Zero serves `tangent_foot` (the
F1 adjudication). The arm is read from below only, which is the shape F1
closed for the circle.

**Why it is not worse than the base.** The base read the chord, which is
at most `edge_extent_inf`, so the new arm sits nearer the upper bound. At
the 12 spiric reaches in the suites, it reads the chord bit for bit.

**What the unit row says.** "Moved to other units at review" covers only
the circle. The unit's "Converted" list names `corner_arms`' line arm but
not the fallback, which the body says this PR converts.

### MINOR 2. The restored `intersect` markers still claim `levered_inv` as "its honest door": a fresh instance of what F2 closed.
BY-INSPECTION · `sure`.

**The markers.**
- `crates/geom-brep/src/intersect.rs:2446-2447`: "its honest door
  `levered_inv(size, steep)` rounds apart".
- `crates/geom-brep/src/intersect.rs:2668-2669`: "its honest door
  `levered_inv(size, d)` rounds apart".

**What contradicts them.**
- The stand-ins row these sites moved to
  (`unsourced-levers-the-stand-ins-and-the-carrier-terms.md:25`) and the
  PR body both say this spelling is "a bare division under a door's
  name", which is "not `levered_inv`'s contract".
- So the reverted site carries a marker that names, as its honest
  re-posing, the spelling the review and its owner row rejected.
- The text is main's, restored verbatim, so the PR adds no new claim. But
  the gate requires the marker to say "why it fits no source", and this
  one now points the next unit at the wrong fix.

**The other reverted markers.**
- **Census wall sides.** The marker says "the chord's own extent read
  rounds differently". That is the bit-only reason; the reason it now
  waits is the record (`Wedge::arm` is `SectorShape::arm`). It is
  incomplete, not false. `likely`.
- **The circle arm** ("longer than the arc's extent") and **the certify
  twin** are true.

### MINOR 3. The PR body credits the sweep sphere-offer red to a row that does not name it.
BY-INSPECTION · `sure` on the citation.

- The body's "Red on the base" list files sweep
  `offer_rows::every_sphere_offer_passes_just_below_it` under
  `scalar/main-red-offer-rows-false-offers-since-the-told-quantity-type`.
- That row names topo's `every_offered_tolerance_passes_just_below_it` and
  the two `a_corner_reads_only_its_own_sectors` rows, not sweep's.
- On main, only `orbit/closed-sphere-escape-is-re-charted-…:52` names the
  sweep row, and only as a row that a proposed change would move.
- The panic I recorded at `a686ebb5` and on its base, "false offers: …
  quotes None and offers Some(4.999e-11)", has the scalar row's shape. So
  the owner is likely right, but the row doesn't list it.

### MINOR 4. The 1e-12 lily red is owned by a row that names the scene's stop, not this test.
BY-INSPECTION · `likely`.

- **The test.** `demo-tour lily::review_probes::the_weld_refuses_at_the_cone_torus_pair`
  stops at `demos/tour/src/lily.rs:287` (`.expect("stem tube arc
  builds")`), with `AnalyticRung3(KernelLimit { operand: Torus, … })`.
  I measured it red on both sides at 1e-12, with identical panics, in the
  first review; the PR re-measured it.
- **The cited row.** `shell/main-red-five-slow-rows-refuse-at-limb-2s-kernel-limit-since-5728`
  (open) carries that stop under "A seventh: the lily's stem tube in the
  K-probe sweep". It does not name this probe row.
- **The scene.** It declares the frontier (`stem_frontier`,
  `lily.rs:1725-1771`), but this probe does not consult it.

The red is main's and is held by a row, though not by name. CI's demos job
was skipped on this PR, so no hosted run sees it.

### NOTE 5. The deleted doc's measurement record is still gone.
`likely`. The base's "Measured on the revolved TUBE, which refused that way
before this" is not restored with the circle arm. The new text keeps the
argument and drops the evidence.

### NOTE 6. The circle arm now has no row of its own.
`sure`. Deleting `a_half_turn_rims_corner_arms_are_its_diameter` is right:
the line's arm governs that corner. The arc-length read is now pinned only
by the sweep rows that catch a chord read, and nothing distinguishes the
arc length from the diameter. The unions row carries my probe as its row,
so this is scheduled.

## Inherited reds

- **The two torn-body seeds.** They are on the corrupt-input job of earlier
  heads (`f930f0d6`, `59e6e11f`). That job is **green** on run 44879067356,
  since the fuzzer draws a fresh seed per run.
  - The seeds are appended as evidence to rows that exist on main and are
    open: `charttail/a-split-reduce-on-a-torn-body-panics-at-pcurves-unreachable-own-loop.md`
    (`0x213eeb488bea0012`, `pcurves/site.rs:395`) and
    `topo/a-torn-body-fuzz-seed-panics-in-wedge-reach-naming-no-row-four-premise.md`
    (`0x2dc7388623d34e51`, `wedge_reach.rs:182`).
  - Both panics match their rows' own signatures.
  - The PR diff touches no file under `pcurves/`, `splitting/`,
    `wedge_reach` or `review_d18.rs` (checked with `git diff --stat`
    against the merge base).
  - Unlike the rows' earlier entries, neither new entry says the seed
    reproduces on main. My reproduction is below.
- **The lily at 1e-12**: MINOR 4.
- **The five slow reds.** I measured them on both sides at `a686ebb5` and
  its base. The PR now lists all five, with the attribution gap in
  MINOR 3.

**Seed reproduction on main: done, DEMONSTRATED, `sure`.**

The run: main `3ef71034a2` (my worktree, probe removed), built with
`CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=false cargo test --release -p topo --lib`,
running `review_d18::torn_bodies_fail_reads_only_on_a_row_four_premise`.

- **`CAD_FUZZ_SEED=0x213eeb488bea0012`** panics at
  `crates/topo/src/pcurves/site.rs:395:36`: "HalfEdgeKey(3v1) is a member
  of its own loop", reached from `split_reduce`.
- **`CAD_FUZZ_SEED=0x2dc7388623d34e51`** panics at
  `crates/topo/src/wedge_reach.rs:182:32`: "HalfEdgeKey(5v1), a half of
  live EdgeKey(3v1), has no face", reached from `classify_neighborhood`.

Both are exactly the panics recorded on the PR's runs, so both seeds are
main's. The rows' new entries could say "reproduces on main", as their
earlier entries do.

## CI run 44879067356, step level

**Ran, all green**

- *change filter*: classify, base and direct-path rows. Its step "the
  merge this dispatch gates" was skipped.
- *lint*: work tracker, gates, rustfmt, clippy and rustdoc. The
  change-filter self-test step was skipped.
- *test (kernel)* and *test (app)*:
  - build;
  - default eps;
  - "eps 1e-6, 1e-12" for the crates and files this diff touches;
  - "the slow-set tests this diff adds or edits";
  - doc-tests.
- *mesh budget* and *corrupt input (release)*.

**Skipped**

- Jobs: the cache prime, viewer, step import, python, interval, tools,
  **demos (tour + wild)**.
- Steps: "the slow set did not run".

So the tour and the interval lane did not run on CI for this head either.

## Style

**Questions exercised:** Q1, Q2, Q4, Q5 and Q6, plus the fresh-instance
check the brief asks for. Q3 only as it bears on the deleted row (NOTE 6).
Q7 found nothing new. Q8 was not repeated: the delta touches no file I had
not read in the first review.

- **Fresh instances**
  - MINOR 2: a reverted site whose marker still claims a rejected door.
  - MINOR 1: the from-below read on an act-on-every-sign verdict survives
    for the non-circle, non-line kinds, justified by a line-only property.
  - No count was moved by hand away from its site.
- **Q2 and Q4, prose that must stay in step**
  - `edge_extent_inf`'s narrowed sentence is now true of its named
    consumer.
  - Its other readers outside that list include `offset_axial` (the
    lines and the fallback), whose Zero serves a foot, and no sentence
    says so. The PR body says "`offset_axial` is not one", and the code
    still reads the door there.
  - `likely`.
- **Q5.** The unit row's title still reads "arms whose honest read moves
  only last bits are re-read". With the circle arm out, that is true of
  what remains. `sure`.
- **Q6, deviations.** Each moved site is scheduled: the unions row, the
  stand-ins row, and the records unit in `plan.md`. `sure`.

### Ratings (1–5)

| | rating | evidence |
|---|---|---|
| Idiom and structure | 4 | Clean reversions. The match keeps the one kind it must; the rest go through the door. |
| Test quality | 3 | It was right to delete a row that couldn't fail. The circle arm and the fallback carry no row of their own (NOTE 6, MINOR 1). |
| Doc and comment honesty | 3 | F1's false premise is gone and the consumer sentence is now true. "so does every carrier but a circle" and the restored "honest door" markers are not. |
