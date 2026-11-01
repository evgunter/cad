# SPEC: INTENT stage 6 — tangency constructions

Scope: `work/intent/plan.md` stage 6, "Tangency constructions. A surface's trace in a sketch plane and continuation tangent to it; coaxiality through one axis variable." The umbrella is `intent-stage6-is-built`.

**What D10 says stage 6 builds** (`docs/DESIGN.md`; D10's text now lives in the bracketed clauses its old-numbering row lists):

- `[coincidence]`, the paragraph **Constructing a coincidence** (it carries no clause id of its own): "Coaxiality is one `Axis` read twice, directly or as projections of one construction; tangency is constructed (a sketch may read another surface's trace in its plane and continue tangent to it)."
- `[intent.statements]`: "that two things coincide is said either by constructing them as one, which proves it, or by asserting a bound, which claims it and is checked".
- `[intent.checked]`: the `unproven-coincidence` lint "reports one that holds only at the current values together with the edit that would make it one construction".
- `[coincidence.structural]`: two cells are structurally one by "the same construction read twice, directly or as projections of one construction" (rung one), or where the kernel's carrier-pair verdict, or the deciding margin, run on the symbolic carriers "is zero as a polynomial" (rungs two and three). "'Structurally' is decided at that door and nowhere else, and a rung added there may only turn a finding quiet."
- State of the build, **Not built**: "tangency constructions (stage 6)".

**The constraints the ratified text puts on it** (each binds a fork in §7):

- `[variables.kinds]`: "A `Profile` is 2-D shape whose numbers are read only against each other".
- `[variables.poses]`: "no pose is free, none is defined from nothing, and no construction reads one". "An incidence between poses (an axis in a plane, a point on an axis) is a construction over one variable, never a check between two."
- `[variables.poses]`: "A 2-D value … lives in the node that holds its frame, as length and angle slots, and is never a variable of a 2-D kind".
- `[operations]`, **Observed values** (fork log row 79, Ev 2026-10-07): "An observed variable, and any definition reading one, is read only by an assertion: a construction reads what was written." Row 79 names stage 6's tangency and "up to" as the intensional spellings that would replace a driven value.
- `[coincidence.door]`: "A per-construction table of output forms is only ever a cache generated from the symbolic evaluation and checked against it, never written by hand."

**Ev's words** (2026-10-03, the transcript behind ruling `one-way-to-say-dependency-and-intent`, `5f7a1c71e3`):

- "all the wacky declared tangencies are replaced by good ways to define parts to make them tangent by construction. e.g. if you want to put a cone on a sphere you should be able to say, like, "a cone {with a base with this radius and pointing in this direction, with a base on this plane, with height h and pointing in this direction} x {on this sphere tangent at the base, on this coaxial torus tangent at the base (inner branch), on this coaxial torus tangent at the base (outer branch)}". more concretely this probably looks like setting a slope in the sketch plane based on a derivative of a shape as it appears sliced by the sketch plane, which is far more general as well."
- "you can use it to define something that's tangent in only one place. i think coaxiality by using the same axis object covers the normal cases".
- The assistant's reply in that conversation, which Ev did not contest: slice tangency implies surface tangency only when the generator preserves the surface's structure.

**Line numbers** cite main at `0cc1eaf9` beside the name and are allowed to drift.

## 0. The state on main

### What earlier stages built that this stage stands on

- **The door and its three rungs** (stage 4 B–D; `crates/editor-core/src/coincide.rs`). `prove` (:619) tries `first_rung` (:684), then the replay at `Sym` (`coincide/replay.rs`): `Rung::CarrierVerdict` and `Rung::MarginIdentity`. Every row a kernel operation decides from values carries a `topo::Relation` (`crates/topo/src/coincidence.rs` :77) and a `DecisionSite` (:165).
- **Rung one is provenance.** `one_construction` (:725) holds only where every cell walks (`construction`, :1059) to one `Origin` through one chain of maps. `Origin` is a minted entity, a plane pose, or a profile piece (:330). Two distinct faces never share an `Origin`, so rung one never proves a `Coaxial` or `SurfaceTangent` row, whose cells are always two faces.
- **A mate is rung one's statement, for planes only.** `stated_by_a_mate` (:767) reads `mate::planes_equated` (`mate.rs` :144), which answers only for `Plane { face }` under flips. It proves `SameOpposite`, `SameOriented` and `OnCarrier` rows. A mate equating two `Axis`, `Pivot` or `Frame` poses states nothing at the door.
- **Rung two answers only one-carrier relations.** `carrier_verdict` (`replay.rs` :366) maps `SameOriented` and `SameOpposite` and returns `None` for every other relation, so a `Coaxial`, `SurfaceTangent` or `Tangent` row reaches only rung three.
- **Rung three** discharges the deciding margin's polynomial form within `BUDGET` (`replay.rs` :63, 256 terms). It proves the blend battery's coaxial supports and some of its joints ("a filleted disc's two coaxial supports prove here and its two joints only from 1024 terms"). A margin computed by sampling, such as the tangent witness lane's (below), is expected to keep values computed apart from the variables, which the tier does not discharge (`Undischarged::KeepsAnOpaqueAtom`); A measures it.
- **Poses** (stage 3 A, A′; `crates/editor-core/src/pose.rs`, `PoseDef` :242). `Axis { of }` reads a carrier's axis off a face or edge; `Slide`, `Spin`, `Through`, `Meet` and the projection roles move and combine poses. `Node::Revolve` (`node.rs` :1496) defines an `axis: Axis` output beside its body and space (`outputs`, :2638). No construction reads a pose (stage 3 D): `Extrude { profile, distance }`, `Revolve { profile, axis_through, axis_heading, angle }`, `Tube`, `HollowTube`, `Loft`, `Sweep`.
- **Mates** (stage 3 C; `mate.rs` module docs): "`Axis`–`Axis` is coaxial". A placement's bundle may equate a copy's axis with any `Axis` variable of the target space, including one another placement also equates. So "one `Axis` read twice" is already writable; nothing reads it at the door.
- **A profile is 2-D and reads nothing** (stage 3 D; `ProfileProgram`, `program.rs` :427, "A profile reads no plane"). Its tangency vocabulary is internal: `ProgramStep::Tangent`, `Cusp`, `TangentArcTo`, `ContinueTo`, `ArcOnto`, the fillets and `ProgramTarget::StartArriving` (`program.rs` :224, :182). Each makes a joint between two of the profile's own pieces.
- **Profile junctions** (stage 4 G, `tangent-joints-are-derived`, closed). The tangent-joint set is derived at lowering (`crates/profile/README.md`, "The profile loop: format and tangency"). A junction decided Zero that no constructor made is a `ProfileJunction` row over two pieces (`coincide::name_junctions`, :1354); "each piece is its own construction, so no rung of today's proves one" (`coincide.rs` module docs).
- **Booleans glue a tangency on Zero** (stage 4 E). The glue door's tangency (`verify_tangency`, `topo/src/boolean/mod.rs` :7516) records a `SurfaceTangent` row at `TangentWitness` (`tangent_row`, :7019). Its margin is "the pair's separation along the shared normal, its worst sample plus the span's sag" (`DecisionSite::TangentWitness`'s doc), a sampled measurement. The closed-form locus lane is plane × cylinder and cylinder × cylinder only (`geom_brep::tangent_locus_kinds`, `locus.rs` :156); every other kind pair is `TangentLocusError::Unsupported`.
- **Booleans record coaxiality on Zero.** The section value's coaxial classifications record rows (`glue.rs` `coincidence_site`, :279): `CylinderTorus` and `TorusTorus` as `Coaxial`, `CylinderSphere` and `SphereTorus` as a sphere's centre `OnCarrier` the axis (sites `CoaxialCylinderTorus`, `CoaxialTori`, `CoaxialSphere`, `SphereOnTorusAxis`).
- **The blend battery records its own** (`the-blends-g1-and-coaxiality-verdicts-are-unrecorded-coincidences`, closed): `BatteryJoint` (`Tangent`) and `BatterySupportAxis` (`Coaxial` or `CoRuled`).
- **At rest** the census records `SameOpposite` and `OnCarrier` only (`checks/at_rest.rs` `contact_cells`' table, :356). Two coaxial copies that do not touch record nothing; a pin pressed in a bore of one radius records a `SameOpposite` wall pair.
- **Quieting** (stage 5; ATREST). A `Gap = 0` says none of the tangent or coaxial rows (`contact_cells`' table), so they are quieted by no assertion: ATREST's open row `a-gap-over-adjacent-cells-states-nothing-of-the-row`. Stage 6 is the other half of that row's answer: it makes the common cases proven rather than quieted.

### What does not exist

- No node, pose or profile step reads another body's surface: there is no trace, no section of a face by a plane in 2-D coordinates, and no step that continues tangent to anything outside its own profile. The kernel's plane sections exist only inside the split (`topo::splitting::section::plane_section`, :296) and the census.
- No rung proves a `Coaxial` row from axes, and no rung proves a `SurfaceTangent` row.

### Measured on main

`coincidence_door::census_the_door_per_row_over_the_corpus` (release, 27 s) prints 299 rows over 13 of the corpus documents. Of the relations this stage is about, the corpus holds only the blend battery's: 44 `Tangent` rows at `BatteryJoint` (15 proven by `MarginIdentity`, 29 unproven `Numeric/Budget`) and one `CoRuled` at `BatterySupportAxis` (proven). It holds **no** `Coaxial`, `SurfaceTangent` or `ProfileJunction` row, and no at-rest row of a coaxial seat. The corpus's `tangency` document carries only constructed profile joints, which record nothing. So no instrument today shows what this stage changes; PR A supplies one.

## 1. Ordering

| PR | Unit (`work/intent/`) | Lands | Waits on | Cost |
|---|---|---|---|---|
| A | `tangent-and-coaxial-scenes-are-in-the-corpus` | the scenes the stage proves, in the corpus, each row's proof pinned as today's | nothing | M (~8 files, 0.8–1.2k lines, tests only) |
| B | `a-coaxiality-is-one-axis-read-twice` | the door proves a coaxial row whose two cells' axes are one `Axis` construction read twice | A; FORK-S6-1 | H (~15 files, 1.5–2.5k lines) |
| C | `a-sketch-continues-tangent-to-a-surfaces-trace` | the trace and the step that continues tangent to it: representation, evaluation, names, persistence, Python | FORK-S6-2, FORK-S6-3 | H (~45 files, 3–5k lines) |
| D | `the-door-proves-a-constructed-tangency` | the door proves the tangency row a C construction makes | A, C; FORK-S6-4 | M (~10 files, 1–1.5k lines); H if S6-4 takes a new rung |
| E | `the-sketch-tool-authors-a-tangent-continuation` | the viewer's sketch tool picks a face, shows its trace and writes C's step | C | M (~15 files, 1–2k lines) |

Why this order:

- **A first** because nothing else measures the stage: every later PR flips rows A pins. A is dispatchable now and needs no fork, so the corpus census is in hand when the designers weigh S6-1 and S6-4.
- **B is independent of C–E.** Coaxiality needs no new construction; D10 says it is "one `Axis` read twice", which stage 3 already lets a document write. B is the door reading it.
- **C before D.** D proves what C constructs. C is a whole representation without D: a trace-tangent step builds, and the row it makes is reported unproven exactly as a hand-typed tangency is today. If S6-4 resolves to "rung three proves it once C lowers in the shape's algebra", D folds into C and is closed with C (§5).
- **E after C.** The gesture writes C's step; it has nothing to write before.

Each intermediate state is a whole representation: after A only tests change; after B a coaxial seat is proven or reported with its recourse, one rule; after C a sketch either continues tangent to a trace or to its own piece, each by construction; after D the door proves both.

**Rejected:**

- **C and D in one PR.** D's shape is a fork of its own (S6-4) that C's representation does not depend on; landing them together makes C wait on it.
- **The trace as its own PR before the tangent step.** A trace no step reads is a variable with no reader (`[operations]`: "an unread body is an unread variable, a finding"), half of the representation.
- **E inside C.** C's surface is the document, the façade and Python, which test C; the viewer's sketch tool is another program's ground (VIEWER's) and another reviewer's.

## 2. PR A — `tangent-and-coaxial-scenes-are-in-the-corpus` (cost M; ~8 files, 0.8–1.2k lines)

Adds corpus documents (`crates/editor-core/tests/corpus/`), each authored through `DocEdit`s like the rest, and registers them in `documents()`:

1. **`pin_in_bore`.** A revolve's bore and a separately constructed pin (a revolve), the pin placed by one `Axis`–`Axis` mate, its shank wall's axis on the bore wall's, the spin it leaves `0` and its slide the housing's height, where the shoulder meets the top. A plane mate on the shoulder beside the axis mate states the axis's direction twice, which the mate door refuses as `Overconstrained`; the slide is measured from the target space's origin. Two radii: one a shared `Length` variable (press fit), one apart (clearance). Combined by `⊔`.
2. **`shaft_and_bearing`.** Two copies each mated by `Axis`–`Axis` to the housing revolve's `axis` output: one `Axis` variable read twice. Combined by `∪`, so the boolean's coaxial classifications record rows where a torus fillet and a cylinder meet.
3. **`axis_projection`.** One revolve whose torus and cylinder faces a later boolean decides coaxial: the cells' axes are projections of one construction (the revolve's).
4. **`boss_tangent_by_value`.** A boss extruded from a profile whose straight edge is typed tangent to a cylinder (its offset written equal to the radius), unioned with the cylinder: one `SurfaceTangent` row at `TangentWitness` (plane × cylinder, in the closed-form lane).
5. **`cone_on_sphere`.** Ev's example as numbers: a revolved cone whose generator is typed tangent to a sphere, unioned with the sphere. Where no glue arm exists for the pair, the document pins the typed refusal it meets today, so the scene says where C's construction needs the kernel to reach.

