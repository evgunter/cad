"""The derived sketch frame and the two facts it is built from
(LIB-B-FACE-FRAME).

Three doors close this census family, and they are one story rather
than three: `Evaluation.face_carrier_kind` asks a named face what KIND
of surface carries it, `Pose.sense` carries the face's orientation
sense beside the chart axis the frame doors already answered with, and
`Node.datum_face_frame` turns a face NAME plus a spin angle into a
sketch frame — "sketch on this face", the thing a user reaches for
before anything else and could not spell in Python at all.

WHAT MAKES IT DERIVED, and what the rows below check. The node stores
the face's name and the spin, and nothing else: origin, +x and +y are
READ off the upstream body at every evaluation. So the plate's own
thickness decides where the frame lands (`TestTheFrameIsRead`), the
body is a DAG INPUT the document refuses to delete out from under it
(`test_the_body_is_a_dag_input`), and a face name that stops denoting
fails the frame typed instead of quietly keeping nine stale numbers
(`face_frame_resolve`). A transcribed frame could do none of those.

NOTHING HERE READS INSIDE A NAME. Every face name is a materializer's
opaque text handed straight back to the constructor, which is the
whole point: the alphabet does not change between selecting a face and
sketching on it.

TWO THINGS THIS FILE DELIBERATELY DOES NOT ASSERT, so their absence is
not read as coverage.

- **A parameter driving the frame's carrier.** "Raise the body and the
  sketch rides up with it" is the derived frame's headline, and the
  only mutation Python can aim at a continuous slot today is
  re-authoring: no node constructor takes an `Expr`, and
  `DocEdit.bind_count_param` is narrowed to structural counts on
  purpose. So `TestTheFrameIsRead` shows the dependence by building
  two plates rather than by moving one, and the DAG edge itself is
  shown by the delete refusal.
- **`face_frame_readback`.** The fourth authoring refusal needs a body
  whose stored geometry cannot be re-read, which is not a state any
  evaluated value is in — the same reason `test_readback.py` has no
  `dangling_entity` row. It is bound and tagged; a row appears when
  some door reaches it.
"""

import math
import unittest

from pncad import (
    Doc,
    DocEdit,
    EditError,
    EntityKind,
    EvaluationError,
    Expr,
    Node,
    NodeId,
    Pose,
    ReadbackError,
    SurfaceKind,
    TubeWindow,
    evaluate,
    m,
    rad,
)

SQUARE = [
    (Expr.length_in(0, m), Expr.length_in(0, m)),
    (Expr.length_in(1, m), Expr.length_in(0, m)),
    (Expr.length_in(1, m), Expr.length_in(1, m)),
    (Expr.length_in(0, m), Expr.length_in(1, m)),
]


def plate(doc, thickness):
    """A 1 m x 1 m plate `thickness` thick, standing on the ground."""
    square = doc.insert(Node.polygon(SQUARE, plane=doc.sketch_frame()))
    return doc.insert(Node.extrude(square, Expr.literal(thickness)))


def top_face(ev, node, height):
    """The plate's upper face, found by where its carrier SITS — read
    off the evaluation rather than transcribed as a role name."""
    found = [
        f
        for f in ev.all_faces(node)
        if abs(ev.face_frame(node, f).origin[2].meters - height) < 1e-12
    ]
    assert len(found) == 1, f"expected one face at z={height}, got {len(found)}"
    return found[0]


def ring(doc):
    """A solid ring torus about the world z axis — a CURVED carrier,
    which is what a non-planar refusal needs."""
    spine = doc.insert(Node.datum_axis((
        Expr.length_in(0, m),
        Expr.length_in(0, m),
        Expr.length_in(0, m),
    ), (
        Expr.literal(0.0),
        Expr.literal(0.0),
        Expr.literal(1.0),
    )))
    return doc.insert(
        Node.tube(spine, (Expr.literal(1.0), Expr.literal(0.0), Expr.literal(0.0)), Expr.length_in(1, m), TubeWindow.full(), Expr.length_in(0.3, m))
    )


