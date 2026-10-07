# FORK-2b — what the product is, when no node replaces another

## For Ev

**Recommendation (likely): the product is the world's space, and nothing else states it.** A body appears exactly when a copy of it is placed relative to the world. No edit moves anything into or out of the world as a side effect: a boolean is a further body, and its operands do not appear because nobody placed them in the world. The world placement replaces the explicit list. It is one statement that says both *that* a body is delivered and *where* export puts it, where the list would need a second statement for the coordinates.

**Terms.**
- *World placement*: a placement whose other side is the world frame. In its final form it is a stage-3 placement (a bundle of mates) like any other. Until then it is a node, `PlaceInWorld { body }`, that reads one `Body` variable at the identity pose. That pose is exactly today's meaning, because a document's own coordinates are export's coordinates.
- *World space*: every copy related to the world, either directly or through other copies (an instance mated to a world-placed instance is in it).
- *Workbench*: everything else. These are the spaces where bodies are built and related to one another, operands and results alike. They are never exported, and the viewer draws them as placement displays.

**Premise check.**
1. *The direction is already half-built (sure).* Today the gather takes roots ∩ world space (`product_in` skips any root whose `Evaluation::space` is not `World`; an unplaced group is "not part of that body", A9/A11 (2)). So the product already has two filters, and A10's sink list is the one that is a holdover. The direction keeps the filter that has a reason, placement, and drops the other.
2. *D10 already forces the empty-world case (sure).* "The world is one undeletable frame … export reads its coordinates and nothing else does." So construction cannot read the world: if a document's default planes read the world frame, every body built on them would be in the world, operands included, and Ev's sentence would fail. Stage 3's replacement for absolute datums therefore has to be a construction frame of the document's own. A document that relates nothing to the world has no world coordinates, so A11 (2) ("STEP export refuses unplaced parts") already makes it unexportable. The product question and the export-coordinates question have one answer.
3. *Where the direction needs a choice (sure).* Iterative modelling is the most common workflow: fillet the part, then chamfer the result. Under the direction, the part that is in the world stays in the world, and the fillet is a new body in the workbench. Something has to move the world placement onto the fillet. If the kernel does it, the holdover is back. My answer is that the GUI gesture does it, as a separate, visible edit (choice 1 below).

**The design (final state).**
- **What appears.** The world space's copies, in the document order of their world placements. Building, combining or measuring a body places nothing. The cut plate stays delivered when measured, because measuring does not touch its placement. A mate, measure or assertion is never in the product. A narration body is simply not placed.
- **Empty world.** The product is empty. That is a valid document: the viewer draws its workbench in placement-display colour. Export, the gather and an instance of the document refuse with `EmptyProduct`, recourse "place a body in the world". There is no fallback. Every fallback (the last body, the frontier, the earliest) picks without being told, and the frontier one is the holdover.
- **Order.** Derived from the placements' document order, with nothing stored and no reorder edit, because nothing reads the order for meaning (export's solid order, D9-deterministic). Re-pointing a placement keeps its position.
- **Deleting a placed body.** The world placement is a reader, so D10's deletion rule applies: it is left unresolved and typed, and the gather and export refuse naming it. The GUI's delete-with-dependents removes it. No special rule is needed.
- **A placement that cannot place.** For example, it reads an instance whose gauge or mate was deleted. Export refuses naming that placement. This keeps what A11 (2)'s refusal protects (a copy someone put in the world silently vanishing), without refusing every unplaced operand.
- **Gather (`product::product`) and the at-rest gate (A5).** Both walk the world placements. The gate judges the world only. A workbench overlaps by design (a box, its tool and the cut all sit at their construction poses), so gating it would be noise.
- **Python and viewer.** `Doc.roots` becomes `Doc.product`, derived and read-only. `DocEdit.set_roots` is replaced by `DocEdit.place(body)`, plus re-pointing a placement's body. The root badge becomes an "in world" badge.
- **Split (A4).** The cut is closed under the dependency relation in both directions, so only its world copies cross. The part's world holds the cut's world copies at their world poses, and the remainder places one instance in its world at the identity. A cut with no world copy refuses, because the part would deliver nothing. **Inline** composes the part's world placements with the instance's. **Promote and Fold** touch no product; they retire with gauges in stage 3. A4's "where the first of them was" goes, because order is derived.
- **Stage 2.** Unit C lands `PlaceInWorld` instead of `Doc::product: Vec<VarId>` and `SetProduct`. That is the same size of change, and stage 3 then *generalises* the placement's pose rather than deleting a list. Instances on the world gauge also need a `PlaceInWorld` in stage 2: the gauge says where and the placement says whether. In stage 3 the gauge retires into the placement.
- **The spec's corpus rule** ("product membership unchanged on every corpus file") becomes a property of the migration, not of the design. Each body-denoting root is given one world placement, in root order, so membership, order and export bits are unchanged on every corpus file. After the migration nothing keeps it: the builders that apply features to a placed body state their re-point, and the test rows that assert root counts are restated.

