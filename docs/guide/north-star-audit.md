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

**Result: 25 of the 34 tour stops are authorable through Python
today** — 22 outright, and 3 more only if you re-author by hand what
the scene says structurally. 9 are blocked by a missing door.

The count moved from 7 to 11 when LIB-PYG1 bound the PATHS lattice
and closed G1 (`bracket`, `vase`, `sheave`, `bossplate`), from 11 to
18 when LIB-PYG23A bound the sketch-plane vocabulary and the loft
node — closing G3 outright (`silhouette`, `silhouette3` and its three
shadow stops) and G2's loft half (`loft_prism`, `nonuniform_loft`) —
and from 18 to 24 when LIB-PYBUNDLE bound the fillet, split,
transform and datum-plane nodes and grew a profile past one loop,
closing G4, G6, G7 and G9, and from 24 to 25 when LIB-LBRET built
PATHS-DESIGN §2b's route-3 door and migrated `rocker`'s outline to
the lattice, closing G12. Five stops flipped NO to YES (`plate`,
`diefillet`, `diepips`, `tiltedcut`, `az`), `crosslap_exploded`
stopped being a YES\* — its lift is a `Node.transform` now, not a
hand-authored copy — and `diecomposed` went NO to YES\*. LIB-PYSEL
then bound the selector surface and closed G13: `diecomposed` is a
plain YES, its two blends narrowed by the SAME two geometric filters
the Rust scene runs, with no name text read.

Three of LIB-PYBUNDLE's stops did NOT flip outright, and each named
the door it was actually waiting on rather than inheriting the one
that closed: `rocker` (G12, since closed), `diecomposed` (G13, since
closed), `cutaway` (G14). G14 is a measured, executed refusal in
`test_north_star.py`. G13 never was a refusal and the page never
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

## The bound surface

Everything Python can say about geometry, in full:

- The **PATHS authoring lattice** — `Open`, `Start`, `circle`,
  `circle_split`, and one class per lattice state (`PathOpen`,
  `PathPoint`, `PathDirectedPoint`, `PathAngle`, `PathDirected`),
  each exposing only its legal continuations. The full current verb
  vocabulary: `at`, `at_on`, `angle`, `toward`, `tangent`, `turn`,
  `to`, `to_on`, `line`, `line_to`, `arc_to`, `arc_via`,
  `arc_center`, `arc_continue`, `tangent_arc_to`, `at_toward`,
  `fillet`.
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
  `Declare` node the boolean consumes. Nothing in Python can BUILD
  one yet, so the argument is presently only reachable for a
  declaration authored in Rust and loaded — which is why G5 is still
  a gap.
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

Plus evaluation, validation, mass properties, STEP export/import,
persistence, and typed quantities. That is the whole vocabulary.

## The audit

Every NO row names its **gap id**, which is the row's pointer: the id
resolves in the gap list below, and each gap there points onward to
the LIB residual register (`docs/LIB-LOG.md`, "LIB residual register",
category B) and to any design doc or register item that owns it. The
`gap` column is the row's *primary* blocker — the most fundamental
missing door — so the ids partition the 10 NO rows exactly; secondary
blockers stay named in the last column.

