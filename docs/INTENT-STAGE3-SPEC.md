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

**Not ratified, not built on.** FORK-DM4 (`a-union-member-is-keyed-by-its-read`: union member keys, `union` and `intersect` n-ary, `Boolean` leaving the document) is open. G is the one unit that touches a union, and it waits on that ruling (§8).

D10's last paragraph retires, here, **ASSEMBLY A3**, **A11 (2)'s gauges and offsets** and **A11 (4)'s declaring mates**. §9 quotes each clause this stage retires or rewrites.

**Baseline.** This spec is written against stage 2's **final shapes**, not today's code: after stage 2 F, every operand is a read, `Select` defines `Face`/`Edge` variables, and mate sides read `Face` variables. Stage 2 C's `PlaceInWorld` is not on main yet (`node.rs` has no such variant), so §3 retires it if it has landed and skips it if it has not.

**Line numbers.** They cite main at the stage 3 spec's first baseline (`044b5eb2e`) and drift under stage 2's units. Each unit's dispatching orchestrator re-greps at the merge of the unit it waits on and states the drift.

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

## 0. Ordering: seven PRs, each green, none landing half a representation

| PR | Unit (`work/intent/`) | Lands | Representation step it completes | Goldens |
|---|---|---|---|---|
| A | `poses-are-variables` | pose definitions read off geometry, by coordinates in a constructed frame, by construction (`Through`, `Meet`, `Flip`, the standoff), by projection, or as an output; one `Subgroup` on the pose value; the revolve's axis as a 2-D line on the node; `Split`'s tool as a `Plane` read | **a pose is a defined variable of its kind** | re-blessed: `AxisInPlane` and `FaceFrame` leave the node table. Geometry bit-equal |
| B | `a-placement-is-the-bundle-of-mates` | `Place { shapes, constraints }` owns today's mates; the world node; an instance is a placement; spaces are kinds from the recipe. Gauges, offsets, `PlaceInWorld`, the spanning tree, roots and `MateRole` retire, and a mate beyond its bundle's pin refuses | **a placement owns its mates; the world is a node** | re-blessed. One-time migration check: poses and product bit-equal |
| C | `a-mate-relates-two-poses` | a mate is `{ on, to }`, two poses of one kind, holding no number; values set the freedoms mates leave. `MatePrimitive`, `MateFrame`, `FrameBase`, `AxisSense`, the rider and `PlanarRest.offset` retire | **a mate equates two poses; a value sets a freedom** | re-blessed: mate wire. Solved poses equal to within the migration's stated rounding |
| D | `transform-retires-into-a-placement` | constructions read no frame: a profile is 2-D, a tube is built in its own coordinates, an extrude has no side. `Datum`, `Transform` and `PlacedFrom` retire; each becomes a placement | **nothing moves a body, and frames enter only at placement** | re-blessed. Each moved body moves by one composition, which the migration reports |
| E | `an-operation-computes-in-a-frame-of-its-reads` | the per-operation computing frame (D9); minted reference directions from the inputs; export composes the world's map and nothing else reads it | **construction never reads the world** | **product body digests move** (computing-frame coordinates); STEP bytes move by rounding |
| F | `a-mate-on-a-pinned-copy-refuses` | a constraint any of whose equations the bundle already fixes refuses, pinned or not, by subgroup algebra; `Contradictory`, `member_of`, `trivial_member` and the measured case splits retire | **a mate places and never checks** | re-blessed: corpus bundles that overconstrain are restated or dropped, each named. Poses unmoved |
| G | `patterns-are-index-variables` | `index(N)`, families, `Member` names; a pattern is a placement whose reads reach an index; `Mirror { body, plane }`; `PatternKind`, `Node::Pattern`, `PlacedUnion` and `Instance(i)` retire | **repetition is an index; no preset is stored** | re-blessed: names move (`Instance` → `Member`). Copies' geometry bit-equal |

The dependency graph:

- **A** waits on stage 2 E (a pose read off a face reads a `Face` variable).
- **B** waits on stage 2 F (a mate's sides read `Face` variables) and on stage 4's unit that turns A5's hard error into a finding (§12, boundary).
- **C** waits on A and B.
- **D** waits on C.
- **E** waits on D.
- **F** waits on C and on stage 4 C (stage 4 D has merged).
- **G** waits on D and on FORK-DM4's ruling (`a-union-member-is-keyed-by-its-read`).

Why this order:

- **A and B are independent.** B moves ownership of today's mates and nothing in it reads a pose variable; A defines poses and changes no mate.
- **B before C.** A value is a constraint of a placement (FORK-S3M), so C cannot move today's in-plane offsets and rider into values before a `Place` exists to own them, and a mate may not hold a number. B therefore moves today's mate payload under `Place` unchanged, and C re-types it.
  - The cost is that the mate wire breaks twice (B moves it, C retypes it), and the old spec rejected this order for that reason. The alternatives are worse: one PR of about 350 files with no point at which poses can be compared, or numbers on a mate between the two units, which D10 forbids.
  - B's bit-equality check (every MSOLVE fixture's `SolvedPoses` unmoved) is cheapest while the mate arithmetic is still today's.
- **C before D.** D re-spells every absolute datum and every `Transform` as a placement whose mates read poses off geometry, plus values. Before C a mate cannot read a pose or carry a value.
- **D before E.** E's default is "a construction computes in its own coordinates". Before D a construction computes in the world coordinates its absolute datum names.
- **F after C.** F's rule reads the kinds' families off the table that C re-keys by pose kind. It also waits on stage 4 C (stage 4 D has merged), whose rungs prove the contacts F's refusals leave behind, so they are not reported as unproven noise.
- **G after D.** A pattern places copies of one body by values over an index. That needs B's placement, C's values and D's "no construction reads a frame". G reads a union of a family, which is where FORK-DM4 sits.

Each intermediate state is a whole representation:

