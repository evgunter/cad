# ASM-UPD — the pin-update door (binding spec)

Binds **A13** (update granularity, ratified #544; all four
clauses), A4 (pins move only by recorded edits), A2 (memo keying),
D9. Pre-logged **M / STRUCTURAL** (edit vocabulary + elaboration +
a multiplicity lint; no numeric decisions). No dispatch gate.
Deviations reported, never absorbed.

## D-1: the primitive

`DocEdit::UpdateReference { node, new_pin }` — recorded, undoable
(prior pin kept), replayable; refusals typed and naming their
subject: non-`InstantiatePart` target (the `PlacementOnNonInstance`
precedent), unknown node, and same-pin no-op (fail-loud: a
semantically empty edit refuses rather than recording — report if
you find a reason to prefer recording). The edit is recipe data:
`new_pin` is NOT resolved at edit time — a wrong pin surfaces at
evaluation through the existing typed seam (`PinMismatch` /
`Unresolved`), which already names both pins. The edit's doc
comment states the A13 clause-4 contract verbatim (re-evaluation
now; mate re-verification when R2-b lands — cite A13, do not
implement R2-b machinery).

## D-2: the elaboration

Document-layer `update_references(doc, id, new_pin)` → one
`UpdateReference` per matching instantiate node, returned as an
ordinary edit list (purity = atomicity, the ASM-4 precedent;
empty match refuses typed naming the id). Workspace-layer
convenience `update_to_store(doc, id, workspace)` computes
`new_pin` from the store's current content (resolves through the
existing read side; store misses refuse through the existing
typed vocabulary).

## D-3: the mixed-pin lint

An expectation-check in the A5 connectedness-lint mold (report,
never a gate): for each referenced document id with pin
multiplicity > 1, one entry listing the id, each pin, and the
referencing node ids. A clean document reports empty. Where the
connectedness lint lives is the placement precedent — same home,
same non-gating discipline.

## D-4: evaluation and identity

A pin move re-keys the part memo automatically (the memo key IS
(DocRef, ε) — evidence required: after an update, the old
content is not served; nested case included). The ASSEMBLY's own
A4 pin moves on update (canonical bytes include node data —
assert both directions: update moves it, unrelated edits do not
move it differently than before). D9: fresh-process byte
identity of the updated document's save and evaluation.

## Acceptance rows

1. The primitive: records, undoes exactly, replays; names its
   node; same-pin and non-instance and unknown-node refusals
   each constructed, message naming its subject.
2. Elaboration: a two-instance + one-other-id document updates
   both matching sites in one returned list; empty-match refusal.
3. Store convenience: resave part → update_to_store picks up the
   new pin; evaluation sees the new geometry through the real
   workspace (e2e, the asm4 store-test mold).
4. Mixed-pin lint: staged state (one of two updated) reports the
   multiplicity with both nodes; uniform state reports empty.
5. Memo re-key evidence: update → re-evaluate serves the NEW
   content (probe geometry changed at the part level), including
   a nested (sub-assembly) case.
6. Assembly pin moves on update; D9 fresh-process byte identity
   post-update.
7. Round-trip persistence of a document carrying an update edit
   in its log (undo across load restores the prior pin).
8. Cold clippy: CI scope + interval + pncad-py python lanes.
   k-lint fires → report, never silence.

## Standing brief lines

As ASM-4-SPEC's, verbatim (OUTPUT DISCIPLINE; foreground rows;
poll harness-backgrounded output files; kill by recorded PID only;
local-scripts/ tooling; merge-before-open + re-merge on movement +
confirm checks START; invariant comments; commit+push per unit;
PR bodies from lane-private paths, never the shared scratchpad).
