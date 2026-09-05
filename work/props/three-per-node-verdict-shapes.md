---
id: three-per-node-verdict-shapes
kind: issue
title: Three shapes for per-node verdicts: consolidate or record the split deliberately
status: review
opened: 2026-08-29
github: 1255
refs: [1231]
pr: 1920
branch: props/verdict-shapes
---

## From GitHub issue 1255

Opened 2026-08-29; 0 comments.

## What

The tree now carries three shapes for "the verdicts a node recorded", and they should be reviewed together even though at least two of them are load-bearing as they stand.

| shape | home | form | job |
|---|---|---|---|
| `NodeValue::verdicts` (`Arc<VerdictLog>`) | `eval/mod.rs` | the raw ordered `Vec<Verdict>` a node's op produced | the substrate both others read |
| `NodeVerdicts` / `VerdictSummary` | `resolve/vdiff.rs` | per-predicate sign POPULATIONS, serializable | permutation-invariant delta between two runs; the ε-audit's cross-process form |
| `VerdictVector` / `VerdictRow` | `drive.rs` (M10-3) | ordered rows + a per-node outcome tag, hashable to a `VerdictVectorKey` | EXACT identity: "is this leaf the witness build" |

## Why it is not obviously one shape

The two derived forms answer genuinely different questions and the difference is argued at `drive::VerdictVector`'s docs:

- Certification wants the STRICTEST test available, because a false yes is a false certificate — order included, outcome tags included, no cancellation.
- Flip NAMING wants a test that survives permutation, because construction order inside an op is itself predicate-steered (`vdiff`'s module docs). Populations cancel a pure sign exchange within one node — deliberately weaker, so it can name but must not gate.

Neither subsumes the other, so a naive merge would either weaken certification or make the flip report positional again (which `vdiff` rules out).

## What is worth deciding anyway

1. Whether `VerdictVector` belongs in `resolve/vdiff` beside the engine it is the strict counterpart of, rather than in `drive` — the "built once" principle is about the ENGINE, but two verdict shapes living in two modules is how a third gets minted.
2. Whether the outcome tag (`ReplayOutcome`) should be folded into `vdiff`'s `RunStatus`, which is the same information under a different name.
3. Whether `VerdictSummary`'s serializable population form should be the only persisted shape (it already is) and said so at the other two.

## Not urgent

Nothing is wrong today: each shape has a stated job and the strict/permutation-invariant split is deliberate. This is a consolidation review, filed so the split is a decision on the record rather than an accumulation.

Raised by M10-3's review (PR #1231) and referenced from `drive::VerdictVector`'s docs.

## Home

`work/props/`, re-homed from `work/m10/` at that program's residue sweep (`work/m10/log.md`, "Seam — residue re-homed for the exit"). `drive.rs` stays an M10 territory glob, so the unit edits it by the announced seam this program's `keep_out` names.

## Decided

All three questions answered yes, with one shape kept:

1. `VerdictVector`, `VerdictRow` and `VerdictVectorKey` live in `crates/editor-core/src/resolve/vdiff.rs`, beside `NodeVerdicts`/`VerdictSummary` — two derived forms over one substrate in one module. `VerdictVector::certifying` stays written in `drive.rs` as an inherent method, because WHICH ROWS A GATE EXCLUDES is driver policy.
2. `ReplayOutcome` is retired; `VerdictRow::outcome` is `RunStatus`, so an absent node reads `Absent` rather than folding into `Poisoned`.
3. `VerdictSummary` is named as the only persisted shape at `NodeValue::verdicts`, at `VerdictVector` and in `vdiff`'s module docs.

The strict/permutation-invariant split STAYS, and is now pinned executably in `crates/editor-core/tests/props_verdict_shapes.rs`.
