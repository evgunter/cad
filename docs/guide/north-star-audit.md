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

**Result: 18 of the 34 tour stops are authorable through Python
today** — 14 outright, and 4 more only if you re-author by hand the
placement or patterning the scene uses. 16 are blocked by a missing
door.

The count moved from 7 to 11 when LIB-PYG1 bound the PATHS lattice
and closed G1 (`bracket`, `vase`, `sheave`, `bossplate`), and from 11
to 18 when LIB-PYG23A bound the sketch-plane vocabulary and the loft
node — closing G3 outright (`silhouette`, `silhouette3` and its three
shadow stops) and G2's loft half (`loft_prism`, `nonuniform_loft`).

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
  `arc_center`, `arc_continue`, `tangent_arc_to`, `fillet`.
- `SketchPlane` — the sketch plane as a VALUE: the named cyclic
  frames `xy` / `yz` / `zx` (normals +z / +x / +y), and the general
  `from_frame(origin, u, v)`. Rigidity is the kernel's own unchecked
  convention and the binding adds no predicate of its own, so a
  non-rigid frame is a well-defined skewed sketch rather than a
  refusal.
- `Node.profile(outline, elevation=…, plane=…)` — one closed loop
  from that lattice, on any sketch plane, built from the loop's
  recorded program. `plane=` and `elevation=` are mutually exclusive
  (`elevation` is the xy sugar); passing both is a `TypeError`.
- `Node.polygon(points, elevation=…, plane=…)` — the straight-segment
  shortcut, same plane story.
- `Node.extrude(profile, distance)` — along the sketch-plane normal,
  so the plane is what chooses the axis.
- `Node.revolve(profile, axis, angle)` about a `Node.datum_axis`.
- `Node.loft(profiles, v_degree)` — a skinned solid through two or
  more section profiles in skin order, at an integer v-degree. There
  is no placement argument: each section rides its own profile's
  sketch plane.
- `Node.boolean(op, a, b)` — union, intersect, subtract, with **no**
  declaration argument.
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
missing door — so the ids partition the 23 NO rows exactly; secondary
blockers stay named in the last column.

