# DECIDE — the log

## 2026-09-20 — opened

Cut out of SYM, which was carrying 78.5 budget points, when Ev
ratified the priority and track-size conventions in chat the same day.
The cut followed SYM's PRIORITY seam per `work/README.md` "Track
size", into several tracks at once so they can run in PARALLEL — Ev, in
chat: *"for these high priority tracks it's ideal to have several
components that can be worked on in parallel."*

8 rows arrived by `git mv` with their ids, bodies and history
unchanged. SYM keeps its band 5800-5899; band 8600-8699 is claimed
for this program in the same commit (`docs/MODEL-AB-LOG.md`). Nothing
dispatched.

## The fork reconciliation, SYM-8's dual, SYM-10's hand-over (2026-09-21)

The SYM orchestrator session was forked on 2026-09-19 (one copy on
Ev's local machine, one in the cloud). The local copy dispatched
SYM-8's v6 dual on 2026-09-19 (ordinal 4704, claimed in SYM's band
before the 09-20 cut; byte 98 ⇒ R1 = OPUS, R2 = FABLE; both arms on
the shared local box, editor-core re-takes riding the hosted gate),
dispatched SYM-10 to slot 2 with SYM-9 displaced, and answered PROPS's
seams on #2468. The cloud copy, out on its usage limit from 09-15 to
09-21, merged main into #2616 on its return. The two reconciled on
`[ev]` #2949 with Ev's delegation ("don't wait on me"): the local side
adjudicated SYM-8's union (both reports verbatim in
`work/sym/logs/fork-state-local.md` on that branch; R1 OPUS
MERGEABLE-AFTER-FIXES 2/3/4, R2 FABLE 1/4/7, the same two defects found
independently, no unilateral MAJOR) and runs the R1 delta; the cloud
side runs the fix pass, state-sync, merge and the row, takes SYM-10
from `fb01a201c`, and is the orchestrator of DECIDE and SYM's remainder
going forward. The block record is `sym/b2-block` (the local side's
"slot 1 dual concluded" at `2fd3edf11`).

**Protocol v7 triage, recorded where the unit lives:** SYM-8 is triaged
IN — a rule of the atom algebra, H / NUMERIC, spec'd 2026-09-14 under
v6 before the seam, its dual dispatched 2026-09-19 22:50Z. **The
orchestrator's model, per phase:** spec, dispatch and adjudication
FABLE (the local fork); fix pass, state-sync and merge FABLE (the cloud
session); the unit changed hands at the fork. SYM-10's implementer arm
is FABLE on both sides.

## SYM-8 merged (2026-09-21): the manifest sign — block SYM-B2 slot 1

PR #2616 merged (fix-pass head `7edff5e97`, run 35554361889 green on
the full matrix; the state-sync commit after it carries main and the
delta's one label finding). Rule F (`sym/manifest.rs`,
`manifest_sign`): `copysign(Y, X) → abs(Y)` and `abs(X) → X` where the
form `X` is manifestly POSITIVE (`manifest::positive`: a term-wise
non-negative numerator with one strictly positive term of manifestly
positive indeterminates, over a manifestly non-negative denominator;
the perfect-square branch and mere non-negativity deliberately
excluded — the signed-zero edge), a value-free fold ordered before
rule C at the `abs` node so a theorem is never counted `sign_gated`.
Measured first on the tilt-U wall: `carrier_endpoint_end` 24/0/0/1 →
33/0/0/0 with the refusal moving by name to `newell_plane_residual`
(the gating row
`m10_the_tilt_u_derived_boss_stops_on_the_newell_residual_and_names_it`).
No per-predicate split or whole-certifying ceiling moves on the eight
measured documents; the ring item's recorded loss is not re-taken by
the fold. The pad's four RULED a ratified spec deviation (the item;
`symbolic_zero` pinned on all five pad documents). The reviews' four
documents run as one ladder: rule F moves no count on any of them;
`tiltUV`, predicted by both reviews as the shape rule F folds, moves
nothing — R1's delta retracts the prediction and reads the likeliest
mechanism as `n.z`'s subform freezing before the `abs`/`copysign` node
combines (the budget, not the predicate); recorded as unmeasured on
the derived-frame item.

