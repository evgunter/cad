---
id: the-tours-post-cell-split-refuses-the-space-its-copies-read
kind: issue
title: the tour's second cut (the patterned-post cell) no longer splits: the cell's placements read a Space node outside it, so the gui lane drops a cell
status: review
opened: 2026-10-23
priority: P1
cost: M
refs: [5592]
pr: 5763
branch: intent/split-carries-spaces
---


## What

The nightly's `render lanes / viewer gui montage` is red on main
`be9b907694` (run 39906156937): the render did not produce
`demos/renders-gui/instantiatepart-part2-pattern-place2.png`, and
`instantiatepart2-part2-pattern-place2.png` came back as
`instantiatepart2-part2-pattern-place2-space.png`. The lane refuses to
commit a dropped scene, rightly.

The cause is in the tour, not the lane. `demos/tour/src/assembly.rs`
`refactorings` makes a second cut, the patterned-post cell (the post
instance, the pattern, its `Part` copies and the placements reading
them), and stores the part it splits out. On the 2026-10-22 nightly's
scene-inputs job (run 39728496598) that printed

    second cut: the patterned-post cell splits out AND inlines back

and on 2026-10-23 (run 39906156937) it prints

    second cut (gap): the patterned-post cell does not split at all —
    PartEdit { error: UnresolvedInput { input: SpokenNode { id: … ordinal: 1 …,
    kind: Some("Space"), label: None } } }

so the posts-cell document (`88b4110c…`) is no longer written, and the
store holds six documents where it held seven. The layout gained a
`Space` node, which is where the renamed cell's `-space` comes from.

Suspected merge: #5592 (D′1, `Node::Space` and the placement's target
slot), merged between the two runs; #5607 (Mirror) also touched
`refactor.rs` and the tour in that window.

## Owed

Decide what a split does with a `Space` whose readers are all in the cut
and which the selection does not name: carry it into the part, refuse
it typed with that recourse, or have the tour's cell name it. That is
a choice of what `split` means for D′1's spaces (`design: true`).
Then make the second cut split again, and re-render the gui lane with
`[render]`, deleting whichever cell names really retire.

— INTENT nightly triage, 2026-10-23

## Evidence from D′2's fix pass (PR 5613)

The gui render lane ran on PR 5613's branch for the first time and its
re-baseline refused: two committed cells were no longer produced.
- `demos/renders-gui/instantiatepart-part2-pattern-place2.png` is the
  posts cell this refusal stops the tour writing.
- `demos/renders-gui/instantiatepart2-part2-pattern-place2.png` is the
  layout document, now named `instantiatepart2-part2-pattern-place2-space`
  since D′1's `new_space()` master.

PR 5613 retired both cells (`git rm`, `[render]`); the lane then
re-baselined, the layout under its `-space` name. Once the second cut
splits again, its posts cell comes back under whatever name its shape
then has.

## Decided (orchestrator, 2026-10-26; designer pair concurred, no ratified text changes)

Designers A (Opus) and B (Fable), byte 234 (Opus is A), recorded on `analysis/design-fork/intent-tour-post-cell-split`. They agreed on split's behaviour from round 1. In round 2, A moved to B's representation. The ruling:

- **The refusal is a hole, not a decision.** `Doc::read_ports` answers nothing for a `Space` read, so split's closure check never sees a placement's link to its target. The carry then hits an unmapped `Node::Space` and surfaces `PartEdit { UnresolvedInput }`. B's probe on main also showed:
  - a cut naming the space but minted after its reader still refuses;
  - a cut space with a kept reader strands silently (`StrandedRead`);
  - the same holes exist for a construction's own `a.space`, in both directions.
- **`Node::Space` stays a node.** A space read by two placements is an output spoken by its node, so it needs no name. Making `new_space()` a variable would put two test sites and every multi-placement inline under the names rule (VR7), which needs a DESIGN exemption. That would be its own `[ev]` fork, and neither designer recommends it.
- **Split's rule.** Every `Space`-kind operand read is an edge to the node that defines the space. A `new_space()` takes the variable regime:
  - every reader cut: it joins the cut, whether named or not;
  - readers on both sides: it refuses, typed, naming one reader on each side;
  - no readers: it stays;
  - the world is exempt, as today.

  A construction's space is its output and goes with it, so a crossing refuses `SeveredEdge` naming the construction. The carry deals `Node::Space` nodes before their readers. The rule is stated in `refactor.rs`'s module docs ("Variables go where their readers go" gains spaces). ASSEMBLY A4 is not changed: its closure rule still holds.
- **Tests.**
  - The four `new_space()` cases: named and unnamed, both mint orders.
  - A shared space refuses.
  - The two construction-space cases.
  - The tour's second cut splits and inlines back; its "gap" print becomes an assertion.
  - `p2_face::a_mate_reading_a_copy_in_a_space_of_its_own_refuses_at_split` keeps its assertion.
  - The gui lane re-renders the posts cell with `[render]`.
- **Check on D′3's branch:** whether inline then flatten leaves the part's product `⊔` as an extra node. That would break A4's round trip by one node. File it if it holds.

## Handoff

PR 5763. Split reads every `Space`-kind operand as an edge to the node
defining the space (`refactor.rs` `spaces_read`), and a `new_space()`
takes the variable regime: all readers cut joins the cut, named or
not; readers on both sides refuses `UncutVarReference` naming the
space's output and one reader each side, after the closure check (so
`p2_face`'s `SeveredEdge` still speaks first); an unread one stays
unless named. A placement across the cut from a construction's own
space is `SeveredEdge` naming the construction. `carry` deals
`Node::Space` nodes first, for split and inline alike. A space named in
the cut under a kept reader now refuses `SeveredEdge` instead of
stranding. Rows: `tests/split_carries_spaces.rs`. The tour's second
cut asserts split and inline; the posts cell re-rendered under
`[render]`.