- After A, a pose is a defined variable, and every pose a reader reads is one. Absolute datums are still the operations that define the poses constructions read; D retires them with those readers.
- After B, a placement owns its mates and the world is a node. A mate is still today's payload, an alignment with offset frames, and may name the world as a base.
- After C, a mate equates two poses and values set freedoms. Constructions still read absolute datums, and `Transform` still stands.
- After D, frames enter only at placement, and only a placement defines a copy.
- After E, every operation computes in a frame of its reads.
- After F, a constraint any of whose equations is already fixed refuses, pinned or not.
- After G, the document stores no pattern preset.

**Rejected:**

- **Mates as poses (C) inside the placement unit (B).** About 350 files in one PR: every mate site, every gauge site and the refactor doors, with no point at which poses can be compared.
- **The old order, mates as poses before placements.** The in-plane numbers would sit on the mate until a placement exists, a second spelling D10 forbids.
- **D before C.** A construction placed against a face needs a mate reading the face's plane and values for the freedoms it leaves.
- **F inside B.** B's migration check is "poses and product bit-equal"; F restates bundles. Mixing them hides which change moved a body.

## 1. Final shapes (after G)

**Pose variables** (`var.rs`). A pose is only defined (FORK-S3P), by `VarDef::Pose(PoseDef)`:

- **Read off geometry.** `Plane { face: Face }` (a face reads as a plane: its carrier's plane with its outward normal); `Axis { of: Face | Edge }` (a carrier's axis or an edge's line); `Point { of: Face | Edge }` (a carrier's centre). No definition reads a carrier's reference direction, so DM1's `FaceFrame` and its spin have no successor.
- **`InFrame { frame, coords }`**: a pose written by coordinates over scalar variables in a frame the definition reads. That frame is itself a definition (a construction over poses read off geometry), so `InFrame` never starts from nothing. A tolerance on a datum's position is a tolerance on these scalars (VR8 unchanged).
- **The named constructions** (FORK-1b), each refusing its degenerate case typed at evaluation, never an automatic join: `Through { axis, point } → Frame` refuses a point on the axis; `Meet { a: Plane, b: Plane } → Axis` refuses parallel planes; the rest wait for a reader (Q1).
- **`Flip { pose }`**: the opposite sense of a `Direction`, `Axis`, `Plane` or `Frame`. It is an involution the door normalises to one side, and it has no `Point` arm (FORK-S3M).
- **`Standoff { plane, by: Length }`**: a plane moved along its own normal, the one number a kind's own equation fixes, as a construction on a mate's target (FORK-S3M).
- **`Project { of, to }`**: a `Frame` to its xy-plane, z-axis or origin; a `Plane` or `Axis` to its `Direction`. A finer value is read through its projection, and a slot holds its own kind (D10).
- **`Output { node, port }`**: an operation's pose output, such as a revolve's `axis`.
- **`Carried { copy, pose }`**: a pose of a copied shape as that copy carries it, keyed by the placement (FORK-S3M). It is how a mate reads a pose of another copy.

A pose variable has no unit and is never an analysis axis itself; its scalars are. It reads as its definition ("the plane of face Top of Extrude "base"").

