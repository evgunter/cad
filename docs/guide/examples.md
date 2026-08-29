# The corpus as the example set

This project does not keep a folder of toy snippets. The examples
*are* the test corpus: the demo tour renders real parts, and the
editor-core corpus documents are replayed and measured by CI. Both are
kept honest by assertions, so nothing here can drift away from the
kernel it documents.

Use this page as a lookup table. Find the row nearest your problem,
open that file, and read it — every one of them was written to
demonstrate something specific, and most also pin a pitfall that once
bit us.

## How to run them

```console
$ cd demos/tour && cargo run --release -- ../out    # all 47 stops
$ local-scripts/render-hosted.sh                           # + montage images (installs CI's render; see demos/README.md)
```

```console
$ cargo test -p editor-core --test all m4_pr8_corpus              # corpus rows
$ cargo test -p editor-core --test all --features interval        # certified lane
```

```console
$ ./crates/pncad-py/run-python-tests.sh             # the Python surface
$ PYTHONPATH=target/python-stage python3 crates/pncad-py/examples/bracket.py
```

The tour writes an STL and a STEP per body plus a `scenes.json`
manifest. Every scene runs the same `run_body` ladder that section 2
of the guide teaches — so reading any scene, you already know the
bottom half of it.

## Where to start

| If you want to… | read |
|---|---|
| author a profile with the PATHS algebra | `bodies.rs` (bracket), `paths.rs` |
| see the whole fillet-corner taxonomy | `rocker.rs` |
| do booleans on planar parts | `bool_bodies.rs`, `projectbox.rs` |
| glue parts that *touch* (declared contact) | `crosslap.rs`, `booleans.rs` |
| work with curved geometry | `bossplate.rs`, `curvedcut.rs`, `diefillet.rs` |
| loft or sweep along a path | `skinned.rs`, `tube.rs`, `lily.rs` |
| build a parametric recipe | `plate_param.rs`, `heatsink.rs` |
| target features by name/selector | `diefillet.rs`, `die_composed.rs` |
| write Python | `examples/bracket.py`, guide §2.8 |

## The demo tour — `demos/tour/src/`

47 stops across 23 scene modules, plus shared helpers (`paths.rs`,
`booleans.rs`, `scalar.rs`, `walls.rs`) and the non-scene lanes:
`probe.rs`, the K-telemetry sweep; `uvdump.rs`, the per-face UV dump;
`tessbudget.rs`, the per-face tessellation-budget sweep; `gallery.rs`,
which saves the document-authored scenes as `.pncad` files the viewer
opens; and `checks.rs`, the advisory-check registry run as narration.
The tour reaches the kernel through `pncad` and **nothing else**,
which is the façade's acceptance evidence — with one stated exception
in its `Cargo.toml`: a direct `profile` edge for the raw-loop door
`lily.rs`'s section loops need and the lattice has no verb for.