class TestTheCarrierKindIsATagRead(unittest.TestCase):
    """`face_carrier_kind`: a face name in, its stored `SurfaceKind`
    out. A tag READ and never a verdict — "is this face planar" is a
    comparison the caller makes against the answer, with no tolerance
    anywhere in it."""

    def setUp(self):
        self.doc = Doc()
        self.plate = plate(self.doc, 0.2 * m)
        self.ev = evaluate(self.doc)
        self.top = top_face(self.ev, self.plate, 0.2)

    def test_a_plates_face_reads_plane_and_the_predicate_is_the_caller_s(self):
        kind = self.ev.face_carrier_kind(self.plate, self.top)
        self.assertIs(kind, SurfaceKind.Plane)
        # The comparison IS the planarity question; nothing in the door
        # made it.
        self.assertTrue(kind == SurfaceKind.Plane)

    def test_every_face_of_an_extrude_is_planar(self):
        faces = self.ev.all_faces(self.plate)
        kinds = {self.ev.face_carrier_kind(self.plate, f) for f in faces}
        self.assertEqual(len(faces), 6)
        self.assertEqual(kinds, {SurfaceKind.Plane})

    def test_a_curved_carrier_reads_its_own_tag(self):
        """The read answers where the frame door declines. A torus
        carrier has no sketch frame, and the tag says so BEFORE the
        frame is built — which is the recourse the refusal's prose
        names."""
        doc = Doc()
        torus = ring(doc)
        ev = evaluate(doc)
        for face in ev.all_faces(torus):
            self.assertIs(ev.face_carrier_kind(torus, face), SurfaceKind.Torus)

    def test_an_edge_name_is_a_kind_mismatch_with_both_kinds_as_values(self):
        edge = self.ev.all_edges(self.plate)[0]
        with self.assertRaises(ReadbackError) as caught:
            self.ev.face_carrier_kind(self.plate, edge)
        err = caught.exception
        self.assertEqual(err.variant, "wrong_kind")
        self.assertEqual(err.wanted, EntityKind.Face)
        self.assertEqual(err.found, EntityKind.Edge)

    def test_a_body_name_refuses_one_rung_earlier(self):
        """A body has no single carrier, and that is a different fact
        from "this door reads faces" — the same rung `face_frame`
        refuses at, reached through the same ladder."""
        body = self.ev.all_bodies(self.plate)[0]
        with self.assertRaises(ReadbackError) as caught:
            self.ev.face_carrier_kind(self.plate, body)
        self.assertEqual(caught.exception.variant, "whole_body")

    def test_a_stale_name_is_refused_not_matched_to_a_twin(self):
        second = plate(self.doc, 0.2 * m)
        ev = evaluate(self.doc)
        with self.assertRaises(ReadbackError) as caught:
            ev.face_carrier_kind(second, top_face(ev, self.plate, 0.2))
        self.assertEqual(caught.exception.variant, "no_such_name")

    def test_text_that_is_no_name_at_all_is_a_boundary_refusal(self):
        with self.assertRaises(ValueError):
            self.ev.face_carrier_kind(self.plate, "the top one")


