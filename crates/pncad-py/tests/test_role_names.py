"""The five role-name doors — `band`, `band_pi`, `band_rim`,
`meridian_vertex` and `carried`, mirroring `pncad::select`'s builders.

`Evaluation.select` and the whole-body materializers answer names FROM
an evaluation. A selection that is AUTHORED — `Node.fillet`'s frozen
selection, `Node.shell`'s open list — is written before any evaluation
of the minting node exists, so its names are spelled, and these five
spell them.

WHAT IS PINNED HERE, and why it is the only claim worth making: the
text a door answers is BYTE-IDENTICAL to the text the kernel's own
emitter answers for that entity. The left side of every equality below
is a name Rust minted and serialized (a materialized selection); the
right side is the Python door's answer on the same arguments. A door
that agreed on shape and not on bytes would author a selection that
resolves to nothing, which is exactly the failure a hand-written name
has.

EVERY DOOR TAKES A PIECE. A profile's segments are named by the
pieces that draw them — the step that authored the segment, by the id
the document minted for it, and its role in that step — and a piece is
spelled from the handle the step's verb returned:
`doc.piece(profile, loop, h.leg)`, with `loop` the loop the author
states (0 the outer one, then the holes in the order the profile
describes them). A rim or meridian vertex is spelled by the piece that
starts at it. No loop is privileged, which is why the third scene
below has a hole and names its bands the same way the first scene
names the outer ones.

The first two scenes are the two shapes a full revolve takes. A
profile that CLEARS the axis sweeps to one face per meridian segment,
so its bands stand alone and its latitude rims are whole circles. A
profile that TOUCHES the axis sweeps to a pole, and the kernel splits
every band into its `[0, pi)` and `[pi, 2pi)` halves — which is what
`band_pi` exists for, and why the second scene is here at all. The
third carries a HOLE, so the emitter mints a second loop's worth of
bands, rims and meridian vertices on its second loop.
"""

import json
import math
import unittest

from pncad import (
    Doc,
    EntityKind,
    Expr,
    GeomPred,
    MeridianEnd,
    NamePat,
    Node,
    Open,
    SegPat,
    SegTag,
    Selector,
    Start,
    SurfaceKind,
    band,
    band_pi,
    band_rim,
    carried,
    evaluate,
    m,
    meridian_vertex,
    rad,
)

# The ring: a square section between radii RI and RO, one turn about
# the sketch's own +y. Four meridian segments, none of them on the
# axis, so nothing sweeps to a pole.
RI, RO, H = 1.0, 2.0, 1.0
# The frustum: a trapezoid whose fourth segment lies ON the axis, so
# the body has a pole and every band is split at pi.
R_BASE, R_TOP, H_F = 1.0, 0.5, 1.0
# The holed ring: the same square section with a square HOLE cut in
# it, so the profile has two loops and the revolve two shells.
HI, HO, HB, HT = 1.25, 1.75, 0.25, 0.75
# The wall and the roll, both dyadic.
T, ROLL = 0.125, 0.125


def axis_of(doc, frame):
    """The sketch frame's own +y through its origin."""
    return doc.insert(Node.datum_axis_in_plane(frame, (
        Expr.length_in(0, m),
        Expr.length_in(0, m),
    ), (
        Expr.literal(0.0),
        Expr.literal(1.0),
    )))


def ring(doc):
    """A square-section ring: `band` 0 is the bottom annulus, 1 the
    outer cylinder, 2 the top annulus, 3 the inner one."""
    frame = doc.sketch_frame()
    chain, legs = polygon([(RI, 0), (RO, 0), (RO, H), (RI, H)])
    profile = doc.insert(Node.profile(chain, plane=frame))
    return Scene(doc, profile, [legs]), doc.insert(
        Node.revolve(profile, axis_of(doc, frame), Expr.angle_in(2 * math.pi, rad))
    )


