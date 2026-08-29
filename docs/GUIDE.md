# The pncad guide

`pncad` is a B-rep CAD kernel: you author exact solids, ask them
questions, and export them. It is a library first — everything below
runs headless, from Rust or from Python, with no GUI in the loop.

One honest note before anything else: **`pncad` is a placeholder
name.** The project has not been named yet (design question Q9). The
crate, the Python module, and the prose all say `pncad` today, and
all of it will be renamed together when the real name is chosen. The
placeholder is deliberately greppable.

What makes this kernel different from a modelling toolkit you may
have used before is that **it refuses**. When two faces coincide and
you have not said they coincide, when a fillet has no corner to sit
in, when a profile crosses itself — the kernel does not guess, repair,
or quietly produce something plausible. It returns a typed error
naming what it would have had to assume. That behaviour is the
product, not a rough edge; the fail-loud tour documents it as such.

Everything in this guide is executed. Rust blocks are doctests
(`cargo test --doc -p pncad`), Python blocks are run by
`crates/pncad-py/tests/test_guide.py`, which reads this very file.

## 1. Quickstart

### 1.1 What you are installing

One Rust crate, `pncad`, is the whole authoring surface: it
re-exports every kernel crate as a module and offers a curated
`prelude`. You never need a second dependency, including for the
payload types inside error enums.

The Python package is the same kernel behind PyO3 bindings. It speaks
the *document* layer — nodes, edits, evaluation — rather than
wrapping the Rust authoring calls one for one. Section 2.8 shows why
that is a deliberate design choice and not a shortfall.

Nothing is published to crates.io or PyPI yet — the project is
unnamed, so there is nothing to publish under. Build from source.

### 1.2 Rust: build, and a first model in a dozen lines

```console
$ git clone <this repo> && cd cad
$ cargo build
$ cargo test --doc -p pncad      # runs every Rust block in this guide
```

To depend on it from your own crate, point at the façade and nothing
else:

```toml
[dependencies]
pncad = { path = "…/crates/pncad" }
```

A first solid — an 80 × 40 × 8 mm plate, validated, measured:

```
use pncad::prelude::*;

let tol = Tol::witness();
let mm = |v: f64| (v * MM).meters();
let rect: ClosedLoop<f64> = Open
    .at(p2(mm(0.0), mm(0.0)))
    .line_to(p2(mm(80.0), mm(0.0)), tol)?
    .line_to(p2(mm(80.0), mm(40.0)), tol)?
    .line_to(p2(mm(0.0), mm(40.0)), tol)?
    .line_to(Start, tol)?;
let profile = validated(SketchPlane::<f64>::xy(), vec![rect.into()], tol)?;
let plate = extrude(&profile, Extrusion::Distance(real(mm(8.0))), tol)?.body;
validate_closed(&plate).expect("a closed solid");
let props = mass_properties(&plate, tol)?;
assert!((props.volume - 2.56e-5).abs() < 1e-18);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Lengths are plain `f64` in **canonical metres** at the kernel seam.
The `quantity` layer (`MM`, `CM`, `M`, `IN`, `DEG`, `RAD`) is there
when you want the units visible, as above; `25.0 * MM` builds a
`Length` and `.meters()` unwraps it. The kernel never sees a unit —
it sees metres.

The first line is the one that needs explaining. `Tol::witness()`
commits this process's tolerance ε — one value per run, never changed
after, the thing that decides what "coincident" means — and hands back
a witness that it is committed. Every call that *decides* something
takes it, which is why it appears so often below.

`Tol` is zero-sized and has exactly one inhabitant, so passing it
costs nothing at runtime and cannot introduce a second ε: it carries
the right to read the run's tolerance, never a copy of the value. Read
it as punctuation rather than an argument — its job is to make a
signature say whether the answer depends on ε. The operations that
*don't* take one, like `validate_closed` above, are telling you
something: they are exact, and no tolerance can change their answer.

Configure ε per run with `CAD_TOLERANCE_EPS`, or state it in code
with `Tolerance::init` before the first witness. `pncad::tolerance`
reports what a run committed to and where the value came from.

### 1.3 Python: build (maturin), and a first model in ten lines

The bindings are a PyO3 extension module. With `maturin` available in
a virtualenv:

```console
$ python3 -m venv .venv && . .venv/bin/activate
$ pip install maturin
$ maturin develop -m crates/pncad-py/Cargo.toml --features extension-module
```

If the box has no `pip`, build the cdylib and stage it by hand. The
repo's own runner does exactly this, and it is also the quickest way
to run the Python suites:

```console
$ ./crates/pncad-py/run-python-tests.sh          # builds, stages, runs the tests
$ PYTHONPATH=target/python-stage python3 crates/pncad-py/examples/bracket.py
```

The same plate, in Python:

```python
from pncad import Doc, Node, evaluate, mm

doc = Doc()
profile = doc.insert(
    Node.polygon([(0 * mm, 0 * mm), (80 * mm, 0 * mm), (80 * mm, 40 * mm), (0 * mm, 40 * mm)])
)
plate = doc.insert(Node.extrude(profile, 8 * mm))
body = evaluate(doc).value(plate).body()
body.validate()
assert abs(body.mass_properties().volume - 2.56e-5) < 1e-18
```

Here `25 * mm` builds a typed `Length`. Dimensions are checked: `25 *
mm + 90 * deg` is a `DimensionError`, not a number.

### 1.4 Where to go next

- Section 2 is the canonical journey, end to end, in both languages.
- Section 3 is parametric modelling — the document layer proper.
- The corpus index (`docs/guide/examples.md`) maps every worked
  example in the repo to what it demonstrates.
- The fail-loud tour (`docs/guide/fail-loud.md`) is the refusal
  vocabulary, layer by layer.
- Selecting entities (`docs/guide/selecting.md`) is how you name a
  face or an edge so a later step can refer to it.
- The north-star audit (`docs/guide/north-star-audit.md`) says
  exactly which demos Python can author today.

## 2. The canonical journey

Every demo in `demos/tour` runs the same ladder, and it is the ladder
this section teaches — it is the shape of *using* this kernel:

> **author → validate → measure → tessellate → cross-check → export**

The tour's `run_body` (`demos/tour/src/main.rs`) is that ladder
written once and applied to every scene. Nothing about it is
demo-specific; your own program should look like it.

Two properties of the ladder are worth naming before we walk it.
First, **the validation tiers are the journey, not a debug mode** —
you run them, in order, on the way to your answer. Second, **the
cross-check is not optional decoration**: the exact B-rep measure and
the tessellated mesh are computed by independent code paths, and
comparing them is how you find out that one of them is wrong.

### 2.1 The worked example

The bracket: a base plate, an upright web sunk into it and poking out
the top, and a lightening pocket entering from below and stopping
inside the material. Three boxes, a union and a subtract.

| part | x (mm) | y (mm) | z (mm) |
|---|---|---|---|
| base plate | 0 … 80 | 0 … 40 | 0 … 8 |
| upright web | 36 … 44 | 5 … 35 | 4 … 34 |
| pocket (subtracted) | 8 … 28 | 10 … 30 | −2 … 5 |

Every solid here genuinely *interpenetrates* the one it is combined
with — the web is sunk 4 mm into the plate, the pocket pokes 2 mm out
below it. That is not laziness with round numbers. The kernel refuses
a boolean whose operands merely *touch* on a shared plane until you
declare that contact, because inferring "these two planes are the
same plane" from float equality is exactly the guess it will not
make. Section 2.3 and the fail-loud tour return to this.

`crates/pncad-py/examples/bracket.py` builds this model with one
addition the table cannot hold: since the PATHS lattice crossed to
Python, its base plate carries a 6 mm round at each of its four
corners, authored as fillets rather than as a rounded outline. The
extents, the web and the pocket are these ones, so the two languages
below are building one solid, and the plate's own closed form is the
only number that differs.

### 2.2 Author

Profiles are closed loops on a sketch plane, and there is one way to
say one: the PATHS algebra, where you walk the outline and the type
system tracks what the tip has bound. (Raw `ProfileLoop` vertex tables
are kernel vocabulary and not part of this surface — Evan's ruling on
#413. The lattice is not merely the nicer spelling; it is the one that
classifies each junction as you author it, so a corner that is
accidentally tangent or reversed refuses here rather than at
`validate`.)

```
use pncad::prelude::*;