class TestTheOrientationSense(unittest.TestCase):
    """`Pose.sense`: the second fact `axis` deliberately does not fold
    in. The axis stays the CHART's; the outward normal is `sense *
    axis`, formed by the reader, a sign SELECTED from a stored bool."""

    def test_every_face_of_a_solid_points_out_of_it(self):
        """The oracle the field exists for. A cube's centre is inside
        it, every face's carrier origin lies in that face's plane, so
        the outward normal must have a positive component along the
        vector from the centre to the plane — for all six, with the
        sign taken from `sense` rather than assumed."""
        doc = Doc()
        cube = plate(doc, 1 * m)
        ev = evaluate(doc)
        centre = (0.5, 0.5, 0.5)
        faces = ev.all_faces(cube)
        self.assertEqual(len(faces), 6)
        for face in faces:
            pose = ev.face_frame(cube, face)
            sign = 1.0 if pose.sense else -1.0
            outward = tuple(sign * c for c in pose.axis)
            away = tuple(
                o.meters - c for o, c in zip(pose.origin, centre, strict=True)
            )
            dot = sum(a * b for a, b in zip(away, outward, strict=True))
            self.assertGreater(dot, 0.4, f"{face} faces into the material")

    def test_a_wall_whose_material_is_outside_its_carrier_records_false(self):
        """`sense` is not decoration: a hollow tube's INNER wall has
        its material outside its own cylinder, so the outward normal
        is `-axis` there and the bool is the only thing that says so.
        Four faces, two of them inner."""
        doc = Doc()
        spine = doc.insert(Node.datum_axis((
            Expr.length_in(0, m),
            Expr.length_in(0, m),
            Expr.length_in(0, m),
        ), (
            Expr.literal(0.0),
            Expr.literal(0.0),
            Expr.literal(1.0),
        )))
        tube = doc.insert(
            Node.hollow_tube(
                spine, (Expr.literal(1.0), Expr.literal(0.0), Expr.literal(0.0)), Expr.length_in(1, m), TubeWindow.full(), Expr.length_in(0.3, m), Expr.length_in(0.1, m)
            )
        )
        ev = evaluate(doc)
        senses = sorted(ev.face_frame(tube, f).sense for f in ev.all_faces(tube))
        self.assertEqual(senses, [False, False, True, True])

    def test_an_edge_has_no_orientation_sense_and_answers_true(self):
        """`True` is the sign that leaves `axis` exactly as the chart
        stores it, which is what the field means for something that
        has no sense at all — stated, so a caller does not read it as
        a claim about the edge."""
        doc = Doc()
        cube = plate(doc, 1 * m)
        ev = evaluate(doc)
        for edge in ev.all_edges(cube):
            self.assertTrue(ev.edge_frame(cube, edge).sense)

    def test_the_repr_carries_it_beside_the_axis(self):
        doc = Doc()
        cube = plate(doc, 1 * m)
        ev = evaluate(doc)
        pose = ev.face_frame(cube, top_face(ev, cube, 1.0))
        self.assertIsInstance(pose, Pose)
        self.assertIn("sense=True", repr(pose))


