---
id: refused-mate-badges-every-instance-row
kind: issue
title: A refused mate solve badges every instance row — the offending mate should be the loudest badge
status: closed
opened: 2026-09-01
github: 1463
refs: [1461]
branch: chrome/refused-mate-badges-every-instance-row
pr: 1769
closed: 2026-09-04
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

**Also changed, and not only for mates**: the badge WEIGHTS. `FAILED`
takes `theme.unresolved`; `POISONED` draws quiet, where before both
took the colour — so ordinary DAG poisoning, which this item is not
about, reads differently too. That is the intent (the row to act on is
the loud one), and it is the reason `crates/viewer/src/theme.rs`'s
`unresolved` doc no longer lists a poisoned badge.

## Fixed, second pass (the style review's 18 findings)

**The prose half of the filed defect, which the first pass left
standing.** The issue asked for two things, and only the badge half was
done: every reached row still printed the cause's FULL refusal text, so
a user still met four identical paragraphs. A downstream row now
carries `tree::downstream_wording` — "upstream failure at node N — that
row carries the cause" — and nothing else, and in the app that line is
a LINK that selects the row it names. The full refusal is read once, on
the row that owns it. Honest for both producers of `Poisoned` because
both now point at a row the tree itself badges `FAILED`; the module
header states the one carve-out this buys — the words about WHERE a
failure is are this module's, the words about WHAT went wrong stay the
payload's.

**`Poisoned`'s invariant, on its other producer.** The first pass
strengthened `through` on the mate branch only, and the two-hop chain
the review suspected is REACHABLE: a boolean over two instances of a
cluster that then refuses is poisoned by the kernel through an
instance, whose own row the tree redraws as downstream of the mate — so
the boolean pointed at a row drawn POISONED and weak, and recited that
instance's own copy of the fault ("node 0 failed …") while node 0's row
denied owning it. `tree::poisoned_through` now re-attributes the DAG
branch through the same rule, one step (a mate a fault names keeps its
own `Failed`, so there is no third hop). Pinned by
`tree_badges::a_boolean_over_a_refused_clusters_instances_points_at_the_mate`,
which also asserts the tree-wide invariant: every POISONED row names a
row this tree badges FAILED.

**Comments that were false in shipped code** are corrected: the
"nothing ran here" reading of a poisoned row, the "only row that takes
the colour" claim (a `Contradictory` naming two mates reddens both),
`theme.rs`'s poisoned-badge listing, the module header's universal
"every `MateFault` arm names its subject" (`Band` names none), the
corroboration guard's two-cases-handled reading (mates are DAG leaves,
so `Ok` is the only reading it guards), and an unenforced `held < added`
document-order claim, now dropped rather than asserted.

**Test mechanism, announced** (`keep_out`): `common::status_of` and
`common::asm::seat_alignment` are lifted into the shared test tree,
replacing five hand-written row lookups and three copies of the seat
frame ladder. The contradiction row now ASSERTS its premise — that the
run reports the first mate `Ok` — instead of resting on it silently, so
fixing `mate-memo-key-does-not-carry-the-solve` turns that row red
rather than leaving it a duplicate of the row above.

**Judged, not fixed.** Badge colour and weight remain unseen by any
test, and the weight half is outside every palette;
`work/chrome/chrome-weight-is-outside-the-palette.md` carries it,
because the question is whether a semantic distinction may be drawn in
weight at all rather than one missing assertion.

`crates/viewer/README.md`'s G4 and GQ2 clauses said what the tree no
longer means; both are corrected under a narrow `paths` amendment
recorded in `work/chrome/program.md`.