| scene(s) | module | demonstrates | pins |
|---|---|---|---|
| `bracket`, `plate`, `vase`, `sheave`, `chute` | `bodies.rs` | The four body ops on public API: polyline+fillet extrude, a genus-2 holed plate, full and partial revolves, plane+cylinder+cone+torus on one part | The bracket's inner corner is *constructive* (`fillet`), not a hand-rounded via point — the pre-#100 decimal sat inside the ε escalation band |
| `rocker` | `rocker.rs` | The complete fillet-corner taxonomy: arc×line, line×line, line×arc, arc×arc, through the PATHS fused fillet verbs (`fillet_arc`, `arc_fillet`, `arc_fillet_arc`, plus the plain line×line seam) | Not a corner typed by hand — every one is DERIVED from its two carriers; each declaration verified, `TangencyContradicted` on a lie. Branch choice read back with `ValidatedLoop::blend_arcs` |
| `diefillet`, `diepips`, `diecomposed` | `diefillet.rs` | Rolling-ball `fillet_edges`; a 21-ball closed-group cut; M6 in-place composition surgery | Sequential pip cuts would present a trimmed sphere as an operand — refused typed. A tilted ball pole makes plane×sphere non-polar — refused typed |
| `lily` (15 bodies) | `lily.rs` | `tube_along_arc` turtle chains, revolved sphere-zone lanterns, swept kite-section leaves | `wall_probes()` is a live record of kernel refusals (coincident-planar glue). Findings 9 and 13 named in place |
| `tiltedcut` | `curvedcut.rs` | An exact `Curve3::Ellipse` section produced by `topo::split` | Three retire-on-closure frontier panics fired and were retired |
| `bossplate` | `bossplate.rs` | The first transverse curved boolean; seam of 3 exact `Circle` arcs | Shared chord ids asserted across the seam; a merely *touching* curved result refuses at tier 3′ |
| `loft_prism`, `nonuniform_loft`, `s_duct`, `twisted_duct` (+2 shadows) | `skinned.rs` | NURBS loft and sweep; the scene *asks* the kernel for its chosen parameters via `loft_parameters` | #207 (weight channel an ulp off 1.0); #210/#218; chord-length vs z-proportional parameterization is a 19% volume difference |
| `tube_along_arc` | `tube.rs` | A tube built from intent parameters rather than a hand-built section | Stored `minor_radius` pinned bit-exact with `==`, retiring the profile→bulge→radius drift |
| `die`, `table` | `bool_bodies.rs` | Planar union/subtract chains against exact dyadic volume oracles | Undeclared coplanar touch refuses; declared flush legs glue |
| `silhouette`, `silhouette3` (+3 shadows) | `letterforms.rs` | The first `intersect`, and intersect-of-intersect | **The design rule**: operands must not share coincident planes. The naive flush variant is built and narrated refusing (`DescriptionNotAdjacent`, `NonMaximalFaces`); 1/16 decoupling turns it green |
| `az` | `az.rs` | The #93 acceptance case: counter-hole A × Z, gated on an exact oracle | #108 (`JoinDesync`) and #111 (CDT centroid parity → exterior needle) both closed and retired |
| `crosslap`, `crosslap_exploded` | `crosslap.rs` | Boolean-of-boolean joinery | The declared/undeclared contrast, asserted live: an undeclared mate still refuses, with a retire-if-it-stops-refusing panic |
| `projectbox` | `projectbox.rs` | A 15-op boolean chain with a per-op dyadic oracle | Honest note: square-only; round bosses/pilot holes are not attempted (curved operands are gated per C5 arm, not by a blanket operand gate) |
| `cutaway` | `cutaway.rs` | The first `topo::split`, on a boolean result, then `transform_rigid` | Split output carries no contacts, so it takes plain tier 3 — the 3/3′ rule in action |
| `heatsink5/7/9` | `heatsink.rs` | The recipe layer via `pncad::document`: one document, structural-param edits, downstream-only recompute, stable `Instance(i)` names | **Named gap F4**: a Boolean node cannot consume a Pattern node's `Instances` payload, so the union step honestly lives outside the document |
| — | `booleans.rs` | The declare door: `flush_declarations` building `BooleanDeclarations` for `union_with`/`intersect_with` | There is no `detect_*` in use anywhere: value equality never classifies |
| — | `paths.rs` | The shared `path_polygon` helper — the tour's polygons said through the PATHS algebra | Since LIB-RETTAIL it is the ONLY way the tour says a polygon: raw `ProfileLoop` construction is off the presented surface, and the one place the tour still needs the raw door — `lily.rs`'s section loops — is a named exception in its `Cargo.toml` |
| — | `probe.rs` | The K-telemetry sweep (`cargo run -- k-probe out.csv`) | One process per ε row |

One deliberate exception worth knowing: the `bracket` scene is retired
from the montage in favour of `rocker`, which shows strictly more.

