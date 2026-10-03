---
id: degenerate-normal-rows-model-resolution-cites-a-deleted-helper
kind: issue
title: the degenerate-normal row's model resolution cites datums::unit, which no longer exists
status: open
opened: 2026-09-16
rides_with: degenerate-triangle-normal-is-substituted
priority: P4
cost: E
---


## Finding

`work/mesh/degenerate-triangle-normal-is-substituted.md`'s resolution 2
offers a model for the shape it wants:

> the doc should say which claim makes it dead, the way `datums::unit`'s
> fallback names `basis`' `1/√3` bound.

`crates/viewer/src/datums.rs`'s `unit` helper, its `+x` fallback and the
`1/√3` bound that argued the fallback unreachable are **deleted**:
`basis` is now `UnitVec3::orthonormal_basis`, which divides by no
length, so there is no fallback to argue about and no bound naming one.
VIEW closed `datums-basis-hand-rolls-the-least-aligned-axis-basis` and
`datums-unit-helper-normalizes-with-a-silent-x-fallback` together.

This is not a shifted line number — the subject is gone — so the
sentence cannot be repointed, and a MESH lane following it will look
for a pattern the tree no longer holds. Reported here rather than
edited into the parent row because that row is MESH's.

**What survives of the model**, if the sentence is wanted: the shape it
named — *a substituted value whose branch is argued dead by a claim
stated at the site* — is what VIEW decided AGAINST, on the ground that
prose-unreachable is the weakest kind of unreachable. That is an
argument for resolution 2's first half (state the guarantee in `mesh`'s
contract) and against its analogy.

**Where**: `work/mesh/degenerate-triangle-normal-is-substituted.md`,
the "Two candidate resolutions" section.

**Confidence**: sure.

## Re-homed at S-MESH's exit (2026-09-16)

Filed into `work/mesh/` by VIEW after the exit walk was cut; moved from `work/mesh/` to TESS (opened at this exit as S-MESH's successor for the tessellation kernel) beside the row it rides with, `degenerate-triangle-normal-is-substituted`, which moved in the same exit; the item's content, id and history are unchanged.
