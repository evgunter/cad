# FORK-2b — what the product is, when no node replaces another

## For Ev — round 2 (the audit's hits, and "the redesign governs")

**Recommendation unchanged (likely): the product is the world's space.** Round 1 is below. Under Ev's rule that the redesign text governs, one item moves out of "Choices for Ev": export writes the world and refuses only an empty world or a placement that cannot place. That changes A11 (2)'s "STEP export refuses unplaced parts" (Ev, #3441) rather than bending the redesign to keep it, and the protection it was for is kept. Choice 1 (the GUI gesture re-points the world placement) remains Ev's.

**The audit's hits, one by one.**
- **H1, A10's invariants** ("the root set is exactly the DAG's sink set"). **Retire.** Nothing is a sink in the product's sense. Replacement for A10 as a whole: "The product is the world's space: the copies placed relative to the world, gathered in the document order of their world placements. An operation places nothing. An empty world is a valid document whose product is empty; a door that needs a product refuses `EmptyProduct`." (sure)
- **H2, A10's maintenance** (replacement, orphaned inputs, `on_insert`, `on_delete`, `on_set_members`). **Retire, with no successor.** No edit moves the product as a side effect. The spec's "membership unmoved on every corpus file" and its test 6 become a one-time migration check (one world placement per body-denoting root, in root order), not a rule. The 73 `combine_ops` and 38 `creation_ops` viewer rows are restated to read the world placements their gestures author (choice 1). (sure)
- **H3, A4's split acceptance.** **Change.** New text: "Split-then-evaluate equals unsplit evaluation on the world space at structural and name-resolution identity, up to the order of copies: the cut's world copies become the instance's copies, each a world placement of the instance's output for it. Inline-of-split returns the document split was given, up to minted ids and order." The "first of them" splice goes, because order is derived. (likely)
- **H6, DM4's pip delete** ("plus a plain `DeleteNode` of the orphaned transform"). **Change.** New text: "Deleting a pip is `SetMembers` without it. The transform stays a value; it is not in the product, because nothing places it in the world. Deleting it as well is tidiness, not a requirement." (likely)
- **H7, D-2's closure.** **Change: the rule narrows and gets a new reason.** The cut must still be closed toward its ancestors, because a part cannot read its host. The half toward consumers rested on "every cut sink is a document sink", and that reason retires. New text: "A remainder read of a cut body is admitted when the cut places that body in the world, and re-points to the instance's output for that copy; a read of a cut body the cut does not place refuses (`SeveredEdge`), because a part delivers only what its world holds." This corrects round 1's "only world copies cross": world copies cross, and remainder reads may follow them. The same reasoning retires inline's `InstanceConsumed` (H8): a reader of an instance output re-points to the inlined body. (likely)
- **H9, `PlacedUnderTwoRoots` / N4's once-per-product rule.** **Retire.** Two world placements of material that shares an instance are two copies (D10: "two placements of a part are two copies"). Each copy qualifies the names it carries by its placement, as `Instance` qualifies a pattern copy, so no name reaches the product twice. In stage 2, where a placement can only be the identity, the one surviving refusal is a door refusal of a second identity placement of the same body. Two coincident copies are not a product statement. (likely)
- **H10, A5's minting lift through consumers.** **Retire `names::lift`'s consumer walk and the `MovedAbove`/`Vanished` arms.** A mate relates copies. Its declaration is minted on the world copies of its two members when both are in the world. When they are not (a mate relating a boolean's operands in the workbench), it is not an at-rest fact of the product and nothing is minted. The boolean's own coincidence door decides the glue (D10). New text for A5's paragraph: "A declaration is minted on the world copies of its mate's members, read through those copies' placements; a mate whose members are not both in the world mints nothing." (likely)
- **H11, A2's "takes its A10 product".** **Change** to "takes its world space: each world copy of the pinned document, in its world coordinates". #4222's "one variable per entry of the part's product" becomes "one `Body` variable per world placement of the part", so a part placed twice in its own world hands out two outputs. (likely)

**Also leaning toward the redesign:** the spec's unit C representation (`Doc::product: Vec<VarId>`, `SetProduct`, `ProductFault`) is replaced by `PlaceInWorld` and a derived product, as round 1 said. Its goal of unchanged membership survives only as the migration check above.

---

## Round 1 (as delivered)

### For Ev

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

### For the orchestrator (round 1)

- **Assumed.** Stage 3 makes the world placement a mate bundle and lets a copy reach the world through other copies. I designed the stage-2 node as the identity case of that placement. If stage 3 chooses otherwise, `PlaceInWorld` still stands as stage 2's honest form.
- **Cross-fork dependencies.**
  - FORK-4 decides whether operand slots are writable. Choice 1(a) needs the world placement's body slot to be writable; delete-plus-insert works too, at the cost of moving the placement's position in the order.
  - FORK-1 decides whether a pattern's copies are one `Body` variable. That decides whether a pattern is placed whole or per copy.
- **Not checked.** The audit lane's holdover list had not reached me. REFERENCES D-2 and DM6, and the guide (`docs/guide/assembly.md` lists mates as roots), still need its sweep. I took the viewer's sites, the 82 Python `roots` sites and the refactor sites from the stage-2 spec without re-grepping. I did not count corpus documents with more than one root.
- **Stage-3 questions this raises, not designed here.** Whether the gate checks mate-placed copies inside a workbench space (#3441 said unplaced groups "check as usual"). How the viewer lays out overlapping workbench bodies. Whether an instance should read a named body of a part document rather than its world (close to AQ4).
- **Ev's 2231 ruling** (option D, no declared root set) rested on the sink rule and is overtaken. The PR should say so.
- **Spec edits if adopted.** In #4216: unit C's representation becomes `PlaceInWorld` plus a derived product, `SetProduct` and `ProductFault::Duplicate` become a door refusal of a second identity placement of one body, the corpus row becomes a migration check, and Promote's slot insert still goes.
- No code was changed and no defect was filed off the question.

## For the orchestrator — round 2

- **H7 against #4222.** #4222's instance outputs make the narrowed closure buildable: a remainder reader re-points to the output for that copy. If #4222 lands an output per *product entry* instead, the text needs only "product entry" read as "world placement".
- **H9's copy qualifier** is a naming addition, owned by stage 3 (copies) and by N4's page. In stage 2 the door refusal covers the only reachable case.
- **H10.** Today's corpus assemblies are world-gauged instances mated directly, so their mints are unchanged. The only behaviour that moves is a mate that is read only through a boolean, which stops minting. That is the D10-correct outcome. I did not check whether any corpus file has one.
- **Untouched hits (outside the product question):** H4 retires with A12 as #4216 F plans. H5 (E3's "sink node" wording) and H12 (A9's vocabulary) are rewordings for the units that touch them. H13's guide line `:1069` should read `doc.product`, as derived.

## Round 3 (against the other report)

**1. Representation: hold the node, and take the other report's pose column.**
- *A's per-space objection does not touch this design.* My world placement is per body: `PlaceInWorld { body }` makes one copy. Round 1 already says the workbench holds operands and results together and that none of them appears unless placed. We agree that "placed" is per copy and that a boolean's result shares its operands' space.
- *What decides between the two: a placement must define the copy.* Two things need to read a copy:
  - stage 3's mates, which name "the copy" (A says so itself: "a mate on a twice-placed body names the copy");
  - an instantiating document, which per H11 gets one output per world placement.
  Under D10 only an operation defines a variable ("a node is an operation: it reads variables and defines one or more"). A row outside the graph can read a body; A treats the row as a reader for deletion. But a row cannot define the copy. So a row would be a second reader kind outside the graph, with its own strand, key and load rules, and it still could not be read in turn. A node is the one way.
- *The shape of the row is the shape stage 3 deletes.* `gauge` and `offset` are what stage 3 retires ("gauges, offsets, `Transform`-as-placement … retire"). Building the product on them hands stage 3 a second home to empty.
- *I move on content.* The stage-2 node carries a pose slot, the same rigid-chain type `Transform` holds (the identity by default), rather than the bare identity: `PlaceInWorld { body, pose }`. That removes my round-1 interim duality. An instance's world pose then moves from its gauge into its placement, instead of the gauge saying where and the placement saying whether. Stage 3 grows `pose` into mates, as A wants for its `offset`.
- *A's earlier argument is answered* where it is about per-copy placement (we agree) and about reusing the instance shape (the pose slot does that). It is not answered on rows: nothing in A's report needs the placement to be outside the graph.

**2. Who writes everyday placements: move for the viewer, hold for Python.**
- *Neither is "the kernel deciding".* A façade constructor and a GUI gesture both author ordinary edits that the log records. My round-1 choice 1(a) is A's "P" for the viewer, so on the viewer we agree.
- *Where we differ is Python, and the asymmetry decides it.* A person in the viewer watches the world continuously, so every feature on the part has to move the world placement or the picture goes stale; a gesture with a visible toggle is right there. A script needs only its end state. One `doc.place(result)` before export says the product in the text the reader reads. Under P, `c = a.cut(b)` silently unplaces `a` in the façade. That is "the tip replaces its operands" respelled as façade semantics: visible in the log, invisible in the script.
- *P's main argument is answered by the rule Ev gave:* "every script and corpus file reads as today" is the migration goal, which no longer binds. The corpus builders add one `place` per product body, and the migration check verifies membership once.
- *So the question for Ev narrows:* not whether the kernel decides (it never does), but whether the Python façade may imply world edits. I lean no (likely).

**3. Export with unplaced bodies: converge on landing it in front of Ev, not as a fork.**
- *Both reports narrow #3441, and neither can keep it literal.* Every boolean operand is unplaced, so "refuses unplaced parts" would make every part unexportable. A change forced by the redesign is, under Ev's rule, not a choice. It still changes Ev's own words, so the PR lists it under "Ev's own text that changes" and Ev sees it there. A is right that Ev must see it; I am right that it is not a fork.
- *Hold on refusal over notice, for the one case #3441 protected.*
  - A placement that cannot place refuses export, naming it. For example, a world placement that reads a deleted body, or (in stage 2) an instance whose placement's pose no longer resolves.
  - That is the deliberate copy that silently vanished, which #3441 was about.
  - A notice listing every unplaced body would list every operand of every boolean: routine state reported as news.
- *I accept one part of A's notice:* an unplaced mated group (copies related to each other but to nothing in the world) is worth a notice, because someone assembled it. A lone unplaced body is not.

**Net.**
- Agreed: per-copy placements, nothing implicit, order derived, stranded placement refuses, `PlacedUnderTwoRoots` retires, P in the viewer.
- Still divided: node versus row (I hold node; it defines the copy), and the Python default (I hold explicit).
- Converged: export's change goes to Ev as a listed edit to Ev's own words.