def frustum(doc):
    """A truncated cone standing on the axis: `band` 0 is the base
    disc, 1 the cone wall, 2 the top disc, and segment 3 is the
    meridian ON the axis, which sweeps nothing."""
    frame = doc.sketch_frame()
    chain, legs = polygon([(0, 0), (R_BASE, 0), (R_TOP, H_F), (0, H_F)])
    profile = doc.insert(Node.profile(chain, plane=frame))
    return Scene(doc, profile, [legs]), doc.insert(
        Node.revolve(profile, axis_of(doc, frame), Expr.angle_in(2 * math.pi, rad))
    )


def holed_ring(doc):
    """The ring's square section with a square hole cut in it: loop 0
    is the outer boundary, loop 1 the hole, and the revolve of a
    lamina-with-hole is one body of two square-torus shells."""
    frame = doc.sketch_frame()
    outer, outer_legs = polygon([(RI, 0), (RO, 0), (RO, H), (RI, H)])
    hole, hole_legs = polygon([(HI, HB), (HO, HB), (HO, HT), (HI, HT)])
    profile = doc.insert(Node.profile([outer, hole], plane=frame))
    return Scene(doc, profile, [outer_legs, hole_legs]), doc.insert(
        Node.revolve(
            profile,
            axis_of(doc, frame),
            Expr.angle_in(2 * math.pi, rad),
        )
    )


def polygon(corners):
    """A closed chain through `corners` (in metres), and the handle of
    each leg in authored order: leg `k` runs from corner `k` to corner
    `k + 1`, the last one back to the start."""
    path = Open.at((corners[0][0] * m, corners[0][1] * m))
    legs = []
    for x, y in corners[1:]:
        path = path.line_to((x * m, y * m))
        legs.append(path.step)
    closed = path.line_to(Start)
    legs.append(closed.step)
    return closed, legs


class Scene:
    """A profile in a document and the handles of its loops' legs."""

    def __init__(self, doc, profile, legs):
        self.doc, self.profile, self.legs = doc, profile, legs

    def piece(self, lp, k):
        """The piece leg `k` of loop `lp` draws — and the one starting
        at the corner that leg leaves."""
        return self.doc.piece(self.profile, lp, self.legs[lp][k].leg)


def step_id(piece):
    """The minted id a piece spells — the key its names sort by."""
    return json.loads(str(piece))["Piece"]["step"]


def of_role(ev, node, kind, tag, side=None):
    """What the kernel minted for one role, in the canonical order
    `select` answers in."""
    pat = SegPat.tag(tag)
    if side is not None:
        pat = pat.side(side)
    return ev.select(node, Selector.of(NamePat.of_kind(kind).seg(pat)))


