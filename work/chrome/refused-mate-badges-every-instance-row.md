---
id: refused-mate-badges-every-instance-row
kind: issue
title: A refused mate solve badges every instance row — the offending mate should be the loudest badge
status: review
opened: 2026-09-01
github: 1463
refs: [1461]
branch: chrome/refused-mate-badges-every-instance-row
---

## From GitHub issue 1463

Opened 2026-09-01; 0 comments.

Found by the `story_assembly` integration lane. When a mate solve refuses (repro: the `mate_clocking_redundant` refusal of issue 1461), the failure badges land on the **instance** rows — including instances the offending mate does not touch: in the observed document all four instance rows read "node 0 failed: the mate solve refused …", while the one mate that caused the refusal is not where the eye is sent.

The GQ2 contract (a failure poisons its descendants, independent subgraphs complete) makes the propagation itself defensible — the solve is one shared computation over the placement graph — but the *attribution* is inverted for the user: the actionable row is the mate whose rider is contradictory, and the message on every other row should read as downstream poisoning ("upstream mate refused"), not as that row's own failure. As shipped, a user with four identical "failed" badges has to read the refusal prose on some unrelated instance to find the mate to fix.

(story-suites orchestrator)

## Home

`work/issues/` — the tree-badge attribution is viewer chrome under the GQ2 contract; GUI and GAUTH are closed and no open program's territory covers `crates/viewer`.

## Fixed (CHROME, 2026-09-04) — in scope, and the item's mechanism was wrong

**The scope question first**, because `program.md`'s `keep_out` said a
badge fix that must thread provenance through the solve becomes a
CURVED rider. It does not have to: **the mate is already nameable from
what the viewer receives.** Every `MateFault` arm but one carries the
offending mate's `RecipeNodeId` in a public field, and `pncad`'s façade
re-exports both `NodeErrorKind` and `MateFault`. No `editor-core` file
is touched and no file outside `crates/viewer/` is touched. The
exception is `MateFault::Band`, which names no mate — correctly, since
a tolerance cause blames no one mate — and rows it reaches keep their
own `Failed`.

**The item's mechanism was wrong, though its conclusion stands.** This
is not GQ2 poisoning: mates and instances are DAG **leaves** (a mate's
references are names, not edges), so no `NodeResult::Poisoned` is ever
produced here and each of those instance rows is its own `Failed`. The
fan-out is the SOLVE's own — it records one fault against every
instance in the cluster and every mate holding it — which is why the
fix had to redefine `Poisoned` rather than merely re-route to it.
`Poisoned` now means "the failure this row shows is not its own", which
covers both the DAG chain and the solve's fan-out; `through`'s
invariant is strengthened, not weakened.

Also corrected: the item reports four rows reading *"node 0 failed"*.
`NodeError`'s `Display` writes its OWN node, so each row named itself;
the `0` is almost certainly from the fault text. The substance — four
badges of identical kind and identical prose, pointing nowhere — is
exactly as filed.

**The hazard that forced a second commit.** A mate a fault names can
read `Ok` in the very evaluation carrying that fault, because a mate's
memo key does not carry the solve. Taking the first blamed mate
unconditionally would have pointed every instance at a GREEN row and
dropped the message — strictly worse than shipped. Blame is therefore
corroborated against the evaluation: the first blamed mate the run
agrees is `Failed`, else the row keeps its own. That guard is a
viewer-side workaround for a kernel inconsistency, filed as
`work/issues/mate-memo-key-does-not-carry-the-solve`.

Three rows, each verified RED with the attribution disabled, including
a windmill story stage where the user clocks the hub and the tree sends
the eye to the rider rather than to four identical instance badges.