Fix pass (the cloud fork's lane, on the local fork's union brief
`work/sym/logs/fork-state-local.md` plus six addendum rows): items A–H
and the addendum all taken; one argued deviation on A —
`shipped_without_the_door()` keeps rule F (its contract is "shipped
minus the door and nothing else"; the false "M10-8's tier exactly"
sentence retired instead; R1's delta read all 14 uses and confirmed
it sound, and its one new non-gating MINOR — the sentence surviving
in two more labels — is taken in the state-sync commit); the ordering
pin re-made with R1's and R2's discriminating rows, and C-before-F
planted reds exactly those two; R2's f64-lift adversary reproduced 6
of 6 and adopted `#[ignore]`d with a gating interval twin;
`ATOM_DEPTH` 8 → 4; the interval rows moved to a wholly gated file
after `discipline (evaluation-code)` red an intermediate run. Delta by
R1 (local fork): MERGEABLE. Row at ordinal 4704, sample #226
(`docs/MODEL-AB-LOG.md`, with the v7 triage line and the per-phase
orchestrator model). Spec deleted with its ledger entry. Item
`work/decide/SYM-8.md` → `closed` (`pr: 2616`); the slate drops 5 points.

Left on the program: `tiltUV` unmeasured (one `explain_depth` render;
the derived-frame item); the five other `copysign` mint sites and the
sign-hull frame's `|n.z|` as the next shape (the PR body's motivation);
the pad's split OOM on TIER's cost row; the three-spelling atom class
on TIER's `sym-rs-is-one-file` item.

## DECIDE-1 spec'd and dispatched (2026-09-21): the self-dot straddle, a census and a measurement

The plan's next after SYM-10 — SYM-10 waits on Ev's ruling on the
fourth piece (#2970, the orchestrator recommending the full canonical
root at the mint site) and SYM-9 is an edit to the manifest SYM-10
defines, so the reach row with the clearest reproducer goes first.
Reading the item against the tree before writing the spec: the fix it
names has existed since M2 PR 4 for every norm (`norm_squared` squares
component-wise through the tight `powi(2)`; every carrier and path
length in `crates/profile` goes through it; `Sym::powi` delegates to
the value channel), so the item's mechanism can reach a measured
document only through a hand-spelled self-product at `Interval`. The
unit is sized to that: Phase 1 a static census and a dynamic count of
clause-1 `Invalid` refusals on the six documents with their mechanism
rendered, Phase 2 the tight square at any site found (bit-identity
pinned, the six documents re-measured, the LINALG seam announced), the
row CLOSED on the measurement if nothing is found — a measurement that
closes a row is a result. Difficulty D, class NUMERIC. **Triaged OUT of
protocol v7** (a census and a measurement, the fix class ratified):
OPUS implementer, OPUS reviewer, the review's depth decided at the PR;
no draw, no ordinal, no row. Spec `docs/DECIDE-1-SPEC.md`; branch
`decide/1-self-dot-census`; the lane is dispatched from this commit on
this box (a fresh lane, private target). Block SYM-B2's slot 2
(SYM-10) stays open; this unit takes no slot.

## DECIDE-1 measured, reviewed and the row CLOSED (2026-09-21): the mechanism does not reach the certification path

Both halves of the spec's stop clause hold. **The static census** found
no self-product at `Interval`/`Sym<Interval>` on a certification path a
measured document takes: the lane swept `x.dot(x)`, `dot(a, a)` and the
interval-square gate's own three blind spots (indexed, repeated-call and
ALL-CAPS operands) over `crates/*/src`, and ran the gate itself green
(440 files, no unratified `x * x` outside the seven allowlisted). Every
hit was f64-only, under `#[cfg(test)]`, nonnegative by construction, or
consumed by `mid(·)` into an f64 seed — except one:
`topo::transform::check_rigid`'s three unit-column residuals, generic
over `T: Decide` and reached at `Sym<Interval>` from `eval/wire.rs`'s
placement, whose consumer is a `sign_within` rather than a `sqrt` (so
not clause 1's class) and which none of the six documents decides — the
probe prints the `transform_rigid_*` rows that DECIDED on each replay
and it is `none` on every one. Filed on SHELL's slate; the gate's
structural blindness to the self-dot spelling on GUARD's.

**The dynamic measurement, stated as what was measured.** A replay
escalates at its FIRST blocked predicate and stops, so the count is over
the decisions SEEN: **every replay that was blocked stopped at its first
blocked predicate, which was `Indeterminate`, and `Invalid` is zero over
the decisions seen** — at ε = default, `1e-6` and `1e-12`, at the
nominal and at ceiling + δ. The annulus sees 352 of 677 and the link
465 of 1102; the plate's replay is not truncated at all (1413 of 1413,
one indeterminate `assert_bound`). The zero holds past the ceiling too:
R1's review probe, adopted into the suite with credit, re-takes it on
the plate at `s = 0.5, 1.0, 2.0` and the annulus at `s = 1.0, 2.0` over
the whole box, `s = 1.0` being the real study — all `Invalid 0`. The
instrument is `crates/editor-core/tests/decide_1_self_dot_interval.rs`
(ignored, prints, asserts nothing) and its measured brackets lie INSIDE
`m10_10_pins_interval`'s outward-rounded pinned ones at every ε (plate
at `1e-9`: `0.2630626…/0.2631643…` inside `0.2630/0.2632`), which is
what says the census measured the documents the pins measure.

