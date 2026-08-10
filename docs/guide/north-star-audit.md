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

**Result: 11 of the 34 tour stops are authorable through Python
today** — 7 outright, and 4 more only if you re-author by hand the
placement or patterning the scene uses. 23 are blocked by a missing
door.

The count moved from 7 to 11 when LIB-PYG1 bound the PATHS lattice
and closed G1 (`bracket`, `vase`, `sheave`, `bossplate`).

Every YES row below is verified by executed Python in
`crates/pncad-py/tests/test_north_star.py`, which rebuilds the scene
and checks it against the *same exact volume oracle the Rust scene
asserts*. Two of the four rows G1 unblocked are the honest exception:
`bracket` and `vase` are scenes the Rust tour holds only to its
generic ladder (validate, tessellate, mesh against mass properties)
and gives no closed form, so their Python rows derive one, state the
derivation, and assert it. Every NO row's gap is asserted as an
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
- `Node.profile(outline, elevation=…)` — one closed loop from that
  lattice, on a plane parallel to the world xy-plane, built from the
  loop's recorded program.
- `Node.polygon(points, elevation=…)` — the straight-segment
  shortcut, unchanged.
- `Node.extrude(profile, distance)` — along the sketch-plane normal,
  so always world +z.
- `Node.revolve(profile, axis, angle)` about a `Node.datum_axis`.
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
| 10 | `lily` (8 bodies) | NO | G2 | tube/sweep; non-xy planes (G3); placement (G7) |
| 11 | `tiltedcut` | NO | G6 | no split node |
| 12 | `bossplate` | **YES** | — | — |
| 13 | `loft_prism` | NO | G2 | no loft |
| 14 | `nonuniform_loft` | NO | G2 | no loft |
| 15 | `s_duct` | NO | G2 | no sweep along a path |
| 16 | `twisted_duct` | NO | G2 | no sweep along a path |
| 17 | `twisted_duct_shadow_z` | NO | G2 | same body — no sweep |
| 18 | `twisted_duct_shadow_y` | NO | G2 | same body — no sweep |
| 19 | `tube_along_arc` | NO | G2 | no tube/torus door |
| 20 | `die` | **YES** | — | — |
| 21 | `table` | NO | G5 | declared flush contact |
| 22 | `silhouette` | NO | G3 | non-xy sketch plane (T is a yz sketch extruded +x) |
| 23 | `silhouette3` | NO | G3 | non-xy sketch plane (T on yz, C on zx) |
| 24 | `silhouette3_shadow_z` | NO | G3 | same body — non-xy sketch planes |
| 25 | `silhouette3_shadow_x` | NO | G3 | same body — non-xy sketch planes |
| 26 | `silhouette3_shadow_y` | NO | G3 | same body — non-xy sketch planes |
| 27 | `az` | NO | G3 | non-xy sketch plane (Z on yz); multi-loop (G9) |
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
| G2 | **Loft, sweep, tube** | 8 | register B ("the big three") | No node kind for any of the path-driven body ops |
| G3 | **Non-xy sketch planes** | 6 | register B ("the big three") | `Node.polygon` takes only an `elevation`. Any part with features on two orthogonal faces is out |
| G4 | **Fillet node** | 2 | register B | `fillet_edges` has no document node, so no edge blends from Python |
| G5 | **Declared flush contact** | 2 | register B; register A **R3** (the SEL2 `UndeclaredContact` refusal-menu wiring) | `Node.boolean` has no `declare` argument, so parts that *touch* cannot be glued — the detect/declare protocol (`find_flush_candidates` → `declare_node`) is entirely unbound |
| G6 | **Split** | 2 | register B | `topo::split` has no node |
| G9 | **Multi-loop profiles** | 2 | register B | A profile is one loop, so a plate with holes needs a boolean per hole. `rocker` joined `plate` here when G1 closed; still a secondary blocker on `az` |
| G7 | **Rigid placement** | 1, degrades 1 | register B | No transform node; bodies must be authored in place. `diepips` became its blocking stop when G1 closed |
| G8 | **Pattern + structural params** | degrades 3 | register B | No pattern node and no `SetStructuralParam` edit, so `heatsink`'s actual subject — one recipe, a count edit, memoized recompute — cannot be said |

G2–G7 and G9 partition the 23 NO rows (8+6+2+2+2+1+2 for G2, G3, G4,
G5, G6, G7, G9); G7 additionally degrades one row and G8 degrades
three, which is what makes four rows YES\* rather than YES.

Two counts in this list were off by one before LIB-PYG1 recounted
them: G1 read 7 stops against 6 table rows and G6 read 1 against 2.
The rows were right and the tallies were wrong; the arithmetic above
is taken from the table.

One further gap blocks no tour scene but matters to the library:

| # | gap | register / pointer | why it matters |
|---|---|---|---|
| G11 | **Tessellation and STL** | register B (named there as completing the ladder) | No mesh door, so Python loses steps 4 and 5 of the guide's ladder — no tessellate, and therefore no mesh-vs-exact cross-check. Step 6 does work: `Evaluation.step_string` exports STEP, and re-importing it is the strongest check available from Python |

## Closed gaps

| # | gap | closed by | register / pointer | what is true now |
|---|---|---|---|---|
| G1 | **Arcs and circles in profiles** | LIB-PYG1 | register B ("the big three"); `docs/PATHS-DESIGN.md` §2/§2a/§2b + LIBRARY-DESIGN §L4 | The PATHS lattice is bound state for state: each state is its own class exposing only its legal continuations, so an off-lattice call is an `AttributeError` (and a `ty` error) rather than a runtime surprise, and every verb crosses into the same Rust machinery, so refusals fire at the call site as the same typed `PathError`. `Node.profile` builds the document node from the loop's RECORDED program. Four stops flipped to YES against the scenes' own oracles (`bracket`, `vase`, `sheave`, `bossplate`); `rocker` and `diepips` re-partitioned to G9 and G7, which is what they were always waiting on second. Residue: Expr-bearing profile steps from Python (a parametric arc radius) are still unbound — with G9, that is what would complete `plate_param`-from-Python |
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
