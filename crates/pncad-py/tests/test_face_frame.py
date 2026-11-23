"""The face a sketch is drawn on, and the two facts it is read from
(LIB-B-FACE-FRAME).

Three doors close this census family, and they are one story rather
than three: `Evaluation.face_carrier_kind` asks a named face what KIND
of surface carries it, `Pose.sense` carries the face's orientation
sense beside the chart axis the frame doors already answered with, and
`Doc.draw_on_face` turns a face NAME into a profile drawn on that
face's chart — "sketch on this face", the thing a user reaches for
before anything else — which an extrude by a signed depth and a
combine make a boss or a pocket.

WHAT MAKES IT READ, and what the rows below check. The profile's chart
stores the face's name, its orientation, spin and slides, and nothing
else: where the face lies is READ off the upstream body at every
evaluation. So the plate's own thickness decides where the sketch lands
(`test_the_sketch_lands_wherever_the_face_it_names_is`), the body is a
read the document reports when it is deleted out from under the sketch
(`test_the_body_is_a_read`), and a face name that stops denoting fails
the chart typed instead of quietly keeping nine stale numbers. A
transcribed frame could do none of those.

NOTHING HERE READS INSIDE A NAME. Every face name is a materializer's
opaque text handed straight back to the door, which is the whole
point: the alphabet does not change between selecting a face and
sketching on it.

ONE THING THIS FILE DELIBERATELY DOES NOT ASSERT, so its absence is not
read as coverage: **a parameter driving the face.** "Raise the body and
the sketch rides up with it" is the headline, and `TestSketchOnAFace`
shows the dependence by building two plates rather than by moving one.
"""

import math
import shutil
import tempfile
import unittest

from pncad import (
    Coord,
    Doc,
    DocEdit,
    EditError,
    EntityKind,
    EvaluationError,
    Formula,
    Mate,
    MateValue,
    Node,
    Open,
    Orientation,
    Placed,
    Pose,
    PoseDef,
    ReadbackError,
    Start,
    SurfaceKind,
    TubeWindow,
    Workspace,
    evaluate,
    m,
    rad,
    solve_document,
)
import bench_scene
from measuring import pinned

SQUARE = [
    (Formula.length_in(0, m), Formula.length_in(0, m)),
    (Formula.length_in(1, m), Formula.length_in(0, m)),
    (Formula.length_in(1, m), Formula.length_in(1, m)),
    (Formula.length_in(0, m), Formula.length_in(1, m)),
]


