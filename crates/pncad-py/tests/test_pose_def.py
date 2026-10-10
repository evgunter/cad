"""`PoseDef`: a pose defined at the seat that reads it (D10,
`docs/INTENT-STAGE3-SPEC.md` §2).

No pose is free and none is defined from nothing. Each row here reads
one off geometry or constructs one from others, hands it to a split's
tool, and reads what the split did: a face reads as its plane with the
OUTWARD normal, a curved face has no plane, a frame through a point on
its own axis has no roll to pick, and a seat holds its kind.
"""

import unittest

import pncad
from pncad import (
    Doc,
    EditError,
    EvaluationError,
    Formula,
    Node,
    PoseDef,
    SurfaceKind,
    circle,
    evaluate,
    m,
)


def cube(doc):
    """A unit cube standing on the ground."""
    square = doc.insert(Node.polygon(
        [(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in ((0, 0), (1, 0), (1, 1), (0, 1))],
        plane=doc.sketch_frame(),
    ))
    return doc.insert(Node.extrude(square, Formula.length_in(1, m)))


def face_at(ev, node, height):
    """The cube's horizontal face at `height`, found by where it sits."""
    found = [
        f
        for f in ev.all_faces(node)
        if ev.face_carrier_kind(node, f) == SurfaceKind.Plane
        and abs(ev.face_frame(node, f).origin[2].meters - height) < 1e-12
    ]
    assert len(found) == 1, found
    return found[0]


class TestAFaceReadsAsItsPlane(unittest.TestCase):
    def test_each_cap_reads_with_its_outward_normal(self):
        """A standoff of −0.25 along each cap's normal lands INSIDE the
        cube on both caps, so the half along the normal is the quarter
        between the cut and the cap. An inward normal on either would
        put the cut outside and leave that half empty."""
        for height in (0.0, 1.0):
            doc = Doc()
            body = cube(doc)
            cap = face_at(evaluate(doc), body, height)
            cut = doc.insert(Node.split(
                body,
                PoseDef.standoff(PoseDef.plane(body, cap), Formula.length_in(-0.25, m)),
            ))
            above, _ = evaluate(doc).value(cut).split()
            self.assertIsNotNone(above, f"z = {height}")
            self.assertAlmostEqual(above.mass_properties().volume, 0.25, delta=1e-12, msg=f"z = {height}")

    def test_a_curved_face_has_no_plane(self):
        doc = Doc()
        profile = doc.insert(Node.profile(circle((0 * m, 0 * m), 1 * m), plane=doc.sketch_frame()))
        rod = doc.insert(Node.extrude(profile, Formula.length_in(2, m)))
        ev = evaluate(doc)
        wall = [f for f in ev.all_faces(rod) if ev.face_carrier_kind(rod, f) == SurfaceKind.Cylinder]
        cut = doc.insert(Node.split(rod, PoseDef.plane(rod, wall[0])))
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(cut)
        self.assertEqual(caught.exception.kind, "pose_read")
        self.assertEqual(caught.exception.inner_kind, "not_planar")


class TestAConstructionRefusesItsDegenerateCase(unittest.TestCase):
    def test_a_frame_through_a_point_on_its_axis_refuses(self):
        """An edge's line and one of its own ends leave no direction
        towards the point, so no roll: `pose_degenerate`, `through`."""
        doc = Doc()
        body = cube(doc)
        ev = evaluate(doc)
        edge = ev.all_edges(body)[0]
        refusals = []
        for v in ev.all_vertices(body):
            cut = doc.insert(Node.split(body, PoseDef.project(
                PoseDef.through(PoseDef.axis(body, edge), PoseDef.point(body, v)), "plane",
            )))
            try:
                evaluate(doc).value(cut)
            except EvaluationError as err:
                refusals.append((err.kind, err.inner_kind))
        # Two of a box's eight corners lie on any one of its edges.
        self.assertEqual(refusals.count(("pose_degenerate", "through")), 2, refusals)


class TestASeatHoldsItsKind(unittest.TestCase):
    def test_a_split_refuses_an_axis_tool_at_the_door(self):
        doc = Doc()
        body = cube(doc)
        edge = evaluate(doc).all_edges(body)[0]
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.split(body, PoseDef.axis(body, edge)))
        self.assertEqual(caught.exception.variant, "slot_var_kind")

    def test_a_projection_to_a_kind_it_does_not_reach_refuses_at_the_door(self):
        doc = Doc()
        body = cube(doc)
        cap = face_at(evaluate(doc), body, 1.0)
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.split(body, PoseDef.project(PoseDef.plane(body, cap), "axis")))
        self.assertEqual(caught.exception.variant, "pose_shape")


class TestTheConstructors(unittest.TestCase):
    def test_a_flip_of_a_flip_is_the_pose_it_flips(self):
        doc = Doc()
        body = cube(doc)
        cap = face_at(evaluate(doc), body, 1.0)
        plane = PoseDef.plane(body, cap)
        self.assertEqual(repr(PoseDef.flip(PoseDef.flip(plane))), repr(plane))

    def test_a_kind_word_is_one_of_the_five(self):
        doc = Doc()
        body = cube(doc)
        cap = face_at(evaluate(doc), body, 1.0)
        with self.assertRaises(ValueError):
            PoseDef.project(PoseDef.plane(body, cap), "line")

    def test_in_frame_takes_one_of_the_five_keyword_shapes(self):
        doc = Doc()
        frame = doc.sketch_frame()
        with self.assertRaises(ValueError):
            PoseDef.in_frame(frame, normal=(0.0, 0.0, 1.0))
        with self.assertRaises(ValueError):
            PoseDef.in_frame(frame, origin=(0 * m, 0 * m, 0 * m), u=(1.0, 0.0, 0.0))

    def test_a_plane_in_a_frame_splits(self):
        doc = Doc()
        body = cube(doc)
        cut = doc.insert(Node.split(body, PoseDef.in_frame(
            doc.sketch_frame(), origin=(0 * m, 0 * m, 0.5 * m), normal=(0.0, 0.0, 1.0),
        )))
        above, below = evaluate(doc).value(cut).split()
        self.assertAlmostEqual(above.mass_properties().volume, 0.5, delta=1e-12)
        self.assertAlmostEqual(below.mass_properties().volume, 0.5, delta=1e-12)

    def test_a_position_coordinate_must_be_a_length(self):
        doc = Doc()
        with self.assertRaises((ValueError, TypeError, pncad.EditError)):
            PoseDef.in_frame(doc.sketch_frame(), origin=(Formula.literal(0.0), 0 * m, 0 * m))


if __name__ == "__main__":
    unittest.main()
