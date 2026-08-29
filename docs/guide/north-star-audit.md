# The north-star audit

The standing goal, recorded by Evan on 2026-08-09 and written into
the demos' own purpose block (`demos/tour/src/main.rs`):

> Standing goal: every demo authorable through the Python bindings;
> what a demo cannot do through the curated document surface is a
> named gap, not a private exception.

This page is the measurement against that goal. It is not a plan and
it builds no doors — it says, scene by scene, what is true **today**,
and names the missing door for everything that is not. The same block
governs how the gaps get treated:

> If some aspect of a demo is AWKWARD to write through the public
> surface, that awkwardness is a LIBRARY FINDING: record it (gap
> comment here + the orchestrator's log) as something to fix in the
> library — never quietly work around it, and never contort the demo
> to hide it.

**Result: 32 of the 47 tour stops are authorable through Python
today** — 29 outright, and 3 more only if you re-author by hand what
the scene says structurally. 15 are blocked by a missing door.

(Those three numbers are counted off the table below at each
revision, never carried forward, and two mechanisms now hold that
discipline rather than this sentence. The tour's roster and this
table's rows are compared in
`crates/pncad/tests/all.rs::the_north_star_audit_has_a_row_for_every_tour_stop`,
which reads the stop names out of `demos/tour/src/`'s own source and
fails in BOTH directions — a stop with no row, and a row for a scene
the tour no longer builds. The headline above and each gap's `stops`
column are re-counted off the rows by its sibling,
`the_north_star_audits_tallies_are_derived_from_its_rows`. Both were
written because the page had drifted twice over: the previous
revision's headline read 26 = 23 + 3 against a table that already
said 25 + 3, and — the larger miss — the table sat at 34 rows while
the tour had grown to 47. The rows are the record; nothing here is
carried forward.)

The count climbed from 7 as each binding unit closed its gap:
**LIB-PYG1** bound the PATHS lattice, closing G1 (`bracket`, `vase`,
`sheave`, `bossplate`); **LIB-PYG23A** bound the sketch-plane
vocabulary and the loft node, closing G3 (`silhouette`, `silhouette3`
and its three shadow stops) and G2's loft half (`loft_prism`,
`nonuniform_loft`); **LIB-PYBUNDLE** bound the fillet, split,
transform and datum-plane nodes and grew a profile past one loop,
closing G4, G6, G7 and G9; **LIB-LBRET** built PATHS-DESIGN §2b's
route-3 door and migrated `rocker`'s outline to the lattice, closing
G12; **LIB-PYSEL** bound the selector surface, closing G13 — so
`diecomposed`'s two blends are now narrowed by the SAME two geometric
filters the Rust scene runs, with no name text read; **LIB-G14** took
down the two split-naming walls, closing G14 (`cutaway`).

**LIB-PYPU** is the counter-example that proves the rule this page
works by: it bound the group boolean (`Node.placed_union`) and the
narrowed structural-count edit (`DocEdit.bind_count_param`), which is
two thirds of what G8 named — and it flipped NO row, because the
third third is a kernel door. So the gap was re-diagnosed rather than
closed, and the count did not move.

**LIB-G15** is the other side of that coin, and it is why the count
still reads 32. It closed G15 outright — `Workspace`, `ContentPin`,
`DocRef`, `content_pin`, `canonical_bytes`, `header_document_id`,
`random_document_id`, and one typed `WorkspaceError` — and flipped no
row, because G15 was never any scene's PRIMARY blocker: it was the
named secondary on rows 46 and 47, and those two wait on G18's node
half (LIB-G18a has since spent the other thing they were waiting on —
`evaluate`'s missing resolver — and flipped no row either, for the
same reason: the rows ask whether a user can AUTHOR the scene). A gap can close
without a mark moving, exactly as a gap can shrink without one. The
rows are what count; a closure is not a promotion.

One instructive pattern from LIB-PYBUNDLE: three of its stops did NOT
flip outright, and each named the door it was actually waiting on
rather than inheriting the one that closed — `rocker` (G12, since
closed), `diecomposed` (G13, since closed), `cutaway` (G14, since
closed). G14 was a measured, executed refusal in `test_north_star.py`
— and its DIAGNOSIS was wrong, which is the sharper lesson: the row
blamed boolean provenance, and the survey that preceded the fix found
the refusing wall had nothing to do with booleans (see the G14
register row). G13 never was a refusal and the page never
claimed one: the wall was CONTRACTUAL — the name text is an opaque
identifier, so narrowing a set by reading inside it was
representation-dependence, hand-authoring rather than the selector
the scene expresses (the ordinal-28 ruling). Closing the gap did not
soften that contract; it bound the doors that make reading the text
unnecessary.

Every YES row below is verified by executed Python in
`crates/pncad-py/tests/test_north_star.py`, which rebuilds the scene
and checks it against the *same exact volume oracle the Rust scene
asserts*. Two of the four rows G1 unblocked are the honest exception:
`bracket` and `vase` are scenes the Rust tour holds only to its
generic ladder (validate, tessellate, mesh against mass properties)
and gives no closed form, so their Python rows derive one, state the
derivation, and assert it. The two loft rows are a third shape of the
same honesty: the tour holds them to the generic ladder too, but each
scene's own note carries a closed-form DERIVATION (9 m³;
8 + 0.25/(t(1−t)) with t the chord-length v-parameter), and the
Python rows assert exactly those numbers against the certified
enclosure — `loft_prism`'s is additionally the bracket pin in
`sweep/tests/m6_loft_body.rs`. Every NO row's gap is asserted as an
absence in the same file, so **the day a gap closes, this audit fails
and must be updated**.

Four rows arrived with the roster re-cut, and each is executed against
whatever its scene actually pins rather than against a volume it does
not have. `hollowring` is the easiest case in the tour to be sure of:
the Rust scene ALREADY builds the ring twice — once through the plain
revolve door and once as a three-node recipe — and asserts the two
bodies agree bit for bit, so the Python row rebuilds that recipe and
checks the same torus closed forms (V = 2π²R(rₒ² − rᵢ²),
A = 4π²R(rₒ + rᵢ)) plus the scene's absolute 4/8/4 census. `klein`'s
two elbows carry a Pappus closed form the scene asserts and the Python
row asserts the same numbers; its bulb has none, so the row asserts
instead the scene's own discriminating pin — twelve faces, of which
exactly four are cylinders. `budfillet` has no closed form either and
the row carries the scene's three checkable proofs: the census before
and after (5/10/5 → 8/16/8, three annulus bands), three torus faces,
and the volume drop inside the scene's own Pappus bracket.
`twopeg_apart` asserts the two parts' volumes, which are the numbers
the scene's own union ladder checks as it builds them.

## The bound surface

Everything Python can say about geometry, in full:

- The **PATHS authoring lattice** — `Open`, `Start`, `circle`,
  `circle_split`, and one class per lattice state (`PathOpen`,
  `PathPoint`, `PathDirectedPoint`, `PathAngle`, `PathDirected`,
  and the arrival builders `PathRadiusArrival` /
  `PathRadiusArrivalAt` / `PathRadiusArrivalDir` / `PathViaArrival` /
  `PathViaArrivalStart`), each exposing only its legal continuations.
  The full current verb vocabulary: `at`, `angle`, `toward`,
  `tangent`, `turn`, `to`, `line`, `line_to`, `arc_to`,
  `arc_continue`, `tangent_arc_to`, `fillet`, `fillet_arc`,
  `arc_fillet`, `arc_fillet_arc`. Arcs are authored by SPEC MODE
  (`Bulge`, `Via`, `Center`, `Radius`, `Sweep`, `ArcLen`, with the
  `ArcSide` bit): one verb per act, the mode carrying the binding,
  and which modes a state admits IS the admissibility matrix — an
  inadmissible pair is a `TypeError` at the boundary with no kernel
  call made, the runtime shadow of Rust's missing trait impl.