`coincidence_door.rs` gains one test per scene pinning each row's relation, site and proof. These are today's values: B and D flip them.

**Done when** (each red without A):

- the per-row census lists at least one `Coaxial` row at `CoaxialCylinderTorus` or `CoaxialTori`, one `OnCarrier` row at `CoaxialSphere` or `SphereOnTorusAxis`, the at-rest row between the press fit's walls (or the census's typed refusal there, pinned), and one `SurfaceTangent` row at `TangentWitness`, each naming its scene;
- each such row's proof is pinned as it is on main (expected: unproven, with its `Unreplayed` and `Recourse`);
- `cone_on_sphere` pins whichever of a row or a typed refusal it meets, by its kind;
- the corpus-wide rows (`m4_pr8_corpus` at every ε row and the `Interval` lane, the persistence round trip) are green with the scenes in.

## 3. PR B — `a-coaxiality-is-one-axis-read-twice` (cost H; ~15 files, 1.5–2.5k lines)

Builds FORK-S6-1's answer. In every option, the door reads a coaxial row structural where the two cells' axes are one `Axis` construction read twice, "directly or as projections of one construction":

- **Directly:** each cell's copy places its axis, by a mate of a pinning bundle, onto one `Axis` variable, or the two copies' axes are equated by one mate (`mate.rs` gains `axes_equated` beside `planes_equated`, over `Axis { of }`, a `Pivot`'s axis and a `Frame`'s `z` read through its projection role).
- **As projections of one construction:** both cells' carriers revolve about one construction's axis (two faces of one revolve or tube), read through one chain of maps.
- The relations it covers: `Coaxial`; the sphere-centre `OnCarrier` rows at `CoaxialSphere` and `SphereOnTorusAxis`, whose axis is the cylinder's or torus's and whose point is the sphere's centre; and `BatterySupportAxis`. `CoRuled` and `AxesMeet` are not coaxiality and stay as they are.
- A press-fit seat (`SameOpposite` between two walls of one radius) is two facts, one axis and one radius. B proves the axis half; whether the row is then structural is FORK-S6-1 (c).

