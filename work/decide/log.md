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

## DECIDE-1 measured and the row CLOSED (2026-09-21): the mechanism does not reach the certification path

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
not clause 1's class) and which none of the six documents decides. It is
filed on SHELL's slate, and the gate's structural blindness to the
self-dot spelling on GUARD's.

**The dynamic measurement** recorded ZERO clause-1 `Invalid` refusals on
the six documents at ε = default, `1e-6` and `1e-12`, at the nominal and
at ceiling + δ. Every blocked replay was blocked by exactly one
predicate and every one was `Indeterminate` — a real margin over the
band, never a domain violation. The instrument is
`crates/editor-core/tests/decide_1_self_dot_interval.rs` (ignored,
prints) and its measured ceilings reproduce `m10_10_pins_interval`'s
pinned table digit for digit at all three ε, which is what says the
census measured the documents the pins measure.

Phase 2 is empty; `Vec::dot`, `powi` and the allowlist gate's logic are
untouched. Two deviations, both stated in the PR body: the ceiling is
MEASURED per run rather than read off `m10_9_pins`' table (that table is
M10-9's tier's, and replaying at another tier's refusing end gives a
zero over a replay that was never blocked), and the pad is measured
opt-in because its replay under the shape report is killed for memory on
a box this size — its ceiling instrument row is in the PR.
