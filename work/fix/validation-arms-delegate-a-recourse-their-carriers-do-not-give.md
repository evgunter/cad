---
id: validation-arms-delegate-a-recourse-their-carriers-do-not-give
kind: issue
title: four ValidationError arms delegate their recourse to a carrier whose own messages give none — Class B of the census recourse sweep
status: closed
opened: 2026-09-11
pr: 2403
branch: fix/recourse-arms-remainder
closed: 2026-09-12
refs: [2403, recourse-chain-stops-at-the-second-hop-carriers]
---


(FIX orchestrator) From the fix pass of PR 2354
(`census-flattens-the-typed-chart-region-declines`), which was told to
sweep the class after its own instance of it was found by review.

## The class

A `ValidationError` arm whose `Display` is a thin wrapper — sometimes
literally `"tier 3: {error}"` — over a **carried** error, on the
assumption that the carrier's own message names a repair. When the
carrier does not, the message a user reads states a condition and
stops.

That is the defect PR 2354 introduced and then removed at the census
door: it dropped a blanket recourse tail because "the cause supplies
the recourse", and **nine of thirteen causes did not**. The unit fixed
its own carriers and made the claim enforceable
(`chart_region::tests::every_chart_region_arm_names_a_recourse`, which
found the ninth arm a reading by eye had missed). These four arms make
the same assumption about four other carriers.

| arm | whole message | carrier | literals with a recourse clause |
| --- | --- | --- | --- |
| `Band { error }` | `"tier 3: {error}"` | `BandError`, `crates/geom-core/src/predicate.rs` | **0 of 2** |
| `VolumeUncomputable { source }` | — | `MassPropsError`, `crates/topo/src/props.rs` | **0 of 5** |
| `Pcurve { finding }` | `"tier 3: {finding}"` | `PcurveMintError`, `crates/topo/src/pcurves.rs` | **1 of 9** |
| `ApproxCertification { error }` | — | `OffsetFitError`, `crates/geom-brep/src/offset_fit.rs` | **1 of 8** |

`Band` and `Pcurve` are the sharp ones: the arm contributes four words
and the carrier contributes everything else, so whatever the carrier
fails to say is simply absent.

Six further arms carrying `Indeterminate` are **sound** and are not on
this list — `Indeterminate`'s `Display` ends in `COINCIDENCE_RECOURSE`.

## The blind spot, stated

The counts come from a verb-vocabulary match over string literals. **It
is a signal to read the arm, not a verdict on it** — a carrier can name
a repair in words the match does not know, and a match can fire on a
sentence that merely contains a verb. Every row above wants reading
before it is fixed.

## What this needs

Per carrier, the same choice PR 2354 faced: give the arms their
recourse (which makes the wrapper's assumption true and is what that
unit did), or have the wrapper supply one (which was what it removed,
because a blanket tail is wrong for some causes). The enforcement row
is the part worth copying either way — a claim of the form *every arm
names its recourse* is cheap to make mechanical and was worth a ninth
arm within minutes of being written.

## Fences — four carriers, four owners

`crates/geom-core/src/predicate.rs` and
`crates/geom-brep/src/offset_fit.rs` are **PROPS's**;
`crates/topo/src/pcurves.rs` is **TRIM's**; `crates/topo/src/props.rs`
is Track M's, which was S-CERT's and S-CERT is closed — that ground
wants an owner named before the row is taken. Homed here because it is
one class with one repair shape and no single program owns the four.

## Cut by fence: what the first PR took (2026-09-12)

**Taken — the whole `VolumeUncomputable` carrier chain, plus the
shared `BandError`.** The choice PR 2354 faced is settled the same way
it settled it: the arms get their recourse, the wrapper supplies none.
Three enforcement rows carry the claim, each proved red-capable by
mutation.

* `BandError` (`crates/geom-core/src/predicate.rs`) — all three
  variants. Reached by `MassPropsError::Band`, `PcurveMintError::Band`,
  `SelectRefusal::Band`, and the `Band` arms of `split_reduce`,
  `boolean_reduce` and `chord_join`, so the remaining rows below
  inherit it.
* `MassPropsError` (`crates/topo/src/props.rs`) — its three own arms;
  `Band` and `Face` delegate, and the row is transitive over them.