let tol = Tol::witness();
// The algebra: open at a point, walk the legs, close at the seam.
let mm = |v: f64| (v * MM).meters();
let outline: ProfileLoop<f64> = Open
    .at(p2(mm(0.0), mm(0.0)))
    .line_to(p2(mm(80.0), mm(0.0)), tol)?
    .line_to(p2(mm(80.0), mm(40.0)), tol)?
    .line_to(p2(mm(0.0), mm(40.0)), tol)?
    .line_to(Start, tol)?
    .into();
assert_eq!(outline.vertices().len(), 4);
# Ok::<(), pncad::profile::PathError<f64>>(())
```

`line_to(Start)` closes the loop, and closing is where both of the
seam's junction checks run with the incoming and outgoing directions
finally known. The payoff is that a corner which is accidentally
tangent — the classic silent-bad-geometry case — refuses *at
authoring time*, before validation ever sees the profile.

The algebra also builds what a polygon cannot. An intended round is
constructive, never a flag you set:

```
use pncad::prelude::*;

let tol = Tol::witness();
let mm = |v: f64| (v * MM).meters();
let rounded: ProfileLoop<f64> = Open
    .at(p2(mm(0.0), mm(0.0)))
    .line_to(p2(mm(40.0), mm(0.0)), tol)?  // sharp corner here, arriving east
    .toward(0.0, 1.0, tol)?                // departure ray: north, the line x = 40
    .fillet(mm(6.0), tol)?                 // round where that ray meets the next
    .toward(-1.0, 0.0, tol)?               // arrival ray: west, the line y = 30
    .to(p2(mm(0.0), mm(30.0)), tol)?       // anchored at the authored far vertex
    .line_to(Start, tol)?
    .into();
// Five vertices: two sharp corners, and the arc's two tangent points
// where the fourth corner used to be.
assert_eq!(rounded.vertices().len(), 5);
# Ok::<(), pncad::profile::PathError<f64>>(())
```

That block repays a careful read, because it encodes the algebra's
central idea. **The rounded corner is never authored.** You give the
two rays that would have met there — the departure ray before the
fillet, the arrival ray after it — and the arc is fitted to their
*virtual* intersection, trimming both. There is no moment at which
the sharp point (40, 30) exists and is then removed, so "author a
corner, then fillet it away" is not a thing you can say. Two parallel
rays have no corner to round, and that refuses as
`NoCornerForFillet { reason: CarriersParallel }`.

Nowhere in this repo does a demo hand-write a tangency flag. Tangency
arrives by construction, and the kernel *verifies* every declaration
rather than trusting it — a contradicted one is
`TangencyContradicted`, not a warning.

**The same two loops, in Python.** One semantics, two host languages:
the lattice binds state for state, so the chain reads verb for verb,
and the states are distinct classes exposing only their legal
continuations — a double director or a leading `.fillet` is an
`AttributeError`, and under `ty` a static error, because the method
is not there to call.

```python
from pncad import Open, Start, mm

outline = (
    Open.at((0 * mm, 0 * mm))
    .line_to((80 * mm, 0 * mm))
    .line_to((80 * mm, 40 * mm))
    .line_to((0 * mm, 40 * mm))
    .line_to(Start)
)
assert outline.vertex_count == 4

rounded = (
    Open.at((0 * mm, 0 * mm))
    .line_to((40 * mm, 0 * mm))       # sharp corner here, arriving east
    .toward(0.0, 1.0)                 # departure ray: north, the line x = 40
    .fillet(6 * mm)                   # round where that ray meets the next
    .toward(-1.0, 0.0)                # arrival ray: west, the line y = 30
    .to((0 * mm, 30 * mm))            # anchored at the authored far vertex
    .line_to(Start)
)
# The same five vertices the Rust block asserts.
assert rounded.vertex_count == 5

# The refusals are the kernel's own, raised where the verb was
# written — nothing is pre-checked on the Python side.
import pncad

try:
    Open.at((0 * mm, 0 * mm)).line_to((40 * mm, 0 * mm)).angle(0 * pncad.deg)
except pncad.PathError as refused:
    assert refused.variant == "junction_tangent"
else:
    raise AssertionError("an accidentally tangent corner must refuse")

# The lattice is the type: a plain point has no incoming tangent to
# inherit, so `.tangent()` is not a method it has.
assert not hasattr(Open.at((0 * mm, 0 * mm)), "tangent")
```

A closed loop becomes a document node with `Node.profile`, which is
built from the loop's RECORDED program — the same verbs, the same
authored numbers — so what Python wrote and what the document replays
are one program:

```python
import math

from pncad import Doc, Node, Open, Start, evaluate, mm

rounded = (
    Open.at((0 * mm, 0 * mm))
    .line_to((40 * mm, 0 * mm))
    .toward(0.0, 1.0)
    .fillet(6 * mm)
    .toward(-1.0, 0.0)
    .to((0 * mm, 30 * mm))
    .line_to(Start)
)

doc = Doc()
plate = doc.insert(Node.extrude(doc.insert(Node.profile(rounded)), 8 * mm))
ev = evaluate(doc)
assert ev.succeeded(plate)

# 40 x 30 mm, one 6 mm corner rounded off, 8 mm thick. The rounded
# corner removes r^2 - pi*r^2/4 of area.
area = 0.040 * 0.030 - (0.006**2 - math.pi * 0.006**2 / 4)
assert abs(ev.value(plate).body().mass_properties().volume - area * 0.008) < 1e-15
```

**Corners between a line and a CIRCLE.** A fillet's two sides do not
have to be straight, and a side that rides a carrier is authored in
the SAME act as the fillet: `arc_fillet(spec, r)` gives the corner an
arc INCOMING side, `fillet_arc(r, spec)` gives it an arc ARRIVAL, and
`arc_fillet_arc(spec, r, spec2)` does both. The spec is the binding
mode: `Center { c, winding, p }` names the centre, the travel sense
and the anchor, so the tangent there is derived rather than authored.
A straight side stays straight, bound by the ordinary `.at(p)` and
`.toward(dx, dy)`. Either way the corner is never written down; it is
the ray-meets-circle intersection, and the gates discard the root the
author's two anchors do not bracket.

```
use pncad::prelude::*;

let tol = Tol::witness();
// Enter on the R = 5 circle at its east point, travelling
// counterclockwise; round where that circle meets the line y = 3.
let blended: ProfileLoop<f64> = Open
    // incoming side rides the CIRCLE; fillet radius in the same act
    .arc_fillet(
        Center { c: p2(0.0, 0.0), winding: ArcSweep::Ccw, p: p2(5.0, 0.0) },
        0.5,
        tol,
    )?
    .at(p2(0.0, 3.0), tol)?          // arrival: the line y = 3 ...
    .toward(-1.0, 0.0, tol)?         // ... heading west
    .line(3.0, tol)?
    .line_to(Start, tol)?
    .into();
// The entry, the trim point on the circle, the arc's far tangent
// point, and the straight side's end.
assert_eq!(blended.vertices().len(), 4);
# Ok::<(), pncad::profile::PathError<f64>>(())
```

The circle meets `y = 3` at `(±4, 3)`, and only one of those is the
corner the author meant: the arrival is anchored at `(0, 3)` heading
west, so it came from `(4, 3)` and never from `(−4, 3)`. That is a
gate, not a nearest-point guess — and the same chain says the same
thing in Python:

```python
from pncad import ArcSweep, Center, Open, Start, m