class TestTheDoorAnswersTheKernelsOwnText(unittest.TestCase):
    """Per builder, on the same arguments: the door's text and the
    emitter's text, compared as bytes."""

    def test_band_and_its_rims_and_meridian_vertices_on_a_ring(self):
        doc = Doc()
        scene, node = ring(doc)
        ev = evaluate(doc)
        bands = of_role(ev, node, EntityKind.Face, SegTag.Band)
        self.assertEqual(len(bands), 4, "one band per meridian segment")
        # `select` answers in NAME order: for names that differ only in
        # the piece, that is the pieces' step ids ascending, not the
        # program's segment order.
        by_id = sorted((scene.piece(0, k) for k in range(4)), key=step_id)
        self.assertEqual(bands, [band(node, p) for p in by_id])
        # No pole, so no `[pi, 2pi)` half exists to name.
        self.assertEqual(of_role(ev, node, EntityKind.Face, SegTag.BandPi), [])
        # A rim per meridian vertex, and a seam vertex under each.
        rims = of_role(ev, node, EntityKind.Edge, SegTag.BandRim)
        self.assertEqual(rims, [band_rim(node, p) for p in by_id])
        seam = of_role(
            ev, node, EntityKind.Vertex, SegTag.MeridianVertex, MeridianEnd.Seam
        )
        self.assertEqual(seam, [meridian_vertex(MeridianEnd.Seam, node, p) for p in by_id])
        # The names denote what the door says they denote: `band` 2 is
        # the top annulus, and rim 2 the circle standing on it.
        self.assertEqual(
            ev.face_carrier_kind(node, band(node, scene.piece(0, 2))), SurfaceKind.Plane
        )
        self.assertEqual(
            ev.face_frame(node, band(node, scene.piece(0, 2))).origin[1].meters, H
        )
        self.assertEqual(ev.edge_frame(node, band_rim(node, scene.piece(0, 2))).origin[1].meters, H)

    def test_band_pi_is_the_half_a_pole_splits_off(self):
        doc = Doc()
        scene, node = frustum(doc)
        ev = evaluate(doc)
        bands = of_role(ev, node, EntityKind.Face, SegTag.Band)
        halves = of_role(ev, node, EntityKind.Face, SegTag.BandPi)
        self.assertEqual(len(bands), 3, "the fourth segment lies on the axis")
        # In NAME order, the pieces' step ids ascending.
        by_id = sorted((scene.piece(0, seg) for seg in range(3)), key=step_id)
        self.assertEqual(bands, [band(node, p) for p in by_id])
        # Only the CURVED wall has a pi half: a full revolve sweeps a
        # planar wall (the two discs) whole, one face, its `band`.
        self.assertEqual(halves, [band_pi(node, scene.piece(0, 1))])
        # A door answering the other door's text would pass every
        # count above; these are two roles and two texts.
        self.assertNotEqual(band(node, scene.piece(0, 1)), band_pi(node, scene.piece(0, 1)))

    def test_a_holes_bands_are_named_at_its_own_loop(self):
        """The claim the outer-loop signature could not make: the
        emitter mints a hole's band, rim and meridian vertex on the
        hole's own pieces, and the door answers those bytes for the
        same pieces. Compared as SETS over the two loops' legs,
        because `select` answers in name order, which is not authored
        order."""
        doc = Doc()
        scene, node = holed_ring(doc)
        ev = evaluate(doc)
        bands = of_role(ev, node, EntityKind.Face, SegTag.Band)
        self.assertEqual(len(bands), 8, "four meridian segments per loop")
        self.assertEqual(
            set(bands), {band(node, scene.piece(lp, s)) for lp in (0, 1) for s in range(4)}
        )
        rims = of_role(ev, node, EntityKind.Edge, SegTag.BandRim)
        self.assertEqual(
            set(rims), {band_rim(node, scene.piece(lp, v)) for lp in (0, 1) for v in range(4)}
        )
        seam = of_role(
            ev, node, EntityKind.Vertex, SegTag.MeridianVertex, MeridianEnd.Seam
        )
        self.assertEqual(
            set(seam),
            {meridian_vertex(MeridianEnd.Seam, node, scene.piece(lp, v)) for lp in (0, 1) for v in range(4)},
        )
        # Neither loop clears into a pole, so nothing is split at pi
        # and no `band_pi` name exists on either loop.
        self.assertEqual(of_role(ev, node, EntityKind.Face, SegTag.BandPi), [])
        # The same position on the two loops is two pieces, and two
        # names.
        self.assertNotEqual(band(node, scene.piece(0, 0)), band(node, scene.piece(1, 0)))
        # And the names denote faces of the shape the section has:
        # two horizontal segments sweep to annuli and two vertical
        # ones to cylinders, on each loop.
        for lp in (0, 1):
            kinds = [
                ev.face_carrier_kind(node, band(node, scene.piece(lp, s))) for s in range(4)
            ]
            self.assertEqual(kinds.count(SurfaceKind.Plane), 2)
            self.assertEqual(kinds.count(SurfaceKind.Cylinder), 2)
        # Pappus over the section the revolve swept: the outer square
        # less the hole, each at its own centroid radius. Every
        # dimension is dyadic; the closed form sums pi's rounding in a
        # different order than the kernel accumulates in.
        outer_area, hole_area = (RO - RI) * H, (HO - HI) * (HT - HB)
        want = 2 * math.pi * (
            outer_area * (RI + RO) / 2 - hole_area * (HI + HO) / 2
        )
        body = ev.value(node).body()
        body.validate()
        self.assertAlmostEqual(
            body.mass_properties().volume, want, delta=1e-12 * want
        )

    def test_carried_is_the_name_a_survivor_of_the_next_op_wears(self):
        doc = Doc()
        scene, node = ring(doc)
        # The top annulus opened: the other three bands survive the
        # hollowing, and each wears its own name under the shell.
        hollow = doc.insert(Node.shell(node, Expr.length_in(T, m), [band(node, scene.piece(0, 2))]))
        ev = evaluate(doc)
        survivors = ev.select(
            hollow,
            Selector.of(
                NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.FromTarget))
            ),
        )
        self.assertEqual(
            sorted(survivors),
            sorted(carried(hollow, band(node, scene.piece(0, seg))) for seg in (0, 1, 3)),
        )
        # The kind is the inner name's, never re-decided: a carried
        # face resolves as a face.
        self.assertEqual(ev.resolve(carried(hollow, band(node, scene.piece(0, 0)))).status, "resolved")

    def test_text_that_is_not_a_name_refuses_at_the_boundary(self):
        doc = Doc()
        _scene, node = ring(doc)
        with self.assertRaises(ValueError):
            carried(node, "the bottom face")