* `PropsError` (`crates/geom-brep/src/props/mod.rs`) — **a fifth
  carrier the item does not name**, reached through
  `MassPropsError::Face`. Four of its eight arms stopped at the
  condition. Without it the chain's claim is false at one remove.

**Remaining, and why.** Two rows are untaken:

* `Pcurve { finding }` → `PcurveMintError`,
  `crates/topo/src/pcurves.rs` (TRIM's). **Ten arms, not the nine the
  item counts**, and by reading NONE of them names a recourse — the
  `1 of 9` the verb match reported is a false positive. Its `Certify`
  and `Escalated` arms delegate further.
* `ApproxCertification { error }` → `OffsetFitError`,
  `crates/geom-brep/src/offset_fit.rs` (PROPS's). **Twelve variants,
  not eight**; several already name their lever (`BudgetExhausted`
  names `OFFSET_FIT_BUDGET`, `SampleCapReached` names
  `OFFSET_FIT_SAMPLE_CAP`), so this row is the one where reading most
  changes the verdict. Four arms delegate to carriers of their own.

The cut is by fence and by risk: thirty-odd recourse clauses in one
PR, written by a lane reading these domains for the first time and
with no reviewer downstream, is how a wrong repair gets shipped in
confident prose — which is the failure PR 2354 removed a blanket tail
to avoid. The pattern, the enforcement-row shape and the mutation
proof are established here for whoever takes the other two.

## Closed (2026-09-12) — the two cut carriers

**Taken: `PcurveMintError` and `OffsetFitError`, the two rows the first
PR cut by fence.** Both settled the way PR 2354 and PR 2403 settled the
others: the arms get their recourse, the wrapper supplies none. Two
enforcement rows carry the claim,
`every_pcurve_mint_error_arm_names_a_recourse` (`crates/topo/src/pcurves.rs`)
and `every_offset_fit_error_arm_names_a_recourse`
(`crates/geom-brep/src/offset_fit.rs`).

**Both counts in the table above are wrong, and both corrections are
the first PR's, re-derived here by reading:**

* `PcurveMintError` is **ten variants, not nine**, and **none of them
  named a recourse** — the `1 of 9` the verb match reported is a false
  positive on `LoopDiscontinuity`'s "*a branch is chosen once per loop
  and certified*", which describes the algorithm and tells a user
  nothing. Seven own arms got a clause; `Band` and `Escalated` needed
  none (their carriers already carry one); `Certify` delegates to a
  carrier that does not.
* `OffsetFitError` is **twelve variants, not eight**, rendering
  **thirteen messages** (`BoundNotFinite` renders its two `last_finite`
  cases as two different messages, sending the caller to two different
  levers). **Four of those thirteen were already right** and were left
  alone: `BudgetExhausted` names `OFFSET_FIT_BUDGET`, `SampleCapReached`
  names `OFFSET_FIT_SAMPLE_CAP`, and both `BoundNotFinite` renderings
  name the lever that is NOT the budget. Five arms got a clause; the
  other four delegate.

**One defect found beside the row, in the same match.**
`OffsetFitError::Structure` rendered its `SplineError` through `{e:?}`
although that type has a `Display`. A Debug rendering cannot carry a
recourse a reader can act on, so the arm could not have delegated one
even once its carrier has it. Changed to `{e}` in the same PR.

**What pinned these messages: nothing.** Neither carrier had a single
test discriminating any rendering — `validate.rs`'s Display-coverage
row asserts only `!err.to_string().is_empty()`, and every other hit is
a panic-message interpolation. As at PR 2403's carriers, the absent pin
is recorded as part of the defect and the enforcement row is the first
pin either type has had.

**Residue, filed rather than disclosed in prose:**
`work/fix/recourse-chain-stops-at-the-second-hop-carriers.md`. The
class is every carrier reachable from a delegating arm, and a verb
match cannot see past the first hop; five carriers at the next hop
(`PcurveCertifyError`, `MeterError`, `PatchBoundError`, `FitError`,
`SplineError`) still stop at the condition. Both rows here assert a
delegating arm transitively ONLY where the carrier below has an
enforcement row of its own, and assert the bare delegation otherwise,
so neither claims a chain it has not proved.
