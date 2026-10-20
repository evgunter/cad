# SPEC: INTENT stage 3 — spaces and placement

Scope: D10's **Spaces and placement** and **Repetition** paragraphs, the pose half of its **Variables** paragraph, and the placement sentences of its **Operations** paragraph (`docs/DESIGN.md` §D10), as `work/intent/plan.md` stage 3 states it:

- Poses become variables of their kinds, and none is free or defined from nothing.
- A placement owns the mates and values that pin its copies. Nothing else moves a body.
- The world is one undeletable node, and only placements and export read it.
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
- **Row 162** (PR 5312, Ev approved; `a-placement-relates-two-frames`). D10 `[placement]` as it stands: a space is a construction's coordinates or the world; only a placement relates two spaces, and spaces form a DAG; a placement reads a source space and places its carried bodies into a space; mates fix what they fix and values set the rest, all six where there is no mate, the rotation then an author-ordered chain of at most three spins about the target space's axes; a bundle leaving a freedom unset refuses at the door; an instance places another document's world or any space of it.
- **Row 165** (PR 5315, Ev 2026-10-19; `a-space-names-its-bodies`). A space's readers name its bodies by its variables: a construction's or operation's output, or a **pick**, a `Body` variable defined as one copy a placement places there (a `Bodies` family where the placement's reads reach an index). A pick's `of` is the path through the source's placements to the first variable it reaches, its one spelling; a pick authored twice is two variables.
- **PR 4666** (Ev 2026-10-19). D10 `[assertions]`' census between arrivals, and `[placement.placement]`'s carry of the source's at-rest decisions.
- **PR 5230** (Ev 2026-10-17). D10 `[assertions]` "Families": an assertion whose reads reach a family's index is one assertion per member.
- **FORK-REST** (orchestrator, 2026-10-19; designers converged, no ratified text moved) and **D brings forward the mateless bundle** (orchestrator, 2026-10-20): only the identity bundle as written places by no map, in every lane; every other placement runs its map. `Value.mate` is optional and a bundle with no mate states the whole motion; `stand` writes the identity bundle.

D10's last paragraph retires, here, **ASSEMBLY A3**, **A11 (2)'s gauges and offsets** and **A11 (4)'s declaring mates**. §9 quotes each clause this stage retires or rewrites.

**Baseline.** This spec is written against stage 2's **final shapes**, not today's code: after stage 2 F, every operand is a read, `Select` defines `Face`/`Edge` variables, and mate sides read `Face` variables. Stage 2 C's `PlaceInWorld` has landed by B: B waits on stage 2 F, which follows stage 2 D, which waits on stage 2 C (`the-product-is-an-explicit-list`). B's migration covers it, since it is how a plain document's bodies reach the world.

**Line numbers.** §2–§5 cite main at the stage 3 spec's first baseline (`044b5eb2e`) and drift under stage 2's units. §5′, §6 and §8 cite stage 3 D's branch at `7bcc7f6b4c` (its handoff commit, main merged at `557e581a64`), by symbol, with a line number only beside a name. Each unit's dispatching orchestrator re-greps at the merge of the unit it waits on and states the drift.

**D is the base of §5′, §6 and §8, and D has not merged.** "D's head" there means `7bcc7f6b4c`. Those sections assume D merges with what its item's draft PR body says it builds: the mateless bundle, FORK-REST, exact rational turns, and `stand` writing the identity bundle. They assume D closes `a-copy-standing-where-its-body-stands-has-no-d10-spelling` and `datum-distance-reads-a-datum-node-not-a-pose`, and settles the gates step's `payload-rung-sweep.py --check` red, where the sweep reads D's `mate::Placed` as the `coincide::Placed` rung. If D lands any of these differently, §5′ moves with it.

Today's state of what this stage touches:

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

## 0. Ordering: nine PRs, each green, none landing half a representation

| PR | Unit (`work/intent/`) | Lands | Representation step it completes | Goldens |
|---|---|---|---|---|
| A | `poses-are-variables` | pose definitions read off geometry, by coordinates in a constructed frame, by construction (`Through`, `Meet`, `Flip`, the standoff), by projection, or as an output; one `Subgroup` on the pose value; the revolve's axis as a 2-D line on the node; `Split`'s tool as a `Plane` read | **a pose is a defined variable of its kind** | re-blessed: `AxisInPlane` leaves the node table, and so does each `FaceFrame` read only as a plane. Geometry bit-equal |
| A′ | `poses-move-by-lengths-and-angles` (FORK-MOVE) | A's built vocabulary restated: `Slide` and `Spin`; a read carries its projection role (a frame's x, y, z, origin, planes); `InFrame`'s coordinate arms, `PoseCoords`, `Project` and `Standoff` retire; `Revolve`'s axis is `axis_through` and `axis_heading`; VR6's constant `0` is built and `equal_variables` compares VR8 lowerings | **a pose is moved by lengths and angles** | re-blessed; a datum that was not axis-aligned moves by rounding, reported |
| B | `a-placement-is-the-bundle-of-mates` | `Place { shapes, constraints }` owns today's mates; the world node; an instance is a placement; spaces are kinds from the recipe. Gauges, offsets, `PlaceInWorld`, the spanning tree, roots and `MateRole` retire, and a mate beyond its bundle's pin refuses | **a placement owns its mates; the world is a node** | re-blessed. One-time migration check: poses and product unmoved or moved by rounding, every move reported |
| C | `a-mate-relates-two-poses` | a mate is `{ on, to }`, two poses of one kind, holding no number; values set the freedoms mates leave. `MatePrimitive`, `MateFrame`, `FrameBase`, `AxisSense`, the rider and `PlanarRest.offset` retire | **a mate equates two poses; a value sets a freedom** | re-blessed: mate wire. Solved poses equal to within the migration's stated rounding |
| D | `transform-retires-into-a-placement` | constructions read no frame: a profile is 2-D, a tube is built in its own coordinates, an extrude has no side. `Datum`, `Transform` and `PlacedFrom` retire; each becomes a placement | **nothing moves a body, and frames enter only at placement** | re-blessed. Each moved body moves by one composition, which the migration reports |
| D′ | `a-placement-pins-by-written-values-without-a-mate` | every freedom written, so no loose copy; a placement reads a source space and places into a space; picks name a space's bodies; a lone point mate's rotation is a spin chain; the helpers write only numbers their callers wrote; inline keeps a part's variables driving its copies. `Space::Loose`, `RoleSeg::Placed`/`InPart`, `CopyKey` and `Anchors` retire | **a placement relates two spaces, and a space names its bodies** | re-blessed: names (no `Placed`/`InPart` layer), ids, and every placement whose helper numbers move. Geometry unmoved, or moved by rounding where a helper wrote a rounded turn, each reported |
| E | `an-operation-computes-in-a-frame-of-its-reads` | the per-operation computing frame (D9), keyed with the reads; minted reference directions from the inputs (may land first, alone); export composes the world's map and nothing else reads it | **construction never reads the world** | **product body digests move** (computing-frame coordinates); STEP bytes move by rounding |
| F | `a-mate-on-a-pinned-copy-refuses` | a constraint any of whose equations the bundle already fixes refuses, pinned or not, by subgroup algebra; `Contradictory`, `member_of`, `trivial_member` and the measured case splits retire | **a mate places and never checks** | re-blessed: corpus bundles that overconstrain are restated or dropped, each named. Poses unmoved |
| G | `patterns-are-index-variables` | `index(N)`, families, `Member` names; a pattern is a placement whose reads reach an index, its copies a pick family; `Mirror { body, plane }`; `PatternKind`, `Node::Pattern`, `PlacedUnion`, `Part`/`PartSelect::Instance` and `Frame`'s improper arm retire; an assertion reaching an index is one per member | **repetition is an index; no preset is stored** | re-blessed: names move (`Instance` → `Member`). Copies' geometry unmoved or moved by rounding, reported |

The dependency graph:

- **A** waits on stage 2 E (a pose read off a face reads a `Face` variable).
- **B** waits on stage 2 F (a mate's sides read `Face` variables) and on stage 4 J (`an-unattributed-contact-at-rest-is-a-finding`), which turns A5's hard error into a finding (§12, boundary).
- **C** waits on A and B.
- **D** waits on C.
- **D′** waits on D, and on the designer pairs' answers to §5′'s open questions.
- **E** waits on D′ for its computing frame. Its rider, `a-minted-reference-direction-follows-the-computing-axes`, waits on nothing and may land first, alone (§6).
- **F** waits on C and on stage 4 C (stage 4 D has merged). It has merged.
- **G** waits on D′, and on the one representation question it shares with E (§6, §8). FORK-DM4 is ruled and built (PRs 4527, 4674).

Why this order:

- **A and B are independent.** B moves ownership of today's mates and nothing in it reads a pose variable; A defines poses and changes no mate.
- **B before C.** A value is a constraint of a placement (FORK-S3M), so C cannot move today's in-plane offsets and rider into values before a `Place` exists to own them, and a mate may not hold a number. B therefore moves today's mate payload under `Place` unchanged, and C re-types it.
  - The cost is that the mate wire breaks twice (B moves it, C retypes it), and the old spec rejected this order for that reason. The alternatives are worse: one PR of about 350 files with no point at which poses can be compared, or numbers on a mate between the two units, which D10 forbids.
  - B's migration check (every MSOLVE fixture's `SolvedPoses`, unmoved or moved by rounding and reported) is easiest to read while the mate arithmetic is still today's.
- **C before D.** D re-spells every absolute datum and every `Transform` as a placement whose mates read poses off geometry, plus values. Before C a mate cannot read a pose or carry a value.
- **D before E.** E's default is "a construction computes in its own coordinates". Before D a construction computes in the world coordinates its absolute datum names.
- **F after C.** F's rule reads the kinds' families off the table that C re-keys by pose kind. It also waits on stage 4 C (stage 4 D has merged), whose rungs prove the contacts F's refusals leave behind, so they are not reported as unproven noise.
- **G after D (as first sliced).** A pattern places copies of one body by values over an index. That needs B's placement, C's values and D's "no construction reads a frame". G reads a union of a family, which is where FORK-DM4 sits.
- **D′ after D.** D′ is the placement pivot of row 162 (PR 5312), opened after this spec was sliced. D brought its data model forward (the mateless bundle); D′ builds the rest of `[placement]` on it.
- **E and G after D′.** E's "an operation over copies computes in its first operand's frame" names its operand by a pick, and G's pattern is a placement whose copies are a pick family: both read D′'s placement and picks. E and G both rewrite how a copy's body is held (`Copies`, `wire_place`), so the question they share is weighed once, before either dispatches.

Each intermediate state is a whole representation:

- After A, a pose is a defined variable, and every pose a reader reads is one. Absolute datums, and a `Datum::FaceFrame` read as a frame (a sketch on a face, a frame mate), are still the operations that define the poses constructions read: a named interim. C restates the frame mates and D retires the rest, with `Datum`.
- After B, a placement owns its mates, the world is a node, and copies take kinds from their bundles; constructions reading no body and absolute datums are all of the document's one kind until D (a named interim). A mate is still today's payload, an alignment with offset frames and its numbers, and may name the world as a base: a migration interim that C ends.
- After C, a mate equates two poses and values set freedoms. Constructions still read absolute datums, and `Transform` still stands. Until F, the refusals that need the bundle's residual are evaluation faults (a named interim): the door decides what the recipe shows, and the placement refuses the rest where it evaluates. A copy standing where its body stands is pinned by a world pin that reads nothing (a named interim, §4) until D, which spells it as a mate on the copy's own plane or axis, read off a face that reads no variable, with the constant `0` on the freedoms left (§5, fork-log row 159).
- After D, frames enter only at placement, and only a placement defines a copy, except `Node::Pattern` and `Node::PlacedUnion`, which still place their copies by a rule until G ends them (a named interim).
- After D, a bundle with no mate states its whole motion and `stand` writes the identity bundle, but a loose copy, a placement that reads shapes rather than a space, copies named by a `Placed`/`InPart` layer and read by `CopyKey`, and helpers that write turns read off geometry all remain (named interims, §5′). D′ ends them.
- After D′, every placement pins its copies and relates two spaces, and readers hold picks.
- After E, every operation computes in a frame of its reads.
- After F, a constraint any of whose equations is already fixed refuses, pinned or not.
- After G, the document stores no pattern preset.

**Rejected:**

- **Mates as poses (C) inside the placement unit (B).** About 350 files in one PR: every mate site, every gauge site and the refactor doors, with no point at which poses can be compared.
- **The old order, mates as poses before placements.** The in-plane numbers would sit on the mate until a placement exists, a second spelling D10 forbids.
- **D before C.** A construction placed against a face needs a mate reading the face's plane and values for the freedoms it leaves.
- **F inside B.** B's migration check reports every pose and product that moves; F restates bundles. Mixing them hides which change moved a body.

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

**Spaces** (D10 `[placement.spaces]`; row 162). A space is a coordinate system: the coordinates a construction is written in, or the world. A body is in exactly one space, decided from the recipe: a construction is in its own, a copy is in the space its placement places it into, and a body an operation builds is in the space of the bodies it reads. Any read across two spaces other than a placement's refuses at the door (`SpaceMismatch`) and is an evaluator assert anywhere else. Only a placement relates two spaces, and spaces form a DAG: a space's content may be placed into several spaces and into one space several times, and never reaches itself. A space's **carried bodies** are its bodies no operation of that space reads to build another. There is no loose copy: every placement pins its copies (below), so a space is a construction's or the world, never a copy's. A space nothing places, transitively, into the world is not in the product, is checked all the same, and is drawn from display state no logic reads (IDENTITY DI5).

**Placements** (B, C, D′; FORK-S3M, row 162).

- **The node.** A placement is an operation reading a **source** — a space of this document, or a space of another (an instance: its world or any other space of it) — and a bundle of constraints, and placing a copy of each of the source's carried bodies into a space under the one rigid motion its constraints state. How the recipe names the source and the space placed into is §5′ Q1; until D′, `Node::Place { shapes, constraints }` reads a list of shapes and its space is derived from its mates' targets (§5′, interim 2).
- **Picks** (row 165). A placement's copies are read through **picks**: a `Body` variable of the space it places into, defined as one copy, or a `Bodies` family read by index where the placement's reads reach an index (G). A pick's `of` is the path through the source's placements to the first variable it reaches, its one spelling. Readers hold the pick, and nothing else reads a placement's copies; a copy nobody picks is still a carried body of the space. A copy's cells keep their names in the source, and an operation reading two picks of one source keys each operand's names by the pick (`From { read, of }`). A pick authored twice is two variables; one whose body the source no longer has is unresolved, typed, never re-pointed.
- **Constraints.** A constraint is a mate or a value, addressed by (placement, an id minted with it), never by its position in the list and never a node. Which copy is defined is which placement reads it, so there is no tree, root or declaring role.
- **Pinned, always.** A placement's mates and values together set every freedom; a bundle that leaves one unset refuses at the door (`MateFault::Unpinned`), whatever its mates, so an unwritten freedom cannot be represented. A body's symmetry pins nothing: a value it makes unobservable is reported, never refused.
- **Overconstraint** (F; FORK-S3O). A constraint any of whose equations the others already fix refuses `Overconstrained { placement, constraints, free }`, pinned or not, decided by subgroup algebra without measuring. A mate that would take a freedom a value sets refuses the same. Where no direction is fixed, the spins are counted by axis: a fourth spin, or two in a row about one axis, refuses. Where two copies meet beyond what their mates fix is a contact, recorded at stage 4's door and linted, and its recourse is an assertion.
- **A placed copy's frame is not a variable.** It is the construction its bundle states, which the coincidence door replays at `Sym` (stage 4 H). Whether a copy is placed by no map is read off the recipe: only the identity bundle as written (no mate, the constant `0` on every freedom) places by no map, in every lane; every other placement runs its map (FORK-REST).
- **The carry** (PR 4666). A placement carries every pose of the source as the copy carries it, and the source's at-rest decisions about those bodies, its contact records and its census's findings, which stay the source's (`a-placement-carries-its-sources-at-rest-result`, after D′).