| # | scene | Python? | gap | the missing door |
|---|---|---|---|---|
| 1 | `bracket` | **YES** | — | — |
| 2 | `plate` | **YES** | — | — |
| 3 | `vase` | **YES** | — | — |
| 4 | `sheave` | **YES** | — | — |
| 5 | `chute` | **YES** | — | — |
| 6 | `rocker` | **YES** | — | — |
| 7 | `diefillet` | **YES** | — | — |
| 8 | `diepips` | **YES** | — | — |
| 9 | `diecomposed` | **YES** | — | — |
| 10 | `lily` (8 bodies) | NO | G2 | tube/sweep, both banked (below); placement (G7) |
| 11 | `tiltedcut` | **YES** | — | — |
| 12 | `bossplate` | **YES** | — | — |
| 13 | `loft_prism` | **YES** | — | — |
| 14 | `nonuniform_loft` | **YES** | — | body transfers; the scene's `loft_parameters` read-back does not — a named residue, not a gap |
| 15 | `s_duct` | NO | G2 | sweep is BANKED, not unbound: `wire_sweep` refuses unconditionally (`SWEEP_FRONTIER`) |
| 16 | `twisted_duct` | NO | G2 | same `SWEEP_FRONTIER` bank; a non-planar spine also needs the 3-D-path tail (U4/LQ3) |
| 17 | `twisted_duct_shadow_z` | NO | G2 | same body — the `SWEEP_FRONTIER` bank |
| 18 | `twisted_duct_shadow_y` | NO | G2 | same body — the `SWEEP_FRONTIER` bank |
| 19 | `tube_along_arc` | NO | G2 | no `Node::Tube` at all: a new node kind is a SCHEMA-VERSION break, and ASM-1 owns the next bump (v5) |
| 20 | `die` | **YES** | — | — |
| 21 | `table` | NO | G5 | declared flush contact |
| 22 | `silhouette` | **YES** | — | — |
| 23 | `silhouette3` | **YES** | — | — |
| 24 | `silhouette3_shadow_z` | **YES** | — | same body as row 23 (a camera, not a construction) |
| 25 | `silhouette3_shadow_x` | **YES** | — | same body as row 23 |
| 26 | `silhouette3_shadow_y` | **YES** | — | same body as row 23 |
| 27 | `az` | **YES** | — | — |
| 28 | `crosslap` (glued) | NO | G5 | declared flush contact — the mate refuses undeclared |
| 29 | `crosslap_exploded` | **YES** | — | — |
| 30 | `projectbox` | **YES** | — | — |
| 31 | `cutaway` | NO | G14 | the split and the two placements now say themselves; the CUT does not — a plane crossing boolean-minted faces refuses in the naming emitter |
| 32 | `heatsink5` | YES\* | G8 | pattern node + structural-param edit |
| 33 | `heatsink7` | YES\* | G8 | pattern node + structural-param edit |
| 34 | `heatsink9` | YES\* | G8 | pattern node + structural-param edit |

**YES** = the exact body is reproducible with the bound surface.
**YES\*** = the exact body is reproducible, but only by hand-authoring
what the scene expresses structurally — so the *body* transfers and
the *point of the scene* does not. `heatsink` hand-authors each fin
where the scene says one pattern. (`diecomposed` was this mark's
other case — hand-narrowing a materialized name set where the scene
says one selector — until LIB-PYSEL bound the selector.)

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

