"""`Node.transform_by` and the `Placement` chain, from Python.

The Python mirror of `crates/editor-core/tests/edit_placement_type.rs`:
a transform holds a chain of rigid steps and literal frames, the
three-component `Node.transform` is one rigid step, `compose` reads in
`Frame.compose`'s order, and a literal is held to the placement bar at
the edit door.
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
    mm,
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


def about_z(t, angle):
    return Placement.rigid(translation=lengths(*t), axis=z_axis(), angle=Expr.angle_in(angle, rad))


def low_corner(placement):
    """The placed unit cube's least corner, in metres, to 9 places."""
    doc = Doc()
    placed = doc.insert(Node.transform_by(cube(doc), placement))
    ev = evaluate(doc)
    positions = ev.value(placed).body().tessellate(5 * mm).positions
    return tuple(round(min(p[i].meters for p in positions), 9) + 0.0 for i in range(3))


class TestPlacement(unittest.TestCase):
    def test_transform_is_one_rigid_step(self):
        """`Node.transform` and `Node.transform_by` over the same rigid
        step place the body identically."""
        masses = []
        for by_chain in (False, True):
            doc = Doc()
            body = cube(doc)
            if by_chain:
                node = Node.transform_by(body, about_z((2.0, 0.0, 0.0), 0.5))
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

    def test_rigid_takes_its_components_by_name(self):
        """Two vectors of one shape cannot trade places unread."""
        with self.assertRaises(TypeError):
            Placement.rigid(lengths(0.0, 0.0, 0.0), z_axis(), Expr.angle_in(0.0, rad))

    def test_compose_puts_the_inner_placement_first(self):
        """`a.compose(b)` is `a ∘ b`: `b` acts first. A quarter turn
        composed inside a shift puts the cube at x in [1, 2]; the other
        order turns the shifted cube to y in [2, 3]."""
        shift = about_z((2.0, 0.0, 0.0), 0.0)
        turn = about_z((0.0, 0.0, 0.0), math.pi / 2)
        self.assertEqual(low_corner(shift.compose(turn)), (1.0, 0.0, 0.0))
        self.assertEqual(low_corner(turn.compose(shift)), (-1.0, 2.0, 0.0))
        self.assertEqual(len(shift.compose(turn)), 2)

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

    def test_a_chain_of_literal_and_rigid_steps_evaluates(self):
        start = Placement.path_start_frame((0 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0))
        chain = start.compose(about_z((0.0, 0.0, 0.0), math.pi / 2))
        doc = Doc()
        placed = doc.insert(Node.transform_by(cube(doc), chain))
        self.assertTrue(evaluate(doc).succeeded(placed))

    def test_a_wrong_dimension_names_the_step_it_lands_at(self):
        """A step's components are checked where the step lands: an
        angle given a length refuses naming step 0's own slot through
        `Node.transform`, and a later step's slot through a chain."""
        doc = Doc()
        body = cube(doc)
        with self.assertRaises(EditError) as caught:
            Node.transform(body, lengths(0.0, 0.0, 0.0), z_axis(), Expr.length_in(1.0, m))
        self.assertEqual(caught.exception.variant, "slot_dimension_mismatch")
        self.assertEqual(caught.exception.slot, "rotation_angle")
        late = Placement.rigid(
            translation=lengths(0.0, 0.0, 0.0), axis=z_axis(), angle=Expr.length_in(1.0, m)
        )
        chain = Placement.literal(Frame.translation((0 * m, 0 * m, 1 * m))).compose(late)
        with self.assertRaises(EditError) as caught:
            Node.transform_by(body, chain)
        self.assertEqual(caught.exception.variant, "slot_dimension_mismatch")
        self.assertEqual(caught.exception.slot, "placement_step")
        self.assertIn("step 2 rotation angle", str(caught.exception))

    def test_equality_is_bit_exact(self):
        self.assertNotEqual(
            Placement.literal(Frame.translation((0 * m, 0 * m, 0 * m))),
            Placement.literal(Frame.translation((-0.0 * m, 0 * m, 0 * m))),
        )

    def test_a_mirrored_literal_is_refused_at_the_edit_door_naming_its_step(self):
        doc = Doc()
        body = cube(doc)
        mirror = Frame.mirror_across_plane((0 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0))
        chain = about_z((0.0, 0.0, 0.0), 0.0).compose(Placement.literal(mirror))
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.transform_by(body, chain))
        self.assertEqual(caught.exception.variant, "improper_placement")
        self.assertEqual(caught.exception.index, 1)
        self.assertIn("Recourse:", str(caught.exception))

    def test_a_degenerate_aim_refuses_as_the_frame_does(self):
        with self.assertRaises(FrameError):
            Placement.point_at((1 * m, 1 * m, 1 * m), (1 * m, 1 * m, 1 * m), (1.0, 0.0, 0.0))


if __name__ == "__main__":
    unittest.main()