Phase 2 is empty; `Vec::dot`, `powi` and the allowlist gate's logic are
untouched. **Five deviations**, each stated in the PR body: (1) the
ceiling is MEASURED per run rather than read off `m10_9_pins`' table —
that table is M10-9's tier's, and replaying at another tier's refusing
end gives a zero over a replay that was never blocked; (2) the pad is
measured on the ceiling instrument only, its replay under the shape
report being killed for memory on a box this size at the nominal as
well as over the whole box; (3) the pad's ceiling search has its own
default bracket, the general one not finishing in an hour and a half at
46 s a probe; (4) nothing was rendered at `explain_depth` — there is no
`Invalid`, so there is no refused residual to render; (5) no gating row
was added — a measurement that found nothing has no state to gate, and
the gap that leaves is filed on GUARD's slate.

**Review: one Opus STYLE review (R1), MERGEABLE-AFTER-FIXES, 0 MAJOR /
5 MINOR / 9 style, both hypotheses CONFIRMED by execution** (the census
re-run three ways with 0 misses; plate, annulus and the pad's bracket
reproduced on an independent box; wider scales also 0 `Invalid`). No
draw, no ordinal, no A/B row — the unit was triaged OUT of protocol v7
at spec time. The fix pass took A–M: the record's five MINORs (the
truncation now said and columned, the `work/README.md` citations
corrected, two standing sentences swept, "digit for digit" replaced by
"inside the pin's bracket" with the probe's own numbers, the log's
deviation count fixed) and eight of nine style items (the per-predicate
blocked table and the `head` helper given one home in
`m10_8_harness.rs`, the search knobs put in one unit, the NaN-`lo` path
named, the decided-predicate readout adopted from R1, the module doc
saying the suite cannot red, R1's wider-scale rows adopted, the `all.rs`
entry moved to the tail's end). S9 declined as taste.

**Costs:** implementer ≈271k tokens / ≈4.4 h; reviewer ≈163k tokens /
≈20 min.

## DECIDE-2 spec'd and dispatched (2026-09-21): one pin per seam

With DECIDE-1 merged (#3001) and SYM-10 still waiting on Ev's ruling
(#2970), the plan's order is re-read: `declared-tangency` waits on
BLEND's constructor change, `revolve-carriers` on a document that is
bounded by a revolve carrier (E6), `rule-d-reaches-the-unit-bulge-only`
on the ring's width (SYM-9's subject), and SYM-9 itself on SYM-10 so
that it is an edit to the manifest rather than a re-design. The
drive-by `registered-is-spelled-five-times-and-pinned-once` (E) is
free of all of that and is taken now, with the decision it asks for
RULED by the orchestrator: shape 2, a pin per seam (shape 1 would
make `k_stats` depend on `sym` and pins nothing across the k-lint
workspace boundary; shape 3 is what let three seams go unpinned when
the door was built). Three rows in `outcome_vocabulary.rs`'s shape,
each shown to red under a planted sixth kind; no behaviour changes.
**Triaged OUT of protocol v7, mechanical tier**: OPUS implementer,
green CI and the orchestrator's read, no review lane, no draw, no row.
Spec `docs/DECIDE-2-SPEC.md`; branch `decide/2-discharge-pins`.

## DECIDE-2 merged (2026-09-21): one pin per seam — and DECIDE-1's merge, recorded

DECIDE-1 merged at `dc224d2f5` (#3001; its own entry above carries the
review line, the five deviations and the costs; the fix leg cost a
further ≈428k tokens / ≈2.75 h on the same agent, two `DOC-LEDGER`
tail conflicts with main resolved on the way). DECIDE-2 merged at
`ced1eb040` (#3011, head `f08190e8b`, run 35605501274 green): three
pins in `outcome_vocabulary.rs`'s shape — `sym::discharge_pins`
(`Discharge` ↔ `SymCounts`, `Discharge` ↔ `ShapeOutcome`, library rows
because `Discharge` is private) and `k_stats_doors`'s
`every_discharge_kind_retags_its_sample_with_a_token_of_its_own`
(`Discharge` ↔ `SampleOutcome`, an integration row reading a new
`probe`-only door `sym::discharge_sample_outcomes`, because a
`probe`-gated library `#[test]` is compiled by CI and run by nothing —
filed on GUARD as
`feature-gated-lib-unit-tests-are-compiled-and-never-run`). The lane's
correction to the item's premise, kept: every `match` on `Discharge`
is exhaustive already, so what compiles-and-passes is an arm folding a
sixth kind into an EXISTING column, row or token; the rows assert
INJECTIVITY, not totality, and each red under the planted sixth kind
(`46e984c82`, reverted). The retag site's mapping is factored into
`Discharge::sample_outcome()` so the pin reads the production
projection. Mechanical tier: merged on green and the orchestrator's
read of the diff and the plant table; no review lane, no row.
Implementer ≈204k tokens / ≈53 min. The spec is deleted with its
ledger entry; the item and the unit are closed. The slate drops 6
points (17.5 remain); what is left waits on Ev (SYM-10, #2970), on
SYM-10 (SYM-9, `rule-d-reaches-the-unit-bulge-only`), on BLEND
(`declared-tangency`) or on a document (`revolve-carriers`, E6).

## Ev's ruling on the fourth piece; SYM-10 closed on Phase 1; DECIDE-3 cut; block DECIDE-B1 opens (2026-09-21)

Ev on #2970: "(a) sounds great" — the full canonical root, at the mint
site, after the orchestrator withdrew its narrow-form recommendation
(re-baseline avoidance dressed as caution; Ev: "having to re-baseline
is never a reason to skip a code change for the better"). Recorded on
`the-candidate-norm-needs-a-canonical-square-root` (`needs_ev` off,
"Ruled"). SYM-10 CLOSES on its Phase 1 measurement — no rule written,
#2970 the record, merged to `props/sign-hull` per its spec, the spec
deleted with its ledger entry; block SYM-B2 slot 2 concludes with no
dual run (the unit stopped at its stop clause before a frozen head
existed). The three rows SYM-10 filed on this branch and FRAME's
evidence row are carried to main here byte-identical to the lane's.
Phase 2 is **DECIDE-3** (`docs/DECIDE-3-SPEC.md`; branch
`decide/3-canonical-root`, cut from `sym/10-decision-door`'s head with
main merged in; PR against `props/sign-hull` while #2468 is open): the
canonicaliser in `mint_atom`'s `Sqrt` arm, the `D ≥ 0` side condition
argued once, the two reads behind every value-free fold, acceptance
"no decision lost on the six documents" with everything that moves
said. **Triaged IN under v7** (a canonical-form decision over every
atom the tier keys; H / NUMERIC, pre-draw). **Block DECIDE-B1 opens**
on `decide/b1-block`: slot 0 = DECIDE-3, slot 1 = SYM-9 (its spec on
main since #2602; H / NUMERIC), slot 2 = `rule-d-reaches-the-unit-bulge-only`
(D / NUMERIC, spec at dispatch); the draw is recorded there. The
slate drops SYM-10 (12.5 points remain).

## DECIDE-3 merged (2026-09-22): the canonical root at the mint site — block DECIDE-B1 slot 0

PR #3039 into `props/sign-hull` (landing head `5d3bc7e8b`, run
35687782592 green on the full matrix). Rule G: every `sqrt` atom keyed
on its argument's value class at the one mint site — content
rationalised `(n·d)/d²` and split off, `sqrt(N/D) = sqrt(N)/sqrt(D)`
under `D > 0` proved from the form (a non-negative constant,
`manifest::nonneg` with a definite quadratic in one indeterminate, a
manifestly non-positive denominator over the negated pair, the
certified bracket), `sqrt(R²) = |R|` through the magnitude door, `|Y|`,
`|−Y|`, `|c·Y|` and `c·|Y|` one atom; the two certified reads (at
`Select`, at `min`/`max`) behind every value-free fold; A0 deciding a
`min`/`max` of two rational constants and of one form as theorems;
rule A's `|X|² = X²` dialled with G. The tilted row green with its
assertion untouched; the tilt-`u` wall answered
(`work/sym/the-tilt-u-newell-residual-is-the-next-wall` closed); the
plate 811/0/140/462, the link 541/0/96/465, the bracket
1104/7/144/766, the pad 890/6/150/907, the slab 490/255 with nothing
gated.

Review: the dual on `a200f768d` against `decide/3-review-base`
(R1 OPUS MERGEABLE-AFTER-FIXES 1/5/6 — two plate re-baselines
misattributed; R2 FABLE NOT MERGEABLE 3/6/6 — the side condition's
atom-table source UNSOUND by a dead-arm adversary, the link's trade,
the un-dialled `Abs` rewrite). Fix pass A–S. The unit STOPPED at C on
the link's `carrier_on_surface_2` (sixteen theorems lost per predicate
while the document's totals rise; the remedy tried recovers ten and
costs forty) against the spec's "no decision LOST" acceptance, which
the ruled row had carried as Ev's ruling. **Ev corrected the
attribution on #3039 (02:03Z)** — the sentence was the orchestrator's
spec text; the ruling is shape 1 plus "never skip out on a change that
would make the code better because it would require rebaselining" —
and added (02:05Z) "make sure that the code that goes in is clean, and
doesn't have any concession towards skipping a rebaseline". So: the
link re-baselined and pinned on both sides, the row open at P1 for
SYM-9; item H's concession (the read declining constant comparisons to
keep the slab pin) reversed into the A0 fold, with the slab, the
plate's `line_span`, the D-tabs, the boss, the ledgers and the
derived-frame A0 rung re-baselined and said
(`a0-leaves-max-and-min-of-constants-opaque` closed). Delta by R1 on
`9399ddaf7`: MERGEABLE, both rulings applied, five non-blocking
findings taken as a polish pass (`5d3bc7e8b`). Spec deleted with its
ledger entry; the ruled row and the unit closed. Two rows stay open
from the unit: `rule-g-trades-sixteen-of-the-links-carrier-on-surface-2`
(P1, SYM-9's) and `decision-read-triples-the-plate-pin-suites-wall-time`
(P2). The A/B row is recorded on main at the merge. Next: SYM-9 (block
DECIDE-B1 slot 1). The SYM-12/DECIDE-3 seam
(`work/sym/the-negative-arms-denominator-clause-widens-with-decide-3s-definite-quadratic`)
is the `props/sign-hull` merge's.

## SYM-9 in review (2026-09-22)

Block DECIDE-B1 slot 1, arm OPUS. PR #3083 against `props/sign-hull`.
The retry ladder: a decision every rung of the first attempt refuses is
re-asked with a rule that opens an atom shut. Phase 1's tables chose
the two kept-atom attempts over the wider ring by measurement — the
ring recovers a strict subset at 4.4x to 11.6x. Twelve decisions on R2's
link (the ten `rule-g-trades-sixteen-…` records as lost) and six on R2's
filleted bracket. Rows: the rule-G row answered to its last shape and
downgraded to P2; the decision-read cost row read and left where it is;
one new row filed on SYM's slate for the pad's unmeasurable replay.

## SYM-9 fix pass (2026-09-24)

Both reviews MERGEABLE-AFTER-FIXES; the union taken. The first cut ran
the ladder inside every rules differential (it rode
`SymbolicDials::default()`), which read rule G's own cost on the link as
recovered; the differentials now run with no ladder and DECIDE-3's link
pin is rule G's trade again. On the release leaf instrument the ladder
adds 36 % on the bracket, 14 % on the link and 12.5 % on the pad, all
three over the 1.6 s line before it — so the drive ships no ladder and
`SymRetry::kept_atom` is the dial. The decision read measured at zero
on that instrument for those three documents. One row filed on SYM's
slate (the retry attempts' missing cross-check).