Note also where the bowtie lives: not in the tour (a broken-on-purpose
scene is not a use case — Evan's ruling on #413) but asserted in
`crates/profile/tests/rejections.rs`. The chain AUTHORS through the
lattice — the junction checks are local and all four corners are sharp
— and `validate` refuses it with the exact typed error.

## The document corpus — `crates/editor-core/tests/corpus/`

18 documents in 16 modules — `islands` registers three, every other
module registers one. Where the tour shows *kernel* usage, these
show **recipe** usage: each one is a `Vec<DocEdit>` edit log replayed
onto an empty document, never a hand-built graph — so each is also a
worked example of the surface Python speaks.

Two properties make them a trustworthy example set. Several carry an
exact **dyadic mass pin**, so the arithmetic is checked to the bit,
not to a tolerance. And `vocabulary()` derives node-kind and edit-kind
tallies *from the documents themselves*, so adding a `Node` or
`DocEdit` variant fails `vocabulary_coverage_is_total` until some
document covers it — the corpus cannot silently fall behind the
vocabulary. (Only `Sweep` sits knowingly at zero, waiting on the
joined-path composition lane.)

| document | demonstrates | pins |
|---|---|---|
| `plate_param` | **The parametric flagship.** A plate whose two hole radii are one shared `DocParam`; the first document where editing a parameter changes a profile's *shape* | Four rows: the volume moves by the right derivative; one parameter drives both holes; `r = 0` refuses at replay naming loop and step; overlapping holes refuse at *validate* — a different door |
| `die` | The M3 exact-oracle die (77 nodes) as a recipe, reused verbatim from the shared fixture so corpus and other suites cannot drift | Exact dyadic volume `7.8359375`, area `26.625`; a minimal-cone bump probe |
| `table` | Four legs with every flush contact **declared by name**, authored through the detect/declare protocol (`find_flush_candidates` → `declare_node`) | The detector also reports coplanar-but-*disjoint* pairs, which are silent no-ops at the op — a real trap, pinned |
| `heatsink` | Carries **both** shapes of "many fins": a `Pattern` whose payload is `Instances`, and the explicit Transform+Union chain | Why both exist: a Boolean cannot consume an `Instances` payload. Fin bases sit 1/16 inside the base — flush would be an undeclared coincidence |
| `slots` | Crossing slots: boolean-of-boolean, with coplanar floors declared by name | Exact pins `6.5` / `32.0` |
| `islands` (×3) | The #93 doubly-nested island chain in general position, at two depths | `105` pins the exact `22.4375` that pre-#93 main silently returned as `22.5` — a fail-loud violation caught by an exact pin |
| `tangency` | Fillet-constructor-declared tangency beside hand-declared tangency | Sits between two doors: `UndeclaredTangency` and `TangencyContradicted { same_carrier: true }` |
| `sink` | `kitchen_sink`: every v1 node kind and all 14 `DocEdit` kinds in one document | Its `SetTolerance` re-records the *ambient* ε — pinning any other value would refuse to load in every CI ε row but one |
| `cut_cylinder` | The first curved cut: an extruded disc split by a tilted datum plane | Section edges carry the exact `Ellipse` carrier |
| `boss` | The first transverse curved boolean; the union seam minted as exact `Circle` arcs on both operands | No mass pin — the value is π-transcendental, and the corpus does not fake exactness |
| `die_fillet` | The first body with planes, cylinders **and** spheres at once (rolling ball on all 12 cube edges) | Documents a retired interval-lane blocker (`from_f64(INFINITY)` → NaI absorbed through `min`) |
| `die_pips` | The first sphere in the corpus | Cuts *one* pip, not 21, and records three honest deviations, including why a per-pole master ball beats a rotated copy (sin/cos of ±π/2 are not exact in f64) |
| `die_composed` | M6 composition surgery: box blends + pip cavity + a pip-rim torus band behind one `Fillet` node | The long-lived "unsayable, not unbuildable" note: cap meridians share one sphere, so there is no dihedral wedge and the fit refuses at margin exactly zero. M6-5 gave the node a 14-name selection so the refusal is *excluded*, not loosened |
| `loft_prism` | The first NURBS-walled solid: three sections skinned at v-degree 2, middle section non-affine so the walls are genuinely curved | Volume `9 m³` derived symbolically in the header, but `pin: None` — a quadrature midpoint is an enclosure, not a closed form, and the corpus says so |

## Python — `crates/pncad-py/`

| file | what it is |
|---|---|
| `examples/bracket.py` | **The Python flagship.** The one-shot journey — build a bracket, evaluate, validate, measure, export STEP — and then re-import its own output and compare volumes. Guide §2.8 walks the same model |
| `tests/test_document.py` | The document surface end to end: edit refusals, evaluation errors including poisoning, literal refusals, the D9 bit-replay seed, persistence round-trips, STEP export refusals, and a test that no arena key is reachable |
| `tests/test_quantities.py` | `25 * mm`, canonical units, and the typed `DimensionError` family |
| `tests/test_stubs.py` | The stubs cannot drift: `pncad.pyi` is parsed and compared name-for-name against the compiled module |
| `tests/test_mesh.py` | The mesh door: tessellation budgets and their refusals, the mesh read-back, watertightness decided on shared indices, the mesh-vs-exact cross-check on planar, boolean and curved bodies, and STL. `docs/guide/meshing.md` is the prose |
| `tests/test_guide.py` | Executes every Python block in this guide, read straight from the Markdown |
