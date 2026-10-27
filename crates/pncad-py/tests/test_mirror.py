"""`Node.mirror`: a mirror is a construction (D10 `[repetition]`).

The Python mirror of `crates/editor-core/tests/a_mirror_is_a_construction.rs`:
a body reflected across a plane read off its own geometry is a new body
of the source's volume at the reflected place, and the union of a body
and its mirror across one of its own faces is one symmetric solid.
"""

import unittest

from pncad import Doc, EditError, Formula, Node, evaluate, m

from placing import plane_on
from measuring import pinned


def plate(doc):
    """An L-shaped plate, 2 x 2 with a 1 x 1 notch off its +x, +y
    corner, extruded 0.5 on its own xy plane."""
    corners = [(0, 0), (2, 0), (2, 1), (1, 1), (1, 2), (0, 2)]
    profile = doc.insert(
        Node.polygon([(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in corners])
    )
    return doc.insert(Node.extrude(profile, Formula.length_in(0.5, m)))


class MirrorTest(unittest.TestCase):
    def test_the_image_is_the_source_reflected(self):
        doc = Doc()
        body = plate(doc)
        image = doc.insert(Node.mirror(body, plane_on(doc, body, (0.0, 0.0, 0.0), (-1.0, 0.0, 0.0))))
        ev = evaluate(doc)
        source, mirrored = ev.value(body).body(), ev.value(image).body()
        mirrored.validate()
        self.assertAlmostEqual(
            pinned(mirrored).volume, pinned(source).volume, delta=1e-12
        )
        xs = sorted(
            round(p[0].meters, 12) + 0.0
            for v in ev.all_vertices(image)
            for p in [ev.vertex_position(image, v)]
        )
        self.assertEqual(xs[0], -2.0, "the image reaches x = -2")
        self.assertEqual(xs[-1], 0.0, "the image stands at x <= 0")

    def test_a_body_and_its_mirror_across_its_own_face_union_to_one_solid(self):
        doc = Doc()
        body = plate(doc)
        image = doc.insert(Node.mirror(body, plane_on(doc, body, (0.0, 0.0, 0.0), (0.0, 0.0, -1.0))))
        whole = doc.insert(Node.union([body, image]))
        ev = evaluate(doc)
        union = ev.value(whole).body()
        union.validate()
        self.assertAlmostEqual(
            pinned(union).volume,
            2.0 * pinned(ev.value(body).body()).volume,
            delta=1e-12,
        )

    def test_a_plane_off_another_construction_refuses_its_space(self):
        doc = Doc()
        body = plate(doc)
        other = plate(doc)
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.mirror(body, plane_on(doc, other, (0.0, 0.0, 0.0), (-1.0, 0.0, 0.0))))
        self.assertEqual(caught.exception.variant, "space_mismatch")


if __name__ == "__main__":
    unittest.main()
