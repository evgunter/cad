# Meshing: the ladder's steps 4 and 5

The guide's ladder is *author → validate → measure → tessellate →
cross-check → export*. Steps 4 and 5 are the pair this page is about,
and they are a pair on purpose: a mesh you have not cross-checked is
a picture, and a cross-check without a second, independent measure is
a tautology.

Everything below runs. The Python blocks are executed by
`crates/pncad-py/tests/test_guide.py` and the door's own suite is
`crates/pncad-py/tests/test_mesh.py`.

## δ is a budget, not a tolerance

The chordal parameter δ is **a distance in metres**: the furthest the
piecewise-linear mesh may sag from the exact analytic surface. It is
deliberately *not* the kernel's ε.

| | δ (chordal) | ε (kernel tolerance) |
|---|---|---|
| what it decides | how coarsely a **view** of the model may approximate it | what the **model is** — predicate bands, residual certification |
| where it is set | per call, at `body.tessellate(δ)` | per document, `DocEdit.set_tolerance` |
| effect on the body | none. Two budgets see the same solid | it *is* the body's definition |

So δ crosses as a `Length`, like every other distance on this
surface, and nothing is pre-checked: a zero, negative or non-finite
budget is the kernel's own refusal, raised where you wrote the call.

```python
from pncad import Doc, Node, TessellateError, evaluate, m, mm

doc = Doc()
sketch = doc.insert(
    Node.polygon([(0 * m, 0 * m), (2 * m, 0 * m), (2 * m, 1 * m), (0 * m, 1 * m)], plane=doc.sketch_frame())
)
slab = doc.insert(Node.extrude(sketch, 1 * m))
body = evaluate(doc).value(slab).body()

mesh = body.tessellate(0.5 * mm)
assert mesh.triangle_count == 12          # a box, however fine the budget

# Refused, never clamped — and the refusal names the arm and the value.
try:
    body.tessellate(0 * mm)
    raise AssertionError("expected a typed refusal")
except TessellateError as refusal:
    assert refusal.variant == "invalid_chordal_tolerance"
    assert refusal.value == 0.0
```

The bound δ carries is **certified-conservative**: every emitted
triangle is checked against a closed-form, per-surface-kind deviation
certificate before the mesh is handed back. A triangle that cannot be
certified is a refusal (`certificate_exceeded`, carrying `bound` and
`requested`), not a shipped approximation.

## What crosses, and what does not

A `Mesh` is the kernel's own value, and it keeps two contracts across
the boundary.

**One shared position buffer.** `mesh.positions` is the single array
every triangle indexes into. Adjacent faces share the *indices* along
their common boundary, which is why a closed body's mesh is
watertight by construction rather than by a repair pass — and why
checking that from Python compares integers, with no coordinate
comparison and no tolerance anywhere in it.

**Per-face patches.** `mesh.patch(i)` answers one face's triangles;
`mesh.triangles` is those same patches concatenated in the fixed
export order — the walk both STL writers make, so the array and an
exported file agree facet for facet.

What does **not** cross is the picking chain. A patch's face, a
boundary polyline's edge and their vertex back-references are arena
keys, and keeping those unnameable is what the whole curated surface
is for. So a patch is addressed by INDEX here, and the per-edge
boundary polylines — whose only content beside indices is those keys
— are not bound at all. A door from a patch to a `StableName` would
be the honest shape and does not exist on either side of the
boundary; see the north-star audit's G11 row.

## Step 5, written by the caller

Rust cross-checks with `pncad::mesh::validate`'s `check_mesh`,
`signed_volume` and `triangle_count`. Python does not bind them, and
that is the deliberate half of this door: those helpers are not on
the façade's curated lists, and — more to the point — a cross-check
is only evidence when the second measure comes from somewhere else.
The mesh's arrays are the somewhere else.

