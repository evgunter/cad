---
id: viewer-free-move-ignores-pattern-headed-mates
kind: issue
title: viewer free-moves an instance a pattern-headed mate constrains: mates_naming compares name.node to the instance, and a pattern head's node is the Pattern
status: open
opened: 2026-09-04
---


Found by PR 1749's review (FIX, `split-crossings-skip-pattern-mate-ends`)
while testing that unit's own disclosed sweep blind spot. Filed by the
FIX orchestrator rather than by the lane, and homed here rather than on
a program's slate: the sibling issue
`work/chrome/viewer-mate-tool-refuses-pattern-picks.md` (1412) is
CHROME's, and this is a different door — re-home it by header edit if
CHROME claims it.

## The defect

`crates/viewer/src/display.rs:186`, `mates_naming`:

```rust
Some(Node::Mate { a, b, .. }) => a.node == instance || b.node == instance
```

A mate reference's `node` is the node the name's head **spells**. Since
MATE-1 landed the A11 member vocabulary (PR 1400), a mate may head a
pattern-placed instance — `Pattern` node plus an `Instance(i)`
qualifier — and for such a reference `a.node` is the **`Pattern`**, not
the instance. So `mates_naming(doc, leg)` comes back EMPTY for a mate
that `mate::reading_edges` says reads at `leg`.

That divergence is executed, against PR 1749's own row: the unit's test
asserts `reading_edges == [(mate, leg), (mate, top)]` while the name's
head is the pattern node.

## The consequence

`crates/viewer/src/display.rs:337-347`. `free_move_check(leg)` consults
`mates_naming`, finds nothing, and returns `Ok` — so **the viewer
permits a free move of an instance that a pattern-headed mate
constrains**, silently invalidating the solve. The complementary door
does not catch it either: `free_move_check(pattern)` refuses
`NotAnInstance`, because a pattern node is not an instance.

Neither door refuses, and the two doors refuse for different reasons,
which is why no single reading of either one shows the hole.

## What this is NOT

**Not issue 1412.** That covers `matetool.rs:417` / `is_instance` — the
*pick* gate, which excludes the very heads the rider admits. This is
`free_move_check`, a different door with a different failure: 1412
refuses something it should allow; this allows something it should
refuse. 1412's fix does not touch `mates_naming`.

## The fix shape

Ask the member vocabulary rather than comparing node ids. PR 1749 made
`editor_core::mate::member_of_head` public for exactly this reason and
collapsed three spellings of the vocabulary onto it; this is the fourth,
and it is the one that was live. A `mates_naming` that resolves each
reference's head through `member_of_head` and compares MEMBERS answers
the question the door is actually asking.

## Reachability

The reviewer read this rather than executing it (a viewer build was
unaffordable at 2.4 GB free). Its confidence: `sure` on the code
reading, `likely` on GUI reachability — issue 1412 records that
pattern-headed mates are authorable through the recipe and Python
doors, so a document carrying one can be opened in the viewer. **An
executed row is one `free_move_check` call away** and is what this issue
wants first.

## The class

The unit that found this collapsed three spellings of "is this name's
head a member?" onto one predicate. This is the fourth, invisible to a
textual `Node::InstantiatePart` sweep because the line mentions neither
`InstantiatePart` nor `Pattern`. Where to look next, per the reviewer:
**any site comparing `name.node` to an instance id.**
