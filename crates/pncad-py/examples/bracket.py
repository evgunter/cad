"""The one-shot user journey, in Python.

§L3's spirit: "build a bracket, export STEP" is served by a SMALL
DOCUMENT, not by a second, kernel-bypassing API. So this script does
what a Rust author would do — insert nodes, evaluate, measure — and
never reaches past the document layer.

The plate is the REAL bracket now: its four corners are rounded, and
they are rounded the way the algebra says rounds happen — you author
the two rays that would have met at each corner and the arc is fitted
to their virtual intersection, trimming both. There is no moment at
which the sharp corner exists and is then removed. The loop is fully
filleted, so no side is privileged: it is a cyclic sequence of
[corner, side-bindings] units, and the seam fillet at the end reads
exactly like the three interior ones.

Run it against a built module:

    cargo build -p pncad-py --features extension-module
    cp target/debug/libpncad_py.so /tmp/pncad-stage/pncad.so
    PYTHONPATH=/tmp/pncad-stage python3 crates/pncad-py/examples/bracket.py

(or `maturin develop` in a virtualenv, which does the staging for you)

The journey ends at a real STEP file (`Evaluation.step_string` is
the document-layer export door), and the
script then re-reads its own output with the kernel's importer — the
file must exist AND parse, asserted rather than assumed.
"""

import math
import os
import tempfile

from pncad import BooleanOp, Doc, Node, Open, Start, evaluate, import_step, mm

PLATE = (80, 40)  # mm
# The plate's corner radius. It is BOUNDED at 4 mm, and the bound is
# the pocket's: a corner round of radius r rides a carrier circle
# spanning x in [0, 2r], and a Boolean refuses when a cutter's plane
# crosses that carrier — even where it stays clear of the arc itself.
# The pocket's x = 8 mm wall therefore requires 2r <= 8.
CORNER = 3 * mm


def rounded_plate(doc, width, height, radius, thickness):
    """The plate outline: a rectangle whose four corners are fillets.

    The entry sits mid-edge and heads east; each corner is a `fillet`
    followed by the next side's ray and anchor; the last `fillet`
    closes on `Start`, which is the seam. `toward` rather than
    `angle`: only the RATIO of the components carries meaning, so an
    axis direction is stored verbatim and the trim points are exact —
    `angle(pi)` would go through `sin`, and 1.22e-16 of tilt would
    move every vertex it touches by an ulp.
    """
    east, north, west, south = (1.0, 0.0), (0.0, 1.0), (-1.0, 0.0), (0.0, -1.0)
    sides = [
        (north, (width * mm, height / 2 * mm)),
        (west, (width / 2 * mm, height * mm)),
        (south, (0 * mm, height / 2 * mm)),
    ]
    tip = Open.toward(*east).at((width / 2 * mm, 0 * mm))
    for ray, anchor in sides:
        tip = tip.fillet(radius).toward(*ray).at(anchor)
    outline = tip.fillet(radius).to(Start)
    assert outline.vertex_count == 8, "four arcs, two tangent points each"
    return doc.insert(Node.extrude(doc.insert(Node.profile(outline)), thickness))


def slab(doc, x, y, z):
    """The axis-aligned box [x0,x1] x [y0,y1] x [z0,z1]."""
    x0, x1 = x
    y0, y1 = y
    z0, z1 = z
    profile = doc.insert(
        Node.polygon(
            [(x0, y0), (x1, y0), (x1, y1), (x0, y1)],
            elevation=z0,
        )
    )
    return doc.insert(Node.extrude(profile, z1 - z0))


def main():
    doc = Doc()

    # The kernel is fail-loud about coincidence — it never INFERS that
    # two faces are the same face — so every solid here genuinely
    # interpenetrates the one it is combined with. Boxes that merely
    # touch on a shared plane are refused until the contact is
    # declared, which is a document-authoring subject of its own.

    # A rounded base plate, and an upright web sunk into it and
    # poking out.
    base = rounded_plate(doc, PLATE[0], PLATE[1], CORNER, 8 * mm)
    web = slab(doc, (36 * mm, 44 * mm), (5 * mm, 35 * mm), (4 * mm, 34 * mm))
    bracket = doc.insert(Node.boolean(BooleanOp.Union, base, web))

    # A lightening pocket, entering from below and stopping inside.
    pocket = slab(doc, (8 * mm, 28 * mm), (10 * mm, 30 * mm), (-2 * mm, 5 * mm))
    lightened = doc.insert(Node.boolean(BooleanOp.Subtract, bracket, pocket))

    ev = evaluate(doc)
    print(f"document: {doc.node_count} nodes, tolerance {doc.epsilon}")
    print(f"evaluated: {ev.recomputed} recomputed, {ev.reused} reused")

    for node in doc.order():
        state = ev.value(node).kind if ev.succeeded(node) else "FAILED"
        print(f"  {node!r}: {state}")

    if not ev.succeeded(lightened):
        # Fail loud: the kernel refused, so the script refuses too.
        raise SystemExit("the bracket did not evaluate")

    body = ev.value(lightened).body()
    body.validate()

    props = body.mass_properties()
    print(f"volume       {props.volume:.9f} m^3  (+/- {props.volume_pad:g})")
    print(f"surface area {props.surface_area:.9f} m^2  (+/- {props.area_pad:g})")

    # The closed form, stated rather than trusted: an 80 x 40 plate
    # less what the four corner rounds take off, times 8 thick; plus
    # the web's 30 mm of stand-off; less the pocket's 5 mm bite.
    r = CORNER.meters
    plate_area = 0.080 * 0.040 - 4.0 * (r * r - math.pi * r * r / 4.0)
    expected = (
        plate_area * 0.008
        + 0.008 * 0.030 * 0.030
        - 0.008 * 0.030 * 0.004
        - 0.020 * 0.020 * 0.005
    )
    assert abs(props.volume - expected) < 1e-15, (props.volume, expected)

    # Export STEP through the document layer (build -> measure ->
    # export, the whole one-shot journey), then prove the file is real:
    # it exists, and the kernel's own importer parses it back to a
    # solid with the same volume.
    step = ev.step_string(lightened, product_name="bracket")
    path = os.path.join(tempfile.mkdtemp(prefix="pncad-bracket-"), "bracket.step")
    with open(path, "w", encoding="ascii") as out:
        out.write(step)
    assert os.path.exists(path), "the STEP file was written"

    with open(path, encoding="ascii") as inp:
        reimported = import_step(inp.read())
    volume = reimported.mass_properties().volume
    assert abs(volume - props.volume) < 1e-12, "the export round-trips"
    print(f"exported     {path} ({os.path.getsize(path)} bytes; re-imported OK)")


if __name__ == "__main__":
    main()
