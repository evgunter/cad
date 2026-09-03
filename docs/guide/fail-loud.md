# The fail-loud tour

Most CAD kernels are built to produce *something*. When the input is
ambiguous they snap, heal, or fall back to a tolerance, and you find
out later — from a part that does not fit, or a mesh with a hole in
it, or a volume that is quietly 0.4% wrong.

This kernel refuses instead. When it would have to assume something
you did not say, it stops and returns a typed value naming the
assumption it declined to make. That is the single most important
thing to understand about using it, so this page is a tour of the
refusals themselves: what they look like at each layer, and how to
read one.

Three properties hold everywhere:

- **Refusals are typed data, never strings.** Every error is an enum
  (Rust) or an exception with attributes (Python). You match on it,
  you read its payload, you branch. Nothing here asks you to parse a
  message.
- **A refusal names the specific thing.** Not "invalid geometry" — the
  loop, the step, the node, the pair of entities, the margin, the
  band.
- **Nothing is softened at a boundary.** The façade adds no `unwrap`
  and no default; the Python bindings translate the same typed
  payloads rather than flattening them to `ValueError`.

Every block below is executed.

## 1. Authoring: the refusal that happens before geometry exists

The PATHS algebra refuses at the moment you *say* the thing, which is
the cheapest possible place — before a profile exists, let alone a
body.

The first class is the **junction check**. Every corner is classified
where it is authored, and a corner that turns out to be *accidentally*
tangent — the classic source of invisible bad geometry — is refused
rather than emitted:

```
use pncad::prelude::*;
use pncad::profile::PathError;

let tol = Tol::witness();
// Arrive heading east, then declare a departure that is also east.
// That is not a corner; it is a tangency nobody asked for.
let refused = Open
    .at(p2(0.0, 0.0))
    .line_to(p2(1.0, 0.0), tol)
    .expect("the leg is fine")
    .toward(1.0, 0.0, tol);

assert!(matches!(refused, Err(PathError::JunctionTangent { margin, .. }) if margin == 0.0));
```

The payload carries the **margin** — how far from tangent the
junction actually was. Zero here, because the two directions are
exactly equal; a near-miss would report the small number instead, and
that number is what tells you whether you have a modelling error or a
sliver.

The second class is structural: a fillet needs a corner, meaning two
rays that actually meet. Turn north, then arrive north, and there is
no corner to round:

```
use pncad::prelude::*;
use pncad::profile::PathError;
use pncad::profile::path::PathNoCornerReason;

let tol = Tol::witness();
let refused = Open
    .at(p2(0.0, 0.0))
    .line_to(p2(1.0, 0.0), tol)
    .expect("the leg is fine")
    .toward(0.0, 1.0, tol)          // departure ray: north. A real corner.
    .expect("north")
    .fillet(0.25, tol)
    .expect("the radius is positive")
    .toward(0.0, 1.0, tol)          // arrival ray: north as well...
    .expect("north again")
    .to(p2(1.0, 2.0), tol);         // ...so the two carriers never meet

assert!(matches!(
    refused,
    Err(PathError::NoCornerForFillet {
        reason: PathNoCornerReason::CarriersParallel,
        ..
    })
));
```

Note what did *not* happen: no arc of radius 0.25 was placed
somewhere plausible, and no zero-length segment was emitted. The
payload carries the radius back, so a caller fitting a blend can
retry with a different one and report exactly what it tried.

A tangency you *declared* but did not build is the mirror case,
`TangencyContradicted`. Declarations are verified, never trusted.

## 2. Profile validation: the loop as a whole

Junction checks are local. Some defects are global, and they are
caught by `Profile::validate` — the door `validated` runs for you:

```
use pncad::prelude::*;
use pncad::profile::ProfileError;

let tol = Tol::witness();
// A bowtie: the two diagonals cross. Every corner is locally fine —
// which is exactly why the lattice AUTHORS it without complaint.
let bowtie: ClosedLoop<f64> = Open
    .at(p2(0.0, 0.0))
    .line_to(p2(1.0, 1.0), tol)?
    .line_to(p2(1.0, 0.0), tol)?
    .line_to(p2(0.0, 1.0), tol)?
    .line_to(Start, tol)?;
let refused = validated(SketchPlane::<f64>::xy(), vec![bowtie.into()], tol);

assert!(matches!(refused, Err(ProfileError::NonSimple { .. })));
# Ok::<(), Box<dyn std::error::Error>>(())
```