blended = (
    Open.arc_fillet(Center((0 * m, 0 * m), ArcSweep.Ccw, (5 * m, 0 * m)), 0.5 * m)
    .at((0 * m, 3 * m))
    .toward(-1.0, 0.0)
    .line(3 * m)
    .line_to(Start)
)
assert blended.vertex_count == 4
```

**The plane is an argument, not an assumption.** A profile lives on a
`SketchPlane` — a rigid frame `origin, u, v`, where sketch (x, y) maps
to `origin + x·u + y·v`. The plane's NORMAL is `u × v`, and that is
the direction `extrude` runs, so choosing the plane is choosing the
axis. Three named frames come cyclically (x→y→z→x): `xy` (normal +z),
`yz` (u = ŷ, v = ẑ, normal +x), `zx` (u = ẑ, v = x̂, normal +y).
`elevation=` remains what it always was — sugar for the xy-plane, that
far up z — and naming the plane both ways at once is a `TypeError`
rather than a silent preference.

Rigidity (u, v unit and perpendicular) is **conventional data,
unchecked**, in Python exactly as in Rust: a non-rigid frame yields a
well-defined skewed sketch, not poison, and the kernel's geometric
validation is what certifies a body at rest.

```python
from pncad import Doc, Node, SketchPlane, evaluate, m

doc = Doc()
# An upright wall: a 2 x 3 sketch on the world yz-plane, extruded
# 0.25 along that plane's normal, which is +x.
wall = doc.insert(
    Node.extrude(
        doc.insert(
            Node.polygon(
                [(0 * m, 0 * m), (2 * m, 0 * m), (2 * m, 3 * m), (0 * m, 3 * m)],
                plane=SketchPlane.yz(),
            )
        ),
        0.25 * m,
    )
)
assert abs(evaluate(doc).value(wall).body().mass_properties().volume - 1.5) < 1e-12

# Naming the plane twice is refused at the boundary.
try:
    Node.polygon([(0 * m, 0 * m)], elevation=1 * m, plane=SketchPlane.yz())
except TypeError:
    pass
else:
    raise AssertionError("plane= and elevation= must be mutually exclusive")
```

**Stacked sections make a loft.** `Node.loft(profiles, v_degree)`
skins a solid through two or more section profiles in skin order — and
takes no placement argument, because each section rides its own
profile's sketch plane. The three sections below are the corpus's
`loft_prism`: squares at z = 0 and z = 2 with a trapezoid between
them, whose non-parallel pair means the middle section is *not* an
affine image of the ends, so the four walls are genuinely curved
rather than ruled.

```python
from pncad import Doc, Node, evaluate, m

SQUARE = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
TRAPEZOID = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)]

doc = Doc()
sections = [
    doc.insert(Node.polygon([(x * m, y * m) for x, y in pts], elevation=z * m))
    for pts, z in [(SQUARE, 0.0), (TRAPEZOID, 1.0), (SQUARE, 2.0)]
]
prism = doc.insert(Node.loft(sections, 2))

# The degree-2 skin through sections at (0, 1/2, 1) is the quadratic
# Lagrange interpolant: corner paths S + 4v(1-v)*D, z = 2v exactly,
# each slice a trapezoid of area 4 + 2*d*4v(1-v) with d = 0.375, so
# V = 8 + 8d/3 = 9 m^3 exactly. Mass properties are an ENCLOSURE, so
# the check is that 9 lies inside the certified pad.
props = evaluate(doc).value(prism).body().mass_properties()
assert abs(props.volume - 9.0) <= props.volume_pad + 1e-9
assert props.volume_pad < 1e-6

# The kernel's rule, not the binding's: 1 <= v_degree <= n - 1. Three
# sections cannot carry degree 3, and nothing here pre-checks that —
# it refuses at evaluation, where the kernel refuses.
overdegree = doc.insert(Node.loft(sections, 3))
assert not evaluate(doc).succeeded(overdegree)
```

A profile becomes usable by turning it into a body. `validated` is
the one two-call wrapper the façade adds (`Profile::new` then
`Profile::validate`); `extrude` takes it from there:

```
use pncad::prelude::*;

# let tol = Tol::witness();
# type E = Box<dyn std::error::Error>;
fn slab(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Result<Body<f64>, E> {
    let tol = Tol::witness();
    let rect: ClosedLoop<f64> = Open
        .at(p2(x.0, y.0))
        .line_to(p2(x.1, y.0), tol)?
        .line_to(p2(x.1, y.1), tol)?
        .line_to(p2(x.0, y.1), tol)?
        .line_to(Start, tol)?;
    let plane = SketchPlane::from_frame(
        p3(0.0, 0.0, z.0), v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0),
    );
    let profile = validated(plane, vec![rect.into()], tol)?;
    Ok(extrude(&profile, Extrusion::Distance(real(z.1 - z.0)), tol)?.body)
}

let mm = |v: f64| (v * MM).meters();
let base = slab((mm(0.0), mm(80.0)), (mm(0.0), mm(40.0)), (mm(0.0), mm(8.0)))?;
let web = slab((mm(36.0), mm(44.0)), (mm(5.0), mm(35.0)), (mm(4.0), mm(34.0)))?;
let pocket = slab((mm(8.0), mm(28.0)), (mm(10.0), mm(30.0)), (mm(-2.0), mm(5.0)))?;

let bracket = union(&base, &web, tol)?;
let bracket = bracket.body().expect("a non-empty union");
let lightened = subtract(&bracket.body, &pocket, tol)?;
let lightened = lightened.body().expect("a non-empty difference");
assert_eq!(lightened.kind, BooleanResultKind::Seamed);
# Ok::<(), E>(())
```

`union` and `subtract` return a `BooleanResult`, which is `Empty` or
a `BooleanBody`. That is the first fail-loud habit to build: an empty
result is a *value*, not an error and not a crash, and you say what
you expect. The `kind` field records how the result came to be —
`Seamed` here, meaning the boundaries genuinely intersected and the
seam was joined and zipped.

### 2.3 Validate — the tier ladder IS the journey

Three gates, each strictly stronger than the last, each returning
`Result<(), Vec<ValidationError>>` — a *vector*, because a broken
body usually has more than one thing wrong with it and reporting the
first one wastes your time.

```
use pncad::prelude::*;
# let tol = Tol::witness();
# type E = Box<dyn std::error::Error>;
# fn slab(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Result<Body<f64>, E> {
#     let tol = Tol::witness();
#     let rect: ClosedLoop<f64> = Open
#         .at(p2(x.0, y.0))
#         .line_to(p2(x.1, y.0), tol)?
#         .line_to(p2(x.1, y.1), tol)?
#         .line_to(p2(x.0, y.1), tol)?
#         .line_to(Start, tol)?;
#     let plane = SketchPlane::from_frame(p3(0.0, 0.0, z.0), v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0));
#     Ok(extrude(&validated(plane, vec![rect.into()], tol)?, Extrusion::Distance(real(z.1 - z.0)), tol)?.body)
# }
# let mm = |v: f64| (v * MM).meters();
# let base = slab((mm(0.0), mm(80.0)), (mm(0.0), mm(40.0)), (mm(0.0), mm(8.0)))?;
# let web = slab((mm(36.0), mm(44.0)), (mm(5.0), mm(35.0)), (mm(4.0), mm(34.0)))?;
# let pocket = slab((mm(8.0), mm(28.0)), (mm(10.0), mm(30.0)), (mm(-2.0), mm(5.0)))?;
# let u = union(&base, &web, tol)?; let u = u.body().expect("union");
# let r = subtract(&u.body, &pocket, tol)?; let result = r.body().expect("difference");
let body = &result.body;