class TestTheDerivedFrame(unittest.TestCase):
    """`Node.datum_face_frame`: the face's pose plus a spin, as a
    document node.

    Every number below is an oracle against the pose the READ door
    answers with, never a transcribed constant: the frame's origin is
    the pose's origin, its normal is `sense * axis`, and its +x is the
    carrier's `u_ref` turned by the spin about that normal."""

    def setUp(self):
        self.doc = Doc()
        self.plate = plate(self.doc, 0.2 * m)
        ev = evaluate(self.doc)
        self.top = top_face(ev, self.plate, 0.2)
        self.pose = ev.face_frame(self.plate, self.top)

    def frame_datum(self, spin):
        node = self.doc.insert(Node.datum_face_frame(self.plate, self.top, Expr.literal(spin)))
        return node, evaluate(self.doc).value(node).datum()

    def outward(self):
        sign = 1.0 if self.pose.sense else -1.0
        return tuple(sign * c for c in self.pose.axis)

    def assert_close(self, got, want, delta=1e-12):
        for a, b in zip(got, want, strict=True):
            self.assertAlmostEqual(a, b, delta=delta)

    def test_it_reads_back_as_an_ordinary_frame_datum(self):
        """A derived frame's VALUE is exactly an authored frame's —
        same kind, same projection — so every reader of a frame takes
        it by value and none has to learn where the numbers came
        from."""
        _, datum = self.frame_datum(0 * rad)
        self.assertEqual(datum.kind, "frame")
        self.assertIsNotNone(datum.axes)
        self.assertIsNone(datum.in_plane)

    def test_its_origin_is_the_faces_carrier_origin(self):
        _, datum = self.frame_datum(0 * rad)
        self.assert_close(
            tuple(c.meters for c in datum.origin),
            tuple(c.meters for c in self.pose.origin),
        )

    def test_its_normal_is_the_sense_times_the_axis(self):
        """DM1a, crossing: the sketch faces OUT of the plate, and the
        sign is selected from the stored bool rather than computed."""
        _, datum = self.frame_datum(0 * rad)
        self.assert_close(datum.direction, self.outward())

    def test_zero_spin_takes_the_carriers_u_reference_unturned(self):
        _, datum = self.frame_datum(0 * rad)
        u, v = datum.axes
        self.assert_close(u, self.pose.u_ref)
        # +y is the right-handed third leg, `n x u`.
        n = self.outward()
        self.assert_close(
            v,
            (
                n[1] * u[2] - n[2] * u[1],
                n[2] * u[0] - n[0] * u[2],
                n[0] * u[1] - n[1] * u[0],
            ),
        )

    def test_the_spin_turns_x_about_the_outward_normal(self):
        """The arithmetic, checked rather than eyeballed: sketch +x is
        `u_ref cos(spin) + (n x u_ref) sin(spin)`, right-handed about
        the OUTWARD normal — so a positive spin turns the same way on
        the underside of a plate as on the top."""
        theta = math.pi / 6
        _, datum = self.frame_datum(theta * rad)
        n, u_ref = self.outward(), self.pose.u_ref
        cross = (
            n[1] * u_ref[2] - n[2] * u_ref[1],
            n[2] * u_ref[0] - n[0] * u_ref[2],
            n[0] * u_ref[1] - n[1] * u_ref[0],
        )
        want = tuple(
            u * math.cos(theta) + c * math.sin(theta)
            for u, c in zip(u_ref, cross, strict=True)
        )
        self.assert_close(datum.axes[0], want)

    def test_the_spin_has_no_default(self):
        """Which way a sketch faces on a face is an authoring
        decision, and the door does not choose one — the argument is
        positional and required."""
        with self.assertRaises(TypeError):
            Node.datum_face_frame(self.plate, self.top)

    def test_a_sketch_on_the_frame_extrudes_out_of_the_plate(self):
        """The scene-scale row, authored the way a user would: select
        the top face, sketch on it, extrude. The boss starts at the
        plate's top and grows away from the material — which is what
        the outward normal buys."""
        frame = self.doc.insert(Node.datum_face_frame(self.plate, self.top, Expr.angle_in(0, rad)))
        # The sketch's coordinates are the FRAME's, and the frame's
        # origin is the carrier's distinguished point — the plate's
        # centre, not its corner — so the pad is written about zero.
        pad = self.doc.insert(
            Node.polygon(
                [
                    (Expr.length_in(-0.3, m), Expr.length_in(-0.3, m)),
                    (Expr.length_in(0.3, m), Expr.length_in(-0.3, m)),
                    (Expr.length_in(0.3, m), Expr.length_in(0.3, m)),
                    (Expr.length_in(-0.3, m), Expr.length_in(0.3, m)),
                ],
                plane=frame,
            )
        )
        boss = self.doc.insert(Node.extrude(pad, Expr.length_in(0.3, m)))
        ev = evaluate(self.doc)
        zs = [ev.vertex_position(boss, v)[2].meters for v in ev.all_vertices(boss)]
        self.assertAlmostEqual(min(zs), 0.2, delta=1e-12)
        self.assertAlmostEqual(max(zs), 0.5, delta=1e-12)
        # And its FOOTPRINT is the sketch's coordinates landed through
        # the frame: the origin is the plate's centre (0.5, 0.5), and
        # at zero spin sketch +x and +y are the carrier's `u_ref` and
        # `n x u_ref`, which here are world +x and +y — so a pad
        # written about zero arrives centred and unrotated.
        xs = [ev.vertex_position(boss, v)[0].meters for v in ev.all_vertices(boss)]
        ys = [ev.vertex_position(boss, v)[1].meters for v in ev.all_vertices(boss)]
        self.assertAlmostEqual(min(xs), 0.2, delta=1e-12)
        self.assertAlmostEqual(max(xs), 0.8, delta=1e-12)
        self.assertAlmostEqual(min(ys), 0.2, delta=1e-12)
        self.assertAlmostEqual(max(ys), 0.8, delta=1e-12)


class TestTheFrameIsRead(unittest.TestCase):
    """Not transcribed: the frame's numbers come off the upstream body
    at every evaluation, and the document knows it depends on it."""

    def test_the_frame_lands_wherever_the_face_it_names_is(self):
        """Two plates, the same six downstream calls, one number
        different: the frame follows the face and nothing in the
        recipe repeats the thickness."""
        for thickness in (0.2, 0.7):
            with self.subTest(thickness=thickness):
                doc = Doc()
                node = plate(doc, thickness * m)
                ev = evaluate(doc)
                face = top_face(ev, node, thickness)
                frame = doc.insert(Node.datum_face_frame(node, face, Expr.angle_in(0, rad)))
                origin = evaluate(doc).value(frame).datum().origin
                self.assertAlmostEqual(origin[2].meters, thickness, delta=1e-12)

    def test_the_body_is_a_dag_input(self):
        """`at` is an INPUT, exactly as `datum_axis_in_plane`'s plane
        is — so the document refuses to delete the body out from under
        the frame, naming both nodes."""
        doc = Doc()
        node = plate(doc, 0.2 * m)
        ev = evaluate(doc)
        frame = doc.insert(
            Node.datum_face_frame(node, top_face(ev, node, 0.2), Expr.angle_in(0, rad))
        )
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.delete_node(node))
        err = caught.exception
        self.assertEqual(err.variant, "delete_would_dangle")
        self.assertIsInstance(frame, NodeId)


