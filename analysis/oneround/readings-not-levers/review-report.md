# Review: PR 7442 `oneround/readings-not-levers`, frozen at `12d3bdf41a`

FULL review (claims to falsify, plus the style lane), against the brief at
`9aeb990c80`. The unit's own change is taken as the frozen head against the
merge of its two bases (`git merge-tree --write-tree 75a008e8f7 7691e4f872`
→ tree `9b92d10554`, clean). On my branch a commit of that tree,
`a0247a54b8`, was checked out as a separate worktree and built (its own
target dir) for every base comparison below.

## Verdict: APPROVE-WITH-FIXES

No MAJOR. All seven conversions are the row's final state (design row 303),
the two new geom-core doors do what they say on everything I scanned, the pins
match the tree, and the roster reds are not this unit's. What needs fixing is
mostly about accuracy: one false statement in the author's report (a red with
"no row" has an open row in the frozen head's own tree); the soundness
argument for the `tip_wraps` meter, which holds for one corner and is stated
as if it held for the sum; a refusal under the caller's name that offers a
tolerance that never decides (demonstrated); and an unpinned guard whose
removal would make `tip_wraps` answer `Ok(true)` silently.

## What I executed and what I only read

Executed, all with `CAD_FUZZ_SEED=12345` and CI's opt-level-1 test profile
unless noted:

- `ci` profile, geom-core + geom-brep + topo, frozen head: ε default **7719/7719
  green**; ε 1e-6 **7719/7719 green**; ε 1e-12 **7717/7719**, the two reds
  being `pcurve_cache::envelope_lemma_fuzz::projected::{a_line_near_a_ruling,a_carrier_on_its_chart}_pinned_where_the_frame_cost_refuses`.
  On the merged bases at 1e-12: **7710/7712**, the same two reds. 7719 − 7712 =
  7, the seven tests this unit adds. Judged against the merged-bases run in the
  same container.
- Those two fuzz rows at 1e-12 under other seeds (head): unset → 2/2 green;
  seed 1 → 1 red; seed 999 → 2/2 green. Seed-dependent.
- Both `recourse_roster` rows on the head: red, with exactly the misses the
  brief lists (geom-brep: `intersect.rs` `EXTERNAL`/`INTERNAL`/`PC_PARALLEL_GAP`,
  `ruling.rs` `TANGENCY`, `cones.rs` ×3, `closed_image_rows.rs` ×1,
  `spline_incidence.rs` ×1, `certify.rs` ×2, `enclose.rs` ×3; topo:
  `Const(NEST)`, `Const(REACH)`, `chain_parity.rs`, `spline_trim.rs`). The same
  head-built binaries run over the merged bases' `src/` (the rows read
  sources at run time from `CARGO_MANIFEST_DIR`) list every one of those
  misses as well, and add only the head tables' own new entries, which do not
  match base sources (`pairing_at_wedge: sin_theta`, `travel: name`). **C9's
  roster claim holds**: every listed miss is there without this unit, and no
  site of this unit is among the head's misses. One caveat: these are the
  head's roster tables against base sources, not the base's own tables, which
  differ only by this diff's three entries.
- The new rows (14 including both rosters) on the head, targeted run.
- Probes, on my branch at `analysis/oneround/readings-not-levers/probes/review-probes.patch`
  (applied to a separate worktree of the frozen head, plus `mutate.py`):
  - `offered_band` scan: 118,702 refused point margins across five bands
    (K = 10, 7, 3.67) including the 4-digit grid points where the told offer
    equals the raw one. In the band `offered_band` returns, every one decides
    **Positive** (at or past the escalation edge). No counterexample.
  - `a_step_finer` against the old `K·ρ` read: ±64 ulps around the edge on
    five bands give **no disagreement**. At `ρ = ε/K` both read Zero; at
    `ρ = ε` both read Positive (Err).
  - `travel`'s half-turn tie at aspect 10 (the edited row's fixture): see m2.
  - The arm's wedge, Zero at the offer, re-read below the offer: see n1.