**Done when** (each red without B):

- `shaft_and_bearing`'s coaxial rows are `Structural`, and the rung that proves them is the one S6-1 picked;
- `axis_projection`'s rows are `Structural` without the replay (a census of the rung, so a rung-three proof that happens to close does not pass it);
- the same scene with the second copy's mate moved to an axis read off a different construction (a parallel bore) reports the row `Unproven` with `Recourse::OneConstruction`;
- an `Axis`–`Axis` mate on a copy that another bundle moves states nothing (the plane rule's twin: "a copy another bundle moves holds the face where the mate does not read it");
- the corpus census moves only in the scenes' rows, each move named in the PR.

**Docs.** `coincide.rs`'s module docs (the mate paragraph says "two faces whose planes one mate … equates"; it gains axes); `[coincidence]`'s "Constructing a coincidence" sentence needs no edit.

## 4. PR C — `a-sketch-continues-tangent-to-a-surfaces-trace` (cost H; ~45 files, 3–5k lines)

Builds FORK-S6-2 and S6-3's answers. Whatever they decide, C carries:

- **The trace.** The section of a face's carrier by a plane, in that plane's 2-D coordinates. Its kinds are the closed forms the carriers give: a plane gives a line, a cylinder lines, a circle or an ellipse, a sphere a circle, a cone a conic, a torus a spiric curve. A kind C does not lower refuses typed, naming the carrier pair (`[one-way.readings]`, "An unlisted arm refuses typed"), never a fallback.
- **The step.** A profile step that starts on, or continues from, a point of the trace with the trace's tangent there as its heading (Ev: "setting a slope in the sketch plane based on a derivative of a shape as it appears sliced by the sketch plane"), with a discrete branch where more than one point answers (Ev: "inner branch", "outer branch"), held as a discrete variable (`[variables.kinds]`). It lowers in the shape's own algebra, as every profile mode does (`crates/profile/README.md`, "each arc mode's lowering is that mode's one conversion"), so the symbolic tier reads the trace's tangent through the authored data.
- **Its refusals.** A plane that misses the carrier, or meets it only at a point, refuses typed at evaluation (`[variables.poses]`: a combination "refuses its degenerate case"); a branch that does not exist at the current values refuses typed, naming it.
- **Its row.** The profile's own junctions are as stage 4 G left them. The tangency between the swept face and the traced face is the row a later boolean or the census records: `SurfaceTangent` at `TangentWitness` today. C changes no kernel decision.
- **The surfaces.** The document vocabulary (`ProgramStep` or what S6-2 names, with its persisted spelling pinned in `tests/switch_program_vocabulary.rs`); the slot table and its golden; the persist walk and the wire; names (a piece drawn by the new step is named like any step's, `names/README.md` N1); `pncad`'s façade and `pncad-py` (the census and the `.pyi`); `crates/profile/README.md`'s tangency paragraph; `[coincidence]`'s **Constructing a coincidence** sentence if S6-2 restates it.

**Done when** (each red without C):

- `boss_tangent_by_value` respelled with the step builds the same body within one rounding of the value-typed one, and moving the cylinder's radius variable moves the boss so the two stay tangent at every value of a box run (the margin's enclosure contains zero over the box);
- the same with the radius typed by value leaves the boss where it was;
- `cone_on_sphere` respelled with the step builds, or refuses with the typed kernel refusal A pinned (and that refusal is filed on its owner's slate with the scene);
- each typed refusal above fires at the scalar that decides it, with the step and the face named;
- `switch_program_vocabulary`, the slot-table golden, the Python census and `.pyi` name the new spelling, and a pre-C file loads unchanged.

## 5. PR D — `the-door-proves-a-constructed-tangency` (cost M; ~10 files, 1–1.5k lines)

Builds FORK-S6-4's answer: the tangency row between a face a C step drew and the face it traced is `Structural`.

- If S6-4 (a): the glue door's tangency records a margin whose form the symbolic tier reads (the closed-form lane's residual, not the sampled witness), and rung three discharges it for C's construction. No rung is added.
- If S6-4 (b): rung one gains the statement that a swept face drawn tangent to a trace is tangent to the traced face where the sweep preserves the tangency (§7), the analogue of `stated_by_a_mate`.
- If S6-4 (c): D folds into C's done-when and is closed with C.

**Done when** (each red without D):

- `boss_tangent_by_value` respelled with C's step reports its `SurfaceTangent` row `Structural`, and the value-typed original still reports it `Unproven` with `Recourse::OneConstruction`;
- a C step on a sphere's small-circle trace, extruded normal to the plane, is not reported tangent at all: the extrude cuts the sphere, and no rung proves a tangency there (the slice-tangency caveat);
- the census of the corpus moves only in the scenes' rows.

## 6. PR E — `the-sketch-tool-authors-a-tangent-continuation` (cost M; ~15 files, 1–2k lines)

The viewer's sketch tool (`crates/viewer/src/sketch.rs`, `pane/profile.rs`): with the sketch's plane chosen, a pick on a face draws its trace in the sketch, and a gesture writes C's step, offering the branches. The trace is display only: it is drawn from the evaluation, and nothing the viewer computes is written to the document. VIEWER's ground; announced on its log.

**Done when** (each red without E): a viewer test drives the gesture on `boss_tangent_by_value`'s cylinder and the document holds C's step, not a heading typed from the drawn trace; an edit of the radius after it moves the boss; a pick on a face the sketch plane misses shows the typed refusal in the pane.

## 7. Open questions

### FORKs, for the designer pair

**FORK-S6-1 — How the door knows two cells' axes are one construction.**

- **The problem.** "Coaxiality is one `Axis` read twice, directly or as projections of one construction". Rung one reads a cell's `Origin`, which is the cell itself; nothing reads a face's carrier axis as a construction. And a revolve's two faces share its axis only because its code computes them about it.
- **Options:**
  - (a) **Rung one reads an axis's provenance.** A cell's axis walks to the construction that made it: a revolve's or tube's faces to its `axis`, a copy's to the `Axis` variable its bundle's mate equates it with. Which faces of a construction revolve about its axis has to be stated per construction; D10 allows that only as "a cache generated from the symbolic evaluation and checked against it", which COINCIDE's parked `a-construction-s-identities-are-a-generated-cache` would build.
  - (b) **Rung two answers coaxiality.** `carrier_verdict` gains the `Coaxial` relation: the kernel's own axis verdict, run on the two symbolic carriers, is zero as a polynomial. One axis variable read twice gives one symbol, so it proves without any per-construction table. It needs the replay to fold a placement's axis mate into a polynomial form; whether it does is unmeasured (stage 3's fold reads directions through normalizations the tier may hold as opaque atoms).
  - (c) **Mates only.** The door states coaxiality only where a mate of a pinning bundle equates the two cells' own axes (the plane rule's twin), and leaves the rest to rung three. Smallest, and it misses a fillet's torus on a mated cylinder.
- **And the seat.** A press fit is "one axis" and "one radius". Whether the seat's `SameOpposite` row is structural when the axis is stated by a mate and the radius is one variable read twice is the same choice: (a) and (c) need a rung that combines two statements, (b) gets it from the carrier verdict if the fold is polynomial.
- **Weighed with** stage 4's FORK-S4-2 (how a construction states its carriers), which chose a stated, witnessed form; and Ev's 2026-10-06 rule that a per-node theorem is "kept only as a possible cache … added only where the algebra is measured too slow or undecidable". A's census is the measurement (b) needs.

**FORK-S6-2 — What a sketch reads, given that a profile reads no plane.**

- **The problem.** A trace is where a face's carrier meets the sketch's plane, so it depends on where the plane stands relative to the face. Stage 3 made a profile 2-D, reading no plane, its construction placed afterwards; no construction reads a pose; a 2-D value is never a variable of a 2-D kind; and a construction reads no observed value (row 79). The ratified sentence "a sketch may read another surface's trace in its plane" predates stage 3's reading of these and does not say how it reconciles with them. Each answer changes ratified text.
- **Options:**
  - (a) **A `Trace` operation defines a `Profile`.** `Trace { face, frame }` reads a face and a `Frame` in the face's space and defines a `Profile` (a 2-D shape in the frame's `xy`; "Only an operation defines a shape"). A profile step reads that `Profile` as what it continues tangent to. The tangency holds in space only where the construction over the sketch is placed by a mate equating its space with the same `frame`, which is a placement the author writes; otherwise the row is unproven, and the lint says so. Changes: a profile may read another `Profile` variable (`[variables.kinds]`).
  - (b) **A sketch on a face is a node that holds its frame.** The sketch reads `frame` and the face, holds the trace's 2-D values as its own slots (`[variables.poses]`' "lives in the node that holds its frame"), and its construction is born in the face's space rather than its own. Changes: FORK-S3P's "every body is built in coordinates of its own" and `[variables.kinds]`' 2-D profile.
  - (c) **The trace's numbers are defined variables.** A trace operation defines `Length` and `Angle` variables (a point's coordinates and the heading there), as a function of the face's construction and the frame, and an ordinary step reads them. Changes row 79: these are functions of built geometry, which row 79 calls observed.
  - (d) **Tangency is a 3-D construction, not a sketch's.** Ev's cone example read literally: a construction reads the face it rests on ("on this sphere tangent at the base"), as stage 6's "up to" would. Changes "no construction reads [a pose]", and the ratified sentence's "a sketch".