**Choices for Ev.**
1. **Who moves the world placement onto the next feature.**
   - (a) *Recommended (likely):* the GUI gesture. A feature applied to a world-placed body re-points that placement in a second edit within the same action, and the gesture has a "keep original" toggle. Creating the first body in an empty world places it in the same way. The kernel and Python have no default: a script writes `doc.place(result)`.
   - (b) The person re-points by hand every time. This is honest, but it doubles every feature.
   - (c) Place only at the end, and while modelling show the workbench by a display rule. That display rule would be the frontier rule again, now as display state.
   - (a) is "the client proposes, the kernel stores what was committed", the same posture as VR2's names and D10's variable offer. No node replaces another; a placement's body slot is rewritten by a stated edit.
2. **Export of a document with copies in the world and an unplaced workbench.** I recommend that export write the world and refuse only an empty world or a failing placement. That rewords Ev's A11 (2) ruling (#3441), which says "refuses unplaced parts". The protection it was for is kept (likely).

**Alternatives, as final states.**
- **Explicit list with no door default** (FORK-2's list without the holdover). Membership and world pose become two statements that can disagree: a listed body that is unplaced, or a world-placed copy that is unlisted. Two copies of one body cannot be listed, and an assembly would list every instance besides relating it. The world placement makes those states unrepresentable. Reversal is easy in either direction, but the list is the weaker final state.
- **Derived frontier** (bodies that no body operation reads, among the world-related ones). This is the holdover; it is rejected by the direction.
- **Implicit world for a document with no world mate.** It needs a rule to pick which bodies, and every such rule is a guess. It is rejected.

**Ratified text that changes.**
- D10, Operations: "The product is an explicit list of `Body` variables." This was an agent proposal approved in passing; Ev's own words were "no more nodes that consume other nodes", and earlier "a boolean … places them in the main space". Replace it with: "The product is the world's space: every copy placed relative to the world, directly or through other copies. Building or combining bodies places nothing; an operand appears only if a copy of it is placed in the world. A document whose world holds nothing has an empty product."
- D10, Spaces: add to "export reads its coordinates and nothing else does": "construction never reads the world; a document builds in a frame of its own."
- A10, whole: "`Doc::roots` is an ordered list … `DocEdit::SetRoots` states the list outright." Replace it with the world-space gather above.
- A12: "so a mate is an ordinary non-body root: an isolated sink, listed, ignored by the gather". This goes.
- A4, acceptance: "except that the cut's roots come together where the first of them was (A10's replacement rule …)". It becomes "on the world space, up to the order of copies; the cut's world copies become one copy of the part".
- A11 (2): "STEP export refuses unplaced parts, naming how to place them". It becomes choice 2's wording.
- A2: "takes its A10 product". It becomes "its world space".

**Confidence.**
- Recommendation: likely.
- Today's gather already filters by world space: sure, read from `product.rs`.
- D10 forces a document's own construction frame: likely, by reading "nothing else does" strictly.
- Migration preserves the corpus: likely; I did not run the corpus.
- Split's interface carries only world copies: likely; this rests on A4's two-way closure.

## For the orchestrator

- **Assumed.** Stage 3 makes the world placement a mate bundle and lets a copy reach the world through other copies. I designed the stage-2 node as the identity case of that placement. If stage 3 chooses otherwise, `PlaceInWorld` still stands as stage 2's honest form.
- **Cross-fork dependencies.**
  - FORK-4 decides whether operand slots are writable. Choice 1(a) needs the world placement's body slot to be writable; delete-plus-insert works too, at the cost of moving the placement's position in the order.
  - FORK-1 decides whether a pattern's copies are one `Body` variable. That decides whether a pattern is placed whole or per copy.
- **Not checked.** The audit lane's holdover list had not reached me. REFERENCES D-2 and DM6, and the guide (`docs/guide/assembly.md` lists mates as roots), still need its sweep. I took the viewer's sites, the 82 Python `roots` sites and the refactor sites from the stage-2 spec without re-grepping. I did not count corpus documents with more than one root.
- **Stage-3 questions this raises, not designed here.** Whether the gate checks mate-placed copies inside a workbench space (#3441 said unplaced groups "check as usual"). How the viewer lays out overlapping workbench bodies. Whether an instance should read a named body of a part document rather than its world (close to AQ4).
- **Ev's 2231 ruling** (option D, no declared root set) rested on the sink rule and is overtaken. The PR should say so.
- **Spec edits if adopted.** In #4216: unit C's representation becomes `PlaceInWorld` plus a derived product, `SetProduct` and `ProductFault::Duplicate` become a door refusal of a second identity placement of one body, the corpus row becomes a migration check, and Promote's slot insert still goes.
- No code was changed and no defect was filed off the question.