- `SketchPlane` — the sketch plane as a VALUE: the named cyclic
  frames `xy` / `yz` / `zx` (normals +z / +x / +y), and the general
  `from_frame(origin, u, v)`. Rigidity is the kernel's own unchecked
  convention and the binding adds no predicate of its own, so a
  non-rigid frame is a well-defined skewed sketch rather than a
  refusal.
- `Node.profile(outline, elevation=…, plane=…)` — one closed loop
  from that lattice, **or a list of them** (outer boundary first,
  then holes), on any sketch plane, built from the loops' recorded
  programs. `plane=` and `elevation=` are mutually exclusive
  (`elevation` is the xy sugar); passing both is a `TypeError`.
  Nothing about the loop SET is pre-checked: nesting and containment
  are `Profile::validate`'s work, reaching Python as a typed refusal
  at `insert`.
- `Node.polygon(points, elevation=…, plane=…)` — the straight-segment
  shortcut, same plane story.
- `Node.extrude(profile, distance)` — along the sketch-plane normal,
  so the plane is what chooses the axis.
- `Node.revolve(profile, axis, angle)` about a `Node.datum_axis`.
- `Node.loft(profiles, v_degree)` — a skinned solid through two or
  more section profiles in skin order, at an integer v-degree. There
  is no placement argument: each section rides its own profile's
  sketch plane.
- `Node.boolean(op, a, b, declare=…)` — union, intersect, subtract.
  `declare` is the DATA door for a declared contact: it names a
  `Declare` node the boolean consumes — and since LIB-PYG5 Python can
  BUILD one: `Evaluation.find_flush_candidates(a, b)` reports the
  flush pairs as typed `FlushFinding` values — **planar pairs only**,
  which is G19 — `Node.declare(findings)`
  (or the `Doc.declare`/`Doc.declare_all` sugar) turns inspected
  findings into the `Declare` node, and the undeclared refusal itself
  is the MENU (`EvaluationError.kind == "undeclared_contact"`, the
  candidate `finding` attached).
- `Node.fillet(target, radius, selection)` — constant-radius blends
  on named edges. The selection is edge names as TEXT, materialized
  off an evaluation and then FROZEN into the recipe: a commitment,
  not a live query. There is no "every edge" spelling.
- `Node.split(target, tool)` — cut a body by a `Node.datum_plane`.
  The value is a split: `Value.split()` answers `(above, below)`,
  `None` where a side is empty.
- `Node.transform(input, translation, rotation_axis, rotation_angle)`
  — a rigid placement, the kernel's convention unchanged: rotate
  about the axis THROUGH THE WORLD ORIGIN, then translate.
- `Node.datum_plane(origin, normal)` — the datum a split cuts with,
  beside `Node.datum_axis`.
- `Evaluation.all_edges/all_faces/all_vertices/all_bodies(node)` —
  U7's whole-body materializers, answering the names as text. A
  materializer, never a query: it answers for the evaluation in hand,
  you store the answer, and a recipe holds no live selection because
  a stored one would silently grow under an upstream edit.
- `Evaluation.select(node, selector)` and
  `Evaluation.select_where(node, selector, geom)` — the narrowing
  doors (LIB-PYSEL), same materializer contract, same opaque-text
  alphabet. A `Selector` is a union of `NamePat` role-path shapes
  (`SegPat` per segment: tag, `OpGroup` group, side, sub-name
  prefix); `geom` is a conjunction of `GeomPred` atoms — the EXACT
  tag reads (`curve_kind`, `surface_kind`, `adjacent_kinds`) and the
  DECIDED `datum_distance(datum, Cmp, Length)`, whose in-band
  candidates refuse as the typed `SelectRefusal` rather than being
  silently included or dropped. Patterns are values built in Python
  and evaluated in Rust; `Selector.matches`/`NamePat.matches`
  classify a materialized text, so the binding stays the one
  licensed reader of a name.
- `SketchPlane.origin/u/v/normal` and bit-exact `==` — the frame
  reads back, and the equality is `SketchPlane::bit_eq` crossing
  unchanged (`-0.0` keeps its own identity; there is no tolerance in
  a plane to compare with).
- `DocParam` compares and hashes, mirroring Rust's `PartialEq` — the
  IEEE comparison, not `bit_eq`'s.
- Edits: `insert_node`, `delete_node`, `set_tolerance`, and — since
  R1-PARAMS — `set_doc_param(ParamName, DocParam)`, the named
  document parameter edit (guide §3.2).
- `Doc(label=…)` and `Doc.id` — a document's IDENTITY. `Doc()` mints
  a fresh random id, so two documents authored from Python are two
  parts and one workspace holds both; `Doc(label)` derives the id
  from the label for callers whose saves must reproduce. `Doc.id` is
  the canonical 32 hex digits the save header carries and the store
  keys on.
