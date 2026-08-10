"""Verification for the north-star audit's YES rows (LIB-U10).

`docs/guide/north-star-audit.md` claims that a handful of demo-tour
scenes are authorable through the Python bindings today. A claim like
that rots the moment the tour or the bindings move, so it is not left
as prose: every YES row is built here, in Python, and checked against
the same exact volume oracle the Rust scene asserts.

A row that stops being true fails this file. A gap that CLOSES makes
`test_the_named_gaps_are_still_gaps` fail, which is the intended
prompt to move a NO row to YES.
"""

import math
import unittest

from pncad import BooleanOp, Doc, DocEdit, Node, deg, evaluate, m


def slab(doc, x, y, z):
    """The axis-aligned box [x0,x1] x [y0,y1] x [z0,z1], in metres."""
    profile = doc.insert(
        Node.polygon(
            [
                (x[0] * m, y[0] * m),
                (x[1] * m, y[0] * m),
                (x[1] * m, y[1] * m),
                (x[0] * m, y[1] * m),
            ],
            elevation=z[0] * m,
        )
    )
    return doc.insert(Node.extrude(profile, (z[1] - z[0]) * m))


def volume_of(doc, node):
    ev = evaluate(doc)
    assert ev.succeeded(node), "the scene evaluated"
    body = ev.value(node).body()
    body.validate()
    return body.mass_properties().volume


class TestChute(unittest.TestCase):
    """Tour scene `chute` (demos/tour/src/bodies.rs): a C-channel
    revolved 270 degrees about the world y axis. One profile, one op,
    no booleans — the cleanest authorable scene in the tour."""

    def test_chute_matches_the_scene_oracle(self):
        poly = [
            (1.0, 0.0), (1.75, 0.0), (1.75, 0.625), (1.5625, 0.625),
            (1.5625, 0.1875), (1.1875, 0.1875), (1.1875, 0.625), (1.0, 0.625),
        ]
        doc = Doc()
        profile = doc.insert(Node.polygon([(x * m, y * m) for x, y in poly]))
        axis = doc.insert(Node.datum_axis((0 * m, 0 * m, 0 * m), (0.0, 1.0, 0.0)))
        chute = doc.insert(Node.revolve(profile, axis, 270 * deg))

        expected = (1287.0 / 2048.0) * math.pi
        self.assertAlmostEqual(volume_of(doc, chute), expected, delta=1e-12)


class TestDie(unittest.TestCase):
    """Tour scene `die` (demos/tour/src/bool_bodies.rs): a cube less 21
    pip pockets, 21 sequential undeclared subtracts. Every cutter
    overshoots its face and is strictly interior in the other two axes,
    so no operand pair shares a plane and the undeclared lane is legal."""

    def test_die_matches_the_exact_dyadic_oracle(self):
        h, inn, out = 0.125, (0.875, 1.5), (-1.5, -0.875)
        n, z, p = -0.5, 0.0, 0.5
        doc = Doc()
        body = slab(doc, (-1, 1), (-1, 1), (-1, 1))

        faces = [
            ([(z, z)], lambda a, b: ((a - h, a + h), (b - h, b + h), inn)),
            ([(n, n), (n, z), (n, p), (p, n), (p, z), (p, p)],
             lambda a, b: ((a - h, a + h), (b - h, b + h), out)),
            ([(n, n), (p, p)], lambda a, b: (inn, (a - h, a + h), (b - h, b + h))),
            ([(n, n), (n, p), (p, n), (p, p), (z, z)],
             lambda a, b: (out, (a - h, a + h), (b - h, b + h))),
            ([(n, n), (z, z), (p, p)], lambda a, b: ((a - h, a + h), inn, (b - h, b + h))),
            ([(n, n), (n, p), (p, n), (p, p)],
             lambda a, b: ((a - h, a + h), out, (b - h, b + h))),
        ]

        pips = 0
        for centres, box in faces:
            for a, b in centres:
                cutter = slab(doc, *box(a, b))
                body = doc.insert(Node.boolean(BooleanOp.Subtract, body, cutter))
                pips += 1

        self.assertEqual(pips, 21)
        expected = 8.0 - pips * 0.25 * 0.25 * 0.125
        self.assertEqual(expected, 7.8359375)
        self.assertAlmostEqual(volume_of(doc, body), expected, delta=1e-12)


