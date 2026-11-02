"""`DocEdit.set_loft_sections`: a loft's sections written whole, each
with its station.

The Python side of `crates/editor-core/tests/intent_loft_set_sections.rs`:
a section added with its station evaluates through it, a node that is
not a loft refuses `set_loft_sections_on_non_loft`, and `set_members`
handed a different count refuses `loft_section_count`.
"""

import unittest

from pncad import (
    Doc,
    DocEdit,
    EditError,
    Formula,
    Node,
    Open,
    Start,
    StationMove,
    evaluate,
    m,
)

from measuring import pinned


def square(half):
    h = half * m
    return (
        Open.at((-h, -h))
        .line_to((h, -h))
        .line_to((h, h))
        .line_to((-h, h))
        .line_to(Start)
    )


def up(z):
    return [StationMove.slide("z", Formula.length_in(z, m))]


class TestSetLoftSections(unittest.TestCase):
    def setUp(self):
        self.doc = Doc()
        self.a = self.doc.insert(Node.profile(square(0.5)))
        self.b = self.doc.insert(Node.profile(square(0.4)))
        self.c = self.doc.insert(Node.profile(square(0.3)))
        self.loft = self.doc.insert(
            Node.loft(self.a, [(self.b, up(1.0))], Formula.count(1))
        )

    def test_a_section_added_with_its_station_evaluates(self):
        # Broken by a binding that drops `then` or its stations: the
        # volume is the frusta through the sections at the stations
        # written.
        self.doc.apply(
            DocEdit.set_loft_sections(
                self.loft, self.a, [(self.b, up(1.0)), (self.c, up(2.5))]
            )
        )
        run = evaluate(self.doc)
        self.assertTrue(run.succeeded(self.loft))
        volume = pinned(run.value(self.loft).body()).volume
        # A degree-1 loft is a frustum per pair of sections.
        frusta = 4 / 3 * (1.0 * (0.25 + 0.2 + 0.16) + 1.5 * (0.16 + 0.12 + 0.09))
        self.assertAlmostEqual(volume, frusta, places=9)

    def test_a_non_loft_refuses(self):
        # Broken by a tag that moved, or a door that writes sections
        # onto any node.
        with self.assertRaises(EditError) as caught:
            self.doc.apply(
                DocEdit.set_loft_sections(self.b, self.a, [(self.b, up(1.0))])
            )
        self.assertEqual(caught.exception.variant, "set_loft_sections_on_non_loft")
        self.assertEqual(caught.exception.node, self.b)

    def test_a_shape_no_evaluation_lofts_refuses_at_both_doors(self):
        # Broken by a shape check one door asks and the other does not:
        # one section, an empty station, a station that slides by
        # nothing, and degree 2 through two sections, each through
        # `insert` and `set_loft_sections`.
        rows = [
            ([], 1, "too_few_sections"),
            ([(self.b, [])], 1, "empty_station"),
            ([(self.b, up(0.0))], 1, "station_in_first_plane"),
            ([(self.b, up(1.0))], 2, "degree_out_of_range"),
        ]
        for then, degree, fault in rows:
            with self.subTest(fault=fault):
                with self.assertRaises(EditError) as inserted:
                    self.doc.insert(Node.loft(self.a, then, Formula.count(degree)))
                self.assertEqual(inserted.exception.variant, "loft_shape")
                self.assertEqual(inserted.exception.inner_variant, fault)
        for then, fault in [
            ([], "too_few_sections"),
            ([(self.b, [])], "empty_station"),
            ([(self.b, up(0.0))], "station_in_first_plane"),
        ]:
            with self.subTest(fault=fault, door="set_loft_sections"):
                with self.assertRaises(EditError) as rewritten:
                    self.doc.apply(DocEdit.set_loft_sections(self.loft, self.a, then))
                self.assertEqual(rewritten.exception.variant, "loft_shape")
                self.assertEqual(rewritten.exception.inner_variant, fault)
        cubic = self.doc.insert(
            Node.loft(self.a, [(self.b, up(1.0)), (self.c, up(2.0))], Formula.count(2))
        )
        with self.assertRaises(EditError) as dropped:
            self.doc.apply(
                DocEdit.set_loft_sections(cubic, self.a, [(self.c, up(2.0))])
            )
        self.assertEqual(dropped.exception.inner_variant, "degree_out_of_range")

    def test_set_members_keeps_the_count(self):
        # Broken by a count check that is dropped: a third section
        # would have no station.
        with self.assertRaises(EditError) as caught:
            self.doc.apply(DocEdit.set_members(self.loft, [self.a, self.b, self.c]))
        self.assertEqual(caught.exception.variant, "loft_section_count")
        self.assertEqual((caught.exception.count, caught.exception.given), (2, 3))


if __name__ == "__main__":
    unittest.main()
