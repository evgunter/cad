# FORK-VTX — how a measure (or anything) reads a vertex — designer B

## For Ev

**Recommendation (likely): a vertex is a selection, like a face or an edge;
a point is what a vertex reads as.** `Vertex` joins `Face` and `Edge` as a
selection kind (a `Select { body, names }` whose name denotes a vertex). A
measure's operand reads the selection itself, as it reads a face or an edge.
A slot of the `Point` pose kind (a point mate, when one exists) reads the
same `Vertex` variable through its projection, as D10 already says "a face
reads as a plane". Of the brief's options this is (A), with (D)'s point
taken: the pose belongs to slots that place, not to the measure.
No `Vertices` set until a reader needs one.

**Premise check.** (D) is not an alternative to (A). A `Point` "read off a
body's geometry" has to say *which* vertex, and the only way to say that is
a stable name held against a body, which is a selection. A `Point`
definition carrying its own `StableName` would be a second kind of select,
with a second home for the name-resolution ladder (N5) that D10 and the
stage spec put "in the select and nowhere else". So the real question is
what a *reader* takes once a vertex is a selection: the entity, or its
point. (sure)

The kinds sentence that leaves vertices out was agent-written (FORK-1/3
text, 2026-10-07, following #4222). Ev's 2026-10-03 words say nothing
about vertices. What it lists is what that fork's readers needed (fillet
and chamfer edges, shell and frame faces), not a ruling that vertices are
not selectable. Persistent naming already names every vertex (N1:
`EntityKind::Vertex`, revolve `Pole`s, `CrossingVertex`), and its
totality check refuses a live vertex left unnamed. A name kind that no
variable can hold is a gap. (sure)

**The options as final states.**

- **(A+) Vertex selection; a measure reads selections; a pose slot projects.**
  A selection is a name for the same entity kinds N1 names below a body:
  a face, an edge or a vertex. A measure reads entities of the built body.
  That is E3's contract: closed forms over the carriers the entities sit
  on. Faces and edges need the entity, not a pose. Plane×plane and
  cylinder×cylinder distances price their parallelism at the faces' extent
  (`reach`). `gap` needs the outward normal and the radius. `min_clearance`
  needs the face region. For a vertex, entity and point coincide, so a
  point measure is lossless. A future point mate's `Point` side reads the
  same `Vertex` variable by projection: one name, one ladder, one position
  read (`vertex_point`). Nothing is said twice. "Which vertex" is the
  selection, and "where it is" is the projection a pose slot takes.
  Reversible: adding a `Point` arm to a measure later is additive.
- **(D-full) measure operands are poses.** Distance and angle read
  `Point`/`Axis`/`Plane`; a selection projects. This is uniform with mates,
  and a measure could also read a datum point or a revolve's `axis` port.
  But a `Plane` pose has no extent, so plane×plane parallelism loses its
  scale-aware lever, and `gap` and `min_clearance` still need faces. So the
  operand becomes "a pose here, a face there". The vertex is the one case
  where the pose loses nothing, and that is not enough to retype the
  measure around. Measuring to a datum pose is an imagined need today.
  (likely)
- **(B) no vertex kind; point measures go.** This removes a working,
  tested capability that a CAD user reaches for first (hole-to-hole
  distance, corner-to-face standoff). It also leaves stage 3's point mates
  nothing to read a vertex through. Reject. (sure)
- **(C) a vertex is an `Edge` plus an end.** This names a vertex by
  position: "end" is an edge's orientation, which is a kernel artefact.
  It gives one vertex as many spellings as it has incident edges times two,
  and a pole or a lone vertex has no honest edge to hang off. It is
  exactly the defect D10 set out to remove. Reject. (sure)

**Mates.** A point mate equates two `Point` poses. A `Point` read off a
vertex is the same value a measure's vertex operand reads: the select
resolves the name and `vertex_point` reads the position. In (A+) a mate
reads it through a pose slot and a measure through a selection slot, and
both readers share one variable. In (D-full) the slot kinds would match too,
but at the cost above. (likely)

**Text that changes** (all agent-written; none is Ev's wording):

1. D10 §Variables, kinds: "…and the selections of a shape (`Face`, `Edge`,
   `Vertex`, and the sets `Faces`, `Edges`), each the entity one stable name
   denotes in a body, or a set of them where a reader takes several."
2. D10 §Variables, projections: "A face reads as a plane and a vertex as a
   point; no reader takes a carrier's reference direction."
3. D10 §Operations: "A `Face`, `Edge` or `Vertex` variable, or a set of
   them, is a selection of a `Body` variable by `StableName`…"
4. FORK-3's ruling sentence (the work item and stage-2 spec §1, §6):
   "`FaceFrame` reads one `Face`, a mate side one `Face`, and a `Measure`
   one `Face`, `Edge` or `Vertex` (`min_clearance` a `Body` or a `Face`)."
5. VARIABLES-DESIGN VR3: the later kinds list gains `Vertex`. VR4 is
   unchanged: a measure operand still holds one `VarId`, and the slot-kind
   check (VR9's load door) admits the operand's kinds.
6. The select's N5 ladder: unchanged. It is generic over names, vertex
   names already resolve and diagnose (`resolve/mod.rs` has the vertex
   arms), and a tied vertex refuses as a tied face does. The kind check
   (`SelectKind { expected, found }`) reads the name's `EntityKind`, so a
   vertex name in a `Face` slot refuses at the door.

**What moves.**

- *Stored documents.* A measure reference holding a vertex name becomes
  a `Vertex` select on that body. The shape is the same as E's face and
  edge conversion, and no extra format appears.
- *Python.* `MeasurePrimitive.distance((prism, corner), (moved, corner))`
  is unchanged at the call site. The tuple sugar lowers to one select per
  operand, with its kind taken from the name (a `StableName` carries its
  kind). That gives two `Vertex` variables with the same name on two body
  variables. The test's meaning moves with stage 3 (see the orchestrator
  section).
- *Rust.* `m10_2_measure.rs`'s vertex rows keep their assertions. Only
  their `SitedRef` construction changes, as every measure row does under E.
- *Goldens.* No vertex-specific golden was found. Measure content keys move
  under D and E regardless.

Confidence: the vertex is a selection (sure); the measure reads the
selection rather than a pose (likely); one sentence for the point
projection now (likely: it is text for a slot stage 3 will add).

## For the orchestrator

- **Off-question (file as an issue):** "A face reads as a plane" (FORK-S3P
  r10) does not say which sense the plane has. `angle` reads the chart
  normal, `gap` and `FaceFrame` (DM1a) read the outward normal, and an
  oriented `Plane` pose has to pick one. Two readers disagree today.
- **Off-question:** `FaceFrame` takes sketch +x from the carrier's
  u-reference (DM1). That contradicts D10's "no reader takes a carrier's
  reference direction". Stage 3 D's retirement of `Datum` may moot it.
  Check that the stage-3 spec says so.
- **The two-site Python and Rust test** (`test_a_measure_reads_the_placed_carrier`,
  `m10_2_measure.rs` ~L870) measures one vertex in a body and in its
  transformed copy. Under D10 that reads across two spaces unless the copy
  is placed against the original. When stage 3 D re-spells `Transform`, the
  test either places against the prism (and keeps its meaning) or refuses
  as a kind mismatch. This fork does not decide that.
- I assumed `Select { body, names }` infers its kind from the names' shared
  `EntityKind`, so mixed kinds refuse. If E stores the kind separately, the
  kind and the names should not be able to disagree.
- I did not fetch any PR comments. #4222's ruling text was read from the
  work item only.
