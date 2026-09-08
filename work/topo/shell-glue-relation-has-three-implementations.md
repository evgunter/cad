---
id: shell-glue-relation-has-three-implementations
kind: issue
title: the per-shell component glue relation is implemented three times, and the test-support copy is the one that drifts
status: open
opened: 2026-09-06
track: P
refs: [S69, 2014]
---

## What

One relation — *a face glues all its loops; a cycle loop glues across
each edge via `mate`; an empty-loop face is its own dartless
component* — is written out three times in `crates/topo/src`:

1. **`movefac.rs:74-118`**, the operator's plan phase. It labels
   components to decide the partition, and its labelling IS the
   partition: whatever it computes is what the shells become.
2. **`validate.rs:5225-5266`**, tier 1's pass 11, whose per-shell counts
   land in `Tier1Report.shell_components` and are reused by tier 2's
   `c = 1` check. `Tier1Report` is a private struct and the counts are
   not reachable from outside the validator.
3. **`seqgen.rs`'s `shell_components`**, added by `S69` (PR 2014) so the
   `movefac` catalog row can offer the two-component shells and so
   `fusion_remake_shell` can refuse a multi-component side.

Three copies of one definition is the ordinary duplication complaint.
The reason it is worth a row is the DIRECTION of the risk: (3) is test
support and it copies (1), the kernel. A generator that re-derives the
kernel's own decision procedure and then tests the kernel against it
cannot detect a defect in that procedure — if `movefac`'s labelling is
wrong, `movefac_candidates` is wrong the same way and the row keeps
choosing sites the operator agrees with. (2) is the independent
implementation and would catch it, which is why the tier-1 assert
inside `assert_euler_postcondition` is the check that actually holds
here, not the generator's site filter.

## What closing it looks like

The natural fix is one relation with the other two as thin callers, and
the shape of the API is the whole question:

- `movefac` needs the LABELS (which face in which component), in its own
  deterministic seed order, and it needs them from the plan phase before
  any mutation.
- pass 11 needs COUNTS per shell, and it runs under a gate (passes 1–7
  clean) that lets it assume things `seqgen` cannot.
- `seqgen` needs a count per shell, cheaply, on every step, with no
  allocation it can avoid — the cost measured in `S69` was ~7% of
  `choose_op` + `apply` for the two new rows together.

A single `pub(crate)` labelling routine returning the component map,
with counts derived from it, serves all three; the risk to weigh is
that pass 11 stops being an INDEPENDENT implementation of the relation
it validates, which is a real loss and may argue for keeping (2)
separate and unifying only (1) and (3).

Not done in `S69` (PR 2014): that unit's fence was the generator, the
ledger and one postcondition site, and moving a relation the operator
and the validator both depend on is not a change to make from a
generator branch.