class TestASelectionAuthoredBeforeAnyEvaluation(unittest.TestCase):
    """The doors' reason to exist: a fillet and a shell whose
    selections are written with no evaluation in hand, then evaluated,
    with the named entities reached and the result held to its own
    closed form."""

    def test_a_shell_opened_at_a_band_named_before_the_revolve_ran(self):
        doc = Doc()
        scene, node = ring(doc)
        # Authored against the recipe alone — nothing is evaluated
        # until the assertion below.
        hollow = doc.insert(Node.shell(node, Expr.length_in(T, m), [band(node, scene.piece(0, 2))]))
        ev = evaluate(doc)
        body = ev.value(hollow).body()
        body.validate()
        # The cavity is the annulus between the walls, one wall short
        # of the top it was opened at.
        cavity = math.pi * ((RO - T) ** 2 - (RI + T) ** 2) * (H - T)
        want = math.pi * (RO * RO - RI * RI) * H - cavity
        # Every dimension here is dyadic and the kernel's volume is
        # analytic, but the closed form sums pi's rounding in a
        # different order, so the two agree to their last bit rather
        # than to every bit — which is the bound worth asserting, not
        # a decimal copied off a run.
        self.assertAlmostEqual(
            body.mass_properties().volume, want, delta=1e-12 * want
        )
        # The named face is gone and its rim stands where it was: one
        # designated chart, one annular rim.
        faces = NamePat.of_kind(EntityKind.Face)
        rim = ev.select(hollow, Selector.of(faces.seg(SegPat.tag(SegTag.Rim))))
        self.assertEqual(len(rim), 1)
        self.assertIn(band(node, scene.piece(0, 2)), rim[0], "the rim wears the opened face's name")

    def test_a_fillet_on_rims_named_before_the_revolve_ran(self):
        doc = Doc()
        scene, node = ring(doc)
        rolled = doc.insert(
            Node.fillet(
                node,
                Expr.length_in(ROLL, m),
                [band_rim(node, scene.piece(0, 2)), band_rim(node, scene.piece(0, 3))],
            )
        )
        ev = evaluate(doc)
        body = ev.value(rolled).body()
        body.validate()
        # Two rims rolled, two tori.
        tori = ev.select_where(
            rolled,
            Selector.of(NamePat.of_kind(EntityKind.Face)),
            [GeomPred.surface_kind(SurfaceKind.Torus)],
        )
        self.assertEqual(len(tori), 2)
        # Pappus over each roll's own removed section: the corner
        # square less its quarter disc, swept about the axis at the
        # section's centroid radius. Exact, not a transcribed run.
        area = ROLL * ROLL * (1 - math.pi / 4)

        def centroid(r, outward):
            half = ROLL / 2 if outward else -ROLL / 2
            corner = ROLL if outward else -ROLL
            third = ROLL / 3 if outward else -ROLL / 3
            return (
                (r - half) - (math.pi / 4) * (r - corner) - third
            ) / (1 - math.pi / 4)

        removed = (
            2 * math.pi * area * (centroid(RO, True) + centroid(RI, False))
        )
        want = math.pi * (RO * RO - RI * RI) * H - removed
        self.assertAlmostEqual(
            body.mass_properties().volume, want, delta=1e-12 * want
        )


if __name__ == "__main__":
    unittest.main()
