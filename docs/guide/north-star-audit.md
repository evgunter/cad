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

**Result: 7 of the 34 tour stops are authorable through Python today**
— 3 outright, and 4 more only if you re-author by hand the placement
or patterning the scene uses. 27 are blocked by a missing door.

Every YES row below is verified by executed Python in
`crates/pncad-py/tests/test_north_star.py`, which rebuilds the scene
and checks it against the *same exact volume oracle the Rust scene
asserts*. Every NO row's gap is asserted as an absence in the same
file, so **the day a gap closes, this audit fails and must be
updated**.

## The bound surface

Everything Python can say about geometry, in full:

- `Node.polygon(points, elevation=…)` — one closed loop of straight
  segments, on a plane parallel to the world xy-plane.
- `Node.extrude(profile, distance)` — along the sketch-plane normal,
  so always world +z.
- `Node.revolve(profile, axis, angle)` about a `Node.datum_axis`.
- `Node.boolean(op, a, b)` — union, intersect, subtract, with **no**
  declaration argument.
- Edits: `insert_node`, `delete_node`, `set_tolerance`.

Plus evaluation, validation, mass properties, STEP export/import,
persistence, and typed quantities. That is the whole vocabulary.

## The audit

| # | scene | Python? | the missing door |
|---|---|---|---|
| 1 | `bracket` | NO | profile needs arcs (`.fillet(0.5)` mints a tangent arc leg) |
| 2 | `plate` | NO | multi-loop profile / holes; also circles |
| 3 | `vase` | NO | profile needs arcs (`arc_via` belly) |
| 4 | `sheave` | NO | profile needs arcs (`arc_via` groove) |
| 5 | `chute` | **YES** | — |
| 6 | `rocker` | NO | profile needs arcs (hub circles, 6 fillets); multi-loop |
| 7 | `diefillet` | NO | no fillet node (`fillet_edges` on 12 edges) |
| 8 | `diepips` | NO | profile needs arcs; rigid placement; group boolean |
| 9 | `diecomposed` | NO | no fillet node; plus everything blocking `diepips` |
| 10 | `lily` (8 bodies) | NO | tube/sweep; arcs; non-xy planes; rigid placement |
| 11 | `tiltedcut` | NO | no split node; also circles |
| 12 | `bossplate` | NO | profile needs arcs (three-arc boss loop) |
| 13 | `loft_prism` | NO | no loft |
| 14 | `nonuniform_loft` | NO | no loft |
| 15 | `s_duct` | NO | no sweep along a path |
| 16 | `twisted_duct` | NO | no sweep along a path |
| 17 | `twisted_duct_shadow_z` | NO | same body — no sweep |
| 18 | `twisted_duct_shadow_y` | NO | same body — no sweep |
| 19 | `tube_along_arc` | NO | no tube/torus door |
| 20 | `die` | **YES** | — |
| 21 | `table` | NO | declared flush contact |
| 22 | `silhouette` | NO | non-xy sketch plane (T is a yz sketch extruded +x) |
| 23 | `silhouette3` | NO | non-xy sketch plane (T on yz, C on zx) |
| 24 | `silhouette3_shadow_z` | NO | same body — non-xy sketch planes |
| 25 | `silhouette3_shadow_x` | NO | same body — non-xy sketch planes |
| 26 | `silhouette3_shadow_y` | NO | same body — non-xy sketch planes |
| 27 | `az` | NO | non-xy sketch plane (Z on yz); multi-loop (A's counter hole) |
| 28 | `crosslap` (glued) | NO | declared flush contact — the mate refuses undeclared |
| 29 | `crosslap_exploded` | YES\* | rigid placement (re-authorable: the lift is axis-aligned) |
| 30 | `projectbox` | **YES** | — |
| 31 | `cutaway` | NO | no split node; also rigid placement |
| 32 | `heatsink5` | YES\* | pattern node + structural-param edit |
| 33 | `heatsink7` | YES\* | pattern node + structural-param edit |
| 34 | `heatsink9` | YES\* | pattern node + structural-param edit |

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

Ranked by how many stops each blocks first:

| # | gap | stops blocked | note |
|---|---|---|---|
| G1 | **Arcs and circles in profiles** | 9 | The single biggest blocker. `Node.polygon` is straight segments only, so every rounded part is out. The real fix is the PATHS lattice (`docs/PATHS-DESIGN.md` §5, LIBRARY-DESIGN §L4), whose stub classes are specified and unbuilt |
| G2 | **Loft, sweep, tube** | 7 | No node kind for any of the path-driven body ops |
| G3 | **Non-xy sketch planes** | 6 | `Node.polygon` takes only an `elevation`. Any part with features on two orthogonal faces is out |
| G4 | **Fillet node** | 2 | `fillet_edges` has no document node, so no edge blends from Python |
| G5 | **Declared flush contact** | 2 | `Node.boolean` has no `declare` argument, so parts that *touch* cannot be glued — the detect/declare protocol (`find_flush_candidates` → `declare_node`) is entirely unbound |
| G6 | **Split** | 1 | `topo::split` has no node |
| G7 | **Rigid placement** | degrades 1 | No transform node; bodies must be authored in place |
| G8 | **Pattern + structural params** | degrades 3 | No pattern node and no `SetStructuralParam` edit, so `heatsink`'s actual subject — one recipe, a count edit, memoized recompute — cannot be said |

Three further gaps block no tour scene but matter to the library:

| # | gap | why it matters |
|---|---|---|
| G9 | **Multi-loop profiles** | A profile is one loop, so a plate with holes needs a boolean per hole. Blocks `plate` and `az` |
| G10 | **Named document parameters** | `SetDocParam` needs `ParamName`/`DocParam`, and neither is bound (nor re-exported by the Rust façade — see guide §3.2). So the parametric flagship `plate_param` is unauthorable from Python **and** from `pncad`. This is the gap nearest the point of the whole switch |
| G11 | **Tessellation and STL** | No mesh door, so the guide's ladder stops at step 4 for Python: no tessellate, and no mesh-vs-exact cross-check. `Evaluation.step_string` is the only export |

## How to read this page next quarter

Run the verification:

```console
$ ./crates/pncad-py/run-python-tests.sh
```

If `test_the_named_gaps_are_still_gaps` fails, someone built a door —
promote the rows it unblocks. If a YES row's oracle fails, either the
scene changed or the bindings regressed, and the audit is the thing
that noticed.