Rows 24–26 are the same body as row 23 seen down a different axis, so
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
| G2 | **Sweep and tube** (loft closed) | 6 | register B ("the big three") | Loft left this gap when LIB-PYG23A bound `Node.loft`. What remains is not an unbound door but two BANKED ones. **Sweep**: `wire_sweep` refuses unconditionally (`SWEEP_FRONTIER`, `editor-core/src/eval/wire.rs`) — the path-composition lane banked past M6 by the PR 10 MAJ ruling, so binding it would flip no row and un-banking is kernel-side. **Tube**: there is no `Node::Tube` at all, and adding a node kind is a schema-version break (the v3 precedent was exactly Loft/Sweep landing) whose next bump ASM-1 owns (v5, `docs/ASM-1-SPEC.md` §D-6). The tube/sweep/3-D-path tail is a design conversation: U4's measured spec, and **LQ3, ratified 2026-08-10 (#362, LIBRARY-DESIGN §L7)** — which names the discharge site rather than building it. A `geom-curves` chain→curve composition door is what would narrow `wire_sweep`'s refusal from everything to genuinely-unjoinable chains, and that un-banking is kernel-side work needing the kernel program's concurrence. Ratified direction, landed door not yet: rows 15–18 stay NO until U4's units land, and `Node::Tube`'s schema bump stays a separate coordination item with ASM's version sequence |
| G5 | **Declared flush contact** | 2 | register B; register A **R3** (the SEL2 `UndeclaredContact` refusal-menu wiring) | `Node.boolean` grew `declare=` in LIB-PYBUNDLE — the DATA door — but nothing in Python can BUILD a declaration: `Node.declare` does not exist and the detect protocol (`find_flush_candidates` → `declare_node`) is entirely unbound, so parts that *touch* still cannot be glued from Python |
| G8 | **Pattern + structural params** | degrades 3 | register B | No pattern node and no `SetStructuralParam` edit, so `heatsink`'s actual subject — one recipe, a count edit, memoized recompute — cannot be said. MEASURED at LIB-PYBUNDLE and deliberately left unbound: binding `Node::Pattern` would flip no row, because the heatsink's shape is a pattern UNIONED into a base and a pattern evaluates to an `Instances` payload, which the boolean's operand door refuses (`wrong_operand`, `eval/wire.rs::body_operand`). The gap is the kernel payload, not the binding; `test_the_named_gaps_are_still_gaps` executes the refusal on the one plural payload Python can already produce |
| G14 | **Split across boolean-minted faces** | 1 | register B (new); issue **#380** carries the `NamingError`-`Display` diagnostic gap this refusal hides behind | KERNEL-side, and measured from Python: `Node::Split` names a cut through PASS-THROUGH faces fine (`tiltedcut` flips on it), but a plane crossing a face the boolean itself minted refuses in the naming emitter (`NodeErrorKind::Naming`). `topo::split` does the geometry — the tour's `cutaway` runs it — so the missing thing is the split emitter's coverage of boolean provenance, not a document node. A cut through a MULTI-LOOP extrude's holes names fine, so the discriminator is provenance and not section-face topology |

G2, G5 and G14 partition the 9 NO rows: 6 + 2 + 1 = 9, counted off
the table above (G2: rows 10, 15–19; G5: 21, 28; G14: 31). G8
degrades three, which is what makes three rows YES\* rather than YES
(32, 33, 34). Authorable = 22 outright + 3 YES\* = 25, and
25 + 9 = 34.

Two counts in this list were off by one before LIB-PYG1 recounted
them: G1 read 7 stops against 6 table rows, and G6 read 1 against 2.
The rows are the record and the tallies are derived — every number
above is counted off the table just now, never carried forward from
the previous revision.

One further gap blocks no tour scene but matters to the library:

| # | gap | register / pointer | why it matters |
|---|---|---|---|
| G11 | **Tessellation and STL** | register B (named there as completing the ladder) | No mesh door, so Python loses steps 4 and 5 of the guide's ladder — no tessellate, and therefore no mesh-vs-exact cross-check. Step 6 does work: `Evaluation.step_string` exports STEP, and re-importing it is the strongest check available from Python |

## Closed gaps