- Mutants, each planted alone and run against the crate's **lib** tests
  filtered to the touched modules (topo: `boolean::fate boolean::join
  chord_join validate vertex_section`; geom-brep: `dihedral intersect certify
  pn_apex enters`; geom-core: `predicate period k_stats`), so a "survives"
  means survives those rows only, not the integration suites:

  | mutant | result |
  |---|---|
  | `travel`'s names swapped (`["bool_join_arc_clear", name]`) | killed (3 `travel_rows`; on the full topo lib run, plus 3 known main reds) |
  | `tip_wraps` without the `halves == 0` guard | **survives** (288 rows) — m3 |
  | `tip_wraps` meter folds `reach.above()` | survives: equivalent (`leaving` builds `LeverBounds::Exact` for a line, `fate.rs:502`) |
  | `resolved_as` in the run's band (unsound, wider) | killed (`an_unresolved_carriers_pin_is_a_kernel_limitation`) |
  | `at_wedge` without `.filter(re_quotes)` | survives: equivalent (`quoting_reading` re-checks `re_quotes`, `enters.rs:154`) — s3 |
  | pairing's offer replaced by the run's band | killed (`a_pairings_wedge_…`, alone) |
  | `apex_start` without `e_δ` | survives, as the author reports |
  | `offered_band` K times wider (`from_thresholds(ε′·K, K)`, unsound) | killed: geom-core 1, geom-brep 6 |
  | `a_step_finer` wider (`ε/√K`, unsound) | killed: geom-core 1, topo 1 |

- CI run 45090709788, resolved to step level through the jobs API (below).

Only read (not executed): the author's differential (14193 + 1612 tests, no
probe of it was available to me), the slow set at 1e-9, the sweep, editor-core,
mesh and step-import suites, the author's other mutants (I re-planted the
unsound ones above, not the whole list), and whether any `ci` test reaches
`tip_wraps` other than the new rows.

### CI 45090709788 at step level (frozen head)

- `test (kernel)`: build, default-eps, "eps 1e-6, 1e-12 — the eps-sensitive
  crates and test files this diff touches", doc-tests and slow-set-names
  steps green. **"the slow set did not run" SKIPPED.** **"test (the slow-set
  tests this diff adds or edits)" FAILURE**: exactly the two `recourse_roster`
  rows. The step summary truncates the geom-brep list after `cones.rs:
  against_wall`, so the brief's longer list cannot be read off CI. I read it
  locally (above), and it matches.
- `lint (fmt, clippy, gates)` green, including gates, clippy and rustdoc.
- `test (app)` green; `corrupt input (release profile)` green.
- **`mesh budget meter + certificate falsifier` was still `in_progress`** at the
  time I read it (step "mesh --features budget" running). "Every step green but
  one" is premature for this job.
- Skipped jobs: `prime the build cache`, `demos`, `tools (k-lint, tess-lint,
  tess-meter)`, `step import (freecad)`, `interval`, `python suite`, `viewer app
  feature`. The k-lint tool job is among them, which matters for n4.

## Findings

### MINOR

**m1. "No row" is false for the two sweep slow-set reds.** DEMONSTRATED (by
the tree), `sure`. The author reports finding no row for `sweep
bisector_nodes::the_near_node_poses_at_the_witness_band` and `sweep
section_end_poses::a_section_end_at_the_band_edge_refuses_as_the_inputs_at_the_boolean_and_its_lever_builds`.
`work/green/main-red-eleven-editor-core-and-sweep-rows-moved-on-main-after-c390e1f9.md`
(status open, opened 2026-11-22) names both, at lines 44 and 46. It came in
with commit `1f64bc2dd`, an ancestor of `7691e4f872`, so it is in the frozen
head's own tree. (`bisector_nodes` also has two closed rows:
`main-red-a-near-node-cap-at-1e-8-refuses-on-quadrature.md` and
`main-red-eighteen-sweep-analytic-rows-…`.) The PR body's red ledger should
cite the row. I did not re-run these two rows on either tree.

**m2. The join's tie refusal, now under the caller's name, offers a tolerance
that never decides it.** DEMONSTRATED (probe `review_probe_the_half_turn_ties_offer_is_honoured`),
`sure`; pre-existing in kind. The half-turn partner at aspect 10 (the
fixture of the edited row
`a_partner_at_the_half_turn_reads_ahead_only_where_the_slack_is_in_band`)
escalates under `bool_join_arc_ahead` with margin `9.9e-9`, and the sentence
reads "if this size is intended, tighten the tolerance below 9.9e-10 m". At
that band it escalates again, offering `9.801e-10`, then `9.702e-10`, then
`9.604e-10`: four offers in a row, none decides. The cause is that the far row
is `|m| + 2ε·cot ψ` with the swing proportional to the band's own ε
(`join.rs:2287`), so it is self-similar in ε, and any `cot ψ ≥ 0.5` never
ties at any tolerance. The old `bool_join_arc_slack` row had the same
self-similar margin, so this is not a regression. What is new is that the
offer is now spelled under the reading's own name, as "this size" of a reading
whose value is 0: the whole 9.9e-9 is slack. The new row
`the-joins-sites-are-read-on-the-conic-so-the-travel-carries-no-swing` removes
the swing eventually, but it does not name the false offer, and the PR body
presents the name change ("not under a slack name") as a gain. Note also the
same shape the PR's own sweep was looking for (a band threshold multiplied
into a reading decided in that same band), at a site the sweep's hit list
does not mention.