def plate(doc, thickness):
    """A 1 m x 1 m plate `thickness` thick, standing on the ground."""
    square = doc.insert(Node.polygon(SQUARE))
    return doc.insert(Node.extrude(square, Formula.literal(thickness)))


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
    """A solid ring torus about its own z axis — a CURVED carrier,
    which is what a non-planar refusal needs."""
    return doc.insert(
        Node.tube(Formula.length_in(1, m), TubeWindow.full(), Formula.length_in(0.3, m))
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
        tube = doc.insert(
            Node.hollow_tube(
                Formula.length_in(1, m), TubeWindow.full(), Formula.length_in(0.3, m), Formula.length_in(0.1, m)
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


def loop(points):
    """The closed polygon through `points`, in metres."""
    path = Open.at((points[0][0] * m, points[0][1] * m))
    for x, y in points[1:]:
        path = path.line_to((x * m, y * m))
    return path.line_to(Start)


PAD = loop([(-0.3, -0.3), (0.3, -0.3), (0.3, 0.3), (-0.3, 0.3)])


def outward(ev, node, face):
    """`node`'s face `face`'s outward normal: its sense times its axis."""
    frame = ev.face_frame(node, face)
    sign = 1.0 if frame.sense else -1.0
    return tuple(sign * a for a in frame.axis)


def positions(ev, node):
    """Every vertex of `node`, in metres."""
    return [tuple(c.meters for c in ev.vertex_position(node, v)) for v in ev.all_vertices(node)]


def volume(ev, node):
    """`node`'s body's volume, pinned."""
    return pinned(ev.value(node).body()).volume


class TestSketchOnAFace(unittest.TestCase):
    """`Doc.draw_on_face`: "sketch on this face" — a profile drawn on
    the named face's chart, in the face's body's space, then an ordinary
    extrude by a signed depth and a combine with the body.

    Every number below is an oracle against where the face SITS, read
    off the evaluation: the boss starts on the face and grows along its
    outward normal, its origin over the face body's own, turned by the
    spin about that normal and moved by the slides."""

    def setUp(self):
        self.doc = Doc()
        self.plate = plate(self.doc, 0.2 * m)
        ev = evaluate(self.doc)
        self.top = top_face(ev, self.plate, 0.2)

    def test_a_boss_on_the_face_extrudes_out_of_the_plate(self):
        """The scene-scale row, authored the way a user would: select
        the top face, draw on it, extrude, unite. The boss starts at the
        plate's top and grows away from the material — which is what
        the outward normal buys — and its footprint is the sketch's
        coordinates over the plate's own origin, unturned at zero
        spin."""
        profile = self.doc.draw_on_face(self.plate, self.top, PAD)
        boss = self.doc.insert(Node.extrude(profile, 0.3 * m))
        union = self.doc.insert(Node.union([self.plate, boss]))
        ev = evaluate(self.doc)
        at = positions(ev, boss)
        for axis, (lo, hi) in enumerate(((-0.3, 0.3), (-0.3, 0.3), (0.2, 0.5))):
            with self.subTest(axis=axis):
                self.assertAlmostEqual(min(p[axis] for p in at), lo, delta=1e-12)
                self.assertAlmostEqual(max(p[axis] for p in at), hi, delta=1e-12)
        # The union holds the plate and the boss: 1 x 1 x 0.2 and
        # 0.6 x 0.6 x 0.3.
        self.assertAlmostEqual(volume(ev, union), 0.2 + 0.108, delta=1e-12)
        self.assertEqual(self.doc.space(boss), self.doc.space(self.plate), "nothing is placed")

    def test_a_pocket_is_the_same_sketch_extruded_back_and_subtracted(self):
        """A `-` depth runs into the material, from the face down, and
        the slides move the pad across the face: slid by `(0.5, 0.5)`
        the pad sits inside the top, so the pocket takes
        `0.6 x 0.6 x 0.1` out of the plate."""
        profile = self.doc.draw_on_face(self.plate, self.top, PAD, slides=(0.5 * m, 0.5 * m))
        tool = self.doc.insert(Node.extrude(profile, -0.1 * m))
        pocket = self.doc.insert(Node.subtract(self.plate, tool))
        ev = evaluate(self.doc)
        at = positions(ev, tool)
        for axis, (lo, hi) in enumerate(((0.2, 0.8), (0.2, 0.8), (0.1, 0.2))):
            with self.subTest(axis=axis):
                self.assertAlmostEqual(min(p[axis] for p in at), lo, delta=1e-12)
                self.assertAlmostEqual(max(p[axis] for p in at), hi, delta=1e-12)
        self.assertAlmostEqual(volume(ev, pocket), 0.2 - 0.036, delta=1e-12)

    def test_a_boss_and_a_pocket_on_the_bottom_face(self):
        """On the bottom face the chart's `z` is the space's `-z`: a `+`
        depth stands below the plate and unites, a `-` depth runs up
        into it and subtracts. Slid `(0.5, 0.5)` the pad sits inside the
        bottom, so each moves `0.6 x 0.6 x 0.1` of material."""
        ev = evaluate(self.doc)
        (bottom,) = [f for f in ev.all_faces(self.plate) if outward(ev, self.plate, f) == (0.0, 0.0, -1.0)]
        profile = self.doc.draw_on_face(self.plate, bottom, PAD, slides=(0.5 * m, 0.5 * m))
        boss = self.doc.insert(Node.extrude(profile, 0.1 * m))
        tool = self.doc.insert(Node.extrude(profile, -0.1 * m))
        union = self.doc.insert(Node.union([self.plate, boss]))
        pocket = self.doc.insert(Node.subtract(self.plate, tool))
        ev = evaluate(self.doc)
        for node, (lo, hi) in ((boss, (-0.1, 0.0)), (tool, (0.0, 0.1))):
            with self.subTest(node=node):
                zs = [p[2] for p in positions(ev, node)]
                self.assertAlmostEqual(min(zs), lo, delta=1e-12)
                self.assertAlmostEqual(max(zs), hi, delta=1e-12)
        self.assertAlmostEqual(volume(ev, union), 0.2 + 0.036, delta=1e-12)
        self.assertAlmostEqual(volume(ev, pocket), 0.2 - 0.036, delta=1e-12)

    def test_the_door_inserts_one_labelled_profile(self):
        """The door inserts the profile and nothing else, as one action
        with its label."""
        before = self.doc.node_count
        profile = self.doc.draw_on_face(self.plate, self.top, PAD, label="boss")
        self.assertEqual(self.doc.node_count, before + 1)
        self.assertEqual(self.doc.node_kind(profile), "profile")
        self.assertEqual(self.doc.label(profile), "boss")

    def test_the_spin_turns_the_sketch_about_the_outward_normal(self):
        """The arithmetic, checked rather than eyeballed: a corner the
        sketch draws at `(0.4, 0)` lands at `(0.4 cos spin, 0.4 sin
        spin)`, right-handed about the face's outward normal."""
        theta = math.pi / 6
        arm = loop([(0, 0), (0.4, 0), (0.4, 0.1), (0, 0.1)])
        profile = self.doc.draw_on_face(self.plate, self.top, arm, spin=Formula.angle_in(theta, rad))
        boss = self.doc.insert(Node.extrude(profile, 0.3 * m))
        want = (0.4 * math.cos(theta), 0.4 * math.sin(theta), 0.2)
        near = [
            p for p in positions(evaluate(self.doc), boss) if max(abs(a - b) for a, b in zip(p, want, strict=True)) < 1e-12
        ]
        self.assertEqual(len(near), 1, "the sketch's (0.4, 0) corner, turned by the spin")

    def test_the_sketch_lands_wherever_the_face_it_names_is(self):
        """Two plates, the same downstream call, one number different:
        the sketch follows the face and nothing in the recipe repeats
        the thickness."""
        for thickness in (0.2, 0.7):
            with self.subTest(thickness=thickness):
                doc = Doc()
                node = plate(doc, thickness * m)
                face = top_face(evaluate(doc), node, thickness)
                boss = doc.insert(Node.extrude(doc.draw_on_face(node, face, PAD), 0.3 * m))
                zs = [p[2] for p in positions(evaluate(doc), boss)]
                self.assertAlmostEqual(min(zs), thickness, delta=1e-12)

    def test_the_body_is_a_read(self):
        """The face is read off the body, so deleting the body out from
        under the sketch is accepted and reported, naming the profile
        whose chart reads it, which refuses until re-pointed."""
        profile = self.doc.draw_on_face(self.plate, self.top, PAD)
        self.doc.apply(DocEdit.delete_node(self.plate))
        stranded = [m.node for m in self.doc.last_maintenance if m.variant == "stranded_read"]
        self.assertIn(profile, stranded)
        with self.assertRaises(EvaluationError):
            evaluate(self.doc).value(profile)


# A pad with no symmetry: a turn or a mirror of it is another set of
# corners, so where its corners land says how the boss is turned.
SKEW = loop([(0.1, 0.05), (0.4, 0.05), (0.4, 0.25), (0.1, 0.15)])

AXES = {"+x": (1.0, 0.0, 0.0), "-x": (-1.0, 0.0, 0.0), "+y": (0.0, 1.0, 0.0),
        "-y": (0.0, -1.0, 0.0), "+z": (0.0, 0.0, 1.0), "-z": (0.0, 0.0, -1.0)}


def nearest(normal):
    """The orientation nearest a face whose outward normal is `normal`:
    `z` on the normal, and no orientation putting it there has a greater
    trace, a tie keeping the space's `+x`."""
    (z,) = [w for w, v in AXES.items() if v == normal]
    rivals = [Orientation(x, z) for x in AXES if x[1] != z[1]]

    def trace(o):
        return AXES[o.x][0] + AXES[o.y][1] + AXES[o.z][2]

    best = max(trace(o) for o in rivals)
    tied = [o for o in rivals if trace(o) == best]
    if len(tied) == 1:
        return tied[0]
    (kept,) = [o for o in tied if o.x == "+x"]
    return kept


class TestSketchOnEveryFace(unittest.TestCase):
    """A sketch on each of the plate's six faces, the orientation left
    to the door: the one nearest the face, the chart's `z` along the
    face's outward normal. So the least turn is the identity, the
    chart's axes are the orientation's, its origin stands on the face
    over the plate's own, and a `+0.3` extrude grows along the outward
    normal, whichever face it is, the bottom included. An orientation
    written backwards refuses, naming the one to write."""

    def faces(self):
        doc = Doc()
        node = plate(doc, 0.2 * m)
        ev = evaluate(doc)
        return doc, node, {outward(ev, node, f): f for f in ev.all_faces(node)}

    def test_the_boss_stands_on_every_face_turned_by_the_nearest_orientation(self):
        """Breaks if the door draws anywhere but on the face, or stands
        the chart's axes anywhere but where the nearest orientation puts
        them: the boss's corners are the skewed pad's, turned by that
        orientation and set on the face over the plate's origin. Off the
        bottom face the identity, carried by the least turn, draws this
        same chart, so only the bottom face tells the nearest
        orientation from the identity: there the identity is backwards
        and refuses."""
        _, _, faces = self.faces()
        self.assertEqual(len(faces), 6, "a plate has six faces, one per outward normal")
        for normal in faces:
            with self.subTest(normal=normal):
                doc, node, by_normal = self.faces()
                profile = doc.draw_on_face(node, by_normal[normal], SKEW)
                boss = doc.insert(Node.extrude(profile, 0.3 * m))
                orient = nearest(normal)
                cols = [AXES[orient.x], AXES[orient.y], AXES[orient.z]]
                k = max(range(3), key=lambda i: abs(normal[i]))
                face_at = (0.2 if k == 2 else 1.0) if normal[k] > 0 else 0.0
                at = tuple(face_at if i == k else 0.0 for i in range(3))
                corners = [(0.1, 0.05), (0.4, 0.05), (0.4, 0.25), (0.1, 0.15)]
                want = sorted(
                    tuple(at[i] + u * cols[0][i] + v * cols[1][i] + h * cols[2][i] for i in range(3))
                    for u, v in corners
                    for h in (0.0, 0.3)
                )
                got = sorted(positions(evaluate(doc), boss))
                self.assertEqual(len(got), len(want))
                for g, w in zip(got, want, strict=True):
                    for a, b in zip(g, w, strict=True):
                        self.assertAlmostEqual(a, b, delta=1e-12)

    def test_each_slide_moves_the_sketch_along_its_own_axis(self):
        """The slides run along the space's axes other than the one the
        face looks along, in `x`, `y`, `z` order: on the top face the
        first moves the boss along `x` and the second along `y`, on the
        `+x` face the first along `y` and the second along `z`. Breaks if
        the two are swapped, or one value stands for both."""

        def lows(normal, slides):
            doc, node, by_normal = self.faces()
            profile = doc.draw_on_face(node, by_normal[normal], SKEW, slides=slides)
            boss = doc.insert(Node.extrude(profile, 0.3 * m))
            at = positions(evaluate(doc), boss)
            return [min(p[i] for p in at) for i in range(3)]

        cases = {(0.0, 0.0, 1.0): (0, 1), (1.0, 0.0, 0.0): (1, 2)}
        for normal, axes in cases.items():
            unslid = lows(normal, None)
            for which, slides in enumerate(((0.5 * m, 0 * m), (0 * m, 0.5 * m))):
                with self.subTest(normal=normal, slide=which):
                    moved = [b - a for a, b in zip(unslid, lows(normal, slides), strict=True)]
                    want = [0.5 if i == axes[which] else 0.0 for i in range(3)]
                    for got, w in zip(moved, want, strict=True):
                        self.assertAlmostEqual(got, w, delta=1e-12)

    def test_an_orientation_written_backwards_refuses_naming_the_one_to_write(self):
        """Breaks if the identity written on the bottom face is answered
        by a half turn about an arbitrary axis, or the refusal names an
        orientation that does not stand the boss under the plate."""
        doc, node, faces = self.faces()
        profile = doc.draw_on_face(node, faces[(0.0, 0.0, -1.0)], SKEW, orient=Orientation.identity())
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(profile)
        refused = caught.exception
        self.assertEqual((refused.kind, refused.inner_kind), ("pose_degenerate", "face_chart_backwards"))
        write = nearest((0.0, 0.0, -1.0))
        self.assertIn(f"Recourse: write the orientation x: {write.x}, z: {write.z}", str(refused))
        doc, node, faces = self.faces()
        profile = doc.draw_on_face(node, faces[(0.0, 0.0, -1.0)], SKEW, orient=write)
        boss = doc.insert(Node.extrude(profile, 0.3 * m))
        zs = [p[2] for p in positions(evaluate(doc), boss)]
        self.assertAlmostEqual(max(zs), 0.0, delta=1e-12)
        self.assertAlmostEqual(min(zs), -0.3, delta=1e-12)


class TestSketchOnAnInstancesFace(unittest.TestCase):
    """The orientation `orient=None` writes is read off the face as the
    document evaluates under `resolver=`: an instance's copy has faces
    only where its part resolves."""

    def setUp(self):
        self.dir = tempfile.mkdtemp()
        self.addCleanup(shutil.rmtree, self.dir, True)
        self.ws = Workspace(self.dir)
        (_, ref), _ = bench_scene.parts(self.ws)
        self.doc = Doc()
        self.post = self.doc.insert(Node.instantiate_part(ref, self.doc.world))
        self.bottom = bench_scene.face_facing(self.ws, self.doc, self.post, (0.0, 0.0, -1.0))

    def drawn(self, resolver):
        profile = self.doc.draw_on_face(self.post, self.bottom, PAD, resolver=resolver)
        boss = self.doc.insert(Node.extrude(profile, 0.1 * m))
        return profile, boss

    def test_under_the_workspace_the_boss_stands_below_the_post(self):
        _, boss = self.drawn(self.ws)
        zs = [p[2] for p in positions(evaluate(self.doc, resolver=self.ws), boss)]
        self.assertAlmostEqual(min(zs), -0.1, delta=1e-12)
        self.assertAlmostEqual(max(zs), 0.0, delta=1e-12)

    def test_without_it_the_face_reads_no_orientation_and_the_identity_is_backwards(self):
        """Breaks if `resolver=` stops reaching the door's evaluation:
        the nearest orientation reads off nothing, and the identity it
        falls to is backwards on the bottom face."""
        profile, _ = self.drawn(None)
        with self.assertRaises(EvaluationError) as caught:
            evaluate(self.doc, resolver=self.ws).value(profile)
        refused = caught.exception
        self.assertEqual((refused.kind, refused.inner_kind), ("pose_degenerate", "face_chart_backwards"))


class TestARestSpinsOneWay(unittest.TestCase):
    """A unit plate rests on a floor plate at slides `(3, 4)` and a
    spin of 0.5, its contact spelled twice: the flip on the floor's
    plane (as the viewer's mate tool and `Mate`'s docs write a rest), and
    the flip on the copy's bottom (as a body placed on a face writes it,
    DM1). The
    one spin turns about the target's own direction whatever side
    carries the flip, so both stand the copy at one place, turned
    counter-clockwise seen from `+z`."""

    SPIN = 0.5

    def rested(self, copy_side):
        doc = Doc()
        floor = doc.place(plate(doc, 0.2 * m), doc.world)
        body = plate(doc, 0.2 * m)
        copy = doc.place(body, doc.world)
        ev = evaluate(doc)

        def facing(node, z):
            (face,) = [f for f in ev.all_faces(node) if outward(ev, node, f) == (0.0, 0.0, z)]
            return PoseDef.plane(node, face)

        bottom, top = facing(body, -1.0), facing(floor, 1.0)
        mate = Mate(PoseDef.flip(bottom), top) if copy_side else Mate(bottom, PoseDef.flip(top))
        doc.set_constraints(
            copy,
            [
                mate,
                MateValue(Coord.slide("x"), 3.0 * m),
                MateValue(Coord.slide("y"), 4.0 * m),
                MateValue(Coord.spin(), self.SPIN * rad),
            ],
        )
        return sorted(positions(evaluate(doc), copy))

    def test_both_spellings_stand_the_copy_at_one_place(self):
        """Breaks if the spin turns about the mated direction with the
        flip included: the floor-side flip points it down, turning that
        copy the other way."""
        floor_side, copy_side = self.rested(False), self.rested(True)
        self.assertEqual(len(floor_side), len(copy_side))
        for a, b in zip(floor_side, copy_side, strict=True):
            for x, y in zip(a, b, strict=True):
                self.assertAlmostEqual(x, y, delta=1e-12)

    def test_a_positive_spin_turns_counter_clockwise_seen_from_above(self):
        """Breaks if a positive spin on a floor rest turns clockwise seen
        from `+z`: the copy's own corner `(1, 0, 0)` lands at
        `(3 + cos 0.5, 4 + sin 0.5)` on the floor's top, 0.2 up."""
        want = (3.0 + math.cos(self.SPIN), 4.0 + math.sin(self.SPIN), 0.2)
        for copy_side in (False, True):
            with self.subTest(copy_side=copy_side):
                corners = self.rested(copy_side)
                near = min(corners, key=lambda p: math.dist(p, want))
                for got, w in zip(near, want, strict=True):
                    self.assertAlmostEqual(got, w, delta=1e-12)


class TestARestOnAWall(unittest.TestCase):
    """A unit plate's `-x` face rests on a wall's `+x` face under the
    identity orientation, so the plane fixes the slide along `x` and
    leaves `y` and `z`. `Doc.constrain`, the gesture, reads the copy's
    geometry and writes those two; the raw `DocEdit.add_constraint`
    reads none and writes `x` and `y`, which the solve refuses, naming
    the pair to write, and that pair places."""

    def scene(self):
        doc = Doc()
        wall = doc.place(plate(doc, 0.2 * m), doc.world)
        body = plate(doc, 0.2 * m)
        copy = doc.place(body, doc.world)
        ev = evaluate(doc)

        def facing(node, normal):
            (face,) = [f for f in ev.all_faces(node) if outward(ev, node, f) == normal]
            return PoseDef.plane(node, face)

        mate = Mate(facing(body, (-1.0, 0.0, 0.0)), PoseDef.flip(facing(wall, (1.0, 0.0, 0.0))))
        return doc, copy, mate

    def test_the_gesture_rests_the_plate_flush_on_the_wall(self):
        """Breaks if `Doc.constrain` fills the slides in axis order, the
        `x` the wall fixes among them."""
        doc, copy, mate = self.scene()
        doc.constrain(copy, mate)
        self.assertIsNone(solve_document(doc).fault(copy))
        xs = [p[0] for p in positions(evaluate(doc), copy)]
        self.assertAlmostEqual(min(xs), 1.0, delta=1e-12)

    def test_the_raw_edit_refuses_naming_the_pair_that_places(self):
        """Breaks if the raw edit's refusal names no pair, or a pair that
        does not place."""
        doc, copy, mate = self.scene()
        doc.apply(DocEdit.add_constraint(copy, mate))
        fault = solve_document(doc).fault(copy)
        self.assertIsNotNone(fault)
        self.assertEqual(fault.why, "slides_do_not_span")
        self.assertEqual(fault.slides, ["y", "z"])
        self.assertIn("write the slides along y and z", str(fault))
        doc, copy, mate = self.scene()
        pair = [MateValue(Coord.slide("y"), 0), MateValue(Coord.slide("z"), 0)]
        doc.set_constraints(copy, Placed.of(doc, [mate], pair))
        self.assertIsNone(solve_document(doc).fault(copy))


class TestSketchOnAFaceRefuses(unittest.TestCase):
    """What the face a sketch names can be wrong about: its kind at the
    edit door, its denotation and its carrier at `evaluate`."""

    def setUp(self):
        self.doc = Doc()
        self.plate = plate(self.doc, 0.2 * m)
        self.ev = evaluate(self.doc)
        self.top = top_face(self.ev, self.plate, 0.2)

    def kind_of(self, node):
        with self.assertRaises(EvaluationError) as caught:
            evaluate(self.doc).value(node)
        return caught.exception

    def test_an_edge_name_refuses_at_the_door_and_nothing_lands(self):
        """An edge name is refused as the seat's kind: what a name
        denotes is its own kind, so the door that reads it asks. The
        door is one action, so nothing of it lands."""
        edge = self.ev.all_edges(self.plate)[0]
        before = self.doc.node_count
        with self.assertRaises(EditError) as caught:
            self.doc.draw_on_face(self.plate, edge, PAD)
        refused = caught.exception
        self.assertEqual(refused.variant, "selection_name_kind")
        self.assertEqual(refused.found_kind, EntityKind.Edge)
        self.assertEqual(refused.expected_kind, EntityKind.Face)
        self.assertEqual(refused.name, edge)
        self.assertEqual(self.doc.node_count, before)

    def test_a_curved_carrier_refuses_at_evaluate(self):
        """A chart stands on a plane: a face read as a plane refuses a
        curved carrier, and `Evaluation.face_carrier_kind` is the door
        that answers the same question BEFORE the sketch is drawn."""
        torus = ring(self.doc)
        ev = evaluate(self.doc)
        face = ev.all_faces(torus)[0]
        profile = self.doc.draw_on_face(torus, face, PAD)
        err = self.kind_of(profile)
        self.assertEqual((err.kind, err.inner_kind), ("pose_read", "not_planar"))
        self.assertIs(ev.face_carrier_kind(torus, face), SurfaceKind.Torus)

    def test_a_name_that_does_not_denote_here_refuses_the_chart(self):
        """The N5 failure mode, and the evidence that the face is READ:
        a transcribed frame could not fail this way. A name the body
        does not hold does not resolve, so the profile's chart refuses;
        the repair is a rebind, not an edit of nine numbers."""
        second = plate(self.doc, 0.2 * m)
        profile = self.doc.draw_on_face(second, self.top, PAD)
        err = self.kind_of(profile)
        self.assertEqual(err.kind, "select_resolve")
        self.assertIn("Recourse: rebind it", str(err))

    def test_a_failed_chart_poisons_the_extrude_above_it(self):
        """One node out: the extrude of a profile whose chart cannot be
        read is poisoned rather than swept at a fabricated frame, and
        `through` names the profile."""
        second = plate(self.doc, 0.2 * m)
        profile = self.doc.draw_on_face(second, self.top, PAD)
        boss = self.doc.insert(Node.extrude(profile, 0.3 * m))
        err = self.kind_of(boss)
        self.assertEqual(err.reason, "poisoned")
        self.assertEqual(err.through, profile)
        self.assertEqual(err.kind, "select_resolve")

    def test_text_that_is_no_name_at_all_is_a_boundary_refusal(self):
        with self.assertRaises(ValueError):
            self.doc.draw_on_face(self.plate, "the top one", PAD)

    def test_a_slide_of_the_wrong_dimension_is_named_by_its_place(self):
        with self.assertRaisesRegex(ValueError, r"the slide at slides\[1\] reads a length, not an angle"):
            self.doc.draw_on_face(self.plate, self.top, PAD, slides=(0 * m, 1 * rad))


if __name__ == "__main__":
    unittest.main()
