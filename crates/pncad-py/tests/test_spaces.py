"""D′1 (`docs/INTENT-STAGE3-SPEC.md` §5′.1, test 23) through Python: an
operator's operands share one space, decided from the recipe at the
edit door, before any evaluation. A construction is in a space of its
own; only a placement relates two spaces."""

import unittest

from pncad import Doc, DocEdit, EditError, Node, m
from test_document import unit_box


class Spaces(unittest.TestCase):
    def boxes(self):
        """Two unit boxes, each a construction in its own space, and a
        copy of the second standing in the first's space."""
        doc = Doc("py-spaces")
        a = unit_box(doc, 1 * m, 1 * m, 1 * m)
        b = unit_box(doc, 1 * m, 1 * m, 1 * m)
        copy = doc.place(b, doc.space(a))
        return doc, a, b, copy

    def assert_two_spaces(self, doc, edit):
        with self.assertRaises(EditError) as caught:
            doc.apply(edit)
        self.assertEqual(caught.exception.variant, "space_mismatch")

    def test_an_operator_over_two_constructions_refuses_at_the_door(self):
        doc, a, b, copy = self.boxes()
        self.assertNotEqual(doc.space(a), doc.space(b))
        self.assertEqual(doc.space(copy), doc.space(a))
        self.assert_two_spaces(doc, DocEdit.insert_node(Node.union([a, b])))
        # In one space, the same operator is admitted.
        doc.insert(Node.union([a, copy]))

    def test_a_re_point_across_two_spaces_refuses_at_the_door(self):
        doc, a, b, copy = self.boxes()
        cut = doc.insert(Node.subtract(a, copy))
        fused = doc.insert(Node.union([a, copy]))
        self.assert_two_spaces(doc, DocEdit.set_param(cut, "cut", b))
        self.assert_two_spaces(doc, DocEdit.set_members(fused, [a, b]))


if __name__ == "__main__":
    unittest.main()