| # | gap | closed by | register / pointer | what is true now |
|---|---|---|---|---|
| G1 | **Arcs and circles in profiles** | LIB-PYG1 | register B ("the big three"); `docs/PATHS-DESIGN.md` §2/§2a/§2b + LIBRARY-DESIGN §L4 | The PATHS lattice is bound state for state: each state is its own class exposing only its legal continuations, so an off-lattice call is an `AttributeError` (and a `ty` error) rather than a runtime surprise, and every verb crosses into the same Rust machinery, so refusals fire at the call site as the same typed `PathError`. `Node.profile` builds the document node from the loop's RECORDED program. Four stops flipped to YES against the scenes' own oracles (`bracket`, `vase`, `sheave`, `bossplate`); `rocker` and `diepips` re-partitioned to G9 and G7, which is what they were always waiting on second. Residue: Expr-bearing profile steps from Python (a parametric arc radius) are still unbound. G9 has since closed, so this is now the WHOLE of what blocks `plate_param`-from-Python — see the note under G10. LIB-PYSEL added a second customer for the same missing Expr door, without changing what it blocks: `GeomPred.datum_distance`'s comparand crosses as a `Length` literal, so a selection rule written against a NAMED parameter (the whole point of the Rust field being an `Expr`, SELECT-DESIGN §5) waits on the same door |
| G3 | **Non-xy sketch planes** | LIB-PYG23A | register B ("the big three"); `crates/profile/src/lib.rs` (`SketchPlane`) + LIBRARY-DESIGN §L3/§L4 | The sketch plane crosses as a VALUE. Rust gained two additive canonical constructors, `SketchPlane::yz()` and `SketchPlane::zx()`, beside `xy()` — the cyclic frames x→y→z→x that the tour's letterform captions already spoke — and Python binds all three plus the general `from_frame(origin, u, v)`. `Node.polygon` and `Node.profile` take `plane=`, mutually exclusive with `elevation=` (both is a `TypeError`), and both lower through the one `from_frame` seam `elevation` already used, so there is a single place a sketch plane is constructed. Rigidity stays the kernel's unchecked convention, stated in the stub and untested by any Python-side predicate: one semantics, two host languages. Five stops flipped against the scenes' own dyadic oracles — `silhouette` (4.5078125), `silhouette3` (2.798095703125) and its three shadow stops, which are the SAME body viewed down a different axis and are asserted as such. `az` re-partitioned to G9, the multi-loop counter it was always waiting on second |
| G2's loft half | **Loft** | LIB-PYG23A | register B ("the big three"); `editor-core/src/node.rs` (`Node::Loft`, M5 PR 10) + `tests/corpus/loft_prism.rs` | `Node.loft(profiles, v_degree)` binds the document node that already existed: sections are NodeIds in skin order, `v_degree` an int crossing as `Expr::count` — the corpus twin's exact form. No placement argument, because the document design puts placement on each section's own sketch plane. Nothing is pre-checked: too few sections, a degree outside `1 ≤ d ≤ n − 1`, non-corresponding loops all refuse as the kernel's typed `LoftError` family through the existing `skin`/`loft` tags. `loft_prism` and `nonuniform_loft` flip against the derived closed forms (9 m³; 8 + 0.25/(t(1−t)) = 9.7219015 m³) bracketed by the certified pad. RESIDUE, measured not guessed: `nonuniform_loft`'s actual subject is the v-parameterization the skin CHOSE, and `sweep::loft_parameters` is not cheaply reachable — it takes `&[Section]` and `&[Affine3]`, kernel values with no Python vocabulary, and the document layer cannot supply them either because a Loft node evaluates to a `Body` and drops `LoftGeometry::section_params`. The row asserts the volume and names the residue (the LIB-PYG1 m3 precedent). What is left of G2 is banked, not unbound — see the gap list |
| G4 | **Fillet node** | LIB-PYBUNDLE | register B; `editor-core/src/node.rs` (`Node::Fillet`, M5 PR 12 + M6-5's selection) + LIB-U7's materializers | `Node.fillet(target, radius, selection)` binds the node that already existed. The selection is edge names as TEXT — the names' own serde encoding, so a name is ONE vocabulary across Rust, Python and the file. The relation to a saved document is VALUE equality, not byte equality: `save` pretty-prints and the binding writes compact, so the two texts differ in whitespace and parse to the same JSON value, and a name taken from either round-trips through the other. **The text is OPAQUE BY CONTRACT**: it is a stable identifier, its internal structure is not API and may change without notice, and the supported operations are equality, ordering, storage and handing it back to `Node.fillet`. Reading inside a name is representation-dependence, not a selector — which is why G13 stayed open past this unit and why `diecomposed` was YES\* until LIB-PYSEL bound the selector doors (see G13 below). `Evaluation.all_edges` and its three siblings are where a name comes from, and they MATERIALIZE: the answer is as of that evaluation, the caller stores it, and the recipe's selection is frozen from then on — a live "all edges" would silently grow under an upstream edit, which is the staleness the freeze exists to prevent. Construction goes through Rust's `Node::fillet`, so the stored set is canonical and two recipes that select the same edges are bit-identical whatever order Python listed them in (asserted with `Doc.bit_eq`). Nothing is pre-checked beyond the text being a name at all: an empty selection, an unresolvable name, a tangential edge all refuse typed at evaluate. `diefillet` flips against the scene's own closed form. `diecomposed` re-graded to YES\* under G13 — the fillet node is not what it was waiting on second |
| G6 | **Split** | LIB-PYBUNDLE | register B; `editor-core/src/node.rs` (`Node::Split`) + `tests/corpus/cut_cylinder.rs` | `Node.split(target, tool)` binds the node, and `Node.datum_plane(origin, normal)` binds the datum it cuts with — the last of `Datum`'s three arms Python was missing. The value is a SPLIT and says so: `Value.split()` (already bound) answers `(above, below)` with `None` for an empty side, and `Value.body()` refuses rather than picking one. `tiltedcut` flips against the scene's own oracle, which is a BRACKET and not an equality — the exact half-volume πr²h/2 must lie inside the certified enclosure the mass-properties door answers with, and it does for both halves. `cutaway` re-partitioned to G14: its cut is refused by the naming emitter, not missing a node |
| G7 | **Rigid placement** | LIB-PYBUNDLE | register B; `editor-core/src/node.rs` (`Node::Transform`) + `tests/corpus/die_pips.rs` | `Node.transform(input, translation, rotation_axis, rotation_angle)`, the kernel's convention unchanged: rotate about the axis THROUGH THE WORLD ORIGIN, then translate. A pure translation still names an axis and a zero angle — a zero-length axis refuses (`degenerate_direction`) rather than being read as "no rotation", which is the fail-loud reading. `diepips` flips OUTRIGHT, structure and all: one ball, twenty-one placements whose pole rides the face normal, the twenty-one fused into a single tool, and ONE subtract — the scene's own group operation, not a re-authoring — against `sweep/tests/m5_pr12_die.rs`'s cube-less-twenty-one-caps oracle. `crosslap_exploded` stops being YES\*: the lift is the scene's statement now |
| G9 | **Multi-loop profiles** | LIB-PYBUNDLE | register B; `editor-core/src/node.rs` (`ProfileProgram.loops`) | `Node.profile` takes one loop OR a list of them, stubbed as an `@overload` pair, lowering through the same one seam. Validation stays kernel-side and untouched: which loop is outer, whether the holes nest, whether two loops cross is `Profile::validate`'s work, reaching Python as a typed `profile_program_refused` at `insert` (the edit door's replay probe) — the binding's only job is that the loops arrive in the order they were written. `plate` flips against a derived closed form (a rectangle less two circles, times the depth) and `az` against the scene's own exact 880383/327680. `rocker` re-partitioned to G12: its holes were never the harder half — its OUTLINE is |
| G10 | **Named document parameters** | R1-PARAMS | register A **R1** (was "the significant one" / "highest-value single residual") — **DISCHARGED**; guide §3.2's `compile_fail` pin is now that section's passing doctest | `ParamName` and `DocParam` are curated through `pncad::document` (and the prelude), and `DocEdit.set_doc_param` is bound with them — so the parametric flagship `plate_param` is authorable façade-only (guide §3.2's doctest authors it) and its one-edit-moves-both-holes claim is executed from Python in `test_north_star.py` against the Rust rows' analytic oracle. Residue, RE-STATED now that G9 has closed (LIB-PYBUNDLE §4.4): the three-loop profile is sayable and so are the circles, so exactly ONE door still blocks authoring `plate_param` from scratch in Python — a profile step whose argument is an EXPRESSION rather than a literal. Its holes are `LoopProgram::Circle { centre, radius: Expr::param("hole_r") }`, and `pncad.circle(centre, radius)` takes a `Length`, so the radius crosses as a number and the parameter link is lost. That is G1's recorded residue, unchanged in substance and now unaccompanied: nothing else is missing. The Python test therefore still loads the document through the persistence door, pinned line-for-line by `crates/pncad/tests/all.rs` (all but the snapshot's ε line, which CI's tolerance sweep varies by design) |

| G12 | **Corner-fillet loop building** | LIB-LBRET | register B; issue **#377** (the `LoopBuilder` retirement conversation, ratified on #386); `docs/PATHS-DESIGN.md` §2b (the LB10 revisit) | The wall was never a bindings omission: PATHS-DESIGN §2b's third ratified wall refused a STRAIGHT arrival off an ARC departure, so `rocker`'s outline could not migrate to the lattice in RUST either, and the raw `LoopBuilder::fillet_corner` surface it used was a second authoring vocabulary nobody wanted to bind. Route 3 (ratified on #386) gives that arrival its own door — `.at_toward(p, dx, dy)`, a sibling of `at_on`/`to_on` living in the same boundary file, so the compound `Decide + Bounds` bound stayed confined and the generic doors gained nothing. The outline then migrated under the ratified LB4/LB5 dispositions (oracle equality, not byte-identity: derived corners land 0–4 ulps off the anchors a hand author would have transcribed; the mid-arc seam RE-ANCHORS onto the keel, so the hub arc is one segment and the solid carries one fewer lateral face), and `LoopBuilder` left the `profile` crate's public API entirely for test support. `rocker` flips against the scene's own oracle: the eye is a HOLE, so the volume is the outline's prism less the eye's, and the census is the tour's exact 26 vertices / 39 edges / 15 faces at genus 1 — a far-pocket S8 pick or a lost seam vertex moves it |
| G13 | **Selectors** | LIB-PYSEL | register B; `docs/SELECT-DESIGN.md` §§1–2; LIB-U7 (structural) + LIB-SEL1 (geometric) | The narrowing surface crosses verb for verb: `Evaluation.select` (a `Selector` union of `NamePat` role-path shapes — `SegPat` tag/group/side/sub-name prefix) and `Evaluation.select_where` (a `GeomPred` conjunction over the survivors), answering in the SAME opaque-text alphabet the materializers speak and `Node.fillet` reads — so the ordinal-28 contract is kept, not softened: name text stays an identifier, and the binding is its one licensed reader (`Selector.matches` classifies a materialized text; nothing user-side parses one). The exact/decided split crosses as TYPED structure, no boolean flattening: the kind atoms (`curve_kind`, `surface_kind`, `adjacent_kinds`) are total tag reads that cannot refuse, while `datum_distance(datum, Cmp, Length)` is the funnel-decided atom whose in-band candidate, disagreeing tied name, or unreadable candidate raises the typed `SelectRefusal` (`reason` + payload attributes) exactly as Rust's `SelectRefusal` refuses — never a silent include or drop. `diecomposed` flips YES\*→YES on the scene's own statement: `test_north_star.py::TestDiecomposed` runs the SAME two filters `lib_sel1_geoselect.rs:507-560` runs — carrier kind `Line` for the twelve box edges, `Plane`/`Sphere` adjacency for the 42 pip-rim arcs — through two in-place fillets against the closed form the Rust scene meters (V = 0.952915 m³, Steiner blank − 21·(cap + rim-torus extra), at 1e-9 relative). Deliberately NOT bound, stated: `TagPat`/`Side` are Rust constructor plumbing (`SegPat.tag`/`group`/`any` and the side-vocabulary union cover them), the kind-SET types cross as `kind | list[kind]` arguments, and SEL2's detect/declare protocol stays G5's slice |

## How to read this page next quarter

Run the verification:

```console
$ ./crates/pncad-py/run-python-tests.sh
```

If `test_the_named_gaps_are_still_gaps` fails, someone built a door —
promote the rows it unblocks. If a YES row's oracle fails, either the
scene changed or the bindings regressed, and the audit is the thing
that noticed.
