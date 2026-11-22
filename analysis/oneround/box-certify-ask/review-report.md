# Review: PR 7272, `oneround/box-certify-ask` (full review: claims and style lane)

Reviewed head: **`34c966f6`** (the orchestrator moved the freeze from
`cc67fa2e` mid-review; the unit's own source is byte-identical between the
two in `geom-brep/src/session_box.rs`, and `in_the_box` is unchanged).
Base as merged in: `oneround/box-region` at `e8111192`. Hosted CI: run
44897731642.

## Verdict: NOT-MERGEABLE-AS-IS

The mechanism is sound where I attacked it: one decision, asked first, and
the edge's read held both ways on every kind I threw at it. What blocks the
merge is narrow: **one of the unit's own re-baselined goldens is red at the
frozen head, deterministically, at all three ε, and hosted CI is green only
because it never ran that row** (M1). The other findings are test gaps, one
silently lost fixture, and a recourse that is false of the limit.

## What I executed and what I only read

Executed (4 cores, my own target dirs):

- **The fast set, whole workspace less `viewer` and `pncad-py`, ε = 1e-9, at
  `34c966f6`**: 16 336 run, 16 336 passed. The binary carried my
  instrumentation patch (`probes/instrumentation.patch`: env-gated, no
  arithmetic; logs each box refusal and each uncovered edge's outcome).
  Judged against: nextest's `ci` default filter, which skips the slow set.
- **A 102-test slice of the slow set** (every re-baselined ledger, count and
  digest row, and the re-pinned slow rows; filter in M1) at 1e-9, 1e-6 and
  1e-12: 101 passed, 1 failed, the same one, at each ε.
- **Probes** (`probes/review_probes.rs`, 8 rows, geom-brep lib): circle,
  ellipse and spline edges against an independent reach; unvalidated spans
  and data at check 0; uncovered edges through `certify`; cost.
- **Seven mutants** (`mutants/*.patch`), each against geom-brep's lib fast
  set (1 468 tests); the three survivors also against 136 topo lib rows
  (`session_box`, `cover::`, `policy_lane`, `validate::tests`).
- The hosted slow-set selection, reproduced locally with the repository's
  own scripts and confirmed against the hosted job log.

Only read: the door survey (C2), the fixture diffs (C6; a delegated
read-only pass over every changed test, of which I re-read five claims
myself and found all five accurate), the base's fold, the README.

Not done: the whole slow set (rows there run past 300 s each in this
container; hours on 4 cores), a build of the base, the demos, a differential
at 1e-6 and 1e-12 beyond the slice. The author's 21.8 M-ask and
19 730-test figures are unverified by me.

## MAJOR

### M1. A re-baselined golden is red at the frozen head; hosted CI never ran it. DEMONSTRATED, `sure`

`editor-core::all m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned`
fails at `34c966f6` at ε = 1e-9, 1e-6 and 1e-12 (three runs at 1e-9, same
result). At 1e-9 every count and form total matches the pin; one digest does
not:

```
Plain/Decision calls 979 forms 39051 frozen 54 digest 432eed5d…  (measured)
Plain/Decision calls 979 forms 39051 frozen 54 digest b7af5595…  (pinned)
```

This ledger was re-baselined by the unit (`77c3ed8d`). I did not bisect the
cause; the base's fix pass and `origin/main` were merged after the
re-baseline, and the implementer's own statement is that the figures were
not re-measured on the merged head, so the merge is the likely cause
(`likely`). Whatever the cause, the row is the unit's to hold green.

Why CI is green: the slow-set step selects tests by names spelled on the
diff's added lines. In run 44897731642 that step ran **46 tests, all in the
`geom-brep` lib binary** (`offset_fit`, `props::quad`, `ssi`, …; none is
this unit's), and 0 tests in the app shard. Of the rows this unit
re-baselined or re-pinned that live in the slow set, **none ran on hosted
CI**: `asm2a_instantiate`, `m10_6_ci_rows_interval` (the re-blessed
`m10_6_certifying_keys.txt`), `m10_8_pins_interval`, `m10_9_pins_interval`
×4, `m10_bulge_interval` ×3, `m10_sym_profile_interval` ×2,
`sym11_exact_channel_rows`, `sym_9_retry_interval` ×2,
`a_sweep_keeps_the_frame_it_is_handed` ×2,
`mass_props_are_thread_count_invariant` ×3 (the three per-ε digests),
`sym11_far_placement_rows` ×2, `chart_region::…the_interior_witness_enters_as_computed`,
`boxes::sphere_rect_rows`, `sphere_island_rows` ×2, `pierce_fuzz`,
`closed_form_limb2` ×2, `reach_split_gate_*`, `review_4781_r1_probes`,
`join_pierce_strut_facing::two_out_runs…`. They are vouched for by the
author's local runs on the old base and, now, by my slice: all green at
three ε except the one above.

The PR body's "Hosted CI on this PR is the verification of record … every
job green" is true of the jobs and false as a statement that the
re-baselines were verified (C10).

## MINOR

### m1. A third of `the_enclosure_holds_the_vertex_polyhedrons_exact_volume` silently stopped running. DEMONSTRATED, `sure`

`crates/topo/src/props.rs:9191`. Its third centre, `(1234.5, −987.25, 3333)`
(`:9207`), is past the box. The needle is built under `catch_unwind` with
`continue` on a panic, and the row's only floor is `read > 0`. My log shows
8 `POINT` refusals in this test: all 8 far needles (16 of 48 readings) are
now skipped. The doc still says "and at `(1234.5, −987.25, 3333)`, either
way out". It is not in the PR's fixture table, not re-pinned and not filed.
This is the C6 answer to "are the three filed rows all of it": no.

### m2. Other subjects lost or thinned without a row. BY-INSPECTION, `likely`

From the fixture read, each re-checked by me where marked:

- `topo boxes::…a_zones_support_near_a_box_row…` (`boxes.rs:4561`, re-read):
  the new skip drops a draw past the box, and at scale `1e3` (`r ∈ [500,
  2000]`, centre ±2000 per axis) almost every draw is past it, so the
  kilometre leg is effectively gone while the doc still lists scale `1e3`
  "inside the session box". Nothing asserts a `1e3` face was read.
- `mesh issue303::huge_offset_volume_through_public_oracle`: now asserts the
  refusal only; the recentred fold at a huge offset is reached through no
  door. Its doc points at `translation_invariance_1e3_1e6`, which now
  compares at 900 m (and keeps the old name).
- `geom-brep analytic_image_rows::a_correct_image_far_out…`: the lever half
  sits behind an `if let Err(Escalated…)`; with every asked rung inside
  920 m it may be a dead branch (`unsure`).
- `step-import …a_one_solid_file_past_the_session_box_refuses_outside_it_alone`:
  import's own tier gate asking the box is no longer reached through import.

The three filed rows (PCERT, ATREST, SYM) and the GERM evidence do cover
what they name: limb 2's `KernelLimit` at 12 km, the far-arc witness at
1e-6/1e-9, the Sym far-placement table (every off-origin cell is now
`BOXED, 0`), the frustum's `EndpointEnd` at 1e-12.

The mesh claim checks out: `issue303` ×2, `issue1362` ×2 and
`r1_e2e_l_prism_near_far` really asserted nothing past the box before. But
the in-box replacements in `issue1362` ×2, `issue303` and
`r1_probes_issue303` still `return`/`continue` on any build error with no
floor, so they can go vacuous again; and
`r1_probes_issue1362::r1_looks_for_an_end_to_end_fixture…` (`:437`) was left
at `d = 1e3…1e5`, all past the box (6 refusals in my log).

### m3. `SessionBoxUnshown`'s recourse is false of it, and differs from the gate's for the same limit. BY-INSPECTION, `sure`

`certify.rs:1262` ends the edge's limit with `NOT_YET_ENDING` ("There is no
way through yet"). There is one: move the geometry in. The gate's
`ValidationError::SessionBoxUnshown` is family `SessionBox`
(`topo/src/finish.rs:883`), whose recourse is `SESSION_BOX_RECOURSE`. So the
same limit ends two ways depending on the door. At rest it reaches the user
through `classify_certify` with the not-yet ending. `OutsideSessionBox`'s
recourse is true. Both variants are in every exhaustive match; in
`classify_certify`'s fallback they sit in the arm that ends in
`unreachable!`, correctly, since `own_ending` is total for them.

### m4. Three mutants survive. DEMONSTRATED, `sure` (against the rows named above)

| mutant | geom-brep lib fast set | topo slice |
|---|---|---|
| ask dropped at `certify_carrying` | red: `every_certification_door_asks…`, `a_correct_image_far_out…` | not run |
| inside read drops the carrier's points | red: `an_arcs_bulge_past_the_box…` | not run |
| cover not hulled with the ends | red: `an_end_past_the_box_refuses`, `every_certification_door_asks…` | not run |
| a point reported as the limit at certification | red: 2 rows | not run |
| **inside read drops the ends** | **survives** | **survives** |
| **a margin inside the band refuses** (`Sign::Zero` too) | **survives** | **survives** |
| **a limit is admitted at certification** (`Unshown → Ok`) | **survives** | **survives** |

- *Ends in the inside read*: on a line or circle the cover's box already
  holds the ends, so the two `.max(...)` terms at `session_box.rs:135-136`
  are redundant there, and no row has a spline or ellipse edge with an end
  alone past the box. The author's "ends left out" mutant removed them from
  the cover as well, which is the half that is pinned.
- *The band of the box's edge*: "geometry within the band of the box's edge
  is in it" is pinned by no row I ran; no fixture stands within ε of 1 km.
- *The limit at certification*: no row drives `CertifyError::SessionBoxUnshown`
  through a door. `a_cover_alone_past_the_box_is_a_limit` pins the core only.
  The variant exists in the suite as a sample and nowhere else; in 16 336
  fast tests and the slice the ask never returned `Unshown` once.

Not planted: "the ask on the unmoved carrier where a door places one". No
certification door places a carrier; `run_checks` asks the spec it certifies.

### m5. The edge's region is now constructed in two places. BY-INSPECTION, `sure`

See Style S1. It is a MINOR on C1 because the claim "one statement" is true
of the decision and false of the region the decision is asked of.

## NOTE

- **N1 (C4), uncovered edges.** In the fast set at 1e-9, 30 edges had no
  cover; all 30 were refused by the checks (22 `IntervalNotForward`, 4
  `Unimplemented`, 3 `Escalated{ParamSpan}`, 1 `Escalated{EndpointStart}`);
  none was certified. DEMONSTRATED for the suite. Two things by probe:
  (a) a decision *is* taken on a region the box has not shown inside: with a
  NaN end, `carrier_endpoint_start` is decided and passes before
  `EndpointEnd` escalates; (b) `BoxRefusal::Uncovered`'s doc ("no point known
  to lie on it is past the box") is false: a NaN end beside an end 5 km out
  reads `Uncovered`, not `Point`, because the poison rides the `max`. The
  invariant "no cover ⇒ the checks refuse" is asserted in a comment
  (`certify.rs:5231`) and enforced by nothing; I could not build a
  counter-example (a spline with poison outside its span's support is the
  shape I would try; `unsure`). On the consistency question: at `e8111192`
  the base still hands an uncovered face back to refuse *after* the checks,
  but always refuses it. The unit never refuses an uncovered edge by the
  box. Against the README's "before anything is decided on that region",
  the base's ruled direction (refuse first, carrying the cover's reason) is
  the one the text supports; the unit's choice is defensible only while the
  comment's invariant holds.
- **N2 (C3), what each kind reads.** Line: its segment, exact. Circle: its
  arc, exact (2 000 tilted arcs 3e-12 to 1e-9 m short of the box's edge at
  ε = 1e-12: none refused; 12 000 arc asks at three ε against a dense scan:
  0 wrong). Ellipse: the cover is the arc's own, so "in" is exact (0 of
  4 000 wrong), but the inside read is 17 samples: 2 of 561 arcs past the
  box were refused as the *limit*, the worse one 2.4 m past, with the
  message "the edge may lie inside the box". Sound, mis-labelled. Spline:
  the pieces' nets; a rational arc about the origin was never refused short
  of 1 km at any half-angle I tried, and a short span of a spline whose net
  reaches 1e5 m reads its own span. Spiric: not attacked beyond cost. A
  far carrier over a short span (line, circle of radius 1e6): in. No edge
  with a point past the box was admitted in any probe, and none inside it
  was refused as past. One early red in my circle probe was my own scan's
  error (a 200 000-sample scan misses a crest by up to 1e-7 m); the row now
  reads the analytic crest.