**m3. `tip_wraps`' `halves == 0` guard is pinned by nothing, and without it
the function answers `Ok(true)`.** DEMONSTRATED (surviving mutant, lib rows of
the touched topo modules), `sure` that it survives, `unsure` that a reachable
input has zero half-turn corners. With the guard removed (`fate.rs:925–927`),
a loop whose corners re-read here as no half-turn gives `count == 0 ==
-halves`, so `Ok(true)`: a wrapped tip asserted with no tip. The caller
(`fate.rs:800–811`) decides the corner with `normal_at` and `corner_holds` at
its own lever. `tip_wraps` re-reads every corner with `face_outward_normal`
and `reading_lever`, so the two reads can disagree in principle. The old
`_ if !any => Err(unread)` arm sat in the same place. I did not check whether
it was pinned on the base.

**m4. The `tip_wraps` meter's soundness argument is per corner; the count is
a sum.** BY-INSPECTION, `sure` on the arithmetic, `likely` harmless at real
fixtures. The PR body: "every corner's admission (within ε at its own lever)
moves the sum by at most ε over this least reach". The admissions add: with
`h` half-turn corners each admitted at `|sin δ_c|·L_c ≤ ε`, the residual left
in `whole − turned` is up to `Σ ε/L_c ≤ h·ε/L_min`, not `ε/L_min`. The meter
decides each mark at `|gap − mark|·L_min` against `(ε, Kε)`. A mark could be
claimed cleared on the wrong side once `h·ε − (π/2)·L_min > Kε`, which needs
more than K admitted half-turn corners at a least reach of order ε. That is
unrealistic, because the quarter-turn room absorbs everything at any real
reach. But the statement is false as a bound on the sum, and the brief's
question "does a sum owe something else" has the answer "yes, `Σ ε/L_c`".

Two further points on the fold:
- The error source is the **half-turn corners'** admissions only. Corners
  whose turn is summed contribute exact `atan2`s, and the loop's total
  turning is topological. Their levers always exist, because `halves ≥ 1` is
  required. So the PR's reason for folding over every run ("a ring of two tips
  sums no turn, so no narrower fold always exists") argues against a fold over
  the summed corners, not against a fold over the half-turn corners.
