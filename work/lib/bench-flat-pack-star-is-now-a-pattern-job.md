---
id: bench-flat-pack-star-is-now-a-pattern-job
kind: issue
title: row 43's flat-pack star is a Python row to write, not a missing node
status: closed
opened: 2026-09-08
closed: 2026-09-08
refs: [LIB-BENCH-PATTERN]
---



Disclosed by LIB-B-PART, which changed the reason rather than the row.

## What changed

`docs/guide/north-star-audit.md`'s row 43 (`bench`) carries a `YES*`,
and the star's stated reason was that the flat-pack layout's posts are
`Node::Pattern` in the scene while Python says them as
`Node.placed_union`, "because `Pattern`'s value is a plural payload
that stays deliberately unbound". `Node.pattern` is bound now
(LIB-B-PART, PR 2163), so that reason is gone.

The star is NOT thereby cleared, and this row exists so the difference
is not read off the prose. This page's discipline is that a YES is a
scene the Python suite EXECUTES, and
`crates/pncad-py/tests/test_assembly_author.py`'s `TestBenchStand`
still authors the posts through `placed_union`. Clearing the star is
therefore a job:

1. author the layout's posts as `Node.pattern` in `TestBenchStand`;
2. assert the same product the row already holds it to — the material,
   the placement cluster, the gauge, the solved translation;
3. drop the star and the sentence about the substitution.

## The thing that has to be measured, not assumed

A pattern's value is PLURAL, and the flat-pack layout gathers a
PRODUCT over it. `editor_core::product` has an `Instances` arm
(`crates/editor-core/src/product.rs:304`), so the gather is expected to
take it — but the layout also carries mates over those posts, and
`crates/editor-core/src/mate/member.rs:389` walks `SlotId::Instance`
for a member, which is a different path from the one `placed_union`
takes. Whether the mates and the A5 gate answer the same way over a
plural payload is exactly what the Python row would establish, and it
is why LIB-B-PART did not flip the star by inspection.

Note also `work/eval/transform-refuses-a-patterns-instances-value.md`
and `work/msolve/nested-pattern-mate-heads-refuse.md`, which are the
kernel-side rows about what a pattern's value can and cannot feed.

## Closed (LIB-BENCH-PATTERN, 2026-09-08)

**Measured, not assumed.** `bench_scene.layout` grew a `posts=` switch
over the two node spellings — one count, one rule, one set of
constants — and `TestBenchLayout` runs its whole battery against BOTH.
Over the flat-pack the plural value answers exactly as the fused one
does: the gather takes the `Instances` arm and yields
`PATTERN_COUNT x post + shelf`, `select` answers PATTERN_COUNT
distinct instance-qualified cap names, each `face_frame` lands on the
placement ladder's rung, every `denotation` is untied at one
candidate, the tessellated outline is the same box, and `assemble`
passes the A5 gate OUTRIGHT with `minted == []`.

**Which expectations belong to which document.** This issue's step 1
named `TestBenchStand`; the pattern is in the LAYOUT, and the layout
declares nothing. The cluster, its gauge, the solved translation, the
two minted declarations and a CERTIFYING gate are the STAND's
expectations, and the stand carries no pattern — so they are
untouched by this unit. The layout's are the material, the placement
outline, the per-placement frames, the names and a gate that passes
with nothing minted.

**The one door where they differ, and it is not the gather.** This
file supposed "the layout also carries mates over those posts"; it
does not. Measured on a document authored for the question
(`test_a_mate_head_stands_on_a_patterned_copy_and_not_on_a_fused_one`):
a mate head on a PATTERN's copy resolves, solves `Determining`, joins
one cluster and mints — the `SlotId::Instance` walk this file cites —
while the same head on a `placed_union`'s copy refuses, because fusing
the family leaves no instance to be a member of. So the substitution
was not free, and the direction it cost in was the mate.

**Shipped:** the scene's layout is `Node.pattern`, the tour's own
node; `posts=` keeps the fused spelling reachable so the comparison
stays executed rather than remembered.

**Moved in the audit:** row 43 is `**YES**` with an empty gap column
and its substitution sentence replaced by what was measured; the
headline reads 34 outright + 3 YES\*; G8's stops column falls to
`degrades 3` and its LIB-G18b sentence is corrected; G18's residue
sentence says the row was written rather than that it is owed. In the
Python tree, `bench_scene`'s "deliberate difference" about the placed
family is deleted (the parametric prisms stay) and the tour guard's
blind spot (2) is one item.
