# ROUND — log

Append-only (`work/README.md`, The log merges by union).

## 2026-11-06 — picked up (`active`)

A new sitting holds the track for its P0 and P1 rows (ten open, 27/30).
The program had no `plan.md` or `log.md` since the 2026-09-20 cut; both
are opened here. First step: a premise check of every row against the
current tree (the rows were filed 2026-09-05 to 09-18), then re-pricing
the legacy `cost: D` rows and cutting units.

## 2026-11-06 — premise check and the cut

A read-only survey checked all ten rows against main `230be11d05`.

- **Closed, already fixed elsewhere:** `arc-carrier-has-three-spellings-under-a-comment-saying-one`
  (PATHS 5b left one chord lowering, `Arc2::from_chord`) and
  `fillet-tangency-is-not-the-constructors-node` (PATHS 5a/5b: a segment
  carries its constructor's arc, the row's route 2). DECIDE's
  `declared-tangency-needs-the-registered-identity-door`, parked on the
  latter, is un-parked with a note that the arc-carrier fillets still
  register no tangency.
- **Re-priced** (legacy `D`): `carriers-are-identical-…` M + design,
  `fillet-step-discoverability-…` M (the refusal as the guide, not docs
  alone), `anchor-fit-…` M + design (a payload choice, weighed with the
  fused-step fork).
- **One unit for four rows.** The fillet-recourse rows
  (`validate-rs-hosts-…`, `fillet-inband-recourse-…`, `the-guided-replay-…`)
  and PROFILETAIL's `profile-endings-say-lower-the-tolerance-and-route-by-name`
  are one repair: ratified D4 ¶1 (ii) asks for a closed decision type
  where `PathError::Escalated` now routes by predicate name, and the
  fillet sentences' home, their tolerance arm and the guided door's text
  all follow from that type. Claimed the PROFILETAIL row (P3 → P1, it
  is the unblocking part); its carrier keeps its other three
  passengers. Filed `the-path-doors-escalation-carries-its-gate` (P1 H)
  as the carrier.
- **Found beside it:** the guided-door characterization row
  `a_guided_replay_still_renders_the_coincidence_recourse_for_a_fillet_gate`
  is vacuous since the declare lever retired (the box lever's own words
  contain "move the geometry"); the unit rewrites it.
- **Not a fork, ruled here:** the P0 turn gate keeps the levered margin
  (D4 [tolerance.margins] (i) requires it); the repair is the refusal's
  class and payload, a faithful elaboration of D4 ¶1 (iv) (the definite
  arm tells its in-band sibling's story). Phase 1 of the unit measures
  and proposes the class; if it needs a binding or editor contract
  change with several viable shapes, it comes back as a fork.
- **Forks, designers first:** carrier identity (two designers drawn,
  byte 217) and the refused corner's payload, rows 9 and 10 together
  because both decide what `CornerRefusal`/`CornerReason` carry for an
  author (three designers drawn, byte 12). Draws on
  `analysis/design-fork/round-*`.

## 2026-11-06 — turn gate, phase 1 adjudicated

The lane reproduced both misclassifications: the arc pair (0.57° on a
50ε leg) and the straight pair (`path_corner_turn`, levered by the
ray-origin-to-anchor separation, where even a 64° corner on a 0.6ε
separation reads "parallel"). `CarriersParallel` has four producers;
the two unlevered ones (`path_carrier_meet` and the circle–circle reach
Zero) are true and keep it.

Ruled unilaterally (a dominant argument, so not a fork): the refusal
kind `NoCornerForFillet` stays, because what the gate decides (no
corner to round at tolerance) is true of every input that reaches it;
`PathNoCornerReason` gains an arm carrying the factors (sin φ, align,
the lever and which lever), named for what the gate decided rather than
for a cause it cannot see. Rejected: a new `PathError` variant with its
own Python tag. That would move exactly parallel rays, where "parallel"
is true, to a new tag, to work around a pre-existing gap (a path
refusal's reason has no Python spelling), which is filed on pydoor
instead. The straight pair's definite arm names its own lever (the
anchor separation); the escalation lane is told its in-band
`path_corner_turn` ending must tell the same story.

## 2026-11-06 — escalation, phase 1 adjudicated

The lane's plan: a closed `profile::Gate` (one variant per decided
name, 77, declared by one table), an `Escalation { gate, source }`
minted only by the crate's funnel, so no ending can be looked up by
name; `DECISION_ENDINGS`, `fillet_recourse_for` and their kin deleted;
the fillet sentences in `fillet_recourse.rs`; the guided door composing
the box lever with the gate's lever through `lever_ending`; and the
read-back sentence's name routing replaced by a closed `seg::ArcRead`.
Ruled: one `PathError::Escalated { site, gate, source }` with a closed
site rather than a second variant for the stored-form read; existing
row kinds kept (`segment_straightness` stays `Lever`). On the two
corner turns my first answer (`Lever`, by analogy with
`path_junction_turn`) was wrong and is withdrawn: both are levered by
a length on the linear band, so they are `Sized` and the conditional
tighten is derived where a smaller ε decides; `path_junction_turn`
stays `Lever` and the lane files a row asking whether it is the odd one
out. Review tier raised from single to **dual** (H): the closed type is
what every path-door escalation carries from now on.

