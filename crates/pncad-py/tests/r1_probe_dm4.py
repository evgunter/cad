"""R1 review probes for PR #4674 (not part of the suite: run directly).

Each row prints what it observed; the asserted expectations are what a
reviewer expects of a consistent door set.
"""

import sys

from pncad import Doc, DocEdit, Formula, Node, PncadError, deg, evaluate, m


def chute(doc):
    poly = [(1.0, 0.0), (1.75, 0.0), (1.75, 0.625), (1.0, 0.625)]
    frame = doc.sketch_frame()
    profile = doc.insert(
        Node.polygon(
            [(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in poly],
            plane=frame,
        )
    )
    axis = (
        (Formula.length_in(0, m), Formula.length_in(0, m)),
        (Formula.literal(0.0), Formula.literal(1.0)),
    )
    return doc.insert(Node.revolve(profile, *axis, Formula.angle_in(270, deg)))


def main():
    failures = []
    doc = Doc()
    rev = chute(doc)
    ev = evaluate(doc)
    faces = ev.all_faces(rev)
    print("P4py all_faces(revolve node):", len(faces))
    name = faces[0]
    print("P4py denotation(revolve node):", ev.denotation(rev, name))
    try:
        verdict = ev.resolve(rev, name)
        print("P4py resolve(revolve node):", verdict.status)
    except ValueError as e:
        print("P4py resolve(revolve node) raised ValueError:", e)
        failures.append("resolve(revolve node) raises while denotation/all_faces answer")

    # A Var whose node was deleted, the name's minting node live: the
    # docstring promises node_gone for a gone NODE; a gone VAR?
    doc = Doc()
    square = doc.insert(
        Node.polygon(
            [
                (Formula.length_in(x, m), Formula.length_in(y, m))
                for x, y in ((0, 0), (1, 0), (1, 1), (0, 1))
            ],
            plane=doc.sketch_frame(),
        )
    )
    cube = doc.insert(Node.extrude(square, Formula.length_in(1.0, m)))
    moved = doc.insert(
        Node.transform(
            cube,
            (Formula.length_in(3, m), Formula.length_in(0, m), Formula.length_in(0, m)),
            (Formula.literal(0.0), Formula.literal(0.0), Formula.literal(1.0)),
            Formula.angle_in(0, deg),
        )
    )
    name = evaluate(doc).all_faces(moved)[0]
    body = doc.output(moved)
    doc.apply(DocEdit.delete_node(moved))
    after = evaluate(doc)
    v = after.resolve(body, name)
    print("P6py resolve(gone var):", v.status, v.variant, v.detail)
    n = after.resolve(moved, name)
    print("P6py resolve(gone node):", n.status, n.variant)
    if v.variant != n.variant:
        failures.append("a gone var and a gone node answer differently")
    print("FAILURES:", failures)
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