class TestTheDerivedFrameRefuses(unittest.TestCase):
    """Typed at `evaluate`, never at the constructor: which face a
    frame names is recipe data, and a recipe that cannot be built is
    still a recipe. Three of the four arms are constructible from
    Python; the fourth is named in this module's docstring."""

    def setUp(self):
        self.doc = Doc()
        self.plate = plate(self.doc, 0.2 * m)
        self.ev = evaluate(self.doc)
        self.top = top_face(self.ev, self.plate, 0.2)

    def kind_of(self, node):
        with self.assertRaises(EvaluationError) as caught:
            evaluate(self.doc).value(node)
        return caught.exception

    def test_nothing_is_pre_checked_at_the_constructor(self):
        """An edge name builds a `Node` and inserts, and fails where
        every other selection failure does."""
        edge = self.ev.all_edges(self.plate)[0]
        node = Node.datum_face_frame(self.plate, edge, Expr.angle_in(0, rad))
        self.assertIsInstance(self.doc.insert(node), NodeId)

    def test_an_edge_name_refuses_face_frame_kind(self):
        edge = self.ev.all_edges(self.plate)[0]
        node = self.doc.insert(Node.datum_face_frame(self.plate, edge, Expr.angle_in(0, rad)))
        err = self.kind_of(node)
        self.assertEqual(err.kind, "face_frame_kind")
        self.assertEqual(err.node, node)

    def test_a_curved_carrier_refuses_face_frame_not_planar(self):
        """A sketch frame wants a plane. The refusal's prose names the
        carrier it found, and `Evaluation.face_carrier_kind` is the
        door that answers the same question BEFORE the frame is
        built."""
        torus = ring(self.doc)
        ev = evaluate(self.doc)
        face = ev.all_faces(torus)[0]
        node = self.doc.insert(Node.datum_face_frame(torus, face, Expr.angle_in(0, rad)))
        err = self.kind_of(node)
        self.assertEqual(err.kind, "face_frame_not_planar")
        self.assertIn("torus", str(err))
        self.assertIs(ev.face_carrier_kind(torus, face), SurfaceKind.Torus)

    def test_a_name_that_does_not_denote_here_refuses_face_frame_resolve(self):
        """The N5 failure mode, and the evidence that the frame is
        READ: a transcribed frame could not fail this way. The repair
        is a rebind, not an edit of nine numbers."""
        second = plate(self.doc, 0.2 * m)
        node = self.doc.insert(Node.datum_face_frame(second, self.top, Expr.angle_in(0, rad)))
        self.assertEqual(self.kind_of(node).kind, "face_frame_resolve")

    def test_a_failed_frame_poisons_the_sketch_above_it(self):
        """The fillet's failure mode, one node out: a sketch drawn on
        a frame that cannot be built is poisoned rather than placed on
        a fabricated plane, and `through` names the frame."""
        edge = self.ev.all_edges(self.plate)[0]
        frame = self.doc.insert(Node.datum_face_frame(self.plate, edge, Expr.angle_in(0, rad)))
        pad = self.doc.insert(Node.polygon(SQUARE, plane=frame))
        err = self.kind_of(pad)
        self.assertEqual(err.reason, "poisoned")
        self.assertEqual(err.through, frame)
        self.assertEqual(err.kind, "face_frame_kind")

    def test_text_that_is_no_name_at_all_is_a_boundary_refusal(self):
        with self.assertRaises(ValueError):
            Node.datum_face_frame(self.plate, "the top one", Expr.angle_in(0, rad))


if __name__ == "__main__":
    unittest.main()