| # | scene | Python? | gap | the missing door |
|---|---|---|---|---|
| 1 | `bracket` | **YES** | — | — |
| 2 | `plate` | NO | G9 | multi-loop profile / holes |
| 3 | `vase` | **YES** | — | — |
| 4 | `sheave` | **YES** | — | — |
| 5 | `chute` | **YES** | — | — |
| 6 | `rocker` | NO | G9 | multi-loop profile (the two eyes are holes); arcs now author |
| 7 | `diefillet` | NO | G4 | no fillet node (`fillet_edges` on 12 edges) |
| 8 | `diepips` | NO | G7 | rigid placement (21 pips placed on six faces); group boolean; arcs now author |
| 9 | `diecomposed` | NO | G4 | no fillet node; plus everything blocking `diepips` |
| 10 | `lily` (8 bodies) | NO | G2 | tube/sweep, both banked (below); placement (G7) |
| 11 | `tiltedcut` | NO | G6 | no split node |
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
| 27 | `az` | NO | G9 | multi-loop profile (the A's counter is a hole); the yz sketch plane now authors |
| 28 | `crosslap` (glued) | NO | G5 | declared flush contact — the mate refuses undeclared |
| 29 | `crosslap_exploded` | YES\* | G7 | rigid placement (re-authorable: the lift is axis-aligned) |
| 30 | `projectbox` | **YES** | — | — |
| 31 | `cutaway` | NO | G6 | no split node; also rigid placement (G7) |
| 32 | `heatsink5` | YES\* | G8 | pattern node + structural-param edit |
| 33 | `heatsink7` | YES\* | G8 | pattern node + structural-param edit |
| 34 | `heatsink9` | YES\* | G8 | pattern node + structural-param edit |

**YES** = the exact body is reproducible with the bound surface.
**YES\*** = the exact body is reproducible, but only by hand-authoring
what the scene expresses structurally — so the *body* transfers and
the *point of the scene* does not.

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
| G9 | **Multi-loop profiles** | 3 | register B | A profile is one loop, so a plate with holes needs a boolean per hole. `rocker` joined `plate` here when G1 closed; `az` joined them when G3 closed, its yz sketch plane no longer the more fundamental blocker |
| G4 | **Fillet node** | 2 | register B | `fillet_edges` has no document node, so no edge blends from Python |
| G5 | **Declared flush contact** | 2 | register B; register A **R3** (the SEL2 `UndeclaredContact` refusal-menu wiring) | `Node.boolean` has no `declare` argument, so parts that *touch* cannot be glued — the detect/declare protocol (`find_flush_candidates` → `declare_node`) is entirely unbound |
| G6 | **Split** | 2 | register B | `topo::split` has no node |
| G7 | **Rigid placement** | 1, degrades 1 | register B | No transform node; bodies must be authored in place. `diepips` became its blocking stop when G1 closed. (A sketch PLANE is now sayable — G3 — but that places a sketch, not a finished body) |
| G8 | **Pattern + structural params** | degrades 3 | register B | No pattern node and no `SetStructuralParam` edit, so `heatsink`'s actual subject — one recipe, a count edit, memoized recompute — cannot be said |

G2, G4, G5, G6, G7 and G9 partition the 16 NO rows: 6 + 2 + 2 + 2 + 1
+ 3 = 16, counted off the table above (G2: rows 10, 15–19; G4: 7, 9;
G5: 21, 28; G6: 11, 31; G7: 8; G9: 2, 6, 27). G7 additionally degrades
one row and G8 degrades three, which is what makes four rows YES\*
rather than YES. Authorable = 14 outright + 4 YES\* = 18, and
18 + 16 = 34.

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
| G1 | **Arcs and circles in profiles** | LIB-PYG1 | register B ("the big three"); `docs/PATHS-DESIGN.md` §2/§2a/§2b + LIBRARY-DESIGN §L4 | The PATHS lattice is bound state for state: each state is its own class exposing only its legal continuations, so an off-lattice call is an `AttributeError` (and a `ty` error) rather than a runtime surprise, and every verb crosses into the same Rust machinery, so refusals fire at the call site as the same typed `PathError`. `Node.profile` builds the document node from the loop's RECORDED program. Four stops flipped to YES against the scenes' own oracles (`bracket`, `vase`, `sheave`, `bossplate`); `rocker` and `diepips` re-partitioned to G9 and G7, which is what they were always waiting on second. Residue: Expr-bearing profile steps from Python (a parametric arc radius) are still unbound — with G9, that is what would complete `plate_param`-from-Python |
| G3 | **Non-xy sketch planes** | LIB-PYG23A | register B ("the big three"); `crates/profile/src/lib.rs` (`SketchPlane`) + LIBRARY-DESIGN §L3/§L4 | The sketch plane crosses as a VALUE. Rust gained two additive canonical constructors, `SketchPlane::yz()` and `SketchPlane::zx()`, beside `xy()` — the cyclic frames x→y→z→x that the tour's letterform captions already spoke — and Python binds all three plus the general `from_frame(origin, u, v)`. `Node.polygon` and `Node.profile` take `plane=`, mutually exclusive with `elevation=` (both is a `TypeError`), and both lower through the one `from_frame` seam `elevation` already used, so there is a single place a sketch plane is constructed. Rigidity stays the kernel's unchecked convention, stated in the stub and untested by any Python-side predicate: one semantics, two host languages. Five stops flipped against the scenes' own dyadic oracles — `silhouette` (4.5078125), `silhouette3` (2.798095703125) and its three shadow stops, which are the SAME body viewed down a different axis and are asserted as such. `az` re-partitioned to G9, the multi-loop counter it was always waiting on second |
| G2's loft half | **Loft** | LIB-PYG23A | register B ("the big three"); `editor-core/src/node.rs` (`Node::Loft`, M5 PR 10) + `tests/corpus/loft_prism.rs` | `Node.loft(profiles, v_degree)` binds the document node that already existed: sections are NodeIds in skin order, `v_degree` an int crossing as `Expr::count` — the corpus twin's exact form. No placement argument, because the document design puts placement on each section's own sketch plane. Nothing is pre-checked: too few sections, a degree outside `1 ≤ d ≤ n − 1`, non-corresponding loops all refuse as the kernel's typed `LoftError` family through the existing `skin`/`loft` tags. `loft_prism` and `nonuniform_loft` flip against the derived closed forms (9 m³; 8 + 0.25/(t(1−t)) = 9.7219015 m³) bracketed by the certified pad. RESIDUE, measured not guessed: `nonuniform_loft`'s actual subject is the v-parameterization the skin CHOSE, and `sweep::loft_parameters` is not cheaply reachable — it takes `&[Section]` and `&[Affine3]`, kernel values with no Python vocabulary, and the document layer cannot supply them either because a Loft node evaluates to a `Body` and drops `LoftGeometry::section_params`. The row asserts the volume and names the residue (the LIB-PYG1 m3 precedent). What is left of G2 is banked, not unbound — see the gap list |
| G10 | **Named document parameters** | R1-PARAMS | register A **R1** (was "the significant one" / "highest-value single residual") — **DISCHARGED**; guide §3.2's `compile_fail` pin is now that section's passing doctest | `ParamName` and `DocParam` are curated through `pncad::document` (and the prelude), and `DocEdit.set_doc_param` is bound with them — so the parametric flagship `plate_param` is authorable façade-only (guide §3.2's doctest authors it) and its one-edit-moves-both-holes claim is executed from Python in `test_north_star.py` against the Rust rows' analytic oracle. Residue, stated plainly: Python still cannot author plate_param's *profile* from scratch (its circle loops are G1, its three-loop profile G9), so the Python test loads the document through the persistence door, pinned line-for-line by `crates/pncad/tests/all.rs` (all but the snapshot's ε line, which CI's tolerance sweep varies by design) |

## How to read this page next quarter

Run the verification:

```console
$ ./crates/pncad-py/run-python-tests.sh
```

If `test_the_named_gaps_are_still_gaps` fails, someone built a door —
promote the rows it unblocks. If a YES row's oracle fails, either the
scene changed or the bindings regressed, and the audit is the thing
that noticed.