validate(body).expect("tier 1: structural integrity");
validate_closed(body).expect("tier 2: a closed, connected solid");
validate_pseudomanifold(body, &result.contacts, tol)
    .expect("tier 3′: geometry, with this operation's declared contacts");
# Ok::<(), E>(())
```

- **Tier 1, `validate`** — structural. Every half-edge has its mate,
  every loop closes, the arena is internally consistent.
- **Tier 2, `validate_closed`** — a closed solid: no empty loops, no
  valence-1 struts, shells connected. It runs tier 1 first.
- **Tier 3, `validate_geometric`** — geometry: faces don't
  self-intersect, orientation agrees with the computed volume, and so
  on. It runs tier 2 first.
- **Tier 3′, `validate_pseudomanifold(body, contacts)`** — tier 3 for
  a body that legitimately touches itself, checked against the
  declared contacts the boolean pipeline carried into the result.

The 3-versus-3′ choice is mechanical, and the tour makes it the same
way you should: **if the value came from a boolean, gate it at 3′
with that operation's own `contacts`; otherwise gate it at 3.** A
body from `split` carries no contacts, so it takes plain tier 3.

```
use pncad::prelude::*;
# let tol = Tol::witness();
# type E = Box<dyn std::error::Error>;
# fn slab(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Result<Body<f64>, E> {
#     let tol = Tol::witness();
#     let rect: ClosedLoop<f64> = Open
#         .at(p2(x.0, y.0))
#         .line_to(p2(x.1, y.0), tol)?
#         .line_to(p2(x.1, y.1), tol)?
#         .line_to(p2(x.0, y.1), tol)?
#         .line_to(Start, tol)?;
#     let plane = SketchPlane::from_frame(p3(0.0, 0.0, z.0), v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0));
#     Ok(extrude(&validated(plane, vec![rect.into()], tol)?, Extrusion::Distance(real(z.1 - z.0)), tol)?.body)
# }
# let mm = |v: f64| (v * MM).meters();
# let base = slab((mm(0.0), mm(80.0)), (mm(0.0), mm(40.0)), (mm(0.0), mm(8.0)))?;
// A body with no declared contacts: the two gates agree.
validate_geometric(&base, tol).expect("tier 3 on a plain extrusion");
validate_pseudomanifold(&base, &ContactRecords::default(), tol)
    .expect("3′ on an empty-contact body is tier 3 plus a census");
# Ok::<(), E>(())
```

### 2.4 Measure — mass properties and their pads

`mass_properties` integrates over the exact B-rep — the divergence
theorem on the real surfaces, not on a mesh:

```
use pncad::prelude::*;
# let tol = Tol::witness();
# type E = Box<dyn std::error::Error>;
# fn slab(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Result<Body<f64>, E> {
#     let tol = Tol::witness();
#     let rect: ClosedLoop<f64> = Open
#         .at(p2(x.0, y.0))
#         .line_to(p2(x.1, y.0), tol)?
#         .line_to(p2(x.1, y.1), tol)?
#         .line_to(p2(x.0, y.1), tol)?
#         .line_to(Start, tol)?;
#     let plane = SketchPlane::from_frame(p3(0.0, 0.0, z.0), v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0));
#     Ok(extrude(&validated(plane, vec![rect.into()], tol)?, Extrusion::Distance(real(z.1 - z.0)), tol)?.body)
# }
# let mm = |v: f64| (v * MM).meters();
# let base = slab((mm(0.0), mm(80.0)), (mm(0.0), mm(40.0)), (mm(0.0), mm(8.0)))?;
# let web = slab((mm(36.0), mm(44.0)), (mm(5.0), mm(35.0)), (mm(4.0), mm(34.0)))?;
# let pocket = slab((mm(8.0), mm(28.0)), (mm(10.0), mm(30.0)), (mm(-2.0), mm(5.0)))?;
# let u = union(&base, &web, tol)?; let u = u.body().expect("union");
# let r = subtract(&u.body, &pocket, tol)?; let result = r.body().expect("difference");
let props = mass_properties(&result.body, tol)?;

assert!((props.volume - 2.984e-5).abs() < 1e-15);       // m³
assert!((props.surface_area - 1.0696e-2).abs() < 1e-12); // m²

// Every face here has a closed form, so the certified pads are
// exactly zero — the measure is not approximate.
assert_eq!(props.volume_pad, 0.0);
assert_eq!(props.area_pad, 0.0);
# Ok::<(), E>(())
```

The **pads** are the part people miss. `volume` and `surface_area`
are not bare numbers, they are the midpoints of *certified
enclosures*, and `volume_pad`/`area_pad` are the half-widths. The
true value is inside `volume ± volume_pad`, guaranteed, not
estimated. Planar and other closed-form faces contribute exactly, so
their pads are `0.0`; a curved *cut* face contributes a certified
quadrature bracket and widens the pad. A nonzero pad is the kernel
telling you how much it does not know, and a program that cares about
a tolerance should read it rather than assume it.

### 2.5 Tessellate

Tessellation is a separate, explicit step with its own budget. The
chordal parameter is a **distance in metres** — the maximum the mesh
may deviate from the true surface — and it is deliberately *not* the
kernel's ε. Meshing precision is a display decision; ε is a modelling
decision.

```
use pncad::prelude::*;
# let tol = Tol::witness();
# type E = Box<dyn std::error::Error>;
# fn slab(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Result<Body<f64>, E> {
#     let tol = Tol::witness();
#     let rect: ClosedLoop<f64> = Open
#         .at(p2(x.0, y.0))
#         .line_to(p2(x.1, y.0), tol)?
#         .line_to(p2(x.1, y.1), tol)?
#         .line_to(p2(x.0, y.1), tol)?
#         .line_to(Start, tol)?;
#     let plane = SketchPlane::from_frame(p3(0.0, 0.0, z.0), v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0));
#     Ok(extrude(&validated(plane, vec![rect.into()], tol)?, Extrusion::Distance(real(z.1 - z.0)), tol)?.body)
# }
# let mm = |v: f64| (v * MM).meters();
# let base = slab((mm(0.0), mm(80.0)), (mm(0.0), mm(40.0)), (mm(0.0), mm(8.0)))?;
# let web = slab((mm(36.0), mm(44.0)), (mm(5.0), mm(35.0)), (mm(4.0), mm(34.0)))?;
# let pocket = slab((mm(8.0), mm(28.0)), (mm(10.0), mm(30.0)), (mm(-2.0), mm(5.0)))?;
# let u = union(&base, &web, tol)?; let u = u.body().expect("union");
# let r = subtract(&u.body, &pocket, tol)?; let result = r.body().expect("difference");
let mesh = tessellate(&result.body, 0.0005, tol)   // 0.5 mm chord budget
    .expect("the bracket tessellates");

