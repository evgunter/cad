---
name: Output stability is not a justification for keeping code
description: Byte- or bit-preservation may pick among equivalent implementations; it may never justify keeping a second implementation, a dead lane, or a worse shape
type: convention
---

# Don't write bad code to avoid changing outputs

**Ratified by Evan, 2026-08-18**, generalising the demo-scoped note in
`demo-purpose.md` ("byte-identity soft for improvements, kept for mechanical
migrations") to the whole repo, after the smell scan found it doing real
damage in kernel code.

## The rule

Byte- or bit-preservation of an **existing artifact** may justify a *choice
among equivalent implementations* — emission order, arithmetic association,
which of two correct spellings to use.

It may **never** justify:

- keeping a second implementation alive,
- keeping a dead or superseded lane,
- declining a better formulation,
- or shaping production code to match a test file.

If retiring or changing code would move committed bytes, say **what the bytes
are, whether they are regenerable, and what re-verification the regeneration
costs** — and put that sentence in the code, not only in a milestone log.

## Why this is a rule (the case that produced it)

`crates/sweep/src/fillet/build.rs:26` keeps an ~890-line second fillet
assembly implementation with the justification *"kept (not subsumed) so its M5
outputs stay bit-preserved."* On inspection:

- The surgery door's front door **strictly contains** the whole-body door's, so
  the older implementation handles a strict subset — and wins by being tried
  first.
- Eleven of its refusal messages are unreachable, because the caller discards
  the error to fall through.
- What the bytes actually cost is a **regeneration chore**: two goldens that
  their own docs say to regenerate deliberately, plus one FreeCAD acceptance
  run. Not a contract.
- The repo had **already formally recorded this claim as unfounded once** —
  the M6-5 PR-2 review's finding F-D rejected a test that claimed to pin the
  bit-preservation, and the fix pass wrote the correction into the test while
  leaving `build.rs:26` untouched.

## What this rule does NOT touch

The vocabulary is load-bearing elsewhere and must not be swept up:

1. **The D2/D9 determinism contract itself.** "Bit-identical replay",
   "byte-identical export", the interval bit-identity channel — these *are* the
   contract. Untouchable.
2. **Math-equivalence annotations.** "`powi(2)` is bit-identical to `a*a`",
   dot-product symmetry — these tell a reader a rewrite changed nothing.
3. **Regression scoping.** "An all-planar split is bit-identical to before this
   pass existed" bounds a change's blast radius. This is the *right* use and
   should be encouraged.

The suspect pattern is narrow: a repo-wide census found only **three** sites
using byte-preservation to *shape or keep* code, and only one of them
(`fillet/build.rs:26`) at any scale.

## The tell

If a comment says a thing is *"kept"*, *"not subsumed"*, or *"retained"* and
the reason given is that its output would otherwise change — ask what pins the
output, whether the pin is regenerable, and whether anything would actually
break. In the one case examined, the answer was: a golden, yes, and no.

## See also

- `docs/SMELL-SCAN-2026-08.md` S7 and its steelman — the full case and the
  repo-wide census.
- `memories/demo-purpose.md` — the demo-scoped predecessor, still correct in
  its own scope.