- **Weighed with** row 79's residual ("the imported-body / off-site case (no construction spelling until AQ4 and stage 6's "up to" land)"), which an answer here may settle.

**FORK-S6-3 — Which point of the trace a step continues from.**

- **The problem.** A heading needs a point of the trace, and that point has to be on the trace by construction (`[variables.poses]`: "An incidence … is a construction over one variable, never a check between two").
- **Options:** (a) the trace's point at a written parameter along it (an arc length or an angle about its centre, per kind); (b) where a line of written heading through the pen meets the trace, with a discrete branch; (c) the point where a line from the pen touches the trace (a tangent from an external point, Ev's cone generator), with a discrete branch; (d) more than one of these, as modes of one step, the way arcs have modes.
- It turns on S6-2: under (c) of S6-2 it is a choice of which numbers the trace defines.

**FORK-S6-4 — What makes a constructed tangency structural at the door.**

- **The problem.** Tangency of the traces in the sketch's plane is tangency of the surfaces only where the sweep keeps the surface's structure: an extrude normal to the plane of a cylinder's straight trace is tangent along a line, but of a sphere's small-circle trace it cuts the sphere. The row the door holds is `SurfaceTangent` between two faces, decided by a sampled margin (`TangentWitness`) that rung three cannot read.
- **Options:**
  - (a) **The margin becomes readable.** The glue door's tangency records, for the closed-form lane, its exact residual rather than the sampled one, and rung three proves it for a C construction. One door, no new rung; pairs outside the closed-form lane stay unproven.
  - (b) **A first-rung statement.** The door reads a C step and the sweep over it, and states the tangency where the sweep direction lies in the traced face's tangent plane along the trace, decided from the recipe's structure. A second description of the operation, which D10 allows only as a generated cache.
  - (c) **Nothing new.** C lowers in the shape's algebra, and whatever rung three proves is what is proven; the sampled-margin rows stay unproven with their recourse.