assert!(!mesh.positions.is_empty());
assert!(!mesh.patches.is_empty());   // one patch per face, addressable
# Ok::<(), E>(())
```

The mesh keeps a patch per face and a polyline per edge, and adjacent
faces share position indices along their common boundary — so the
triangle set of a closed body is watertight by construction, not by
a repair pass.

### 2.6 Cross-check

Two independent computations of the same quantity, compared. This is
the step that earns trust in both of them:

```
use pncad::prelude::*;
use pncad::mesh::validate::{check_mesh, signed_volume, triangle_count};
# let tol = Tol::witness();
# type E = Box<dyn std::error::Error>;
# fn slab(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Result<Body<f64>, E> {
#     let tol = Tol::witness();
#     let rect: ClosedLoop<f64> = Open
#         .at(p2(x.0, y.0))
#         .line_to(p2(x.1, y.0), tol)?
#         .line_to(p2(x.1, y.1), tol)?
#         .line_to(p2(x.0, y.1), tol)?
#         .line_to(Start, tol)?;
#     let plane = SketchPlane::from_frame(p3(0.0, 0.0, z.0), v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0));
#     Ok(extrude(&validated(plane, vec![rect.into()], tol)?, Extrusion::Distance(real(z.1 - z.0)), tol)?.body)
# }
# let mm = |v: f64| (v * MM).meters();
# let base = slab((mm(0.0), mm(80.0)), (mm(0.0), mm(40.0)), (mm(0.0), mm(8.0)))?;
# let web = slab((mm(36.0), mm(44.0)), (mm(5.0), mm(35.0)), (mm(4.0), mm(34.0)))?;
# let pocket = slab((mm(8.0), mm(28.0)), (mm(10.0), mm(30.0)), (mm(-2.0), mm(5.0)))?;
# let u = union(&base, &web, tol)?; let u = u.body().expect("union");
# let r = subtract(&u.body, &pocket, tol)?; let result = r.body().expect("difference");
# let props = mass_properties(&result.body, tol)?;
# let mesh = tessellate(&result.body, 0.0005, tol).expect("tessellate");
// 1. The mesh is a closed 2-manifold — no boundary edges, no
//    non-manifold junctions. A refusal here is fail-loud, not a hint.
check_mesh(&mesh).expect("a watertight mesh");

// 2. Its signed volume is positive: the winding really is outward.
let v_mesh = signed_volume(&mesh);
assert!(v_mesh > 0.0);

// 3. It agrees with the exact B-rep measure. This body is all
//    planar, so the triangulation is exact and the agreement is at
//    rounding level; a curved body's error shrinks with the budget.
let rel = (v_mesh - props.volume).abs() / props.volume;
assert!(rel < 1e-12, "mesh vs exact: {rel:e}");
assert!(triangle_count(&mesh) > 0);
# Ok::<(), E>(())
```

### 2.7 Export

STEP (AP242) for exchange, STL for the mesh:

```
use pncad::prelude::*;
use pncad::step_import::StepImport;
# let tol = Tol::witness();
# type E = Box<dyn std::error::Error>;
# fn slab(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Result<Body<f64>, E> {
#     let tol = Tol::witness();
#     let rect: ClosedLoop<f64> = Open
#         .at(p2(x.0, y.0))
#         .line_to(p2(x.1, y.0), tol)?
#         .line_to(p2(x.1, y.1), tol)?
#         .line_to(p2(x.0, y.1), tol)?
#         .line_to(Start, tol)?;
#     let plane = SketchPlane::from_frame(p3(0.0, 0.0, z.0), v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0));
#     Ok(extrude(&validated(plane, vec![rect.into()], tol)?, Extrusion::Distance(real(z.1 - z.0)), tol)?.body)
# }
# let mm = |v: f64| (v * MM).meters();
# let base = slab((mm(0.0), mm(80.0)), (mm(0.0), mm(40.0)), (mm(0.0), mm(8.0)))?;
# let web = slab((mm(36.0), mm(44.0)), (mm(5.0), mm(35.0)), (mm(4.0), mm(34.0)))?;
# let pocket = slab((mm(8.0), mm(28.0)), (mm(10.0), mm(30.0)), (mm(-2.0), mm(5.0)))?;
# let u = union(&base, &web, tol)?; let u = u.body().expect("union");
# let r = subtract(&u.body, &pocket, tol)?; let result = r.body().expect("difference");
# let props = mass_properties(&result.body, tol)?;
# let mesh = tessellate(&result.body, 0.0005, tol).expect("tessellate");
let step = step_string(&result.body, &StepOptions {
    product_name: "bracket".to_string(),
    ..Default::default()
}, tol)?;
assert!(step.starts_with("ISO-10303-21;"));

// STL's two formats carry different things, so they take different
// options. The binary format's 80 bytes are free text — conventionally
// the producer; the ASCII format's `solid <name>` names the part. Each
// is a validated newtype, so a name that cannot be written is refused
// here rather than when you export.
let mut stl = Vec::new();
write_binary(&mesh, &BinaryOptions {
    header: BinaryHeader::new("bracket, exported by the tour")?,
}, &mut stl)?;
let mut stl_text = Vec::new();
write_ascii(&mesh, &AsciiOptions {
    solid_name: SolidName::new("bracket")?,
}, &mut stl_text)?;
assert!(String::from_utf8(stl_text)?.starts_with("solid bracket\n"));
let declared = u32::from_le_bytes(stl[80..84].try_into().unwrap()) as usize;
assert_eq!(declared, pncad::mesh::validate::triangle_count(&mesh));

// The strongest export check available: read your own file back with
// the kernel's importer and re-measure it.
let StepImport::Solid { body: reimported, .. } =
    import_step(&step, &ImportOptions::default(), tol)?
else {
    panic!("the bracket re-imports as a solid, not a wireframe");
};
let back = mass_properties(&reimported, tol)?;
assert!((back.volume - props.volume).abs() < 1e-15);
# Ok::<(), E>(())
```

That round-trip is the habit to copy. An export that writes bytes
proves nothing; an export that re-imports to the same volume proves
the geometry survived.

STEP export is one of the few places the kernel refuses for a reason
you should *expect* rather than fix: `UnsupportedSurface`,
`UnsupportedCurve` and `CurvedShellClassification` mean the AP242
writer has no representation for a surface the modeller can build.
The tour treats exactly those three as tolerated, per scene, and
panics on anything else.

### 2.8 The same journey in Python

Python does not mirror the Rust calls above. It speaks the **document
layer**: you insert nodes describing what to build, evaluate the
document, and read typed values out. This is deliberate (LIBRARY-DESIGN
§L3) — the document layer is the single API surface shared by the
GUI, macro recording, and the bindings, so a Python script is a
recipe that persists, replays, and undoes, rather than a pile of
opaque kernel calls.

The same bracket, the same numbers:

```python
from pncad import BooleanOp, Doc, Node, evaluate, import_step, mm


def slab(doc, x, y, z):
    profile = doc.insert(
        Node.polygon(
            [(x[0], y[0]), (x[1], y[0]), (x[1], y[1]), (x[0], y[1])],
            elevation=z[0],
        )
    )
    return doc.insert(Node.extrude(profile, z[1] - z[0]))


doc = Doc()
base = slab(doc, (0 * mm, 80 * mm), (0 * mm, 40 * mm), (0 * mm, 8 * mm))
web = slab(doc, (36 * mm, 44 * mm), (5 * mm, 35 * mm), (4 * mm, 34 * mm))
bracket = doc.insert(Node.boolean(BooleanOp.Union, base, web))
pocket = slab(doc, (8 * mm, 28 * mm), (10 * mm, 30 * mm), (-2 * mm, 5 * mm))
lightened = doc.insert(Node.boolean(BooleanOp.Subtract, bracket, pocket))

# Evaluation is TOTAL: it never raises. Ask which nodes succeeded.
ev = evaluate(doc)
assert ev.succeeded(lightened), "the bracket did not evaluate"

# Validate, then measure — the same ladder, the same pads.
body = ev.value(lightened).body()
body.validate()
props = body.mass_properties()
assert abs(props.volume - 2.984e-5) < 1e-15
assert props.volume_pad == 0.0

