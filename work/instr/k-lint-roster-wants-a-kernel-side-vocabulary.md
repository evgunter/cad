---
id: k-lint-roster-wants-a-kernel-side-vocabulary
kind: issue
title: k-lint's roster pin parses kernel source because the fence forbade the two cheaper shapes
status: open
opened: 2026-09-07
---


## What

`tools/k-lint/tests/predicate_roster.rs` pins `EPS_COUPLED_PREDICATES`
against the kernel by **parsing Rust source text**: `include_str!` over
`crates/geom-brep/src/props/quad.rs`, two blanked views from
`test_utils::source`, a bracket walk to find each
`classify_len::<T>(` call, and a comma split to read its first two
arguments. That is roughly 200 lines of machinery policing a
one-element array.

Two cheaper shapes exist and neither was available to the unit that
wrote it:

- **a `#[test]` inside `geom-brep`** asserting the crate's own minted
  predicate names against a list, which the roster then reads. No lexer,
  no cross-root text read, and the assertion sits where the names are.
- **a `const` slice exported from `geom_core::k_stats`** — the
  vocabulary as data rather than as `&'static str` arguments — which
  `k-lint` already dev-depends on and could simply import, the way
  `tests/outcome_vocabulary.rs` imports `SampleOutcome`. The roster
  then becomes a subset assertion over an imported set, with nothing to
  parse and the ADDED direction checkable for the first time.

Both are edits under `crates/`. METER's fence covers `tools/k-lint`,
`tools/tess-meter` and `docs/K-REPORT.md`; `crates/geom-brep/src/props/*`
and `crates/geom-core/src/k_stats.rs` are PROPS' seam. **The fence, not
the problem, chose the instrument** — which is worth writing down
because a future reader finds a hand-rolled parser and no statement
that it was second choice.

## Finding

The second shape is strictly the better one and it composes with
`k-lint-eps-coupled-criterion-unwritten`: a kernel-side declaration of
the vocabulary is the natural place to also declare *which* of those
names are metered against an ε-derived target, which is the criterion
that item says nothing in the tree states. One PROPS unit could land
both, and the roster pin would shrink to an import and two set
comparisons.

Until then the parser stays, because a roster with no pin at all was
the defect `k-lint-predicate-roster-unpinned` filed, and a textual pin
that reds loudly on every spelling change is a worse instrument than an
import and a better one than nothing.

**Confidence:** sure that both shapes are outside METER's fence and
that neither exists today; likely that the `const` slice is the shape a
PROPS unit would actually pick, unsure whether `k_stats`' `&'static str`
signature is load-bearing somewhere that makes a slice awkward.

## Was

Raised by the style review of `meter/klint-roster-pin` (NOTE-4 and
NOTE-6, `sure`), which observed that a reviewer would not have written
a Rust parser to police a one-element array and named both alternatives.
Recorded rather than acted on by that PR's fix pass, per the fence.

## Moved to INSTR (2026-09-08)

Moved from `work/meter/` to `work/instr/` by `git mv` when METER's exit
walk opened the successor (`docs/METER-EXIT-WALK.md` §4, ratified by Ev on
2026-09-08 at PR #2212). Id, header and body unchanged; the directory is
the claim. This row is one of the twenty on INSTR's opening slate.