- `Workspace(path)` — a DIRECTORY of documents, scanned by each
  file's `id:` header alone. `documents()` is the identity → path
  map; `create` and `resave` are the two write doors (there is no
  general mutation API, which is the Rust surface's posture kept);
  `resolve(DocRef)` loads through the full `load` door sequence and
  hands the document back IFF its recomputed pin matches. Refusals
  are one typed `WorkspaceError` carrying `variant` plus the arm's
  payload — `path`, `id`, `first`, `second`, `wanted`, `found` —
  present on every arm.
- `ContentPin` and `DocRef` — which VERSION, and the (part, version)
  pair a cross-document reference carries. `content_pin(doc)` and
  `canonical_bytes(doc)` are the pin doors, `header_document_id(text)`
  the store's cheap scan, `random_document_id()` the interactive
  mint. A pin that moved refuses (`variant == "pin_mismatch"`,
  `PIN_MISMATCH_RECOURSE` the recourse the message ends on) and is
  never silently retargeted; `current_pin(id)` says what the new
  version is. Read that as a statement about evaluations that ASK:
  `evaluate`'s `prior=` serves a memo hit without re-running the
  seam's gates, so a reused instantiate node raises no availability
  refusal — it serves what its own `DocRef` pins, content-certified,
  and nothing is retargeted either way (LIB-G18a, stated at the door
  and pinned in `test_assembly_eval.py`). A reference is USED at two doors and only one
  of them is bound: `evaluate(doc, resolver=store)` carries the
  document seam, so an assembly document that already holds
  `InstantiatePart` nodes evaluates through the store (LIB-G18a); what
  is still absent is the node itself, so Python can evaluate an
  assembly it loads and cannot author one. See G18.

- `Body.tessellate(chordal)` and the `Mesh` it answers — the ladder's
  steps 4 and 5 (LIB-G11). δ is a `Length`, not the kernel's ε; the
  mesh's shared position buffer and its per-face patches both cross,
  so the mesh-vs-exact cross-check is a computation the CALLER writes
  over the bound triangles rather than a second reading of the same
  code path. `Mesh.to_stl_ascii` / `to_stl_binary` answer the bytes.
  What does not cross is the picking chain: a patch's face and a
  boundary's edge are arena keys, so a patch is addressed by index and
  the per-edge boundary polylines are unbound.

Plus evaluation, validation, mass properties, STEP export/import,
persistence, and typed quantities. That is the whole vocabulary.

## The audit

Every NO row names its **gap id**, which is the row's pointer: the id
resolves in the gap list below, and each gap there points onward to
the LIB residual register (`docs/LIB-LOG.md`, "LIB residual register",
category B) and to any design doc or register item that owns it. The
`gap` column is the row's *primary* blocker — the most fundamental
missing door — so the ids partition the 15 NO rows exactly; secondary
blockers stay named in the last column.

The rows are in TOUR ORDER: `walk_tour` in `demos/tour/src/main.rs` is
the one enumeration of what the tour contains, and this table walks it.
One row per `Stop`, which is what the roster guard compares against —
so the narration-only passes `walk_tour` also makes (`checks`, the
wall-probe batteries, the coincidence-ladder narrations) have no row
here. They build no stop and render nothing; `checks` says so in its
own module doc, *"narration-only (no render stop): the subject is a
REPORT, and its `Display` is the picture."* A row for one of them
would fail the guard's decay direction, which is the right answer:
this page grades BODIES against the bound surface.

| # | scene | Python? | gap | the missing door |
|---|---|---|---|---|
| 1 | `bracket` | **YES** | — | — |
| 2 | `spacer` | NO | G16 | `chamfer_edges` has no recipe node, so a consumer wanting a chamfer in a RECIPE — with names, with a rebuild — cannot have one; the scene's own second recorded friction, on a part with no recipe behind it |
| 3 | `plate` | **YES** | — | — |
| 4 | `vase` | **YES** | — | — |
| 5 | `sheave` | **YES** | — | — |
| 6 | `chute` | **YES** | — | — |
| 7 | `rocker` | **YES** | — | — |
| 8 | `diefillet` | **YES** | — | — |
| 9 | `diepips` | **YES** | — | — |
| 10 | `diecomposed` | **YES** | — | — |
| 11 | `diechamferblank` | NO | G16 | same missing node, on the blank |
| 12 | `diechamfer` | NO | G16 | same missing node, on the die — and the scene's second finding is the narrower one this page cares about: `select_where`'s answer (stable names) cannot be handed to a KERNEL verb (arena keys), so even with the body in hand the selection has to be re-said by hand |
| 13 | `lily` (15 bodies) | NO | G2 | tube/sweep, both banked (below); placement (G7) |
| 14 | `budfillet` | **YES** | — | the curved-support arms reach through `Node.fillet` unchanged; the rims are named by `select_where(adjacent_kinds)` plus a `datum_distance` station, which is the scene's own by-description selection said in the document layer's words |
| 15 | `klein` | **YES** | — | three revolves and no boolean — the meridian band's `.fillet`/`.tangent_arc_to` chain is on the lattice, and the elbows are a two-loop profile partially revolved about a datum axis at a negative angle |
| 16 | `tiltedcut` | **YES** | — | — |
| 17 | `bossplate` | **YES** | — | — |
| 18 | `loft_prism` | **YES** | — | — |
| 19 | `nonuniform_loft` | **YES** | — | body transfers; the scene's `loft_parameters` read-back does not — a named residue, not a gap |
| 20 | `s_duct` | NO | G2 | sweep is BANKED, not unbound: `wire_sweep` refuses unconditionally (`SWEEP_FRONTIER`) |
| 21 | `twisted_duct` | NO | G2 | same `SWEEP_FRONTIER` bank; a non-planar spine also needs the 3-D-path tail (U4/LQ3) |
| 22 | `twisted_duct_shadow_z` | NO | G2 | same body — the `SWEEP_FRONTIER` bank |
| 23 | `twisted_duct_shadow_y` | NO | G2 | same body — the `SWEEP_FRONTIER` bank |
| 24 | `tube_along_arc` | NO | G2 | no `Node::Tube` at all: a new node kind is a SCHEMA-VERSION break, so it has to be sequenced against whatever bump is in flight. **Widened by VERBS-TUBEWALL:** the door now has a hollow sibling (`tube_along_arc_hollow`, outer minor radius + wall), so the missing node has to carry the wall too — one node kind, not two, and the same bump |
| 25 | `hollowring` | **YES** | — | the scene settles this itself: `ring::through_the_document` builds the same ring as a three-node recipe — a two-loop `Profile`, a `Datum::Axis`, a full `Revolve` — and asserts the two doors agree bit for bit on volume. Those are exactly the three doors Python binds |
| 26 | `hollowelbow` | NO | G2 | the same missing `Node::Tube`, now carrying the wall (row 24). A partial revolve of an annulus makes this SHAPE, but the scene's subject is the parameter door's storage contract — both outer half-walls store the caller's `minor_radius` bit for bit and both inner ones store `minor_radius - wall` — which a revolve reconstructs rather than stores |
| 27 | `hollowtorus` | NO | G2 | same door at the full period. `hollowring` next door is this census through the PROFILE door, which is why the two rows differ: that scene's claim is about a holed profile, this one's is about stored intent parameters |
| 28 | `teapot` | NO | G17 | `shell`/`shell_open` shipped at #1048 and have no recipe node, so the vessel — the whole point of the scene — has no document. The handle is `tube_along_arc` (G2) as well; the lid (revolve + fillet) and the spout (revolve + transform) are sayable today |
| 29 | `die` | **YES** | — | — |
| 30 | `table` | **YES** | — | — |
| 31 | `silhouette` | **YES** | — | — |
| 32 | `silhouette3` | **YES** | — | — |
| 33 | `silhouette3_shadow_z` | **YES** | — | same body as row 32 (a camera, not a construction) |
| 34 | `silhouette3_shadow_x` | **YES** | — | same body as row 32 |
| 35 | `silhouette3_shadow_y` | **YES** | — | same body as row 32 |
| 36 | `az` | **YES** | — | — |
| 37 | `crosslap` (glued) | **YES** | — | the mate still refuses undeclared — now as the typed menu; a named residue: declaring the detector's merge-stage (`SameOriented`) bottom pairs trips the naming emitter (pinned in `TestCrosslapGlued`), the scene's own mate needs none of them |
| 38 | `crosslap_exploded` | **YES** | — | — |
| 39 | `twopeg` | NO | G19 | the mate's three declared contacts are one planar `Rest` and two CYLINDRICAL ones, and Python can say only the planar third: the detector is plane-only and a `FlushFinding` is the sole input to the declare arm. Measured, not inferred — see the gap |
| 40 | `twopeg_apart` | **YES** | — | the apart framing is the two PARTS, which are ordinary transverse booleans plus a rigid lift; only the mate needs G19 |
| 41 | `projectbox` | **YES** | — | — |
| 42 | `cutaway` | **YES** | — | — |
| 43 | `heatsink5` | YES\* | G8 | fin family is ONE node with a param-driven count (LIB-PYPU); the fins-into-base fusion is still hand-authored |
| 44 | `heatsink7` | YES\* | G8 | same |
| 45 | `heatsink9` | YES\* | G8 | same |
| 46 | `bench` | NO | G18 | the assembly AUTHORING vocabulary is unbound: no `InstantiatePart`, no `mate`, no `set_placement`, no `set_roots`. Two of this row's named blockers are now spent and it stays NO, which is the row's own claim doing its job — the question is whether a user can AUTHOR the scene, and neither closure touches that. G15 (LIB-G15) made the two bench documents writable into a workspace side by side; the resolver (LIB-G18a) made an assembly document that already CARRIES instantiate nodes evaluate through that workspace — `crates/pncad-py/tests/test_assembly_eval.py` loads this exact scene from `tests/corpus/bench/` and reproduces the tour's own 3-solid oracle. What a Python author still cannot do is write the document it evaluates |
| 47 | `benchlayout` | NO | G18 | same series and the same re-cut: the layout's documents evaluate from Python now (the flat-pack's 4-post-plus-shelf oracle is asserted in `test_assembly_eval.py`) and cannot be authored there. It also wants the plain N-bodies `Pattern`, which is G8's deliberately-unbound node |

**YES** = the exact body is reproducible with the bound surface.
**YES\*** = the exact body is reproducible, but only by hand-authoring
what the scene expresses structurally — so the *body* transfers and
the *point of the scene* does not. `heatsink`'s residue is now
narrower than it was: the fin FAMILY is one `Node.placed_union`
node whose count rides a document parameter (LIB-PYPU), and what
still has to be hand-authored is the last step — fusing that
multi-solid family into the base, which is a kernel door that does
not exist. (`diecomposed` was this mark's other case — hand-narrowing
a materialized name set where the scene says one selector — until
LIB-PYSEL bound the selector.)