## 2026-11-06 — forks reconciling; a P0 found by measuring one

Both forks asked for facts the designers could not settle by reading,
so a probe lane measured them (branch `round/probes`,
`crates/profile/tests/round_probe_*`):

- **M1:** `line(L).cusp().fillet(r)` and `.cusp().fillet_arc(..)` move
  the straight leg's end vertex BACKWARDS (the ray-extension merge reads
  every by-tangent ray as a forward extension): the authored end leaves
  the path, and past the start the declared cusp is silently dropped;
  both validate. Filed as P0
  `a-cusp-before-a-fillet-pulls-the-straight-leg-end-backwards` and
  dispatched (`round/cusp-fillet-keeps-its-leg`, single FULL review):
  the merge extends a leg only along its forward tangent, which holds
  under either answer to the carrier-identity fork's open sub-choice.
- **M2:** the closed-form radius of the fillet that ends at the anchor
  equals the anchor-fit boundary on every straight×straight and
  line×arc entry, and on 2923 of 3185 grid-A arc×arc entries; in every
  other case it is below the loop's boundary because the other derived
  corner builds there; never above. Exact values land on the overrun
  side about a third of the time, so a rendered value must sit strictly
  on the fitting side.

Reconciliation so far (rows on the `[ev]` PRs): the refused-corner
designers converged on the shape after one round and are writing final
sections on the measurements; the carrier-identity designers agree on
retiring the arc identity decision and have crossed twice on the
straight-leg merge, so the last round asks for the question beneath
the split.

## 2026-11-06 — carrier identity goes to Ev (PR 6542)

