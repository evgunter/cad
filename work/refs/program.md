---
id: refs
kind: program
title: REFS — references and reads: a name scoped by its read, indexed reads, and the façade's slot words
status: ready
opened: 2026-10-11
group: api/names
prefix: reads/
tag: (REFS orchestrator)
ab_band: 13100-13199
paths: [crates/editor-core/src/names/*, crates/editor-core/src/names/geompred.rs, crates/editor-core/tests/name_invariance.rs]
keep_out: [opened 2026-10-11 by INTENT's split (Ev approved the division in chat 2026-10-11) - the rows moved by git mv with ids and bodies unchanged, DM4 unit 2 (PR 4674) was dispatched by INTENT and its lane reports to the INTENT orchestrator until it merges, its merge waits on Ev's answer on PR 4691 whose ruling item a-reference-whose-read-is-gone lands in work/intent and moves here after, a row that needs patterns-are-index-variables stays in INTENT (a-split-name-picks-a-pattern-copy-by-its-evaluated-index did), the ground is shared with DOCTAIL (doc.rs, edit.rs, persist), BIND and LIB (pncad-py), TALLY (eval/select.rs) and the viewer programs, so run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, WIRETAIL merged in on 2026-11-20 (docs/doc-ledger/wiretail-and-namecheck-merge-into-refs.md) - it held no P0 or P1 row, from WIRETAIL: opened 2026-10-17 by WIRE's priority-seam cut - the rows moved by git mv with ids and bodies unchanged, from WIRETAIL: WIRE stays open with its P0-P2 slate, from WIRETAIL: crates/editor-core/src/eval/* names/* and placement.rs are shared ground with WIRE and WIREHOLD, from WIRETAIL: node-error-kind-has-no-fieldless-projection's three consumer doors (drive.rs mc.rs stackup.rs) are PROPS's ground and its declaration is here, from WIRETAIL: split-half-side-words reaches viewer/src/tree.rs, from WIRETAIL: run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, NAMECHECK merged in on 2026-11-20 (docs/doc-ledger/wiretail-and-namecheck-merge-into-refs.md) - it held no P0 or P1 row, from NAMECHECK: opened 2026-10-20 by WIRE's cut (WIRE measured 69 budget points against 30 - work/README.md Track size) - the rows moved by git mv with ids and bodies unchanged and WIRE kept its P2 naming-defect rows, from NAMECHECK: crates/editor-core/src/names/* is shared ground with WIRE WIREREST and WIRETAIL, from NAMECHECK: names/role.rs is DOCM's by its table and the form map lands beside it by announced seam, from NAMECHECK: the arena-permuting helper lands in topo behind test-support (TOPO's ground - announce it), from NAMECHECK: run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---

D10's references: a name is scoped by the variable that holds its body
(FORK-DM4), a read is the only dependency, and a selection reads a body
by its read. This program owns DM4 unit 2 and the rows on reads and
their façade: indexed reads (rank two, a selection's indexed body,
re-pointing an index from Python), `SetExpression` still addressed by
slot, the slot-form wrappers, Python's slot words, the viewer's boolean
tool picking a read, and the GUI's typed-value minting (in review).

## From WIRETAIL (merged 2026-11-20)

WIRETAIL — WIRE's P3 and P4 tail: the selection vocabulary's missing atoms, label and kind-mirror hygiene, and at-rest gating repeated per consumer

WIRE's P3 and P4 rows, cut out when WIRE measured 48 live budget
points against 30. Two of them are one design weighing
(`selector-has-no-sharp-edge-atom-so-a-kind-pair-takes-tangent-seams`
and `adjacent-kinds-cannot-tell-a-crease-from-a-co-surface-seam`, both
GS-Q2's missing atoms); the rest are execution.

Slate, order and posture: `work/refs/plan.md`; narrative in
`docs/doc-ledger/wiretail-and-namecheck-merge-into-refs.md`.

## From NAMECHECK (merged 2026-11-20)

NAMECHECK — N0's checks: the denotation verifier, the coverage fuzzer, the RoleSeg form map, the storage-order perturbation, and the clauses that point at N0

**The checks N0 names** (PR 5455, `crates/editor-core/src/names/README.md`
N0). Each N0 principle is stated with the check that enforces it. Today
only NS1's perturbation harness exists (`tests/name_invariance.rs`, PR
5460). The rest are here:
- NS2's denotation verifier and its exhaustive `RoleSeg`-to-form map;
- NS4's coverage fuzzer;
- the eighth perturbation (storage order), which needs a topo helper;
- the pointer rewrite of the clauses N0 now states once.

These rows sit apart from WIRE's because they are checks, not naming
defects: none of them changes a name.

Slate, order and posture: `work/refs/plan.md`; narrative in
`docs/doc-ledger/wiretail-and-namecheck-merge-into-refs.md`.