class TestProjectbox(unittest.TestCase):
    """Tour scene `projectbox` (demos/tour/src/projectbox.rs): the
    longest boolean chain in the tour, 15 ops over 16 boxes. Its own
    design rule — no two operand planes coincide anywhere in the chain,
    every offset in 1/16 steps — is exactly what makes it authorable
    without a declaration door."""

    def test_projectbox_matches_the_exact_dyadic_oracle(self):
        doc = Doc()
        body = slab(doc, (0, 3), (0, 2), (0, 1.5))
        body = doc.insert(
            Node.boolean(BooleanOp.Subtract, body, slab(doc, (0.25, 2.75), (0.25, 1.75), (0.25, 2.0)))
        )
        for x in [(0.5, 0.875), (1.3125, 1.6875), (2.125, 2.5)]:
            for y in [(-0.25, 0.5), (1.5, 2.25)]:
                body = doc.insert(
                    Node.boolean(BooleanOp.Subtract, body, slab(doc, x, y, (0.5, 1.25)))
                )
        bx = [(0.4375, 0.8125), (2.1875, 2.5625)]
        by = [(0.4375, 0.8125), (1.1875, 1.5625)]
        for x in bx:
            for y in by:
                body = doc.insert(
                    Node.boolean(BooleanOp.Union, body, slab(doc, x, y, (0.1875, 0.875)))
                )
        for x in bx:
            for y in by:
                px = (x[0] + 0.09375, x[1] - 0.09375)
                py = (y[0] + 0.09375, y[1] - 0.09375)
                body = doc.insert(
                    Node.boolean(BooleanOp.Subtract, body, slab(doc, px, py, (0.5625, 1.0625)))
                )

        # The scene's own running oracle, term for term:
        #   9 - 2.5*1.5*1.25                     the cavity
        #   - 6 * 0.375*0.25*0.75                the vent slots
        #   + 4 * 0.375*0.375*0.625              the bosses
        #   - 4 * 0.1875*0.1875*0.3125           the pilot pockets
        expected = (
            9.0
            - 2.5 * 1.5 * 1.25
            - 6 * 0.375 * 0.25 * 0.75
            + 4 * 0.375 * 0.375 * 0.625
            - 4 * 0.1875 * 0.1875 * 0.3125
        )
        self.assertEqual(expected, 4.1982421875)
        self.assertAlmostEqual(volume_of(doc, body), expected, delta=1e-12)


class TestHeatsink(unittest.TestCase):
    """Tour scenes `heatsink5/7/9` — authorable only as YES*: the BODY
    is reproducible by hand-authoring each fin, but the scene's actual
    point (one recipe, a LinearPattern node, a structural-param count
    edit 5->7->9, memoized recompute) is unreachable. There is no
    pattern node and no parameter edit in the Python surface."""

    def build(self, fins):
        doc = Doc()
        body = slab(doc, (0, 3), (0, 1), (0, 0.25))
        for i in range(fins):
            dx = i * 0.3125
            fin = slab(doc, (0.25 + dx, 0.4375 + dx), (0.125, 0.875), (0.1875, 1.0))
            body = doc.insert(Node.boolean(BooleanOp.Union, body, fin))
        return volume_of(doc, body)

    def test_each_fin_count_matches_the_scene_oracle(self):
        for fins in (5, 7, 9):
            with self.subTest(fins=fins):
                self.assertAlmostEqual(
                    self.build(fins), 0.75 + fins * 0.10546875, delta=1e-12
                )


class TestNamedGapsAreStillGaps(unittest.TestCase):
    """The NO rows' gaps, asserted as absences.

    If one of these fails, a door has been BUILT — good news, and the
    signal to promote the corresponding rows in the audit from NO to
    YES."""

    def test_the_bound_vocabulary_is_exactly_this(self):
        self.assertEqual(
            sorted(n for n in dir(Node) if not n.startswith("_")),
            ["boolean", "datum_axis", "extrude", "polygon", "revolve"],
        )
        self.assertEqual(
            sorted(n for n in dir(DocEdit) if not n.startswith("_")),
            ["delete_node", "insert_node", "set_tolerance"],
        )

    def test_the_named_gaps_are_still_gaps(self):
        import pncad

        for door in [
            "tessellate", "Mesh", "write_stl",        # mesh + STL
            "select", "select_where", "Selector",     # selectors
            "StableName", "find_flush_candidates",    # names, detect/declare
            "ParamName", "DocParam",                  # named parameters
        ]:
            with self.subTest(door=door):
                self.assertFalse(hasattr(pncad, door), f"{door} is now bound")

        for node_kind in ["fillet", "loft", "sweep", "tube", "pattern",
                          "transform", "split", "circle"]:
            with self.subTest(node=node_kind):
                self.assertFalse(hasattr(Node, node_kind), f"Node.{node_kind} exists")


if __name__ == "__main__":
    unittest.main()