# Export through the document layer, and re-import to prove it.
step = ev.step_string(lightened, product_name="bracket")
assert step.startswith("ISO-10303-21;")
assert abs(import_step(step).mass_properties().volume - props.volume) < 1e-15
```

Two differences from the Rust walk are real and worth stating plainly
rather than hiding:

- **`evaluate` is total.** It never raises. Every node either has a
  value or has failed, and you ask with `succeeded(node)`. Reading
  the value of a failed node is what raises — an `EvaluationError`
  carrying `reason`, `node`, and, for a node poisoned by an upstream
  failure, the `through` node that actually broke. Failure does not
  propagate as an exception up your call stack; it sits in the result
  DAG where you can inspect all of it at once.
- **Tessellation is a `Body` method.** The free `tessellate` above is
  `body.tessellate(chordal)` here, beside `mass_properties` and the
  validators, and δ crosses as a `Length` because it is a distance.
  The mesh cross-check that follows is not a second reading of the
  kernel: `mesh::validate`'s helpers are not bound, so the Python
  ladder's step 5 is a sum the CALLER writes over the mesh's own
  triangles — which is what makes agreeing with the exact measure
  evidence rather than a tautology. `docs/guide/meshing.md` is the
  page for it.

Steps 2.5 and 2.6 finish the same way. The mesh crosses with its
shared position buffer and its per-face patches intact, so both of
the ladder's claims are checkable from Python — closure on INDICES,
volume on the triangles:

```python
from pncad import BooleanOp, Doc, Node, evaluate, m, mm

doc = Doc()
profile = doc.insert(
    Node.polygon([(0 * m, 0 * m), (2 * m, 0 * m), (2 * m, 3 * m), (0 * m, 3 * m)])
)
block = doc.insert(Node.extrude(profile, 1 * m))
body = evaluate(doc).value(block).body()
body.validate()

# 4. Tessellate. The budget is a DISTANCE (delta), not the kernel's
#    epsilon: how coarsely a view of the model may approximate it,
#    not what the model is.
mesh = body.tessellate(0.5 * mm)
assert mesh.patch_count == 6            # one patch per face, addressable
assert mesh.triangle_count == 12

# 5. Cross-check, two independent ways.
#
#    Closure first, on INDICES: adjacent faces share position indices
#    along their common boundary, so every directed triangle edge has
#    exactly one opposite twin. No coordinates and no tolerance.
half_edges = {}
for i, j, k in mesh.triangles:
    for a, b in ((i, j), (j, k), (k, i)):
        half_edges[(a, b)] = half_edges.get((a, b), 0) + 1
assert all(
    n == 1 and half_edges.get(e[::-1]) == 1 for e, n in half_edges.items()
), "watertight and consistently wound"

#    Then volume, by the divergence theorem over the same triangles.
#    The winding is OUTWARD, so this is positive for a closed body.
points = [tuple(q.meters for q in p) for p in mesh.positions]
measured = 0.0
for i, j, k in mesh.triangles:
    (ax, ay, az), (bx, by, bz), (cx, cy, cz) = points[i], points[j], points[k]
    measured += (
        ax * (by * cz - bz * cy)
        - ay * (bx * cz - bz * cx)
        + az * (bx * cy - by * cx)
    )
measured /= 6.0

exact = body.mass_properties().volume
assert abs(measured - exact) / exact < 1e-12, "mesh vs exact"

# 6. Export the mesh. Both writers ANSWER the bytes rather than take
#    a sink, and their options are keyword arguments.
text = mesh.to_stl_ascii(solid_name="block")
assert text.startswith("solid block\n")
data = mesh.to_stl_binary(header="pncad")
assert int.from_bytes(data[80:84], "little") == mesh.triangle_count
```

This body is all planar, so its triangulation is exact and the two
measures agree at rounding level. On a curved body they differ by the
budget, and the difference shrinks with it — `docs/guide/meshing.md`
runs that convergence.

Python's document also persists and replays bit-identically:

```python
from pncad import Doc, Node, evaluate, load, mm

doc = Doc()
profile = doc.insert(Node.polygon([(0 * mm, 0 * mm), (1 * mm, 0 * mm), (1 * mm, 1 * mm)]))
doc.insert(Node.extrude(profile, 1 * mm))

text = doc.save()
replayed = load(text).doc
assert doc.bit_eq(replayed), "replay is bit-identical, not merely close"
```

Bit-identity is a real guarantee, not a hope: the kernel uses pure
libm and a fixed evaluation order (design decision D9), so the same
document replays to the same bits.

### A profile with holes

A profile is a list of loops: the outer boundary first, then the
holes. This is the same `ProfileProgram.loops` the Rust side fills,
so the two languages say one thing — Rust hands `Profile::new` a
`Vec<ProfileLoop>`, Python hands `Node.profile` a list.

Nothing about the loop SET is checked at the boundary. Which loop is
outer, whether the holes nest, whether two loops cross — that is
`Profile::validate`'s work, and it reaches Python as a typed refusal
at `insert`, not as a guess.

```python
import math

from pncad import Doc, EditError, Node, Open, Start, circle, evaluate, m

# The tour's `plate` stop: a 6 x 3 slab, 0.6 deep, with two holes.
outer = (
    Open.at((-3 * m, -1.5 * m))
    .line_to((3 * m, -1.5 * m))
    .line_to((3 * m, 1.5 * m))
    .line_to((-3 * m, 1.5 * m))
    .line_to(Start)
)
holes = [circle((-1.5 * m, 0 * m), 0.7 * m), circle((1.5 * m, 0 * m), 0.7 * m)]

doc = Doc()
sketch = doc.insert(Node.profile([outer, *holes]))
plate = doc.insert(Node.extrude(sketch, 0.6 * m))

body = evaluate(doc).value(plate).body()
body.validate()
area = 6.0 * 3.0 - 2.0 * math.pi * 0.7 * 0.7
assert abs(body.mass_properties().volume - 0.6 * area) < 1e-12

# Two disjoint circles are not an outline and its hole. The kernel
# says so; the binding does not pre-empt it.
try:
    Doc().insert(
        Node.profile([circle((0 * m, 0 * m), 1 * m), circle((5 * m, 0 * m), 1 * m)])
    )
    raise AssertionError("that profile should not have validated")
except EditError as refusal:
    assert refusal.variant == "profile_program_refused"
```

### Filleting edges you name

A fillet blends a SELECTION of a body's edges, and the selection is
stable names — not indices, not "all of them". There is no
every-edge spelling on purpose: a selection is a COMMITMENT, and a
live "all" would silently grow the day an upstream edit adds an edge.

So you materialize the names you want off an evaluation, and store
what you got. `Evaluation.all_edges(node)` is the whole-body
materializer (with `all_faces`, `all_vertices` and `all_bodies`
beside it), and the strings it answers with are carried to
`Node.fillet` unread. Rust says the same thing with
`editor_core::all_edges` and `Node::fillet`.

**A name is an opaque identifier.** It is the name's own serde
encoding — the same value a saved document carries, modulo the
whitespace `save` pretty-prints with — but its internal structure is
NOT API: it may change without notice, so the supported operations
are equality, ordering, storage, and handing it back. Narrowing a
materialized set is a SELECTOR's job: `Evaluation.select` and
`Evaluation.select_where`, the same doors Rust narrows with — the
next section runs them.

```python
import math

from pncad import Doc, EvaluationError, Node, evaluate, m

L, R = 1.0, 0.12

doc = Doc()
square = doc.insert(
    Node.polygon([(0 * m, 0 * m), (L * m, 0 * m), (L * m, L * m), (0 * m, L * m)])
)
cube = doc.insert(Node.extrude(square, L * m))

# The twelve names, as of THIS evaluation. Stored into the recipe,
# they are frozen: the repair path for a moved edge is a rebind, not
# a re-query.
edges = evaluate(doc).all_edges(cube)
assert len(edges) == 12
blank = doc.insert(Node.fillet(cube, R * m, edges))

# The tour's `diefillet` blank: a shrunk core, six slab faces, twelve
# quarter-cylinders and eight sphere octants.
core = L - 2 * R
want = (
    core**3
    + 6 * R * core**2
    + 12 * (math.pi * R * R / 4) * core
    + (4 / 3) * math.pi * R**3
)
body = evaluate(doc).value(blank).body()
body.validate()
assert abs(body.mass_properties().volume - want) < 1e-9 * want

# An empty selection is refused by the node, not by the binding.
empty = doc.insert(Node.fillet(cube, R * m, []))
try:
    evaluate(doc).value(empty)
    raise AssertionError("an empty selection should not blend")