**Mates** (C; D10 `[placement.mates]`). A mate is `{ on, to }`, two poses of one kind; the kind is the primitive:

- `on` is read off the source's geometry; `to` is read off geometry of the space the source is placed into, or is a role of that space itself (its origin, an axis, a coordinate plane), moved by the target constructions inside the mate. Today that role is read of the world alone (`Target::World(WorldRead)`, Q7); D′ reads it of whichever space the placement places into. Neither side is a frame standing for a part's coordinates.
- `Frame`–`Frame` coincides, `Axis`–`Axis` is coaxial, `Plane`–`Plane` rests, `Point`–`Point` is a ball, and `Direction`–`Direction` is parallel. Each pins the relative pose to a coset of the kind's `Subgroup` (A11 (1)).
- A mate holds no number. Its sense is `Flip` on one side; a number a kind's own equation fixes (a standoff) is a construction on the target, never a value. A placement whose source touches nothing has no mate.
- `class: ContactClass` stays until stage 4's `mates-declare-no-contact` retires it.

**Values** (C, D, D′; D10 `[placement.values]`). A value sets one freedom of the placement's motion that its mates leave, a slide or a spin, to a `Length` or `Angle` variable, along or about an axis of the freedoms the mates leave: `Value { coord, by }`, `coord` one of `Slide(X | Y | Z)`, `Spin` (about a mated direction) and `SpinAbout(X | Y | Z)` (about the target space's own axis). How a value names the freedom it sets in a bundle holding a mate is §5′ Q2; D's head keeps `Value.mate`, an id of a mate in the same bundle, `None` where the bundle has no mate.