- **N3 (C3), check 0 runs before the span is read.** `in_the_box` sees the
  spec before `ReadSpec::of`. NaN, infinite and 1e300 spans on line, circle
  and spline: no panic; a line over `(0, 1e300)` reads `Point`, so a span
  that used to refuse as a bad interval now refuses as past the box.
- **N4 (C2), doors.** Every certifying constructor goes through
  `run_checks`; at this head there are **five** public doors, not four:
  `certify_carrying` arrived from main and is the one `certify_stated` now
  calls. The row `every_door` and the PR body still say four; the new door
  is covered by construction, and by my first mutant. Not asked, and what
  asks instead: `carried` (via `map_isometry`): `FinishedBody::moved` and
  the placing door ask the fold; but `topo::transform_rigid` and
  `mirror_body` are public construction-state maps that ask nothing, and
  `mirror_body` mints pcurves on the image before any ask. Those are the
  base's doors, reported here because the PR body says the fold asks of
  every carried image. Surface-key remap and chart mirror: same region,
  asked at its mint. The dual's mint: never asked at that scalar.
- **N5 (C7).** I took no differential against a base build. By
  construction a verdict moves only where the ask refuses; my log has 130
  refusals in 33 fast tests and 35 in the slice, all `Point`, every one on
  an edge that is past the box (smallest: a 1 µm ball at 1 km, 1e-6 m
  past). No `Unshown`. So no body inside the box changed verdict in what I
  ran. Judged against the fast set and the slice at 1e-9, not the whole
  suite.