except EvaluationError as refusal:
    assert refusal.kind == "fillet_selection_empty"
```

### Narrowing a selection: select, then fillet

`all_edges` answers a whole kind, and on a boolean output that is
more than one blend wants. The narrowing language is the same one
Rust's selector section speaks, crossed verb for verb:
`Evaluation.select` materializes by role-path SHAPE (`Selector`, a
union of `NamePat`s — which op minted the entity, which role, which
side), and `Evaluation.select_where` filters the survivors by
GEOMETRY (`GeomPred` atoms: carrier kind, adjacent-surface kinds,
datum-relative distance, in conjunction). Both answer in the same
opaque alphabet the materializers speak, ready for `Node.fillet`
unread — narrowing happens through the doors, never by parsing a
name.

The scene below is the composed die's shape in miniature: a cube
with one spherical pip cut into its top face, then ONE fillet whose
selection is said twice geometrically — the box edges by carrier
kind, the pip rim by the plane/sphere pair across it. What is NOT
selected is the point: the cavity's two meridian seams are sphere on
BOTH sides (no dihedral wedge — unfilletable at any radius), and
they match neither filter, so the refusal falls out of the geometry.

```python
import math

from pncad import (
    BooleanOp, Bulge, CurveKind, Doc, EntityKind, GeomPred, NamePat,
    Node, Open, Selector, SketchPlane, Start, SurfaceKind, evaluate, m,
    rad,
)

R, H = 0.09, 0.05  # the pip ball's radius; how deep it dips in

doc = Doc()
square = doc.insert(
    Node.polygon([(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m), (0 * m, 1 * m)])
)
cube = doc.insert(Node.extrude(square, 1 * m))

# A ball, revolved as two quarter arcs, sunk H into the top face.
half = (
    Open.at((0 * m, -R * m))
    .arc_to(Bulge((R * m, 0 * m), math.tan(math.pi / 8)))
    .arc_continue((0 * m, R * m))
    .line_to(Start)
)
plane = SketchPlane.from_frame((0 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0), (0.0, 0.0, 1.0))
axis = doc.insert(Node.datum_axis((0 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0)))
ball = doc.insert(Node.revolve(doc.insert(Node.profile(half, plane=plane)), axis, (2 * math.pi) * rad))
pip = doc.insert(
    Node.transform(ball, (0.5 * m, 0.5 * m, (1.0 + R - H) * m), (0.0, 0.0, 1.0), 0 * rad)
)
pipped = doc.insert(Node.boolean(BooleanOp.Subtract, cube, pip))

# The two filters, materialized off ONE evaluation and stored. A
# list of atoms is a conjunction; a union is two calls concatenated.
edges = Selector.of(NamePat.of_kind(EntityKind.Edge))
ev = evaluate(doc)
straight = ev.select_where(pipped, edges, [GeomPred.curve_kind(CurveKind.Line)])
rims = ev.select_where(
    pipped, edges, [GeomPred.adjacent_kinds(SurfaceKind.Plane, SurfaceKind.Sphere)]
)
assert len(straight) == 12  # the box edges the subtraction kept
assert len(rims) == 2       # the pip rim is two arcs, not one circle

# The excluded remainder, BY GEOMETRY: sphere-on-both-sides.
meridians = ev.select_where(
    pipped, edges, [GeomPred.adjacent_kinds(SurfaceKind.Sphere, SurfaceKind.Sphere)]
)
assert len(meridians) == 2 and len(ev.all_edges(pipped)) == 16

# One fillet takes both selections — stored, frozen, never re-queried.
blended = doc.insert(Node.fillet(pipped, 0.05 * m, straight + rims))
body = evaluate(doc).value(blended).body()
body.validate()
```

## 3. Parametric models

Section 2's Rust walk built a solid by calling operations. That is a
fine way to get one solid, but the value is gone the moment the
function returns: nothing recorded what you did, so nothing can
re-do it with a different number.

The **document layer** records it. A document is a DAG of nodes — a
recipe — and `evaluate` turns the recipe into values. Edits are data
(`DocEdit`), so the history is inspectable, persistable, undoable, and
replayable, and evaluation reuses everything an edit did not touch.
This is the same surface the Python bindings and the GUI speak;
there is deliberately only one.

Since the profiles-as-programs switch, **a profile's geometry is a
program too**: the loops are `LoopProgram` values whose coordinates
are `Expr`s, not baked floats. That is what makes a sketch
parametric rather than opaque.

```
use pncad::prelude::*;
use pncad::document::NodeResult;