- **Where a mate fixes a direction** (a plane's normal, an axis, a lone direction), the chart is ruling C-1's: each side's frame is swung onto the mated direction `d` by the shortest turn carrying its own `z` to `d` (`reference_about`), the half-turn about its own `x` at `d = −z`; inside the sliver band about that one pole the chart refuses, typed at evaluation (`PoseDegenerate { construction: Chart }`). A slide is the copy's origin measured from the target's along the target's swung `x`, `y` or `z`; a spin is the angle about `d` from the target's swung `x` to the copy's. The mated direction is the target's, with a `Flip` normalised to the copy's side. A plane mate leaves `Slide(X)`, `Slide(Y)` and `Spin`; an axis mate `Slide(Z)` and `Spin`; a lone direction mate all three slides and `Spin`; a frame mate nothing.
- **Where no direction is fixed** (no mate, or a lone point mate), the rotation is a written chain of at most three `SpinAbout`s about the target space's `x`, `y` or `z` (through the mated point, for a point mate), no two in a row about one axis, applied in the order written; the slides run along the target space's axes. Turns do not commute, so the author writes the order and the kernel chooses none.
- A slide is a function of the relative pose alone, so no slide is ordered. Zero is always valid and says the two spaces agree as far as the mates allow; a placement whose values are all zero and which has no mate is the identity. **The resting convention:** a plain `Plane`–`Plane` mate equates outward normals; two faces stacked against each other carry a `Flip` on the target. A mate's `to` reading two bodies has no one body's frame to chart a value on (`NoChart`). The façade writes the constant `0` (VR6) for each freedom a gesture leaves unnamed. A placement is the only reader of a source's own coordinates, and reads them only through the freedoms its mates leave.

**The world** (B; D10 `[placement.world]`). One undeletable space. It defines no pose variable, so only placements (as the space a source is placed into, or through a mate's target) and export read it, and no construction does.

**The product** is the world's carried bodies, in placement order. Building, combining or placing shapes adds nothing to it. Export writes it and refuses an empty one, naming the bodies no placement relates to the world.

**The computing frame** (E; D10 `[placement.frame]`). An operation computes in a frame that is a function of what it reads and of nothing else, chosen so its arithmetic is well conditioned near the geometry it builds, and never in the world's. The frame is keyed with its inputs, so an edit that leaves an operation's reads alone moves none of its bits (D9). The frame is no part of the operation's meaning: the body up to that rigid map, its names and every verdict outside the sliver band are the same in any frame, and a minted reference direction is a function of the inputs, not of the axes. Where conditioning does not decide, a construction computes in its own coordinates and an operation over copies in its first operand's, as the author lists it. The at-rest census is order-free: each pair's verdict is the same in either member's frame, or the sliver band refuses.

**Repetition** (G; D10 `[repetition]`). `k = index(N)` defines a `Count` over `0..N`. A definition or operation whose reads reach `k` is evaluated once per value, and its outputs are families keyed by index tuples, outer first. A pattern is a placement whose reads reach an index, the index entering as a value (`SpinAbout`/`Spin` of `scalar(k)·turn/N`, a slide of `scalar(k)·pitch`) or through a mate's target (`bolt.axis ≡ holes[k].axis`); its copies are a pick family. No pose is constructed from an index. A mirror image is `Mirror { body, plane }`, a construction. The presets are the façade's, and the GUI recognises one for display only. Each member is its own arrival at rest, and an assertion whose reads reach the index is one assertion per member (D10 `[assertions]` "Families").

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
- **Values** (§1; ruling C-2): `Constraint { id, kind: Mate(Mate) | Value(Value) }`, `Value { mate, coord: Slide(X | Y | Z) | Spin, by: S }`, added by `DocEdit::AddValue`. The door (the named interim C-3) refuses what the recipe shows: a value naming no mate of its bundle (`ValueNamesNoMate`), a freedom its mate's kind does not leave (`NotAFreedom { kind }`, e.g. a plane mate's `Slide(Z)`), a lone point mate's rotation and a target read off two bodies (`NoChart`), and a number of another dimension. The placement refuses at evaluation what needs the bundle's residual: `Overconstrained`, `Contradictory`, a freedom the bundle's other constraints already fix (`NotAFreedom` with no kind), and a chart the band cannot swing (`PoseDegenerate { Chart }`). Until F, the refusals that need the bundle's residual are evaluation faults. The chart is one function, `reference_about(body_frame, axis)` (§1), and the solve places the folded copy at its values spins first, then slides, inside the folded residual.
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

## 5′. PR D′ — `a-placement-pins-by-written-values-without-a-mate` (cost H; design open)

This unit builds D10 `[placement]` as rows 162 and 165 left it, together with IDENTITY DI5 (`crates/editor-core/IDENTITY.md`). D already built its data model, the mateless bundle (§0), and D′ builds the rest on it. What D built and D′ keeps:

- `Value.mate` is optional. `Coord` holds `Slide(TargetAxis)`, `Spin` and `SpinAbout(TargetAxis)`.
- The door rule for a bundle with no mate is in `mate::solve`: `unpinned`, `unmated_bundle`, `identity_bundle`, `overconstrained` and `MateFault::Unpinned`.
- A bundle with no mate is evaluated by `mate::value::unmated_motion`.
- `DocEdit::AddValues` and `DocEdit::RemoveConstraints`.
- `mate::pin::Placed::{identity, write}`, and `stand`, which writes the identity bundle.
- FORK-REST: `SolvedPoses::at_identity` is a recipe query.
- Exact rational turns: `geom_core::Angle`, `VarEnv::turns` and Sym's `trig_at_rational_pi`.
- The writers of exact frames: `corner_frame`, `moved_frame` and `FrameMove`.

**What D's head carries, and what D′ makes of it.** Each numbered item below is an interim on D's head, and D′ ends every one of them.

1. **The loose copy.** `mate::solve::Space { World, Loose { root } }` (`mate/solve.rs`) uses one arm for two different things: a construction's own space, and a copy whose bundle does not pin it. An empty bundle folds to `Folded::Loose`, because `mate::solve::unpinned` admits it. A bundle that holds a mate need not write every freedom, because `unpinned` checks only bundles with no mate. The readers are:
   - `Evaluation::loose_copies` and `loose_roots`, and `spaces_of` and `root_space` (all `eval/mod.rs`);
   - `product::loose_spaces`, `LooseSpace` and `Product::spaces`;
   - `ExportError::Loose` (`pncad/src/export.rs`, `step_for_node` and `export_document_step`);
   - in the viewer, `display.rs`'s `loose_root` and `Unplaced`, the drawn filter in `pickindex.rs`, and the re-placement into a loose root by `place_against_in` in `session.rs`;
   - `refactor::inline`'s `in_loose_space`;
   - pncad-py's `Space::Loose` (`py/value.rs`, `py/mate.rs`).

   **D′:** `unpinned` checks every bundle, with mates or without, and refuses an empty one. `Folded::Loose` and the `Loose` arm retire, and `Space` names either a construction's space or the world (its spelling follows Q1). `loose_copies`, `loose_roots`, `loose_spaces`, `LooseSpace`, `Product::spaces` and `ExportError::Loose` retire too. A space outside the product is a construction's space, drawn as DI5 says (Q3). The viewer admits a free-move over a shown space (`display.rs`'s admission, `AdmissionFault`) and keys it by that space. The free-move may write display metadata outside the canonical bytes (DI5).

2. **The placement reads shapes, and its space is derived.** `Node::Place { shapes: Vec<BodyRead>, constraints }` (`node.rs`) copies each shape listed. Nothing writes the space it places into:
   - `spaces_of` and the fold in `wire_place` (`eval/wire.rs`, `fold_with`) put a bundle with no mate in the world, and a mated bundle in the space its first mate's target reads.
   - A bundle whose mates read two targets refuses with `MateFault::TargetsDiffer` (`a-bundle-folds-in-the-joined-space`).
   - An instance (`Node::InstantiatePart`) can place only its part's world.

   **D′:** a placement reads a source space and places a copy of each of its carried bodies into a space. Q1 decides how the recipe names both spaces.
   - The door refuses a placement whose source reaches the space it places into (the DAG must stay acyclic). This ends a loophole: `mate::pin::copy_of` tells a placed copy by `value.motion.is_some()`, so it takes a copy at the bit-exact identity as unplaced (`PinRefusal::AlreadyPlaced`). The D rows that rely on the loophole are restated: they are in `msolve1_transform_aware`, `msolve13_read_at_operand` and `mate1_member_vocab`.
   - Every mate of a bundle targets the space the placement places into, read off any body of that space. `TargetsDiffer` therefore retires, and the fold reads each target's pose in that space's coordinates.
   - A mate's `to` reads a role of that space (its origin, axes and coordinate planes), the way `Target::World(WorldRead)` reads the world's roles today.

3. **Copies are read by key, and their names are wrapped per copy.**
   - A copy is read by `BodyRead.key: Option<CopyKey>` (`operand.rs`, `CopyKey`; `SlotValue::Copy` in `edit.rs`). It is delivered as `Copies { bodies, members: Vec<KeyedCopy> }` (`eval/mod.rs`) through `eval/wire.rs`'s `copy_of`.
   - Its names are wrapped in `RoleSeg::Placed { of }` (`names/role.rs`, at `:1586`), or in `RoleSeg::InPart { of }` (`:1576`) under an instance. `StableName::copy_of` (`:2309`) reads them back, and `names/emit.rs` mints both wrappers at `:967–1110`.

   **D′ builds picks** (§1; row 165):
   - a variable definition for a pick;
   - readers that hold picks (a `BodyRead` with no key);
   - names that pass through under `From { read: pick }`;
   - the two rows D deleted while it waited for the ruling, rebuilt over picks: `msolve1_transform_aware::a3b_a_placed_copy_of_a_pattern_seats_and_gates` and `msolve13_read_at_operand::a1b_a_placement_above_the_operand_mints_nothing`.

   `RoleSeg::Placed`, `RoleSeg::InPart`, `CopyKey`, `StableName::copy_of`, `SlotValue::Copy` and `KeyedCopy.key` retire, along with the `Placed` and `InPart` arms of every rewriter and reader. Once they are gone, `RoleSeg::From`'s doc ("a placement adds no name segment") is true.

   Counts on D's head, to re-grep at dispatch:
   - `Placed` is named on 32 src and 38 test lines, and `InPart` on 35 and 40.
   - `CopyKey` is named on 43 src, 21 test, 7 viewer, 2 pncad and 2 pncad-py lines.
   - The readers outside `names/` are `coincide.rs`, `edit.rs` (`stranded_by_repoint`'s copy reach), `eval/parts.rs`, `eval/mod.rs`, `eval/class.rs`, `checks/at_rest.rs`, `product.rs`, `refactor.rs`, `resolve/mod.rs`, `mate/anchor.rs` and `mate/pin.rs`, plus the viewer's `session/select.rs`, `session/op.rs`, `app.rs` and `world.rs`.

   `name_words_corpus`'s `NAME_WORDS` and `OVER_BUDGET` are ratchets, so they re-tighten (`name-words-budget-grew-under-placed-copy-wrapping`).

4. **A value names its mate, and a point mate's rotation has no chart.** `Value.mate` names a mate wherever the bundle holds one. In `mate.rs`, `NoChartWhy::BesideAMate` refuses a value that names no mate beside a mate, and `NoChartWhy::PointRotation` refuses any value on a lone point mate's rotation.

   **D′:** a lone point mate leaves its rotation to a chain of at most three `SpinAbout`s about the target space's axes, through the mated point. The chain is evaluated about the point, the way `mate::value::unmated_motion` evaluates a chain with no mate, and `PointRotation` retires. The bundle rule counts spins by axis wherever no direction is fixed, including a point mate's chain (`mate::solve::equations_of`, `already_fixed`, `overconstrained`): a fourth spin, or two in a row about one axis, refuses as `Overconstrained`. How a value names its freedom in a bundle that holds a mate is Q2, and `BesideAMate` and `Value.mate` follow its answer.

5. **The helpers write turns read off geometry.** `mate::pin` reads an evaluated pose (`Frame`) and writes back the numbers it reads off it as typed literals:
   - `turns` and `moved_by` write the world read's spins, taken by `atan2`;
   - `moved_pose` writes `frame_at`'s picked frame;
   - `written`, and `Chart::spin_at` and `slide_at`, write the plane path's values.

   The helpers also choose geometry by heuristics the author never stated:
   - `mate::anchor::Anchors` (`fixed`, `unmoved`, and `part_start_cap`, the rule for an instance's anchor);
   - `picked_frame`, which takes the first axis-bearing edge by id and a vertex off it;
   - "the first planar face by id";
   - the `own_axis` / `axis_in_world` / `axis_placed` fallback for a root with no plane (row 161 Q1).

   `PlacedBy::{PlanarFace, Axis, Pin}` reports which choice was taken. D's exact-turn lane moved the callers that mean an exact frame onto `corner_frame`, `moved_frame` and `FrameMove`, or onto a bundle with no mate spun by `turn/4`. These four helpers still read turns back (call lines on D's head):

   | helper | src | test | viewer | pncad-py | tour |
   |---|---|---|---|---|---|
   | `place_at` | 4 | 15 | 2 | 7 | 3 |
   | `pin_at` | 9 | 23 | 3 | 0 | 4 |
   | `frame_at` | 13 | 39 | 2 | 8 | 2 |
   | `place_against` | 4 | 6 | 3 | 27 | 1 |

   **D′:** no helper writes a number it read off geometry. Q4 decides what each helper takes from its caller, and which helpers survive. `Anchors`, `part_start_cap`, `picked_frame`'s rule, the fallback for roots with no plane and the reports in `PlacedBy` retire with whatever Q4 retires. `stand` stays, writing the identity bundle.

6. **Inline at a moved landing.** When an instance stands off the identity (`Landing::Pinned(Some(motion))`), `refactor::inline` handles each part placement whose world bundle holds only numbers by re-pinning it with `pin_at` at the copy's moved pose. Where the copy offers no frame, that re-pin refuses with `InlineError::RepinRefused`. Inline refuses outright a part placement that reads a named or toleranced variable (`InlineError::ValueOnAMovedWorld`), and `world_pin_of` refuses an instance pin that reads a variable (`PinReadsVariable`). `mate::pin::written_moves` flattens the moved world read.

   **D′:** inline lands every part placement so that every variable it read still drives its copy after the splice, and it refuses no landing because of its numbers. `ValueOnAMovedWorld`, `RepinRefused` and the re-pin retire. Q5 decides how the landing's motion enters the spliced document.

7. **The viewer.** It inserts every new body by `stand`. Its mate tool adds a mate to a bundle that already pins its copy (`the-viewer-cannot-mate-two-freshly-inserted-parts`). Its authoring forms build `Operand`s, so they cannot read one copy (`the-viewer-authors-no-read-of-one-copy`).

   **D′:** the mate tool writes the mate and removes the values whose freedoms the mate fixes, as one edit. The forms read picks.

**Not D′'s.**
- D′ leaves the box lanes' chart for a mated direction (`Chart::turned`, `mate::solve::tightened`, `spins_by_frames`) as it is, unless Q2's answer rewrites the mated chart. If it does, D′ absorbs `sym-solve-atan2-has-no-branch-rule-about-pi` and `a-slide-value-doubles-its-translations-width`; otherwise they stay rows of their own.
- The census per space, and the carry of a source's at-rest result, are units of their own after D′ (§12, boundaries).

**Absorbed.** Each of these closes with D′:
- `a-stand-on-an-instance-reads-its-part-by-the-names-shape`, `a-value-charted-on-the-target-reads-frame-carries-the-datum-chain`, `an-inline-at-a-moved-landing-reading-a-variable-refuses` and `inline-refuses-a-placed-part-off-the-identity`, all four superseded by row 162;
- `name-words-budget-grew-under-placed-copy-wrapping`;
- `a-space-names-its-bodies` and `a-placement-relates-two-frames`, the rulings D′ builds;
- `a-bundle-folds-in-the-joined-space` (interim 2);
- `a-placement-copy-is-its-own-table-in-the-product` (`Placed` retires);
- `an-instance-defines-one-body-per-part-placement` (an instance copies each carried body of the space it places, read by picks);
- `the-viewer-authors-no-read-of-one-copy` and `the-viewer-cannot-mate-two-freshly-inserted-parts` (interim 7).

**Order of work inside D′.**
- (a) Q1's spelling of the source and target spaces, the door's acyclicity check, and `Space` without `Loose` (interims 1 and 2). Everything else reads these.
- (b) Picks (interim 3), on top of (a).
- (c) The bundle rule for every bundle, and the point mate's chain (interim 4). Independent of (b) once (a) lands.
- (d) The helpers (interim 5), after (a) and Q4.
- (e) Inline (interim 6), after (b) and (d).
- (f) The viewer and the façades (interim 7), last.

(c) and (d) can run beside (b) in sub-lanes. Two pairs share files: (b) and (e) both edit `refactor.rs`, and (b) and (f) both edit the viewer's read sites.

**Re-bless.**
- Names move wherever a copy is named, since the `Placed`/`InPart` layer is gone.
- Ids move wherever a placement's constraints or a pick change what is minted.
- The stored documents (`golden.cad`, `die_tool.pncad`, `plate_param.pncad`, `gallery_ring.pncad`, `die_composed_tour.pncad`) are regenerated from their generators. A document in the old wire format refuses, typed, at load (DESIGN Band 4).
- Geometry does not move, except where a helper used to write a turn rounded off geometry and its caller now writes the turn it meant. Each such move is reported by document and node.

**Open design questions** (for designer pairs; stated as problems).

1. **How the recipe names a placement's source and the space it places into.** D10 makes a placement read a source space and place that space's carried bodies into a space. It also makes an instance place "another document's world or any space of it". But `Node::Place` reads a list of shapes, and nothing writes the space it places into: a bundle with no mate goes to the world (`spaces_of`, `fold_with`). So a placement with no mate cannot place into a construction's space, and an instance cannot place any space of its part except the part's world. A space is not a variable kind (`[variables.kinds]`), so nothing in the recipe can name one.
2. **How a value names the freedom it sets in a bundle that holds a mate.** Row 162 removed "a value names its mate" from D10. It charts the freedoms the mates leave on the target's axes, swung onto the direction the mates fix. The code still makes each value name one mate (`Value.mate`), and `NoChartWhy::BesideAMate` refuses a value that names none. Where two mates of a bundle fix different directions, or one fixes a direction and another fixes a point, nothing decides which swung axes chart the freedoms that are left.
3. **What the viewer shows besides the world.** Today it draws the world's placements and each loose copy's space (`pickindex.rs`). Once loose copies retire, every construction is a space of its own that nothing places into the world until a placement does. DI5 says only that several shown spaces may overlap. Nothing decides which spaces are shown, or how a new body the author has not yet placed appears.
4. **What a placement helper takes from its caller.** `place_at`, `pin_at`, `frame_at` and `place_against` take a pose as an evaluated `Frame` and write turns and lengths read back off it by `atan2` and projection. They read those numbers off geometry they choose by heuristics (`Anchors`, the first planar face by id, the first axis-bearing edge by id). Under D10 `[variables.literals]` and the exact turns D built, a turn is exact only where the author wrote it, and a placement with no mate can now state any motion by values. Three things are undecided: what each helper's caller supplies, so that every number it writes is one an author wrote; which helpers still have a caller once a bundle with no mate can say the same motion; and what the "place part" gesture writes.
5. **What inline makes of a moved landing.** An instance places its part's world into the host under the instance's bundle. Splicing the part in puts the part's placements into the host's world, where each one must also carry the instance's motion. But a spin chain composed with another chain is not, in general, a chain of three spins over the same variables. So writing the landing's motion into each part placement's values would either read turns off numbers or lose the variables those values read. And D10's spaces are a construction's coordinates or the world, so the part's world has no space of its own to become in the host.

## 6. PR E — `an-operation-computes-in-a-frame-of-its-reads` (cost H; design open)

E builds D10 `[placement.frame]` (FORK-S3P). Its core waits on D′. Its rider does not wait on anything.

**State on D's head.**
- A construction already computes in its own coordinates, because D made every construction the root of its own space.
- A placement maps each copy into the space it places into, eagerly. `wire_place` (`eval/wire.rs`) folds the bundle and maps the source body by `topo::transform::transform_rigid`, so a `Copies` value holds its bodies in that space's coordinates. Every operation that reads copies therefore computes in that space's coordinates, which for the product are the world's.
- No value records the frame its body is in.
- The product's bodies (`product.rs`, `product_recorded` and `product_in`) and their digests are in world coordinates. Export (`pncad/src/export.rs`, `export_document_step` and `step_for_node`) writes the bodies as they stand.
- `SolvedPoses` (`mate/solve.rs`) answers `relative_map` and `relative` per placement. It has no `world_of`.
- Minted reference directions branch on the coordinate axes: `Vec3::orthonormal_basis` (`geom-core/src/linalg/vec.rs`, `:640`) and `UnitVec3::orthonormal_basis` (`unit_vec.rs`, `:353`).

**Build.**
- **The computing frame.**
  - Each operation computes in a frame that is a function of its reads, keyed with them (D9). Where that frame is held, and who composes it for a reader, is Q1, which G shares.
  - A construction computes in its own coordinates.
  - A copy of a body is that body built once and mapped (D10 `[repetition]`), never rebuilt in the space it is placed into.
  - An operation over copies computes in the frame of its first listed operand (a pick, after D′) and reads every other operand through the relative map.
  - A value-informed refinement is allowed: re-centre on the operands' bounds, snapped to a power-of-two grid at their scale, with the author's order breaking ties. It is measured on the far-from-origin rows, starting with `a-far-meeting-point-fails-membership-by-its-own-rounding`.
- **Readers compose.** Any reader that needs two bodies in one space composes the two bodies' frames. Those readers are the at-rest census, `Measure`, a mate's pose read off a copy, the coincidence door's replay and the viewer's drawing. Nothing reads a body's coordinates as its space's without its frame. The census is order-free: each pair's verdict is the same in either member's frame, or the band refuses.
- **The world's map.** `SolvedPoses::world_of(copy) -> Frame` (new, `mate/solve.rs`) is the one door that composes a body's frame up to the world. Export (`pncad/src/export.rs`) reads it, and nothing else in `crates/{editor-core,pncad,pncad-py,viewer}/src` does. A gate script beside `scripts/gates/test-features-dev-only.sh` holds that rule. Q3 asks how the viewer draws the world's space under it.
- **Minted reference directions** (the rider, `a-minted-reference-direction-follows-the-computing-axes`). Every carrier an operation mints with a reference direction from `orthonormal_basis` takes that direction from its inputs instead.
  - FORK-S3P's designers named three sites:
    - a sweep cap's plane (`sweep/src/swept.rs`, `:884`), which takes it from the path frame;
    - a Newell plane (`geom-brep/src/newell.rs`, `:169`), from its loop;
    - a split's one-corner section (`topo/src/splitting/section_loops.rs`, the `chord_u_ref` fallback at `:77`), from the cutting plane.
  - Those three are not all of them. `orthonormal_basis` is also read in `topo/src/boolean/` (`join.rs`, `boxes.rs`, `reduce.rs`, `ops.rs`, `circle_torus.rs` and its `shallow_sweep.rs`, `conic_quadric/`, `section_cert.rs` and `section_cert/`), in `topo/src/ring_path/cone_islands.rs`, `topo/src/side_reader.rs`, `mesh/src/planar.rs` and `step-import/src/recognize.rs`.
  - The rider classifies each caller, and the PR carries the table. A caller whose basis reaches an output carrier's stored reference is re-derived: `join.rs`'s `u_ref` at `:7237`, and `boxes.rs`'s at `:4444`, `:4934`, `:5142` and `:5266`, among others. A caller whose basis is scratch that no output keeps is left as it is.
  - Q2 asks which input supplies the reference where the inputs carry none.
  - The rider touches none of D′'s or G's ground, so it can be dispatched now, on its own, and it closes in its own PR.
- **What moves.** Product body digests move into computing-frame coordinates. STEP bytes move by rounding (one more composition), and so may a measured value across copies. Test 13 checks each moved digest against the old one mapped through `world_of`. Its two-frame row computes every corpus operation in its own frame and in a second frame, rotated by an irrational angle and translated, and expects the same body up to that map, the same names, and the same verdicts outside the band.

**Order.**
1. The rider, now, on its own.
2. After D′, one PR carrying the computing frame, the readers, the `world_of` gate and export. A frame carried halfway would be two representations, and export moves with the representation.

**Parallelism.**
- The rider is disjoint from D′ and from G. Its ground is topo, sweep, geom-brep, mesh and step-import.
- E's core waits on D′, because both rewrite `wire_place`, `Copies`, `product.rs` and the viewer's drawing.
- E's core and G both change how a copy's body is held (G builds copies of one body once and maps them). Once Q1 is answered for both, they can run concurrently: one owns `wire_place` and `Copies`, and the other rebases onto it.

**Open design questions.**
1. **How a value holds its body and the frame it is computed in.** Today every value's body is in its space's coordinates, because `wire_place` maps each copy by `transform_rigid`. So no value says which frame its arithmetic ran in, and no copy shares its source's body. E needs an operation's frame to be a function of its reads, with the body it reads given up to that map. G needs copies of one body built once and mapped. Who holds the frame, who composes it for a reader, and what a content key and a digest range over are all undecided.
2. **Where a minted reference comes from when the inputs carry none.** D10 says a minted reference direction is a function of the inputs. But some carriers the kernel mints read inputs that carry no reference along the axis that needs one, such as a Newell plane from a loop or a boolean-minted circle on a plane. Others read inputs whose own reference was itself minted. At each such site, which input supplies the reference, and which of its features, is undecided.
3. **How the world's space is drawn without reading the world's map.** E's gate admits no `world_of` caller outside export, and §12 Q5 says the viewer draws every space from display state that no logic reads. The product is the world's carried bodies, and each body is now held in a computing frame. Drawing them where they stand needs each body's frame up to the world, which is exactly what `world_of` answers. Which readers other than export may compose a body into the world's coordinates is undecided.

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

## 8. PR G — `patterns-are-index-variables` (cost H; design open)

G builds D10 `[repetition]`, the sentence in `[placement.placement]` that makes a pattern a placement whose reads reach an index, and `[assertions]` "Families". It rests on three rulings: FORK-PAT (row 99), FORK-DM4 (ruled and built) and row 165 (a pattern's picks are a `Bodies` family). `work/intent/patterns-are-index-variables.md` records FORK-PAT's build. Where that item and this section differ, this section is the version restated against D's head.

**State on D's head.**
- **Pattern nodes.** `Node::Pattern { input, count, kind }` and `Node::PlacedUnion { input, count: Option, kind }` (`node.rs`) place copies by a rule. The rule is `PatternKind::{Linear { direction: [S; 3], spacing }, Circular { axis, step }, Explicit(Vec<placement::Frame>)}`. `Linear`'s `direction` holds three components of a direction in slots, which FORK-MOVE forbids. This is a named interim on D's head, and G removes it.
- **Evaluation.** `wire_pattern`, `wire_placed_union` and `place_each` (`eval/wire.rs`), with `stepped_map` and `eval/wire/stepped.rs` (`stepped_rule_map`, `SteppedOperands`, `turns_held`).
- **Names and selection.** `name_pattern` and `name_placed_union` (`names/emit.rs`, `:837` and `:897`) mint `RoleSeg::Instance { i, of }`. One body is selected by `Node::Part { select: PartSelect::Instance(i) }`, DM3 as it stood.
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
  - Its reads reach an index. The index enters through a value (a `SpinAbout` or `Spin` of `scalar(k)·turn/N`, or a slide of `scalar(k)·pitch`), or through a mate's target (`bolt.axis ≡ holes[k].axis`).
  - Its copies are read through a `Bodies` pick family (row 165), and `p[k]` is one member.
  - Copies of one body are built once and mapped. How the copy is held is §6 Q1.
  - A construction whose scalar inputs read the index is built once per member.
- **Names.**
  - `RoleSeg::Instance { i, of }` becomes `Member { (i, j, …), of }`, keyed by the index variables' ids and the integers.
  - P5's placement-major layout goes, and the `SegPat` index predicate matches `Member` by tuple.
  - Split's re-anchor matches a member by its index variable and integer, never by an evaluated count. This closes `a-split-name-picks-a-pattern-copy-by-its-evaluated-index`.
  - A mate target on a member is a pick, so its value is charted on that member's frame. This closes `a-value-on-a-placed-copy-target-is-charted-on-the-space`.
- **Mirror.**
  - `Node::Mirror { body, plane }` is a construction that defines a new `Body` in its source's root. Its plane is a `Plane` pose whose reads reach that root alone (MIRROR-DESIGN P1–P4, P6).
  - `Frame`'s improper arm retires, along with `Frame::admission_fault`'s improper case, `EditError::ImproperPlacement` and pncad-py's `mirror_across_plane`. Every pose is then proper.
- **`PlacedUnion` retires** (`placed-union-places-and-fuses-in-one-node`).
  - It becomes a placement of the copies and a `Union` reading their pick family.
  - Its `Separation` certificate becomes the union's fast path wherever the union reads one pick family of one placement. Its members are then rigid images of one body, which the reads show.
  - An uncertified pair goes through DM4's pairwise pass instead of refusing.
  - Three texts are re-worded with the code: `crates/editor-core/README.md`'s section "The group boolean: `PlacedUnion`", that page's code-map row, and DESIGN.md's companion-table row for it.
- **Explicit frames** (`explicit-placement-frames-hold-floats`).
  - An irregular family is its members written out, each saying where it is (D10 `[repetition]`). So `PatternKind::Explicit(frames)` becomes one placement per frame, gathered by `Union([...])`.
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
- G's (a) and (b) need D′: a pattern's copies are a pick family, and a pattern is a D′ placement. They also rewrite files D′ rewrites (`names/role.rs`, `names/emit.rs`, `eval/wire.rs`, `node.rs`, `refactor.rs`). So G follows D′.
- Once §6 Q1 is answered, G can run beside E's core. The seam is `wire_place` and `Copies`.
- `Mirror` is additive beside D′: one new variant in `node.rs`, a new wire function, an entry in the content key's tag list, the façade's `mirror` and pncad-py's frame doors. It can run in parallel with D′ or with E.

**Open design questions.**
1. **Whether a ring closes structurally.**
   - Two members under one `Axis` mate differ only by their spin values, so `N · (turn/N) ≡ turn` is a comparison of angles modulo a turn.
   - The symbolic tier proves a coincidence as a polynomial identity over ℚ in matrix entries. It folds `sin` and `cos` of a rational multiple of π only where the value is rational (D's `trig_at_rational_pi`).
   - So for most `N` it cannot prove that member `N − 1`'s neighbour is member 0.
   - Measure first: a ring of 12 under the `unproven-coincidence` lint. If the tier cannot prove it, how a pattern's closure is proved is undecided.
2. **How a copy's body is held.** This is §6 Q1, shared with E.

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

D′ is not a column above: it moves names (the `Placed`/`InPart` layer goes), ids and the stored documents, which are regenerated; its geometry is unmoved except where a helper's rounded turn is replaced by the turn its caller meant, each reported (§5′).

**f64 geometry moves only where a unit says so.** In A, B, F and G a body digest, a measured bit or a solved pose that changes is a bug. C, D and E state what moves and by which map, and their tests check each moved value against that map.

## 10. Test plan (each row names the runtime value that breaks it)

1. **(A) A face reads as a plane.** `Plane { face }` on a cylinder's flat cap equals the cap's carrier plane with its outward normal; on the curved face it refuses DM1b's typed error. *Breaks if* a reader reads the carrier's u-reference (a roll appears in a value that should have none).
2. **(A) A slot holds its kind.** `Split { tool: <an Axis variable> }` refuses `SlotVarKind { expected: Plane }`; `<frame> through xy` is accepted. *Breaks if* a finer value is admitted where a coarser one is asked for without its projection.
3. **(A) A combination refuses its degenerate case.** `Through { axis, point }` with the point on the axis refuses `PoseDegenerate { construction: Through }` at evaluation, at the scalar that decides it. *Breaks if* it picks a frame anyway (an arbitrary roll).
4. **(A) The revolve's axis cannot leave its plane.** `Revolve { axis_through: [0, 0], axis_heading: turn/4 }`: the `axis: Axis` output lies in the profile's plane for every value in a box run (its direction's margin against the plane's normal is identically zero in `Sym`). *Breaks if* the axis is lifted through 3-D coordinates.
5. **(B) Two placements, two copies.** Two `Place`s of one body, each against the world: two copies, distinct output ids, each name resolving once per copy. *Breaks if* the bundle is keyed by body, not by placement.
6. **(B) The product is the world's kind.** A copy pinned against a copy pinned against the world is in the product. Two copies pinned only against each other are one loose space: not in the product, `EmptyProduct` names them, and the census between them runs. A copy whose bundle pins less than its body needs is a loose copy, not a refusal. *Breaks if* membership is "has a bundle" rather than "reaches the world", or an under-pinned copy refuses.
7. **(B) The one-time migration.** For every corpus `.pncad` and every MSOLVE fixture, `SolvedPoses::placement` for every copy and `product_recorded`'s digests in order are unmoved or moved by rounding, and the report names every move. *Breaks if* a tree mate goes into the parent's bundle instead of the child's, or a gauge chain is composed outer-first.
8. **(C) The mate respelling.** Every MSOLVE fixture's solved poses agree with pre-C within the rounding of one value re-composition, and the report names every dropped mate. *Breaks if* a migrated standoff lands in the other side's frame (a non-zero standoff flips sign).
9. **(C) Values.** A `Plane` mate leaves two slides and a spin. Setting each to zero places the copy with its own coordinates agreeing with the target's in the plane, whatever order the values are listed in. A spin value on a lone point mate refuses `NoChart`. A second mate taking a freedom a value sets refuses `NotAFreedom` where the placement evaluates (until F, which refuses it `Overconstrained` at the door). *Breaks if* a slide is charted along the other slide's moved axis (order-dependent).
10. **(D) A construction is placed, not framed.** The corpus's sketch-on-face bodies: each copy's digest equals the pre-D digest within one composition, and editing a value moves the copy, not the construction (the construction's digest is unmoved). *Breaks if* a construction reads the frame it is placed by.
11. **(D, G) Nothing moves a body.** No variant reads a body and defines it moved except `Place`, and, until G, `Pattern` and `PlacedUnion`; from G the exemption is gone. A second placement of a copy is a second copy. *Breaks if* `Transform` survives under another name (a census of `Node` variants reading a `Body` and a rigid motion).
12. **(B, E) The world is read by placements and export alone.** `Delete(world)` refuses `WorldIsUndeletable`. A construction reading the world refuses at the door. The gate admits no `world_of` caller outside `pncad/src/export.rs`. *Breaks if* the world is admitted as an ordinary pose.
13. **(E) The computing frame is no part of meaning.** Every corpus operation computed in its frame and in a second frame (the first rotated by an irrational angle and translated) gives the same body up to that map, the same names, and the same verdicts outside the sliver band. Moving a document's only world mate moves no body digest and no measured bit, and moves the STEP bytes. *Breaks if* a minted u-reference follows the axes (a seam moves, a name changes) or a path composes the world's map.
14. **(F) A constraint already fixed refuses, measure-free.** A copy pinned by a mate and values: inserting any further mate refuses `Overconstrained { held: Trivial }` with an empty predicate log. A `Plane` mate, then a second `Plane` mate on any other face, refuses whatever the faces' angle. *Breaks if* the door folds the new mate before checking codimension, or admits a redundant mate whose offset agrees (the fallback Ev rejected).
15. **(F) A contact beyond the mates.** Two copies resting on a second pair of faces their mates do not fix: the census records the contact and the lint reports it; an assertion on its `Gap` quiets it. *Breaks if* the contact is minted as a declaration.
16. **(G) A ring is an index.** `circular_pattern(body, axis, 12)` writes an `Axis` mate and `spin = scalar(k)·turn/12`, no preset; the copies are unmoved or moved by rounding, reported; `Member { (k), of }` names resolve per member. *Breaks if* a preset tag is stored or the copies are built twelve times.
17. **(A–G) Python.** `doc.plane(face)` returns a `Var` of kind `Plane`; `doc.place([body], [...])` builds a copy. The census and `.pyi` agree, and every gauge, datum, transform and pattern door is absent from the census. *Breaks if* a gauge, datum, transform or pattern door is still exported by the `.pyi` or the census.
18. **(D′) Every freedom is written.** An empty bundle, and a plane mate with only one of its three values, each refuse `Unpinned` at the door. No evaluation of any corpus document has a `Space` other than a construction's or the world's. *Breaks if* any bundle that leaves a freedom unset folds (a loose copy survives under another name).
19. **(D′) Picks.** Two placements of one body into one space, read by two picks, are unioned. The result's names key each operand by its pick (`From { read }`) and carry no `Placed` or `InPart` segment. Picking one copy twice gives two variables. *Breaks if* a name spells the placement, or a reader reads a placement's copy without a pick.
20. **(D′) A point mate's chain.** A lone `Point`–`Point` mate plus `SpinAbout(Z)`, `SpinAbout(X)` and three slides pins its copy, turned about the mated point in the order written; adding a `SpinAbout(X)` right after the `SpinAbout(X)` refuses `Overconstrained`. *Breaks if* the chain turns about the copy's origin rather than the mated point, or the order is the kernel's.
21. **(D′) Inline keeps a variable driving.** A part whose placement's slide reads a named variable, under an instance whose bundle turns it by `turn/4`, is inlined. Moving the variable then moves the spliced copy as it moved the part's copy before the splice. *Breaks if* inline refuses, or writes the variable's current value as a number.
22. **(G) Per-member assertions.** One authored assertion over `fins[k]` and the base quiets all `N` interference findings, and still quiets them after `N` changes from 12 to 16. *Breaks if* the count is fixed at authoring, or one member's finding is quieted by another member's assertion.

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
- **Load.** Seven units (H, H, H, H, M, M, H: 30 points by the work README's weights) are filed `parked` behind stage 2 and so count nothing until it closes. At stage 2's close the orchestrator splits stage 3 into its own program or confirms it fits.
- **D′'s reach.** D′ changes how a placement names its source and its target space, and how every reader reads a copy. That touches about as many files as B did, through `names/`, `refactor.rs`, the viewer's read sites and the façades. Its order of work (§5′) is the split it allows: each step leaves one way to read a copy.

## 12. Open questions

### The forks are settled

Each of the six forks this spec opened was weighed by a designer pair and ruled by Ev:

- **FORK-S3-1** (what a free pose is) and **FORK-S3-6** (what a space's member is): FORK-S3P, fork log row 95, PR 4324. No pose is free; there is no seed and no member order, and an operation computes in a frame of its reads.
- **FORK-S3-4** (what overconstraint means): FORK-S3O, row 96, PR 4325. Pinned or not, by codimension, measuring nothing.
- **FORK-S3-2** (what a placement is), **FORK-S3-3** (what a mate reads) and **FORK-S3-5** (what placing a multi-body instance copies): FORK-S3M, row 97, PR 4326. `Place` owns mates and values, reads a list of shapes, and an instance keeps one output per world placement.

FORK-PAT (row 99, PR 4341) added unit G.

### Questions with a recommendation (not forks)

1. **Which constructions A builds.** **Recommendation:** the reads off geometry, reads through a projection role, `Slide`, `Spin`, `Flip`, `Through` and `Meet`. The rest wait for a reader. The extrude's slant is built when a corpus document or a façade gesture writes one.
2. **`Point` and `Direction` subgroups.** **Recommendation:** both in A, because D10 names a lone point mate and its direction-mate lowering.
3. **A clocked coaxial** (today's `Coaxial` plus rider, residual `Prismatic`). **Recommendation:** an `Axis`–`Axis` mate plus a slide and a spin value (C), with the `Cylindrical` residual's chart in the shared `Subgroup`.
4. **Where the door reads the held fold** (F). **Recommendation:** fold the bundle at the door from its definitions. It is a handful of table rows, and it keeps "the doors decide edits" free of a stored solve.
5. **The viewer and the world.** **Recommendation:** the viewer reads no `world_of`. It draws every space from display state of its own that no logic reads, the rule G3's free-move probe already follows, and E's grep gate admits no display door. §6 Q3 re-opens this for the world's own space, whose bodies are held in computing frames after E.
6. **`Transform`'s Python spelling.** **Recommendation:** none. `body.transform(by)` goes with the node; the façade's `place` with values is the one spelling (Ev on #4220: one semantics in Python and Rust).
7. **What a mate reads on the world** (orchestrator's ruling, 2026-10-09). The world defines no pose variable, and a placement with no mates cannot chart a rotation. Ev: "the world is one frame among many that cannot be deleted" (#4326). A mate's `to` may read the world node through the same pose reads any target offers: its frame, a plane, an axis or its origin, and the target constructions (a world plane slid along its normal, the world's frame slid and spun, `Flip`). The read and its constructions are part of the mate (`WorldRead`) and never a variable, so D10's "defines no pose variable" holds and nothing else can read it. So a copy at any world pose is a world placement: a `Plane` mate to a world plane plus slides and a spin, or a `Frame` mate to the world's frame spun and slid, as `Spin { world, about: world through x, by: 30° }`.
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
  - Its unit C (`the-at-rest-census-is-a-check`) checks per space and names F as its stage-3 trigger, which stands.
  - Its "copy" is "a `Body` output of a world placement"; this spec keeps that: a copy is an output of `Place` or of an instance.
- **Stage 6** owns coaxiality through one axis variable and the tangency constructions. A builds the `Axis` reads they use.

- **The census per space and the carry** (PR 4666; `each-space-runs-its-census-between-arrivals`, P1 H, and `a-placement-carries-its-sources-at-rest-result`, P1 M). Neither can start before D′. They share D′'s ground at the following sites:
  - `product.rs`: `product_recorded`'s call to `topo::record_at_rest` with its `groups` closure (`checks/at_rest.rs` `copy_groups`), `product_in`, and `loose_spaces`/`LooseSpace`, which D′ retires;
  - `eval/wire.rs` `wire_place`, where a placement's value is built and the carry has to ride;
  - `eval/parts.rs` `PartValue`, the instance seam the carry generalises;
  - `eval/mod.rs`'s spaces (`spaces_of`, `Space`).

  The census decides "per space", and on D's head a space is `Space::{World, Loose}`, which D′ replaces. A finding names its cells on the space's bodies by picks (D′'s). "A space placed n times is decided once" needs a placement whose source is a space (D′). The carry crosses every placement as one type, which `wire_place`'s value after D′ has to hold, and §5′ leaves that slot open. So:
  1. D′;
  2. the carry, whose solid-pair match becomes the census's confirm-and-match arm;
  3. the census per space.

  Before G, a `Pattern`'s or `PlacedUnion`'s copies arrive as one node's output, so the census would treat them as one arrival and decide nothing between members. The census unit therefore treats each such copy as its own arrival, a named interim that G removes, because G makes each member an arrival by construction. The carry and the census share their ground with D′ and with each other, and the carry overlaps E's core at `wire_place`, so neither runs beside D′ or E's core.

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
- `a-minted-reference-direction-follows-the-computing-axes` closes at E.
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
- **`a-placement-pins-by-written-values-without-a-mate`'s site list has drifted.**
  - Its line citations moved on D's head: `names/role.rs` `:1579`/`:2287`/`:2302` are now `:1576`/`:1586`/`:2298`/`:2309`, `names/emit.rs` `:919–1000` is now `:967–1110`, and `eval/wire.rs` `:2575` is now `copy_of` at `:2742`.
  - Its "callers left on them" list names `die_tool`'s master on `place_against` and its rule frames on `Frame::rotate_then_translate` from radians. D's exact-turn lane (`5cf76d715e`) moved both: the master stands on `corner_frame` by an axis mate, and the rule frames come from `rotate_by_turns_then_translate`.

  §5′ restates the sites by symbol.
- **`an-operation-computes-in-a-frame-of-its-reads`** and its rider name three minted-reference sites. `orthonormal_basis` reaches output carriers' `u_ref` in topo's boolean emitters too (§6).
- **`a-mate-relates-two-poses` reads `status: review`.** Its PR, #5267, merged on 2026-10-18. `transform-retires-into-a-placement` is `parked` on it while a lane builds D. Both headers are the orchestrator's to settle, since a closed blocker under a `parked` row is a lint error.