## What the YES rows look like

`chute` is the cleanest: one profile, one operation, no booleans.

```python
import math

from pncad import Doc, Node, deg, evaluate, m

poly = [
    (1.0, 0.0), (1.75, 0.0), (1.75, 0.625), (1.5625, 0.625),
    (1.5625, 0.1875), (1.1875, 0.1875), (1.1875, 0.625), (1.0, 0.625),
]

doc = Doc()
profile = doc.insert(Node.polygon([(x * m, y * m) for x, y in poly]))
axis = doc.insert(Node.datum_axis((0 * m, 0 * m, 0 * m), (0.0, 1.0, 0.0)))
chute = doc.insert(Node.revolve(profile, axis, 270 * deg))

body = evaluate(doc).value(chute).body()
body.validate()
assert abs(body.mass_properties().volume - (1287 / 2048) * math.pi) < 1e-12
```

`die` (a cube less 21 pip pockets, 21 sequential subtracts) and
`projectbox` (15 ops over 16 boxes) are longer but no harder — both
are chains of the same `slab` helper. Both reproduce their scene's
**exact dyadic oracle** from Python: `7.8359375` and `4.1982421875`
respectively.

The G3 family reads the way the captions do. `silhouette3` is three
letterform prisms and two intersects, and each letter is a polygon on
its own plane extruded along that plane's normal — `SketchPlane`'s
`from_frame` carries the 1/16-decoupled offsets the scene's
no-shared-carrier rule needs:

```python
from pncad import Doc, Node, SketchPlane, evaluate, m

# The scene's "T": a yz sketch at x = -0.25, extruded +x by 2.5 —
# the plane's normal is u x v = y x z = +x, so the plane IS the axis.
t_plane = SketchPlane.from_frame(
    (-0.25 * m, 0 * m, 0 * m), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)
)
T = [
    (1.1875, 0.125), (1.8125, 0.125), (1.8125, 2.625), (3.25, 2.625),
    (3.25, 3.125), (-0.25, 3.125), (-0.25, 2.5625), (1.1875, 2.5625),
]

doc = Doc()
sketch = doc.insert(Node.polygon([(a * m, b * m) for a, b in T], plane=t_plane))
prism = doc.insert(Node.extrude(sketch, 2.5 * m))

# Area = left bar 1.4375*0.5625 + stem 0.625*3.0 + right bar 1.4375*0.5
# = 3.40234375; times the 2.5 extrusion, exactly dyadic.
body = evaluate(doc).value(prism).body()
body.validate()
assert abs(body.mass_properties().volume - 8.505859375) < 1e-12
```

Rows 33–35 are the same body as row 32 seen down a different axis, so
they flip together and the Python rows read one node three times
rather than rebuilding three solids — exactly what the Rust scene does
with `three.body.clone()`. The sharing is true BY CONSTRUCTION, not by
a discriminating assertion: there is no body-identity surface in
Python that could make it one.

`loft_prism` and `nonuniform_loft` are the minimal pair: the same
three sections and the same v-degree, differing only in where the
middle section sits, which from Python is the three `elevation=`
values. The full pair is `test_north_star.py`; the guide's §2.2 shows
the executed three-section loft.

There is a reason those two transfer. Both scenes were authored under
the tour's own rule that no two operand planes may coincide anywhere
in a chain — so they never needed the declaration door in the first
place. The undeclared lane was always enough for them.

## The gap list

Ranked by how many stops each blocks first. **Register** is the
pointer each NO row inherits: every gap is carried as a
bindings-parity residual in `docs/LIB-LOG.md` → "LIB residual
register" → **category B**, which the register itself notes is
self-enforcing because this audit's test fails as doors land. Where a
gap also has a design doc or a lettered register item of its own, that
is named too.

