# SPEC: INTENT stage 3 — spaces and placement

Scope: D10's **Spaces and placement** paragraph and the pose half of its **Variables** paragraph (`docs/DESIGN.md` §D10), as `work/intent/plan.md` stage 3 states it:

- Frames and directions become variable kinds. A datum's origin and axes stop being independent reals.
- A placement is the bundle of mates that pins one copy.
- The world is one undeletable frame. Export reads its coordinates and nothing else does.
- Gauges, offsets, `Transform`-as-placement and absolute datums retire.
- The kernel computes each space in the frame of its earliest member (D9).
- A mate added to a pinned copy refuses as an overconstraint, decided by subgroup algebra (A11 (1)). That closes `mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion` with A11 (4).

**Rulings this spec follows.** Ev's rule that the redesign text governs earlier text applies throughout (`docs/prompts/designer.md` §2).

- **D10** (#3990/#4002, Ev-ratified). This is the provenance of the Spaces paragraph: `git log --all -S'frame of its earliest' -- docs/DESIGN.md` finds `84404cdbf`, the draft for the `[ev]` PR. The ruling's inputs are Ev's direction of 2026-10-03 (`work/recipe/one-way-to-say-dependency-and-intent.md`, items 1 and 4). Three of Ev's words bear on this stage:
  - "parts don't sit at (0,0,0) in their own space; they just don't have a location (or orientation)";
  - "you can't Transform an already placed part, you just determine its placement by relation to other parts";
  - on a redundant but consistent mate, "it would be kind of slick for the solver's DOF-counting layer to be clever … but i won't fight over it".
- **FORK-2b** (#4220): the product is the world, every copy a world placement defines. Stage 2 unit C builds `PlaceInWorld { body, pose }`. Ev's residue 3 put the pose in the placement "but it doesn't really matter since the state is transient", and the orchestrator answered: "Stage 3 turns a placement's pose into its mates".
- **FORK-1 and FORK-1b** (#4222):
  - each pose is a frame known up to its kind's symmetry;
  - the kinds stay D10's five (`Point`, `Direction`, `Axis`, `Plane`, `Frame`);
  - a combination is a named construction that refuses its degenerate case;
  - an incidence is constructed, never checked;
  - a 2-D value lives in the node that holds its frame.

  Ev added: "ideally the mates' `Subgroup` stuff can literally be shared". So one `Subgroup` type is both a pose's symmetry and what a mate folds.
- **FORK-3** (#4222): a selection is a definition, not a node, and the selection kinds are `Face`, `Edge`, `Faces` and `Edges`. This stage's constructions are definitions in the same way, as the scope states.

D10's last paragraph retires three things here: **ASSEMBLY A3**, **A11 (2)'s gauges and offsets** and **A11 (4)'s declaring mates**. §8 quotes each clause this stage retires or rewrites.

**Baseline.** The grep is at main `044b5eb2e`, with stage 2 unit A merged (PR 4295) and unit B in progress (`intent/s2-b-reads`). This spec is written against stage 2's **final shapes** (its §1), not today's code: after stage 2 F, every operand is a read, `PlaceInWorld` defines the product, `Select` defines `Face`/`Edge` variables, and mate sides read `Face` variables.

**Line numbers.** They cite current main and will drift under stage 2's units B–F. Each unit's dispatching orchestrator re-greps at the merge of the stage-2 unit it waits on and states the drift, as stage 2 did for stage 1.

Today's state of what this stage touches:

- **Poses.** `VarKind` already names the five poses (`var.rs:47`, from stage 2 A). `VarDef` (`var.rs:165`) is `Free`, `Defined` or `Output`, and `kind()` derives `Free` and `Defined` from a scalar `Dimension` only, so a free or defined pose cannot be represented.
  - Every pose is an output of a `Datum` node (`node.rs:877`). Four of the six datums are absolute, written in world coordinates: `Plane`, `Axis`, `Point` and `Frame` (`node.rs:880–914`; `Frame`'s doc at :916 says "in world space").
  - `wire_datum` (`eval/wire.rs:1333`) evaluates them to `topo::query::DatumValue` (`crates/topo/src/query.rs:320`).
  - No node defines a `Direction` port, and none reads one.
- **Placement.** There are six ways to place something:
  - absolute datums;
  - `Node::Transform` (`node.rs:2499`, a `Placement<S>` chain);
  - `PatternKind::Explicit(Vec<placement::Frame>)` (`node.rs:1276`, raw f64);
  - a gauge plus an offset (`Node::Gauge` `node.rs:2646`; `InstantiatePart.gauge`/`offset` `:2629`/`:2637`);
  - mates;
  - and, after stage 2 C, `PlaceInWorld`'s pose.
- **The solve.** `solve_group` (`mate/solve.rs:2269`) takes a spanning tree over each placing group (`places`, `:931`, is "same gauge reference"). Tree mates are `Determining` and fold to `Trivial`, else `MateFault::Under` (`:2353`). Every other mate stays `Declaring`.
  - **Nothing verifies a declaring mate against the solved poses.** The only traces are the comments at `:2147–2151`, `:2263–2268` and the `MateRole` display at `:350`.
  - ASSEMBLY A11 (4)'s "an inconsistent loop dies at its closing mate's verification (`MateFault::Contradictory`)" is not what the code does. `Contradictory` is raised only inside one member pair's fold (`:1873`, `:1882`) or by a self-contradicting rider (`:1223`). §12 lists this.
  - The at-rest gate mints a declaring mate exactly as it mints a determining one (`assembly.rs:1333–1337`, `:1350–1438`). It checks contact, never the alignment.
- **Offsets checked against the solve.** A non-root member's offset is checked by `check_offsets` (`mate/solve.rs:2417`), refusing `OffsetDisagrees` (`mate.rs:1232`) or `OffsetUnchecked` (`:1245`).
- **What a mate stores.** `Alignment<S>` (`mate.rs:361`) holds two `MateFrame { base: FrameBase, offset: Placement<S> }` (`:193`), a `MatePrimitive` (`:285`) and an `AxisSense` (`:272`).
  - `PlanarRest { offset: f64 }` and the rider `clocking: Option<f64>` are raw floats that no variable can drive.
  - `MateFrame::authored` (`:241`) turns raw f64 vectors into a `Step::Literal`.
- **The world and spaces.** The world is `gauge: None`. A placed group computes in world coordinates (gauge chain ∘ root offset, `group_frame` `mate/solve.rs:809`).
  - An unplaced group computes at the identity of its earliest instance (`Space::Own`, `:231`; `instance_frame` `eval/wire.rs:400`).
  - There is no computing frame anywhere else in the code.
- **Export.** It writes bodies in whatever frame they were evaluated in (`crates/step-export/src/lib.rs:81–84`; `pncad/src/export.rs:185`, `:239`).

## 0. Ordering: six PRs, each green, none landing half a representation

| PR | Unit (`work/intent/`) | Lands | Representation step it completes | Goldens |
|---|---|---|---|---|
| A | `poses-are-variables` | pose definitions (`VarDef::Pose`: coordinates in a frame, offsets, projections, the named combinations, the face frame), each space's seed (FORK-S3-1), and pose slots on every reader. The `Datum` node retires, the revolve's axis moves onto the node (FORK-1b), and explicit pattern frames become `Frame` variables | **a pose is a variable of its kind** | re-blessed: datum nodes become definitions, so ids move. Geometry bit-equal |
| B | `a-mate-relates-two-poses` | a mate reads two pose variables and its kinds are its primitive (FORK-S3-3). `MatePrimitive`, `MateFrame`, `FrameBase`, `AxisSense`'s literal arm, the clocking rider and `PlanarRest.offset` retire into pose definitions | **a mate equates two poses** | re-blessed: mate wire. Solved poses bit-equal |
| C | `a-placement-is-the-bundle-of-mates` | `Place { body, mates }` defines a copy (FORK-S3-2), and the world is an undeletable `Frame` variable. `PlaceInWorld`, gauges, offsets, `SetOffset`/`SetGauge`/`Promote`/`Fold`/`regauge_then_mate`, the spanning tree, roots and `MateRole::Declaring` retire, and spaces are derived from the bundles | **a placement is a bundle of mates; the world is a frame** | re-blessed. One-time migration check: poses and product bit-equal |
| D | `transform-retires-into-a-placement` | `Node::Transform` retires. A rigid motion of a body is a copy placed by one frame mate. `PlacedFrom` ports retire | **nothing moves a body but a placement** | re-blessed. Geometry bit-equal |
| E | `each-space-computes-in-its-earliest-members-frame` | the per-space computing frame (D9). Export composes the world's map, and nothing else reads it | **construction never reads the world** | **product body digests move** (computing-frame coordinates), and STEP bytes move by rounding. Measured values move within rounding |
| F | `a-mate-on-a-pinned-copy-refuses` | overconstraint refuses by subgroup algebra (FORK-S3-4): a mate that lowers no dimension of its bundle's fold refuses at the insert door and in the solve. The redundant-member re-measures retire | **a mate places and never checks** | re-blessed: corpus mates that overconstrain are dropped by a one-time migration that names each. Poses unmoved |

Why this order:

- **A before B.** A mate reads pose variables, so they must exist first. A is also where the seed, and hence "a part has no location", first exists. B then has something to equate.
- **B before C.** B changes *what* a mate reads, and C changes *what places*.
  - Doing them together would rewrite the mate wire, the coset table's inputs, the gauge model, the product, the refactor doors and every mate fixture in one diff.
  - After B the spanning tree still runs, but over pose pairs. After C the tree is gone.
  - B's bit-equality check (every MSOLVE fixture's `SolvedPoses` unmoved) is only cheap while the tree that produced those poses still exists.
- **C before D, E and F.**
  - D re-spells `Transform` as a `Place`, which is C's.
  - E's "space" is C's derived space. Before C a space is a gauge group, and it would be built twice.
  - F's "pinned copy" is a bundle's fold, which exists only after C.
- **D and E are independent** of each other, and both can be dispatched at C's merge.
- **F waits on stage 4 as well as on C.** Today a contact between two copies that are not directly mated is declared by a *declaring mate* (A11 (2), "contact between groups on different gauges is declared and verified at the at-rest gate").
  - If F refused such a mate before stage 4 retires A5's hard error on an unattributed contact ("Undeclared contact between instances is a hard error, never blessed"), every such contact would turn from a declaration into a refusal, and the documents that rest one placed copy on another through a third would have no product.
  - A mate beyond its bundle's pin refuses from C on (PRs 4325 and 4326): nothing verifies and mints it, and C's migration turns each declaring mate into an assertion or drops it, naming each. Until stage 4's `mates-declare-no-contact` (I) retires A5's hard error, the contact such a mate declared is unattributed. F, the overconstraint test in general, lands after I.
  - Stage 4's I in turn needs this stage's C, so the order is C → stage 4 H → stage 4 I → F (§11, boundary).

Each intermediate state is a whole representation:

- After A, poses are variables and every reader reads one. Mates still read faces and frame offsets, and gauges still place.
- After B, a mate equates two poses. Gauges, offsets and the tree still decide what places.
- After C, placements are bundles and the world is a frame. `Transform` remains as a body operation, computing is in world coordinates, and a mate beyond its bundle's pin refuses.
- After D, only placements move bodies.
- After E, each space computes in its earliest member's frame.
- F leaves "a mate places and never checks".

**Rejected:**

- **B inside C.** It is about 300 mate sites plus every gauge site in one PR, with no point at which the poses can be compared.
- **C before B, keeping face-and-offset mates in bundles.** The bundle would fold `MatePrimitive`s, and B would re-type it a stage later, so the wire would break twice.
- **E before C, over gauge groups.** Gauge groups vanish in C, so the computing frame would be chosen twice.
- **F inside C.** C's migration check is "poses and product bit-equal". F deletes mates. Mixing them hides which change moved a body.

## 1. Final shapes (after F)

**Pose variables** (`var.rs`). A pose variable is defined by `VarDef::Pose(PoseDef)`, a construction over other variables:

- **`Seed`**: a space's own frame (FORK-S3-1, recommended). It has no value, because "a part has no location". A document is born with one, and every pose in a part is defined from it.
  - Two seeds are two spaces until a placement relates them.
  - The seed is the earliest member of its space (§E).
- **`InFrame { frame, coords }`**: a pose written by coordinates in a frame, the absolute datum relativised.
  - It covers a point at `[S; 3]`, a direction `[S; 3]` (normalised, refusing zero), an axis (point and direction), a plane (point and normal) and a frame (origin, `u`, `v`, orthonormalised, refusing a degenerate pair).
  - The coordinates are scalar variables in `frame`'s own axes, so a tolerance on a datum's position is a tolerance on these scalars (VR8 unchanged).
- **`Offset { base: Frame, by: Placement<S> }`**: a frame moved by a rigid chain of scalar variables, written in `base`'s axes. A gauge's parametric placement (#3437) becomes this.
- **`Project { of, to }`**: the projections, by the order of FORK-1b:
  - `Frame` → `Plane` (its xy-plane), `Axis` (its z-line), `Point` (its origin) and `Direction` (its z);
  - `Plane` → `Direction` (its normal);
  - `Axis` → `Direction`.

  A finer value is read through its projection. A slot holds its own kind (D10).
- **The named combinations** (FORK-1b): `Through { axis, point } → Frame` refuses a point on the axis, `Meet { a: Plane, b: Plane } → Axis` refuses parallel planes, and so on. Each refuses its degenerate case typed and is never an automatic join. Only those a reader needs are built in A (Q4).
- **`FaceFrame { face: Face, spin: Angle }`**: DM1's derived frame, now a definition over stage 2 E's `Face` variable.
- **`World`**: the one undeletable world frame (C). It defines nothing and is read by placements and by export alone.
- **`OfCopy { copy: Body, pose }`** (C): a pose of a body as it is carried by one copy, which is how a mate reads a pose of another placed copy (§4).

A pose variable has no free arm with a value (FORK-S3-1). It has no unit, and it is never an analysis axis itself: its scalars are. It reads as its definition ("the xy-plane of the frame of Sketch "base"").

**One `Subgroup`** (FORK-1b; Ev on #4222). `VarKind::symmetry` returns the `SubgroupFamily`, and a pose *value*, `PoseValue<T>` (renamed from `topo::query::DatumValue` and moved into `editor-core`'s pose module), implements `PoseSymmetry` with a `Subgroup<T>`.

- `mate/coset.rs` reads the sides' symmetries there, and `side_symmetry` (`mate/solve.rs:1392`) goes.
- `Point` and `Direction` gain their subgroups when a mate reads one (Q5). A builds neither, because no mate reads them yet.

**Readers.** Every slot that reads a pose is a slot of its kind:

| Reader | Slot | Was |
|---|---|---|
| a profile | `plane: Frame` | `ProfileProgram.plane: RecipeNodeId` (`program.rs:470`) read as `Frame` or `FaceFrame` |
| `Split` | `tool: Plane` | a `Datum::Plane` id (`node.rs:2366`) |
| `Pattern`, `PlacedUnion` `Circular` | `axis: Axis` | a `Datum::Axis` id (`node.rs:1258`) |
| `Pattern`, `PlacedUnion` `Linear` | `direction: Direction` | three `Scalar` slots (`node.rs:1249`) |
| `PatternKind::Explicit` | `frames: Vec<Frame>` | `Vec<placement::Frame>`, raw f64 |
| `Tube`, `HollowTube` | `frame: Frame` | stage 2 B's frame read |
| `Revolve` | two 2-D slots in its profile's frame (`axis_origin: [S; 2]`, `axis_direction: [S; 2]`); it defines `axis: Axis` (FORK-1b) | an `AxisInPlane` id |
| a mate side (B) | a pose of its kind | `SitedFace` + `MateFrame` |

**Mates** (B, C). A mate is `Mate { a: S, b: S, sense: S }`. `a` and `b` are pose variables of one kind, and `sense` is a discrete `Sense` variable (`Aligned` | `Opposed`, D10's "a sense").

- The kinds are the primitive. `Frame`–`Frame` coincides, `Axis`–`Axis` is coaxial and `Plane`–`Plane` rests.
- Each pins the relative pose to a coset of the kinds' common symmetry, the `Subgroup` both poses name (A11 (1), reworded).
- Every number a mate used to carry is a scalar variable in a pose definition: a roll is an `Offset` rotation, and a stand-off is an `Offset` along the normal. So no primitive holds a number.
- `class: ContactClass` stays until stage 4 retires it.

**Placements** (C; FORK-S3-2, recommended).

- **The node.** `Node::Place { body: S /* Body */, mates: Vec<Mate> }` is an operation. It defines one `Body`, the copy. Its `mates` are its bundle: each relates a pose of `body` (read as the copy carries it) to a pose that is not the copy's own (another copy's, a pose of this document's seed, or the world).
  - The bundle folds by coset intersection to the copy's residual subgroup.
- **Pinned and placed.** A copy is **pinned** when its bundle folds to `Trivial`. A pinned copy is **placed in** the space of what its mates read.
  - A copy with an empty bundle lives in its own space (A11 (2)'s own-space rule, kept).
  - A bundle that folds to anything else refuses `Under` (A1's rung (c) is unbuilt).
- **The product** is every copy placed, transitively, in the world's space, in the placements' document order.
  - A10's sentences hold with "world placement" read as "a placement whose space is the world's".
  - Export writes that space and refuses an empty one, naming the unplaced bodies.
- **Two placements of one body are two copies** (D10).
- **What retires.** Nothing else places: no tree, no roots, no `Declaring`, no gauge and no offset.
  - Copies related only to each other are one space, unplaced in the world.
  - A boolean whose operands are in two spaces refuses (D10, Booleans). Today's `NodeErrorKind::Unplaced` (`eval/mod.rs:2027`) is that refusal, re-derived from bundles.

**Overconstraint** (F; FORK-S3-4, recommended). A mate added to a bundle that does not lower its fold's family dimension refuses `EditError::Overconstrained { placement, mate, held: SubgroupFamily }` at the insert door.

- A mate added to a pinned copy, whose held fold is `Trivial`, is the common case and is decided with no predicate at all.
- The solve refuses the same verdict for a state the door did not see (a rebind, a load), as A11 (1)'s door/solve split says.
- A dimension-lowering mate whose cosets do not meet still refuses `Contradictory` with its clash.
- `OffsetDisagrees`, `OffsetUnchecked`, `OffsetCheck` and the redundant-member re-measures in `coset::intersect` (`:1039–1046`) are gone.

**The computing frame** (E; D9). Each space computes in the frame of its earliest member, and a member is a seed or a copy (FORK-S3-6).

- A body is computed in its construction space's seed frame. A copy is a rigid image of it under its placement, and the at-rest census and cross-copy measures of a space run in the frame of that space's earliest copy.
- The choice reads document order only, never a value and never the world, so the world mate is never chosen.
- Export composes "computing frame → world" once per copy, and nothing else reads the world. The viewer reads no map to the world: it draws each space from display state of its own, which no logic reads (Q8).

**Instances** (C; FORK-S3-5, recommended). `InstantiatePart` defines, beside its per-world-placement `Body` ports (FORK-1):

- one `Bodies` port, `world`, holding all of them in placement order, in one space, so its copies keep their relative poses.

It defines no `Frame` port: no part's world frame is readable by a parent (PR 4326). Placing the instance places `world`, which is one `Place` reading a `Bodies`, and its mates read poses off the copies' geometry. Picking one body is DM3's index operation.

## 2. PR A — `poses-are-variables` (cost H; ~220 files, 6–9k lines, about half compile-driven)

**Kernel**

1. `VarDef::Pose(PoseDef)` with the arms of §1, except `World` and `OfCopy`, which are C's.
   - Each arm states its result kind, and the door checks it as `SlotVarKind` checks a slot. `InFrame`'s coordinates read scalars of the right dimension (`Length` for a point, `Scalar` for a direction).
   - `VarDef::kind()` (`var.rs:187`) reads the arm.
   - A pose definition may not reach its own variable, by VR3's acyclicity walk.
2. **The seed** (FORK-S3-1). `Doc::new` mints one seed. A seed is deletable only when nothing reads it (VR7). A second seed is `DeclareVar { def: Pose(Seed) }`, which the GUI offers as "a new part space".
3. **Evaluation.** Pose definitions are scheduled as definitions, not bound in `Doc::var_env` (`doc.rs:1651`).
   - A pose's value can depend on built geometry (`FaceFrame` reads a `Face`), so it is bound when its reads are, at the lane scalar, by one evaluator `eval_pose` that replaces `wire_datum` (`eval/wire.rs:1333`). The helpers are `frame_axes` (`:1298`), `frame_value` (`:1322`) and `frame_plane_lane` (`:1271`).
   - The schedule (`eval/schedule.rs`) treats a pose definition as a node of the dependency graph over `Doc::upstream`. That is the machinery stage 2 D builds for observed definitions, generalised to "a definition bound mid-evaluation".
   - A seed evaluates to the identity of its space.
4. **`Datum` retires** (`node.rs:877–1013`, and its arms at `:56`, `:1355`, `:3036`, `:3399–3422`, `:4056`, `:4222–4228`).
   - `Node::Datum` is deleted, together with its port arms in `Node::outputs()` (`:4217`).
   - The six datum constructors in `pncad-py` (`py/doc.rs:2514–2738`) become pose-definition builders with the same Python names: `sketch_frame`, `datum_plane` and the rest each return a `Var`.
5. **`AxisInPlane` retires into `Revolve`** (FORK-1b). `Revolve` gains `axis_origin: [S; 2]` and `axis_direction: [S; 2]` in its profile's frame coordinates, and its `axis: Axis` port is the lift.
   - `wire_revolve` (`eval/wire.rs:1773`, the kind match at `:1793`) reads the slots.
   - The F15 doc on `AxisInPlane` (`node.rs:924–948`) moves to `Revolve`'s fields, still saying why: four numbers in the frame cannot leave the plane.
6. **Readers** per §1's table:
   - `frame_plane_lane` call sites (`eval/wire.rs:1590`, `:1596`, `:4515`, `:4529`);
   - `tube_args` (`:1852`);
   - `stepped_map` (`:4282`, Circular at `:4308`; Explicit is refused at `:4293` and now reads frames);
   - `wire_split` (`:2668`, through `verbs/split.rs` `plane_of` `:155`);
   - `wire_placed_union` (`:4399`, the frames at `:4410`).
   - `PatternKind::Linear.direction` becomes one `Direction` slot. That makes it the first reader of a `Direction`, and it gives the kind a reason to exist (Q4).
7. **One `Subgroup`.** `PoseSymmetry` moves from `DatumValue` (`mate/coset.rs:245–260`) to `PoseValue`. `VarKind::symmetry` (`var.rs:102`) has no callers in src today; the solve reads `PoseValue::symmetry` and the kind's family is asserted equal at the door.
8. **The FaceFrame's zero spin** (DM1). It is "the carrier's u-reference", and that reference comes from `Vec3::orthonormal_basis` against the coordinate axes of the frame the body is computed in (`node.rs:1000–1010`).
   - From E, that frame is the body's seed, so zero spin stays a function of the recipe.
   - A states this in DM1's text (a re-wording forced by the representation, not a design change). E's test 17 pins it.

**Migration** (a one-time check, not a rule). Regenerating a pre-A file mints one seed and turns each datum into a definition on it:

- each absolute datum becomes `InFrame { frame: seed, coords }` over **the same scalar variables** (their ids are kept);
- `AxisInPlane` becomes `Revolve`'s slots;
- `FaceFrame` becomes the definition.

Every body digest is bit-equal, because `InFrame` on a seed evaluates the same arithmetic on the same values.

**Sites** (counts at the baseline; brace-matched constructions, match patterns in brackets):

| Area | `Datum::` constructions | Helpers |
|---|---|---|
| editor-core src | 21 [71] | `test_support::frame`/`xy_frame` (`test_support.rs:178`/`:188`) |
| editor-core tests | 145 [19] | `xy_frame()` 168, `on_frame(` 231, `on_frame_keeping(` 28, `axis_in_plane(` 45 |
| viewer | 10 + 22 [15] | `xy_frame()` 30, `framed_square(` 46; `session/author.rs` 6 |
| tour | 18 + 3 | teapot 4, diefillet 3 |
| pncad | 4 (tests) | `xy_frame()` 13 |
| pncad-py | 9 in src; Python `.sketch_frame(` 127, `.datum_*(` 75 | |

The helpers keep their signatures, returning a `Var` instead of a node id. Stage 2's `RecipeNodeId → port 0` sugar (its Q5) covers the struct-literal residue.

**Goldens.**

- Datum nodes leave the node table and enter the vars table, and the mint chain moves, so ids move. Re-bless with `M4_PR6_BLESS_GOLDEN=1` and state it.
- The six `Datum` rows of `tests/golden/slot_tables.txt` (lines 1–35) become pose-definition rows, and `Explicit([])` (line 137) gains its frame slots.
- `die_tool.pncad`'s six `Explicit` frames (line 372) become six `Frame` variables `InFrame { seed, … }` holding the same numbers. That closes `explicit-placement-frames-hold-floats`, whose "twenty-one pip frames" is six in the file; §12 lists the discrepancy.
- Every f64 geometry digest is bit-equal.

## 3. PR B — `a-mate-relates-two-poses` (cost H; ~140 files, 4–6k lines)

**Kernel**

- `Node::Mate { a: S, b: S, sense: S, class }` (FORK-S3-3). `a` and `b` are pose slots, and the door refuses a kind mismatch (`MateFault::KindsDiffer { a, b }`) and a kind no primitive equates (`Point` and `Direction` until Q5).
- **Retired** (`mate.rs`):
  - `Alignment` (`:361`), `MateFrame` (`:193`), `FrameBase` (`:121`) and `MatePrimitive` (`:285`);
  - the clocking rider (`Alignment::clocking`, the `FrameCoincidence` rider arm of `mate_coset`, `mate/solve.rs:1200–1229`, `Refuted::ClockingRedundant`);
  - `PlanarRest.offset`, `MateFrame::authored` (`:241`) and `table_gap` (`:628`);
  - `SlotId::MateFrameStep` (`node.rs:438`) and `VectorSlot::MateFrame*` (`:578`, `:586`).
- `mate_coset` (`mate/solve.rs:1168`) reads the two sides' `PoseValue`s and their common `Subgroup`, the table's row by kind.
- **A side on a face.** It is a `FaceFrame` definition, or a `Project` of one, over the face's `Select`.
  - The face-base read (`resolve_side` `:1436`, `topo::readback::face_pose`) moves into `eval_pose`, so it runs once per definition, not once per mate.
  - A11 (5)'s "the face name is the state, the frame is derived" holds unchanged.
  - The FaceFrame's convention (origin, u-reference, outward normal) is stated once, in DM1.
- **A pose read on a placed member.** Stage 2 F's member walk (`mate/member.rs` `walk` `:224`) still finds the member under the pose's face or body, because the tree and gauges stand until C. It now starts at the pose definition's body read.
- **Analysis.** A mate's scalars live in pose definitions, so they are seeded and boxed as any slot is, through `Doc::reads`. The solve's lanes (MSOLVE-14) are unchanged.
- **The edit door** (`admit_mate`, `mate/solve.rs:1676`) checks the kinds, the walk and the class. The table gap goes, because kinds make it unrepresentable.

**Migration.** Each `MateFrame` becomes a pose definition:

- `FrameBase::Part` + offset becomes `Offset { base: <the member body's seed>, by: offset }`.
- `FrameBase::Face` + offset becomes `Offset { base: FaceFrame { face }, by: offset }`.
- The primitive's kind picks the projection: `Coaxial` gives `Project { to: Axis }` and `PlanarRest` gives `Project { to: Plane }`.
- A `PlanarRest.offset` of `h` becomes one more `Offset` translation along z, of a new anonymous `Length` holding `h`.
- A rider `θ` becomes one more `Offset` rotation about z, of a new anonymous `Angle` holding `θ`.
- `MateFrame::authored`'s literal step becomes `InFrame` coordinates.
- `Coaxial` + `Clocking` becomes a `Frame`–`Frame` mate whose offset frees nothing: a roll pins the frame. The two-step `Prismatic` case (`mate/solve.rs:1192–1280`) needs Q6.

**Surfaces.**

- The viewer's mate tool (`crates/viewer/src/matetool.rs`) authors `FaceFrame` definitions and projections. `MateChoice { primitive, clocking }` (`:451`) becomes `{ kind, roll: Option<Formula> }`, and the face-frame pre-check (`:682–690`) reads the definition.
- Python's `MateFrame`, `Alignment` and `Mate(...)` (54 lines in `test_assembly_author.py`) take two poses.

**Sites.** `Node::Mate {`: 53 src, 115 tests, 19 viewer, 6 tour, 2 pncad, 2 pncad-py. `MateFrame::` constructions: 11 src, 114 tests, 16 viewer, 9 tour, 9 py. `Alignment {`: 23 src, 82 tests.

**Goldens.**

- Mate wire rows and `slot_tables.txt`'s mate rows move, and ids move.
- **Every `SolvedPoses` in the MSOLVE fixtures (`msolve1`–`msolve14`, `asm_r2a_mate_solve.rs`, `place_mate_frame_offset.rs`) is bit-equal.** The migration composes the same frames in the same order: `compose_offset` (`mate/solve.rs`) becomes `Offset`'s evaluation, so this is a refactor of one arithmetic path.

**Closes** (the rows parked on `intent-stage3-is-built` whose code B deletes):

- `a-clocking-rider-is-levered-unreduced`;
- `a-face-frame-cannot-turn-its-roll` (a roll is an `Offset` angle on any frame);
- `a-face-base-puts-its-reference-on-local-y` (the convention is DM1's, stated once, and an offset along the face's reference is an `Offset` along x);
- `mate-primitive-unit-variants-load-from-a-null-payload` (`MatePrimitive` is deleted);
- `a-mate-frame-axis-is-decided-against-a-length-band` (the composed axis is a pose value with its own unit witness, never re-minted against a length band);
- `a-mate-frame-is-written-in-the-reading-instances-coordinates`, together with C (a pose is defined over the face it names, so its numbers mean the same thing whichever instance reads it).
- MSOLVE-15 (#3681, rider, `Clocking` and stand-off deletion) is subsumed. Its PR is closed with a pointer here.

## 4. PR C — `a-placement-is-the-bundle-of-mates` (cost H; ~260 files, 8–11k lines)

The largest unit. Its size is in the gauge sites, which are compile-driven, and in the refactor doors, which are not.

**Kernel**

- **`Node::Place { body: S, mates: Vec<Mate> }`** per §1 (FORK-S3-2). It defines one `Body`, the copy.
  - A mate in the bundle reads a pose of `body` on one side and, on the other, a pose outside the copy: `OfCopy { copy, pose }` of another `Place`'s output, a pose of this document's seed, or the world.
  - The door refuses a bundle mate with no side on `body` (`MateFault::NotOnTheCopy`) and one that reads its own copy through `OfCopy` (acyclicity over `Doc::upstream`).
- **The world.** `VarDef::Pose(World)` is minted with the document, undeletable (`EditError::WorldIsUndeletable`), and read only by bundle mates and by export.
- **The solve** (`mate/solve.rs`) is per placement. Each bundle folds through `fold_pair`'s coset intersection (`:1779`), and a copy's pose is its pinning target composed with the folded representative.
  - **Retired:**
    - `solve_group` (`:2269`) and its BFS (`:2337–2383`);
    - `groups` (`:943`), `root_and_cause` (`:951`), `root_of` (`:1051`) and `places` (`:931`);
    - `group_frame` (`:809`), `gauge_chain` (`:743`) and `gauge_frame` (`:774`);
    - `MateRole` (`:216`), its Python door, and 93 test mentions;
    - `Unplaced::{NoOffset, DeadGauge}` (`:263`), which become "an empty bundle" and "a bundle reading an unresolved pose".
  - `SolvedPoses` (`:375`) keeps `placement` (`:588`) and the A2a pairing door, keyed by placement.
- **A mate beyond the pin refuses.** A bundle mate after its fold is `Trivial` refuses `Overconstrained` at the insert door and in the solve; nothing verifies and mints it (PRs 4325 and 4326 state the rule).
- **Gauges and offsets retire:**
  - `Node::Gauge` (`node.rs:2646`) and `InstantiatePart.gauge`/`offset` (`:2629`, `:2637`, the serde `present` door `persist/wire.rs:451`);
  - `DocEdit::SetOffset`, `SetGauge`, `Promote` and `Fold` (`edit.rs:521`, `:543`, `:567`, `:595`; applied at `:6547`, `:6617`, `:6648`, `:6684`);
  - `regauge_then_mate` (`:5272`), `clear_joined_offsets` (`:7007`), `mate_that_would_start_placing` (`:6828`) and `check_gauge_ref` (`:6971`);
  - the thirteen gauge `EditError`s (`:2177–2267`) and `Maintenance::OffsetCleared` (`:3962`);
  - `check_offsets` (`mate/solve.rs:2417`) and `MateFault::OffsetDisagrees`/`OffsetUnchecked`/`OffsetCheck`/`OFFSET_RECOURSE` (`mate.rs:1232`, `:1245`, `:1303`, `:1298`);
  - `GaugeRefFault` (`doc.rs:2099`), the load walk's gauge check (`persist/check.rs:1736–1760`) and `SnapshotError::NotAGauge`/`GaugeCycle` (`:1140`, `:1148`).
- **`PlaceInWorld` retires** into `Place` with one `Frame`–`Frame` mate between the copy's seed and `Offset { World, pose }`.
  - Stage 2 C's product gather reads placements in the world's space.
  - The derived spaces replace `Product::spaces`/`own_spaces` (`product.rs:924`, `:947`), `gate_spaces` (`assembly.rs:1199`), `Evaluation::across_spaces`/`unplaced`/`unplaced_below` (`eval/mod.rs:181`, `:130`, `:136`) and `CarriedUnplaced` (`assembly.rs:357`). Each keeps its meaning, read from bundles instead of gauges.
- **A9** (`relative_freedom_components`, `mate/solve.rs:877`) runs over `Doc::upstream` alone. The gauge edges stage 2 F kept (its Q3) are gone with gauges.
- **Instances** (FORK-S3-5): the `world: Bodies` port, and no `frame` port (PR 4326). `eval/parts.rs` delivers the part's world copies in one space.
- **Refactor** (`refactor.rs`). Split cuts a set of placements and the bodies they read. Inline puts an instance's part's placements back as copies.
  - **Retired:**
    - the anchor vote (`:2885–2918`, `TwoAnchors` `:554`), `UnplaceableRoot` (`:599`), `UnplacedAlone` (`:609`) and `SeveredGauge` (`:541`);
    - `DeadGaugeReference` (`:577`), `TornGroup` (`:528`), `PlacingMateLeft` (`:569`) and `WouldStartPlacing` (`:620`);
    - inline's `UnplaceableFrame`, `MatePlaced`, `Unplaced`, `MovedMemberOffset` and `PartDeadGauge` (`:1229–1273`);
    - `gauges_first` (`:235`) and the offset re-statement (`:372–425`).
  - **What replaces them.** A cut takes whole placements, and a bundle mate reading across the cut re-points to the same pose as the instance's copy carries it (`OfCopy`), or refuses as a severed read.
  - **Acceptance** keeps A4's form: split-then-evaluate equals unsplit evaluation at structural and name identity, and inline-of-split returns the document up to minted ids.
  - `split-and-inline-over-a-mate-read-at-a-union-are-unmeasured` gets its rows here if stage 2 F did not land them, because C rewrites `is_mate_edge_end`, `frame_survives`, `MateFrameCrosses` and `MatePairSplits`.

**Migration** (a one-time check, not a rule). Regenerating a pre-C file:

- **Gauges.** Each `Gauge { parent, placement }` becomes a `Frame` variable `Offset { base: <parent's frame, or World>, by: placement }`, keeping its name if it had one. The turntable (`demos/tour/src/assembly.rs:598`) becomes a named `Frame` driven by its angle.
- **A group root on a live gauge chain.** It becomes `Place { body, mates: [Frame(copy seed) = Offset { gauge frame, offset }] }`.
- **Every tree mate** goes into the bundle of the copy it determined: the child in today's BFS from the root (`mate/solve.rs:2337–2383`). So every pose is reproduced by the fold that produced it.
- **Non-tree (declaring) mates** become assertions where a stage 2 D measure states them, and are otherwise dropped; the migration's report names each.
- **A non-root member offset** (a check under A11 (2)) is dropped, and the migration's report names each. Stage 5's `Assert` is the place to say it again.
- **An unplaced group** (no live chain) becomes copies whose bundles read each other and not the world, so it is its own space.
- **The check** (test 6): every corpus document's `SolvedPoses::placement`, for every copy, and its product digests are bit-equal to pre-C.

**Surfaces.**

- Python:
  - `Doc.regauge_then_mate`, `Doc.offset`/`gauge`, `Node.gauge`, `DocEdit.set_offset`/`set_gauge`/`promote`/`fold` and the `groups`/`root_of`/`reading_edges` functions go (`py/doc.rs:1106–4581`, `py/mate.rs:1134–1173`; `.pyi` :2659–6253);
  - `Doc.place(body, mates=[…])` and `Doc.world` arrive, with `Doc.place(body, at=frame)` as the façade's one-mate sugar;
  - `test_gauges.py` (73 lines) is restated as placements.
- Viewer: there is no gauge UI. Its exhaustive arms (`session/refuse.rs:160`, `:1415`; `combine.rs:1028`; `tree.rs:646`, `:842`, `:886`, `:1189`; `session.rs:3137–3143`, `:3372`) follow.
  - The mate tool commits into a `Place`'s bundle. Across two unrelated spaces it first places one copy relative to the other, so `vseam/the-mate-tool-across-two-gauges-commits-a-declaring-mate` is answered.
  - A GUI gesture may author a mate ("place where shown", `offer/viewer-free-move-and-place-where-shown-over-a-whole-group`), but the mate reads poses off geometry and numbers the user supplies. The viewer's display location is never real and is never read as a mate's target.
- Tour: `assembly.rs` (43 gauge lines) and `bench`.
- Docs: `docs/guide/assembly.md` :26–28, :134–151, :246, :304–312, :480–487, :711, :752–753, :870 and :1052.

**Sites.** `[Gg]auge` lines: 615 src, 803 tests (34 files; `p2_split.rs` 226, `p2_gauges.rs` 136, `p2_gauge_offsets_and_spaces.rs` 110, `p2_promote_fold.rs` 100), 37 viewer, 34 tour, 132 pncad-py src, 73 `.pyi`, 93 Python tests. `SetOffset` 19 + 76, `set_gauge` 113 in tests.

**Closes:**

- `mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion`, its offset half: offsets are gone. The declaring half is F's, so the row closes at F.
- `a-declaring-mates-alignment-is-never-read`: there is no gauge, and a bundle mate's poses are read by the fold.
- `msolve/gauge-of-recomputes-the-clusters-per-placement-lookup`: it is stale. `gauge_of` is only a test helper (`tests/p2_promote_fold.rs:48`), and the clusters are gone.
- `msolve/a-placer-row-states-what-a-poisoned-row-cannot`: `PlacerRow` and the member walk through placers retire. A pose over a pattern copy is a definition over DM3's index output, and its failure is that output's.
- `recipe/placement-step-slots-are-spelled-three-ways`: one spelling is left, `Offset.by`'s steps.
- `topo/a-boxed-rotation-refuses-not-rigid-at-every-placer`, its placement half only. A boxed `Pattern` angle still refuses, as the re-homing table says, so the row is re-opened on that half.

## 5. PR D — `transform-retires-into-a-placement` (cost M; ~90 files, 2–3k lines)

- `Node::Transform` (`node.rs:2499`, `wire_transform` `eval/wire.rs:4220`, `placeable_operand` `:914`) and `PortKind::PlacedFrom` (`node.rs:4191`) retire.
- A rigid motion of a body is a `Place` with one `Frame`–`Frame` mate between the copy's seed and `Offset { <the reader's frame>, by: placement }`, in the space of what it is placed against. "You can't Transform an already placed part" (Ev) is now unrepresentable: a copy is placed by its bundle, and a second statement is a second mate, which F refuses.
- **Names.** DM3 says "names pass through unchanged, as `Transform`'s do". A copy's names pass through in the same way (`names::role`), so the downstream selectors spell what they already spell.
- **`Bodies`.** Stage 2 A made `Transform` over a `Bodies` define a `Bodies`. `Place` over a `Bodies` does the same (FORK-S3-5).
- **The member walk** (`mate/member.rs` `transform_map` `:863`, `Placing::Transform` `:64`) has nothing left to walk through. `member.rs` keeps only the pattern and union descent that stage 2 F leaves, if any.
- **Sites:**
  - `Node::transform(` 98 in tests, 20 in viewer tests and 1 in py;
  - Python `.transform(` 26 and `.transform_by(` 6;
  - in the tour, `chain.rs:463` (links), `diefillet.rs:313` (a pip ball per transform, then a union: the copies are placed against the die's seed, in one space, so the union stands) and `teapot.rs:942` (the spout);
  - the viewer's transform gesture, `session.rs:2881`.
- `placement::Frame` stays as the evaluated value type.
  - `Step::Literal` (`placement.rs:565`) retires: after A and B no document holds a literal frame, because every frame is a variable, so A6's admission (`Frame::admission_fault` `:315`) runs on evaluated pose values at `eval_pose`.
  - The edit-door half of A6 (`node.rs:3946` `placement_frame_fault`) checks that an `Offset`'s chain is rigid by construction (a rotation axis and angle) and no longer needs to look at a matrix.
- **Goldens.** Bit-equal geometry: `Place` evaluates through the same `topo::transform_rigid` call. The test `r1_the_placement_frame_matches_the_transform_node_bit_for_bit` (`asm2a_instantiate`) is restated as "matches the copy".

## 6. PR E — `each-space-computes-in-its-earliest-members-frame` (cost M; ~60 files, 2–3k lines)

- **Members.** A space's members are its seeds and its copies (FORK-S3-6). The earliest is the first in document order: the seed's mint position, or the `Place` node's.
  - The world is never a member.
  - A seed's space computes bodies in the seed's frame, which A already does, since a seed is the identity.
  - A space of copies computes the at-rest census, the cross-copy measures (`clearance.rs`, `names/flush.rs`) and the boolean of copies in the frame of its earliest copy. Each other copy is mapped relative to it.
- **The world's map.** `SolvedPoses::world_of(space) -> Frame` is the one door that composes a space with the world. Export (`pncad/src/export.rs:185`, `:239`) reads it, and **nothing else may**, the viewer included.
  - A lint test greps `editor-core/src` for callers outside `export` and `persist`, as `scripts/gates/test-features-dev-only.sh` does for its feature.
- **D9's promise.** An edit that changes only the world mate moves no body bit and no measured bit. Only the export bytes move.
- **What moves.** Product body digests on every corpus assembly whose earliest copy is not at the world's origin: they are now in computing coordinates.
  - STEP export bytes move at rounding level (one more composition).
  - A measured value across copies may move by rounding.
  - Re-baseline and say what moved: the bodies are the same sets, now relative to the earliest copy.
- **The MSOLVE numerics.** The rows re-homed to stage 3 whose cause is magnitude, not a re-measure (`a-far-meeting-point-fails-membership-by-its-own-rounding`, at a pose some 1e7 m out), are re-read here. A computing frame near the geometry may move their numbers; F removes the re-measure that refuses.

## 7. PR F — `a-mate-on-a-pinned-copy-refuses` (cost M; ~70 files, 2–3k lines)

**Kernel**

- **The door.** `admit_mate` (`mate/solve.rs:1676`) gains the bundle's held fold, read from the last solve or folded at the door (Q7).
  - It refuses `EditError::Overconstrained { placement, mate, held }` when the added mate's subgroup does not lower the held family's dimension (FORK-S3-4).
  - The rows the refusal's dimension test reads are `coset::table` (`mate/coset.rs:647–824`). The pinned case is `(Trivial, _) => Trivial` (`:660–662`), with no predicate.
- **The solve** refuses the same in `fold_pair` (`:1779`) for a state the door did not see: a rebind, a load, or a definition edit that changed a kind's family. It reports `MateFault::Overconstrained`, keeping A11 (1)'s split.
- **Retired:**
  - `coset::intersect`'s membership re-measure of a non-lowering mate (`:1039–1046`) and `trivial_member` (`:974`);
  - the redundant arms of `Refuted` (`mate.rs:1496`).
- **Kept:** `Contradictory` for a lowering mate whose cosets do not meet (`FoldStop::Clash`, `Subgroup::Empty`), with its recourse.
- **The at-rest gate.** It no longer mints the retired mates. Their contacts are stage 4's `unproven-coincidence` findings, structural when "a mate-placed face" is (D10's Assertions).

**Migration.** Regenerating drops every corpus mate that overconstrains its bundle, and the report names each.

**Goldens.** Poses are unmoved (a dropped mate placed nothing). The gallery's at-rest rows that counted declared contacts are restated by stage 4 and 5's shapes.

**Closes:**

- `mate-offset-verified-against-the-solve-is-a-constraint-falling-back-to-an-assertion`;
- `a-box-over-a-solved-clocking-widens-thirty-thousandfold` (its widening is `member_of`'s re-measure of a residual met by construction);
- `an-identically-zero-margin-escalates-at-a-fine-eps` (the same re-measure and `check_offsets`, both gone).

**Re-read, released:**

- `a-far-meeting-point-fails-membership-by-its-own-rounding`: its false `Contradictory` came from `mate_member_translation_in_plane`'s re-measure. Close it if F's door refuses the case first, else re-open it on the dimension-lowering arm.
- `a-box-independent-mate-fault-bisects-the-whole-leaf-budget`: F's `Overconstrained` and C's `Under` are box-independent mate faults. Its fix (a terminal class in `drive.rs` `classify_replay`) is workable once F names the vocabulary, so it is opened with that list. It keeps `the-box-driver-carries-no-part-resolver`.

## 8. What moves and what retires

**Ratified clauses this stage retires or rewrites.** Each one quoted is D10-listed (its last paragraph) or is consequential re-wording. The forks say where substance moves.

- **ASSEMBLY A3** (B, C; D10-listed): "`Node::InstantiatePart { doc_ref, interface, gauge, offset }` instantiates a pinned document; it names its gauge and may carry an offset in it (A11 (2))" … "`Node::Mate { a, b, class, alignment }` is one contact declaration that also places" … "Every number that says where the two sides meet lives in the sides' offsets".
  - It is rewritten as the node vocabulary: `InstantiatePart { doc_ref, interface }`, `Place`, and a mate equating two poses.
  - "Mates are declarations" goes. A mate places and never checks.
- **A11 (2)** (C; D10-listed), entire: "Placement lives on a gauge. A gauge is a document node that holds a placement and denotes no body …" through "A further statement of where a placed instance sits is verified against the solve, never trusted and never silently ignored."
  - These are kept, re-stated over bundles: the parametric placement (#3437, now `Offset` definitions); "no edit records a frame"; and the own-space rule (#3441), now "a copy whose bundle reads nothing placed in the world".
- **A11 (3)** (C): "A group's tree is rooted at its earliest member carrying an offset when its gauge chain is live, and at its earliest instance in document order otherwise". It retires, because there are no roots.
- **A11 (4)** (C, F; D10-listed): "Tree mates DETERMINE and must fold to `Trivial` …; non-tree mates DECLARE and are only verified by the gate. No cycle is ever solved; an inconsistent loop dies at its closing mate's verification". It retires.
  - C keeps "the solve is total and per-node" and "whether the document has a product is the gather's question alone".
  - F replaces the rest with the overconstraint rule.
- **A11 (1)** (B, F): "Each primitive pins the pair's relative pose to a coset of an SE(3) subgroup … several mates on one pair fold by exact coset intersection … to DETERMINED, UNDER or CONTRADICTORY". It is reworded: a mate's kinds name its coset (B).
  - It gains "a mate that lowers no dimension refuses as an overconstraint" (F, FORK-S3-4).
  - "exact" is corrected to "decided": every branch of the fold is a decided predicate (`mate/coset.rs`, `k_stats::decide`), and the module doc's "exact coset intersection" is the same misstatement (§12).
- **A11 (5)** (B, C, D): the world-pose and member-walk paragraphs ("A placed instance's world pose composes its gauge's frame and its root's offset onto the solved relative pose"; "The member's frame is the composed offset of every node that walk passed"). Both are rewritten over bundles and pose definitions. "What the solve reads" keeps its two answers: the extent lever, and the face pose, now read in `eval_pose`.
- **A6** (A, D): "a literal step of an instance's offset, a gauge's placement or a transform's chain, and an explicit placement rule's listed frames, refuse `EditError::ImproperPlacement`". Its sites become: an evaluated pose value, and an `Offset`'s chain. The rule (det ≤ 0 refuses) stands.
- **A9** (C): "every placed group has a world pose, so the placed part of an assembly is one body. A group nothing places lives in its own space (A11 (2))". It is rewritten over bundles.
- **A10** (C; FORK-S3-2): "A world placement (`PlaceInWorld`) is an operation reading one `Body` and defining its copy as an output". It becomes "a placement whose space is the world's". FORK-S3-2 sends it to Ev, because #4220 wrote it.
- **A4** (C): the gauge sentences of *Split* and *Inline* ("every gauge reference leaving it lands on one anchor, a kept gauge or the world"; "the instance's frame becomes a gauge under the instance's gauge holding its offset") and all of *Promote and Fold*. They retire, and the acceptance stays.
- **REFERENCES DM1** (A, B): "A11 keeps the solve over authored numbers" and "a mate frame by a solve that must not [read geometry]". A11 (5) already reads the face pose, so the clause was stale. It is rewritten: a frame definition reads its face, and a mate reads a frame.
- **REFERENCES §0**: "`AxisInPlane` … the one datum with a DAG input". It is deleted (A).
- **D10 itself**: no sentence changes unless FORK-S3-1 (free poses), FORK-S3-2 (A10's world-placement sentence) or FORK-S3-6 (what a member is) rules so.

| | A | B | C | D | E | F |
|---|---|---|---|---|---|---|
| Node ids, pinned hex, memo fixtures | **all** (datum nodes leave) | mates and later | gauges and placements | transforms | — | — |
| Var table, mint log | grows (seed, pose definitions) | grows (mate offsets as definitions) | grows (world, gauge frames) | grows | — | — |
| Wire | `Datum` gone; pose definitions; `Revolve` slots; `Explicit` frames | `Mate` | `Gauge`, `gauge`/`offset`, `PlaceInWorld` gone; `Place` | `Transform` gone | — | mates dropped |
| Solved poses | — | **bit-equal** | **bit-equal** (test 6) | — | relative to the earliest copy | **bit-equal** |
| Product body digests | **bit-equal** | **bit-equal** | **bit-equal** | **bit-equal** | **move** (computing frame) | **bit-equal** |
| STEP export bytes | equal | equal | equal | equal | **move by rounding** | equal |
| `slot_tables.txt` | datum rows → pose definitions | mate rows | gauge rows go | transform rows go | — | — |
| Tokens, axes, MC, stackup | — (the same scalars) | the mate scalars join as slots | — | — | — | — |
| Python `.pyi` | datum builders return `Var` | `Mate(a, b)` | `place`, `world`; gauge doors go | `transform` goes | — | — |

**f64 geometry moves in E alone.** In every other unit, a body digest, a measured bit, a solved pose or a tour frame that changes is a bug, not a re-baseline. In E, what changes is the coordinates the bodies are written in, never their shape: test 15 checks each moved digest against the old one mapped through the world's map.

## 9. Test plan (each row names the runtime value that breaks it)

1. **(A) Datums are definitions; geometry unmoved.**
   - `golden.cad` regenerated: no `Node::Datum`, one seed, and every datum an `InFrame { seed }` over the same scalar ids.
   - Every body digest is bit-equal.
   - *Breaks if* `InFrame`'s evaluation orthonormalises in a different order from `frame_axes` (a frame's `v` moves by an ulp, so a profile's digest moves).
2. **(A) A slot holds its kind.**
   - `Split { tool: <a Frame variable> }` refuses `SlotVarKind { expected: Plane }`. `Split { tool: Project { of: f, to: Plane } }` is accepted and cuts as the frame's xy-plane.
   - *Breaks if* a finer value is admitted where a coarser one is asked for without its projection (D10: "a finer value is read through its projection").
3. **(A) A combination refuses its degenerate case.**
   - `Through { axis, point }` with the point on the axis refuses `PoseDegenerate { construction: Through }` at evaluation, at the scalar that decides it.
   - *Breaks if* the construction picks a frame anyway (an arbitrary roll, D10's "never an automatic join").
4. **(A) Two seeds are two spaces.**
   - A boolean of a body on seed 1 and a body on seed 2 refuses `Unplaced` (spanning two spaces), with no value read.
   - *Breaks if* spaces are decided from coordinates (two seeds "at the same place" fuse).
5. **(A) The revolve's axis cannot leave its plane.**
   - `Revolve { axis_origin: [0, 0], axis_direction: [0, 1] }` on a frame tilted by a variable `θ`: the `axis: Axis` port lies in the frame's plane for every `θ` in a box run (its direction's margin against the plane's normal is identically zero in Sym).
   - *Breaks if* the axis is lifted through world coordinates.
6. **(C) The one-time migration.**
   - For every corpus `.pncad` and every MSOLVE fixture: `SolvedPoses::placement` for every copy, and `product_recorded`'s digests in order, are bit-equal to pre-C.
   - *Breaks if* a tree mate goes into the parent's bundle instead of the child's (the pose is computed by the inverse composition, which need not round alike), or a gauge chain is folded outer-first.
7. **(B) Poses unmoved by the mate respelling.**
   - Every MSOLVE fixture's `SolvedPoses` is bit-equal pre/post B.
   - *Breaks if* a migrated `PlanarRest.offset` becomes a translation in the *other* side's frame (a non-zero stand-off flips sign).
8. **(B) A roll on a face frame.**
   - A `FaceFrame` mate whose `Offset` rotates by `θ = turn/4` turns the copy a quarter turn about the face normal. That is the case `a-face-frame-cannot-turn-its-roll` could not author.
   - *Breaks if* the rotation composes on the base side (it turns about the base's z, not the face's).
9. **(B) Kinds are the primitive.**
   - `Mate { a: Axis, b: Plane }` refuses `KindsDiffer` at the door, and the doc is unchanged.
   - *Breaks if* the door admits it and the solve folds a cylindrical against a planar subgroup.
10. **(C) Two placements, two copies.**
    - Two `Place`s of one body, each with one frame mate to `World` at different offsets: two copies, distinct output ids, and each name resolves once per copy.
    - *Breaks if* the bundle is keyed by body, not by placement (the second overwrites the first).
11. **(C) The product is the world's space.**
    - A copy placed against a copy placed against the world is in the product. A pair of copies placed only against each other is one own space: they are not in the product, `EmptyProduct` names them, and the at-rest census between them runs (A11 (2)'s own-space rule).
    - *Breaks if* membership is "has a bundle" rather than "reaches the world".
12. **(C) Under refuses; empty is own space.**
    - A copy with one `Axis` mate refuses `Under { residual: Cylindrical }`. A copy with no mates is its own space and refuses nothing.
    - *Breaks if* an empty bundle is read as `Se3` and refused.
13. **(C) The world is undeletable and read by placements only.**
    - `DeleteVar(world)` refuses `WorldIsUndeletable`. A construction slot (an extrude's profile frame) reading `World` refuses `ConstructionReadsTheWorld` at the door.
    - *Breaks if* the world is admitted as an ordinary frame (D10: "construction never reads the world").
14. **(C) Split and inline over bundles.**
    - A cut taking one of two mated copies re-points the remainder's bundle mate to the pose as the instance's copy carries it, and split-then-evaluate equals the unsplit poses.
    - Inline-of-split returns the document up to minted ids.
    - *Breaks if* the re-pointed mate reads the part's *world* coordinates (the copy moves by the part's world offset).
15. **(E) The computing frame.**
    - For every corpus assembly: each copy's digest at E equals the pre-E digest mapped through the inverse of `world_of(space)`, up to the rounding the test states.
    - Moving the world mate of a one-space document moves no body digest and no measured bit, and moves the STEP bytes.
    - *Breaks if* any evaluation path still composes the world's map (a measure across copies moves with the world mate).
16. **(E) The earliest member decides, never a value.**
    - Reordering two copies' `Place` nodes (a recipe edit) changes the computing frame. Changing a copy's mate offset (a value edit) does not.
    - *Breaks if* the frame is chosen by extent or by distance to the origin.
17. **(E) The face frame is the recipe's.**
    - A `FaceFrame` with zero spin on a body: its value relative to the body is bit-equal whether the body's space is evaluated alone or as a copy placed in a larger space.
    - *Breaks if* `orthonormal_basis` reads the copy's computing frame instead of the body's seed (the sketch on the face turns).
18. **(F) A mate on a pinned copy refuses, measure-free.**
    - A copy pinned by a `Frame` mate: inserting any further mate refuses `Overconstrained { held: Trivial }`, and the door's predicate log is empty (no `decide` call).
    - *Breaks if* the door folds the new mate before checking the held family.
19. **(F) A redundant mate on an unpinned copy.**
    - A `Plane` mate, then a second `Plane` mate on parallel faces, refuses `Overconstrained { held: Planar }` whatever its offset.
    - A second `Plane` mate on perpendicular faces is accepted (`Prismatic`).
    - *Breaks if* the redundant mate is admitted when its offset agrees (Ev's "slick" case, deferred by FORK-S3-4).
20. **(F) A lowering mate that does not meet.**
    - Two `Axis` mates on skew lines whose distance differs from the copy's axes' distance refuse `Contradictory` with the clash.
    - *Breaks if* F's rule swallows `Contradictory`.
21. **(A–F) Python.**
    - `doc.datum_plane(...)` returns a `Var` of kind `Plane`; `doc.place(body, at=doc.world)` builds a copy.
    - The census and `.pyi` agree, and every gauge door is absent from the census.

Loud census rows: `pncad-py` `tags.rs` / `surface_census` / `prose_census`, `display_contract`, the persist `Walk` roster, `tests/golden/slot_tables.txt`, `f6_variants!` and `eval/class.rs`'s class table (`MateOffsetDisagrees`/`MateOffsetUnchecked` go; `Overconstrained` comes).

## 10. Risks

- **C's size.** About 1,600 gauge lines in src and tests plus the refactor doors. The gauge sites are compile-driven. The refactor rewrite is not, and it is where review time goes.
  - C can split only along a representation boundary, and the one that exists is B|C. A split of C itself (gauges, then the tree) would leave a document with bundles and gauges, two ways to place, which "no stage lands half a representation" forbids.
- **Pose definitions bound mid-evaluation** (A). A `FaceFrame` reads built geometry, so a pose value exists only after the body it reads. This is the same schedule shape stage 2 D builds for observed definitions, at a much wider reach: every profile reads a frame.
  - If D's machinery is narrower than this needs, A widens it, and the content key must hash a pose definition's upstream keys, not its id.
- **Mates reading poses of copies** (C). `OfCopy` is new: a pose as carried by a copy is a function of the copy's solved pose, so the bundle fold reads poses that depend on other bundles' solves.
  - The order is the bundles' read order over `Doc::upstream`, acyclic by the door (test 14's twin). A placement graph that forms a cycle (A against B, B against A) is a read cycle and refuses at the door, which replaces A11 (4)'s "no cycle is ever solved" with something stronger.
- **F's dependency on stage 4.** If stage 4 retires A5's hard error late, F waits, and from C until then a contact a declaring mate used to declare is unattributed at the at-rest gate (PRs 4325 and 4326).
- **E moves goldens on purpose.** Every assembly digest, the STEP bytes and the gallery frames move. The test is test 15's mapping, not bit equality. A reviewer must check that each moved digest moved by its world map and nothing else.
- **Migration reproducing the tree** (C). Today's tree is "the first member pair per instance pair in `Member` key order" (`mate/solve.rs:2269`). The migration must orient mates by that exact rule, or test 6 fails on the documents where two orders differ.
- **The member walk after stage 2 F.** B and C assume F leaves the walk starting from a select's body read. If F instead deletes the walk for face reads, B's "pose read on a placed member" is simpler, and the reconciliation is at F's merge.
- **Load.** Six units (H, H, H, M, M, M: 22.5 points by the work README's weights) are filed `parked` behind stage 2, so they do not count until it closes. At stage 2's close the orchestrator splits stage 3 into its own program or confirms it fits.

## 11. Open questions

### FORKs (each a design fork; a designer pair weighs it)

**FORK-S3-1 — What a free pose is.** *Changes ratified text (D10's Variables); a designer pair, then an `[ev]` PR.*

- **The problem.** D10 says the poses "may be free or defined" and that a free variable is "a value, its written unit (D6) and optionally a distribution". For a pose, a value is coordinates, and coordinates in what? Ev: "parts don't sit at (0,0,0) in their own space; they just don't have a location". A free pose with a value is an absolute datum respelled, which is the thing this stage retires.
- **Options** (final states):
  - **(a) A free pose is a seed** (recommended). It has no value: it is its space's own frame. Every other pose is a definition over a seed, by coordinates `InFrame`, by `Offset`, by projection or by combination. Tolerancing a datum's position is tolerancing those coordinates' scalars.
    - D10's free-variable sentence gains "a free pose has no value: it is a space's frame".
    - Two seeds are two spaces, so a part can hold unrelated workbench spaces, and a boolean across them refuses (D10's Booleans, already ratified).
  - **(b) A free pose holds coordinates in an implicit per-document origin**, with a distribution over its tangent. This keeps D10's sentence. But it re-creates a canonical origin per document ("there is no canonical main space", Ev), the computing-frame rule becomes trivially "the document origin", and a pose distribution needs a tangent-space vocabulary nothing else uses.
  - **(c) No free pose.** One undeletable document frame, like the world, and everything defined from it. This loses (a)'s multiple workbench spaces in a part. D10's "may be free" is then false and is deleted.
- **Recommendation: (a)**, *likely*. It is the only option where "a part has no location" is a property of the types.

**FORK-S3-2 — What a placement is in the document.** *Changes ratified text (A10's `PlaceInWorld` sentence, #4220, Ev-approved); a designer pair, then an `[ev]` PR.*

- **The problem.** D10 says "a **placement** is the bundle of mates that pins one copy … relative to others". A10 (#4220) says "a world placement (`PlaceInWorld`) is an operation reading one `Body` and defining its copy". Today's mates are symmetric, and the solve picks which side moves by a spanning tree (A11 (3), (4)), which D10 retires.
- **Options:**
  - **(a) `Place { body, mates }`** (recommended). The placement node owns its bundle and defines the copy. Each mate's placed side is the copy. The world placement is a `Place` whose bundle reaches the world. Deleting a placement deletes its bundle, and a mate cannot outlive what it places.
  - **(b) Directed mate nodes.** `Mate { places: copy, a, b }` stays a node, and a copy's bundle is derived as the mates naming it. The bundle is one source of truth, but a copy is defined by a `Place { body }` with no content, and a mate can be stranded from its copy.
  - **(c) Symmetric mates with a derived tree.** This is today's model. D10 retires A11 (3) and (4), so it is listed only to reject it.
- **Recommendation: (a)**, *likely*. It is D10's sentence made a type. `PlaceInWorld { body, pose }` becomes the one-mate case, `Place { body, mates: [Frame(seed) = Offset { World, pose }] }`.

**FORK-S3-3 — What a mate reads.** *A3 retirement is D10-listed; the replacement is design; a designer pair. It goes to Ev only if A11 (1)'s substance moves.*

- **The problem.** After stage 2 F, a side is a `Face` read plus a `MateFrame` offset, and the mate carries a `MatePrimitive`, a sense, and two raw floats (`PlanarRest.offset`, the clocking rider). Poses as variables (A) give a mate something to equate directly.
- **Options:**
  - **(a) Two pose variables of one kind** (recommended). The kinds are the primitive, every number lives in a pose definition, and a discrete `Sense` is a slot. `MatePrimitive`, `MateFrame`, `FrameBase`, the rider and the table gap are deleted, so a primitive the table has no row for cannot be written.
  - **(b) Face reads plus offsets, with the floats made slots.** This is the smallest change. It keeps two vocabularies for one pose, a mate's frame and a datum's, and a face base that only a mate can use.
- **Recommendation: (a)**, *sure*. Ev's #4222 comment ("the mates' `Subgroup` stuff can literally be shared") is this.

**FORK-S3-4 — What overconstraint means.** *Changes A11 (1)'s outcome list; a designer pair, then `[ev]`.*

- **The problem.** D10: "a mate added to a pinned copy refuses as an overconstraint, decided by subgroup algebra (A11 (1)) without measuring". It does not say what happens to a mate that is redundant on a copy that is *not* pinned: a second `Plane` mate on a parallel face, which pins nothing new. Today `intersect` re-measures it (`mate/coset.rs:1039–1046`) and admits it when its offset agrees. That re-measure is the shape Ev rejected ("constraints that fall back to being assertions").
- **Options:**
  - **(a) Pinned only.** D10's literal text. Redundancy on an unpinned copy is admitted when consistent and refused `Contradictory` otherwise, so a mate can still check.
  - **(b) A mate that lowers no dimension refuses** (recommended). The test is the held family's dimension against the fold's, read off the table's family arms. The pinned case is the instance where no mate can lower it.
    - The parallel verdict that decided `Planar ∩ Planar` is already a decided predicate of the fold, not a measurement of consistency.
    - `Contradictory` stays for a lowering mate whose cosets do not meet.
  - **(c) Redundant-but-consistent admitted** (Ev's "slick" DOF-counting). It needs the consistency measure that (b) deletes. Ev deferred it ("won't fight over it").
- **Recommendation: (b)**, *likely*. It is the general form, with (a) as its special case, and nothing checks.

**FORK-S3-5 — What a placement copies when its body is one of several related bodies.** *Touches FORK-1's instance signature (#4222); a designer pair, then `[ev]`.*

- **The problem.** An instance of a multi-body part defines "one `Body` variable per world placement of the part" (FORK-1). A `Place` reads one `Body`. Placing each output separately loses their relative poses, and the part's world frame, which relates them, is the inner world, whose coordinates D10 says nothing but export reads.
- **Options:**
  - **(a) `Place` reads a `Body` or a `Bodies`, and an instance adds a `world: Bodies` port** (recommended). The instance's copies are one space in the outer document, related by the part's own relations. No part's world frame is readable by a parent, so there is no `frame` port (PR 4326).
  - **(b) An instance defines one multi-solid `Body`** (A2's "its evaluation is one kernel `Body`"). FORK-1's per-placement ports go.
  - **(c) Each output is placed by its own bundle**, and the part's internal relations are re-stated as mates in the outer document.
- **Recommendation: (a)**, *likely*. It keeps FORK-1's ports and adds one.

**FORK-S3-6 — What a "member" of a space is, for the computing frame.** *A clarification of D10's "the frame of its earliest member" (`docs/prompts/designer.md` §2); a designer pair. It goes to Ev only if the text changes.*

- **The problem.** D10 defines a space as "a set of copies related to one another" and computes it "in the frame of its earliest member". A part's construction space has no copies, only a seed. And the FaceFrame's zero spin (DM1) depends on the coordinates a body is computed in, so a body that changed its computing frame whenever its space merged with another would turn the sketches on its faces.
- **Options:**
  - **(a) Members are seeds and copies** (recommended). A body computes in its construction seed's frame, a copy is a rigid image, and a space of copies runs its cross-copy work in its earliest copy's frame. D10's sentence gains "a space's members are its seeds and copies".
  - **(b) Members are copies only**, and a part's bodies compute at its single seed by fiat. This is the same behaviour, with the seed unnamed in the rule.
  - **(c) One computing frame per merged space for everything in it.** It moves construction geometry whenever spaces merge, against D9's "an unrelated edit moves no bit".
- **Recommendation: (a)**, *sure* on rejecting (c).

### Questions with a recommendation (not forks)

1. **Migration of absolute datums.** **Recommendation:** each becomes `InFrame { seed }` over its existing scalar ids. No new variable is minted, and tokens are unmoved.
2. **Non-root member offsets.** They were checks (A11 (2)) and are dropped at C's migration, with the report naming each. **Recommendation:** do not convert them to anything. Stage 5's `Assert` is where a person says it again, if they meant it.
3. **The gauge's name.** **Recommendation:** a gauge becomes a named `Frame` variable when it had a label, and anonymous otherwise. Its readers are the bundle mates that read it, so VR7's one-reader rule needs a name exactly when two copies read one gauge, which is the turntable's case.
4. **Which combinations A builds.** **Recommendation:** the projections, `InFrame` for all five kinds, `Offset`, and `Through { axis, point }` (FORK-1b's example). The rest wait for a reader. `Direction` gets its first reader in `PatternKind::Linear`.
5. **`Point` and `Direction` subgroups.** **Recommendation:** none in this stage. A ball mate (`Point`–`Point`) and a parallel mate (`Direction`–`Direction`) add `Subgroup` arms when someone authors one. `KindsDiffer`'s sibling `NoMateForKind` refuses them until then.
6. **`Coaxial` + `Clocking`** (today's `Prismatic` residual). A clocked coaxial pins rotation and frees slide. **Recommendation:** after B it is an `Axis`–`Axis` mate plus a `Plane`–`Plane` mate through the axis (the clocking plane). B's migration emits both, and test 7 checks the pose.
7. **Where the door reads the held fold** (F). **Recommendation:** fold the bundle at the door from the definitions. It is cheap (a handful of table rows), and it keeps "the doors decide edits" free of a stored solve.
8. **The viewer and the world.** "Export reads its coordinates and nothing else does." **Recommendation:** the viewer does not read `world_of`. It draws every space from display state of its own that no logic reads, the rule G3's free-move probe already follows, and E's grep gate admits no display door.
9. **`Transform`'s Python spelling.** **Recommendation:** `body.transform(by)` becomes façade sugar for `doc.place(body, at=Offset(seed, by))`, with one semantics in Python and Rust (Ev on #4220).

### Boundaries with the neighbouring stages

- **Stage 2.**
  - A waits on E (`FaceFrame` reads a `Face` variable).
  - B waits on F (mate sides read `Face` variables).
  - C retires `PlaceInWorld`, which stage 2 C builds. Building it was right: stage 2 had no world frame to mate against, and Ev called the state transient (#4220).
  - Stage 2 F keeps `reading_edges`' gauge edges (its Q3), and C deletes them with gauges.
- **Stage 4** (`intent/stage4-spec`, reconciled against its draft).
  - Stage 4 owns `ContactClass` on mates, the declared seats, the undeclared refusals and A5's hard error, in its unit I (`mates-declare-no-contact`). This spec leaves `Mate.class` in place for it.
  - **F waits on stage 4's I.** I retires A5's hard error on an unattributed contact. Before that, refusing an over-pinning mate would leave its contact undeclared.
  - **I waits on stage 4's H** (`placed-carriers-compare-through-their-frames`), and H needs a copy's frame defined from its bundle. That is this stage's **C**, not the whole stage.
  - So H's `blocked_on` names `a-placement-is-the-bundle-of-mates`, not `intent-stage3-is-built`. Otherwise the two stages wait on each other: the umbrella parks on F, F on I, I on H, and H on the umbrella. The order is then C → H → I → F.
  - Ruled by the orchestrator on this PR: H's `blocked_on` names C, and F's names `mates-declare-no-contact`.
  - **Stage 4's FORK-S4-5** (a mate-placed face is structural because the placed copy's frame is *defined as* its partner's frame composed with the mate's offsets) assumes the shape C builds. A copy's pose is derived from its bundle (`OfCopy`), never stored as a free value, which is the case stage 4's risks name as fatal.
  - Stage 4's canonical forms compare poses "modulo the kind's own symmetry", the same `Subgroup` this stage shares (A).
- **Stage 5** (`intent/stage5-spec`).
  - Its unit C (`the-at-rest-census-is-a-check`) checks per space, "in whatever shape stage 3 leaves", and names `mate-offset-verified-…` as its stage-3 trigger. That row closes at F, so stage 5 C's trigger is F (`a-mate-on-a-pinned-copy-refuses`), as its row now says.
  - Its "copy" is "a `Body` output of a world placement; stage 3 respells the placement, not the output". This spec keeps that: a copy is `Place`'s one `Body` output.
  - `AssemblyError::Space` is C's to re-derive from bundles.
- **Stage 6** owns coaxiality through one axis variable and the tangency constructions. A builds the `Axis` variables they read.

### Rows released by this stage (the 2026-10-08 re-homing, `work/intent/log.md`)

| Row | Unit | Disposition |
|---|---|---|
| `msolve/a-box-independent-mate-fault-bisects-the-whole-leaf-budget` | F | opened: its vocabulary is F's `Overconstrained` and C's `Under` |
| `msolve/a-box-over-a-solved-clocking-widens-thirty-thousandfold` | F | closed: the re-measure is gone |
| `msolve/a-clocking-rider-is-levered-unreduced` | B | closed: the rider is gone |
| `msolve/a-declaring-mates-alignment-is-never-read` | C | closed: declaring by gauge is gone |
| `msolve/a-face-base-puts-its-reference-on-local-y` | B | closed: the convention is DM1's, stated once |
| `msolve/a-face-frame-cannot-turn-its-roll` | B | closed: a roll is an `Offset` angle |
| `msolve/a-far-meeting-point-fails-membership-by-its-own-rounding` | F (re-read after E) | closed if F's door refuses it first, else opened on the lowering arm |
| `msolve/a-mate-frame-axis-is-decided-against-a-length-band` | B | closed: no re-mint |
| `msolve/a-mate-frame-is-written-in-the-reading-instances-coordinates` | B, C | closed at C |
| `msolve/an-identically-zero-margin-escalates-at-a-fine-eps` | F | closed |
| `msolve/mate-primitive-unit-variants-load-from-a-null-payload` | B | closed: `MatePrimitive` is gone |
| `recipe/placement-step-slots-are-spelled-three-ways` | C (with D) | closed: one spelling |
| `topo/a-boxed-rotation-refuses-not-rigid-at-every-placer` | C, D | placement half closed; re-opened on the boxed `Pattern` angle |

Also in scope, outside the umbrella:

- `explicit-placement-frames-hold-floats` closes at A.
- `mate-offset-verified-…` closes at F.
- `msolve/gauge-of-recomputes-the-clusters-per-placement-lookup` (open, stale) closes at C.
- `msolve/a-placer-row-states-what-a-poisoned-row-cannot` (parked on stage 2 F) closes at C.
- `msolve/the-mate-solve-reads-the-platform-atan2` (open) is untouched: `candidate_rotation` and `clocking_about` survive B. It stays workable.

## 12. Inconsistencies found

- **ASSEMBLY A11 (4)** says "an inconsistent loop dies at its closing mate's verification (`MateFault::Contradictory`)". No code verifies a declaring mate (`mate/solve.rs:2147–2151`, `:2263–2268`). `Contradictory` arises only inside one pair's fold or from a rider. A loop-closing mate is minted as a contact and never refused. C and F delete the clause, and until then it overstates.
- **`mate/coset.rs`'s module doc and A11 (1)** call the fold "exact coset intersection". Every branch is a decided predicate over measured values (`k_stats::decide`, levered by `Arm`).
- **REFERENCES DM1** says "A11 keeps the solve over authored numbers", but A11 (5) reads face poses off the part's geometry.
- **`work/msolve/gauge-of-recomputes-the-clusters-per-placement-lookup`** cites `mate::gauge_of` and `Doc::placement`. Neither exists in src: `gauge_of` is a test helper, and the door is `SolvedPoses::placement`.
- **`work/intent/explicit-placement-frames-hold-floats`** says "the die's twenty-one pip frames". `die_tool.pncad` holds six `Explicit` frames (line 372).
- **`VarKind::symmetry`** (`var.rs:102`) has no callers in src. The solve reads `PoseSymmetry` on `DatumValue`, so the "one `Subgroup`" sharing #4222 approved is two parallel tables kept in step by a test (`intent_s2_a_outputs.rs:552`, `:589`). A folds them.