Read the two steps: the chain **authors** — the algebra's junction
checks are local, and all four corners are sharp — and then
`validated` refuses it typed. A local check cannot see a global
self-intersection; that is what the profile-level validator is for.
The contract is pinned in the kernel's own suite
(`crates/profile/tests/rejections.rs`), where it moved from the demo
tour: a broken-on-purpose scene is not a use case (Ev's ruling on
#413).

## 3. Contact: the refusal that defines this kernel

Here is the one that surprises people, and the one most worth
understanding. Two boxes stacked so they share a face plane:

```
use pncad::prelude::*;
use pncad::topo::BooleanError;
let tol = Tol::witness();
# type E = Box<dyn std::error::Error>;
# fn slab(z: (f64, f64)) -> Result<Body<f64>, E> {
#     let tol = Tol::witness();
#     let rect: ClosedLoop<f64> = Open
#         .at(p2(0.0, 0.0)).line_to(p2(1.0, 0.0), tol)?
#         .line_to(p2(1.0, 1.0), tol)?.line_to(p2(0.0, 1.0), tol)?.line_to(Start, tol)?;
#     let plane = SketchPlane::from_frame(p3(0.0, 0.0, z.0), v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0));
#     Ok(extrude(&validated(plane, vec![rect.into()], tol)?, Extrusion::Distance(real(z.1 - z.0)), tol)?.body)
# }
let lower = slab((0.0, 1.0))?;   // z from 0 to 1
let upper = slab((1.0, 2.0))?;   // z from 1 to 2 — they meet exactly at z = 1

let refused = union(&lower, &upper, tol);
assert!(matches!(refused, Err(BooleanError::UndeclaredCoincidence { .. })));
# Ok::<(), E>(())
```

Those two boxes obviously form a 1 × 1 × 2 block, and most kernels
will hand you one. This one will not, and the reason is worth stating
carefully.

The kernel has two floating-point planes that are equal *as far as it
can tell at this tolerance*. Treating them as the same face means
deciding that a numerical coincidence was intentional. Sometimes it
is — you meant to glue these parts. Sometimes it is a 0.001 mm
modelling error that a tolerant kernel will silently weld into a part
that cannot be manufactured. **The kernel cannot tell the difference,
so it refuses to guess, and asks you.**

You answer by *declaring* the contact. The declaration is data
attached to the operation — `union_with(&a, &b, &decls)` — and it is
verified, not believed: a declaration whose planes are in fact
distinct is `DeclarationContradicted`. So the fail-loud property
survives the escape hatch.

Working examples of the declared path, in increasing order of realism:
`demos/tour/src/booleans.rs` (the declare door itself),
`demos/tour/src/crosslap.rs` (which asserts *live* that the
undeclared version still refuses, with a "retire this if it ever
stops refusing" panic), and the `table` corpus document, which
declares every leg contact by name through the detect/declare
protocol (`find_flush_candidates` → `declare_node`).

Notice the shape of that protocol: detection *proposes*, a human or a
recipe *declares*. Value equality never classifies on its own — there
is no `detect_and_apply` anywhere in this codebase, and that absence
is deliberate.

## 4. The edit door: refusals before anything is evaluated

Document edits are checked when applied. Deleting a node something
else depends on would leave a dangling reference, so it is refused
and the document is left untouched:

```
use pncad::prelude::*;
use pncad::document::EditError;

let tol = Tol::witness();
let len = |v: f64| Expr::literal(v, Dimension::Length).expect("a length");
let scl = |v: f64| Expr::literal(v, Dimension::Scalar).expect("a scalar");
let square = LoopProgram::polygon([(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)])
    .expect("finite corners");

let doc = Doc::<ProfileProgram>::empty_derived("guide", tol);
// The frame the square is drawn on — a dependency of the profile
// exactly as the profile is a dependency of the extrude.
let applied = apply(&doc, &DocEdit::InsertNode {
    node: Node::Datum(Datum::Frame {
        origin: [len(0.0), len(0.0), len(0.0)],
        u: [scl(1.0), scl(0.0), scl(0.0)],
        v: [scl(0.0), scl(1.0), scl(0.0)],
    }),
}, tol)?;
let (doc, frame) = (applied.doc, applied.record.minted.expect("minted"));
let applied = apply(&doc, &DocEdit::InsertNode {
    node: Node::Profile(ProfileProgram { plane: frame, loops: vec![square] }),
}, tol)?;
let (doc, profile) = (applied.doc, applied.record.minted.expect("minted"));
let doc = apply(&doc, &DocEdit::InsertNode {
    node: Node::Extrude { profile, distance: len(1.0) },
}, tol)?.doc;

let refused = apply(&doc, &DocEdit::DeleteNode { id: profile }, tol);
assert!(matches!(refused, Err(EditError::DeleteWouldDangle { .. })));
assert_eq!(doc.len(), 3, "the refused edit changed nothing");
# Ok::<(), Box<dyn std::error::Error>>(())
```

An edit is a value that either applies or does not. There is no
half-applied state to clean up.

## 5. Evaluation: total, and therefore inspectable

Evaluation is the one layer that does **not** refuse by raising,
and the reason is structural: a document is a DAG, and one broken
node should not hide the state of every other node. So `evaluate` is
*total* — it always returns, and each node carries its own outcome.

```python
from pncad import BooleanOp, Doc, EvaluationError, Node, evaluate, mm


def slab(doc, z0, z1):
    profile = doc.insert(
        Node.polygon(
            [(0 * mm, 0 * mm), (10 * mm, 0 * mm), (10 * mm, 10 * mm), (0 * mm, 10 * mm)],
            plane=doc.sketch_frame(elevation=z0),
        )
    )
    return doc.insert(Node.extrude(profile, z1 - z0))


# The same undeclared coincidence as section 3, now inside a document.
doc = Doc()
lower = slab(doc, 0 * mm, 10 * mm)
upper = slab(doc, 10 * mm, 20 * mm)
glued = doc.insert(Node.boolean(BooleanOp.Union, lower, upper))

ev = evaluate(doc)                     # does NOT raise
assert ev.succeeded(lower)             # the operands are fine...
assert not ev.succeeded(glued)         # ...the union is not

# Reading a failed node's value is what raises, and the payload says why.
try:
    ev.value(glued)
    raise AssertionError("expected a typed refusal")
except EvaluationError as err:
    assert err.reason == "node_failed"
    assert err.node == glued
```

And for this particular refusal, the payload does better than say
why: **the refusal is the menu**. An undeclared-contact refusal
carries the candidate declaration itself — the face pair by stable
name, with the relation the verifier decided — as a typed
`FlushFinding` on the exception, so the recourse is in the error, not
in a doc. The menu has exactly two arms: declare that finding, or
move the geometry. Here is the whole conversation, end to end —
author the undeclared boolean, read the typed menu, declare, succeed:

```python
from pncad import (
    BooleanOp, ContactClass, Doc, EvaluationError, Node, PlaneRelation,
    evaluate, mm,
)


def slab(doc, z0, z1):
    profile = doc.insert(
        Node.polygon(
            [(0 * mm, 0 * mm), (10 * mm, 0 * mm), (10 * mm, 10 * mm), (0 * mm, 10 * mm)],
            plane=doc.sketch_frame(elevation=z0),
        )
    )
    return doc.insert(Node.extrude(profile, z1 - z0))


doc = Doc()
lower = slab(doc, 0 * mm, 10 * mm)
upper = slab(doc, 10 * mm, 20 * mm)   # they meet exactly at z = 10 mm
naive = doc.insert(Node.boolean(BooleanOp.Union, lower, upper))

# 1. The undeclared union refuses — with the typed menu attached.
ev = evaluate(doc)
try:
    ev.value(naive)
    raise AssertionError("the undeclared union must refuse")
except EvaluationError as err:
    assert err.kind == "undeclared_contact"
    menu = err.finding                      # the candidate declaration
    assert menu.relation == PlaneRelation.SameOpposite  # resting contact
    assert menu.class_ == ContactClass.Rest

# 2. The declare arm: detect, INSPECT, declare. The detector is the
#    boolean's own verifier run in candidate-generation mode, so a
#    finding can never disagree with verify-at-use — and the menu's
#    finding is drawn from the same inventory.
findings = ev.find_flush_candidates(lower, upper)
assert menu in findings
decl = doc.declare_all(findings)            # or doc.declare(menu)

# 3. The SAME union, with the contact declared: verified and glued.
glued = doc.insert(Node.boolean(BooleanOp.Union, lower, upper, declare=decl))
body = evaluate(doc).value(glued).body()
body.validate()
# 10 × 10 × 20 mm³ — one block, watertight.
assert abs(body.mass_properties().volume - 2e-6) < 1e-15
```

Notice what is *not* here: no `detect_and_declare`. Findings pass
through your hands as values — that pause is the enforceable
intent-recording property, and the declaration is still verified at
use (a declaration the geometry contradicts refuses loudly).

A node downstream of a failure is not itself broken — it is
**poisoned**, and it says so, naming the node that actually failed:

```python
from pncad import BooleanOp, Doc, EvaluationError, Node, evaluate, mm

doc = Doc()


def slab(z0, z1):
    profile = doc.insert(
        Node.polygon(
            [(0 * mm, 0 * mm), (10 * mm, 0 * mm), (10 * mm, 10 * mm), (0 * mm, 10 * mm)],
            plane=doc.sketch_frame(elevation=z0),
        )
    )
    return doc.insert(Node.extrude(profile, z1 - z0))


lower = slab(0 * mm, 10 * mm)
upper = slab(10 * mm, 20 * mm)
broken = doc.insert(Node.boolean(BooleanOp.Union, lower, upper))
third = slab(-20 * mm, -10 * mm)
downstream = doc.insert(Node.boolean(BooleanOp.Union, broken, third))

ev = evaluate(doc)
try:
    ev.value(downstream)
    raise AssertionError("expected a typed refusal")
except EvaluationError as err:
    assert err.reason == "poisoned"
    assert err.through == broken, "the poisoned node names the one that broke"
```

That `through` field is the difference between "something upstream
failed" and a debugging session. In a 500-node document it points
straight at the culprit.

## 6. Validation: a vector, not the first complaint

The tier ladder returns `Result<(), Vec<ValidationError>>`. The vector
is deliberate: a body with a real problem usually has several
symptoms, and reporting only the first one costs you a round trip per
symptom.

```
use pncad::prelude::*;
let tol = Tol::witness();
# type E = Box<dyn std::error::Error>;
# let rect: ClosedLoop<f64> = Open
#     .at(p2(0.0, 0.0)).line_to(p2(1.0, 0.0), tol)?
#     .line_to(p2(1.0, 1.0), tol)?.line_to(p2(0.0, 1.0), tol)?.line_to(Start, tol)?;
# let body = extrude(&validated(SketchPlane::<f64>::xy(), vec![rect.into()], tol)?, Extrusion::Distance(real(1.0)), tol)?.body;
match validate_geometric(&body, tol) {
    Ok(()) => { /* the body is sound at tier 3 */ }
    Err(failures) => {
        // Every failure, not just the first: report them all at once.
        for failure in &failures {
            eprintln!("tier 3 refusal: {failure:?}");
        }
        panic!("{} validation failures", failures.len());
    }
}
# Ok::<(), E>(())
```

In Python the same ladder raises `ValidationError` carrying `door`
(which gate refused) and `failure_count`. For a worked refusal, see
the `plate_param` corpus row where a hole radius grows until the two
holes overlap: it refuses at *validate*, a different door from the
replay refusal the same document produces at radius zero. Two failure
modes, two doors, kept distinguishable.

## 7. The boundary layers: quantities, literals, export

The Python boundary refuses before a bad value ever reaches the
kernel. Dimensions are checked by construction:

```python
from pncad import DimensionError, LiteralError, Node, PncadError, deg, mm

try:
    25 * mm + 90 * deg
    raise AssertionError("expected a typed refusal")
except DimensionError as err:
    assert (err.op, err.left, err.right) == ("+", "length", "angle")

# A bare number is not a length either — the dimension is named.
try:
    25 * mm + 3
    raise AssertionError("expected a typed refusal")
except DimensionError as err:
    assert err.right == "scalar"

# Non-finite values are refused where they enter, not where they explode.
try:
    Node.extrude(None, float("nan") * mm)
    raise AssertionError("expected a typed refusal")
except (LiteralError, TypeError) as err:
    if isinstance(err, LiteralError):
        assert err.kind == "non_finite"

# Every one of these is a PncadError, so a caller can catch the family.
assert issubclass(DimensionError, PncadError)
```

Export refuses in the same style. `ExportError` names the node and,
when the value was the wrong shape, the kind it actually found — so
"you asked me to export a profile" is a matchable fact rather than a
message. On the Rust side, STEP export has three refusals you should
*expect* rather than fix — `UnsupportedSurface`, `UnsupportedCurve`,
`CurvedShellClassification` — meaning the AP242 writer has no
representation for a surface the modeller can legitimately build. The
tour tolerates exactly those three, per scene, and panics on anything
else. That is the right posture to copy: enumerate the refusals you
accept, and let every other one be loud.

## 8. The mesh door: budgets and files

Tessellation refuses on the SAME principle, one layer further out. δ
is refused rather than clamped — a zero, negative or non-finite
budget is not a request the kernel can round into a sensible one —
and the STL writers refuse a name or header they cannot write AT THE
CALL rather than emitting a file that no reader can parse.

```python
from pncad import Doc, Node, PncadError, StlError, TessellateError, evaluate, m, mm

doc = Doc()
sketch = doc.insert(
    Node.polygon([(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m), (0 * m, 1 * m)], plane=doc.sketch_frame())
)
cube = doc.insert(Node.extrude(sketch, 1 * m))
body = evaluate(doc).value(cube).body()

# Refused, never clamped. `value` is the budget that was rejected.
try:
    body.tessellate(-1 * mm)
    raise AssertionError("expected a typed refusal")
except TessellateError as refusal:
    assert refusal.variant == "invalid_chordal_tolerance"
    assert refusal.value == -0.001
    # The payload attributes are always PRESENT, `None` where this arm
    # has nothing to put there — so a caller reads without a trap.
    assert refusal.bound is None and refusal.note is None

# Sanitizing a name would produce a file that parses to the wrong
# thing, which is worse than not writing one.
mesh = body.tessellate(1 * mm)
try:
    mesh.to_stl_ascii(solid_name="two\nlines")
    raise AssertionError("expected a typed refusal")
except StlError as refusal:
    assert refusal.variant == "solid_name_unrepresentable"

assert issubclass(TessellateError, PncadError)
assert issubclass(StlError, PncadError)
```

One honest wrinkle, since this page is about reading refusals:
`TessellateError`'s human message is a `Debug` rendering rather than
the door's own prose, because `mesh::TessellateError` implements no
`Display`. The `variant` tag is unaffected and is what a caller
branches on — which is the general rule this page teaches, holding up
in the one place the prose is weakest.

## Reading a refusal, in general

1. **Match the variant.** It names the class of thing that went
   wrong.
2. **Read the payload.** It names the specific entity, node, loop,
   step, or pair.
3. **If there is a margin and a band, the answer is not "loosen the
   tolerance".** An `Escalated` refusal means the decision was
   genuinely in-band — a sliver — and the model is ill-conditioned at
   this ε. The recourse is to fix the geometry or state the intent,
   not to widen the band until the kernel stops noticing.
4. **If it is a coincidence refusal, decide whether you meant it.**
   If you did, declare it. If you did not, you just found a bug in
   your model that a tolerant kernel would have shipped.
