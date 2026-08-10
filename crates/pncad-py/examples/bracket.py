"""The one-shot user journey, in Python.

§L3's spirit: "build a bracket, export STEP" is served by a SMALL
DOCUMENT, not by a second, kernel-bypassing API. So this script does
what a Rust author would do — insert nodes, evaluate, measure — and
never reaches past the document layer.

Run it against a built module:

    cargo build -p pncad-py --features extension-module
    cp target/debug/libpncad_py.so /tmp/pncad-stage/pncad.so
    PYTHONPATH=/tmp/pncad-stage python3 crates/pncad-py/examples/bracket.py

(or `maturin develop` in a virtualenv, which does the staging for you)

The journey ends at a real STEP file (LIB-DOORS F2 closed the export
gap: `Evaluation.step_string` is the document-layer door), and the
script then re-reads its own output with the kernel's importer — the
file must exist AND parse, asserted rather than assumed.
"""

import os
import tempfile

from pncad import BooleanOp, Doc, Node, evaluate, import_step, mm


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

    # A base plate, and an upright web sunk into it and poking out.
    base = slab(doc, (0 * mm, 80 * mm), (0 * mm, 40 * mm), (0 * mm, 8 * mm))
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

    # Export STEP through the document layer (build -> measure ->
    # export, the whole §L3 journey), then prove the file is real:
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
