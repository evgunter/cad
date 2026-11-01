# SPEC: INTENT stage 3 — spaces and placement

Scope: D10's **Spaces and placement** and **Repetition** paragraphs, the pose half of its **Variables** paragraph, and the placement sentences of its **Operations** paragraph (`docs/DESIGN.md` §D10), as `work/intent/plan.md` stage 3 states it:

- Poses become variables of their kinds, and none is free or defined from nothing.
- A placement owns the mates and values that pin its copy, and places it into a space it names. Nothing else moves a body.
- The world is one undeletable space. Its axes chart the values of placements into it, and no mate or construction reads it.
- Bodies are combined by four operators, and a document's product is one explicit slot.
- Constructions read no frame: each is built in coordinates of its own and placed.
- Gauges, offsets, `Transform`, absolute datums and the pattern presets retire.
- An operation computes in a frame that is a function of its reads (D9).
- A constraint any of whose equations the bundle already fixes refuses as an overconstraint, by subgroup algebra (A11 (1)). That closes `mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion` with A11 (4).

**Rulings this spec follows.** Ev's rule that the redesign text governs earlier text applies throughout (`docs/prompts/designer.md` §2). Where a work item's body and D10 differ, D10 as it stands on main is the text built.

- **D10** (#3990/#4002), Ev's direction of 2026-10-03 (verbatim at `5f7a1c71e3`). Four of Ev's words bear on this stage:
  - "parts don't sit at (0,0,0) in their own space; they just don't have a location (or orientation)";
  - "you can't Transform an already placed part, you just determine its placement by relation to other parts";
  - "the world frame is a special undeletable node that looks like a part in the sense that other things can relate to it kind of like it's a part, but it actually just sets the coordinates";
  - "having constraints that fall back to being assertions seems worse than either".
- **FORK-1 and FORK-1b** (#4222): a pose is a frame known up to its kind's symmetry; the kinds are D10's five; a combination is a named construction that refuses its degenerate case; an incidence is constructed, never checked; a 2-D value lives in the node that holds its frame. Ev: "ideally the mates' `Subgroup` stuff can literally be shared".
- **FORK-3** (#4222): a selection is a definition, not a node.
- **FORK-S3P** (fork log row 95, PR 4324; settles the old FORK-S3-1 and S3-6). No pose is free, none is defined from nothing, and no construction reads one. A `Profile` is 2-D; a sweep reads at most a direction in the profile's own axes or a 2-D axis line. Every body is built in coordinates of its own, and a feature on a face is a construction placed against the face by mates, the side being the mates' to say. Spaces are kinds decided from the recipe. A face reads as a plane, and no reader takes a carrier's reference direction. An operation computes in a frame that is a function of its reads, chosen for conditioning, never the world's. Ev (round 10): a spin within a symmetry "does need to be set explicitly, but 0 (or (0,0) etc) is always a valid value".
- **FORK-S3O** (row 96, PR 4325; settles the old FORK-S3-4 and stage 4's FORK-S4-5). A constraint any of whose equations the others already fix refuses, pinned or not, by subgroup algebra, with nothing measured and nothing symbolic. Where two copies meet beyond what their mates fix is a contact, recorded and linted, and its recourse is an assertion. A placed copy's frame is not a variable: it is the construction its bundle states, which the coincidence door replays. This supersedes "the kernel computes each space in the frame of its earliest member".
- **FORK-S3M** (row 97, PR 4326; settles the old FORK-S3-2, S3-3 and S3-5). `Place` owns its constraints, mates and values on equal footing, each addressed by (placement, an id minted with it). It reads a list of shapes of one space, as a union reads its operands. A mate equates a pose read off the copied shapes' geometry with one read off geometry of the space the copy joins, and holds no number; its sense is `Flip`. A value sets one freedom the mates leave, charted on the two bodies' own coordinates, and zero is always valid. Nothing moves a body: `Transform` and `PlacedFrom` retire. An instance is a placement with one output per world placement of the part; there is no instance `frame` port.
- **FORK-PAT** (row 99, PR 4341). Repetition is an index variable: a pattern is a placement whose reads reach an index, the presets are the façade's, `PlacedUnion` retires into a placement and a union, and a mirror is the construction `Mirror { body, plane }`.
- **FORK-MOVE** (fork log row 149). No slot holds a component of a direction or of a frame's axis: a pose is moved from another by a `Slide` along or a `Spin` about a direction or axis it reads, a read carrying the projection role it reads through; a revolve's axis is a point and a heading, a slant a heading and a tilt; the façade writes the constant `0` for a freedom nobody named; the offer pairs only variables not already the same (VR8).

- **FORK-DM4** (`a-union-member-is-keyed-by-its-read`, PR 4527; `a-name-is-scoped-by-the-variable-holding-its-body`, PR 4674). An operation names what it carries in by the read (`From { read, of }`); `Union { members }` and `Intersect { members }` take one `Bodies` argument, a family read or reads spelled at the slot; `Subtract { from, tool }` is the one pair; `Boolean` has left the document; a name is scoped by the variable holding its body.
- **Row 162** (PR 5312, Ev approved; `a-placement-relates-two-frames`). D10 `[placement]` as it stands: a space is a construction's coordinates or the world; only a placement relates two spaces, and spaces form a DAG; a placement reads a source space and places its carried bodies into a space; mates fix what they fix and values set the rest, all six where there is no mate, the rotation then an author-ordered chain of at most three spins about the target space's axes; a bundle leaving a freedom unset refuses at the door; an instance places another document's world or any space of it. Rows 189–191 restate its source as a body and its target as a named space, and retire carried bodies and the space DAG.
- **Row 165** (PR 5315, Ev 2026-10-19; `a-space-names-its-bodies`). A space's readers name its bodies by its variables: a construction's or operation's output, or a **pick**, a `Body` variable defined as one copy a placement places there (a `Bodies` family where the placement's reads reach an index). A pick's `of` is the path through the source's placements to the first variable it reaches, its one spelling; a pick authored twice is two variables. Row 191 restates a pick as one piece of a copy, since a placement now defines one body.
- **PR 4666** (Ev 2026-10-19). D10 `[assertions]`' census between arrivals, and `[placement.placement]`'s carry of the source's at-rest decisions.
- **PR 5230** (Ev 2026-10-17). D10 `[assertions]` "Families": an assertion whose reads reach a family's index is one assertion per member.
- **FORK-REST** (orchestrator, 2026-10-19; designers converged, no ratified text moved) and **D brings forward the mateless bundle** (orchestrator, 2026-10-20): only the identity bundle as written places by no map, in every lane; every other placement runs its map. `Value.mate` is optional and a bundle with no mate states the whole motion; `stand` writes the identity bundle.
- **Rows 189–191** (PR 5494, Ev 2026-10-22: "it looks good!!"; D10 at `4a4bb23608`). A body is defined by exactly one of a construction, a one-input operation, an operator over bodies, or a placement (`[operations.operators]`). There are four operators, DM4's `∪ − ∩` and `⊔` assemble, which keeps its operands as separate pieces; `∪` does not absorb `⊔`. Each operator decides every meeting between its operands at the coincidence door, a meeting inside one operand is that operand's decision, carried, and cells that are one construction read twice are proven by the first rung (`[coincidence.booleans]`). `Space` is a kind that is only minted: the world, `new_space()`, or a construction's own (`a.space`); every body is in exactly one space, derived from the recipe. A placement reads a source body, a target space and a bundle, and defines one copy in the target. Mates read only geometry, no space has roles, and values chart on the target space's axes, so a value names no mate (row 190). An operator's operands share one space, a static check (`SpaceMismatch`); a re-target leaves the reads it breaks unresolved and typed. A document has one explicit product slot, which reads a body in its world, and an instance reads another document's product. Inline is one rule (ASSEMBLY A4). The census is each `⊔`'s door, and an arrival is one operand at one value of its indices (`[assertions]`).
- **Row 187** (PR 5555, Ev 2026-10-22). A mate is the first rung's statement: the two poses it equates are one construction read twice (`[coincidence.structural]`). The census decides each pair once at the highest-dimensional cells that touch, so an area contact is one face pair, and a flush pair is a finding unless established. Two pose kinds, `Flag` and `Pivot`, under the principle that a kind exists for each subgroup that is the symmetry of what a cell, alone or with one incident cell, determines.
- **The helpers ruling** (orchestrator, 2026-10-20, `work/intent/log.md`; the designer pair converged, no ratified text moved). No helper reads geometry or writes a number its caller did not write: each is one pure constructor of a bundle from the mates and moves its caller wrote, with the constant `0` on each freedom left unnamed. `place_at`, `pin_at`, `frame_at`, `place_against` and `Anchors` retire, and the place-part gesture writes the identity bundle.
- **The copies ruling** (orchestrator, 2026-10-20, `work/intent/log.md`; a third designer weighed the crossed pair). A body value is its body in its own coordinates, shared by `Arc`, plus one composed frame: `Framed { body, frame: Own | Map(RigidMotion) }`. `relative` is the one door between two operands, and content keys are unchanged.
- **The E rider** (PR 5547, merged 2026-10-21). A minted reference comes from the inputs: every carrier mint takes a reference its caller already holds (`newell_plane` takes `u_ref`), and `scripts/gates/minted-reference-from-inputs.sh` holds `orthonormal_basis` to its scratch homes.

D10's last paragraph retires, here, **ASSEMBLY A3**, **A11 (2)'s gauges and offsets** and **A11 (4)'s declaring mates**. §9 quotes each clause this stage retires or rewrites.

**Baseline.** This spec is written against stage 2's **final shapes**, not today's code: after stage 2 F, every operand is a read, `Select` defines `Face`/`Edge` variables, and mate sides read `Face` variables. Stage 2 C's `PlaceInWorld` has landed by B: B waits on stage 2 F, which follows stage 2 D, which waits on stage 2 C (`the-product-is-an-explicit-list`). B's migration covers it, since it is how a plain document's bodies reach the world.

**Line numbers.** §2–§5 cite main at the stage 3 spec's first baseline (`044b5eb2e`) and drift under stage 2's units. §5′, §6 and §8 cite main at `4a4bb23608` (D merged as PR 5506, and PR 5494 on top of it), by symbol, with a line number only beside a name. Each unit's dispatching orchestrator re-greps at the merge of the unit it waits on and states the drift.

**What has merged.** A, A′, B, C, D and F have merged, and so has E's rider (PR 5547). §2–§5 and §7 are the record of what they were specified to build. What remains is D′ (three PRs), E's core and G, and the census units that wait on D′ (§0, §12).

The first baseline's state of what this stage touches (`044b5eb2e`):

- **Poses.** `VarKind` names the five poses (`var.rs`, from stage 2 A). `VarDef` is `Free`, `Defined` or `Output`, and `kind()` derives `Free` and `Defined` from a scalar `Dimension`, so a defined pose cannot be represented.
  - Every pose is an output of a `Datum` node (`node.rs`, `Node::Datum`). Four of the six datums are absolute, written in world coordinates: `Plane`, `Axis`, `Point` and `Frame`. `FaceFrame` is DM1's derived frame; `AxisInPlane` is a revolve's axis.
  - `wire_datum` (`eval/wire.rs`) evaluates them to `topo::query::DatumValue`.
- **Constructions that read a frame.** A profile reads a plane (`ProfileProgram.plane`, `program.rs`); `Tube` and `HollowTube` read `frame`; `Revolve` reads `axis` (an `AxisInPlane` id); `Extrude` carries `side: ExtrudeSide`; `Split` reads a `Datum::Plane` id.
- **Placement.** There are six ways to place something: absolute datums; `Node::Transform` (a `Placement<S>` chain); `PatternKind::Explicit(Vec<placement::Frame>)` (raw f64); a gauge plus an offset (`Node::Gauge`; `InstantiatePart.gauge`/`offset`); mates; and, if stage 2 C has landed, `PlaceInWorld`'s pose.
- **Patterns.** `Node::Pattern` with `PatternKind::{Linear, Circular, Explicit}`, `Node::PlacedUnion`, and `PartSelect::Instance(i)`, named by `RoleSeg::Instance { i, of }`.
- **The solve.** `solve_group` (`mate/solve.rs`) takes a spanning tree over each placing group (`places` is "same gauge reference"). Tree mates are `Determining` and fold to `Trivial`, else `MateFault::Under`. Every other mate stays `Declaring`.
  - **Nothing verifies a declaring mate against the solved poses.** ASSEMBLY A11 (4)'s "an inconsistent loop dies at its closing mate's verification" is not what the code does (§13).
  - The at-rest gate mints a declaring mate as it mints a determining one (`assembly.rs`), checking contact, never the alignment.
- **Offsets checked against the solve.** A non-root member's offset is checked by `check_offsets` (`mate/solve.rs`), refusing `OffsetDisagrees` or `OffsetUnchecked` (`mate.rs`).
- **What a mate stores.** `Alignment<S>` (`mate.rs`) holds two `MateFrame { base: FrameBase, offset: Placement<S> }`, a `MatePrimitive` and an `AxisSense`. `PlanarRest { offset: f64 }` and the rider `clocking: Option<f64>` are raw floats. `MateFrame::authored` turns raw f64 vectors into a `Step::Literal`.
- **The world and spaces.** The world is `gauge: None`. A placed group computes in world coordinates (`group_frame`, `mate/solve.rs`). An unplaced group computes at the identity of its earliest instance (`Space::Own`; `instance_frame`, `eval/wire.rs`).
- **Export.** It writes bodies in whatever frame they were evaluated in (`crates/step-export/src/lib.rs`; `pncad/src/export.rs`).

## 0. Ordering: each PR green, none landing half a representation

| PR | Unit (`work/intent/`) | Lands | Representation step it completes | Goldens |
|---|---|---|---|---|
| A | `poses-are-variables` (merged) | pose definitions read off geometry, by coordinates in a constructed frame, by construction (`Through`, `Meet`, `Flip`, the standoff), by projection, or as an output; one `Subgroup` on the pose value; the revolve's axis as a 2-D line on the node; `Split`'s tool as a `Plane` read | **a pose is a defined variable of its kind** | re-blessed: `AxisInPlane` leaves the node table, and so does each `FaceFrame` read only as a plane. Geometry bit-equal |
| A′ | `poses-move-by-lengths-and-angles` (FORK-MOVE; merged) | A's built vocabulary restated: `Slide` and `Spin`; a read carries its projection role (a frame's x, y, z, origin, planes); `InFrame`'s coordinate arms, `PoseCoords`, `Project` and `Standoff` retire; `Revolve`'s axis is `axis_through` and `axis_heading`; VR6's constant `0` is built and `equal_variables` compares VR8 lowerings | **a pose is moved by lengths and angles** | re-blessed; a datum that was not axis-aligned moves by rounding, reported |
| B | `a-placement-is-the-bundle-of-mates` (merged) | `Place { shapes, constraints }` owns today's mates; the world node; an instance is a placement; spaces are kinds from the recipe. Gauges, offsets, `PlaceInWorld`, the spanning tree, roots and `MateRole` retire, and a mate beyond its bundle's pin refuses | **a placement owns its mates; the world is a node** | re-blessed. One-time migration check: poses and product unmoved or moved by rounding, every move reported |
| C | `a-mate-relates-two-poses` (merged) | a mate is `{ on, to }`, two poses of one kind, holding no number; values set the freedoms mates leave. `MatePrimitive`, `MateFrame`, `FrameBase`, `AxisSense`, the rider and `PlanarRest.offset` retire | **a mate equates two poses; a value sets a freedom** | re-blessed: mate wire. Solved poses equal to within the migration's stated rounding |
| D | `transform-retires-into-a-placement` (merged) | constructions read no frame: a profile is 2-D, a tube is built in its own coordinates, an extrude has no side. `Datum`, `Transform` and `PlacedFrom` retire; each becomes a placement | **nothing moves a body, and frames enter only at placement** | re-blessed. Each moved body moves by one composition, which the migration reports |
| F | `a-mate-on-a-pinned-copy-refuses` (merged) | a constraint any of whose equations the bundle already fixes refuses, pinned or not, by subgroup algebra; `Contradictory`, `member_of`, `trivial_member` and the measured case splits retire | **a mate places and never checks** | re-blessed: corpus bundles that overconstrain are restated or dropped, each named. Poses unmoved |
| D′1 | `a-placement-names-its-target-space` | `Space` a variable kind, minted by the world, `new_space()` and each construction (`a.space`); `Place` and `InstantiatePart` read a `target`; every body's space derived statically, and `SpaceMismatch` decided from the recipe; every bundle pinned. `mate::solve::Space`, `Space::Loose`, the loose copy and `TargetsDiffer` retire | **a placement names the space it places into, and a space is minted** | re-blessed: every placement's wire and key (its target), the var tables (a `Space` output per construction), ids. Geometry unmoved |
| D′2 | `a-mate-reads-only-geometry-and-a-value-charts-on-the-target-space` | `Target::World` and `WorldRead` retire into values; a value names no mate and charts on the target space's axes, swung onto the one mated direction; a lone point mate's spin chain; the helpers become one pure constructor; inline is one rule; the flatten edit | **a mate states a physical relation; a value is a coordinate of the target space** | re-blessed: the mate and value wire, the chart's numbers. Geometry moves where a mated target body stands off its space's identity (the chart's owner changed) or a helper wrote a rounded turn, each reported |
| D′3 | `assemble-is-an-operator-and-the-product-is-a-slot` | `Node::Assemble` (`⊔`) with the census at its door; one product slot; a placement reads one source body and defines one copy; an instance reads its part's product; picks. `RoleSeg::Placed`/`InPart`, `CopyKey`, `Copies`/`KeyedCopy` and the gather of carried bodies retire | **bodies are combined by four operators, and a space names its bodies by picks** | re-blessed: names (no `Placed`/`InPart` layer), ids, the stored documents (regenerated). Product digests unmoved; census rows move where a `⊔`'s operands differ from today's world copies, each reported |
| E | `an-operation-computes-in-a-frame-of-its-reads` | `Framed { body, frame }`: a copy shares its source's body and holds one map; an operation over operands in one space computes in its first operand's frame and reads the rest through `relative`; one composing door for export and the viewer's drawing | **construction never reads a space's map** | **product body digests move** (computing-frame coordinates); STEP bytes move by rounding |
| G | `patterns-are-index-variables` | `index(N)`, families, `Member` names; a pattern is a placement whose reads reach an index, its copies a family in the target space, combined by `⊔` or `∪`; `Mirror { body, plane }`; `PatternKind`, `Node::Pattern`, `PlacedUnion`, `Part`/`PartSelect::Instance` and `Frame`'s improper arm retire; an assertion reaching an index is one per member | **repetition is an index; no preset is stored** | re-blessed: names move (`Instance` → `Member`). Copies' geometry unmoved or moved by rounding, reported |

The dependency graph of what remains:

- **D′1** waits on nothing. D has merged and PR 5494 ruled its design.
- **D′2** waits on D′1: a value charts on the target space's axes, which only a named target gives it.
- **D′3** waits on D′2: inline, which D′2 makes one rule, lands the part's product body only once D′3 has a product slot, and the two rewrite `refactor.rs`'s inline and `node.rs`'s `Place` together.
- **E's core** waits on D′3: an operation over copies names operands that are each one body in one space, and E rewrites the value D′3 leaves `wire_place` defining.
- **G** waits on D′3 (a pattern's copies are a family of one placement's copy, read by index and combined by `⊔` or `∪`) and on E's core (copies of one body are built once and mapped, which is E's `Framed`). Its `Mirror` (§8 (c)) waits on nothing.
- **The seat and census units** (§12, Sequencing): `the-door-reads-a-mated-seat-by-its-mate` and `the-census-records-a-planar-area-contact-as-one-face-pair` wait on nothing; `flag-and-pivot-pose-kinds` waits on D′2; `a-placement-carries-its-sources-at-rest-result` waits on D′3 and E's core; `each-space-runs-its-census-between-arrivals` waits on the carry.

Why this order:

- **D′ in three PRs.** D′ changes how a placement names its target, what a mate may read, how a value is charted, how bodies combine, what the product is and how a copy is named. That is about as many files as B (§11). Each of the three leaves one way to say each thing it touches, and no two of them share a representation step.
- **D′1 first.** Everything else reads the target: the space chart (D′2), the `SpaceMismatch` of a `⊔`'s operands (D′3), inline's `new_space()` (D′2), the viewer's shown spaces.
- **Mates and values (D′2) before `⊔` and the product (D′3).** Retiring `WorldRead` breaks today's inline, which composes the instance's motion into world reads (`mate::pin::written_moves`, `refactor::inline`). Inline as one rule copies the part's placements verbatim into a `new_space()`, which is sound only once their values chart on their target space's axes and no mate reads the world's roles. So inline lands with D′2. Before D′3 it places the spliced copies as a list (the one interim D′2 adds); D′3 makes that the copied product body.
- **E after D′.** E's operation over copies computes in its first operand's frame and reads the rest through `relative`. That presumes each operand is one body and every operand is in one space, which D′1 checks and D′3 makes true of copies.
- **G after E.** G builds copies of one body once and maps them, which is E's `Framed`.

Each intermediate state is a whole representation:

- After D, a bundle with no mate states its whole motion and `stand` writes the identity bundle. Named interims remain, which D′ ends: the loose copy, a placement whose space is derived from its mates, `WorldRead`, a value naming its mate, copies named by a `Placed`/`InPart` layer and read by `CopyKey`, the gathered product, and helpers that write turns read off geometry.
- After D′1, every placement names its target, every body's space is derived from the recipe, and every bundle pins its copy. Named interims: a mate's `to` may still read the world's roles (`Target::World`), legal only where the placement's target is the world; a placement copies a list of shapes; the product is the gather of the world's placements; an instance copies its part's world placements. D′2 and D′3 end them.
- After D′2, mates read only geometry, values chart on the target space's axes and name no mate, every helper writes only what its caller wrote, and inline is one rule. Named interim: inline places the spliced copies as a list until D′3.
- After D′3, bodies combine by four operators, the product is a slot, a placement copies one body, and readers hold picks.
- After E, every operation computes in a frame of its reads, and only export and the viewer's drawing compose a body into a space's coordinates.
- After G, the document stores no pattern preset.

**Rejected:**

- **Mates as poses (C) inside the placement unit (B).** About 350 files in one PR: every mate site, every gauge site and the refactor doors, with no point at which poses can be compared.
- **The old order, mates as poses before placements.** The in-plane numbers would sit on the mate until a placement exists, a second spelling D10 forbids.
- **D before C.** A construction placed against a face needs a mate reading the face's plane and values for the freedoms it leaves.
- **F inside B.** B's migration check reports every pose and product that moves; F restates bundles. Mixing them hides which change moved a body.
- **D′ in one PR.** About B's size, and no point at which the space derivation can be checked before every reader moves.
- **`⊔` and the product (D′3) before mates and values (D′2).** Inline has to be rewritten in whichever lands second. Landing `⊔` first leaves inline either composing world reads over a product body or reading the roles of a non-world space, a capability D′2 would only delete.

## 1. Final shapes (after G)

**Pose variables** (`var.rs`). A pose is only defined (FORK-S3P), by `VarDef::Pose(PoseDef)`:

- **Read off geometry.** `Plane { face: Face }` (a face reads as a plane: its carrier's plane with its outward normal); `Axis { of: Face | Edge }` (a carrier's axis or an edge's line); `Point { of: Face | Edge | Vertex }` (a carrier's centre, or a vertex's point). No definition reads a carrier's reference direction, so DM1's `FaceFrame` and its spin have no successor.
- **The moves** (FORK-MOVE): `Slide { pose, along: <a Direction read>, by: Length }` and `Spin { pose, about: <an Axis read>, by: Angle }`, each of `pose`'s kind and each holding one number. A chain is nesting in the order the author wrote it, so no construction bundles numbers whose order would matter. A point at coordinates in a frame is the frame slid along its own axes and read through its origin; the façade's `point_in(frame, x, y, z)` writes it. No slot holds a component of a direction or of a frame's axis, so an axis that is not orthogonal, a zero direction and a normalisation cannot be written. A tolerance on a datum's position or tilt is a tolerance on these lengths and angles (VR8 unchanged).
  - A pair the kinds forbid (a slide of a `Direction`) is a missing variant.
  - A move with no result at these values refuses `PoseDegenerate` at evaluation, as `Through` and `Meet` do (FORK-1b).
  - A move whose result its number does not change at these values (a plane slid along a direction lying in it, an axis spun about a parallel line) is admitted, since admission measures nothing (FORK-S3O), and the unobservable-value report names it, as it names a round pin's spin.
- **The named constructions** (FORK-1b), each refusing its degenerate case typed at evaluation, never an automatic join: `Through { axis, point } → Frame` refuses a point on the axis; `Meet { a: Plane, b: Plane } → Axis` refuses parallel planes; the rest wait for a reader (Q1).
- **`Flip { pose }`**: the opposite sense of a `Direction`, `Axis`, `Plane` or `Frame`. It is an involution the door normalises to one side, and it has no `Point` arm (FORK-S3M).
- **A standoff** is `Slide { pose: P, along: P through normal, by }`: the one number a kind's own equation fixes, as a construction on a mate's target (FORK-S3M).
- **A projection is part of a read**, not a definition. A slot of a coarser kind reads a finer pose through a role the pose's kind offers: a `Frame` through its x, y or z axis, its origin or one of its three coordinate planes; a `Plane` through its normal; an `Axis` through its line or direction (D10: "a finer value is read through its projection"). `Spin { pose: F, about: F through z }` reads one variable from one definition, so VR7 counts one reader (`Doc::reader_counts` dedupes a definition's reads, at the edit and the load doors alike) and a frame's own turned axis needs no name.
- **`Output { node, port }`**: an operation's pose output, such as a revolve's `axis`.
- **`Carried { copy, pose }`**: a pose of a copied shape as that copy carries it, keyed by the placement (FORK-S3M). It is how a mate reads a pose of another copy.

A pose variable has no unit and is never an analysis axis itself; its scalars are. It reads as its definition ("the plane of face Top of Extrude "base"").

**One `Subgroup`** (FORK-1b; Ev on #4222). A pose value, `PoseValue<T>` (renamed from `topo::query::DatumValue` and moved into `editor-core`'s pose module), implements `PoseSymmetry` with a `Subgroup<T>`. `VarKind::symmetry` names the family, and the door asserts the two agree. `mate/coset.rs` reads the sides' symmetries there, and `side_symmetry` (`mate/solve.rs`) goes. `Point` and `Direction` have arms, because D10 names a lone point mate.

**Constructions read no pose** (FORK-S3P). A root construction (one reading no body) is built in coordinates of its own:

- a `Profile` is 2-D: its numbers are read only against each other, and it reads no plane;
- `Extrude` reads a profile and a depth, and a slanted extrude a `heading: Angle` about the profile's normal and a `tilt: Angle` from it, in `[0, turn/4)` (at tilt `0` the heading is reported unobservable). `ExtrudeSide` goes: which side of a face the material lies on is the placement's mates' to say;
- `Revolve` reads a profile, a 2-D axis line in the profile's axes (`axis_through: [Length; 2]` and `axis_heading: Angle` from the profile's x axis, so `turn/4` is the constant "along y" and a zero direction cannot be written) and an angle, and defines `axis: Axis`, the line's lift;
- `Tube` and `HollowTube` are built about their own axis and read no frame.

An operation that reads a body may read a pose whose reads reach that body's root alone: `Split`'s tool is a `Plane` read off its operand's geometry (or constructed from it), and so is `Mirror`'s plane.

**Two further kinds** (row 187; `flag-and-pivot-pose-kinds`, after D′2). `Flag`: a plane, a line in it and the side of the line its face lies on, read off a face and one of its bounding edges or built as a plane cut by a plane, with symmetry `Prismatic`; its `Flip` keeps its side. `Pivot`: an axis with a point on it, read off a circular edge, a cone, a torus or a face with a vertex, with symmetry `Revolute`. A kind exists for each subgroup that is the symmetry of something a cell determines, alone or with one cell incident to it, and only for those.

**Bodies and operators** (D10 `[operations.operators]`; rows 189–191). A `Body` variable is defined by exactly one of: a construction; a one-input operation (a fillet, a shell, a split, a mirror); an operator over bodies; or a placement. There are four operators:

- `Union { members }` (`∪`, n-ary) fuses its operands into one material;
- `Subtract { from, tool }` (`−`) is the one pair;
- `Intersect { members }` (`∩`, n-ary);
- `Assemble { members }` (`⊔`, n-ary, new in D′3) keeps its operands as separate pieces.

They differ only where two operands' material meets: `∪` and `∩` glue there and merge an overlap, `−` removes, and `⊔` records a touch and reports an overlap as an interference finding. `−` and `∩` distribute over `⊔`, so `(p ⊔ q) − t` cuts both and keeps two pieces. `∪` does not absorb `⊔`: a meeting inside an operand is that operand's decision, carried, and no operator above it judges it again (`[coincidence.booleans]`), so `(bolt ⊔ plate) ∪ bead` fuses the bead to each part and keeps the bolt–plate meeting a record. A solid that touches itself over an area (that bolt and plate, now one solid) refuses, typed and naming the pair, until the census and tier 3′ can read one (`crates/topo/README.md`, the conventions). Within one n-ary operator, member order decides only which description a glued face keeps and which fold step a refusal names; an order that refuses where another builds is a kernel defect. A written list is the façade's sugar for the left-nested tree.

**Spaces** (D10 `[variables.kinds]`, `[placement.spaces]`; rows 189 and 191). `Space` is a variable kind, a coordinate system that holds nothing and contains nothing: no pose, no number, no member list, and no origin, axis or plane a reader takes as a pose. It is only ever minted, three ways, all one kind:

- the world's, which `Node::World` defines at `RecipeNodeId::WORLD` and no edit deletes;
- `new_space()`, a `Node::Space` reading nothing and defining one `Space`;
- each construction's own, a `space` output beside its body (`a.space`). The constructions are the nodes that read no body: `Extrude`, `Revolve`, `Tube`, `HollowTube`, `Loft` and `Sweep` (`eval/mod.rs` `root_space` lists them today). `Mirror` defines no space: its image is in its source's.

The only slot of kind `Space` is a placement's `target`, and the values of placements into a space are the only readers of its axes.

Which space a body is in is derived from the recipe alone (`Doc::space_of`, new): a construction's body is in its own space, a placed copy in its placement's target, an operator's result in the space its operands share, and a one-input operation's result in its input's. So every body is in exactly one space, and nothing is evaluated to know it. An operator's operands share one space, and a mate's `to` reads a body of the placement's target: the edit door, the façade's builder and the Python binding refuse a node that breaks either (`SpaceMismatch { a, b }`), and the GUI does not offer the gesture. An edit that re-targets a placement so that the reads of its copy land in another space is admitted, and leaves each such read unresolved and typed, as deleting a variable does: the document stays valid and loads, and each such node refuses at evaluation with `NodeErrorKind::SpaceMismatch`, naming both spaces. A document edited by hand or merged into that state loads with those nodes marked the same way. Only a placement relates two spaces.

**Placements** (B, C, D′; D10 `[placement.placement]`).

- **The node.** `Place { source: BodyRead, target: Space read, constraints }` reads one source body, of this document, and a target space, and defines one `Body`: a copy of the source in the target under the one rigid motion its constraints state. An instance is the same placement with its source another document's product body: `InstantiatePart { doc_ref, interface, target, constraints }`. Two placements of one body are two copies, at two places, and no placement's data is ever repeated: a sub-assembly is a body written with `⊔` in its space, placed as often as it is wanted.
- **The carry.** A copy carries every pose of the source as the source carries it, and the source's contact records and census decisions, which stay the source's (`a-placement-carries-its-sources-at-rest-result`).
- **Picks** (rows 165 and 191). A **pick** is a `Body` variable defined as one piece of a copy: it names the placement and the piece's variable in the source, through the placements and `⊔` operands inside the source down to the first variable it reaches, its one spelling. A pick through a `∪`, `−` or `∩` has no piece to name and refuses at the door, since those operators keep no pieces. A copy's cells keep their names in the source, and an operator reading two copies of one source keys each operand's names by the read it came through (`From { read, of }`). A pick authored at two sites is two variables, and one whose body the source no longer has is unresolved, typed, never re-pointed.
- **Constraints.** A constraint is a mate or a value, addressed by (placement, an id minted with it), never by its position in the list and never a node. Which copy is defined is which placement reads it, so there is no tree, root or declaring role.
- **Pinned, always.** A placement's mates and values together set every freedom; a bundle that leaves one unset refuses at the door (`MateFault::Unpinned`), whatever its mates, so an unwritten freedom cannot be represented. A body's symmetry pins nothing: a value it makes unobservable is reported, never refused.
- **Overconstraint** (F; FORK-S3O). A constraint any of whose equations the others already fix refuses `Overconstrained { placement, constraints, free }`, pinned or not, decided by subgroup algebra without measuring. A mate that would take a freedom a value sets refuses the same. Where no direction is fixed, the spins are counted by axis: a fourth spin, or two in a row about one axis, refuses.
- **A placed copy's frame is not a variable.** It is the construction its bundle states. The coincidence door reads it by what its mates state (rung 1, row 187) and replays it at `Sym` only where they state nothing. Only the identity bundle as written (no mate, the constant `0` on every freedom) places by no map, in every lane; every other placement runs its map (FORK-REST).

**Mates** (C, D′2; D10 `[placement.mates]`). A mate is `{ on, to }`, two poses of one kind; the kind is the primitive:

- `on` is read off the source's geometry; `to` is read off a body in the target space that does not read this placement. A mate reads only bodies' geometry, never a space and never a frame standing for a part's coordinates, and no space has roles a mate can read, the world included. `Target` is a pose read and nothing else.
- `Frame`–`Frame` coincides, `Axis`–`Axis` is coaxial, `Plane`–`Plane` rests, `Point`–`Point` is a ball, `Direction`–`Direction` is parallel, `Flag`–`Flag` leaves a slide along the line and `Pivot`–`Pivot` a spin about the axis. Each pins the relative pose to a coset of the kind's `Subgroup` (A11 (1)).
- A mate states a physical relation and holds no number. Its sense is `Flip` on one side; a number a kind's own equation fixes (a standoff) is a construction on the target, never a value. A placement whose source touches nothing has no mate.
- The two poses a mate equates, read with the sense it states, are one construction read twice: the coincidence door's first rung (row 187).
- `class: ContactClass` stays until stage 4's `mates-declare-no-contact` retires it.

**Values** (C, D, D′2; D10 `[placement.values]`). A value sets one freedom of the placement's motion that its mates leave, a slide or a spin, to a `Length` or `Angle` variable: `Value { coord, by }`, `coord` one of `Slide(X | Y | Z)`, `Spin` (no axis, beside a mate fixing a direction) and `Spin(X | Y | Z)` (a chain link). A value names no mate: a bundle holds at most one mate fixing a direction, since a second states the angle between them, an equation the first already fixes, so the bundle's values have one chart.

- **Where no mate fixes a direction** (no mate, or a lone point mate), the chart is the target space's own axes. The rotation is a written chain of at most three spins about the target space's `x`, `y` or `z` (through the mated point, for a point mate), no two in a row about one axis, applied in the order written; the slides run along the target space's axes. Turns do not commute, so the author writes the order and the kernel chooses none.
- **Where a mate fixes a direction** (a plane's normal, an axis, a lone direction, a flag's plane, a pivot's axis), the mate carries an `Orientation` (`Mate { on, to, class, orient }`), one of the 24 ways the copy's axes lie along the target space's, the identity unless written: it puts the copy's mated direction `d_c` at `n = O·d_c`, and the least turn `S` carries `n` onto the mated direction `d` (the `to` pose's, its `Flip` the contact's sense). The copy's turn is `rot(a, θ)·S·O`, `a` the target's own direction (its outward normal for a plane: `d` with every flip on the target's side undone). A slide is the copy's origin measured from the target's along `S·x`, `S·y` or `S·z`; the one spin, `Spin` with no axis, is `θ`, the turn left about `a` whatever the contact's sense, so a flip written on either side of a rest turns it the same way. A plane mate leaves two slides and `Spin`, an axis mate one slide and `Spin`, a lone direction mate the three slides and `Spin`; a direction and a point mate `Spin` through the point; a frame mate nothing. Where `n` is a target axis, the slide `S` carries onto `d` is the one a plane mate fixes and an axis mate leaves; otherwise the written slides are admitted only where they span what the mate leaves, decided once from the source's geometry (`NoChartWhy::SlidesDoNotSpan`). The one pole, `n = −d`, is the orientation written backwards: inside the band about it the chart refuses, typed at evaluation and naming the orientation to write (`MateFault::Backwards`); nothing refuses short of it. A point or a frame mate carries the identity (`MateFault::Unoriented`). A flag mate leaves one slide along its line and a pivot mate one spin about its axis; their charts are `flag-and-pivot-pose-kinds`', on this rule.
- The chart reads the target **space**, never the body the mated pose is read off, so one face means one thing however its body got into the space (row 190's ruled owner).
- A slide is a function of the relative pose alone, so no slide is ordered. Zero is always valid and says the two spaces agree as far as the mates allow; a placement whose values are all zero and which has no mate is the identity. **The resting convention:** a plain `Plane`–`Plane` mate equates outward normals; two faces stacked against each other carry a `Flip` on the target. Setting a part on the world's xy is values (`Slide(Z) = 0`, the two tilts `0`, and the slides and spin the author chooses). The façade writes the constant `0` (VR6) for each freedom a gesture leaves unnamed. A placement is the only reader of a source's own coordinates, and reads them only through the freedoms its mates leave.

**The world** (B, D′1; D10 `[placement.world]`). A space like any other, the one every document holds and cannot delete. Its axes chart the values of placements into it, and it is nothing else: no datum plane, origin or axis is a recipe object, and no mate or construction reads it. The viewer may draw its axes or a grid as a display aid.

**The product** (D′3; D10 `[operations]`). A document has one product slot, which reads a body in its world: `product: p ⊔ bolt ⊔ nut`. Nothing is derived from which bodies go unread: being the product is a use, as being what an assertion states is, and what the person made and nothing uses (an operation they inserted none of whose outputs is used, a variable they named that nothing uses) is the `unused` check's finding, an unnamed output or anonymous variable being a byproduct and never one; a slot that reads nothing is an empty product, which export refuses. One document enters another as an instance, a placement whose source is its product body, with that body's contact records and census decisions carried and reported once, as that document's.

**The census** (D′3 and the census units; D10 `[assertions]`). The census is each `⊔`'s door: at rest it examines the meetings between that operator's operands pairwise, and nothing it finds refuses. An **arrival** is one operand at one value of its indices. Between two arrivals it decides each pair once, at the highest-dimensional cells that touch (an area contact is one face pair, a flush pair a finding unless established); inside one arrival it decides and reads nothing, since the arrival's verdicts are its source's or its door's, carried. A finding belongs to the `⊔` that decided it and is reported once, however many times its body is placed or read, and a `⊔` the product does not read is checked all the same.

**Composing into a space** (E; D10 `[placement.world]`). Export and the viewer's drawing compose a body into the coordinates of a space it reaches, the world's or any other shown space's, through one door nothing else calls, and what they produce never re-enters the model.

**The computing frame** (E; D10 `[placement.frame]`; the copies ruling). A body value is `Framed { body, frame }`: the body in its own coordinates, shared by `Arc`, and one map, `Own` (the value is in its space's coordinates) or `Map(RigidMotion)` (the map from the body's coordinates into its space's). A copy is its source's `Arc` under the composed map and is never rebuilt. An operation computes in a frame that is a function of what it reads and of nothing else, chosen so its arithmetic is well conditioned near the geometry it builds, and never in the world's; its operands are in one space (D′1), so where conditioning does not decide, an operation computes in its first operand's frame, as the author lists it, and reads every other operand through `relative`, the one door between two operands of one space. A construction computes in its own coordinates. The frame is keyed with the inputs, so an edit that leaves the bodies an operation reads where they stand in their space moves none of its bits (D9). The frame is no part of the operation's meaning: the body up to that rigid map, its names and every verdict outside the sliver band are the same in any frame, and a minted reference direction is a function of the inputs (PR 5547). The census at a `⊔` is order-free: each pair's verdict is the same in either operand's frame, or the sliver band refuses.

**Repetition** (G; D10 `[repetition]`). `k = index(N)` defines a `Count` over `0..N`. A definition or operation whose reads reach `k` is evaluated once per value, and its outputs are families keyed by index tuples, outer first. A pattern is a placement whose reads reach an index, into the target space it names, the index entering as a value (`Spin(Z)` of `scalar(k)·turn/N`, a slide of `scalar(k)·pitch`) or through a mate's target (`bolt.axis ≡ holes[k].axis`); its copies are a `Bodies` family in that space, read by `p[k]`, and combined like any family: `⊔ p` keeps them as pieces (each member its own arrival at the census), `∪ p` fuses them. No pose is constructed from an index. A mirror image is `Mirror { body, plane }`, a construction. The presets are the façade's, and the GUI recognises one for display only. An assertion whose reads reach the index is one assertion per member (D10 `[assertions]` "Families").

## 2. PR A — `poses-are-variables` (cost H; ~150 files)

**Kernel**

1. `VarDef::Pose(PoseDef)` with the arms of §1 except `Carried`, which is B's.
   - Each arm states its result kind, and the door checks it as `SlotVarKind` checks a slot. `InFrame`'s coordinates read scalars of the right dimension.
   - `VarDef::kind()` reads the arm.
   - A pose definition may not reach its own variable (VR3's acyclicity walk).
2. **Evaluation.** A pose's value can depend on built geometry (a face's plane reads a `Face`), so a pose definition is bound when its reads are, at the lane scalar, by one evaluator `eval_pose` that replaces `wire_datum`. Its helpers are today's `frame_axes`, `frame_value` and `frame_plane_lane`. The schedule treats a pose definition as a node of the dependency graph over `Doc::upstream`: stage 2 D's machinery for observed definitions, generalised to "a definition bound mid-evaluation".
3. **`AxisInPlane` retires into `Revolve`** (FORK-1b). `Revolve` gains `axis_origin: [S; 2]` and `axis_direction: [S; 2]` in its profile's axes, and its `axis: Axis` port is the lift. `wire_revolve` reads the slots. The F15 doc on `AxisInPlane` moves to `Revolve`'s fields, still saying why: four numbers in the plane cannot leave it.
4. **`FaceFrame` retires from its plane readers.** A face reads as a plane (FORK-S3P round 10), so a reader that reads it as a plane reads `Plane { face }`. One read as a frame stays a `Datum::FaceFrame` until C (a frame mate) or D (a sketch on a face) restates it, and D removes the last with `Datum`. A reader that needs a frame on a face reads a constructed one (`Through`). DM1's sketch-on-face is D's: a construction placed against the face's plane, its spin a value.
5. **`Split`'s tool** reads a `Plane` pose whose reads reach its operand's root alone. A tool reading a pose of another space refuses `SpaceMismatch` from B on. Until D, an absolute `Datum::Plane` output is also a `Plane` pose (stage 2 A's port), of the document's one kind like every construction on a datum (§3's interim).
6. **One `Subgroup`.** `PoseSymmetry` moves from `DatumValue` (`mate/coset.rs`) to `PoseValue`, with `Point` (spherical) and `Direction` arms. `VarKind::symmetry` has no callers in src today, and the two tables are kept in step by a test (`intent_s2_a_outputs.rs`); A folds them into one.

**Migration** (a one-time regenerate):

- `AxisInPlane` becomes `Revolve`'s slots;
- `FaceFrame` with zero spin, read only as a plane, becomes `Plane { face }`. One read as a frame (a sketch on a face, a frame mate) stays a `Datum::FaceFrame` until D or C restates its reader. The migration's report names each.

Every body digest is bit-equal.

**Sites.** `axis_in_plane(` 45 in editor-core tests; `Datum::FaceFrame` constructions in src, tests and viewer (re-grep at stage 2 E's merge); `wire_revolve`; `wire_split` and `verbs/split.rs` `plane_of`.

**Goldens.** The `AxisInPlane` and `FaceFrame` rows of `crates/editor-core/tests/golden/slot_tables.txt` become pose-definition and `Revolve` rows; ids move, re-blessed with `M4_PR6_BLESS_GOLDEN=1`. Every f64 geometry digest is bit-equal.

**A′ (FORK-MOVE), after A and before C and D.** A landed `InFrame`/`PoseCoords`, `Project`, `Standoff` and `Revolve { axis_origin: [S; 2], axis_direction: [S; 2] }` (`crates/editor-core/src/pose.rs`, `eval/wire/pose.rs`, `node.rs`). A′ restates them as §1 now says: `Slide`, `Spin`, reads that carry a projection role (adding the x and y roles today's projection arms lack), and `Revolve`'s `axis_through`/`axis_heading`. It builds VR6's constant `0` (an `Expr` leaf of any dimension; `Formula::slot_root` lowers a lone `0` to it, where today it mints a free `Scalar`) and makes `equal_variables` (`crates/viewer/src/props.rs`) compare VR8 lowerings, so typing a lone `0` draws no offer and a default `0` is never the one offered. Its migration computes each angle by `atan2` and each length by projection once; a datum that was not axis-aligned moves by rounding, reported. Tests: a self-read through a role is one reader at the edit and load doors; a frame's x and y roles read; the tilted seat plane `Spin { Plane { face: top }, about: Axis { of: front edge }, by: 15° }` evaluates, and its `15°` is offered only free `15°` values.

**The symbolic tier's right angles** (stage 4 D/H, independent of A′). `Expr` evaluates `turn` to `2π` with π an indeterminate in `Sym`, and `sin_cos` mints an opaque atom that folds only at a zero form (`unary_at_zero`, `crates/geom-core/src/sym.rs`), so a copy spun by `turn/4` has faces the tier cannot prove parallel to its target. One fold rule beside `unary_at_zero`: `sin`, `cos` and `tan` of a rational multiple of π fold where the value is rational. A typed `90 deg` binds its nominal in radians (VR8) and is not folded unless its nominal is bound from its written unit; the unit owning the fold decides that.

## 3. PR B — `a-placement-is-the-bundle-of-mates` (cost H; ~260 files, 8–11k lines)

The largest unit. Its size is in the gauge sites, which are compile-driven, and in the refactor doors, which are not.

**Kernel**

- **`Node::Place { shapes, constraints }`** per §1. In B a constraint is today's mate payload (`Alignment`: two `MateFrame`s, a `MatePrimitive`, an `AxisSense`), numbers included, addressed by its minted id. That is a migration interim, not a final state, and C ends it.
  - A constraint reads the copy's side through the shapes it copies and, on the other, a pose outside the copy: another placement's copy, a body of the space the copy joins, or the world.
  - The door refuses a constraint with no side on the copy (`MateFault::NotOnTheCopy`) and one that reads its own copy (acyclicity over `Doc::upstream`).
  - **`FrameBase::World`** joins `FrameBase` for B's migration, so a gauge root's world pose is one mate to the world. C deletes `FrameBase`.
- **`Carried { copy, pose }`** (§1) joins the pose arms.
- **The world.** `Node::World`, minted with the document, undeletable (`EditError::WorldIsUndeletable`), defining no variable, and read only by a placement's constraints and by export.
- **The solve** (`mate/solve.rs`) is per placement. Each bundle folds through `fold_pair`'s coset intersection, and a copy's pose is its target composed with the folded representative.
  - **Retired:** `solve_group` and its BFS; `groups`, `root_and_cause`, `root_of` and `places`; `group_frame`, `gauge_chain` and `gauge_frame`; `MateRole`, its Python door and 93 test mentions; `Unplaced::{NoOffset, DeadGauge}`; `MateFault::Under`, since an under-pinned copy is a loose copy.
  - `SolvedPoses` keeps `placement` and the A2a pairing door, keyed by placement.
- **A mate beyond the pin refuses** (#4326: there is no interim that verifies a mate). At the insert door, a constraint added to a bundle whose fold is already `Trivial` refuses `Overconstrained`. The solve, for a state the door did not see (a load, a rebind), refuses naming every constraint whose removal leaves the fold's codimension unchanged, never "the later one": constraints have no order. F extends the rule to every constraint, pinned or not.
- **A loose copy.** A bundle that pins less than its body needs makes a loose copy, a root of its own kind (§1), not a refusal.
- **Gauges and offsets retire:**
  - `Node::Gauge` and `InstantiatePart.gauge`/`offset` (the serde `present` door in `persist/wire.rs`);
  - `DocEdit::SetOffset`, `SetGauge`, `Promote` and `Fold`;
  - `regauge_then_mate`, `clear_joined_offsets`, `mate_that_would_start_placing` and `check_gauge_ref`;
  - the thirteen gauge `EditError`s and `Maintenance::OffsetCleared`;
  - `check_offsets` and `MateFault::OffsetDisagrees`/`OffsetUnchecked`/`OffsetCheck`/`OFFSET_RECOURSE`;
  - `GaugeRefFault` (`doc.rs`), the load walk's gauge check (`persist/check.rs`) and `SnapshotError::NotAGauge`/`GaugeCycle`.
- **`PlaceInWorld` retires** into `Place` with one mate to the world. Stage 2 C's product gather reads the placements whose space reaches the world.
- **Spaces are kinds** (§1). The kind walk replaces `Product::spaces`/`own_spaces`, `gate_spaces`, `Evaluation::across_spaces`/`unplaced`/`unplaced_below` and `CarriedUnplaced`, read from the recipe instead of gauges. `NodeErrorKind::Unplaced` becomes `SpaceMismatch`.
  - **Interim until D.** Every construction that reads no body, and every absolute datum, is of one kind, the document's own, because today they all share world coordinates. So `union(box on datum P, cylinder on datum Q)` still evaluates, and test 7's check holds. Copies take kinds from their bundles as §1 says. D lands "a body's kind is its root" for constructions, with the migration that places each one (§5).
- **A9** (`relative_freedom_components`) runs over `Doc::upstream` alone. The gauge edges stage 2 F kept (its Q3) are gone with gauges.
- **An instance is a placement** (§1). `eval/parts.rs` delivers one output per world placement of the part, keyed by that placement, and no `frame` port.
- **Refactor** (`refactor.rs`). Split cuts a set of placements and the bodies they read; inline puts an instance's part's placements back as copies.
  - **Retired:** the anchor vote, `TwoAnchors`, `UnplaceableRoot`, `UnplacedAlone`, `SeveredGauge`, `DeadGaugeReference`, `TornGroup`, `PlacingMateLeft`, `WouldStartPlacing`; inline's `UnplaceableFrame`, `MatePlaced`, `Unplaced`, `MovedMemberOffset` and `PartDeadGauge`; `gauges_first` and the offset re-statement.
  - **What replaces them.** A cut takes whole placements. A constraint reading across the cut re-points to the same pose as the instance's output carries it (`Carried`), or refuses as a severed read.
  - **Acceptance** keeps A4's form: split-then-evaluate equals unsplit evaluation at structural and name identity, and inline-of-split returns the document up to minted ids.
  - `split-and-inline-over-a-mate-read-at-a-union-are-unmeasured` gets its rows here if stage 2 F did not land them.

**Migration** (a one-time check, not a rule). Regenerating a pre-B file:

- **A group root on a live gauge chain** becomes `Place { shapes: [body], constraints: [Frame coincidence of FrameBase::Part(copy) with FrameBase::World, offset = gauge chain ∘ root offset] }`. The turntable (`demos/tour/src/assembly.rs`) keeps its angle variable inside the chain. C restates these mates over geometry plus values.
- **Every tree mate** goes into the bundle of the copy it determined: the child in today's BFS from the root. So every pose is reproduced by the fold that produced it, and the tree's member-key order is the order the migration orients mates by.
- **Non-tree (declaring) mates** become assertions where a stage 2 D measure states them, and are otherwise dropped. The report names each.
- **A non-root member offset** (a check under A11 (2)) is dropped and named. Stage 5's `Assert` is the place to say it again.
- **An unplaced group** becomes copies placed against each other, not the world, so it is a loose space.
- **The check** (test 7): every corpus document's `SolvedPoses::placement` for every copy, and its product digests, are unmoved or moved by rounding, and the report names every move.

**Surfaces.**

- Python: `Doc.regauge_then_mate`, `Doc.offset`/`gauge`, `Node.gauge`, `DocEdit.set_offset`/`set_gauge`/`promote`/`fold` and the `groups`/`root_of`/`reading_edges` functions go; `Doc.place(shapes, constraints=[…])` and `Doc.world` arrive; `test_gauges.py` is restated as placements.
- Viewer: there is no gauge UI, and its exhaustive arms follow. The mate tool commits into a `Place`'s bundle; across two unrelated spaces it first places one copy relative to the other, which answers `vseam/the-mate-tool-across-two-gauges-commits-a-declaring-mate`. A gesture may author a mate ("place where shown", `offer/viewer-free-move-and-place-where-shown-over-a-whole-group`), but the mate reads poses off geometry and the numbers are values the user supplies; the viewer's display location is never read.
- Tour: `assembly.rs` (43 gauge lines) and `bench`.
- Docs: `docs/guide/assembly.md`'s gauge sections.

**Sites.** `[Gg]auge` lines at the first baseline: 615 src, 803 tests (34 files; `p2_split.rs` 226, `p2_gauges.rs` 136, `p2_gauge_offsets_and_spaces.rs` 110, `p2_promote_fold.rs` 100), 37 viewer, 34 tour, 132 pncad-py src, 73 `.pyi`, 93 Python tests. `SetOffset` 19 + 76, `set_gauge` 113 in tests.

**Closes:**

- `a-declaring-mates-alignment-is-never-read`: there is no declaring role.
- `msolve/gauge-of-recomputes-the-clusters-per-placement-lookup`: stale (`gauge_of` is a test helper), and the clusters are gone.
- `msolve/a-placer-row-states-what-a-poisoned-row-cannot`: `PlacerRow` and the walk through placers retire.
- The offset half of `mate-offset-verified-…`; the row closes at F.

## 4. PR C — `a-mate-relates-two-poses` (cost H; ~160 files, 5–7k lines)

**Kernel**

- **A mate is `{ on, to }`** (§1). The door refuses a kind mismatch (`MateFault::KindsDiffer { on, to }`) and an `on` that does not read the copied shapes.
- **Values** (§1; ruling C-2): `Constraint { id, kind: Mate(Mate) | Value(Value) }`, `Value { mate, coord: Slide(X | Y | Z) | Spin, by: S }`, added by `DocEdit::AddValue`. The door (the named interim C-3) refuses what the recipe shows: a value naming no mate of its bundle (`ValueNamesNoMate`), a freedom its mate's kind does not leave (`NotAFreedom { kind }`, e.g. a plane mate's `Slide(Z)`), a lone point mate's rotation and a target read off two bodies (`NoChart`), and a number of another dimension. The placement refuses at evaluation what needs the bundle's residual: `Overconstrained`, `Contradictory`, a freedom the bundle's other constraints already fix (`NotAFreedom` with no kind), and a chart the band cannot turn (`PoseDegenerate { Chart }`). Until F, the refusals that need the bundle's residual are evaluation faults. The chart is one function (§1; its rule is now the least turn from a written orientation, `the-value-chart-reads-a-written-orientation`), and the solve places the folded copy at its values spins first, then slides, inside the folded residual.
- **The "place part" gesture** (`place_at`): a plane mate on the copy's first planar face to the world plane that face lies in at the pose `P`, and the three values computed from `P`; a copy with no planar face falls back to `pin_at`, and the gesture says so. `pin_at` is the migration's, tests' and demos' helper, not a gesture: a `Frame` mate read off the copy's first axis-bearing edge and a vertex off it, equated with the world's frame slid and spun to where that frame sits at `P`. Both prefer an anchor whose definition reaches no toleranced variable, decided from the recipe (`mate::anchor`: an extrude's start cap reads its profile's frame alone), and report which they took (`Anchor`).
- **A copy standing where its body stands** (a named interim, the orchestrator's ruling; `work/intent/a-copy-standing-where-its-body-stands-has-no-d10-spelling.md`): `ConstraintKind::WorldPin(WorldPin::Identity)`, written by `DocEdit::PinWhereItStands`, a world pin with no read and no number that places its copy at the identity of the world's coordinates in every lane, so the copy follows every variable its body reads. A mate reads a pose off the copy's geometry against numbers from one evaluation, so a copy whose body is built in world coordinates through a variable would be pulled back toward the nominal and widened by the variable's width. It stands alone in its bundle (`Overconstrained` beside anything else); split carries it; inline refuses it on a moved landing (`InlineError::WorldPinOnAMovedWorld`). The migration, the tour, the demos and the fixtures place a copy at the identity by it; the gesture and `pin_at` place at any other pose. D retires it (§5).
- **The unobservable-value report** (`unobservable`): a pose move inside its pose's own symmetry, and a spin value about a line its placed copy is round about.
- **Retired** (`mate.rs`, `mate/solve.rs`):
  - `Alignment`, `MateFrame`, `FrameBase` (with B's `World` arm), `MatePrimitive` and `AxisSense`;
  - each `Datum::FaceFrame` a frame mate read;
  - the clocking rider (`Alignment::clocking`, the `FrameCoincidence` rider arm of `mate_coset`, `Refuted::ClockingRedundant`);
  - `PlanarRest.offset`, `MateFrame::authored` and `table_gap`;
  - the `Offset { frame, (dx, dy, θ) }` mate target, a second way to write three values (FORK-S3M);
  - `SlotId::MateFrameStep` and `VectorSlot::MateFrame*`.
- `mate_coset` reads the two sides' `PoseValue`s and their common `Subgroup`, the table's row by kind. The face-base read (`resolve_side`, `topo::readback::face_pose`) moves into `eval_pose`, so it runs once per definition, not once per mate.
- **The world as a target** (Q7): a mate's `to` may read the world through any pose read the world offers and the target constructions, inside the mate (`WorldRead`). Written inside the mate, a world read nests in the saved body, so the door bounds its depth (`MAX_WORLD_READ_DEPTH`, fifteen: the gesture's nine and inline's six appended moves; `WorldReadFault::TooDeep`), as an expression's is bounded, and the load door's envelope is the deepest such read.
- **Analysis.** A value is a slot variable, seeded and boxed as any slot is, through `Doc::reads`. The solve's lanes (MSOLVE-14) are unchanged.

**Migration.** Each mate is restated over geometry plus values:

- A `FrameBase::Face` side becomes the face's `Plane` (and, where the primitive was coaxial, the carrier's `Axis`). Its offset's in-plane translation and roll become values, computed once from today's solved relative pose.
- A `PlanarRest.offset` of `h` becomes a standoff, `Slide { plane, along: plane through normal, by: h }`, on the target, `h` a new anonymous `Length`.
- A rider `θ` becomes a spin value.
- A clocked coaxial (today's `Coaxial` plus rider, residual `Prismatic`) becomes an `Axis`–`Axis` mate plus a slide and a spin value.
- An `AxisSense::Opposed` side becomes `Flip` on the copy's side.
- A side written against a part frame (`FrameBase::Part`) or an authored vector is absolute coordinates. The migration restates it over geometry plus values where the copy has a planar face or a carrier axis that pins the same coset; otherwise it drops the mate and names it in its report. B's `FrameBase::World` mates are restated as world reads (Q7) with the same arithmetic. A copy whose geometry offers no pose of that coset is never dropped, because dropping a world mate would empty the product: the migration names it and refuses to regenerate, as Q8 does.
- A part-frame mate to the world at the identity (B's `FrameBase::World` pin with an empty chain, a copy standing where its body stands) stays the identity pin until D: it is restated as the named interim's world pin (`DocEdit::PinWhereItStands`), which reads nothing, not as a world read over geometry, which would read the copy's geometry against the nominal's numbers. §5's "whatever placed it in the world keeps placing it" is this pin.
- **The check** (test 8): every MSOLVE fixture's solved poses agree with pre-C to within the stated rounding of one value re-composition, and every dropped mate is named. A pose that moves by more is a bug.

**Surfaces.** The viewer's mate tool authors `Plane`/`Axis` reads, `Flip` and values: `MateChoice { primitive, clocking }` becomes `{ kind, flip, values }`. Python's `MateFrame`, `Alignment` and `Mate(...)` (54 lines in `test_assembly_author.py`) take two poses and values.

**Sites.** `Node::Mate {`: 53 src, 115 tests, 19 viewer, 6 tour, 2 pncad, 2 pncad-py. `MateFrame::` constructions: 11 src, 114 tests, 16 viewer, 9 tour, 9 py. `Alignment {`: 23 src, 82 tests. Re-grep at B's merge, which moves each under `Place`.

**Closes:**

- `a-clocking-rider-is-levered-unreduced` (the rider is gone);
- `a-face-frame-cannot-turn-its-roll` (a roll is a spin value);
- `a-face-base-puts-its-reference-on-local-y` (no reader takes a carrier's reference direction);
- `mate-primitive-unit-variants-load-from-a-null-payload` (`MatePrimitive` is gone);
- `a-mate-frame-axis-is-decided-against-a-length-band` (no mate frame is composed);
- `a-mate-frame-is-written-in-the-reading-instances-coordinates` (a mate reads poses off geometry, so its meaning does not depend on the reading instance);
- `recipe/placement-step-slots-are-spelled-three-ways` (the offset steps are values; one spelling).
- MSOLVE-15 (#3681, rider, `Clocking` and stand-off deletion) is subsumed; its PR is closed with a pointer here.

## 5. PR D — `transform-retires-into-a-placement` (cost H; ~200 files, 6–8k lines)

Frames enter only at placement (FORK-S3P), and nothing moves a body (FORK-S3M).

**What D built, where this section differs.** The record is the draft PR body in D's item.
- A copy standing where its body stands is `mate::pin::stand`, which writes the identity bundle. The migration below did not write a world-read `Plane` mate carrying the datum chain.
- A world mate's values are charted from the world's own frame: ruling (A), pinned by `a_datums_in_plane_offset_rides_on_the_values`. A datum's chain therefore splits: its in-plane part goes onto the values, the rest onto the world read.
- A plane-less root is placed on its own axis only by `place_at` and `place_against`.
- An extrude goes up its own +z; `ExtrudeSide` retires.
- Inline at a moved landing re-pins a bundle of numbers and refuses one that reads a variable.
- A turn written as a rational fraction of a turn is exact in every lane.
- FORK-REST holds.

D′ (§5′) ends each of D's interims.

**Kernel**

- **Constructions read no frame** (§1): `ProfileProgram.plane` goes and a profile is 2-D; `Tube` and `HollowTube` lose `frame`; `Extrude` loses `side` (and gains the slant's 2-D direction slots only if a reader asks, Q1); `tube_args` and the `frame_plane_lane` call sites in `eval/wire.rs` follow.
- **`Datum` retires** (`node.rs`, `Node::Datum` and its arms, the last `FaceFrame` with them; `Node::outputs()`'s port arms). The six datum constructors in `pncad-py` become pose-definition builders where a pose is read off geometry, and placement builders where a datum sat a construction in space.
- **`Node::Transform` and `PortKind::PlacedFrom` retire** (`wire_transform`, `placeable_operand`). A rigid motion of a body is a `Place`. "You can't Transform an already placed part" (Ev) is unrepresentable: a copy is defined by its one placement and never moved after.
- **Names.** A copy's names are qualified by its placement (`RoleSeg::Placed`, B's), once per copy by construction, so two copies of one body name apart; a selector reading a body a `Transform` moved spells the copy's `Placed` layer. DM3's sentence on a member read loses its comparison with `Transform`, and keeps what it decides.
- **The member walk** (`mate/member.rs` `transform_map`, `Placing::Transform`) has nothing left to walk through.
- **`placement::Frame`** stays as the evaluated value type. `Step::Literal` retires: no document holds a literal frame except `PatternKind::Explicit`'s, until G, so A6's admission (`Frame::admission_fault`) runs on evaluated pose values in `eval_pose`. A mirror is G's construction, so no placement needs an improper frame.
- **Sketch on a face.** The façade's gesture writes profile, construction, placement (a `Plane` mate to the face's plane, the side a `Flip`, its slides and spin values) and combine, as one gesture.

**Migration.**

- **A construction on an absolute datum** is rebuilt in its own coordinates: its 2-D profile is the datum-plane coordinates of today's profile, so its geometry is today's mapped by the inverse of its datum. Then:
  - **combined with another body**, it is placed against that body, the combine's first operand's root, by a `Plane` mate from its own profile plane (`Flip` on the copy's side) to the target's datum plane, the target side carrying `datum_first⁻¹ ∘ datum_this` as that chain's own moves (its slides and spins over the document's variables, verbatim), with the constant `0` on the freedoms left;
  - **the first operand of a combine, or never combined**, it is placed nowhere new: C's interim identity pin (`pin_where_it_stands`, the successor of B's migrated world mate and a former `PlaceInWorld`) becomes a `Plane` mate from its own profile plane (`Flip` on the copy's side) to a world read carrying the datum's move chain verbatim, with the constant `0` on the freedoms left, and the interim retires;
  - **every value charted on a re-coordinated body** (C's world-mate values, any mate value naming it) is restated over the datum chain's variables, never recomputed from the pre-D solve into constants: a toleranced datum keeps its tolerance axis, and no pose moves at any value of any variable (test 10).

  - **the freedoms the mate leaves** are charted from the target body's frame (the world's is the identity), not from the moved read. So the chain's component along each left freedom goes onto that freedom's value, as an expression over the chain's variables; the rest of the chain goes onto the target read as moves, verbatim where it separates. The constant `0` is written only where the chain has no component on that freedom.

  **A construction with no plane of its own** (a full revolve, a full-ring tube) is placed the same way by an `Axis` mate from its own axis: a revolve's `axis` output, or a ring's carrier axis, which reads nothing. The target carries the revolve's axis slots as moves reading the same variables, beside the datum chain. A constant slot writes no move. A slot that is a typed number becomes a variable with two readers, and the migration names it (VR7) and reports the name. Writing `0 mm` where the axis passes through the profile's origin is a helper defect; it is restated as the constant zero, so the corpus's revolves read nothing on their axis. (Fork-log row 161.)
- **A loft's sections** are 2-D profiles. Each section after the first carries a station, a `Slide`/`Spin` move chain in the first section's own axes. A section is migrated as `datum_0⁻¹ ∘ datum_k`, over the datums' variables verbatim, and `Datum::Frame` retires with the rest (row 161).
  
  No construction is placed against the world by this migration, so a cutter that is only subtracted never joins the product. Product body digests move by one composition (the construction now computes in its own coordinates) and the world placement compensates it, so export moves by rounding.
- **The restate ladder**, one for a construction's placement and for an absolute pose a non-root operation reads (a `Split` tool): a `Plane` mate to a parallel planar face of the target, at a standoff where the planes differ; else a mate to a frame constructed from the target's carriers, slid and spun, each angle computed by `atan2` and each length by projection once, `0` written where the old document wrote nothing, and poses moved by rounding reported; failing both, the migration names the node and the document refuses to regenerate (Q8).
- **A `Transform`** becomes a `Place` of its input with one mate to the target plus values computed from its chain.
- The tour: `chain.rs` (links), `diefillet.rs` (a pip ball per transform, then a union; the copies are placed against the die's faces, so they are of one space and the union stands) and `teapot.rs` (the spout).

**Sites** (counts at the first baseline):

| Area | `Datum::` constructions | Helpers | `Node::transform(` |
|---|---|---|---|
| editor-core src | 21 [71] | `test_support::frame`/`xy_frame` | |
| editor-core tests | 145 [19] | `xy_frame()` 168, `on_frame(` 231, `on_frame_keeping(` 28 | 98 |
| viewer | 10 + 22 [15] | `xy_frame()` 30, `framed_square(` 46; `session/author.rs` 6 | 20 (tests) + the transform gesture in `session.rs` |
| tour | 18 + 3 | teapot 4, diefillet 3 | chain, diefillet, teapot |
| pncad | 4 (tests) | `xy_frame()` 13 | |
| pncad-py | 9 in src; Python `.sketch_frame(` 127, `.datum_*(` 75 | | 1; Python `.transform(` 26, `.transform_by(` 6 |

The helpers become "build in own coordinates, then place", returning the placement's copy.

**Goldens.** Re-blessed: datum and transform nodes leave the node table and every downstream id moves. Body digests move by one composition where a construction was built off-origin; the migration report states each, and test 10 checks each against the old digest mapped through the placement.

**Closes** `topo/a-boxed-rotation-refuses-not-rigid-at-every-placer`, its placement half: a placement's motion is a fold of mates and values, rigid by construction. The boxed `Pattern` angle half goes to G.

## 5′. D′ — the placement pivot, in three PRs (cost H each)

D′ builds D10 `[placement]`, `[operations.operators]` and `[variables.kinds]`' *Spaces* as rows 189–191 left them (PR 5494), with the helpers ruling, row 190's chart and ASSEMBLY A4's inline. Its umbrella item is `a-placement-pins-by-written-values-without-a-mate`; each PR is its own unit. All three are re-grepped at dispatch against the merge of the one before.

**What D built, and D′ keeps.**
- `Value.mate` is optional. `Coord` holds `Slide(TargetAxis)`, `Spin` and `SpinAbout(TargetAxis)`.
- The door rule for a bundle with no mate is in `mate::solve`: `unpinned` (`mate/solve.rs:990`), `unmated_bundle`, `identity_bundle`, `overconstrained` and `MateFault::Unpinned`.
- A bundle with no mate is evaluated by `mate::value::unmated_motion` (`mate/value.rs:310`).
- `DocEdit::AddValues` and `DocEdit::RemoveConstraints`.
- `mate::pin::Placed::{identity, write}` (`mate/pin.rs:432`), and `stand` (`:675`), which writes the identity bundle.
- FORK-REST: `SolvedPoses::at_identity` (`mate/solve.rs:293`) is a recipe query.
- Exact rational turns: `geom_core::Angle`, `VarEnv::turns` and Sym's `trig_at_rational_pi`.
- The writers of exact frames: `corner_frame`, `moved_frame` and `FrameMove` (`mate/pin.rs:975–1105`).

The paths below are under `crates/editor-core/src/` unless named otherwise; line numbers are main's at `4a4bb23608`.

### 5′.1 D′1 — `a-placement-names-its-target-space`

**Builds.**

- **The kind.** `VarKind::Space` (`var.rs:50`). A `Space` variable is an output (`VarDef::Output`), of exactly three definers:
  - `Node::World` (`node.rs:2014`) gains its one output, the world's space, at a port every document holds (it stays off the wire, so the variable is minted with the document);
  - `Node::Space` (new): reads nothing, defines one `Space`; `new_space()` in the façade and Python writes it;
  - each construction gains a `space` output beside its body (`Node::outputs`, `node.rs:2999`): `Extrude`, `Revolve`, `Tube`, `HollowTube`, `Loft`, `Sweep`, the list `root_space` holds (`eval/mod.rs:3396`).
  No other slot admits `Space`, so a construction cannot read one; the door refuses `SlotVarKind` as for any kind.
- **The target slot.** `Node::Place { shapes, target, constraints }` (`node.rs:2003`) and `Node::InstantiatePart { doc_ref, interface, target, constraints }` (`:2101`). `target` is a `Space` read, always written: `Node::place` (`:3712`) and `DocEdit::place` (`edit.rs:740`) take it, and the façade and Python's `place` (`pncad-py/src/py/doc.rs:1684`, `:3069`) and `instance` require it. Re-targeting is a `SetParam` of the `target` slot with a `Space` read (`edit.rs:243`), the operand-write door, which reports and never refuses what it strands.
- **Static space derivation.** `Doc::space_of(var) -> VarId` (new, beside `Doc::placements`, `doc.rs:1080`), a recipe walk in schedule order: a construction's body is in its `space` output, a placement's copy in its `target`, an operator's result in its operands' space, a one-input operation's in its input's; a profile or pose is in no space. It replaces `spaces_of` (`eval/mod.rs:3422`), `root_space`, `Evaluation::loose` (`:127`), `Evaluation::space` (`:343`) and `across_spaces` (`:332`), whose readers (`resolve/pick.rs:1644`, `clearance.rs:1299`) ask `space_of` instead. The space a value lives in is no longer stored on `NodeValue` (`eval/wire.rs:88`, `moved`).
- **`SpaceMismatch`, decided from the recipe.** `Doc::space_faults` (new) names every node whose reads lie in two spaces: an operator's operands, a `Measure`'s two sites, an assertion's `Gap`, and a mate's `to` not in its placement's target. Three askers:
  - the edit door refuses a node `insert_into` (`edit.rs:6662`) would write with one, `EditError::SpaceMismatch { a, b }`, and so do the façade's builder and Python, which go through it;
  - a `SetParam` of a `target` that breaks a downstream read is admitted, and the reads it breaks are unresolved and reported (`Maintenance::Strand`);
  - `validate_document` (`persist/check.rs:416`) does not refuse it (§13 states why): a file in that state loads, and the node refuses at evaluation, naming both spaces.
  Evaluation keeps `NodeErrorKind::SpaceMismatch` (`eval/mod.rs:1997`, asked by `input_space`, `:1396`) as the refusal of a marked node, its two spaces now `Space` variables, and `input_space` reads `space_of` rather than its inputs' values.
- **`mate::solve::Space` retires** (`mate/solve.rs:149`): a space is its `Space` variable. `Space::Loose`, `Folded::Loose` (`:1114`, `:1173`, `:1183`), `PoseRefusal::Loose` (`:327`) and `MateFault::TargetsDiffer` (`mate.rs:1199`, raised at `eval/wire.rs:788`; `eval/class.rs:199`) retire: the fold reads every mate's `to` in the target space (`fold_with`, `eval/wire.rs:663`), which the door has already checked.
- **Every bundle pinned.** `unpinned` checks every bundle, with mates or without, and refuses an empty one. With it retire `Evaluation::loose_copies` and `loose_roots` (`eval/mod.rs:132`, `:353`), `product::loose_spaces`, `LooseSpace` and `Product::spaces` (`product.rs:888`, `:900`, `:865`), `ProductError::EmptyProduct`'s `loose` field (`:131`), `ExportError::Loose` (`pncad/src/export.rs:68`, raised in `step_for_node` `:150` and `export_document_step` `:202`), `refactor::inline`'s `in_loose_space` (`refactor.rs:3921`), and pncad-py's `Space::Loose` (`py/value.rs:1792`, `:1843`; `py/mate.rs:976`; `product_memo.rs`; `tags.rs`).
- **The viewer.** It shows a set of spaces, each at a display frame kept as display metadata outside the canonical bytes (IDENTITY DI5): by default the world and every space holding a body no operation reads as a body (an operand, a one-input operation's input, a placement's source; a member read takes over that member alone), then the author's show and hide. A new construction appears at once in its own shown space, tinted as outside the product, and once placed its own picture goes. `world.rs`' `loose_root` (`viewer/src/world.rs:57`), `pickindex.rs`' drawn filter (`:1024–1033`, `:1314–1324`), `display.rs`' admission (`AdmissionFault`, `:176`) and the re-placement into a loose root by `place_against_in` (`viewer/src/session.rs:3797`) are re-keyed by the space variable; free-move goes over any shown space. Before E the picture reads a body's coordinates in its space as they stand, since D′1 holds every value in its space's coordinates. After D′3 the picture is the product slot's body plus the shown spaces (§5′.3, the viewer's contract, (c)).
- **Content key and wire.** `target`, the `space` outputs and `Node::Space` enter the wire, persistence and the content key (`tag::format::VERSION`, `eval/mod.rs:4474`, to 18).

**Interims D′1 leaves** (named in §0): `Target::World` stays, admitted only where the placement's target is the world, and `SpaceMismatch` otherwise; `Place` copies a list of shapes; the product is the gather of the placements whose target is the world (`product_in`, `product.rs:999`); an instance copies its part's world placements.

**Done when.**
- An operator over two bodies of two spaces refuses at the edit door, in the façade and in Python, `SpaceMismatch` naming both, with no evaluation run (the refusal comes from the edit, before any `Evaluation`).
- A `SetParam` of a placement's `target` from one construction's space to another's leaves the downstream union unresolved; the document saves, loads, and the union refuses `SpaceMismatch` at evaluation; re-targeting back resolves it.
- An empty bundle refuses `Unpinned`, and so does a plane mate with one of its three values.
- No evaluation of any corpus document names a space other than a `Space` variable, and `grep -rn 'Space::Loose\|loose_copies\|LooseSpace\|ExportError::Loose\|TargetsDiffer' crates/ demos/` is empty.
- A placement of `a` into `new_space()` with the identity bundle, and a placement of that copy into the world, put `a` in the product.

**Hit list** (by symbol; re-grep at dispatch): `Node::Place {` 76 src, 49 test, 27 viewer, 2 pncad, 5 pncad-py and 5 tour lines; `Node::InstantiatePart` 61, 40, 17, 2, 6, 7; `Space::World`/`Space::Loose` in 10 src files (`eval/mod.rs` 19, `eval/wire.rs` 10, `product.rs` 5, `refactor.rs` 4, `clearance.rs` 4, `resolve/hit.rs` 3, `mate/solve.rs` 3, `eval/class.rs` 2, `assembly.rs` 2, `eval/parts.rs` 1); the loose readers above; the viewer's `display.rs` (15 lines), `world.rs` (3), `session.rs` (3), `pickindex.rs` (1).

**Absorbs:** `a-bundle-folds-in-the-joined-space` (`TargetsDiffer` retires), `a-placement-relates-two-frames` (the ruling's target slot). Re-reads: `a-parts-loose-spaces-and-member-reads-drop-its-census-rows` (its loose half is gone), `a-fused-instances-held-frame-has-no-reset` (a loose instance), `the-world-still-speaks-of-roots`.

**Risks.** The derivation moves from values to the recipe, so a node that evaluated in one space and was recorded in another changes answer; the D′1 lane lists every corpus node whose space changes. A `space` output per construction grows every var table and moves ids.

### 5′.2 D′2 — `a-mate-reads-only-geometry-and-a-value-charts-on-the-target-space`

**Builds.**

- **No space has roles.** `Target::World` and `WorldRead` retire (`mate.rs:106`, `:348`, `:401–405`), so `Target` is a pose read. Each world mate migrates to values on the world's axes: a `Plane` mate to a world plane read becomes the mateless bundle its chain states (slides along and spins about the world's axes); the generators are rewritten to write the values (DESIGN Band 4), never converted from stored bytes.
- **A value names no mate.** `Value { coord, by }` (`mate.rs:734`; `mate` at `:738` goes). `Coord` becomes `Slide(TargetAxis) | Spin(TargetAxis)` (`:757`): the mated-direction `Spin` is `Spin(Z)` on the swung chart, and `SpinAbout` is `Spin` on the unswung one. `Coord::admitted_by` and `on_own_axes` become one question, `coord` against the bundle's residual family. `NoChartWhy::PointRotation`, `NoMatedDirection` and `BesideAMate` retire (`mate.rs:848–862`), with `MateFault::ValueNamesNoMate` and the `settle` ordering key that reads `Value.mate` (`mate/value.rs:363`).
- **The space chart.** The chart reads the target space's axes carried onto the one mated direction by the least turn from where the mate's orientation puts the copy's (`the-value-chart-reads-a-written-orientation`), not the body the `to` pose is read off: `chart_body` (`mate/solve.rs:1379`, asked at `eval/wire.rs:752` and `mate/solve.rs:1477`) retires, and `Chart::of` (`mate/value.rs:78`) takes the target space's frame. `NoChartWhy::TwoBodies` retires with it, since a `to` over two bodies of one space has one chart. Where the target body stands at the identity in its space, nothing moves.
- **A lone point mate's chain.** A point mate leaves its rotation to a chain of at most three spins about the target space's axes, through the mated point, evaluated as `unmated_motion` evaluates a chain; `PointRotation` retires at `mate/solve.rs:1199` and `mate/value.rs:374`. The bundle rule counts spins by axis wherever no direction is fixed, a point mate's chain included (`mate::solve::equations_of`, `overconstrained`, `:951`).
- **The helpers ruling.** `mate::pin` keeps one pure constructor of a bundle from the mates and moves its caller wrote, with the constant `0` on each freedom left unnamed, plus `stand`, `corner_frame`, `moved_frame` and `FrameMove`. Retired: `place_at`/`place_at_in`, `pin_at`/`pin_at_in`, `frame_at`/`frame_at_in` and `place_against`/`place_against_in` (`mate/pin.rs:200–1424`), with `picked_frame`, `world_at`, `written`, `moved_by`, `turns`, `written_moves`, `own_axis`, `axis_against`, `framed_against`, `axis_in_world`, `axis_placed`, `moved_pose`, `target_own_plane`, `PlacedBy` and `copy_of`'s motion test (`:96`), and `mate::anchor::Anchors` with `part_start_cap` (`mate/anchor.rs:43`, `:250`). The place-part gesture writes the identity bundle. A caller that meant a frame writes the moves it meant (`moved_frame`), or a mate and values.
- **Inline as one rule** (ASSEMBLY A4). `refactor::inline` (`refactor.rs:3855`) mints a `new_space()`, copies the part's nodes into the host verbatim with the part's world read as that space, and rewrites the instance as a placement into the instance's target under the instance's own bundle, verbatim. Until D′3 that placement copies the spliced copies of the part's world placements as a list; D′3 makes it the copied product body. No motion is composed and no landing refuses for its numbers: `Landing` (`:3820`), `InlineError::UnplaceablePart`, `PinReadsVariable`, `ValueOnAMovedWorld` and `RepinRefused` (`:1371–1467`) and `world_pin_of` (`:2730`) retire. A host mate reading the instance's copy reads the new placement's copy.
- **The flatten edit** (new, `refactor::flatten`). A placement whose bundle is the identity as written, and whose source is in a `new_space()` no other placement takes a source from, is flattened: every placement into that space is re-targeted to the placement's target, every read of the copy re-pointed to the source, and the placement and the space deleted. Anything else refuses, typed. Inline-of-split then returns the document split was given, after a flatten (A4's acceptance).
- **The viewer's mate tool** writes the mate and removes the values whose freedoms it fixes, as one edit (`viewer/src/session.rs:1763`), so a freshly inserted copy (identity bundle) can be mated.
- **pncad-py and the façade.** The world-read and helper doors go (`WorldRead`: 37 pncad-py lines, 3 pncad, 8 tour; `place_at` 14 pncad-py, 12 tour; `pin_at` 10 tour; `place_against` 4 pncad-py, 4 tour); `path_start_frame` (`pncad-py/src/py/place.rs:306`) is FRAME's to decide under this ruling (its two parked rows).

**Done when.**
- `grep -rn 'WorldRead\|Target::World\|place_at\|pin_at\|frame_at\|place_against\|Anchors' crates/ demos/` names no live symbol.
- The row-190 example: a boss mated to a side face of plate copy `B`, `B` turned 30° in the target space, `Slide(X) = 10`. The slide is measured from the target space's origin along its `x` as the least turn carries it onto the face's flipped normal, so editing `B`'s turn to 40° moves the boss across the face, and placing the boss against a face of `B ∪ C` that is the same face gives the same copy. *Breaks if* the chart reads the body the pose is read off (the boss would ride with `B`).
- A lone `Point`–`Point` mate plus `Spin(Z)`, `Spin(X)` and `Spin(Y)` pins its copy, turned in the order written with the mated point on its target; a second `Spin(X)` right after the first refuses `Overconstrained`, and so does a slide beside the ball, which fixes every translation.
- Test 21: a part whose placement's slide reads a named variable, under an instance turned by `turn/4`, inlines; moving the variable moves the spliced copy as before.
- Inline then flatten of a split returns the document split was given, up to minted ids and order.
- No helper reads a `Frame` value off an evaluation (a grep gate beside `scripts/gates/minted-reference-from-inputs.sh` on `mate/pin.rs` for `Evaluation`).

**Hit list:** `Value {` 61 src, 145 test, 46 viewer, 2 pncad, 22 pncad-py, 10 tour lines; `mate: Some/None` 17, 39, 5, 1, 6, 3; `TargetAxis::`/`SpinAbout` 72, 176, 36, 2, 17, 12; `Coord::Spin` 19, 70, 9, 1, 3, 4; `Target::World`/`WorldRead` 95, 147, 10, 3, 37, 8; the four helpers 62, 140, 20, 10, 19, 32; `Anchors` 11 src, 1 pncad-py.

**Absorbs:** `a-value-charted-on-the-target-reads-frame-carries-the-datum-chain`, `sym-solve-atan2-has-no-branch-rule-about-pi` and `a-slide-value-doubles-its-translations-width` (the chart is rewritten), `an-inline-at-a-moved-landing-reading-a-variable-refuses` and `inline-refuses-a-placed-part-off-the-identity` (inline is one rule), `the-viewer-cannot-mate-two-freshly-inserted-parts`. Re-reads: `a-lone-slide-refuses-the-chart-it-never-reads` and `a-slide-beside-a-ball-refuses-though-it-fixes-the-turn` (the chart's refusals are restated), `refactor-remaps-no-pose-definition` (inline copies nodes verbatim), `test-fixtures-write-a-placements-zero-values-as-literals`, `frame/path-start-frame-is-the-only-frame-from-a-tangent-and-is-named-for-the-start` and `frame/two-doors-answer-an-axis-off-a-tangent-with-no-rule-for-which` (FRAME's).

**Risks.** The space chart moves geometry wherever a mated target body is placed off its space's identity; the lane reports each by document and node, and checks it against the old chart composed with the target body's placement. A world read's move chain with more than three spins, or a slide after a spin, has no verbatim mateless spelling: its generator writes the motion it means, and a document whose chain cannot be written that way refuses to regenerate, named (Q8's rule). The box lanes' chart (`Chart::turned`, `mate::solve::tightened`, `spins_by_frames`) is rebuilt on the space frame; the two absorbed width rows are its measure.

### 5′.3 D′3 — `assemble-is-an-operator-and-the-product-is-a-slot`

**Builds.**

- **`Node::Assemble { members: Bodies<BodyRead> }`** (`⊔`, beside `Union`, `node.rs:1968`), a list like a union's. Its wire grafts its operands as separate pieces (today's gather graft) and runs the census at its door: `topo::record_at_rest` over the aggregate, one group per operand at one index value (an arrival), lifted from `product_in`'s call (`product.rs:1236`, `:1269`; `checks/at_rest.rs:468` `copy_groups`). Its result is a body of several solids, an ordinary operand: `(p ⊔ q) − t` cuts both pieces, and FORK-2b's refusal of a product as a boolean operand (`crates/topo/README.md`) retires. Until G, a `Pattern` or `PlacedUnion` operand's bodies are each their own arrival, a named interim G ends.
- **`∪` over a `⊔`.** A meeting inside an operand is carried, never re-decided: a union, cut or meet reading a `⊔` result keeps its pieces' records. A result that is one solid touching itself over an area refuses, typed, naming the pair (`crates/topo/README.md`).
- **One product slot.** `Doc` gains `product: Option<BodyRead>` (`doc.rs:809`), set by `DocEdit::SetProduct` (new), on the wire and in the content key. The door refuses a product read whose body is not in the world (`SpaceMismatch`). `product_recorded` (`product.rs:981`) reads the slot's value; the gather of placements (`product_in`, `:999`), `Doc::placements`' role in it and `doc.unplaced()` (`doc.rs:1094`) retire, and `EmptyProduct` is a slot that reads nothing. Export (`pncad/src/export.rs`) writes the slot.
- **One source.** `Place { source: BodyRead, target, constraints }`: `shapes: Vec<BodyRead>` goes, and a placement defines one `Body`. A list placement in the corpus becomes a `⊔` of the shapes, placed once.
- **An instance reads its part's product.** `InstantiatePart` defines one `Body`, a copy of the part's product body (`wire_instantiate_part`, `eval/wire.rs:876`; `PartValue`, `eval/parts.rs:110`), carrying its records and census decisions as `PartValue` does today.
- **Picks.** `VarDef::Pick { placement, of: Vec<VarId> }` (new): one piece of a copy, its `of` the path through the source's placements and `⊔` operands to the first variable it reaches. The door refuses a path through any other operator. Readers hold picks: a `BodyRead` has no key. Retired: `CopyKey` and `BodyRead.key` (`operand.rs:502`, `:560`), `SlotValue::Copy` (`edit.rs:45`), `Copies` and `KeyedCopy` (`eval/mod.rs:1279`, `:1290`; `ValuePayload::Copies`, `:1060`), `eval/wire.rs`' `copy_of` (`:2757`), `RoleSeg::Placed` and `RoleSeg::InPart` (`names/role.rs:1581`, `:1571`) with `StableName::copy_of` (`:2304`) and their minting in `names/emit.rs` (`name_placed`, `:1354`; `name_placed_copies`, `:1418`; `InPart` at `:1320`, `:1400`), and the `Placed`/`InPart` arms of every rewriter and reader. `RoleSeg::From`'s doc ("a placement adds no name segment") is then true. `name_words_corpus`'s `NAME_WORDS` and `OVER_BUDGET` re-tighten.
- **The two rows D deleted**, rebuilt over picks: `msolve1_transform_aware::a3b_a_placed_copy_of_a_pattern_seats_and_gates` and `msolve13_read_at_operand::a1b_a_placement_above_the_operand_mints_nothing`.
- **Inline** lands the copied product body as the placement's source (§5′.2's interim ends).
- **The viewer's forms** read picks (`viewer/src/session/select.rs`, `session/op.rs`, `app.rs`, `world.rs`).

**The viewer's contract after D′3.** Three sentences the viewer's authoring and drawing rest on. D′3 writes them; the rows named below build them.

- **(a) The takeover re-points along the seen path.** A gesture that takes a body over (a feature, the mirror, sketch-on-face, a combine) re-points whatever reads the picked body along the path it was seen through: a placement's source, a `⊔`/`∪` operand slot, or the product slot. This generalises the residue-1 gesture rule (INTENT-STAGE2, fork row 85) beyond placements. D′3's takeover re-points placement sources only, and it refuses one that would strand a pick (`Refusal::StrandedPick`: a pick is never re-pointed). The rest is built by `a-takeover-re-points-along-the-seen-path-and-a-creation-joins-the-product` (DOORS).
- **(b) Nothing joins the product unasked.** A new body lands unplaced (Ev, 2026-10-31), and the product reads world bodies, so a creation gesture writes no `SetProduct`; a body reaches the product through one door, *Add to the product* (`crates/viewer/README.md`, *A body reaches the product through one door*).
- **(c) The picture draws the product slot's body plus the shown spaces**, and a body the product does not read is tinted as outside it. This is built by `the-viewer-does-not-show-a-constructions-own-space` (INTENT, with E's drawing).

**Done when.**
- `(p ⊔ q) − t` keeps two pieces; `A ⊔ A` is an interference finding; `h1 ⊔ h2` of a split's halves records the cut and `h1 ∪ h2` glues it, with no `unproven-coincidence` finding on the cut.
- `(bolt ⊔ plate) ∪ bead` keeps the bolt–plate seat as the `⊔`'s record, cited, and the union refuses typed where the result is one solid touching itself over an area.
- Every corpus document's product slot reads a `⊔` whose census rows equal today's gather's, each move reported; a `⊔` the slot does not read is still checked.
- Two placements of one body into one space, unioned, name each operand by its read and carry no `Placed` or `InPart` segment; a pick authored twice is two variables; `grep -rn 'CopyKey\|RoleSeg::Placed\|RoleSeg::InPart\|KeyedCopy' crates/ demos/` is empty.
- An instance of a two-part document defines one body; a pick of one part piece reads it.

**Hit list:** `RoleSeg::Placed` 28 src, 37 test, 1 tour; `InPart` 63, 60, 4 viewer, 3 pncad, 3 pncad-py, 2 tour; `CopyKey` 43, 21, 7 viewer, 2 pncad; `shapes:` 14, 20, 10 viewer, 10 pncad-py, 2 tour; `product_recorded`/`product_named`/`product::product(` 21, 128, 6, 5, 8, 2; the readers outside `names/`: `coincide.rs`, `edit.rs` (`stranded_by_repoint`), `eval/parts.rs`, `eval/mod.rs`, `eval/class.rs`, `checks/at_rest.rs`, `product.rs`, `refactor.rs`, `resolve/mod.rs`, `mate/pin.rs`.

**Absorbs:** `a-space-names-its-bodies` (picks), `name-words-budget-grew-under-placed-copy-wrapping`, `a-placement-copy-is-its-own-table-in-the-product` (`Placed` retires), `an-instance-defines-one-body-per-part-placement` (an instance is one copy of a product), `the-viewer-authors-no-read-of-one-copy`. Re-reads: `an-instance-value-carries-its-copies-only`, `an-operand-is-one-body-by-a-recipe-count-the-kernel-does-not-read`, `a-product-whose-copies-overlap-may-double-count-its-mass`, `a-placement-adds-no-name-segment-is-said-three-ways`, and stage 4's `a-boolean-reading-two-bodies-of-one-arrival-re-decides-their-contact` and `a-boolean-result-reused-without-its-records-passes-at-rest-on-its-shared-keys`, whose design question PR 5494's rule answers.

**Risks.** `names/` is the bulk, as B's refactor doors were. If D′3 is too large for one green PR it splits along one seam only: one source and picks first (the gathered product still reading the world's placements, an instance still copying its part's world placements, each read by a pick), then `⊔`, the product slot and the instance reading its part's product. Each half leaves one way to read a copy. The census rows of a corpus product can move where today's gather grouped copies differently from the `⊔`'s operands; the lane reports each.

**Re-bless across D′.** Names move wherever a copy is named; ids where a placement's target, a construction's `space` output or a pick changes what is minted; the stored documents (`golden.cad`, `die_tool.pncad`, `plate_param.cad`, `gallery_ring.pncad`, `die_composed_tour.pncad`) are regenerated from their generators, and a document in an older wire format refuses, typed, at load (DESIGN Band 4). Geometry moves only in D′2, where the space chart or a helper's written turn says so, each reported by document and node.

**Not D′'s.** The carry and the census per `⊔` are units of their own after D′3 (§12). The flag and pivot kinds follow D′2 (`flag-and-pivot-pose-kinds`).

## 6. PR E — `an-operation-computes-in-a-frame-of-its-reads` (cost H)

E builds D10 `[placement.frame]` (FORK-S3P) on the copies ruling, and the one composing door of `[placement.world]`. Its rider, minted references from the inputs, merged as PR 5547 (`a-minted-reference-direction-follows-the-computing-axes`, closed): every carrier mint takes a reference its caller holds, and `scripts/gates/minted-reference-from-inputs.sh` keeps `orthonormal_basis` to its scratch homes. No design question is open on E.

**State after D′3.**
- Every body is in exactly one space, derived from the recipe, and an operator's operands share one (D′1).
- A placement defines one copy of one source body (D′3), and maps it into its target eagerly: `wire_place` (`eval/wire.rs:4133`) folds the bundle and maps the source body by `topo::transform::transform_rigid` (`place`, `:1041`). So every value holds its body in its space's coordinates, and every operation computes in its space's coordinates, which for the product are the world's.
- No value records the frame its body is in, and no copy shares its source's body.
- `SolvedPoses` (`mate/solve.rs`) answers `relative_map` and `relative` per placement (`:278`, `:300`).
- Export (`pncad/src/export.rs`) and the viewer's picture read bodies as they stand in the world's coordinates.

**Build.**
- **`Framed`.** A body value is `Framed { body: Arc<Body<T>>, frame: Own | Map(RigidMotion<T>) }` (the copies ruling): the body in its own coordinates, and the map from them into its space's. `Own` is the identity, and is what a construction and the identity bundle as written hold. `NodeValue` (`eval/mod.rs:764`) holds `Framed` wherever it holds a body today, including `BooleanValue::Body`, `PartValue` (`eval/parts.rs:110`) and a `⊔`'s pieces.
- **A copy shares its source.** `wire_place` composes the bundle's motion onto the source's frame and keeps the source's `Arc`: a copy is never rebuilt (`transform_rigid` leaves `place`). An instance's copy shares its part's product body the same way.
- **The computing frame of an operation over operands in one space.** Every operator's and one-input operation's operands are in one space, so they are related by their frames alone. The operation computes in its first operand's frame, as the author lists it (`Subtract`'s `from`), and reads every other operand through `relative`, the one door between two operands of one space: `Framed::relative(&self, other) -> RigidMotion` = `self.frame⁻¹ ∘ other.frame`, mapping the other's body into the first's coordinates once, at the operation. Its result is `Framed { body, frame: <the first operand's frame> }`. A construction computes in its own coordinates (`Own`).
- **A value-informed refinement** is allowed: re-centre on the operands' bounds, snapped to a power-of-two grid at their scale, with the author's order breaking ties. It is measured on the far-from-origin rows, starting with `a-far-meeting-point-fails-membership-by-its-own-rounding`, and lands only if it moves a measured row.
- **Content keys are unchanged** (the copies ruling): a node's key is its reads' keys, and a placement's key already carries its bundle, so a copy and its source share a body without a key of their own.
- **Readers compose through `relative`.** The census at a `⊔` (each pair in either operand's frame, or the band refuses), `Measure`'s two sites and an assertion's `Gap` (one space, D′1), a mate's pose read off a copy (the fold reads the `to` pose in the target space: its body's frame applied to the pose), and the coincidence door's `Sym` replay. Nothing reads a body's coordinates as its space's without its frame.
- **One composing door** (`[placement.world]`). `in_space(value) -> RigidMotion` (new, beside `SolvedPoses`) composes a body's frame into the coordinates of its space, and, through the placements that copy it, of a space it reaches. Export (`pncad/src/export.rs`, `step_for_node` `:150`, `export_document_step` `:202`), the façade's read-outs beside it (a value's bodies, a product's body, a pick mesh, each handed to a host language with no way back into a document; pncad-py calls these doors and composes nothing itself) and the viewer's picture (`viewer/src/pickindex.rs`, `scene.rs`) call it, and nothing else in `crates/{editor-core,pncad,pncad-py,viewer}/src` does: a gate script beside `scripts/gates/minted-reference-from-inputs.sh` holds that. The viewer multiplies each shown body by its frame on the GPU, so a value edit moves a frame, not a mesh. What either produces never re-enters the model.
- **What moves.** Product body digests move into computing-frame coordinates; STEP bytes move by rounding (one more composition), and so may a measured value across copies. Test 13 checks each moved digest against the old one mapped through `in_space`. Its two-frame row computes every corpus operation in its own frame and in a second frame, rotated by an irrational angle and translated, and expects the same body up to that map, the same names, and the same verdicts outside the band.

**Done when.**
- Two copies of one body share one `Arc` (`Arc::ptr_eq`), and a third placement of it builds nothing (`EvalCounters` count no op for it).
- Test 13 holds for every corpus document.
- `scripts/gates/` refuses an `in_space` caller outside export and the viewer's picture.
- Moving a document's only world placement moves no body digest and no measured bit, and moves the STEP bytes.

**Hit list:** `transform_rigid` in `eval/wire.rs` (`:37`, `:1048`) and `place`'s callers; every `NodeValue` body read (`ValuePayload`, `eval/mod.rs:1054–1060`, after D′3); `relative_map`/`relative` readers (`refactor.rs`, `clearance.rs`, `mate/solve.rs`, `viewer/src/session.rs`); `product.rs`' graft of a `⊔` (D′3's `wire_assemble`); export; `pickindex.rs` and `scene.rs`.

**Order and parallelism.**
- E's core waits on D′3, because both rewrite `wire_place`, the value a placement defines, `product.rs` and the viewer's drawing.
- The carry (`a-placement-carries-its-sources-at-rest-result`) also rewrites `wire_place`'s value and `PartValue`, so it follows E rather than running beside it (§12).
- G follows E: it builds copies of one body once and maps them, which is `Framed`.
- E can run beside `flag-and-pivot-pose-kinds` (mate and pose ground) and beside the seat door (`coincide/`), announcing the seam at the replay's pose reads.

**Closes:** `each-space-computes-in-its-earliest-members-frame` (closed for this row already); `frame/a-widened-derived-placement-normalises-a-straddling-newell-sum` re-measures `m10_derived_frame_interval` when E lands (FRAME's).

## 7. PR F — `a-mate-on-a-pinned-copy-refuses` (cost M; ~70 files, 2–3k lines)

**Kernel**

- **The door.** `admit_bundle_make_up` (`edit.rs`) reads the bundle's equations from its definitions (Q4, `solve::equations_of`) and refuses `EditError::MateRefused(MateFault::Overconstrained { placement, constraints, free })` wherever a constraint `c` fixes an equation the rest already fix, codim(S∖c) + codim(c) > codim(S), pinned or not. `constraints` names every such constraint, since the bundle has no order; `free` is the freedoms the whole bundle leaves (0 where it pins its copy), since with values the residual is no family. The test reads the families' general-position meet (`SubgroupFamily::meet`), and nothing is measured. A copy pinned already is the case codim(S∖c) = 6, with no predicate.
- **The solve** refuses the same in `solve_bundle`, before it folds (`fold_bundle`), for a state the door did not see (a rebind, a load, a definition edit that changed a kind), reporting `MateFault::Overconstrained` and keeping A11 (1)'s door/solve split.
- **Retired** (FORK-S3O): `MateFault::Contradictory`; `member_of`, `trivial_member` and `coset::intersect`'s membership re-measure; the table's measured case splits; the redundant arms of `Refuted`. A configuration degenerate at its values refuses as its construction does (`Through`, `Meet`).
- **The at-rest gate** no longer mints anything for a mate: where two copies meet beyond what their mates fix is a contact, recorded at stage 4's door and reported by the `unproven-coincidence` lint, and its recourse is an assertion.

**Migration.** Regenerating restates every corpus bundle that overconstrains as one mate plus values, the pose computed once from today's solve, and drops each mate that pins nothing new. The report names each.

**Goldens.** Poses are unmoved (a dropped mate fixed nothing new). The gallery's at-rest rows that counted declared contacts are restated by stage 4's and 5's shapes.

**Closes:**

- `mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion`;
- `a-box-over-a-solved-clocking-widens-thirty-thousandfold` (its widening is `member_of`'s re-measure);
- `an-identically-zero-margin-escalates-at-a-fine-eps` (the same re-measure and `check_offsets`, both gone).

**Re-read, released:**

- `a-far-meeting-point-fails-membership-by-its-own-rounding`: its false `Contradictory` came from a membership re-measure, and both are gone. Closed if E's frame and F's door leave no arm that reaches it.
- `a-box-independent-mate-fault-bisects-the-whole-leaf-budget`: F's `Overconstrained` is a box-independent mate fault, so its fix (a terminal class in `drive.rs` `classify_replay`) is workable once F names the vocabulary. It keeps `the-box-driver-carries-no-part-resolver`.

## 8. PR G — `patterns-are-index-variables` (cost H)

G builds D10 `[repetition]`, the sentence in `[placement.placement]` that makes a pattern a placement whose reads reach an index, and `[assertions]` "Families". It rests on FORK-PAT (row 99), FORK-DM4 (ruled and built), rows 189–191 (a placement names its target space; `⊔` keeps pieces, `∪` fuses; an arrival is one operand at one value of its indices) and the copies ruling (§6). `work/intent/patterns-are-index-variables.md` records FORK-PAT's build. Where that item and this section differ, this section is the version restated against main at `4a4bb23608`.

**State on main.**
- **Pattern nodes.** `Node::Pattern { input, count, kind }` and `Node::PlacedUnion { input, count: Option, kind }` (`node.rs`) place copies by a rule. The rule is `PatternKind::{Linear { direction: [S; 3], spacing }, Circular { axis, step }, Explicit(Vec<placement::Frame>)}`. `Linear`'s `direction` holds three components of a direction in slots, which FORK-MOVE forbids. This is a named interim, and G removes it.
- **Evaluation.** `wire_pattern`, `wire_placed_union` and `place_each` (`eval/wire.rs`), with `stepped_map` and `eval/wire/stepped.rs` (`stepped_rule_map`, `SteppedOperands`, `turns_held`).
- **Names and selection.** `name_pattern` and `name_placed_union` (`names/emit.rs`, `:1159` and `:1219`) mint `RoleSeg::Instance { i, of }`. One body is selected by `Node::Part { select: PartSelect::Instance(i) }`, DM3 as it stood.
- **Doors.** `placement_rule_fault` (`node.rs`, `:3476`) and `CountMismatch`.
- **Mirror.** A mirror is an improper `Explicit` frame, written by pncad-py's `Frame.mirror_across_plane` (`py/place.rs`). `Frame::admission_fault` (`placement.rs`) admits it, and every other door refuses it as `EditError::ImproperPlacement`.
- **Explicit frames.** D's exact-turn lane writes `die_tool`'s explicit frames, in Rust and in the Python twin, by `Frame::rotate_by_turns_then_translate`. The frames are exact, but the stored document still holds them as floats.
- **Unions.** `Union` and `Intersect` already read a `Bodies` argument: a family read, or reads spelled at the slot (FORK-DM4).
- **No index yet.** `index(N)` does not exist. `Count` has `CountToScalar` (`expr.rs`) but no `mod`, and no definition is evaluated per member.
- **Reads of pattern copies.** A value whose mate targets a pattern copy is charted on the space's frame (`a-value-on-a-placed-copy-target-is-charted-on-the-space`). Split re-anchors a pattern copy's name by evaluating its index (`a-split-name-picks-a-pattern-copy-by-its-evaluated-index`).

**Build.**
- **The index.**
  - `VarDef` gains `index(N)` and `index(N) within j` (`var.rs`): a `Count` over `0..N`, where `N` is any `Count` expression.
  - `Count` arithmetic gains exact `mod`.
  - The schedule evaluates a definition or operation whose reads reach an index once per value. Its outputs are families keyed by index tuples, outer first.
  - The kind rule is enforced at the door. A reader that reaches the same index reads the member; any other reader reads the whole family. Two indices meet in one reader only when one is `within` the other; otherwise the reader refuses.
  - A definer that refuses at one index refuses the whole family, naming the index. A read out of range is unresolved, typed.
  - `xs[i, j]` reads a member. `Union`'s indexed read is FORK-DM4's and stays.
  - D9 keys a node that reads an index by its content key plus the index values.
- **A pattern is a placement.**
  - Its reads reach an index, and it places into the target space it names (D′1). The index enters through a value (a `Spin` of `scalar(k)·turn/N`, or a slide of `scalar(k)·pitch`), or through a mate's target (`bolt.axis ≡ holes[k].axis`).
  - Its copy is a `Bodies` family in the target space, one member per value, and `p[k]` reads one member. A pick into a member's piece is a pick of `p[k]`.
  - The family is combined like any other: `⊔ p` keeps its members as pieces, each member its own arrival at that `⊔`'s census, and `∪ p` fuses them. A product reads a `⊔` that holds the family, or the fused body.
  - Copies of one body are built once and mapped: each member is `Framed { body, frame: Map(motion_k) }` over the source's one `Arc` (§6).
  - A construction whose scalar inputs read the index is built once per member.
- **Names.**
  - `RoleSeg::Instance { i, of }` becomes `Member { (i, j, …), of }`, keyed by the index variables' ids and the integers.
  - P5's placement-major layout goes, and the `SegPat` index predicate matches `Member` by tuple.
  - Split's re-anchor matches a member by its index variable and integer, never by an evaluated count. This closes `a-split-name-picks-a-pattern-copy-by-its-evaluated-index`.
  - A mate's target read off a member is a pose of that member's body, and the value is charted on the target space's axes (D′2), whichever member it is. This closes `a-value-on-a-placed-copy-target-is-charted-on-the-space`.
- **Mirror.**
  - `Node::Mirror { body, plane }` is a construction that defines a new `Body` in its source's root. Its plane is a `Plane` pose whose reads reach that root alone (MIRROR-DESIGN P1–P4, P6).
  - `Frame`'s improper arm retires, along with `Frame::admission_fault`'s improper case, `EditError::ImproperPlacement` and pncad-py's `mirror_across_plane`. Every pose is then proper.
- **`PlacedUnion` retires** (`placed-union-places-and-fuses-in-one-node`).
  - It becomes a placement over an index and a `Union` reading its family.
  - Its `Separation` certificate becomes the union's fast path wherever the union reads one family of one placement. Its members are then rigid images of one body, which the reads show.
  - An uncertified pair goes through DM4's pairwise pass instead of refusing.
  - Three texts are re-worded with the code: `crates/editor-core/README.md`'s section "The group boolean: `PlacedUnion`", that page's code-map row, and DESIGN.md's companion-table row for it.
- **Explicit frames** (`explicit-placement-frames-hold-floats`).
  - An irregular family is its members written out, each saying where it is (D10 `[repetition]`). So `PatternKind::Explicit(frames)` becomes one placement per frame, combined by `Union([...])` where `PlacedUnion` fused them and by `Assemble([...])` where `Pattern` kept them apart.
  - The generators (`die_tool`'s Rust and Python authoring, and the tour's `diefillet`) state each frame's moves as written turns and lengths. D's exact-turn lane already does this with `rotate_by_turns_then_translate`.
  - A pip on the die's face is a `Plane` mate against that face, with `Flip` on the copy's side, plus its in-plane slides and its spin value. Which side of the face the material lies on is the mate's to say.
  - Each document is regenerated from its generator, never migrated from its stored floats (DESIGN Band 4). `die_tool.pncad` holds six `Explicit` frames.
- **`Linear` and `Circular`.** The façade re-authors both:
  - `linear_pattern` writes a mate that leaves one slide, with `slide = scalar(k)·pitch`.
  - `circular_pattern` writes an `Axis` mate with `spin = scalar(k)·turn/N` for a full ring, or `scalar(k)·step` for a step the author wrote.
  - `grid` and `bolt_circle` are re-authored the same way.
- **What retires.** `PatternKind`, `Node::Pattern`, `Node::PlacedUnion` and `Node::Part`'s `PartSelect::Instance` retire. So do the placement-rule slot table, `placement_rule_fault`, `CountMismatch`, `wire_pattern`, `wire_placed_union`, `place_each`, `stepped_map`, `eval/wire/stepped.rs`, `name_pattern` and `name_placed_union`, and the pattern level of the member walk in `mate/member.rs`. Their content-key tags go with them; re-grep for the tag numbers, since the item's 12/13/19–22 predate D.
- **Façade and GUI.**
  - `linear_pattern`, `circular_pattern`, `grid`, `bolt_circle` and `mirror` write the program.
  - A display-only recogniser in the façade reads a placement under an index, whose values are affine in that index, back as "circular, 12 about A" for the viewer's forms. `PatternKindChoice` and `PatternRuleSpec` become the recogniser's output and input.
  - The sites that follow:
    - viewer: `forms.rs`, `session/author.rs`, `session/op.rs`, `combine.rs`, `tools.rs`, `vocab.rs`, `widgets.rs`, `pane/create.rs`, `drafts.rs`, `frame.rs`, `tree.rs`, `session.rs`, `session/refuse.rs`;
    - pncad: `document.rs`, `prelude.rs`;
    - pncad-py: `py/doc.rs` (`pattern`, `placed_union`), `py/place.rs` (`PatternKind`, `Frame.mirror_across_plane`), `py/analysis.rs`, `py/measure.rs`, `node_kind.rs`, `pncad.pyi`;
    - tour: `gallery.rs`, `heatsink.rs`, `assembly.rs`, `impeller.rs`, `diefillet.rs`.
- **Per-member assertions** (`a-map-over-a-patterns-bodies-asserts-once-per-member`; its design was settled by Ev on PR 5230).
  - An `Assertion` whose reads reach an index is one assertion per member. Each reports its own verdict and quiets its own finding (D10 `[assertions.quiet]`).
  - A pair of members is two reads: `c[k]` and `c[(k + 1) mod N]`, or two indices with one `within` the other.
  - Rows: twelve fins sunk into a base, quieted by one authored assertion that follows an edit of `N`; and a ring's neighbours, quieted through `mod`.
- **Docs that follow:** INTENT-STAGE2 §Pattern, the one-node overlap sentence in INTENT-STAGE5 B, ASSEMBLY A11 (5)'s member walk, NAMES N1 (`Instance` → `Member`), and REFERENCES DM3's member pick.

**Order.**
- (a) The index, families, `mod`, `xs[i, j]`, `Member` names and the per-member assertion. This is a whole representation of repetition over definitions and operations. But `Node::Pattern` would still stand beside it as a second way to repeat a copy, so (a) does not merge alone.
- (b) Patterns as placements, `PlacedUnion`'s retirement, the façade and the regenerated documents. These go in the same PR as (a).
- (c) `Mirror`, with the improper frame's retirement. This can be its own PR, before or after (a) and (b). On its own it leaves exactly one way to mirror.

**Parallelism.**
- G's (a) and (b) need D′3: a pattern is a placement into a named target, its copy a family combined by `⊔` or `∪`. They also rewrite files D′ rewrites (`names/role.rs`, `names/emit.rs`, `eval/wire.rs`, `node.rs`, `refactor.rs`). So G follows D′3.
- They follow E's core too: a member is a `Framed` over its source's one body, and G and E would otherwise both rewrite `wire_place`.
- Before G, D′3's `⊔` treats each body of a `Pattern` or `PlacedUnion` operand as its own arrival; G makes each member an arrival by construction and removes that interim.
- `Mirror` is additive beside D′: one new variant in `node.rs`, a new wire function, an entry in the content key's tag list, the façade's `mirror` and pncad-py's frame doors. It can run in parallel with D′ or with E.

**Open design questions.**
1. **Whether a ring closes structurally.**
   - Two members under one `Axis` mate differ only by their spin values, so `N · (turn/N) ≡ turn` is a comparison of angles modulo a turn.
   - The symbolic tier proves a coincidence as a polynomial identity over ℚ in matrix entries. It folds `sin` and `cos` of a rational multiple of π only where the value is rational (D's `trig_at_rational_pi`).
   - So for most `N` it cannot prove that member `N − 1`'s neighbour is member 0.
   - Measure first: a ring of 12 under the `unproven-coincidence` lint. If the tier cannot prove it, how a pattern's closure is proved is undecided. Row 187's first rung does not reach it: neighbouring members touch where no mate states it.

How a copy's body is held, once §6's Q1 and shared with E, is settled by the copies ruling (§6).

## 9. What moves and what retires

**Ratified clauses this stage retires or rewrites.** Each one quoted is D10-listed (its last paragraph) or consequential re-wording, which lands with the unit that moves the code it describes.

- **ASSEMBLY A3** (B, C, G; D10-listed): "`Node::InstantiatePart { doc_ref, interface, gauge, offset }` instantiates a pinned document; it names its gauge and may carry an offset in it (A11 (2))" … "`Node::Mate { a, b, class, alignment }` is one contact declaration that also places" … "Every number that says where the two sides meet lives in the sides' offsets". It is rewritten as the node vocabulary: an instance as a placement, `Place`, a mate equating two poses, a value. "Mates are declarations" goes; a mate places and never checks.
- **A11 (2)** (B; D10-listed), entire: "Placement lives on a gauge …" through "A further statement of where a placed instance sits is verified against the solve, never trusted and never silently ignored". The parametric placement (#3437) is kept as values; "no edit records a frame" is kept; the own-space rule (#3441) becomes the loose copy.
- **A11 (3)** (B): "A group's tree is rooted at its earliest member carrying an offset …". It retires: there are no roots of that kind.
- **A11 (4)** (B, F; D10-listed): "Tree mates DETERMINE and must fold to `Trivial` …; non-tree mates DECLARE and are only verified by the gate. No cycle is ever solved; an inconsistent loop dies at its closing mate's verification". It retires. B keeps "the solve is total and per-node" and "whether the document has a product is the gather's question alone"; F replaces the rest with the overconstraint rule.
- **A11 (1)** (C, F): "Each primitive pins the pair's relative pose to a coset of an SE(3) subgroup … several mates on one pair fold by exact coset intersection … to DETERMINED, UNDER or CONTRADICTORY". After F it reads: "a placement's mates fold by subgroup algebra, measuring nothing, to DETERMINED or UNDER, or refuse OVERCONSTRAINED (a constraint any of whose equations the bundle already fixes, named)" (FORK-S3O). C re-keys it by pose kind.
- **A11 (5)** (B, C, D): the world-pose and member-walk paragraphs. Both are rewritten over placements and pose definitions. "What the solve reads" keeps the extent lever, and the face pose is now read in `eval_pose`.
- **A6** (C, D, G): "a literal step of an instance's offset, a gauge's placement or a transform's chain refuse `EditError::ImproperPlacement`". Its sites become an evaluated pose value; G's `Mirror` makes the improper frame unrepresentable.
- **A9** (B): "every placed group has a world pose … A group nothing places lives in its own space". It is rewritten over kinds.
- **A10** (B): its world-placement sentence follows D10's Operations paragraph as ratified on #4326.
- **A4** (B): the gauge sentences of *Split* and *Inline* and all of *Promote and Fold* retire; the acceptance stays.
- **REFERENCES DM1** (A, D): the derived frame carrying a face name retires; a face reads as a plane (FORK-S3P), and a sketch on a face is a placement. DM1a and DM1b's typed refusal of a non-planar carrier stay with `Plane { face }`.
- **REFERENCES DM3** (D, G): "names pass through unchanged, as `Transform`'s do" is re-worded to a copy's.
- **REFERENCES §0**: "`AxisInPlane` … the one datum with a DAG input" is deleted (A).
- **D10 itself**: one parenthetical, "a face's frame" → "a face's plane" (§13).
- **Text that follows D′ and E's code** (PR 5494's body names it; each re-worded by the unit that moves the code it describes, a re-wording carried by the approved change):
  - `docs/guide/assembly.md` and `crates/viewer/README.md` describe world planes as mate targets (D′2);
  - ASSEMBLY A10 and A11 describe the gathered product (D′3), and A11 (5)'s world-pose paragraph the world's roles (D′2);
  - `crates/editor-core/README.md`'s copy and `Placed`-layer sentences, NAMES N1 and N4 on `RoleSeg::Placed` (D′3; `a-placement-adds-no-name-segment-is-said-three-ways`);
  - IDENTITY DI5's shown spaces (D′1) and its composing door (E).

| | A | B | C | D | E | F | G |
|---|---|---|---|---|---|---|---|
| Node ids, pinned hex, memo fixtures | `AxisInPlane`, `FaceFrame` | gauges, mates under placements | mates | **all** (datums, transforms leave) | — | — | patterns |
| Var table, mint log | grows (pose definitions) | grows (`Carried`) | grows (values, standoffs) | grows | — | — | grows (indices) |
| Wire | `Revolve` slots; pose definitions | `Gauge`, `gauge`/`offset`, `PlaceInWorld` gone; `Place`, `World` | `Mate` | `Datum`, `Transform`, profile plane, tube frame, extrude side gone | — | mates dropped | `Pattern`, `PlacedUnion` gone |
| Solved poses | — | unmoved or rounding (test 7) | within stated rounding (test 8) | — | relative to the computing frame | unmoved or rounding | — |
| Product body digests | unmoved | unmoved or rounding | within stated rounding | **move by one composition**, compensated by the world placement (reported) | **move** (computing frame) | unmoved or rounding | unmoved or rounding for copies |
| STEP export bytes | equal | equal | rounding | rounding | **rounding** | equal | equal |
| Names | — | — | — | — | — | — | `Instance` → `Member` |
| Python `.pyi` | pose builders | `place`, `world`; gauge doors go | `Mate(on, to)`, values | datum builders, `transform` go | — | — | presets become façade functions |

D′ is not a column above. D′1 moves the wire, the content key and the var tables (a `target` on every placement, a `space` output on every construction) and no geometry. D′2 moves the mate and value wire and the chart's numbers, and geometry only where a mated target body stands off its space's identity or a helper's rounded turn is replaced by the turn its caller meant, each reported. D′3 moves names (the `Placed`/`InPart` layer goes), ids and the stored documents, which are regenerated; product digests are unmoved, and census rows move only where a `⊔`'s operands group copies differently from today's gather, each reported.

**f64 geometry moves only where a unit says so.** In A, B, D′1, D′3, F and G a body digest, a measured bit or a solved pose that changes is a bug. C, D, D′2 and E state what moves and by which map, and their tests check each moved value against that map.

## 10. Test plan (each row names the runtime value that breaks it)

1. **(A) A face reads as a plane.** `Plane { face }` on a cylinder's flat cap equals the cap's carrier plane with its outward normal; on the curved face it refuses DM1b's typed error. *Breaks if* a reader reads the carrier's u-reference (a roll appears in a value that should have none).
2. **(A) A slot holds its kind.** `Split { tool: <an Axis variable> }` refuses `SlotVarKind { expected: Plane }`; `<frame> through xy` is accepted. *Breaks if* a finer value is admitted where a coarser one is asked for without its projection.
3. **(A) A combination refuses its degenerate case.** `Through { axis, point }` with the point on the axis refuses `PoseDegenerate { construction: Through }` at evaluation, at the scalar that decides it. *Breaks if* it picks a frame anyway (an arbitrary roll).
4. **(A) The revolve's axis cannot leave its plane.** `Revolve { axis_through: [0, 0], axis_heading: turn/4 }`: the `axis: Axis` output lies in the profile's plane for every value in a box run (its direction's margin against the plane's normal is identically zero in `Sym`). *Breaks if* the axis is lifted through 3-D coordinates.
5. **(B) Two placements, two copies.** Two `Place`s of one body, each against the world: two copies, distinct output ids, each name resolving once per copy. *Breaks if* the bundle is keyed by body, not by placement.
6. **(B) The product is the world's kind.** As B built it: a copy pinned against a copy pinned against the world is in the product, and two copies pinned only against each other are one loose space. D′1 retires the loose space (test 18) and D′3 the derived product (test 26), so from D′3 this row is test 26's.
7. **(B) The one-time migration.** For every corpus `.pncad` and every MSOLVE fixture, `SolvedPoses::placement` for every copy and `product_recorded`'s digests in order are unmoved or moved by rounding, and the report names every move. *Breaks if* a tree mate goes into the parent's bundle instead of the child's, or a gauge chain is composed outer-first.
8. **(C) The mate respelling.** Every MSOLVE fixture's solved poses agree with pre-C within the rounding of one value re-composition, and the report names every dropped mate. *Breaks if* a migrated standoff lands in the other side's frame (a non-zero standoff flips sign).
9. **(C) Values.** A `Plane` mate leaves two slides and a spin. Setting each to zero places the copy with its own coordinates agreeing with the target's in the plane, whatever order the values are listed in. A spin value on a lone point mate refuses `NoChart`. A second mate taking a freedom a value sets refuses `NotAFreedom` where the placement evaluates (until F, which refuses it `Overconstrained` at the door). *Breaks if* a slide is charted along the other slide's moved axis (order-dependent).
10. **(D) A construction is placed, not framed.** The corpus's sketch-on-face bodies: each copy's digest equals the pre-D digest within one composition, and editing a value moves the copy, not the construction (the construction's digest is unmoved). *Breaks if* a construction reads the frame it is placed by.
11. **(D, G) Nothing moves a body.** No variant reads a body and defines it moved except `Place`, and, until G, `Pattern` and `PlacedUnion`; from G the exemption is gone. A second placement of a copy is a second copy. *Breaks if* `Transform` survives under another name (a census of `Node` variants reading a `Body` and a rigid motion).
12. **(B, D′2, E) Nothing reads the world but a placement's target.** `Delete(world)` refuses `WorldIsUndeletable`. A construction or a mate reading the world refuses at the door (from D′2 there is no `Target::World` to write). The gate admits no `in_space` caller outside export and the viewer's picture. *Breaks if* the world is admitted as an ordinary pose, or a value is read off the world's frame anywhere but the chart.
13. **(E) The computing frame is no part of meaning.** Every corpus operation computed in its frame and in a second frame (the first rotated by an irrational angle and translated) gives the same body up to that map, the same names, and the same verdicts outside the sliver band. Moving a document's only world mate moves no body digest and no measured bit, and moves the STEP bytes. *Breaks if* a minted u-reference follows the axes (a seam moves, a name changes) or a path composes the world's map.
14. **(F) A constraint already fixed refuses, measure-free.** A copy pinned by a mate and values: inserting any further mate refuses `Overconstrained { held: Trivial }` with an empty predicate log. A `Plane` mate, then a second `Plane` mate on any other face, refuses whatever the faces' angle. *Breaks if* the door folds the new mate before checking codimension, or admits a redundant mate whose offset agrees (the fallback Ev rejected).
15. **(F) A contact beyond the mates.** Two copies resting on a second pair of faces their mates do not fix: the census records the contact and the lint reports it; an assertion on its `Gap` quiets it. *Breaks if* the contact is minted as a declaration.
16. **(G) A ring is an index.** `circular_pattern(body, axis, 12)` writes an `Axis` mate and `spin = scalar(k)·turn/12`, no preset; the copies are unmoved or moved by rounding, reported; `Member { (k), of }` names resolve per member. *Breaks if* a preset tag is stored or the copies are built twelve times.
17. **(A–G) Python.** `doc.plane(face)` returns a `Var` of kind `Plane`; `doc.place(body, target, [...])` builds a copy, and `new_space()` and a construction's `.space` name targets. The census and `.pyi` agree, and every gauge, datum, transform, world-read, loose-copy and pattern door is absent from the census. *Breaks if* any of them is still exported by the `.pyi` or the census.
18. **(D′1) Every freedom is written.** An empty bundle, and a plane mate with only one of its three values, each refuse `Unpinned` at the door. No evaluation of any corpus document has a space that is not a `Space` variable. *Breaks if* any bundle that leaves a freedom unset folds (a loose copy survives under another name).
19. **(D′3) Picks.** Two placements of one body into one space are unioned. The result's names key each operand by its read (`From { read }`) and carry no `Placed` or `InPart` segment. A pick of one piece of a copy of `bolt ⊔ plate` reads the bolt's copy; picking it twice gives two variables; a pick through a `∪` refuses at the door. *Breaks if* a name spells the placement, or a reader reads one piece of a copy without a pick.
20. **(D′2) A point mate's chain.** A lone `Point`–`Point` mate plus `Spin(Z)`, `Spin(X)` and `Spin(Y)` pins its copy, turned in the order written with the mated point on its target; adding a `Spin(X)` right after the `Spin(X)` refuses `Overconstrained`, and so does a slide beside the ball. *Breaks if* the order is the kernel's.
21. **(D′2) Inline keeps a variable driving.** A part whose placement's slide reads a named variable, under an instance whose bundle turns it by `turn/4`, is inlined: the part's nodes land in a `new_space()` verbatim, and the instance becomes a placement of them into its target under its own bundle. Moving the variable then moves the spliced copy as it moved the part's copy before the splice. *Breaks if* inline refuses, composes a motion, or writes the variable's current value as a number.
22. **(G) Per-member assertions.** One authored assertion over `fins[k]` and the base quiets all `N` interference findings, and still quiets them after `N` changes from 12 to 16. *Breaks if* the count is fixed at authoring, or one member's finding is quieted by another member's assertion.
23. **(D′1) Operands share one space, decided from the recipe.** `union(a, b)` with `a` and `b` in two construction spaces refuses `SpaceMismatch` at the edit door, in the façade and in Python, before any evaluation. A `SetParam` of a placement's `target` that strands a downstream union is admitted; the document saves and loads, and the union refuses `SpaceMismatch` at evaluation, naming both spaces. *Breaks if* the check reads a value (it fires only after an evaluation), or the load door refuses the stranded document.
24. **(D′2) A value charts on the target space.** The row-190 boss on plate copy `B` (§5′.2): `Slide(X) = 10` under `B` turned 30° and 40° lands where the target space's `x`, carried by the least turn, puts it, and the same face read off `B ∪ C` gives the same copy. *Breaks if* the chart reads the body the pose is read off.
25. **(D′3) Each operator decides its operands' meetings.** `A ⊔ A` is an interference finding; `A ∪ A` is `A`; `h1 ⊔ h2` of a split's halves records the cut with no `unproven-coincidence` finding and `h1 ∪ h2` glues it; `(bolt ⊔ plate) ∪ bead` keeps the bolt–plate record the `⊔` decided and refuses typed where the result touches itself over an area. *Breaks if* an operator re-decides a meeting inside an operand.
26. **(D′3) The product is a slot.** A document whose slot reads `p ⊔ bolt ⊔ nut` exports those three pieces; a placement nothing reads is an `unused` finding and is not exported; a slot reading nothing is an empty product, which export refuses. An instance of that document defines one body, its census decisions carried and reported once, as that document's. *Breaks if* the product is derived from which bodies go unread, or an instance's census rows are decided again in the host.
27. **(E) A copy shares its source.** Two placements of one body hold one `Arc` (`Arc::ptr_eq`), and the second builds nothing. *Breaks if* `wire_place` maps the source body into the target.

Loud census rows: `pncad-py` `tags.rs` / `surface_census` / `prose_census`, `display_contract`, the persist `Walk` roster, `crates/editor-core/tests/golden/slot_tables.txt`, `f6_variants!` and `eval/class.rs`'s class table (`MateOffsetDisagrees`/`MateOffsetUnchecked` go; `Overconstrained` comes).

## 11. Risks

- **B's size.** About 1,600 gauge lines in src and tests plus the refactor doors. The gauge sites are compile-driven; the refactor rewrite is not, and it is where review time goes. B can split only along a representation boundary, and none exists inside it: gauges then the tree would leave two ways to place.
- **D's reach.** Every profile, tube and datum site moves, and every construction off the origin moves by one composition. The migration report is the reviewer's check, and test 10 its gate. D is the unit most likely to need a split along the readers (profiles, then tubes and splits); each half must leave no construction reading a frame of its kind.
- **The migrations that cannot restate.** C and D restate absolute coordinates over geometry where a face or carrier pins the same coset, and otherwise drop and name a mate (C) or refuse to regenerate (D, Q8). Measure the corpus's count before C dispatches.
- **Pose definitions bound mid-evaluation** (A). A face's plane exists only after the body it reads. If stage 2 D's machinery is narrower than this needs, A widens it, and the content key hashes a pose definition's upstream keys, not its id.
- **Constraints reading copies** (B). `Carried` is new: a pose a copy carries is a function of that copy's solved pose, so a bundle's fold reads poses that depend on other bundles. The order is the bundles' read order over `Doc::upstream`, acyclic by the door: A placed against B and B against A is a read cycle and refuses, which replaces A11 (4)'s "no cycle is ever solved".
- **The contacts declaring mates declared** (B). B drops declaring mates, so their contacts are unattributed. Stage 4 J makes them findings before B lands (§12). They are unproven findings from B until stage 4 H, because stage 4 C compares a placement chain as one opaque atom until H, unless B's migration asserts them. B's migration report counts them.
- **E moves goldens on purpose.** Every assembly digest, the STEP bytes and the gallery frames move. The check is test 13's map, not bit equality.
- **Migration reproducing the tree** (B). Today's tree is "the first member pair per instance pair in `Member` key order". The migration orients mates by that exact rule, or test 7 fails where two orders differ.
- **Load.** What remains is D′1, D′2, D′3, E's core and G, each H (25 points by the work README's weights), with the umbrella `a-placement-pins-by-written-values-without-a-mate` parked on its three PRs. The seat and census units add `the-door-reads-a-mated-seat-by-its-mate` (M), `the-census-records-a-planar-area-contact-as-one-face-pair` (H), `flag-and-pivot-pose-kinds` (H), the carry (M) and the census per `⊔` (M). Dispatchable at once: D′1 and the two seat units, under the budget.
- **D′'s reach.** D′ changes how a placement names its target, what a mate may read, how a value is charted, how bodies combine, what the product is and how every reader reads a copy: about as many files as B, through `names/`, `refactor.rs`, the viewer's read sites and the façades. The three PRs (§5′) are the split it allows, and D′3 has one more seam (§5′.3, Risks).
- **The space chart moves geometry** (D′2). A value mated against a target body that stands off its space's identity changes meaning. The lane reports every such copy, by document and node, against the old body chart composed with the target body's placement.
- **World reads with no verbatim mateless spelling** (D′2). A world read's chain of more than three spins, or a slide after a spin, cannot be written as values over the same variables verbatim. Its generator writes the motion it means; a stored document whose chain cannot be written that way refuses to regenerate and is named (Q8's rule). Measure the corpus's count before D′2 dispatches; above a handful, it comes back to the orchestrator.
- **The census moves to a node** (D′3). Today's gather groups world copies one per placement output; a `⊔` groups by operand at one index value. Where a corpus document's product `⊔` groups differently, census rows move, each reported with its cause.
- **E and the carry share `wire_place`.** E rewrites the value a placement defines (`Framed`) and the carry rides on it; running them together would land two representations of a copy's records. The carry follows E.

## 12. Open questions

### The forks are settled

Each of the six forks this spec opened was weighed by a designer pair and ruled by Ev:

- **FORK-S3-1** (what a free pose is) and **FORK-S3-6** (what a space's member is): FORK-S3P, fork log row 95, PR 4324. No pose is free; there is no seed and no member order, and an operation computes in a frame of its reads.
- **FORK-S3-4** (what overconstraint means): FORK-S3O, row 96, PR 4325. Pinned or not, by codimension, measuring nothing.
- **FORK-S3-2** (what a placement is), **FORK-S3-3** (what a mate reads) and **FORK-S3-5** (what placing a multi-body instance copies): FORK-S3M, row 97, PR 4326. `Place` owns mates and values, reads a list of shapes, and an instance keeps one output per world placement.

FORK-PAT (row 99, PR 4341) added unit G.

The questions this spec opened on D′, E and G are settled too:

- **§5′'s Q1** (how the recipe names a placement's source and target space) and **Q3** (what the viewer shows besides the world): PR 5494, rows 189 and 191. `Space` is a minted kind; a placement reads a source body and a target space; the viewer shows a set of spaces through the one door export uses.
- **§5′'s Q2** (how a value names its freedom beside a mate): PR 5494, row 190. A value names no mate, and values chart on the target space's axes.
- **§5′'s Q4** (what a placement helper takes from its caller): the helpers ruling (orchestrator, 2026-10-20; no ratified text moved). One pure constructor from the caller's mates and moves; the four helpers and `Anchors` retire; the place-part gesture writes the identity bundle.
- **§5′'s Q5** (what inline makes of a moved landing): PR 5494, ASSEMBLY A4. Inline mints a `new_space()`, copies the part verbatim, and places its product under the instance's bundle; flattening an identity placement is its own edit.
- **§6's Q1** (how a value holds its body and the frame it is computed in, shared with G's Q2): the copies ruling (orchestrator, 2026-10-20). `Framed { body, frame: Own | Map }`, `relative` the one door, content keys unchanged.
- **§6's Q2** (where a minted reference comes from when the inputs carry none): PR 5547. Every caller supplies one it holds, so D10's sentence holds as written.
- **§6's Q3** (how the world's space is drawn without reading the world's map): PR 5494, `[placement.world]`. Export and the viewer's drawing share one composing door that nothing else calls.

One question stays open: **§8's Q1**, whether a ring closes structurally, which is measured first.

### Questions with a recommendation (not forks)

1. **Which constructions A builds.** **Recommendation:** the reads off geometry, reads through a projection role, `Slide`, `Spin`, `Flip`, `Through` and `Meet`. The rest wait for a reader. The extrude's slant is built when a corpus document or a façade gesture writes one.
2. **`Point` and `Direction` subgroups.** **Recommendation:** both in A, because D10 names a lone point mate and its direction-mate lowering.
3. **A clocked coaxial** (today's `Coaxial` plus rider, residual `Prismatic`). **Recommendation:** an `Axis`–`Axis` mate plus a slide and a spin value (C), with the `Cylindrical` residual's chart in the shared `Subgroup`.
4. **Where the door reads the held fold** (F). **Recommendation:** fold the bundle at the door from its definitions. It is a handful of table rows, and it keeps "the doors decide edits" free of a stored solve.
5. **The viewer and the world.** Answered by PR 5494 (`[placement.world]`): export and the viewer's drawing compose a body into a space's coordinates through one door nothing else calls, and what they produce never re-enters the model. The viewer's shown spaces and display frames are display state (DI5). E builds the door (§6).
6. **`Transform`'s Python spelling.** **Recommendation:** none. `body.transform(by)` goes with the node; the façade's `place` with values is the one spelling (Ev on #4220: one semantics in Python and Rust).
7. **What a mate reads on the world** (orchestrator's ruling, 2026-10-09). Superseded by PR 5494: no space has roles a mate can read, the world included, so `WorldRead` retires in D′2 and a placement against the world states its motion by values on the world's axes.
8. **An absolute pose with no geometry to restate it over** (D's migration). **Recommendation:** refuse to regenerate and name the node, after measuring how many corpus documents reach it; a corpus count above a handful comes back to the orchestrator before D dispatches.

### Boundaries with the neighbouring stages

- **Stage 2.**
  - A waits on E (a face's plane reads a `Face` variable).
  - B waits on F (mate sides read `Face` variables). Stage 2 F keeps `reading_edges`' gauge edges (its Q3), and B deletes them with gauges.
  - B retires `PlaceInWorld`, which stage 2 C builds; stage 2 had no world node to mate against, and Ev called the state transient (#4220).
- **Stage 4.**
  - **H waits on this stage's C** (`placed-carriers-compare-through-their-frames`). A placed copy's frame is the construction its bundle states (FORK-S3O), and H replays that fold at `Sym`. The bundle exists after B, but its mates' poses are read off geometry only after C, so H waits on C rather than replaying today's offset frames and then rewriting the replay. This moves H from "after B" to "after C".
  - **F waits on stage 4's C** (D has merged). F turns redundant mates into contacts the census records, and the rungs C and D are what prove those contacts structural rather than report them as unproven.
  - **B waits on stage 4 J** (`an-unattributed-contact-at-rest-is-a-finding`). B drops declaring mates, so a contact one declared is unattributed, and A5's hard error would refuse that document's product. Stage 4 I retired that error but waits on H, which waits on this stage, so the orchestrator cut it out of I (2026-10-09) into J, after stage 4 B and C (D has merged). Contacts no mate attributes are then non-refusing `unproven-coincidence` findings, as D10 says ("recorded and linted, and its recourse is an assertion"). The contacts between differently placed copies stay unproven until stage 4 H, which proves mate-placed faces; the interim is loud and refuses nothing.
  - Stage 4's canonical forms compare poses "modulo the kind's own symmetry", the same `Subgroup` A shares.
- **Stage 5.**
  - Its unit C (`the-at-rest-census-is-a-check`) checks at each `⊔`, between that operator's operands (PR 5494: "per space" becomes "per operator, between its operands"), and names F as its stage-3 trigger, which stands. Its spec's wording is re-read when the unit dispatches.
  - Its "copy" is "a `Body` output of a world placement"; under PR 5494 a copy is the one body a placement defines, in any target, and the product is the slot's body.
- **Stage 6** owns coaxiality through one axis variable and the tangency constructions. A builds the `Axis` reads they use.

### Sequencing: the seat and census units

What remains of this stage, and the INTENT units that ride on it, in dispatch order. A row names what it waits on; rows on one line can run beside each other.

1. **Now, in parallel:**
   - D′1 (`a-placement-names-its-target-space`);
   - `the-door-reads-a-mated-seat-by-its-mate` (P0 M; being built on `intent/door-reads-a-mated-seat`). Its ground is `coincide/` and the at-rest rows `assembly.rs` hands it, disjoint from D′1. It closes `a-mate-placed-seat-is-unproven-at-the-doors-budget` and, for mated rows, stage 4's `placed-carriers-compare-through-their-frames`;
   - `the-census-records-a-planar-area-contact-as-one-face-pair` (P1 H). Its ground is `crates/topo/src/census.rs` and the at-rest checks, disjoint from D′. It is on stage 4 I's path (`mates-declare-no-contact` waits on it and on the seat door);
   - G's `Mirror` (§8 (c)), additive in `node.rs`; it announces the seam to D′1;
   - the listing half of `a-seated-pin-written-as-an-axis-and-a-number-is-a-finding` (P2 M): the corpus sweep for pins and bolts seated by an axis mate and a numeric slide.
2. **D′2** after D′1.
3. **D′3** after D′2, and beside it `flag-and-pivot-pose-kinds` (P1 H), which waits on D′2 for the value chart its two kinds' mates fit. Its ground is `var.rs`, `pose.rs`, `mate/coset.rs`, `mate/value.rs`, persistence and the viewer's pick; D′3's is `node.rs`'s `Place`, `names/`, `product.rs` and `refactor.rs`'s inline. Then the re-mating half of `a-seated-pin-written-as-an-axis-and-a-number-is-a-finding`, as pivots.
4. **E's core** after D′3.
5. **The carry** (`a-placement-carries-its-sources-at-rest-result`, P1 M) after E. Under PR 5494 a placement defines one copy of one body, which carries every pose of its source and the source's contact records and census decisions, which stay the source's. It generalises `PartValue`'s carry (`eval/parts.rs:110`) to every placement, so one type crosses every placement, riding on E's `Framed`. It waits on D′3 (one source, one copy; an instance reads its part's product) and follows E, since both rewrite `wire_place`'s value.
6. **The census per `⊔`** (`each-space-runs-its-census-between-arrivals`, P1, re-titled "each `⊔` decides its operands' meetings" and re-priced M) after the carry. D′3 already moves the census to each `⊔`'s door with today's `record_at_rest` over its operands. This unit builds the rest of `[assertions]`: inside one arrival the census decides and reads nothing, so the transitional check confirms carried records and matches carried findings (the carry's solid-pair match is its confirm-and-match arm), and anything else is that arrival's door's defect, typed; a finding names its cells by the reads they entered the `⊔` through, and is reported once, at the `⊔` that decided it, however many times its body is placed or read. Its open measure stays `assemble-aggregate-census-is-quadratic-in-solids`. Then `the-in-arrival-census-check-retires`.
7. **G** after D′3 and E's core. It ends D′3's interim (each body of a `Pattern` or `PlacedUnion` operand its own arrival) by making each member an arrival.

The seat door and the face-pair census row are stage 4 I's, not this stage's, and run whenever their lanes are free. The census units wait on D′3 because "an arrival is one operand" needs the `⊔` node and the carry needs one copy per placement.

### Rows released by this stage (the 2026-10-08 re-homing, `work/intent/log.md`)

| Row | Unit | Disposition |
|---|---|---|
| `msolve/a-box-independent-mate-fault-bisects-the-whole-leaf-budget` | F | opened: its vocabulary is F's `Overconstrained` |
| `msolve/a-box-over-a-solved-clocking-widens-thirty-thousandfold` | F | closed: the re-measure is gone |
| `msolve/a-clocking-rider-is-levered-unreduced` | C | closed: the rider is gone |
| `msolve/a-declaring-mates-alignment-is-never-read` | B | closed: no declaring role |
| `msolve/a-face-base-puts-its-reference-on-local-y` | C | closed: no reader takes a carrier's reference |
| `msolve/a-face-frame-cannot-turn-its-roll` | C | closed: a roll is a spin value |
| `msolve/a-far-meeting-point-fails-membership-by-its-own-rounding` | E, F | closed if no arm reaches it after F, else opened |
| `msolve/a-mate-frame-axis-is-decided-against-a-length-band` | C | closed: no mate frame is composed |
| `msolve/a-mate-frame-is-written-in-the-reading-instances-coordinates` | C | closed |
| `msolve/an-identically-zero-margin-escalates-at-a-fine-eps` | F | closed |
| `msolve/mate-primitive-unit-variants-load-from-a-null-payload` | C | closed: `MatePrimitive` is gone |
| `recipe/placement-step-slots-are-spelled-three-ways` | C | closed: offset steps are values |
| `topo/a-boxed-rotation-refuses-not-rigid-at-every-placer` | D, G | placement half closed at D; pattern-angle half at G |

Also in scope, outside the umbrella:

- `explicit-placement-frames-hold-floats` and `placed-union-places-and-fuses-in-one-node` close at G.
- `a-minted-reference-direction-follows-the-computing-axes` closed with E's rider, PR 5547.
- `mate-offset-verified-…` closes at F.
- `msolve/gauge-of-recomputes-the-clusters-per-placement-lookup` and `msolve/a-placer-row-states-what-a-poisoned-row-cannot` close at B.
- `msolve/the-mate-solve-reads-the-platform-atan2` is untouched by A and B. C re-reads it: `clocking_about` goes with the rider, and `candidate_rotation` survives.

## 13. Inconsistencies found

- **D10's Variables paragraph** said poses are "read off a body's geometry (a face's frame, …)". That parenthetical came from FORK-S3P's final draft (`de87a9fd39`). The same paragraph's round 10 sentence, "A face reads as a plane; no reader takes a carrier's reference direction" (`7147b6046d`), is Ev-ratified (row 95, #4324). This PR re-words the parenthetical to "a face's plane": a re-wording carried by that approved change.
- **`work/intent/a-mate-on-a-pinned-copy-refuses`**'s FORK-S3O section says a pin is "in practice … one `Frame` mate between two constructed frames (`FaceFrame` plus `Offset`, …)" and that the migration rewrites each bundle as one constructed `Frame` mate. FORK-S3M, merged after it and cited by Ev's approval of FORK-S3O, puts mates and values on equal footing and retires the `Offset` target; D10 says so. This spec builds D10, and the row's body is re-worded to match.
- **ASSEMBLY A11 (4)** says "an inconsistent loop dies at its closing mate's verification (`MateFault::Contradictory`)". No code verifies a declaring mate; `Contradictory` arises only inside one pair's fold or from a rider. B and F delete the clause.
- **`mate/coset.rs`'s module doc and A11 (1)** call the fold "exact coset intersection". Every branch is a decided predicate over measured values (`k_stats::decide`). F's rule decides by codimension alone, and A11 (1)'s rewrite (§9) says "subgroup algebra".
- **`work/intent/explicit-placement-frames-hold-floats`** says "the die's twenty-one pip frames". `die_tool.pncad` holds six `Explicit` frames.
- **`VarKind::symmetry`** has no callers in src. The solve reads `PoseSymmetry` on `DatumValue`, so the one `Subgroup` #4222 approved is two tables kept in step by a test (`intent_s2_a_outputs.rs`). A folds them.
- **This spec's §8 said G waits on FORK-DM4's ruling.** FORK-DM4 was ruled and built in PRs 4527 and 4674, so G waits on D′ instead (§0).
- **`patterns-are-index-variables`'s item is stale in two places.**
  - Its Sites cite `placer_axis`, which D's head does not have, and `eval/stepped.rs`, which is now `eval/wire/stepped.rs`.
  - Its Migration converts stored frames, but under DESIGN Band 4 a document is regenerated from its generator.

  §8 restates both.
- **`PatternKind::Explicit`'s doc** (`node.rs`) still says "the die's twenty-one pip locations". `die_tool.pncad` holds six. G deletes the variant.
- **`PatternKind::Linear { direction: [S; 3] }`** holds a direction's components in slots, which FORK-MOVE (row 149) forbids. Neither A′ nor D listed it. It is an interim on D's head, and G removes it.
- **`validate_document` and "loads with them marked".** `[placement.spaces]` says the operands' shared space is "the document's [check], as a dimension mismatch is (`persist::check::validate_document`)", and also that a document re-targeted or merged into a mismatch "loads with them marked". `validate_document` (`persist/check.rs:416`) is a refusing door: a slot read at another kind refuses the load (`SlotVarKind`). Read literally, the parenthetical would refuse a file the next sentence says loads. PR 5494's body settles which governs: under Ev's (a) the document still loads. §5′.1 builds that: the edit door, the builder and Python refuse to write a mismatch, and `validate_document` does not refuse one. `[placement.spaces]` is re-worded to say so: a file holding such a read loads, and the node refuses at evaluation, naming both spaces.
- **A pick through `∪`, `−` or `∩`.** `[placement.placement]` defines a pick as "one piece of a copy", through "the placements inside the source down to the first variable it reaches". It does not say what a path through a fusing operator names. Those operators keep no pieces, so §1 and §5′.3 refuse such a pick at the door; a `⊔` operand and a placement are the only steps a pick's path takes.
- **`Mirror`'s "source's root".** D10 `[repetition]` says `Mirror { body, plane }` defines "a new `Body` in its source's root". After PR 5494 a body is in a space, and "root" is the pre-5494 word for it. §1 reads it as the source's space; the wording is G's to carry when it re-words `[repetition]` with the code.