let tol = Tol::witness();
// Author a plate with a round hole. Both the outline and the hole
// are programs; every coordinate is an expression.
let len = |v: f64| Expr::literal(v, Dimension::Length).expect("a length");
let outline = LoopProgram::polygon([(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (0.0, 2.0)])
    .expect("finite corners");
let hole = LoopProgram::Circle {
    centre: [len(1.0), len(1.0)],
    radius: len(0.25),
};

let mut doc = Doc::<ProfileProgram>::empty_derived("guide", tol);
let mut insert = |doc: &Doc<ProfileProgram>, node| {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the edit applies");
    (applied.doc, applied.record.minted.expect("a minted id"))
};

let (next, profile) = insert(
    &doc,
    Node::Profile(ProfileProgram {
        plane: SketchPlane::xy(),
        loops: vec![outline, hole],
    }),
);
doc = next;
let (next, plate) = insert(&doc, Node::Extrude { profile, distance: len(0.5) });
doc = next;

let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
assert_eq!(ev.recomputed, 2);
assert_eq!(ev.reused, 0);

// Reach the body the same way the export door does.
let NodeResult::Ok(value) = ev.result(plate).expect("the node is live") else {
    panic!("the plate evaluated");
};
let ValuePayload::Body(body) = &value.payload else {
    panic!("an extrude yields a body");
};
let props = mass_properties(body.as_ref(), tol)?;
let expected = 4.0 * 2.0 * 0.5 - core::f64::consts::PI * 0.25 * 0.25 * 0.5;
assert!((props.volume - expected).abs() < 1e-9);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Now the parametric part: change one number and rebuild, reusing
everything the change did not reach. `SetParam` replaces the
expression in a named **slot** — never an index, so an edit cannot
silently target the wrong argument when the node changes:

```
use pncad::prelude::*;
# let tol = Tol::witness();
# let len = |v: f64| Expr::literal(v, Dimension::Length).expect("a length");
# let outline = LoopProgram::polygon([(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (0.0, 2.0)]).expect("corners");
# let hole = LoopProgram::Circle { centre: [len(1.0), len(1.0)], radius: len(0.25) };
# let mut doc = Doc::<ProfileProgram>::empty_derived("guide", tol);
# let mut insert = |doc: &Doc<ProfileProgram>, node| {
#     let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("applies");
#     (applied.doc, applied.record.minted.expect("minted"))
# };
# let (next, profile) = insert(&doc, Node::Profile(ProfileProgram { plane: SketchPlane::xy(), loops: vec![outline, hole] }));
# doc = next;
# let (next, plate) = insert(&doc, Node::Extrude { profile, distance: len(0.5) });
# doc = next;
# let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
// Make the plate twice as thick.
let thicker = apply(&doc, &DocEdit::SetParam {
    node: plate,
    slot: SlotId::Distance,
    expr: len(1.0),
}, tol)?.doc;

// Pass the PRIOR evaluation: the profile is untouched, so its value
// is reused by content key and only the extrude re-runs.
let ev2 = evaluate::<f64>(&thicker, Some(&ev), &CancelToken::new(), &EvalOptions::default(), tol);
assert_eq!(ev2.recomputed, 1);
assert_eq!(ev2.reused, 1);
# Ok::<(), Box<dyn std::error::Error>>(())
```

`recomputed` and `reused` are not statistics for a progress bar —
they are the acceptance counters for incremental recompute, and the
corpus asserts on them. If an edit recomputes more than its
downstream cone, that is a bug with a number attached.

### 3.1 The parametric flagship: `plate_param`

The example above drives geometry by *editing a slot*. The stronger
form is a **named document parameter** that several places reference,
so one edit moves all of them coherently. That is
`crates/editor-core/tests/corpus/plate_param.rs`, and it is the
corpus document to read after this guide: a plate with **two** holes
whose radii are both `Expr::param("hole_r")` — one parameter, two
loops, one edit.

Its acceptance rows (`crates/editor-core/tests/switch_plate_param.rs`)
are worth knowing because they are the four things you want to be
true of a parametric system, stated as tests:

1. Editing `hole_r` produces genuinely new geometry, and the volume
   moves by exactly `2·π·(r₁²−r₀²)·d` — the *derivative* is the claim,
   not merely "the number changed".
2. One parameter drives both holes: the delta is twice a single
   hole's.
3. `hole_r = 0` refuses at replay with `NonpositiveCircleRadius`, and
   the typed error names the loop and the step that failed.
4. `hole_r = 0.8` (holes overlapping each other) refuses at
   *validate* — a different door from the replay one, which is the
   point: two distinct failure modes stay distinguishable.

Note the deliberate asymmetry in row 3: the `SetDocParam` edit itself
still applies cleanly. A program that refuses under the current
binding is legal *at rest*; the refusal belongs to replay, not to the
edit.

### 3.2 The flagship, façade-only

Named document parameters were LIB-U10's headline finding: the façade
did not re-export `ParamName` or `DocParam`, so `DocEdit::SetDocParam`
and `Expr::param` were doors a `pncad`-only consumer could see and not
open, and a `compile_fail` doctest sat here pinning the hole.
R1-PARAMS cured it — both names are curated through `pncad::document`
(and the prelude), so what follows is `plate_param` itself, authored
through the façade alone and executed as this page's doctest. This is
the standing goal's register at work: the gap was named in the
north-star audit rather than worked around, and closing it flips this
section from a pin to a demonstration.

One parameter, referenced by two loops, moved by one edit:

```
use pncad::prelude::*;
use pncad::document::{BooleanOp, BooleanValue, NodeResult};

let tol = Tol::witness();
let lit = |v: f64| Expr::literal(v, Dimension::Length).expect("a length");
// ONE expression, shared: BOTH holes' radius reads `hole_r`.
let hole = |cx: f64, cy: f64| LoopProgram::Circle {
    centre: [lit(cx), lit(cy)],
    radius: Expr::param(ParamName::new("hole_r"), Dimension::Length),
};

let mut doc = Doc::<ProfileProgram>::empty_derived("guide", tol);

// Declare the parameter. An ordinary edit: recorded, replayable,
// undoable like any other.
doc = apply(&doc, &DocEdit::SetDocParam {
    name: ParamName::new("hole_r"),
    value: DocParam::Continuous { dim: Dimension::Length, value: 0.25 },
}, tol)?.doc;

let mut insert = |doc: &Doc<ProfileProgram>, node| {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the edit applies");
    (applied.doc, applied.record.minted.expect("a minted id"))
};

// The plate: outline plus both parametric holes, one profile.
let outline = LoopProgram::polygon([(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (0.0, 2.0)])
    .expect("finite corners");
let (next, profile) = insert(&doc, Node::Profile(ProfileProgram {
    plane: SketchPlane::xy(),
    loops: vec![outline, hole(1.0, 1.0), hole(2.2, 1.0)],
}));
doc = next;
let (next, plate) = insert(&doc, Node::Extrude { profile, distance: lit(0.5) });
doc = next;

// A plain tab on its own branch — parametrically inert, there so the
// re-evaluation below has a sibling to REUSE.
let (next, tab_p) = insert(&doc, Node::Profile(ProfileProgram {
    plane: SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.125))),
    loops: vec![
        LoopProgram::polygon([(3.5, 1.75), (4.5, 1.75), (4.5, 2.5), (3.5, 2.5)])
            .expect("finite corners"),
    ],
}));
doc = next;
let (next, tab) = insert(&doc, Node::Extrude { profile: tab_p, distance: lit(0.25) });
doc = next;
let (next, solid) = insert(&doc, Node::Boolean {
    op: BooleanOp::Union,
    a: plate,
    b: tab,
    declare: None,
});
doc = next;

let volume = |ev: &Evaluation<f64>, node: RecipeNodeId| {
    let NodeResult::Ok(value) = ev.result(node).expect("the node is live") else {
        panic!("the solid evaluated");
    };
    let ValuePayload::Boolean(BooleanValue::Body { body, .. }) = &value.payload else {
        panic!("a union yields a body");
    };
    mass_properties(body.as_ref(), tol).expect("mass properties").volume
};

// The analytic oracle: plate + tab − their overlap − two cylinders of
// radius r (the same closed form the corpus acceptance rows assert).
let v = |r: f64| {
    4.0 * 2.0 * 0.5 + 1.0 * 0.75 * 0.25
        - 0.5 * 0.25 * 0.25
        - 2.0 * core::f64::consts::PI * r * r * 0.5
};

let ev = evaluate::<f64>(&doc, None, &CancelToken::new(), &EvalOptions::default(), tol);
assert!((volume(&ev, solid) - v(0.25)).abs() < 1e-6);

// One `SetDocParam` moves BOTH holes; the tab branch never re-runs.
let bigger = apply(&doc, &DocEdit::SetDocParam {
    name: ParamName::new("hole_r"),
    value: DocParam::Continuous { dim: Dimension::Length, value: 0.4 },
}, tol)?.doc;
let ev2 = evaluate::<f64>(&bigger, Some(&ev), &CancelToken::new(), &EvalOptions::default(), tol);
assert_eq!(ev2.recomputed, 3); // the profile, the plate, the union
assert_eq!(ev2.reused, 2);     // the tab's whole branch, by content key
assert!((volume(&ev2, solid) - v(0.4)).abs() < 1e-6);
# Ok::<(), Box<dyn std::error::Error>>(())
```

Note what row 3 of §3.1 already told you: `SetDocParam` applies
cleanly even for a value the geometry will refuse — a program that
refuses under the current binding is legal *at rest*, and the refusal
belongs to replay.

From Python the same edit is `DocEdit.set_doc_param(ParamName(…),
DocParam.length(…))`, demonstrated against this exact document in
`crates/pncad-py/tests/test_north_star.py`. Authoring the *profile*
above from Python now awaits exactly ONE door. Circles came with the
audit's G1 and the three-loop profile with G9; what is left is a
profile step whose argument is an EXPRESSION rather than a literal —
the holes above are `LoopProgram::Circle { radius: Expr::param(…) }`,
and `pncad.circle(centre, radius)` takes a `Length`, so the radius
crosses as a number and the parameter link is lost.

## 4. The rest of the documentation

- **`docs/guide/examples.md`** — the corpus as the example set: every
  tour scene and every corpus document, mapped to what it
  demonstrates and which pitfall it pins. Browse this to find the
  worked example nearest your problem.
- **`docs/guide/fail-loud.md`** — the refusal vocabulary, layer by
  layer, with executed examples of reading each one. If you are new
  here and something refused, start there.
- **`docs/guide/selecting.md`** — naming and selecting entities: the
  materializers, the structural pattern language, the geometric
  filters, the doors from a name back to geometry, and the
  detect/declare protocol for flush contact.
- **`docs/guide/north-star-audit.md`** — which demos are authorable
  through the Python bindings today, and the named gap for each that
  is not.
- **`docs/LIBRARY-DESIGN.md`** — why the library is shaped this way.
- **`docs/PATHS-DESIGN.md`** — the authoring algebra in full.
- **`docs/DESIGN.md`** — the kernel's ratified design contract.
