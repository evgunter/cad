"""`Node.transform_by` and the `Placement` chain, from Python.

The Python mirror of `crates/editor-core/tests/edit_placement_type.rs`:
a transform holds a chain of rigid steps and literal frames, the
three-component `Node.transform` is one rigid step, and a literal is
held to the placement bar at the edit door.
"""

import math
import unittest

from pncad import (
    Doc,
    EditError,
    Expr,
    Frame,
    FrameError,
    Node,
    Placement,
    evaluate,
    m,
    rad,
)


def cube(doc):
    """The unit cube [0, 1]^3, in metres."""
    profile = doc.insert(
        Node.polygon(
            [
                (Expr.length_in(0.0, m), Expr.length_in(0.0, m)),
                (Expr.length_in(1.0, m), Expr.length_in(0.0, m)),
                (Expr.length_in(1.0, m), Expr.length_in(1.0, m)),
                (Expr.length_in(0.0, m), Expr.length_in(1.0, m)),
            ],
            plane=doc.sketch_frame(),
        )
    )
    return doc.insert(Node.extrude(profile, Expr.length_in(1.0, m)))


def lengths(x, y, z):
    return (Expr.length_in(x, m), Expr.length_in(y, m), Expr.length_in(z, m))


def z_axis():
    return (Expr.literal(0.0), Expr.literal(0.0), Expr.literal(1.0))


class TestPlacement(unittest.TestCase):
    def test_transform_is_one_rigid_step(self):
        """`Node.transform` and `Node.transform_by` over the same rigid
        step place the body identically."""
        masses = []
        for by_chain in (False, True):
            doc = Doc()
            body = cube(doc)
            if by_chain:
                node = Node.transform_by(
                    body,
                    Placement.rigid(lengths(2.0, 0.0, 0.0), z_axis(), Expr.angle_in(0.5, rad)),
                )
            else:
                node = Node.transform(
                    body, lengths(2.0, 0.0, 0.0), z_axis(), Expr.angle_in(0.5, rad)
                )
            placed = doc.insert(node)
            ev = evaluate(doc)
            self.assertTrue(ev.succeeded(placed))
            props = ev.value(placed).body().mass_properties()
            masses.append((props.volume, props.surface_area))
        self.assertEqual(masses[0], masses[1])

    def test_a_point_at_placement_places_the_body(self):
        doc = Doc()
        body = cube(doc)
        aimed = Placement.point_at((1 * m, 2 * m, 3 * m), (4 * m, -1 * m, 5 * m), (1.0, 0.0, 0.0))
        self.assertEqual(len(aimed), 1)
        self.assertEqual(
            aimed,
            Placement.literal(
                Frame.point_at((1 * m, 2 * m, 3 * m), (4 * m, -1 * m, 5 * m), (1.0, 0.0, 0.0))
            ),
        )
        placed = doc.insert(Node.transform_by(body, aimed))
        ev = evaluate(doc)
        self.assertTrue(ev.succeeded(placed))
        self.assertAlmostEqual(ev.value(placed).body().mass_properties().volume, 1.0, places=12)

    def test_a_chain_holds_its_steps_in_order(self):
        spin = Placement.rigid(lengths(0.0, 0.0, 0.0), z_axis(), Expr.angle_in(math.pi / 2, rad))
        start = Placement.path_start_frame((0 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0))
        chain = start.then(spin)
        self.assertEqual(len(chain), 2)
        self.assertNotEqual(chain, spin.then(start))
        doc = Doc()
        placed = doc.insert(Node.transform_by(cube(doc), chain))
        self.assertTrue(evaluate(doc).succeeded(placed))

    def test_equality_is_bit_exact(self):
        self.assertNotEqual(
            Placement.literal(Frame.translation((0 * m, 0 * m, 0 * m))),
            Placement.literal(Frame.translation((-0.0 * m, 0 * m, 0 * m))),
        )

    def test_a_mirrored_literal_is_refused_at_the_edit_door(self):
        doc = Doc()
        body = cube(doc)
        mirror = Frame.mirror_across_plane((0 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0))
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.transform_by(body, Placement.literal(mirror)))
        self.assertEqual(caught.exception.variant, "improper_placement")

    def test_a_degenerate_aim_refuses_as_the_frame_does(self):
        with self.assertRaises(FrameError):
            Placement.point_at((1 * m, 1 * m, 1 * m), (1 * m, 1 * m, 1 * m), (1.0, 0.0, 0.0))


if __name__ == "__main__":
    unittest.main()
