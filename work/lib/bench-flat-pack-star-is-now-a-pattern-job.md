---
id: bench-flat-pack-star-is-now-a-pattern-job
kind: issue
title: row 43's flat-pack star is a Python row to write, not a missing node
status: open
opened: 2026-09-08
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