**One `Subgroup`** (FORK-1b; Ev on #4222). A pose value, `PoseValue<T>` (renamed from `topo::query::DatumValue` and moved into `editor-core`'s pose module), implements `PoseSymmetry` with a `Subgroup<T>`. `VarKind::symmetry` names the family, and the door asserts the two agree. `mate/coset.rs` reads the sides' symmetries there, and `side_symmetry` (`mate/solve.rs`) goes. `Point` and `Direction` have arms, because D10 names a lone point mate.

**Constructions read no pose** (FORK-S3P). A root construction (one reading no body) is built in coordinates of its own:

- a `Profile` is 2-D: its numbers are read only against each other, and it reads no plane;
- `Extrude` reads a profile and a depth, and a slanted extrude a direction in the profile's own axes held to one side of its plane (scalar slots on the node, normalised at evaluation). `ExtrudeSide` goes: which side of a face the material lies on is the placement's mates' to say;
- `Revolve` reads a profile, a 2-D axis line in the profile's axes (`axis_origin: [S; 2]`, `axis_direction: [S; 2]`) and an angle, and defines `axis: Axis`, the line's lift;
- `Tube` and `HollowTube` are built about their own axis and read no frame.

An operation that reads a body may read a pose whose reads reach that body's root alone: `Split`'s tool is a `Plane` read off its operand's geometry (or constructed from it), and so is `Mirror`'s plane.

**Mates** (C). A mate is `{ on: S, to: S }`, two pose variables of one kind, and the kind is the primitive:

- `on` is read off geometry of the copied shapes; `to` is read off geometry of the space the copy joins, or reads the world (Q7). Neither is a frame standing for a part's coordinates: there is no `FrameBase::Part` and no part frame to read.
- `Frame`–`Frame` coincides, `Axis`–`Axis` is coaxial, `Plane`–`Plane` rests, `Point`–`Point` is a ball, and `Direction`–`Direction` is parallel. Each pins the relative pose to a coset of the kind's `Subgroup` (A11 (1)).
- A mate holds no number. Its sense is `Flip` on one side, so a `Frame` "opposed" is a `Flip`ped plane mate plus values, and `opposed()`'s hidden half-turn about `x` retires.
- `class: ContactClass` stays until stage 4's `mates-declare-no-contact` retires it.

**Values** (C). A value sets one freedom the bundle's mates leave, a slide or a spin, to a `Length` or `Angle` variable. It is charted on the two bodies' own coordinates as the placement carries them:

- a slide is the copy's origin measured from the target's along the freedom;
- a spin is the angle between their reference directions about it.

Each is a function of the relative pose alone, so no order of the values is chosen. Zero is always valid: it says the two bodies' own coordinates agree as far as the mates allow. The façade writes a free variable holding zero for each freedom a gesture leaves unnamed. A rotation left by a lone point mate has no chart, and a value on it refuses (`NoChart`); a direction mate lowers it first. A placement is the only reader of a body's own coordinates, and reads them only through the freedoms its mates leave.

**Placements** (B, C; FORK-S3M).

- **The node.** `Node::Place { shapes: Vec<S>, constraints: Vec<Constraint> }` is an operation. It reads a list of shapes of one space, as a union reads its operands (a list of reads, not a list literal), and defines one copy of each under the one rigid motion its constraints pin.
- **Constraints.** A constraint is a mate or a value, addressed by (placement, an id minted with it), never by its position in the list and never a node. Which copy is defined is which placement reads it, so there is no tree, root or declaring role.
- **Pinned.** A copy is pinned when its mates and values together leave nothing free. A body's symmetry pins nothing: a value it makes unobservable is reported, never refused.
- **Overconstraint** (F; FORK-S3O). A constraint any of whose equations the others already fix refuses `Overconstrained { placement, constraint, held }`, pinned or not, decided by subgroup algebra without measuring: it is admitted only when codim(held) + codim(added) = codim(result). A mate that would take a freedom a value sets refuses the same. Where two copies meet beyond what their mates fix is a contact, recorded at stage 4's door and linted, and its recourse is an assertion.
- **A placed copy's frame is not a variable.** It is the construction its bundle states, which the coincidence door replays at `Sym` (stage 4 H).
- **An instance of a part is a placement.** `InstantiatePart { doc_ref, interface, constraints }` defines one output per world placement of the part, keyed by that placement's node id, and a family under the part's own index where that placement is one; all of them are of its targets' space. It defines no `frame` port. A re-pin that adds a world copy mints a port no placement reads yet, and the maintenance report names it. The GUI's "place part" writes `Place { shapes: [every output], … }`.

**Spaces are kinds** (FORK-S3P). A **root** is a construction that reads no body, the world, or a copy whose bundle pins less than its body needs (a loose copy). A body's kind is its root; a placement's copy takes its targets' root once its bundle pins it; a copy pinned, transitively, against the world is of the product's kind. A loose copy is its own kind with everything pinned to it, solved among itself, read at a pose by nothing outside it, and drawn from display state no logic reads. Only a mate reads across kinds: any other read across two spaces refuses at the door (`SpaceMismatch`) and is an evaluator assert anywhere else. Kinds are decided from the recipe, never from a value.

**The world** (B). One undeletable node that copies may be related to like a part. It defines no pose variable, so only a placement's mates (Q7) and export read it, and no construction does.

**The product** is every copy whose space reaches the world, in placement order. Building, combining or placing shapes adds nothing to it. Export writes it and refuses an empty one, naming the bodies no placement relates to the world.

**The computing frame** (E; FORK-S3P). An operation computes in a frame that is a function of what it reads and of nothing else, chosen so its arithmetic is well conditioned near the geometry it builds, and never in the world's. The frame is keyed with its inputs, so an edit that leaves an operation's reads alone moves none of its bits (D9). The frame is no part of the operation's meaning: the body up to that rigid map, its names and every verdict outside the sliver band are the same in any frame, and a minted reference direction is a function of the inputs, not of the axes. Where conditioning does not decide, a construction computes in its own coordinates and an operation over copies in its first operand's, as the author lists it. The at-rest census is order-free: each pair's verdict is the same in either member's frame, or the sliver band refuses.

**Repetition** (G; FORK-PAT). `k = index(N)` defines a `Count` over `0..N`. A definition or operation whose reads reach `k` is evaluated once per value, and its outputs are families keyed by index tuples. A pattern is a placement whose reads reach an index, the index entering as a value (`spin = scalar(k)·turn/N`) or through a mate's target (`bolt.axis ≡ holes[k].axis`). No pose is constructed from an index. The presets are the façade's, and the GUI recognises one for display only.

## 2. PR A — `poses-are-variables` (cost H; ~150 files)

**Kernel**

1. `VarDef::Pose(PoseDef)` with the arms of §1 except `Carried`, which is B's.
   - Each arm states its result kind, and the door checks it as `SlotVarKind` checks a slot. `InFrame`'s coordinates read scalars of the right dimension.
   - `VarDef::kind()` reads the arm.
   - A pose definition may not reach its own variable (VR3's acyclicity walk).
2. **Evaluation.** A pose's value can depend on built geometry (a face's plane reads a `Face`), so a pose definition is bound when its reads are, at the lane scalar, by one evaluator `eval_pose` that replaces `wire_datum`. Its helpers are today's `frame_axes`, `frame_value` and `frame_plane_lane`. The schedule treats a pose definition as a node of the dependency graph over `Doc::upstream`: stage 2 D's machinery for observed definitions, generalised to "a definition bound mid-evaluation".
3. **`AxisInPlane` retires into `Revolve`** (FORK-1b). `Revolve` gains `axis_origin: [S; 2]` and `axis_direction: [S; 2]` in its profile's axes, and its `axis: Axis` port is the lift. `wire_revolve` reads the slots. The F15 doc on `AxisInPlane` moves to `Revolve`'s fields, still saying why: four numbers in the plane cannot leave it.
4. **`FaceFrame` retires.** A face reads as a plane (FORK-S3P round 10), so its readers read `Plane { face }`. A reader that needs a frame on a face reads a constructed one (`Through`). DM1's sketch-on-face is D's: a construction placed against the face's plane, its spin a value.
5. **`Split`'s tool** reads a `Plane` pose whose reads reach its operand's root alone. A tool reading a pose of another space refuses `SpaceMismatch` from B on. Until D, an absolute `Datum::Plane` output is also a `Plane` pose (stage 2 A's port).
6. **One `Subgroup`.** `PoseSymmetry` moves from `DatumValue` (`mate/coset.rs`) to `PoseValue`, with `Point` (spherical) and `Direction` arms. `VarKind::symmetry` has no callers in src today, and the two tables are kept in step by a test (`intent_s2_a_outputs.rs`); A folds them into one.

**Migration** (a one-time regenerate):

- `AxisInPlane` becomes `Revolve`'s slots;
- `FaceFrame` with zero spin, read only as a plane, becomes `Plane { face }`. One read as a frame (a sketch on a face, a frame mate) stays a `Datum::FaceFrame` until D or C restates its reader. The migration's report names each.

Every body digest is bit-equal.

**Sites.** `axis_in_plane(` 45 in editor-core tests; `Datum::FaceFrame` constructions in src, tests and viewer (re-grep at stage 2 E's merge); `wire_revolve`; `wire_split` and `verbs/split.rs` `plane_of`.

**Goldens.** The `AxisInPlane` and `FaceFrame` rows of `tests/golden/slot_tables.txt` become pose-definition and `Revolve` rows; ids move, re-blessed with `M4_PR6_BLESS_GOLDEN=1`. Every f64 geometry digest is bit-equal.

## 3. PR B — `a-placement-is-the-bundle-of-mates` (cost H; ~260 files, 8–11k lines)

The largest unit. Its size is in the gauge sites, which are compile-driven, and in the refactor doors, which are not.

**Kernel**

- **`Node::Place { shapes, constraints }`** per §1. In B a constraint is today's mate payload (`Alignment`: two `MateFrame`s, a `MatePrimitive`, an `AxisSense`) addressed by its minted id; C re-types it.
  - A constraint reads the copy's side through the shapes it copies and, on the other, a pose outside the copy: another placement's copy, a body of the space the copy joins, or the world.
  - The door refuses a constraint with no side on the copy (`MateFault::NotOnTheCopy`) and one that reads its own copy (acyclicity over `Doc::upstream`).
  - **`FrameBase::World`** joins `FrameBase` for B's migration, so a gauge root's world pose is one mate to the world. C deletes `FrameBase`.
- **`Carried { copy, pose }`** (§1) joins the pose arms.
- **The world.** `Node::World`, minted with the document, undeletable (`EditError::WorldIsUndeletable`), defining no variable, and read only by a placement's constraints and by export.
- **The solve** (`mate/solve.rs`) is per placement. Each bundle folds through `fold_pair`'s coset intersection, and a copy's pose is its target composed with the folded representative.
  - **Retired:** `solve_group` and its BFS; `groups`, `root_and_cause`, `root_of` and `places`; `group_frame`, `gauge_chain` and `gauge_frame`; `MateRole`, its Python door and 93 test mentions; `Unplaced::{NoOffset, DeadGauge}`.
  - `SolvedPoses` keeps `placement` and the A2a pairing door, keyed by placement.
- **A mate beyond the pin refuses** (#4326: there is no interim that verifies a mate). A constraint added after the bundle's fold is `Trivial` refuses `Overconstrained` at the insert door and in the solve. F extends the rule to every constraint, pinned or not.
- **A loose copy.** A bundle that pins less than its body needs makes a loose copy, a root of its own kind (§1), not a refusal.
- **Gauges and offsets retire:**
  - `Node::Gauge` and `InstantiatePart.gauge`/`offset` (the serde `present` door in `persist/wire.rs`);
  - `DocEdit::SetOffset`, `SetGauge`, `Promote` and `Fold`;
  - `regauge_then_mate`, `clear_joined_offsets`, `mate_that_would_start_placing` and `check_gauge_ref`;
  - the thirteen gauge `EditError`s and `Maintenance::OffsetCleared`;
  - `check_offsets` and `MateFault::OffsetDisagrees`/`OffsetUnchecked`/`OffsetCheck`/`OFFSET_RECOURSE`;
  - `GaugeRefFault` (`doc.rs`), the load walk's gauge check (`persist/check.rs`) and `SnapshotError::NotAGauge`/`GaugeCycle`.
- **`PlaceInWorld` retires**, if stage 2 C landed it, into `Place` with one mate to the world. Stage 2 C's product gather reads the placements whose space reaches the world.
- **Spaces are kinds** (§1). The kind walk replaces `Product::spaces`/`own_spaces`, `gate_spaces`, `Evaluation::across_spaces`/`unplaced`/`unplaced_below` and `CarriedUnplaced`, each keeping its meaning, read from the recipe instead of gauges. `NodeErrorKind::Unplaced` becomes `SpaceMismatch`.
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
- **The check** (test 7): every corpus document's `SolvedPoses::placement` for every copy, and its product digests, are bit-equal to pre-B.

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
- **Values** (§1): `Constraint::Value { freedom, by: S }`. The door refuses a value on a freedom the mates do not leave (`NotAFreedom`), a value on a lone point mate's rotation (`NoChart`), and a mate that would take a freedom a value sets (`Overconstrained`). The chart of each residual (`Planar`: two slides and a spin; `Cylindrical`: a slide and a spin; `Prismatic`: a slide; `Revolute`: a spin) lives in the shared `Subgroup`, order-free, a function of the relative pose alone.
- **Retired** (`mate.rs`, `mate/solve.rs`):
  - `Alignment`, `MateFrame`, `FrameBase` (with B's `World` arm), `MatePrimitive` and `AxisSense`;
  - the clocking rider (`Alignment::clocking`, the `FrameCoincidence` rider arm of `mate_coset`, `Refuted::ClockingRedundant`);
  - `PlanarRest.offset`, `MateFrame::authored` and `table_gap`;
  - the `Offset { frame, (dx, dy, θ) }` mate target, a second way to write three values (FORK-S3M);
  - `SlotId::MateFrameStep` and `VectorSlot::MateFrame*`.
- `mate_coset` reads the two sides' `PoseValue`s and their common `Subgroup`, the table's row by kind. The face-base read (`resolve_side`, `topo::readback::face_pose`) moves into `eval_pose`, so it runs once per definition, not once per mate.
- **The world as a target** (Q7): a mate's `to` reads the world through a projection of its frame.
- **Analysis.** A value is a slot variable, seeded and boxed as any slot is, through `Doc::reads`. The solve's lanes (MSOLVE-14) are unchanged.

**Migration.** Each mate is restated over geometry plus values:

- A `FrameBase::Face` side becomes the face's `Plane` (and, where the primitive was coaxial, the carrier's `Axis`). Its offset's in-plane translation and roll become values, computed once from today's solved relative pose.
- A `PlanarRest.offset` of `h` becomes `Standoff { plane, by: h }` on the target, `h` a new anonymous `Length`.
- A rider `θ` becomes a spin value.
- A clocked coaxial (today's `Coaxial` plus rider, residual `Prismatic`) becomes an `Axis`–`Axis` mate plus a slide and a spin value.
- An `AxisSense::Opposed` side becomes `Flip` on the copy's side.
- A side written against a part frame (`FrameBase::Part`), an authored vector or B's `FrameBase::World` is absolute coordinates. The migration restates it over geometry plus values where the copy has a planar face or a carrier axis that pins the same coset; otherwise it drops the mate and names it in its report. A world mate restated this way reads the world's projections (Q7).
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

**Kernel**

- **Constructions read no frame** (§1): `ProfileProgram.plane` goes and a profile is 2-D; `Tube` and `HollowTube` lose `frame`; `Extrude` loses `side` (and gains the slant's 2-D direction slots only if a reader asks, Q1); `tube_args` and the `frame_plane_lane` call sites in `eval/wire.rs` follow.
- **`Datum` retires** (`node.rs`, `Node::Datum` and its arms; `Node::outputs()`'s port arms). The six datum constructors in `pncad-py` become pose-definition builders where a pose is read off geometry, and placement builders where a datum sat a construction in space.
- **`Node::Transform` and `PortKind::PlacedFrom` retire** (`wire_transform`, `placeable_operand`). A rigid motion of a body is a `Place`. "You can't Transform an already placed part" (Ev) is unrepresentable: a copy is defined by its one placement and never moved after.
- **Names.** A copy's names pass through unchanged (`names::role`), as `Transform`'s did, so downstream selectors spell what they already spell. DM3's sentence is re-worded to say so.
- **The member walk** (`mate/member.rs` `transform_map`, `Placing::Transform`) has nothing left to walk through.
- **`placement::Frame`** stays as the evaluated value type. `Step::Literal` retires: no document holds a literal frame, so A6's admission (`Frame::admission_fault`) runs on evaluated pose values in `eval_pose`. A mirror is G's construction, so no placement needs an improper frame.
- **Sketch on a face.** The façade's gesture writes profile, construction, placement (a `Plane` mate to the face's plane, the side a `Flip`, its slides and spin values) and combine, as one gesture.

**Migration.**

- **A construction on an absolute datum** becomes the same construction in its own coordinates, placed against what its datum was related to: one mate from the datum's frame, with values. Its bits move by one composition, which the migration reports per body.
- **A `Transform`** becomes a `Place` of its input with one mate to the target plus values computed from its chain.
- **An absolute pose read by a non-root operation** (a `Split` tool, an `Explicit` frame) becomes a `Standoff` of a parallel planar face of the operand where one exists, else `InFrame` of a frame constructed from the operand's carriers; failing both, the migration names the node and the document refuses to regenerate (Q8).
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

## 6. PR E — `an-operation-computes-in-a-frame-of-its-reads` (cost M; ~70 files, 2–3k lines)

- **The frame.** Each operation computes in a frame that is a function of its reads, keyed with them (§1). The default is the author's: a construction in its own coordinates, an operation over copies in its first listed operand's.
- **A value-informed refinement** is allowed and measured: re-centre on the operands' bounds, snapped to a power-of-two grid at their scale, the author's order breaking ties. It is measured on the far-from-origin rows, `a-far-meeting-point-fails-membership-by-its-own-rounding` first.
- **Minted reference directions** follow the inputs, not the axes: `Vec3::orthonormal_basis` decides by a branch on the coordinate axes, so a minted carrier's u-reference, and with it a seam and its names, depends on the computing frame. E re-derives each from its inputs: a sweep cap from its path frame (`crates/sweep/src/swept.rs`), a Newell plane from its loop (`crates/geom-brep/src/newell.rs`), a split's section from the cutting plane (`section_loops.rs`, `chord_u_ref`). This closes `a-minted-reference-direction-follows-the-computing-axes`.
- **The world's map.** `SolvedPoses::world_of(copy) -> Frame` is the one door that composes a copy with the world. Export (`pncad/src/export.rs`) reads it, and **nothing else may**, the viewer included. A lint test greps `editor-core/src` for callers outside `export` and `persist`, as `scripts/gates/test-features-dev-only.sh` does for its feature.
- **The census** is order-free (§1).
- **What moves.** Product body digests on every corpus assembly move into computing-frame coordinates, STEP bytes move by rounding (one more composition), and a measured value across copies may move by rounding. Re-baseline and say what moved: test 13 checks each moved digest against the old one mapped through the world's map.

## 7. PR F — `a-mate-on-a-pinned-copy-refuses` (cost M; ~70 files, 2–3k lines)

**Kernel**

- **The door.** `admit_mate` (`mate/solve.rs`) folds the bundle's held subgroup from its definitions (Q4) and refuses `EditError::Overconstrained { placement, constraint, held }` unless codim(held) + codim(added) = codim(result), pinned or not. The test reads the families of `coset::table`'s rows, and nothing is measured. A copy pinned already is the case `(Trivial, _)`, with no predicate.
- **The solve** refuses the same in `fold_pair` for a state the door did not see (a rebind, a load, a definition edit that changed a kind), reporting `MateFault::Overconstrained` and keeping A11 (1)'s door/solve split.
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

## 8. PR G — `patterns-are-index-variables` (cost H; ~150 files)

The unit's design is FORK-PAT's (row 99), and `work/intent/patterns-are-index-variables.md` carries it in full: what is built, the unchecked requirement (ring closure is structural only if the symbolic tier reduces a spin modulo a turn), the sites and the migration. In outline:

- **The index.** `k = index(N)` and `index(N) within j`; families keyed by index tuples; the kind rule; `xs[i, j]` reads (DM3); exact `mod` on `Count`; no list literal.
- **Names.** `RoleSeg::Instance { i, of }` becomes `Member { (i, j, …), of }`, keyed by the index variables' ids and the integers.
- **A pattern is a placement** whose values are expressions in the index: `spin = scalar(k)·turn/N` under an `Axis` mate is a ring, `slide = scalar(k)·pitch` a row. Copies of one body are built once and mapped.
- **Mirror.** `Mirror { body, plane }`, a construction defining a new `Body` in its source's root, the plane read off the source alone (MIRROR-DESIGN P1–P4, P6).
- **`PlacedUnion` retires** into a placement of the copies and a `Node::Union` reading the family (REFERENCES DM4 as it stands). Its `Separation` certificate becomes the union's fast path when its bodies are rigid images of one body. This closes `placed-union-places-and-fuses-in-one-node`.
- **Explicit frames.** `PatternKind::Explicit(frames)` becomes one placement per frame, a `Plane` mate against the face with the frame's in-plane offsets and spin as values, computed from the stored frame once at migration. This closes `explicit-placement-frames-hold-floats` (`die_tool.pncad` holds six `Explicit` frames, not the row's "twenty-one").
- **Retired:** `PatternKind`, `Node::Pattern`, `Node::PlacedUnion`, `PartSelect::Instance`, the placement-rule slot table, `placement_rule_fault`, `CountMismatch`, P5's placement-major layout, and the boxed `Pattern` angle half of `topo/a-boxed-rotation-refuses-not-rigid-at-every-placer`.
- **Façade.** `linear_pattern`, `circular_pattern`, `grid`, `bolt_circle` and `mirror` write the program; a display-only recogniser reads it back for the GUI's forms.

**Waits on FORK-DM4.** G's union reads a family as one member (DM4 as ratified on #4341). FORK-DM4 is deciding how a union keys its members' names, whether `union` and `intersect` are n-ary, and whether `Boolean` leaves the document. G names union members by those keys, so it is dispatched after that ruling and follows it. Nothing in A–F reads a union.

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
- **D10 itself**: no sentence changes. §13 lists one parenthetical that disagrees with its own paragraph.

| | A | B | C | D | E | F | G |
|---|---|---|---|---|---|---|---|
| Node ids, pinned hex, memo fixtures | `AxisInPlane`, `FaceFrame` | gauges, mates under placements | mates | **all** (datums, transforms leave) | — | — | patterns |
| Var table, mint log | grows (pose definitions) | grows (`Carried`) | grows (values, standoffs) | grows | — | — | grows (indices) |
| Wire | `Revolve` slots; pose definitions | `Gauge`, `gauge`/`offset`, `PlaceInWorld` gone; `Place`, `World` | `Mate` | `Datum`, `Transform`, profile plane, tube frame, extrude side gone | — | mates dropped | `Pattern`, `PlacedUnion` gone |
| Solved poses | — | **bit-equal** (test 7) | within stated rounding (test 8) | — | relative to the computing frame | **bit-equal** | — |
| Product body digests | **bit-equal** | **bit-equal** | within stated rounding | **move by one composition** (reported) | **move** (computing frame) | **bit-equal** | **bit-equal** for copies |
| STEP export bytes | equal | equal | rounding | rounding | **rounding** | equal | equal |
| Names | — | — | — | — | — | — | `Instance` → `Member` |
| Python `.pyi` | pose builders | `place`, `world`; gauge doors go | `Mate(on, to)`, values | datum builders, `transform` go | — | — | presets become façade functions |

**f64 geometry moves only where a unit says so.** In A, B, F and G a body digest, a measured bit or a solved pose that changes is a bug. C, D and E state what moves and by which map, and their tests check each moved value against that map.

## 10. Test plan (each row names the runtime value that breaks it)

1. **(A) A face reads as a plane.** `Plane { face }` on a cylinder's flat cap equals the cap's carrier plane with its outward normal; on the curved face it refuses DM1b's typed error. *Breaks if* a reader reads the carrier's u-reference (a roll appears in a value that should have none).
2. **(A) A slot holds its kind.** `Split { tool: <an Axis variable> }` refuses `SlotVarKind { expected: Plane }`; `Project { of: frame, to: Plane }` is accepted. *Breaks if* a finer value is admitted where a coarser one is asked for without its projection.
3. **(A) A combination refuses its degenerate case.** `Through { axis, point }` with the point on the axis refuses `PoseDegenerate { construction: Through }` at evaluation, at the scalar that decides it. *Breaks if* it picks a frame anyway (an arbitrary roll).
4. **(A) The revolve's axis cannot leave its plane.** `Revolve { axis_origin: [0, 0], axis_direction: [0, 1] }`: the `axis: Axis` output lies in the profile's plane for every value in a box run (its direction's margin against the plane's normal is identically zero in `Sym`). *Breaks if* the axis is lifted through 3-D coordinates.
5. **(B) Two placements, two copies.** Two `Place`s of one body, each against the world: two copies, distinct output ids, each name resolving once per copy. *Breaks if* the bundle is keyed by body, not by placement.
6. **(B) The product is the world's kind.** A copy pinned against a copy pinned against the world is in the product. Two copies pinned only against each other are one loose space: not in the product, `EmptyProduct` names them, and the census between them runs. A copy whose bundle pins less than its body needs is a loose copy, not a refusal. *Breaks if* membership is "has a bundle" rather than "reaches the world", or an under-pinned copy refuses.
7. **(B) The one-time migration.** For every corpus `.pncad` and every MSOLVE fixture, `SolvedPoses::placement` for every copy and `product_recorded`'s digests in order are bit-equal to pre-B. *Breaks if* a tree mate goes into the parent's bundle instead of the child's, or a gauge chain is composed outer-first.
8. **(C) The mate respelling.** Every MSOLVE fixture's solved poses agree with pre-C within the rounding of one value re-composition, and the report names every dropped mate. *Breaks if* a migrated standoff lands in the other side's frame (a non-zero standoff flips sign).
9. **(C) Values.** A `Plane` mate leaves two slides and a spin. Setting each to zero places the copy with its own coordinates agreeing with the target's in the plane, whatever order the values are listed in. A spin value on a lone point mate refuses `NoChart`. A second mate taking a freedom a value sets refuses `Overconstrained`. *Breaks if* a slide is charted along the other slide's moved axis (order-dependent).
10. **(D) A construction is placed, not framed.** The corpus's sketch-on-face bodies: each copy's digest equals the pre-D digest within one composition, and editing a value moves the copy, not the construction (the construction's digest is unmoved). *Breaks if* a construction reads the frame it is placed by.
11. **(D) Nothing moves a body.** No variant reads a body and defines it moved except `Place`; a second placement of a copy is a second copy. *Breaks if* `Transform` survives under another name (a census of `Node` variants reading a `Body` and a rigid motion).
12. **(B, E) The world is read by placements and export alone.** `Delete(world)` refuses `WorldIsUndeletable`. A construction reading the world refuses at the door. The grep gate admits no `world_of` caller outside `export` and `persist`. *Breaks if* the world is admitted as an ordinary pose.
13. **(E) The computing frame is no part of meaning.** Every corpus operation computed in its frame and in a second frame (the first rotated by an irrational angle and translated) gives the same body up to that map, the same names, and the same verdicts outside the sliver band. Moving a document's only world mate moves no body digest and no measured bit, and moves the STEP bytes. *Breaks if* a minted u-reference follows the axes (a seam moves, a name changes) or a path composes the world's map.
14. **(F) A constraint already fixed refuses, measure-free.** A copy pinned by a mate and values: inserting any further mate refuses `Overconstrained { held: Trivial }` with an empty predicate log. A `Plane` mate, then a second `Plane` mate on any other face, refuses whatever the faces' angle. *Breaks if* the door folds the new mate before checking codimension, or admits a redundant mate whose offset agrees (the fallback Ev rejected).
15. **(F) A contact beyond the mates.** Two copies resting on a second pair of faces their mates do not fix: the census records the contact and the lint reports it; an assertion on its `Gap` quiets it. *Breaks if* the contact is minted as a declaration.
16. **(G) A ring is an index.** `circular_pattern(body, axis, 12)` writes an `Axis` mate and `spin = scalar(k)·turn/12`, no preset; the copies are bit-equal to pre-G; `Member { (k), of }` names resolve per member. *Breaks if* a preset tag is stored or the copies are built twelve times.
17. **(A–G) Python.** `doc.plane(face)` returns a `Var` of kind `Plane`; `doc.place([body], [...])` builds a copy. The census and `.pyi` agree, and every gauge, datum, transform and pattern door is absent from the census.

Loud census rows: `pncad-py` `tags.rs` / `surface_census` / `prose_census`, `display_contract`, the persist `Walk` roster, `tests/golden/slot_tables.txt`, `f6_variants!` and `eval/class.rs`'s class table (`MateOffsetDisagrees`/`MateOffsetUnchecked` go; `Overconstrained` comes).

## 11. Risks

- **B's size.** About 1,600 gauge lines in src and tests plus the refactor doors. The gauge sites are compile-driven; the refactor rewrite is not, and it is where review time goes. B can split only along a representation boundary, and none exists inside it: gauges then the tree would leave two ways to place.
- **D's reach.** Every profile, tube and datum site moves, and every construction off the origin moves by one composition. The migration report is the reviewer's check, and test 10 its gate. D is the unit most likely to need a split along the readers (profiles, then tubes and splits); each half must leave no construction reading a frame of its kind.
- **The migrations that cannot restate.** C and D restate absolute coordinates over geometry where a face or carrier pins the same coset, and otherwise drop and name a mate (C) or refuse to regenerate (D, Q11). Measure the corpus's count before C dispatches.
- **Pose definitions bound mid-evaluation** (A). A face's plane exists only after the body it reads. If stage 2 D's machinery is narrower than this needs, A widens it, and the content key hashes a pose definition's upstream keys, not its id.
- **Constraints reading copies** (B). `Carried` is new: a pose a copy carries is a function of that copy's solved pose, so a bundle's fold reads poses that depend on other bundles. The order is the bundles' read order over `Doc::upstream`, acyclic by the door: A placed against B and B against A is a read cycle and refuses, which replaces A11 (4)'s "no cycle is ever solved".
- **The contacts declaring mates declared** (B). B drops declaring mates, so their contacts are unattributed. Before stage 4 retires A5's hard error, that refuses the document's product; §12's boundary orders B after that retirement.
- **E moves goldens on purpose.** Every assembly digest, the STEP bytes and the gallery frames move. The check is test 13's map, not bit equality.
- **Migration reproducing the tree** (B). Today's tree is "the first member pair per instance pair in `Member` key order". The migration orients mates by that exact rule, or test 7 fails where two orders differ.
- **Load.** Seven units (H, H, H, H, M, M, H: 30 points by the work README's weights) are filed `parked` behind stage 2 and so count nothing until it closes. At stage 2's close the orchestrator splits stage 3 into its own program or confirms it fits.

## 12. Open questions

### The forks are settled

Each of the six forks this spec opened was weighed by a designer pair and ruled by Ev:

- **FORK-S3-1** (what a free pose is) and **FORK-S3-6** (what a space's member is): FORK-S3P, fork log row 95, PR 4324. No pose is free; there is no seed and no member order, and an operation computes in a frame of its reads.
- **FORK-S3-4** (what overconstraint means): FORK-S3O, row 96, PR 4325. Pinned or not, by codimension, measuring nothing.
- **FORK-S3-2** (what a placement is), **FORK-S3-3** (what a mate reads) and **FORK-S3-5** (what placing a multi-body instance copies): FORK-S3M, row 97, PR 4326. `Place` owns mates and values, reads a list of shapes, and an instance keeps one output per world placement.

FORK-PAT (row 99, PR 4341) added unit G.

### Questions with a recommendation (not forks)

1. **Which constructions A builds.** **Recommendation:** the reads off geometry, the projections, `InFrame`, `Flip`, `Standoff`, `Through` and `Meet`. The rest wait for a reader. The extrude's slant direction is built when a corpus document or a façade gesture writes one.
2. **`Point` and `Direction` subgroups.** **Recommendation:** both in A, because D10 names a lone point mate and its direction-mate lowering.
3. **A clocked coaxial** (today's `Coaxial` plus rider, residual `Prismatic`). **Recommendation:** an `Axis`–`Axis` mate plus a slide and a spin value (C), with the `Cylindrical` residual's chart in the shared `Subgroup`.
4. **Where the door reads the held fold** (F). **Recommendation:** fold the bundle at the door from its definitions. It is a handful of table rows, and it keeps "the doors decide edits" free of a stored solve.
5. **The viewer and the world.** **Recommendation:** the viewer reads no `world_of`. It draws every space from display state of its own that no logic reads, the rule G3's free-move probe already follows, and E's grep gate admits no display door.
6. **`Transform`'s Python spelling.** **Recommendation:** none. `body.transform(by)` goes with the node; the façade's `place` with values is the one spelling (Ev on #4220: one semantics in Python and Rust).
7. **What a mate reads on the world.** The world defines no pose variable, and a placement with no mates cannot chart a rotation. **Recommendation:** a mate's `to` may read the world node through a projection of its frame (the frame, its xy-plane, z-axis or origin). The read is part of the mate, never a variable, so nothing else can read it.
8. **An absolute pose with no geometry to restate it over** (D's migration). **Recommendation:** refuse to regenerate and name the node, after measuring how many corpus documents reach it; a corpus count above a handful comes back to the orchestrator before D dispatches.

### Boundaries with the neighbouring stages

- **Stage 2.**
  - A waits on E (a face's plane reads a `Face` variable).
  - B waits on F (mate sides read `Face` variables). Stage 2 F keeps `reading_edges`' gauge edges (its Q3), and B deletes them with gauges.
  - B retires `PlaceInWorld`, which stage 2 C builds; stage 2 had no world node to mate against, and Ev called the state transient (#4220).
- **Stage 4.**
  - **H waits on this stage's C** (`placed-carriers-compare-through-their-frames`). A placed copy's frame is the construction its bundle states (FORK-S3O), and H replays that fold at `Sym`. The bundle exists after B, but its mates' poses are read off geometry only after C, so H waits on C rather than replaying today's offset frames and then rewriting the replay. This moves H from "after B" to "after C".
  - **F waits on stage 4's C** (D has merged). F turns redundant mates into contacts the census records, and the rungs C and D are what prove those contacts structural rather than report them as unproven.
  - **B waits on A5's hard error retiring.** B drops declaring mates, so a contact one declared is unattributed, and A5 refuses that document's product. Stage 4 I retires A5's hard error but waits on H, which waits on this stage. The cut is to retire A5's hard error into an `unproven-coincidence` finding in its own stage 4 unit after stage 4 B, with C and D so most contacts prove, and to have B wait on it. Mate-placed contacts are then loud, non-refusing findings until H. Stage 4's spec owns that cut; until it is made, B is parked on `mates-declare-no-contact` and the cycle is named in `work/intent/log.md`.
  - Stage 4's canonical forms compare poses "modulo the kind's own symmetry", the same `Subgroup` A shares.
- **Stage 5.**
  - Its unit C (`the-at-rest-census-is-a-check`) checks per space and names F as its stage-3 trigger, which stands.
  - Its "copy" is "a `Body` output of a world placement"; this spec keeps that: a copy is an output of `Place` or of an instance.
- **Stage 6** owns coaxiality through one axis variable and the tangency constructions. A builds the `Axis` reads they use.

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

- **D10's Variables paragraph** says poses are "read off a body's geometry (a face's frame, a carrier's axis or centre)". The same paragraph's FORK-S3P round 10 sentence says "A face reads as a plane; no reader takes a carrier's reference direction", and a face's frame is only a frame by its carrier's reference direction. This spec builds the second sentence. Removing the parenthetical's "a face's frame" changes ratified text, so it goes to Ev.
- **`work/intent/a-mate-on-a-pinned-copy-refuses`**'s FORK-S3O section says a pin is "in practice … one `Frame` mate between two constructed frames (`FaceFrame` plus `Offset`, …)" and that the migration rewrites each bundle as one constructed `Frame` mate. FORK-S3M, merged after it and cited by Ev's approval of FORK-S3O, puts mates and values on equal footing and retires the `Offset` target; D10 says so. This spec builds D10, and the row's body is re-worded to match.
- **ASSEMBLY A11 (4)** says "an inconsistent loop dies at its closing mate's verification (`MateFault::Contradictory`)". No code verifies a declaring mate; `Contradictory` arises only inside one pair's fold or from a rider. B and F delete the clause.
- **`mate/coset.rs`'s module doc and A11 (1)** call the fold "exact coset intersection". Every branch is a decided predicate over measured values (`k_stats::decide`). F's rule decides by codimension alone, and A11 (1)'s rewrite (§9) says "subgroup algebra".
- **`work/intent/explicit-placement-frames-hold-floats`** says "the die's twenty-one pip frames". `die_tool.pncad` holds six `Explicit` frames.
- **`VarKind::symmetry`** has no callers in src. The solve reads `PoseSymmetry` on `DatumValue`, so the one `Subgroup` #4222 approved is two tables kept in step by a test (`intent_s2_a_outputs.rs`). A folds them.
