# FORK-VTX: how a measure (or anything) reads a vertex

## For Ev

**Recommendation (sure): a vertex is a cell, so `Vertex` is a selection kind
beside `Face` and `Edge`.** A measure reads the `Vertex` selection, as it
reads a `Face` or `Edge`. A mate, or any pose reader, reads the pose
`Point { of: v }` defined off that selection, exactly as a face is read as a
plane. No `Vertices` set until a slot takes one. Options B, C and D as posed
are rejected below; this is the brief's (A) plus the pose half of (D), and
the two are not rivals.

**Premise check.** The fork treats "what a vertex reference becomes" as open.
It is not: a vertex is already a first-class cell everywhere but D10's kind
list. N1 names it (`EntityKind::Vertex`, six role segments: `CapVertex`,
`Pole`, `MeridianVertex`, `CrossingVertex`, `OnToolVertex`, `FootVertex`),
and edge names *cite* vertex names (`Qualifier::Ends`); the viewer hit-tests
it; Python lists it (`all_vertices`); `distance` has two arms for it. D10's
first text (#3990) listed the kinds as "(`Face`, `Edge`, `Body`)" and every
later sentence copied that list; FORK-3's "read one `Face`/`Edge`" copied it
again. Ev's own words in the D10 conversation never mention cells or kinds.
So the omission is agent text approved in passing, never a decision, and the
only real design content is the second half: does a measure read the cell or
a pose, and what does a mate read.

**Terms.** A *cell* is a face, edge or vertex of a built body, named by
`StableName` and resolved by the N5 ladder. A *selection* is a definition
naming one cell (or a set) of a `Body` variable. A *pose* is a frame known up
to its kind's symmetry; a `Point` is a position. A cell has poses read off
it: a face its plane, a cylinder face its axis, a sphere its centre, a vertex
its point. The cell is what was built; a pose is one function of it. So "a
point as a selection vs a point as a pose" is a false pair: the selection is
the *vertex*, and the point is read off it. There is one way to say the
vertex (`select(body, v)`) and one way to say its point (`Point { of: v }`),
which is already how a face and its plane relate under D10 and the stage 3
spec (`Plane { face }`, `Axis { of: Face | Edge }`, `Point { of: Face | Edge }`).

**Why a measure reads the cell, not the pose (sure).** A measure's primitives
are margined verdicts over cells, not functions of poses: `distance` between
two planes decides parallelism with the faces' extents as its lever arm
(`reach`); `gap` reads the faces' outward (material) normals; `min_clearance`
is over material. A pose has neither extent nor material. Reading a vertex
through a `Point` pose (option D as posed) would make the vertex the one cell
a measure reads through a pose, two operand regimes in one operation, and
would admit a measure against a written coordinate (`InFrame` point) that
nothing asked for. Making every `distance`/`angle` operand a pose (the "one
level up" answer) fails on the same extent-and-material facts; it is
reversible later if measures are ever split into pose measures and cell
measures, and should not be done now.

**The mate (likely).** A point mate reads `Point { of: v }`, a `Point` pose,
`Point`–`Point` being the ball joint stage 3 names. That is not the thing a
measure reads, and it should not be: the mate equates poses, the measure
judges cells. They share the one `Vertex` selection, so one resolution, one
N5 diagnosis and one `Rebind` serve both. Nothing else a vertex could yield
is a pose (it has no direction), so `Point` is its only read-off.

**Options rejected.**
- *(B) no vertex kind; point measures go.* Deletes a working, basic
  capability (corner-to-corner is the first measurement any CAD user takes;
  a vertex pick is how the GUI picks a point), while the names system keeps
  minting vertex names nothing can read. Unneeded machinery is a flaw, but so
  is a cell you can name and not select.
- *(C) a vertex is an `Edge` plus an end.* This is the positional defect the
  D10 conversation set out to remove: which end depends on the edge's
  orientation, a vertex on `n` edges has `n` spellings (many ways to say one
  thing), and it inverts N1, where edges are told apart by their end
  vertices.
- *`Vertices` now.* Ev: "the variable's type should suit its slot". No slot
  reads a vertex set (fillet reads `Edges`, shell `Faces`), so no kind; the
  sentence below says why the list is asymmetric.

**A consequence worth taking (likely).** A selection's kind is fixed at
minting, so each measure primitive's admitted kinds (`distance`: Face, Edge,
Vertex; `angle`: Face, Edge; `min_clearance`: Body, Face; `gap`: Face) are
checked at the edit door as the ordinary slot-kind refusal, where today they
refuse at evaluation (`MeasureSelectionKind`, "resolves to a vertex"). The
carrier-class refusal (a cone face in `distance`) stays at evaluation, since
the surface class is not in the kind.

**Text after the answer.** Ratified clauses, with the sentence each becomes:
1. D10 Variables, kinds: "and the selections of a shape: one cell of a body
   (`Face`, `Edge`, `Vertex`), or the set of them a reader takes (`Faces`,
   `Edges`; a set kind exists only for a slot that reads one)."
2. D10 Variables, poses: "read off a body's geometry (a face's plane, a
   carrier's axis or centre, a vertex's point)".