Converged after three rounds: retire the arc identity decision (both
sure) and the straight-leg merge with it (both, one unsure). The PR
states the question beneath the split that crossed twice: is a
profile's segment list what the author emitted, or the maximal figure
where merging is free (is loft correspondence the profile's job)?
Draw bytes on `analysis/design-fork/round-carrier-identity`.

## 2026-11-06 — the refused corner goes to Ev (PR 6547); the cusp fix in review

The three refused-corner designers converged after two rounds and a
measurement: the `replay_drawn` side door (PR 6306, built on Ev's
2026-11-02 chat request) is the contract, completed; the anchor-fit
refusal carries both legs' setback and extent plus the closed-form
radius at the anchor, rendered strictly below, and its sentence values
both levers; a closed `Recourse`. A sweep of 40 radii below the
closed-form radius over 4073 entries found no re-trip, so "or less"
holds. Draw bytes on `analysis/design-fork/round-refused-corner-payload`.

The P0 cusp fix is PR 6545 (CI green): `ByTangent { No, Forward,
Reversed }`; only a forward ray merges; a cusp-reversed ray emits its
own segment from the leg's end, which validation refuses as an overlap
exactly as `.cusp().line(..)`. Single FULL review dispatched; it reuses
the probe lane's idle target because the box was down to 8 GB.

## 2026-11-06 — the turn gate in review (PR 6544); friction: disk

PR 6544 (CI green): `PathNoCornerReason::TurnWithinTolerance { sin_turn,
align, lever, lever_of }` at both turn gates' Zero arms, `CarriersParallel`
kept for its two unlevered producers; the sentence prints cos φ beside
sin φ, because `scripts/gates/atan2-sites.sh` refuses a new `atan2`
outside `geom::azimuth`. The lane found the same class at
`JunctionTangent` and an undeclared `SeamTangent` after a sub-ε leg, and
filed it on PROFILE (`a-real-junction-on-a-short-leg-refuses-as-tangent`,
P2). Single FULL review dispatched.

**Friction (a finding):** three implementer lanes and a reviewer on one
container ran the per-session disk allowance from 28 GB free to 3.2 GB
within an hour: the escalation lane's target alone reached 13 GB, a
turn-gate target 7.6 GB. Two build lanes plus one reviewer is what this
box holds; a third build lane belongs in its own cloud session
(`memories/orchestration-model.md`, lanes may run in their own sessions),
which is how `fillet-step-discoverability-six-door-refusals` will run.
- 2026-11-06 — `fillet-step-discoverability-six-door-refusals` dispatched to its own cloud session (session_013mPGJ4pin8WZZttgvZtpq7), branch `round/fillet-step-discoverable`: a Transition refusal names what its state accepts, derived from the transition data; `ProgramStep::Fillet` documents its legs; the reviewer's pad rebuilt from a naive attempt as the acceptance. Review tier: single, STYLE (docs and refusal text over one derived set).
- 2026-11-06 — PR 6544 merges: the P0 turn-gate row closes (single FULL review APPROVE-WITH-FIXES; the fix pass found the straight lever's first wording unfollowable and replaced it with one followed to a valid loop).
- 2026-11-06 — PR 6545 merges: the P0 cusp-fillet row closes (single FULL review APPROVE; fix pass took its one-home predicate, doc and coverage points).
- 2026-11-06 — PR 6549 (discoverability) reviewed STYLE: APPROVE-WITH-FIXES, fix pass sent to its session. Filed `a-corner-behind-the-incoming-rays-start-names-no-recourse` (P1 E): the geometry half of the newcomer's walk, which the acceptance walk's premise could not reach.
- 2026-11-06 — PR 6549 merges: `fillet-step-discoverability-six-door-refusals` closes (STYLE review APPROVE-WITH-FIXES; the fix pass made the `ProgramStep` docs point at the rows, one lattice sentence `profile::Refused` at every door, and filed the `AlongTrace` naming gap on DOCTAIL). Its session is archived.

## 2026-11-06 — the escalation unit lands (PR 6546); the last code P1 dispatched

PR 6546 merges after its dual review and fix pass: one closed
`profile::Gate` routes every in-band ending, `fillet_offset_lever` is
`Lever` (its margin moves against the user as ε shrinks, R1's tallied
finding), and `tests/gate_kinds_followed.rs` pins each gate's kind by
following its tighten. The unit and its four passengers close; the row
is `docs/dual-review-log/rows/PR-6546.md` (tally 1). All three P1 code
units of the original cut and both P0s are on main.

`a-corner-behind-the-incoming-rays-start-names-no-recourse` (P1 E, filed
from PR 6549's review) dispatched: `round/corner-behind-names-its-recourse`,
single STYLE review. What remains at P0/P1 after it: the two forks with
Ev (PRs 6542, 6547), each a build unit once ruled.

## 2026-11-06 — hand-back (`ready`)

PR 6556 merged: the corner-window refusals name their lever, a pair whose
corners differ names each beside its corner, and the last
`.contains("anchor")` text match is gone. With it every P0 and P1 code
row of the sitting is on main: PRs 6544 and 6545 (the two P0s), 6546
(the escalation unit and its four passengers, dual-reviewed), 6549
(discoverability), 6556. Two rows closed as already fixed, two forks put
to Ev.

The three P1 rows left are the forks, parked on their `[ev]` PRs (6542,
6547); each becomes a build unit when Ev rules, and `plan.md` says what
each build touches. No P0/P1 row is dispatchable, so the track goes back
`ready` for its tail (one P2, three P3). The tail's two easy rows were
not taken: neither is cheaper for this sitting than for a successor, and
their item files carry the evidence.

Findings recorded this sitting as classes: the inherited `topo` link
fuzzer red at the default ε (ATREST's row); the disk ceiling of three
build lanes on one container; GitHub's merge ignoring the union driver
on `log.md`, so a unit PR's tracker commit should not touch the log.

Watching: this session stays subscribed to PRs 6542 and 6547 and acts on
Ev's answers there (the second, Ev-decision commit of each design-fork
row; updating the PR in place).

## 2026-11-09 — carrier identity ruled; picked back up (`active`)

Ev on PR 6542 asked what had forced the August compromise the merges
were. A history lane traced it: round 10 (2026-08-11/12) set the
"own segment" state; LIB-RESPELL (#531, 2026-08-15) found validation then
refused a declared tangent joint between two pieces on one carrier
(`TangencyContradicted { same_carrier }`, reproduced on that commit's
parent) and added `extend_leg_to` to dodge it; the OnArc dissolution
(2026-08-16) copied the move for arcs under a bit-identity condition.
The sixth round (2026-09-02) retired the refusal, and no one revisited
the workaround. On main with all three merge sites disabled, validation
refuses nothing. Ev: "this sounds great!"

Took the recommendation (both merges retire). The fork's log row (254;
253 was taken on main meanwhile) carries the decision and the mapping.
`carriers-are-identical-…` unparked and dispatched as the build unit
(`round/extensions-are-their-own-segments`, single FULL review). Loft's
segment-by-segment correspondence, the one remaining cost, filed on
LATHE as `loft-corresponds-sections-by-segment-not-by-run` (P2). PR 6547
answered Ev's two questions (the side door; the anchor lever now states
its target distance) and waits on Ev.

## 2026-11-09 — the extensions unit is up (PR 6682)

The implementer opened PR 6682 with CI green. It deletes the merges and
the identity gate and constructs the tip joint for every `r`. The pins
it re-baselined are listed in the PR: the corpus hash (one row moved),
`path_program`, editor-core `sym_9`, the pncad-py vertex counts and the
tour rocker doc. Single FULL review dispatched on head `6988cd9f9f`.

The PR also turned up a pre-existing editor-core gap: the strand report
reads drawn pieces, not run walls. The implementer reproduced it on
main and filed it on DOCTAIL as
`a-reshaping-that-joins-a-kept-piece-into-a-run-reports-nothing` (P1),
with the two affected rows pinned red. Main's 2026-11-09 nightly is red
across editor-core, sweep and the m10_9 pins, independently of this
unit. The PR does not re-pin m10_9 `r2_rounded_pad`, which this unit
moves further and which is already red on main.

PR 6682's review returned REQUEST-CHANGES with one major finding. With
the merge gone, a bare `fillet(r)` after a straight leg emits runs the
fit gate accepts but validation refuses (`DegenerateSegment`, 156 of 212
on a rounded-corner grid that main builds). That is the P2 class
`a-fillet-run-the-fit-gate-passes-validation-calls-degenerate`, which
the merge used to shield; it is folded into this unit, which closes it.
The minors are folded in too:
- in-band r = R − 1e-9 escalates at validation: trust a constructed
  joint if both answers are legal, else document it;
- one pinned row per sub-case;
- the two duplicate emit lines get one home.
The DOCTAIL strand row goes to P0, since this unit widens it to an
ordinary fillet insert. Fix pass to the implementer.

2026-11-10: 6682's fix pass is green on `9ebbf2ce90`.
- **Fit gate.** `fillet_leg_fit` now decides a run on validation's
  linear band: below the band it is a Zero fit, in the band it
  escalates as `Sized`.
- **Ripple.** At 1e-6, six grid-A anchor-fit overruns fall inside the
  band and no longer refuse; the censuses are now keyed per ε.
- **Rows.** The fit-gate row closes in the PR. The DOCTAIL strand row
  is now at P0.
- **The in-band arc-extension escalation stays.** The escalation sits
  in `seg::arc_arc`, the check that decides whether the run lowers as
  one wall or two. It is now documented.

The fix changed the fit gate's band, which is new correctness surface,
so the fix-pass delta gets a second FULL review before merge.

6682 re-review: APPROVE-WITH-FIXES.
- Confirmed: the Zero-arm geometry (the arc lands ≤ ε off, joints
  stay constructed) and that the in-band escalation is real (its
  reader is `sweep::swept::cosurface`).
- Fix now:
  - F1 (D4): the fit's Sized ending offered a tighten on an in-band
    overrun, which then refuses. Tag it `NonNegative`, reword the lever
    per side, and give it a Driven row in `gate_kinds_followed`.
  - F3: compute the per-ε census from the grid instead of a literal
    lookup.
  - F4: stale docs.
  - Style: the arc run-in gets one home.
- Filed and pinned, not fixed: F2, a ~1ε build-then-escalate window just
  above Kε (the span checks on the non-adjacent pair). The window is
  new for the ray and arc-tip extensions and already on main for
  authored runs. The fix belongs in validation's span reading, so it
  gets a ROUND P2 issue.

## 2026-11-10 — the extensions unit landed (PR 6682); handed back (`ready`)

PR 6682 merged as `e7acc15c96` after the review, two fix passes and a
second FULL review. The carriers row and the fit-gate row are closed.
The span window just past the band is filed at P2. The second fix pass
added a small `decide_passing` door in geom-core so the fit's in-band
overrun offers no tighten; I read that pass myself and did not
re-review it, since each fix carries a mutation-checked row.

No P0 or P1 row is dispatchable: the refused-corner pair waits on Ev's
answer on PR 6547. The two cost-E P3 refusal-sentence rows ride with
that unit, since it rewrites the same sentences. ROUND goes back
`ready`, with the P2/P3 tail on the board. When Ev answers 6547:
- record the decision and the label mapping in its DESIGN-FORK-LOG row,
  renumbered past main's last;
- refresh its PR body;
- unpark the two rows.

## 2026-11-10 — the refused corner ruled (PR 6547); picked back up (`active`)

Ev, after two rounds of questions on the PR: "ok, these changes sound
good then!" The answer carries both decisions with Ev's two wording
changes:
- the anchor lever states the target distance, not an increment;
- the sentence drops the redundant current distance.

The fork's log row is 255 (254 was taken on main) and carries the
decision and the mapping. Both rows are unparked into one build unit,
`round/refused-corner`, which gets a single FULL review: the payload
and record changes reach the viewer and Python, and the closed-form
radius has branch assignment to get right. The two P3 sentence rows
ride with it.

PR 6719 (the refused-corner unit) is up, and its single FULL review
returned REQUEST-CHANGES.
- **The major (M1).** The rendered X and S sat only ~1e-10 on the
  fitting side, inside the fit gate's own band. Typed back, they
  re-tripped `fillet_leg_fit` as an escalation (1215 grid-A cases at
  ε = 1e-12). The pinning test only looked for the entry kind, so it
  could not see this. Fix: render outside the band, and have the
  typed-back row assert no `fillet_leg_fit` refusal of any form.
- **Minors folded in:**
  - noisy 10-figure numbers;
  - the README parenthetical reworded to "each overrunning leg";
  - `CuspDoublesBackOverLeg` added to PATHS §3's list;
  - the flip band asserted per ε row;
  - the real over-budget distribution recorded;
  - one home for `num`/`num_toward`, and the `Drawing` step check
    made fail-loud;
  - the which-legs type made unable to say neither;
  - the turn refusal's corner drawn.
- **The cusp passenger's new door refusal is accepted.** The row left
  the choice to the unit, after `.cusp()` the lattice admits only the
  exact reverse, and the reviewer found no valid profile it refuses.

6719's first fix pass named the levers twice the escalation threshold
past the band (2Kε). The re-review found that this leaves a 2Kε sliver
at the anchor, and the anchor junction then re-refuses: 355 of 3148
default-row cases, against the old head's 2. Ruled: aim both levers at
the exact-fit band instead, with the margin in [0, ε/2] on the fitting
side. Since PR 6682 the fit gate's Zero arm is an exact fit, so the
named fillet ends at the anchor. That is the ratified "the fillet that
ends exactly at the anchor", and it lets the README keep the wording Ev
approved ("strictly on the fitting side"). The first pass's "past the
band" rewording would have changed what that clause decides, so it
comes out. The typed-back rows now pin per-row counts of other-gate
refusals, and an independent margin ties the bound to the kernel.

## 2026-11-10 — the refused corner landed (PR 6719); handed back (`ready`)

PR 6719 merged as `05e7001f94` after one FULL review and two fix
passes; the four rows (two units, two passengers) are closed.
- **The rendering rule.** The levers are named within the exact-fit band
  on the fitting side, so the named radius and anchor distance, typed
  back, are exact fits at their corner. Other gates still refuse a
  handful of typed-back levers (4 X and 3 S on grid A at 1e-6, 0
  elsewhere), pinned per row.
- **A reading I accepted.** Below the window, a radius in the fit gate's
  own band escalates there in that gate's words. That is the sliver
  region the contract leaves to other wording, so the sweep below X
  counts it rather than failing it.
- **The cusp passenger** refuses at the door as `CuspDoublesBackOverLeg`.
- **CI.** The implementer pushed two empty commits to re-run CI after
  `test (kernel)` tripped the 1.0 s per-test budget on two untouched
  rows (topo 1.032 s, sweep 1.014 s), since re-runs are refused from
  here. That is a workaround I would not choose again: those rows sit at
  the budget's edge and belong to whoever owns the slow-set roster.

No P0/P1 is left. Two new P3 rows came out of the unit: the enclosing
refusal's rendering, and the over-budget sentences. Both need a
wording choice, so a successor can take them as cheaply. ROUND goes
back `ready`.