- Folding over all runs is conservative (shorter lever, more refusals), so the
  choice is sound in direction. But the meter row
  `a_quarter_turn_within_the_band_at_the_least_reach_counts_nothing` puts the
  least reach at the 0.2 mm jog, a corner that is not a half-turn. It
  therefore pins the designer's fold choice at the fixture, and would kill the
  narrower fold that the error analysis supports (C8's "pin of the contract or
  of the fixture": the fixture).

**m5. Two seed-dependent 1e-12 reds counted as "not ours" have no row.**
DEMONSTRATED, `sure`. Both `envelope_lemma_fuzz::projected::…_pinned_where_the_frame_cost_refuses`
rows are red at 1e-12 under seed 12345 on the head and on the merged bases
(confirmed, so not this unit's). They are red under seed 1 for one of the two,
and green unseeded and under seed 999. `git grep envelope_lemma_fuzz
origin/main -- work/green` finds nothing. The nearest row,
`work/onehome/the-carrier-frame-fuzz-bodies-go-red-under-no-mutant.md`, is
about mutant coverage, not a red. A red known to exist and excluded from the
count owes a row (style Q6).

### NOTE

**n1. "Zero at the offer" is asked at the offered tolerance; the sentence
sends the user below it.** DEMONSTRATED (probe
`review_probe_a_wedge_zero_at_the_offer_at_half_the_offer`), `sure`;
pre-existing and by design. Over an in-band arm of 1.5e-9 (band 1e-9/1e-8,
told offer 1.5e-10), a wedge at 0.9 or 0.6 of the offer reads Zero at the
offer, so the arm binds and its refusal offers 1.5e-10. A run at half that
offer, as "tighten below" invites, refuses again on `dihedral_wedge` (margins
1.35e-10 and 9e-11). At 0.3 of the offer it reads Smooth. The doc at
`dihedral.rs:1085` ("reads smooth at every tolerance from there down to its
own") is literally true, because the range ends at the wedge's own value. The
PR's direction claim ("the old read left the first refusal quoting a tolerance
that does not decide the edge; the new one quotes the wedge") fixes only the
told-grid sliver between the told and the raw offer. The same two-round
recourse remains for every wedge in roughly `(offer/K… offer]` below any
chosen tolerance. `offered_band`'s rustdoc ("a reading decided in it is
decided as a run at the offered tolerance decides it") is accurate at the
offer. Neither caller asks about a tolerance below it.

**n2. C1's census of refusal kinds is incomplete, harmlessly.** BY-INSPECTION,
`likely`. Three refusals reach `pairing_at_wedge`, not two:
- `material_wedge_side` in band (margin `cos θ·arm`);
- `GRADIENT_ALIGNMENT_LEVER`'s (margin the arm itself);
- the Perpendicular `gate_rejection` (`dihedral.rs:2172`), a cosine **decided
  Zero** and refused under the pairing's own name, which still offers
  `|cos θ·arm|/K` (`Shows`: a nonzero size on a passing side).

For the last two the wedge is about the arm, so it reads Positive in the
offered band and both old and new code go on to decide it in the run's band.
The verdict does not move, but the PR body's "pairing margin is |cos θ|·arm …
except GRADIENT_ALIGNMENT_LEVER's" omits the third.

**n3. `resolved_as` now has a panic path the old code did not.** BY-INSPECTION,
`unsure` (reachability). `a_step_finer` fails on a band whose `ε/K` underflows
(the PR's own row: `Band::least()`). `resolved_as` `unreachable!`s on that
(`validate.rs:499–500`), and its signature takes any `Band`. The old
`levered(K, ρ)` read never failed. `Band::least()` is in production use
(`splitting/order.rs:111`, `chart.rs:156`). I found no path from those to
`chord_join`'s pin or `vertex_section`, but nothing in the type keeps one out.

**n4. The K-recorder change is disclosed and unscheduled.** BY-INSPECTION,
`likely`. "The nightly k-lint may want a re-baseline": no row. Four names now
record in derived bands or with a summed far-row magnitude, and the CI tools
job (k-lint) was skipped on this run. The nightly `k-lint` rows will re-measure
(`nightly.yml:311`), and the lint's own runbook (`tools/k-lint/src/main.rs:107`)
is the recourse, so the register exists. But a known shift in a gating lint's
input is a deferral without a schedule (Q6).

**n5. `apex_start`'s `e_δ` is unpinned in a function this unit authored.**
DEMONSTRATED (surviving mutant, as reported), `sure`. The spec puts `gap_err`
out of scope and the re-posing row now records it, so it is scheduled.
Recording only that the unit threads an unpinned charge through a new named
function, which makes the gap easier to miss, not harder.

**n6. C5's divisors are definite on every arm.** BY-INSPECTION, `likely`.
- Point arm: `|D|` is decided Negative past `Kε + |δ|` by `decide_across`'s
  floor.
- Tangent arm: `steep = sin α·s + cos α·|c|`, with both aperture terms decided
  positive by the aperture guards and `s² + c² = 1`.
- `pair_bound`: `d` is decided Positive, and `σ² = d·m/sin²α`.
- No sine in band is ever a divisor.

`Margin::of` of a length over a dimensionless sine does not change dimension,
so the door's text admits it under "a distance computed by upstream geometry
code whose contract states metres" (`apex_start`'s doc). Not a finding;
answering the brief.

**n7. C4 holds.** BY-INSPECTION, `sure`. `2ε·cot ψ` is bit-identical to the
old `levered(cot + cot, ε)` (same operand order). `decide_across` takes
`[far, floor]` and receives `[name, "bool_join_arc_clear"]` (far under the
caller's name), which matches its contract and the audit row. The swing is a
**tolerance-scaled position term** (two sites each within ε of the conic),
not a rounding swing. `decide_across`'s doc admits it ("a position the row is
read beside"). Definite signs need the same as before (`|m| − s` past Kε);
ties are stricter (`|m| + s ≤ ε`).

**n8. C6 holds; payload readers.** DEMONSTRATED (probe) for the decision,
BY-INSPECTION for the readers, `likely`. The Ok/Err partition is unchanged at
every ρ I scanned (see above). Readers of `SectionPinMiss::Undecided::resolution`:
`section_end_rows` (edited to `ρ`), the `Display` arm (which does not print
it) and `crates/sweep/examples/section_end_census_probe.rs:45`, which prints
`{resolution:?}`. Its output moves from `K·ρ` to `ρ`. No committed output of
that probe quotes the resolution (the two rows citing it quote the diag
margin).

**n9. The brief's premises.** `sure`.
- "About 27 files" is GitHub's count against `main`, which includes the
  stacked PR 7268. The unit's own diff against the merged bases is **17
  files**.
- The brief says the author reports "every step green but one". One job
  (mesh budget) had not finished when I read it.

## C-by-C summary

- **C1**: holds, with n1 and n2. `None` (no offer) returns the refusal
  unchanged in both callers, as before: `re_quotes` already required
  `offers_tolerance`. The wedge is the same quantity for both callers
  (`sin θ · arm`, the same `Lever`). Every outcome of the Zero test is a
  choice of which refusal to quote, never a geometric verdict, so the arm's
  bound direction decides recourse text only.
- **C2**: holds. Neither door takes a number. The offered band's zero is
  `Told::spelled`, the same float the sentence prints (row plus my scan). The
  refused margin decides past its edge on every scanned value. `a_step_finer`
  is `(ε/fl(Kε/ε), ε)`, its escalation equal to the coarser zero to the bit,
  and fails only on an `ε/K` underflow (n3).
- **C3**: holds in code; m3 and m4 on the guard and the argument. "All runs
  are lines" is **enforced**: `edge_is_line` returns `TipUnread` before the
  count (`fate.rs:901`), and `leaving` builds `LeverBounds::Exact` for lines,
  so below = above (equivalent mutant). The new rows are in the `ci` profile
  and run at every ε row, but on a band of their own (`Band::new(1e-9,
  1e-8)`) whatever the run's ε. I did not verify "no `ci` test reached
  `tip_wraps` before".
- **C4**: holds (n7); m2.
- **C5**: holds (n6, n5).
- **C6**: holds (n8).
- **C7**: not re-taken. Partial corroboration: the geom-core/geom-brep/topo
  `ci` profile is green at default ε and 1e-6, and at 1e-12 its reds equal the
  merged bases'. That is consistent with "no stored bit or golden moved" for
  those crates.
  - The 1e-6 point: CI's 1e-6 step and my run found nothing. No site's
    conversion depends on ε in a way that is absent at 1e-9, except `travel`'s
    swing, which scales with ε.
- **C8**: the unsound-direction mutants I re-planted are killed. Survivors:
  `halves == 0` (m3), and three equivalents (`above`, the `re_quotes` filter,
  `e_δ`, the last as reported). The arm's and the pairing's old-number mutants
  are each seen by one told-grid row. That row pins the told-versus-raw
  property of `offered_band` (a contract), at a sliver of the fixture (n1).
  The meter row pins a fold choice (m4).
- **C9**: holds. `lever-sources` −10 = dihedral 2 + intersect 5 + fate 1 +
  join 1 + validate 1. `from-f64-sites` −2 entries (dihedral's two
  `band.escalate()`, validate's ratio), and `join.rs`'s `band.zero()` keeps its
  entry. `lever-fixtures` geom-brep +2 = the two new `fixture_lever` calls.
  Gates green on CI. Rosters as above.
- **C10**: the filed row is true of the tree. The sweep states what it could
  not match. It omits its own `off_conic_slack` (m2), which has the shape and
  is scheduled by the filed row. The K-recorder disclosure is unscheduled (n4).
- **C11**: the PR body is accurate on what I checked, except m1, and "the new
  rows build their own bands": `travel_rows` reads `Band::linear(Tol::witness())`
  (`join.rs:6303`), so it varies with the run's ε. The 1e-12 `ci` reds are
  confirmed on the merged bases (geom-brep's two; the sweep row was not run).
  The 1e-12 slow-set sample (topo, head against merged bases): the same
  seven reds on both trees (addendum).

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8 only partly: I read
`tip_wraps` and its neighbours, the `at_wedge`/`pairing_at_wedge` region and
`plane_cone_section`'s apex arm end to end, not a whole file. The largest
touched file, `join.rs`, is about 8000 lines. Q8 was not done.

- **s1 (Q1)** `intersect.rs:2758` beside `:2660`: `pair_bound` keeps `let
  size = gap.abs() + gap_err` and uses it for `lean` and `offset`, while `far`
  and `near` call `apex_start`, which spells the same sum again. That makes
  two spellings of `|δ̂| + e_δ` in one function, in the unit that named the
  function to give it one home. This is the brief's "a fix mints a fresh
  instance". `sure`.
- **s2 (Q1)** `predicate.rs:3470–3492`: `offers_tolerance` and `offered_band`
  each run `self.margin.shows(self.band, self.passes())` and match `Offer`.
  The first could be `offered_band().is_some()`. As written they are two
  readers of one classification that can drift. `likely`.
- **s3 (Q7)** `dihedral.rs:1103–1106`: `.filter(|_| escalation.re_quotes())`
  is redundant. `quoting_reading` re-checks `re_quotes` (`enters.rs:154`),
  and the equivalent mutant survives. It costs a second `shows()`, and it
  reads as if it guarded something. `likely`.
- **s4 (Q3)** `fate.rs:2788`: the meter row pins the fold at a non-tip corner
  (m4). It is red under a narrower, better-argued fold, and green under any
  fold at or below the jog. `likely`.
- **s5 (Q3)** `fate.rs:925`: the `halves == 0` guard has no row (m3). `sure`.
- **s6 (Q2/Q5)** The PR body's per-corner sentence on the meter is stated as a
  bound on the sum (m4). Code comment `fate.rs:918–920` is true as written
  (least over corners = least over runs). `sure`.
- **s7 (Q6)** n4 (the K recorder) and m5 (two reds with no row): disclosed
  and unscheduled. `likely`, `sure`.
- **s8 (Q7)** `validate.rs:497–500`: an `unreachable!` asserts a caller's band
  is a run's, an invariant the `Band` type does not carry (n3). `unsure`.
- **s9 (Q4)** Swept prose citing the old premises: `K times over`,
  `wedge_reads_zero_at_the_arm`, `bool_join_arc_slack`, `levered at 1 m`,
  `fate::sign`. None is left in `crates/`, `docs/` or `tools/`, and the audit
  rows were updated. Clean, so no finding; pattern stated so a reader can see
  what it could not match (paraphrases). `sure`.
- **s10 (Q5)** `join.rs:2287` `off_conic_slack` now returns a bare `T`, which
  a reader can no longer tell from a reading. Its doc still says why it is a
  length. Taste only. `unsure`.

### Ratings

- **Idiom/structure: 4.** Each conversion is small and lands in an existing
  door (`Period::decide`, `decide_across`) or one of two narrow new ones.
  Docked for s1 and s2, two spellings each of a quantity the unit set out to
  name once.
- **Test quality: 3.** The `travel` 0.4-band case and the `offered_band` /
  `a_step_finer` rows pin contracts, and every unsound mutant I planted at the
  new doors went red. But the dihedral rows pin a told-grid sliver, the meter
  row pins a fold choice, and the `halves == 0` guard and `e_δ` have no row.
- **Doc/comment honesty: 3.** The PR body is careful and mostly true, and its
  sweep says what it could not see. But it states a "no row" that is false
  (m1), a per-corner bound as a sum's (m4), and "new rows build their own
  bands" where `travel_rows` does not. It also presents a name move as a gain
  without saying that the refusal's offer is never honoured (m2).

## Addendum: 1e-12 slow-set sample (topo)

EXECUTED: topo's slow set (`--ignore-default-filter -E 'not default()'`,
`ci` profile at opt-level 1, ε 1e-12, seed 12345) on the frozen head and on
the merged bases, 305 tests each. **Head: 298/305, 7 red.** That matches the
author's "7 in topo". **Merged bases: 298/305, the same 7 red**:
- `offer_rows::every_offered_tolerance_passes_just_below_it`
- `face_normal::tests::no_source_file_folds_the_bit_by_hand`
- `square_search::the_square_wall_arm_agrees_with_the_traced_section`
- `a_corner_reads_only_its_own_sectors::the_corners_offer_executes`
- both `a_seamless_pipe_cut_by_an_oblique_plane_measures` rows
  (`…encloses_its_closed_form_at_interval`,
  `…a_curved_junction_certifies_across_the_departure_arm`)
- `recourse_roster`

So for topo the author's argument ("no site verdict or name moved in any of
their processes") is borne out by a reproduction: none of the seven is this
unit's. The geom-brep and sweep share (20) was not sampled.

---

# Delta read: frozen head `cb36266ddd`, against `origin/main` at `c9cf004f9`

Reviewed: the fix pass (the orchestrator's F1 to F8) and the merges. The
unit's own change is now `cb36266dd` against `c9cf004f9`, which is an
ancestor of it: 23 files. I merged `cb36266dd` into my review branch to
build it. Same method and standing briefs as above.

## Verdict: APPROVE-WITH-FIXES

No MAJOR. The fixes I could test hold:
- F3's meter is sound as argued.
- F5's pin goes red.
- F6, F8 and F4 are as described.
- The `intersect.rs` merge resolution carries only the unit's reads.

Against that:
- F1's new door under-offers where its swing scales with the band, which is
  the very caller that prompted it. Demonstrated.
- The F3 fold's direction (least, not greatest) is pinned by no row.
- The frozen head no longer merges cleanly with current `origin/main`.

## What I executed

- `ci` profile, nine crates (geom-core, geom-brep, topo, sweep, verbs,
  step-import, editor-core, mesh, profile), opt-level 1,
  `CAD_FUZZ_SEED=12345`, on the merged head:
  - **default ε: 15629/15629 green.**
  - **ε 1e-12: 15626/15629.** The three reds:
    - the two `envelope_lemma_fuzz::projected::…_pinned_where_the_frame_cost_refuses`
      rows (seed-dependent; now on
      `work/pcert/a-pinned-fuzz-witness-reads-the-environments-seed.md`, and
      red on the earlier merged bases in my first pass);
    - `sweep a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta`
      (open main-red row
      `work/tesslie/main-red-a-cylinder-wall-…-at-1e-12.md` at `c9cf004f9`).

  I did not rebuild `c9cf004f9` itself, so on main at this exact sha these
  three are taken from their rows, not re-run.
- Not run: 1e-6 on the nine crates; any slow set on the merged head except
  the two rosters; the lane's differential.
- Rosters, on the merged head:
  - **geom-brep: green.**
  - **topo: red** on `chain_parity.rs`, `pcurves/collapsed_side_rows.rs`
    (×2), `pcurves/decision.rs` and `spline_trim.rs`. Every one of these is
    also listed when the same binary reads `c9cf004f9`'s `topo/src`. The one
    extra on main's sources, `boolean/join.rs: travel: name`, is the head
    table's PAIRED entry finding nothing there. So the red is confirmed as
    main's. **`live.rs` does not appear in my run**: it, `conic_section_frame_at`,
    `section_cert.rs: cone_torus` and geom-brep's `APEX_FORM`/`APEX_ON_TORUS`
    are what hosted CI reports, and CI tests the PR's merge ref with a newer
    main (see d4).
- Probe (on my branch at `probes/delta-across-offer-probe.rs`):
  `decide_across` with a band-scaled swing. See d1.
- Mutants, topo lib `tip_count_rows` / geom-brep `--test all pn_apex`:

  | mutant | result |
  |---|---|
  | F3: `off / shares` dropped | killed (`two_admitted_tips_are_counted_at_half_their_least_lever`) |
  | F3: shares for one corner (`halves.min(1)`) | killed (same row) |
  | F3: fold by `max` of `above()` (unsound direction) | **survives** (4/4) — d2 |
  | F5: `e_δ` dropped (`gap.value.abs()` for `gap.magnitude()` at the apex arms) | killed (`a_far_stated_grazing_plane_is_read_to_its_gaps_rounding`, `a_plane_through_the_apex_reads_its_reach_on_its_gaps_rounding`) |

- CI 45222877266 on `cb36266dd` (refs/pull/7442/merge), at step level:
  - `lint` → **clippy FAILURE**: `unneeded_struct_pattern` at
    `crates/geom-brep/tests/cone_torus_pose.rs:229–230`. That file is not in
    the head's tree; it is main's, and current `origin/main` has it fixed.
  - `test (kernel)`:
    - **eps 1e-6/1e-12 step FAILURE**: `topo splitting::spiric_arc::fuzz::random_arcs_at_interval`
      at 1e-6, under an unpinned random seed `0x071f26b5a2315365`;
    - **slow-set step FAILURE**: both rosters, on the names above.
  - `test (app)` → **slow-set-names FAILURE**: dead entry
    `viewer: app::properties_pane_tests::an_offer_an_earlier_op_of_its_frame_outran_is_dropped_and_said`.
  - The mesh budget and corrupt-input jobs are green; the tools (k-lint) and
    interval jobs were skipped; the gate job (`gate ok`) is red.

  None of these reds touches a site of this unit, but the run is not "green
  but the roster".

## Findings

**d1 (MINOR, DEMONSTRATED, `sure`). F1's "none" is false for a band-scaled
swing with a reading that is not zero: a tolerance that decides exists and
none is offered.** The new `decide_across` doc says "where the bracket holds
zero the floor has no size, and the refusal offers no tolerance: none decides
a reading that is its own swing". With the swing a fixed length that holds,
because the floor stays zero at every smaller ε. With a swing proportional to
ε, `travel`'s case, it holds only for a datum of exactly 0. Probe, band
(1e-9, 1e-8):

| datum | swing | refused under | offers | first ε where it decides Positive |
|---|---|---|---|---|
| 1e-9 | 1.5ε | far row `row` (F1's new path) | none | ≈ 8.2e-11 |
| 3e-10 | 9.9ε | floor `row_floor` | none | ≈ 1.4e-11 |
| 5e-9 | 9.9ε | floor `row_floor` | none | ≈ 2.4e-10 |

The `Interval` lane gives the same offers. The floor-row path is not F1's
change (it was already so). The far-row case is F1's, and the geom-core row
`a_row_across_a_swing_offers_what_its_floor_decides_at` asserts
`scaled(3e-10, 9.9, band)` offers none "a floor of no size", which is that
claim pinned. The door cannot know whether its swing scales with ε, so its
"none" is honest only for fixed swings. For `travel` the sentence now says the
reading has no size when it has 1e-9. Claims 1(b) and 1(d) hold, by
inspection and the rows:
- an offered tolerance decides for both fixed and scaled swings, because
  ε′ ≤ (|m| − s)/K and the floor only grows as a scaled swing shrinks;
- the `Interval` floor `(|d| − s).max(0)` quotes `lo − s_hi`, its nearer end.

Claim 1(a), that no verdict moves: not re-taken. Every `decide_across` caller
I read (`intersect`, `torus`, `cylinders`, `locus`, `join`) propagates the
`Err` with `map_err`. I did not trace which ladders compare refusals'
offers downstream.

**d2 (MINOR, DEMONSTRATED, surviving mutant, `sure`). F3's fold direction is
unpinned.** Folding the half-turn corners' levers by `max` of `above()`
instead of `min` of `below()` passes all four `tip_count_rows`, because both
tips in `two_spikes` sit at 1 mm levers. The argument in the comment
(`fate.rs`, above `let shares`) is sound:
- each admitted corner deviates by `δ ≤ (π/2)·ε/L_c`;
- `h` of them sum to at most `(π/2)·h·ε/L`;
- the meter `off·L/(h·π/2)` claims a side only past `Kε`, i.e. `off > K·R_max`;
- a wrong side needs `R_max − π/2 > K·R_max`, impossible for K > 1.

What no row checks is the `L` it rests on, the least lever: no fixture has
two half-turn corners at different levers.

**d3 (NOTE, BY-INSPECTION, `likely`). F4's least band admits at most one
subnormal more than the share it stands for.** Where `ε/K` underflows, the
true share is below `2⁻¹⁰⁷⁴` and `Band::least()` reads `ρ = 2⁻¹⁰⁷⁴` as
Zero. That is unsound by one subnormal, which nothing can see. The callers'
decisions are right in it in every practical sense.

**d4 (MINOR, DEMONSTRATED, `sure`). The frozen head does not merge cleanly
with current `origin/main` (`032c2f9f5`).** `git merge-tree` conflicts in
`crates/topo/src/boolean/join.rs` (`travel_rows`, the half-turn row's
closure: main moved `geom::Line::dir` to `UnitVec3::exact_z()`) and in
`scripts/gates/lever-fixtures.counts`. The unit's new helper
`half_turn_arm_in` still builds `dir: Vec3::unit_z().map(T::from_f64)`, so
the next merge has to carry it onto main's type, in the file that holds this
unit's `travel` site. The hosted CI reds above come from that newer main.

**d5 (NOTE, `sure`).** Brief item 8 names `live.rs` among topo's roster
misses. It is CI's (newer-main merge ref), not the frozen head's (d4).

**Confirmed by reading:**
- F2 is an `Option` fold with a row (`a_loop_with_no_half_turn_corner_is_unread`).
  "Unreachable through `places` for line runs" was not checked.
- F6: one `size` per function (`gap.magnitude()` at each, `apex_start(size,
  sine)`), and `offers_tolerance`/`offered_band` both read one private
  `offer()`.
- F7: the `intersect.rs` diff against main is the unit's reads only, onto
  main's `Rounded`. `lever-fixtures` geom-brep is 502 (the gates step is
  green). The geom-brep roster re-adds only the
  `wedge_reads_zero_at_the_offer` forwarder.
- The two filed rows exist and say what they claim.

## Style (delta)

Q1, Q3, Q5 and Q6 were exercised on the fix pass; Q8 was not.
- s-d1 (Q3, `sure`): d2. A fold with two candidate directions is pinned
  where both give one value.
- s-d2 (Q5, `sure`): d1. The `decide_across` doc's "none decides a reading
  that is its own swing" promises more than the door can know.
- s-d3 (Q1, `likely`): `decide_quoting`, `classify` and `classified` are
  three names for one body. `classify` and `decide_quoting` are each one
  line, and `classified` is the body; the stated aim ("one classification
  body") is met, but with an extra layer.

Ratings for the delta: idiom/structure 4; test quality 3 (d2, d1's pinned
"none"); doc honesty 4 (the PR body's dispositions match the tree where I
checked; d1).