- **N6 (C7), the gate can admit what certification refuses as a limit.**
  The fold's second stage reads a face's own cover; certification reads the
  edge's cover alone. A moved body whose spline edge's net pokes past the
  box while its face's cover does not is admitted by the move and refused
  at its next re-certification. A sliver near the box's edge; BY-INSPECTION,
  `unsure`.
- **N7 (C8).** Per ask, this build: line 1.7 µs, circle 9.6, ellipse 9.3,
  spiric 5.3, a 3-point rational spline 29, a 20-point cubic 186, a refused
  far line 2.9. Consistent with the author's 6.1 µs mean; I did not
  reproduce the 1 ms donut figure (see the addendum for the on/off timing).
- **N8, premise in the brief.** The brief's reds "not the unit's"
  (`germ_tilted_rod`, the censuses, `nist_ftc_09`, the rosters, …) did not
  appear: the fast set is fully green at `34c966f6`. The brief names the
  row file `crates/topo/src/cover.rs` and `topo/src/session_box.rs` "as
  they stand at the frozen head"; the freeze then moved, and the engine's
  signature changed under the claims (C2's "four doors").

## Claims

| | |
|---|---|
| C1 | Holds for the decision (`session_box.rs:57` is the only statement; grep by data and prose). The home is right and the dependency runs the right way. "The only home" is overstated (geom reaches `Cover` and `Band` too) and the region's construction is doubled (m5). |
| C2 | Holds. Five doors, one engine (N4). |
| C3 | Holds on every attack I ran (N2, N3). |
| C4 | Holds on the suite; rests on an unenforced comment (N1). |
| C5 | Placement holds; `SessionBoxUnshown`'s recourse does not (m3). |
| C6 | Production: nothing I saw works past the box. Re-pins: not all of the lost subjects are scheduled (m1, m2). |
| C7 | Not falsified in what I ran (N5); one re-baseline is stale (M1). |
| C8 | Order of magnitude confirmed (N7). |
| C9 | The author's mutants' rows exist and my equivalents are red; three of mine survive (m4). |
| C10 | False as to what hosted CI vouches for (M1). |

## Style

Questions exercised: Q1 (prose sweep and constants grep over both
`session_box.rs` files, `cover.rs`, `extent.rs`), Q2, Q3 (mutants), Q4, Q5,
Q6, Q7. Q8: read `geom-brep/src/session_box.rs` end to end; `certify.rs`
(the largest file touched) only around the engine and the error type.

- **S1. The fix for "one decision" minted a second copy of the edge's
  region.** `geom-brep/src/session_box.rs:120-124` builds "the carrier's
  cover over the span hulled with the ends"; `topo/src/cover.rs:1291-1299`
  (`edge_cover_over`) builds the same, and the fold reads topo's. The copy
  declares itself in prose (`session_box.rs:106`, "topo's
  `cover::edge_cover_over`"; the PR body, "the same construction as"). The
  inside read is doubled the same way (`edge_in_box`'s closure against
  `reach_of`). The two doors now agree by two hands. `sure`.
- **S2.** `(lo − origin).norm_inf().max((hi − origin).norm_inf())` is
  written at `session_box.rs:101` (`cover_reach`) and `extent.rs:690`. `sure`.
- **S3. Two Displays for one limit**, drifted: certify's "a tighter bound of
  it is not built", validate's "is not built yet"; and two endings (m3). `sure`.
- **S4. A premise this unit invalidated.** `topo/src/finish.rs:738-745`
  states, as "the one place the partition is stated", that every family but
  `SessionBox` decides a fact up to isometry. `EdgeCertification` is family
  `Certificate` (`finish.rs:947`) and can now carry a box refusal, which is
  not. Code drifted from an invariant that was meant to hold, not a stale
  sentence. `likely`.
- **S5.** `CertifyError::decided`'s doc (`certify.rs:1175`) says `None` is
  for "a refusal that is no decision's"; `OutsideSessionBox` is the refused
  arm of `session_box_point` and carries its margin. The PR body argues the
  point by analogy with the gate. `likely`.
- **S6.** `in_the_box` wraps its one call in a closure, `let ask = || …;
  match ask()` (`certify.rs:5222`). `sure`.
- **S7.** `edge_in_box`'s doc and `BoxRefusal::Uncovered`'s doc promise more
  than the code does on poisoned ends (N1 b). `sure`.
- **S8. Stale test prose after the re-pins**: `sym11_far_placement_rows`
  (`…is_a_counted_dispute…` names and docs; no cell disputes now),
  `issue1362` (`…at_a_kilometre…`, `ulp(1e3)`), `boxes.rs:4526`,
  `pierce_rows.rs` ("a kilometre"), `newell_probes.rs:169`,
  `step-import …refuses_outside_it_alone`, `props.rs:9189`. A class; I would
  sweep every test the fixture table names. `sure`.
- **S9.** `sweep/tests/common/mod.rs` `refused_by_the_session_box`: the
  margin check went from an equality within `1e-9·reach` to an interval
  (`want − 1e-9·reach ≤ margin < 0`); any edge of the body satisfies it. `sure`.
- **S10.** Not pure count bumps among the re-baselines, each explained only
  by a phrase: `m10_sym_profile` form totals *fall* (8308 → 7991) while
  assertion forms nearly double; `sym11_exact_channel` FROZEN 2738 → 2746;
  the thread-count digests' `chk` moves on one body at 1e-12 only. `unsure`
  whether any is wrong; M1 is in this family.
- **S11.** The PR body runs to a justification several times the length of
  the 25-line change it defends, and its measured figures are for a base
  that is no longer the base. `sure`.

## Ratings

- **Idiom and structure: 4.** One generic core, lazily read inside, a typed
  refusal; marred by the doubled region construction.
- **Test quality: 3.** The new rows kill the mutants that matter most, but
  three survive, the limit is pinned through no door, and several re-pins
  traded a subject for a refusal.
- **Doc and comment honesty: 3.** The limit's recourse, `Uncovered`'s doc,
  "four doors", and a verification paragraph that claims more than the run
  ran.

## Files on this branch

`probes/review_probes.rs` and `.patch` (apply to the head; module
`session_box::review_probes`), `probes/instrumentation.patch`,
`mutants/*.patch` with `*.result.txt` (geom-brep) and `*.topo-result.txt`.