3. D10 Variables, projection: "A face reads as a plane and a vertex as a
   point; no reader takes a carrier's reference direction."
4. D10 Operations: "A `Face`, `Edge` or `Vertex` variable, or a set of them,
   is a selection of a `Body` variable by `StableName` …" (rest unchanged).
5. FORK-3's ruling sentence (#4222; `work/intent/select-defines-face-and-
   edge-variables.md`, stage 2 spec §1, §6, §11): "`FaceFrame` and a mate
   side read one `Face`; a `Measure` operand reads one selection of the
   kinds its primitive admits, or a `Body`." (It already omitted `Body`,
   which `min_clearance` reads.)
6. VARIABLES-DESIGN VR3's later-kinds list gains `Vertex`. VR4 is unchanged:
   a `Vertex` slot holds a read like any other.
7. Stage 3 spec §1: `Point { of: Face | Edge | Vertex }` (a carrier's centre,
   or a vertex's point); `Flip` still has no `Point` arm.
The N5 ladder needs no vertex clause: it runs over a `StableName` of any kind
inside the select, then `SelectKind { expected: Vertex, found }`. N1 and
SELECT-DESIGN §4 are unchanged. Provenance of what changes: 1 and 4 were
written in #3990 and recopied in #4222 (8d63e344f); 2 in FORK-S3P
(de87a9fd3); 3 in FORK-S3P round 10 (7147b6046); 5 in #4222. All are agent
text in `[ev]` PRs, none Ev's words.

**What moves.**
- *Stored documents:* a vertex measure ref migrates like a face ref: one
  `Select { body, names: [v] }` of kind `Vertex` plus a read. No committed
  fixture or golden holds a vertex ref (sure: `golden.cad` has none);
  `slot_tables.txt` moves with E anyway. No body digest moves.
- *Python:* `MeasurePrimitive.distance((node, name), (node, name))` takes
  selection reads like every operand after E. `test_measures.py`'s two-site
  vertex becomes two `Vertex` selects of one name on two `Body` variables,
  the extrude's and the transform's, with the same numbers (zero at one
  body, the translation across the two); `all_vertices` is unchanged.
- *Rust tests:* `m10_2_measure.rs`'s two-site test likewise;
  `wire_entity_door.rs`'s "min_clearance … resolves to a vertex" rows become
  door refusals if the consequence above is taken.
- *Viewer:* a vertex pick already inverts to `EntityKey::Vertex`; E's
  "picks become select edits" gets a `Vertex` arm (`frame.rs` maps a vertex
  pick to the node today).

**Reversibility.** Adding `Vertices` later is additive. Moving measures onto
poses later touches only the measure's slots. Nothing here forecloses either.

## For the orchestrator

- **Brief errors.** Option A's "a measure operand admits Body | Face | Edge |
  Vertex" is per primitive, not per operation (`distance` refuses a whole
  body as `Carrier::Other`; `gap` takes faces only); FORK-3's sentence
  already omits `Body`, which `min_clearance`'s `Scope::WholeBody` reads.
- **Assumed:** the ladder (`wire.rs` `mod ladder`, `named_entity`) is
  kind-agnostic over `StableName.kind`; I read its signature and the kind
  check sites, not every rung. `scope_of` (`wire.rs`) is `min_clearance`'s
  kind door and is what the door-time check replaces.
- **Not checked:** PR comments (per the brief); no tests run; stage 3 A's
  `PoseDef::Point` is spec text, not code.
- **Off-question:** `Carrier::class` calls a point carrier "a vertex",
  naming the cell for the carrier; trivial, not filed. The stage 2 spec §1
  `VarKind` sentence and §6's `SelectKind` fold are sites for `Vertex` too.
- Ev's transcript contains nothing on vertices or kinds; the only words I
  lean on are "the variable's type should suit its slot" (for no
  `Vertices`) and "many ways to say these things" (against C).
