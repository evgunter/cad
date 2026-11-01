"""`Doc.take_over`: copies of a body re-pointed to a result that
replaces it, every name the result carries uniquely re-seated.

The Python face of `refactor::take_over`, which the viewer's fillet,
mirror, sketch-on-face and transform gestures author through. A plate's
world copy is taken over by the union a sketch on its top makes: the
union carries each of the plate's faces as one face of its own, so
each face name of the copy moves to the union's name for it, and the
copy then holds the new names and not the old.
"""

import unittest

from pncad import Doc, Formula, Node, Open, Start, evaluate, m

SQUARE = [
    (Formula.length_in(0, m), Formula.length_in(0, m)),
    (Formula.length_in(1, m), Formula.length_in(0, m)),
    (Formula.length_in(1, m), Formula.length_in(1, m)),
    (Formula.length_in(0, m), Formula.length_in(1, m)),
]


def pad():
    """A 0.4 m square, centred on the sketch origin."""
    path = Open.at((-0.2 * m, -0.2 * m))
    for x, y in [(0.2, -0.2), (0.2, 0.2), (-0.2, 0.2)]:
        path = path.line_to((x * m, y * m))
    return path.line_to(Start)


class TestATakeoverMovesTheCopysNames(unittest.TestCase):
    def setUp(self):
        self.doc = Doc()
        square = self.doc.insert(Node.polygon(SQUARE))
        self.plate = self.doc.insert(Node.extrude(square, Formula.length_in(0.2, m)))
        self.copy = self.doc.place(self.plate, self.doc.world)
        ev = evaluate(self.doc)
        self.faces = ev.all_faces(self.copy)
        top = [
            f
            for f in ev.all_faces(self.plate)
            if abs(ev.face_frame(self.plate, f).origin[2].meters - 0.2) < 1e-12
        ]
        self.assertEqual(len(top), 1, "one face at the top")
        self.sketched = self.doc.sketch_on_face(self.plate, top[0], pad(), 0.1 * m)

    def test_each_face_of_the_copy_moves_to_the_unions_name(self):
        moved = self.doc.take_over([self.copy], self.sketched.union)
        faces = {old: new for (copy, old, new) in moved if old in self.faces}
        self.assertTrue(all(copy == self.copy for (copy, _, _) in moved), moved)
        self.assertEqual(sorted(faces), sorted(self.faces), "every face moved")
        now = set(evaluate(self.doc).all_faces(self.copy))
        for old, new in faces.items():
            with self.subTest(face=old):
                self.assertNotEqual(old, new)
                self.assertIn(new, now, "the copy holds the new name")
                self.assertNotIn(old, now, "and not the old")

    def test_a_copy_list_with_nothing_in_it_takes_nothing_over(self):
        before = self.doc.node_count
        self.assertEqual(self.doc.take_over([], self.sketched.union), [])
        self.assertEqual(self.doc.node_count, before)


if __name__ == "__main__":
    unittest.main()
