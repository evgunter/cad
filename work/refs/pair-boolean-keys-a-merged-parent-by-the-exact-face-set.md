---
id: pair-boolean-keys-a-merged-parent-by-the-exact-face-set
kind: issue
title: The pair boolean keys a merged parent by its exact operand-face set, where the union's link is transitive
status: closed
opened: 2026-11-02
priority: P3
cost: M
closed: 2026-11-03
pr: 6381
---


## What

`name_boolean` (`crates/editor-core/src/names/emit_topo.rs`) keys a
merged parent by the exact set of operand faces its merge lists
(`parent_at`, "two merges that list the same operand faces hold one
parent between them"). Two merges whose sets overlap without being
equal, {P0, P1} and {P1, P2}, are two parents that share P1. The
union's link is transitive (N2 in `crates/editor-core/src/names/README.md`;
`emit_union`'s `Parents::of`), so the same two merges in a union are
one parent `Merged{P0, P1, P2}`: a subtract and a union of one geometry
would name the shared constituent's faces differently, and a seam
beside P1 would have two parents to cite.

Reachability is unverified. A candidate: a boss whose wall is the three
pieces of `circle_split(.., 3, ..)`, cut by two slots so one merge glues
pieces 0 and 1 on one side and another glues 1 and 2 on the other. The
first job is a probe that builds it as a subtract and reads whether two
merge groups share an operand face; if none can, say why and close.

**Two more findings (2026-11-02, the merged-parent fork's designers):**
- The subtract also publishes an operand face's unmerged pieces as that face's own `Borders` pieces when the same face survives inside a merged face. That is `name_boolean`'s `groups` and `in_merged` arm. So a constituent publishes beside its merge, and N3's "constituents retire" holds for the union only. The witness to build: a sunk boss minus a narrow chord slot near the boss's rim.
- If Ev takes per-face exact (PR 6342; unit `a-merged-face-is-named-for-exactly-the-member-faces-it-holds`), this row's exact-set keying is the intended rule, and the row closes as not a defect.

## Closed (2026-11-03, PR 6381): not a defect

Ev took per-face exact (PR 6381). A face is named for exactly the
operand faces it holds, so keying by the exact set is the rule, and the
union now does the same (`name_held_faces`, which both emitters call).
The constituent beside its merge takes `Borders` like the merge: both
hold area of one operand face.