```python
import math

from pncad import Doc, Node, deg, evaluate, m, mm


def signed_volume(mesh):
    """The divergence theorem over the mesh's own triangles.

    Each tetrahedron is measured from `o`, the positions' bounding-box
    centre. For a closed mesh the anchor cancels out over the reals, so
    this is the same volume from any anchor; in floating point it is
    the choice that keeps the products at the body's own scale instead
    of at its distance from the world origin.
    """
    p = [tuple(q.meters for q in point) for point in mesh.positions]
    if not p:
        return 0.0
    lo = [min(q[d] for q in p) for d in range(3)]
    hi = [max(q[d] for q in p) for d in range(3)]
    o = [lo[d] + (hi[d] - lo[d]) * 0.5 for d in range(3)]
    total = 0.0
    for i, j, k in mesh.triangles:
        (ax, ay, az), (bx, by, bz), (cx, cy, cz) = (
            tuple(q[d] - o[d] for d in range(3)) for q in (p[i], p[j], p[k])
        )
        total += (
            ax * (by * cz - bz * cy)
            - ay * (bx * cz - bz * cx)
            + az * (bx * cy - by * cx)
        )
    return total / 6.0


def is_closed(mesh):
    """Every directed triangle edge has exactly one opposite twin."""
    seen = {}
    for i, j, k in mesh.triangles:
        for a, b in ((i, j), (j, k), (k, i)):
            seen[(a, b)] = seen.get((a, b), 0) + 1
    return all(n == 1 and seen.get(e[::-1]) == 1 for e, n in seen.items())


# A washer: a rectangle revolved a full turn about the y axis, so
# every lateral face is a cylinder and the mesh is an approximation.
doc = Doc()
frame = doc.sketch_frame()
outline = doc.insert(
    Node.polygon(
        [(0.5 * m, 0 * m), (1.5 * m, 0 * m), (1.5 * m, 2 * m), (0.5 * m, 2 * m)],
        plane=frame,
    )
)
# The axis in the sketch's own coordinates: the frame's v is world
# +y, so the world y axis IS its own +y through (0, 0).
axis = doc.insert(Node.datum_axis_in_plane(frame, (0 * m, 0 * m), (0.0, 1.0)))
washer = doc.insert(Node.revolve(outline, axis, 360 * deg))

body = evaluate(doc).value(washer).body()
body.validate()
exact = body.mass_properties().volume
assert abs(exact - math.pi * (1.5**2 - 0.5**2) * 2.0) < 1e-9

# The error is first order in the budget: the mesh is inscribed, so
# it under-measures, and quartering delta buys roughly a quarter of
# the error back. "Roughly a quarter" is the shape of the curve, not
# what the assertion checks: that is a CONVERGENCE floor — each
# quartering must buy at least a factor of two — left deliberately
# looser than the rate, because asserting a rate would pin the
# tessellator's internals rather than the geometry.
errors = []
for budget in (20 * mm, 5 * mm, 1 * mm):
    mesh = body.tessellate(budget)
    assert is_closed(mesh), "watertight at every budget"
    errors.append(abs(signed_volume(mesh) - exact) / exact)

assert errors[0] > 2 * errors[1] > 4 * errors[2]
assert errors[-1] < 1e-3
```

A positive signed volume is itself a check: it says the winding
really is outward. The patches promise counterclockwise-seen-from-
outside triangles **in the outward frame** — the tessellator reaches
that without consulting a face's sense bit — so a consumer derives
orientation from the winding alone and must not re-apply a sense on
top of it.

## Step 6 for the mesh: STL

The two writers answer the bytes rather than take a sink, because
Python holds no `Write`. Their option structs are keyword arguments,
and the two validated newtypes behind them cross as the `str` those
arguments take — so a name or header that cannot be written is
refused **at the call**, not when someone later fails to open the
file.

```python
from pncad import Doc, Node, StlError, evaluate, m, mm

doc = Doc()
sketch = doc.insert(
    Node.polygon([(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m), (0 * m, 1 * m)], plane=doc.sketch_frame())
)
cube = doc.insert(Node.extrude(sketch, 1 * m))
mesh = evaluate(doc).value(cube).body().tessellate(1 * mm)

text = mesh.to_stl_ascii(solid_name="cube")
assert text.startswith("solid cube\n") and "endsolid cube" in text
assert text.count("facet normal") == mesh.triangle_count

data = mesh.to_stl_binary(header="pncad, from the guide")
assert len(data) == 84 + 50 * mesh.triangle_count
assert int.from_bytes(data[80:84], "little") == mesh.triangle_count

# A newline would make `endsolid <name>` unmatchable, so the name is
# refused rather than sanitized into a file no parser can read.
try:
    mesh.to_stl_ascii(solid_name="two\nlines")
    raise AssertionError("expected a typed refusal")
except StlError as refusal:
    assert refusal.variant == "solid_name_unrepresentable"

# A binary header that reads as the `solid` keyword makes the file
# sniff as ASCII STL in readers that decide by it — refused, and the
# recognised class is wider than a byte-exact prefix.
try:
    mesh.to_stl_binary(header=" Solid v2")
    raise AssertionError("expected a typed refusal")
except StlError as refusal:
    assert refusal.variant == "binary_header_sniffs_ascii"
```

Writing the file is then Python's own business — `open(path, "wb")`
and the bytes — which is the same division `Evaluation.step_string`
already makes.

## Reading a tessellation refusal

`TessellateError` carries `variant`, the refusing arm's stable tag,
and the arm's numbers as attributes — always present, `None` where
the arm has nothing to put there. The offending face or edge is an
arena key and does not cross, so what a caller gets is *which* arm
fired and, where there is one, the number that makes it actionable:

- `invalid_chordal_tolerance` — `value` is the budget that was
  refused;
- `resolution_overflow` — `value` is the chord or grid count that
  blew the sanity cap, i.e. δ was small enough to be a mistake;
- `certificate_exceeded` — `bound` is the worst triangle's
  closed-form deviation and `requested` the budget it was checked
  against;
- `unsupported_surface` / `unsupported_nurbs_face` /
  `unsupported_curve` — a lane that is not written, with `note`
  carrying the kernel's own prose about which one and why. Partial
  coverage stated typed beats a dishonest bound.

One honest wrinkle: the message on this class is a `Debug` rendering
rather than the door's own prose, because `mesh::TessellateError`
implements no `Display`. The tag is the branchable part and is not
affected; the message will improve when the kernel type grows one.
