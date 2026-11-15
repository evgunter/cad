"""`PoseDef`: a pose defined at the seat that reads it (D10,
`docs/INTENT-STAGE3-SPEC.md` §2).

No pose is free and none is defined from nothing. Each row here reads
one off geometry, moves one by a length or an angle, or constructs one
from others, hands it to a split's tool, and reads what the split did:
a face reads as its plane with the OUTWARD normal, a curved face has no
plane, a frame through a point on its own axis has no roll to pick, a
finer pose is read through its role, and a seat holds its kind.
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
    RoleRead,
    SurfaceKind,
    circle,
    evaluate,
    deg,
    m,
)

from placing import own_frame
from measuring import pinned


def cube(doc):
    """A unit cube standing on the ground."""
    square = doc.insert(Node.polygon(
        [(Formula.length_in(x, m), Formula.length_in(y, m)) for x, y in ((0, 0), (1, 0), (1, 1), (0, 1))],
    ))
    return doc.insert(Node.extrude(square, Formula.length_in(1, m)))


def standoff(plane, by):
    """`plane` slid along its own normal by `by`."""
    return PoseDef.slide(plane, RoleRead(plane, "normal"), by)


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
                standoff(PoseDef.plane(body, cap), Formula.length_in(-0.25, m)),
            ))
            above, _ = evaluate(doc).value(cut).split()
            self.assertIsNotNone(above, f"z = {height}")
            self.assertAlmostEqual(pinned(above).volume, 0.25, delta=1e-12, msg=f"z = {height}")

    def test_a_curved_face_has_no_plane(self):
        doc = Doc()
        profile = doc.insert(Node.profile(circle((0 * m, 0 * m), 1 * m)))
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
            cut = doc.insert(Node.split(body, RoleRead(
                PoseDef.through(PoseDef.axis(body, edge), PoseDef.point(body, v)), "xy",
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

    def test_a_role_its_pose_does_not_offer_refuses_at_the_door(self):
        """A plane offers its normal, not a frame's xy plane."""
        doc = Doc()
        body = cube(doc)
        cap = face_at(evaluate(doc), body, 1.0)
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.split(body, RoleRead(PoseDef.plane(body, cap), "xy")))
        self.assertEqual(caught.exception.variant, "slot_var_kind")

    def test_a_slide_along_a_plane_refuses_at_the_door(self):
        """A slide is along a direction or an axis: a frame's xy plane
        is neither."""
        doc = Doc()
        body = cube(doc)
        frame = own_frame(doc, body)
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.split(body, PoseDef.slide(
                RoleRead(frame, "xy"), RoleRead(frame, "xy"), 0.5 * m,
            )))
        self.assertEqual(caught.exception.variant, "pose_shape")


class TestTheConstructors(unittest.TestCase):
    def test_a_flip_of_a_flip_is_the_pose_it_flips(self):
        """The edit door normalises the involution: a split by a cap's
        plane flipped twice cuts as the plane itself, and a point, which
        has no sense, refuses even flipped twice."""
        doc = Doc()
        body = cube(doc)
        ev = evaluate(doc)
        cap = face_at(ev, body, 1.0)
        once = standoff(PoseDef.plane(body, cap), Formula.length_in(-0.25, m))
        plain = doc.insert(Node.split(body, once))
        twice = doc.insert(Node.split(body, PoseDef.flip(PoseDef.flip(once))))
        halves = evaluate(doc)
        for cut in (plain, twice):
            above, _ = halves.value(cut).split()
            self.assertAlmostEqual(pinned(above).volume, 0.25, delta=1e-12)
        corner = ev.all_vertices(body)[0]
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.split(body, PoseDef.flip(PoseDef.flip(PoseDef.point(body, corner)))))
        self.assertEqual(caught.exception.variant, "slot_var_kind")

    def test_a_role_word_is_one_of_the_nine(self):
        doc = Doc()
        with self.assertRaises(ValueError):
            RoleRead(cube(doc), "line")

    def test_a_frames_plane_slid_along_its_axis_splits(self):
        """The cube's own frame's xy plane slid half a metre along its
        own z halves the cube."""
        doc = Doc()
        body = cube(doc)
        frame = own_frame(doc, body)
        cut = doc.insert(Node.split(body, PoseDef.slide(
            RoleRead(frame, "xy"), RoleRead(frame, "z"), 0.5 * m,
        )))
        above, below = evaluate(doc).value(cut).split()
        self.assertAlmostEqual(pinned(above).volume, 0.5, delta=1e-12)
        self.assertAlmostEqual(pinned(below).volume, 0.5, delta=1e-12)

    def test_a_plane_spun_about_a_line_in_it_tilts_the_cut(self):
        """The cube's mid-height plane spun 45° about the line through
        its centre along x stays through the centre, so half the cube
        lies on each side, whichever way it turns."""
        doc = Doc()
        body = cube(doc)
        frame = own_frame(doc, body)
        mid = PoseDef.slide(RoleRead(frame, "xy"), RoleRead(frame, "z"), 0.5 * m)
        line = PoseDef.slide(
            PoseDef.slide(RoleRead(frame, "x"), RoleRead(frame, "y"), 0.5 * m),
            RoleRead(frame, "z"),
            0.5 * m,
        )
        cut = doc.insert(Node.split(body, PoseDef.spin(mid, line, 45 * deg)))
        above, below = evaluate(doc).value(cut).split()
        for half in (above, below):
            self.assertAlmostEqual(pinned(half).volume, 0.5, delta=1e-9)

    def test_a_point_in_a_frame_is_its_origin_slid(self):
        """The plane through the cube's own frame's z axis and the point
        `(1, 0.5, 0)` in it cuts the cube's footprint along the line from
        the origin through `(1, 0.5)`, leaving a quarter on one side."""
        doc = Doc()
        body = cube(doc)
        frame = own_frame(doc, body)
        through = PoseDef.through(
            RoleRead(frame, "z"), PoseDef.point_in(frame, 1 * m, 0.5 * m, 0 * m),
        )
        cut = doc.insert(Node.split(body, RoleRead(through, "zx")))
        above, below = evaluate(doc).value(cut).split()
        volumes = sorted(pinned(half).volume for half in (above, below))
        self.assertAlmostEqual(volumes[0], 0.25, delta=1e-9)
        self.assertAlmostEqual(volumes[1], 0.75, delta=1e-9)

    def test_a_slide_length_must_be_a_length(self):
        doc = Doc()
        frame = own_frame(doc, cube(doc))
        with self.assertRaises((ValueError, TypeError, pncad.EditError)):
            PoseDef.slide(RoleRead(frame, "origin"), RoleRead(frame, "x"), Formula.literal(0.5))


if __name__ == "__main__":
    unittest.main()
