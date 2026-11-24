---
id: user-facing-number-format-is-not-one-across-payloads-endings-and-escalations
kind: issue
title: geom-core: the user-facing number format is not yet one across refusal payloads, sized endings, Indeterminate payloads and the blend payload
status: parked
opened: 2026-10-14
priority: P4
cost: E
blocked_on: [patherror-display-renders-float-noise]
---

(Filed by the ENCL implementer of `zero-arm-payload-wording-doubles-words-and-hedges-a-definite-verdict`, PR 5157, at the ENCL orchestrator's direction.)

## What

PR 5157 put one number format, `geom_core::Readable` (positional for `1e-4 ≤ |x| < 1e16` and zero, scientific outside), on two places:
- ENCL's refusal payloads, through `MarginDiag::readable()`.
- The numbers a sized ending and the import door print (geom-core `predicate.rs`: `Shows::ending`'s offer, `MarginDiag::sized_recourse_in_file`'s ε_in / near / offer, `FileCoincidence::miss_recourse_in_file`'s ε_in).

So one refusal sentence now prints one format. Two sibling renderings of the same readings still print `{:e}`, so a payload or ending in `Readable` can sit next to an escalation or a blend payload in exponent form ("0.0005 m" beside "5e-4"):

- **geom-core `predicate.rs`, `impl Display for IndeterminatePayload`** (SCALAR): every arm prints `{margin:e}` and the band's `({zero:e}, {escalate:e})`. Every undecided arm's payload goes through it.
- **sweep `blend/mod.rs`, `impl Display for ClassifiedMargin`** (owner: BAND / CARVE, per `work.py territory`): it prints `margin {reading:e} m` and `band ({zero:e}, {escalate:e})`. It does so on purpose, to match `IndeterminatePayload` (its doc comment cites the E3 review), so it moves with that payload, not on its own.

The bare-`{}` population is the separate row `work/comb/refusal-floats-outside-the-spline-stack-render-through-f64-display.md`, which also notes the `{:e}` sites as a second spelling.

## Ask

Decide whether `IndeterminatePayload` takes `Readable` (via `MarginDiag::readable()`, with the band thresholds through `Readable` too). Then move `ClassifiedMargin` in the same change, so the definite and undecided halves still read alike. Re-baseline the pins that move. Most band thresholds sit below `1e-4` and keep exponent form, so few renders should move.

## Evidence added 2026-10-19 (ENCL, PR 5157 delta review)

More `{:e}` sites print user-facing numbers outside `Readable`:
- `crates/geom-core/src/predicate.rs`, `impl fmt::Display for BandError`: `{value:e}` for `InvalidValue` and `InvalidLeverArm`, and `{zero:e}`/`{escalate:e}` for `Empty`.
- `crates/step-import/src/error.rs`: `{residual:e} m` (the ambient-tolerance refusal) and `{margin:e} m` (the ε_in ambiguity refusal).

PR 5157 moved the geom-brep payloads onto `MarginDiag::readable`. A margin of `0` therefore now prints "0 m" there, while these sites still print "0e0". (ENCL orchestrator)

## 2026-11-22: two more sites beside the told offer (PR 7267's review)

The told-quantity type (PR 7159) now spells a sized ending's offer as an offered bound (`geom-core` README, "A number told to a person"). Two sites still sit outside it:

- `predicate.rs` `impl Display for IndeterminatePayload` (the bullet above) prints `{margin:e}` in the same sentence as the told offer. The margin is not told as a computed number on the rule's grid, so a sentence can quote the margin at seventeen digits and its offer at four.
- `crates/step-import/src/interpret.rs` (about line 364) writes its own "tighten the tolerance below {} m" through `Readable(below)`, the exact spelling, and not through the offered role. Its offer is therefore not on the grid, and not cut to the side that works when typed. That file is EXCH/EXPORT/IMPORTGATE ground, so the fix is coordinated there.
