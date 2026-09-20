---
id: review-m1-pr4s-verbatim-sentence-is-one-edit-less-true
kind: issue
title: review_m1_pr4.rs says its probes are otherwise verbatim, and the promotion exception list no longer covers what has been changed
status: open
opened: 2026-09-20
refs: [the-pr-17-promotion-attribution-is-half-checked, the-guarded-shell-list-of-a-solid-is-spelled-thirteen-times]
---

## Finding

- **Where**: `crates/topo/src/review_m1_pr4.rs`, the module header's
  last line.
- **Importance**: low
- **Confidence**: sure.
- **Raised by**: the `Body::shells_of_solid` fold, 2026-09-20.

The header closes: *"Only lint fixes and the promotion of the
corrected re-make taxonomy were applied at promotion; the probes are
otherwise verbatim."* It is a **closed list of exceptions to a
verbatim claim**, and the list has not been kept.

`Body::shells_of_solid`'s fold rewrote four lines inside
`seqgen_kvfs_availability_instrumented` — a solid-arity predicate in
an instrumentation probe — which is neither a lint fix nor the re-make
taxonomy. The fold itself is **in bounds**: the same header's
no-simplify sentence scopes its protection to *the derivations*
(splice geometry, the single-op re-make taxonomy, the shell-component
formula, orbit orders), and an availability instrument is none of
those. What is now untrue is the sentence's own word, *verbatim*.

## Why it is its own row

`work/dup/the-withdrawn-never-simplify-rule-still-stands-in-seventeen-files.md`
lists this file among the three whose no-simplify sentence says
*"…to match the implementation"* and leaves them unexamined — but that
row is **closed** (PR #2886), and a closed row is a record, not a
slate. `the-pr-17-promotion-attribution-is-half-checked` covers seven
headers and this file is not one of them: it carries no PR #17
attribution. So the finding has no existing home.

**This is not a request to edit the attribution or the no-simplify
sentence**, both of which wait on Ev. It is about the third sentence,
which is a factual claim about what the file contains and which any
lane can check. The two dispositions are: re-word it to say what it
means (the DERIVATIONS are verbatim), or keep the exception list and
add to it at each edit — the second being what this file has been
doing and losing.

## Not measured

How many edits the file has taken since promotion, and whether the
other two files carrying the same sentence shape have the same drift,
is unmeasured. One is a floor.
