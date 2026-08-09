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

STEP export is NOT reachable from here yet: `step_string`/`write_step`
take a kernel `Body`, and the curated document surface exposes no
export door that accepts an evaluated body. That gap is recorded as a
FINDING in the LIB-U9S PR rather than papered over by binding the
kernel directly — which is exactly what §L3 forbids.
"""

from pncad import BooleanOp, Doc, Node, evaluate, mm


def plate(doc, width, depth, thickness):
    """A rectangular plate rooted at the origin."""
    profile = doc.insert(
        Node.polygon(
            [
                (0 * mm, 0 * mm),
                (width, 0 * mm),
                (width, depth),
                (0 * mm, depth),
            ]
        )
    )
    return doc.insert(Node.extrude(profile, thickness))


def main():
    doc = Doc()

    # A base plate, and an upright web standing on it.
    base = plate(doc, 80 * mm, 40 * mm, 8 * mm)
    web = plate(doc, 8 * mm, 40 * mm, 30 * mm)
    bracket = doc.insert(Node.boolean(BooleanOp.Union, base, web))

    # A lightening pocket taken out of the base.
    pocket = plate(doc, 20 * mm, 20 * mm, 8 * mm)
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


if __name__ == "__main__":
    main()