### Questions with a recommendation (not forks)

1. **Which kinds the trace lowers first.** **Recommendation:** the plane, cylinder, sphere and cone traces (line, circle, ellipse, conic), and a torus's spiric trace refusing typed until the profile carries a carrier for it. The profile's carriers are lines and arcs (`crates/profile/README.md`, "Profile format"), and a step needs only a point and a heading of the trace, not the trace as a segment.
2. **Who quiets a tangency no construction makes.** **Recommendation:** ATREST's `a-gap-over-adjacent-cells-states-nothing-of-the-row`, unchanged by this stage. Stage 6 shrinks what reaches it.
3. **The umbrella.** `d10-one-way-to-say-intent-is-unbuilt` is parked on `intent-stage3-is-built` only. Stage 6 is the last stage; the orchestrator decides whether the hold also waits on `intent-stage6-is-built` or on the exit walk.

### Inconsistencies found

- `docs/DESIGN.md` State of the build still lists "the rest of placement (stage 3 …)" under **Not built**, and stage 3 closed with G (#5954).
- `docs/GERM-CONE-SECTOR-SPEC.md` and `docs/GERM-TORUS-PLANE-FRAME-SPEC.md` hold `sectors::tangent_lump` as running "only for a declared-`Tangent` pair (`BooleanCoincidence::TANGENT`)". `BooleanCoincidence` is gone (stage 4 F), and `tangent_lump` is called from `boolean/recl.rs`. Those are GERM's specs, cited here only because they name stage 6 as the seat's retirement.
- The **Constructing a coincidence** paragraph of `[coincidence]` has no clause id, so code cannot cite it the way it cites the rest of D10.
- §2's first scene as first written, an `Axis`–`Axis` mate and a plane mate on the shoulder in one bundle, is refused as `Overconstrained`: the two state the axis's direction twice. PR A seats the pin by the axis mate and its slide.
- The at-rest census records no row for a press fit's walls, nor for the shoulders' rests: the press fit's census holds `CensusUndecidable` ("cannot yet tell whether curved faces of two parts touch") and unrecorded contacts, so the press fit's at-rest row this section expects does not exist on main.

## 8. What retires

Nothing D10's retirement list names is left for this stage: the profiles' stored tangent-joint flags and the undeclared-tangency refusal retired in stage 4 G. What this stage changes is findings, not mechanisms: the coaxial and tangent rows a construction makes stop reaching the lint. When E merges, State of the build's **Not built** line "tangency constructions (stage 6)" moves to **Built**, and the program's exit walk follows (`work/intent/plan.md`, "Exit criteria").

## 9. Risks

- **The corpus measures nothing today** (§0). A is the instrument; B and D are sized on what A shows.
- **The fold at `Sym`** (S6-1 (b), S6-4 (a)). Whether a placement's mate folds into a polynomial form at the door's budget is unmeasured. A's scenes measure it before the designers rank.
- **Glue arms the construction needs** (C). A tangency the boolean has no closed-form arm for (sphere × cone) refuses typed today. C makes it easy to author; the refusal then belongs to CONTACTHOLD or GERM, and C files it with A's scene rather than building the arm.
- **Shared ground.** `coincide.rs` and `coincide/*` are COINCIDE's; the census and `checks/at_rest.rs` ATREST's; `mate.rs` MSOLVE's; the profile crate PATHS'; the viewer VIEWER's. B, C, D and E each announce on the owning program's log.
- **Load.** A (M), B (H), C (H), D (M) and E (M) are 25 points by the work README's weights. B, D and E are parked, so A and C are what the board charges: the track goes from 32 to 43 of 45. B, C and D carry `design: true` until their forks are decided.