| # | gap | stops | register / pointer | note |
|---|---|---|---|---|
| G2 | **Sweep and tube** (loft closed) | 8 | register B ("the big three") | Loft left this gap when LIB-PYG23A bound `Node.loft`. What remains is not an unbound door but two BANKED ones. **Sweep**: `wire_sweep` refuses unconditionally (`SWEEP_FRONTIER`, `editor-core/src/eval/wire.rs`) — the path-composition lane banked past M6 by the PR 10 MAJ ruling, so binding it would flip no row and un-banking is kernel-side. **Tube**: there is no `Node::Tube` at all, and adding a node kind is a schema-version break (the precedent was exactly Loft/Sweep landing), so it has to be sequenced against whatever bump is in flight. The tube/sweep/3-D-path tail is a design conversation: U4's measured spec, and **LQ3, ratified 2026-08-10 (#362, LIBRARY-DESIGN §L7)** — which names the discharge site rather than building it. A `geom-curves` chain→curve composition door is what would narrow `wire_sweep`'s refusal from everything to genuinely-unjoinable chains, and that un-banking is kernel-side work needing the kernel program's concurrence. Ratified direction, landed door not yet: rows 20–23 stay NO until U4's units land, and `Node::Tube`'s schema bump stays a separate coordination item with ASM's version sequence. **What VERBS-TUBEWALL added to the count**: the tube door's hollow sibling put two more stops behind the same missing node (`hollowelbow`, `hollowtorus`), so this gap now partitions eight rows rather than six. The hollow pair is the sharpest statement of what the node has to carry: the wall is one number the parameter door STORES (both inner half-walls hold `minor_radius - wall`, one IEEE subtraction of the caller's own two numbers), and a revolve of an annulus — which Python CAN say, and `hollowring` is the row where it says it — reconstructs that radius instead of storing it. Same shape, different claim, which is why `hollowring` is YES and `hollowtorus` is not |
| G16 | **Chamfer has no recipe node** | 3 | register B; issue **#918** (scheduled at VERBS-CHAMFER's merge as one of its three named deviations, beside #917 and #919 — `docs/VERBS-LOG.md`) | The verb SHIPPED and is curated: `sweep::chamfer_edges` is on the façade (`pncad::prelude`, beside `Chamfered`), plane–plane at a symmetric setback. What does not exist is a `Node::Chamfer` — `Node`'s variant list runs Datum, Profile, Extrude, Revolve, Loft, Sweep, Fillet, Split, Boolean, Transform, Pattern, PlacedUnion, Declare, InstantiatePart, Mate, and there is no chamfer among them. So a chamfered part has no document: no node, no stable names, no rebuild. `bodies::spacer` records this on a part with no recipe behind it and `diechamfer` on one that HAS a recipe and has to leave it — take the source body out, do the surgery beside it, and hand back something the document cannot name. The second half is narrower and is `diechamfer`'s own finding 2: `select_where` answers stable NAMES and `chamfer_edges` takes arena KEYS, so even calling the kernel verb next to a document cannot reuse that document's selection — "the twelve box edges" is re-said as a hand-rolled carrier-kind loop. **Not a bindings gap.** The day `Node::Chamfer` lands, binding it is the mechanical LIB-PYBUNDLE shape — `Node.fillet`'s twin, same frozen text selection |
| G18 | **The Python assembly series** | 2 | register B; the ASM cross-program deposit at the tail of `docs/LIB-LOG.md` (2026-08-23, recorded at Evan's direction), which is where the series and its order are stated | The assembly surface is entirely absent from `pncad-py`: no `instantiate_part` / `mate` (+ `Alignment`, `MateFrame`, `MatePrimitive`, `AxisSense`), no plain N-bodies `Pattern`, no `set_placement` / `set_roots` / `update_reference` / `mixed_pins` / `solve_document` / `product` / `assemble` / `split` / `inline`. And structurally FIRST, before any of those: **`evaluate(doc)` takes no resolver**, so an `InstantiatePart` node cannot evaluate from Python at all — a document that references another document has nothing to resolve the reference against. That is why the deposit states an ORDER: the resolver/workspace door first (small, possibly wanting a short design conversation on the workspace-from-Python shape), then the node/edit/refactoring bindings, which are mechanical once evaluation can resolve. **THAT FIRST DOOR IS SPENT (LIB-G18a).** `evaluate(doc, resolver=store)` passes a `Workspace` as the `PartResolver` it already is, so an assembly document loaded from a store evaluates, its parts shared across instances (`Evaluation.part_evaluations`), and the seam's whole refusal family — `part_no_resolver`, `part_pin_mismatch`, `part_epsilon_seam`, `part_unresolved`, `part_root_failed`, `part_product`, `part_reference_cycle`, `part_depth_exceeded` — is typed on `EvaluationError.kind`, which is where the tag map already put it. **Reachability is narrower than the tag map, and is stated per arm rather than by exception.** THREE arms are exercised from Python and pinned in `test_assembly_eval.py`: `part_no_resolver` (evaluate with no store), `part_pin_mismatch` (resave a part under a pinned reference) and `part_unresolved` (remove the file the reference names). The rest are UNREACHED TODAY, each for its own reason, and the common one is that reaching them needs a document Python cannot author or a store state Python cannot produce: `part_epsilon_seam` needs a stored document recording a different ε; `part_root_failed` and `part_product` need a part document whose own product is broken; `part_reference_cycle` needs an instantiate node pointing back up its own chain — and note that an HONEST store cannot hold one at all, since a cycle with valid pins would need a document whose content hash contains its own hash, and with invalid pins `part_pin_mismatch` fires first, so hand-crafted bytes do not defeat the claim either. `part_depth_exceeded` is left UNCLAIMED: a hand-crafted acyclic chain deep enough might reach it, and this unit did not establish whether it does. `evaluate`'s `prior=` landed with it and closed G8's memo residue. What is left is the whole SECOND half of the stated order and nothing of the first: the node, the edits, the refactorings, the gate, the mates, the product roots — so this row keeps both its stops and its stops count, and the two bench rows re-cut rather than moved. The Rust side is curated already — `pncad::document` carries the authoring vocabulary, and the validation half (`assemble`, `Assembly`, `AssemblyError`, `AtRestFinding`, `Attribution`, `MintedDeclaration`, `RefusedRef`) landed in #938's fix pass — so this is a binding unit and not a kernel one. **G15 was this series' first half and CLOSED at LIB-G15** — the `Workspace`/`DocRef`/`ContentPin` family is bound, so the store half of the deposit's stated order is spent; LIB-G18a spent the resolver parameter beside it, and the node/edit half is what is left. What moved onto this row with it: `update_to_store`, `update_references`, `mixed_pins`, `UpdateError`, `PinMultiplicity`, `PinSites` — the pin-UPDATE door, which G15 could not usefully bind because what it moves is a pin at its SITES and a site is an `InstantiatePart` node's `DocRef`. The two bench rows are what stopped this being hypothetical: the day the tour grew an assembly stop, "a Python author can produce two documents a workspace will accept side by side, and cannot then assemble them" acquired the two rows it had always predicted — and LIB-G15 made the first half of that sentence true, leaving the second exactly where it was |
| G17 | **Shell has no recipe node** | 1 | register B; `docs/KERNEL-VERBS.md`'s shell row (**SHIPPED at #1048**, the Q8 substrate arc) | The same shape as G16, one verb over. `topo::shell` and `topo::shell_open` shipped, and `Node` has no shell variant either — so the vessel that is `shell`'s DESIGNATED demo (Evan, 2026-08-09: *"a vessel is a shelled revolve"*) is the one part of the teapot with no document. The scene's other three bodies are not what blocks it: the lid is a revolve plus a `Node.fillet`, the spout a revolve plus a `Node.transform`, both sayable today; the handle is `tube_along_arc`, so it waits on G2. Klein is `shell`'s other consumer and does NOT appear here, which is the distinction worth keeping: that scene pays for the verb's absence by authoring every wall as its own two offsets by hand, and the result is a body Python can build (row 15). Paying a cost is not the same as being blocked |
| G19 | **Declared contact beyond the plane** | 1 | register B; `docs/SELECT-DESIGN.md` §3 (the detect/declare protocol G5 closed **for planes**); the kernel side is M9-3's declared cylindrical `Rest`, which `twopeg` renders | G5 bound the protocol whole and the protocol's v1 detector is planar. Three facts, each measured rather than read off a signature: (1) the declare arm takes `FlushFinding` VALUES and nothing else — `Node.declare` / `Doc.declare` / `Doc.declare_all` have no other input; (2) a `FlushFinding` cannot be built by hand from Python (`TypeError: cannot create 'pncad.FlushFinding' instances` — it is a report type by construction, which is the no-fusion boundary doing its job); (3) the detector that produces them is plane-only — `find_flush_candidates`'s probe calls `topo::flush_pair_relation`, whose `face_plane` answers `None` for any curved carrier, and the probe's own comment says it: *"Not a planar pair: not a v1 candidate, honestly."* So the two CYLINDRICAL `Rest`s `twopeg`'s mate declares have no Python path. **What is NOT the blocker**, checked so the gap points at the right door: the document layer's `Declare` is carrier-agnostic (`resolve_declarations` resolves a cross-operand FACE pair and pushes a `FacePairDeclaration` whatever the surface kind), and `topo` already has the curved verifier — `carrier_pair_relation`, *"the one carrier-pair door"*, which the planar arm delegates into. What is missing is the DETECTOR's curved arm and a Python route to a finding it produces. `TestTwopeg` executes all of it: the detector reports **7** findings on this scene's two parts, all planar (one `SameOpposite` — the mating plane — and six `SameOriented`); declaring every one of them still refuses, and refuses in the reduction's CURVED-face arm, which is exactly the arm the scene says a cylindrical declaration unlocks; and a pair whose ONLY coincidence is cylindrical reports nothing at all |
| G8 | **Multi-solid boolean operand** (replication + structural params CLOSED) | degrades 3 | register B | **Re-diagnosed at LIB-PYPU, from executed evidence.** The row used to read "no pattern node and no `SetStructuralParam` edit". Two of those three halves are now bound: `Node.placed_union`/`placed_union_at` say a whole placed family in ONE node whose value is an ordinary `body` (`TestHeatsinkFins`, `tests/test_placed_union.py`), and `DocEdit.bind_count_param` binds its Count slot to a document parameter, so `heatsink`'s 5 → 7 → 9 is ONE `set_doc_param` edit each against the corpus's own dyadic pins. What is left is the residual the old entry hid behind the binding question: the heat sink's shape is that family UNIONED INTO A BASE, and the kernel's `combine` door takes two SINGLE-SOLID operands (`JoinDesync`: "operand A/B is not a single-solid body"), so the fusion needs a kernel door that does not exist. That is why the rows stay YES\* rather than flipping — the gap is the kernel operand, not the binding, exactly as the old entry's own reasoning ran, one level down. `Node::Pattern` stays deliberately UNBOUND for the unchanged reason: its value is a plural `Instances` payload no boolean can consume, so binding it would still flip no row (`test_a_plural_payload_cannot_feed_a_boolean` executes the split's refusal and asserts the group's singular `body` as the contrast). One further residue, measured here and NOT a row-blocker, and now CLOSED (LIB-G18a): `evaluate(doc, prior=previous)` is the memo, and `Evaluation.reused` / `Evaluation.recomputed` — bound already, and until there was a prior to pass they could only ever read (0, n) — make the scene's *memoized recompute* observable. The two counters sum to the nodes that RAN OR WERE REUSED — the live node count when nothing refused, and short by exactly the poisonings when something did — so they are evidence rather than a hint. The row is unchanged otherwise: the fusion is still a kernel door |
| G14 | **Split naming: chord multiplicity + tied upstreams** | 0 (**CLOSED**, LIB-G14) | closed by LIB-G14 against `docs/NAMING-DESIGN.md`'s split-naming-walls section (RATIFIED, #512); the `NamingError`-`Display` gap it hid behind is #380, closed separately | The row's original diagnosis — "a plane crossing boolean-minted faces" — was WRONG, and the survey (`cad-work/g14-survey.md`) is how that surfaced: a split ACROSS boolean-minted faces named fine all along. TWO disjoint M4-era deferrals refused, and the one this scene hit involves no boolean at all — a section line that re-enters ONE operand face (an inner loop, or any non-convex face) would mint `SectionEdge{side, face}` twice; a plain L-shaped single-loop extrude reproduced it. The second wall refused a split whenever ANY operand-table entry was tied, even for pass-throughs nowhere near the tie. Both were logged as one M4 PR-3 sentence and read as one refusal because `NamingError` had no `Display`. Fixed as ratified: the chords become an N2 TIE (A2), and tied upstreams PROPAGATE as tied (B1), matching `name_pattern`/`name_in_part`/`graft_names` |

The five gaps partition the 15 NO rows exactly, counted off the table
above: **G2** takes 8 (rows 13, 20–24, 26, 27), **G16** 3 (rows 2, 11,
12), **G18** 2 (rows 46, 47), **G17** 1 (row 28) and **G19** 1 (row 39)
— 8 + 3 + 2 + 1 + 1 = 15. G8 blocks none and still degrades three,
which is what makes three rows YES\* rather than YES (43, 44, 45):
LIB-PYPU narrowed what G8 IS without moving a mark, because the half it
closed was never the half the mark depended on.
Authorable = 29 outright + 3 YES\* = 32, and 32 + 15 = 47.

The rows are the record and the tallies are derived. That used to be a
promise in prose, and it was broken three times: two of this list's
counts were off by one before LIB-PYG1 recounted them (G1 read 7 stops
against 6 table rows, G6 read 1 against 2), the headline once read 26 =
23 + 3 against a table saying 25 + 3, and the table itself sat at 34
rows while the tour grew to 47. Every number above is now counted off
the table by a test —
`the_north_star_audits_tallies_are_derived_from_its_rows` re-derives the
headline and each gap's `stops` column, and
`the_north_star_audit_has_a_row_for_every_tour_stop` re-derives the
table's own roster from `demos/tour/src/`. Both are in
`crates/pncad/tests/all.rs`.

No gap currently sits in this class. (There were two — G11, the
generic ladder's anchor, and G15, the named secondary on rows 46 and
47 — until LIB-G11 built the mesh door and LIB-G15 bound the store in
the same wave; the closed-gaps table below is where both went, and
neither closure moved a mark, which is what this class predicts.)

**What this page structurally cannot see.** Every id above is
SCENE-ANCHORED: a gap is named here because the tour puts the missing
door in view — a stop blocked, a stop degraded, or the generic ladder
every scene is held to (which was G11's anchor, and is why closing it
moved no mark) — and the partition
just above means something only because that holds. So library debt no
scene exercises gets no id on this page — a curated door with no
Python spelling, in a family the tour never walks through, is
invisible to this measurement however plainly it is missing. That
surface is enumerated by the BINDING CENSUS instead
(`crates/pncad-py/tests/test_binding_census.py`), which asks the other
question — can a caller REACH this door? — over the façade's curated
lists, name by name, and fails when one is neither bound nor listed.
It cites the ids above wherever one owns the door, and owns its own
`B-*` ids where none does. The two lists together are the debt;
reading only this page under-counts it.

## Closed gaps

| # | gap | closed by | register / pointer | what is true now |
|---|---|---|---|---|
| G1 | **Arcs and circles in profiles** | LIB-PYG1 | register B ("the big three"); `docs/PATHS-DESIGN.md` §2/§2a/§2b + LIBRARY-DESIGN §L4 | The PATHS lattice is bound state for state: each state is its own class exposing only its legal continuations, so an off-lattice call is an `AttributeError` (and a `ty` error) rather than a runtime surprise, and every verb crosses into the same Rust machinery, so refusals fire at the call site as the same typed `PathError`. `Node.profile` builds the document node from the loop's RECORDED program. Four stops flipped to YES against the scenes' own oracles (`bracket`, `vase`, `sheave`, `bossplate`); `rocker` and `diepips` re-partitioned to G9 and G7, which is what they were always waiting on second. Residue: Expr-bearing profile steps from Python (a parametric arc radius) are still unbound. G9 has since closed, so this is now the WHOLE of what blocks `plate_param`-from-Python — see the note under G10. LIB-PYSEL added a second customer for the same missing Expr door, without changing what it blocks: `GeomPred.datum_distance`'s comparand crosses as a `Length` literal, so a selection rule written against a NAMED parameter (the whole point of the Rust field being an `Expr`, SELECT-DESIGN §5) waits on the same door |
| G3 | **Non-xy sketch planes** | LIB-PYG23A | register B ("the big three"); `crates/profile/src/lib.rs` (`SketchPlane`) + LIBRARY-DESIGN §L3/§L4 | The sketch plane crosses as a VALUE. Rust gained two additive canonical constructors, `SketchPlane::yz()` and `SketchPlane::zx()`, beside `xy()` — the cyclic frames x→y→z→x that the tour's letterform captions already spoke — and Python binds all three plus the general `from_frame(origin, u, v)`. `Node.polygon` and `Node.profile` take `plane=`, mutually exclusive with `elevation=` (both is a `TypeError`), and both lower through the one `from_frame` seam `elevation` already used, so there is a single place a sketch plane is constructed. Rigidity stays the kernel's unchecked convention, stated in the stub and untested by any Python-side predicate: one semantics, two host languages. Five stops flipped against the scenes' own dyadic oracles — `silhouette` (4.5078125), `silhouette3` (2.798095703125) and its three shadow stops, which are the SAME body viewed down a different axis and are asserted as such. `az` re-partitioned to G9, the multi-loop counter it was always waiting on second |
| G2's loft half | **Loft** | LIB-PYG23A | register B ("the big three"); `editor-core/src/node.rs` (`Node::Loft`, M5 PR 10) + `tests/corpus/loft_prism.rs` | `Node.loft(profiles, v_degree)` binds the document node that already existed: sections are NodeIds in skin order, `v_degree` an int crossing as `Expr::count` — the corpus twin's exact form. No placement argument, because the document design puts placement on each section's own sketch plane. Nothing is pre-checked: too few sections, a degree outside `1 ≤ d ≤ n − 1`, non-corresponding loops all refuse as the kernel's typed `LoftError` family through the existing `skin`/`loft` tags. `loft_prism` and `nonuniform_loft` flip against the derived closed forms (9 m³; 8 + 0.25/(t(1−t)) = 9.7219015 m³) bracketed by the certified pad. RESIDUE, measured not guessed: `nonuniform_loft`'s actual subject is the v-parameterization the skin CHOSE, and `sweep::loft_parameters` is not cheaply reachable — it takes `&[Section]` and `&[Affine3]`, kernel values with no Python vocabulary, and the document layer cannot supply them either because a Loft node evaluates to a `Body` and drops `LoftGeometry::section_params`. The row asserts the volume and names the residue (the LIB-PYG1 m3 precedent). What is left of G2 is banked, not unbound — see the gap list |
| G4 | **Fillet node** | LIB-PYBUNDLE | register B; `editor-core/src/node.rs` (`Node::Fillet`, M5 PR 12 + M6-5's selection) + LIB-U7's materializers | `Node.fillet(target, radius, selection)` binds the node that already existed. The selection is edge names as TEXT — the names' own serde encoding, so a name is ONE vocabulary across Rust, Python and the file. The relation to a saved document is VALUE equality, not byte equality: `save` pretty-prints and the binding writes compact, so the two texts differ in whitespace and parse to the same JSON value, and a name taken from either round-trips through the other. **The text is OPAQUE BY CONTRACT**: it is a stable identifier, its internal structure is not API and may change without notice, and the supported operations are equality, ordering, storage and handing it back to `Node.fillet`. Reading inside a name is representation-dependence, not a selector — which is why G13 stayed open past this unit and why `diecomposed` was YES\* until LIB-PYSEL bound the selector doors (see G13 below). `Evaluation.all_edges` and its three siblings are where a name comes from, and they MATERIALIZE: the answer is as of that evaluation, the caller stores it, and the recipe's selection is frozen from then on — a live "all edges" would silently grow under an upstream edit, which is the staleness the freeze exists to prevent. Construction goes through Rust's `Node::fillet`, so the stored set is canonical and two recipes that select the same edges are bit-identical whatever order Python listed them in (asserted with `Doc.bit_eq`). Nothing is pre-checked beyond the text being a name at all: an empty selection, an unresolvable name, a tangential edge all refuse typed at evaluate. `diefillet` flips against the scene's own closed form. `diecomposed` re-graded to YES\* under G13 — the fillet node is not what it was waiting on second |
| G6 | **Split** | LIB-PYBUNDLE | register B; `editor-core/src/node.rs` (`Node::Split`) + `tests/corpus/cut_cylinder.rs` | `Node.split(target, tool)` binds the node, and `Node.datum_plane(origin, normal)` binds the datum it cuts with — the last of `Datum`'s three arms Python was missing. The value is a SPLIT and says so: `Value.split()` (already bound) answers `(above, below)` with `None` for an empty side, and `Value.body()` refuses rather than picking one. `tiltedcut` flips against the scene's own oracle, which is a BRACKET and not an equality — the exact half-volume πr²h/2 must lie inside the certified enclosure the mass-properties door answers with, and it does for both halves. `cutaway` re-partitioned to G14: its cut was refused by the naming emitter, not missing a node (G14 since closed) |
| G7 | **Rigid placement** | LIB-PYBUNDLE | register B; `editor-core/src/node.rs` (`Node::Transform`) + `tests/corpus/die_pips.rs` | `Node.transform(input, translation, rotation_axis, rotation_angle)`, the kernel's convention unchanged: rotate about the axis THROUGH THE WORLD ORIGIN, then translate. A pure translation still names an axis and a zero angle — a zero-length axis refuses (`degenerate_direction`) rather than being read as "no rotation", which is the fail-loud reading. `diepips` flips OUTRIGHT, structure and all: one ball, twenty-one placements whose pole rides the face normal, the twenty-one fused into a single tool, and ONE subtract — the scene's own group operation, not a re-authoring — against `sweep/tests/m5_pr12_die.rs`'s cube-less-twenty-one-caps oracle. `crosslap_exploded` stops being YES\*: the lift is the scene's statement now |
| G9 | **Multi-loop profiles** | LIB-PYBUNDLE | register B; `editor-core/src/node.rs` (`ProfileProgram.loops`) | `Node.profile` takes one loop OR a list of them, stubbed as an `@overload` pair, lowering through the same one seam. Validation stays kernel-side and untouched: which loop is outer, whether the holes nest, whether two loops cross is `Profile::validate`'s work, reaching Python as a typed `profile_program_refused` at `insert` (the edit door's replay probe) — the binding's only job is that the loops arrive in the order they were written. `plate` flips against a derived closed form (a rectangle less two circles, times the depth) and `az` against the scene's own exact 880383/327680. `rocker` re-partitioned to G12: its holes were never the harder half — its OUTLINE is |
| G10 | **Named document parameters** | R1-PARAMS | register A **R1** (was "the significant one" / "highest-value single residual") — **DISCHARGED**; guide §3.2's `compile_fail` pin is now that section's passing doctest | `ParamName` and `DocParam` are curated through `pncad::document` (and the prelude), and `DocEdit.set_doc_param` is bound with them — so the parametric flagship `plate_param` is authorable façade-only (guide §3.2's doctest authors it) and its one-edit-moves-both-holes claim is executed from Python in `test_north_star.py` against the Rust rows' analytic oracle. Residue, RE-STATED now that G9 has closed (LIB-PYBUNDLE §4.4): the three-loop profile is sayable and so are the circles, so exactly ONE door still blocks authoring `plate_param` from scratch in Python — a profile step whose argument is an EXPRESSION rather than a literal. Its holes are `LoopProgram::Circle { centre, radius: Expr::param("hole_r") }`, and `pncad.circle(centre, radius)` takes a `Length`, so the radius crosses as a number and the parameter link is lost. That is G1's recorded residue, unchanged in substance and now unaccompanied: nothing else is missing. The Python test therefore still loads the document through the persistence door, pinned line-for-line by `crates/pncad/tests/all.rs` (all but the snapshot's ε line, which CI's tolerance sweep varies by design) |

| G12 | **Corner-fillet loop building** | LIB-LBRET | register B; issue **#377** (the `LoopBuilder` retirement conversation, ratified on #386); `docs/PATHS-DESIGN.md` §2b (the LB10 revisit) | The wall was never a bindings omission: PATHS-DESIGN §2b's third ratified wall refused a STRAIGHT arrival off an ARC departure, so `rocker`'s outline could not migrate to the lattice in RUST either, and the raw `LoopBuilder::fillet_corner` surface it used was a second authoring vocabulary nobody wanted to bind. Route 3 (ratified on #386) gave that arrival its own door, and the §2c re-spell then dissolved the whole compound register into the FUSED family (`fillet_arc` / `arc_fillet` / `arc_fillet_arc`): an arc and the fillet that trims it are one authoring act, so they are one call, and the compound `Decide + Bounds` bound sits on those verbs alone. The outline then migrated under the ratified LB4/LB5 dispositions (oracle equality, not byte-identity: derived corners land 0–4 ulps off the anchors a hand author would have transcribed; the mid-arc seam RE-ANCHORS onto the keel, so the hub arc is one segment and the solid carries one fewer lateral face), and `LoopBuilder` left the `profile` crate's public API entirely for test support. `rocker` flips against the scene's own oracle: the eye is a HOLE, so the volume is the outline's prism less the eye's, and the census is the tour's exact 26 vertices / 39 edges / 15 faces at genus 1 — a far-pocket S8 pick or a lost seam vertex moves it |
| G13 | **Selectors** | LIB-PYSEL | register B; `docs/SELECT-DESIGN.md` §§1–2; LIB-U7 (structural) + LIB-SEL1 (geometric) | The narrowing surface crosses verb for verb: `Evaluation.select` (a `Selector` union of `NamePat` role-path shapes — `SegPat` tag/group/side/sub-name prefix) and `Evaluation.select_where` (a `GeomPred` conjunction over the survivors), answering in the SAME opaque-text alphabet the materializers speak and `Node.fillet` reads — so the ordinal-28 contract is kept, not softened: name text stays an identifier, and the binding is its one licensed reader (`Selector.matches` classifies a materialized text; nothing user-side parses one). The exact/decided split crosses as TYPED structure, no boolean flattening: the kind atoms (`curve_kind`, `surface_kind`, `adjacent_kinds`) are total tag reads that cannot refuse, while `datum_distance(datum, Cmp, Length)` is the funnel-decided atom whose in-band candidate, disagreeing tied name, or unreadable candidate raises the typed `SelectRefusal` (`reason` + payload attributes) exactly as Rust's `SelectRefusal` refuses — never a silent include or drop. `diecomposed` flips YES\*→YES on the scene's own statement: `test_north_star.py::TestDiecomposed` runs the SAME two filters `lib_sel1_geoselect.rs:507-560` runs — carrier kind `Line` for the twelve box edges, `Plane`/`Sphere` adjacency for the 42 pip-rim arcs — through two in-place fillets against the closed form the Rust scene meters (V = 0.952915 m³, Steiner blank − 21·(cap + rim-torus extra), at 1e-9 relative). Deliberately NOT bound, stated: `TagPat`/`Side` are Rust constructor plumbing (`SegPat.tag`/`group`/`any` and the side-vocabulary union cover them), the kind-SET types cross as `kind | list[kind]` arguments, and SEL2's detect/declare protocol stayed G5's slice — closed next, below |
| G5 | **Declared flush contact** | LIB-PYG5 | register B; register A **R3** DISCHARGED (the SEL2 refusal-menu wiring — `NodeErrorKind::UndeclaredContact`); `docs/SELECT-DESIGN.md` §3 | The detect/declare protocol crosses whole, and the refusal is the menu: an undeclared touching boolean raises `EvaluationError` with `kind == "undeclared_contact"` and the candidate declaration attached as a typed `FlushFinding` (`finding` attribute) — the pair by stable name in the one opaque alphabet, the verify door's own relation, no re-detection on the error path (the raise site keeps its pair; register R3's exact shape). `Evaluation.find_flush_candidates(a, b)` is the detect arm — the C4 verifier in candidate-generation mode, so a finding cannot disagree with verify-at-use — and `Node.declare(findings)` / `Doc.declare(finding)` / `Doc.declare_all(findings)` are the declare arm feeding `Node.boolean`'s `declare=`. Detection and declaration stay separate doors (the ruled no-fusion boundary), findings are typed values (`relation`/`class_`/`rung` mirrors), and in-band pairs refuse (`pair_in_band`) rather than being included or dropped. `table` flips on the corpus's own protocol authoring (per-leg detect → inspect 2/4/5/7 findings → declare_all → union; volume 4.0, area 35.5, both exact) and `crosslap` (glued) flips on the joint's mate (the five `SameOpposite` resting contacts; 1.875 exact) — with a MEASURED residue pinned in `TestCrosslapGlued`: declaring the merge-stage `SameOriented` bottom pairs glues kernel-side but fails in the document layer's naming emitter (`kind == "naming"`, the G14-adjacent emitter-coverage class), so the full-inventory declaration the Rust corpus uses on `table` is not yet expressible through the DOCUMENT layer on `crosslap`'s shape (the kernel-direct tour union declares everything and glues; only the N3 emission is missing) |
| G11 | **Tessellation and STL** | LIB-G11 | register B (named there as completing the ladder); `crates/pncad/src/prelude.rs`'s section 7 | The ladder's steps 4 and 5 are sayable from Python. `Body.tessellate(chordal)` is the free `mesh::tessellate` as a method on the value it takes — the posture `mass_properties` and the three validators already set — and δ crosses as a `Length`, because it is a DISTANCE and D6 has a type for that. It is deliberately not the kernel's ε, and the stub says so at the door. The `Mesh` it answers carries the value's two ratified contracts across: `positions` is the one shared buffer, so watertightness is checkable by INDEX equality and never by comparing coordinates, and `patch(i)` keeps per-face separability addressable. **The cross-check is the caller's own computation, on purpose**: `mesh::validate`'s `check_mesh` / `signed_volume` / `triangle_count` are NOT on the façade's curated lists (reachable only as `pncad::mesh::validate::*`), so binding them would have reached past the curation — and a divergence-theorem volume the caller writes over the bound triangles is a genuinely INDEPENDENT second measure of the same solid, which is what step 5 is for. `TestMeshCrossCheck` runs it against the exact measure on a planar body (agreement at rounding level, the triangulation being exact) and on a curved one (the error shrinking with δ). STL comes with it, as the two doors that answer the bytes rather than take a sink: `Mesh.to_stl_ascii` and `Mesh.to_stl_binary`, their option structs as keyword arguments and their two validated newtypes as the `str` those arguments take — an unrepresentable solid name or an ASCII-sniffing binary header refuses typed at the call. NO row moved, and that is the point of this gap's anchor: G11 was never a stop's blocker but the LADDER's, so what it bought is a check every YES row can now run rather than a mark. RESIDUE, measured: the picking chain does not cross (a patch's face and a boundary's edge are arena keys, so the per-edge boundary polylines are unbound and a patch is addressed by index), and `mesh::TessellateError` implements no `Display`, so its Python message is a `Debug` rendering — the tag is the branchable part, and the missing `Display` is the #380 shape one crate over |
| G15 | **The workspace store, content pins, and cross-document references** | LIB-G15 | register B; `crates/pncad/src/workspace.rs` + ASSEMBLY-DESIGN A4 (the identity/pin split); the ASM cross-program deposit at the tail of `docs/LIB-LOG.md`, which put this half FIRST in G18's stated order | The store crosses whole: `Workspace(path)` scans a directory of `*.pncad` files by their `id:` header alone, `documents()` is the identity -> path map, `create`/`resave` are the two write doors and there is no general mutation API — the Rust surface's own posture, unsoftened. Identity and version stay two vocabularies: `Doc.id` answers WHICH PART (32 hex digits, unchanged since G10's era), `ContentPin` answers WHICH VERSION (the SHA-256 of `canonical_bytes`, executed as such in `test_workspace.py` — `hashlib.sha256(canonical_bytes(doc)).hexdigest() == content_pin(doc).hex`), and `DocRef` pairs them. `Workspace.resolve` is A4's Cargo.lock semantics with nothing softened: a document edited under a reference REFUSES (`WorkspaceError`, `variant == "pin_mismatch"`, carrying `wanted` and `found` and ending on the library's own `PIN_MISMATCH_RECOURSE`) rather than resolving to the new content, and `current_pin` is the door that says what the new version is. Every store refusal is one typed `WorkspaceError` whose arm payload rides as attributes present on every arm — `path`, `id`, `first`, `second`, `wanted`, `found` — so handling reads `err.wanted` without first branching on `variant`. `random_document_id` and `header_document_id` land beside them. **This closed NO row, deliberately**: the store was never any scene's primary blocker, and rows 46/47 stay NO on G18 alone — the sentence this row was written around, *"a Python author can produce two documents a workspace will accept side by side, and cannot then assemble them"*, is now true in its FIRST half and blocked in its second by `InstantiatePart` alone — LIB-G18a spent the resolver half of that blockage, and the rows did not move, because neither closure is about authoring. RESIDUE, measured and moved with its reason: `update_to_store` (and the document layer's `update_references` / `mixed_pins` / `UpdateError` / `PinMultiplicity` / `PinSites`) is NOT bound and is now cited to G18, because what it moves is a pin AT ITS SITES and a site is an `InstantiatePart` node's `DocRef` — on any document Python can author it would answer the "referenced nowhere" refusal and nothing else |

## How to read this page next quarter

Run both halves of the verification. The Python suite is the one that
checks what the rows SAY:

```console
$ ./crates/pncad-py/run-python-tests.sh
```

If `test_the_named_gaps_are_still_gaps` fails, someone built a door —
promote the rows it unblocks. If a YES row's oracle fails, either the
scene changed or the bindings regressed, and the audit is the thing
that noticed.

The Rust guards are the ones that check the page's SHAPE — that there
is a row per scene at all, and that the numbers are the rows' own:

```console
$ cargo test -p pncad --test all north_star
```

If `the_north_star_audit_has_a_row_for_every_tour_stop` fails, the
tour moved: it names the stops with no row (grade each and add one) or
the rows with no stop (delete them). If
`the_north_star_audits_tallies_are_derived_from_its_rows` fails, a
count above stopped agreeing with the table — re-count, never
re-word. The three failures those two guards exist for are named in
the parenthetical at the top of this page; all three were found by
reading, months late, which is why they are tests now.
