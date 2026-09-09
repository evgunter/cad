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

THE OUTER-LOOP LIMIT, restated because these doors inherit it: `seg`
and `vertex` index the OUTER loop's canonical chain
(`work/lib/the-role-name-builders-reach-only-the-outer-profile-loop.md`
carries it on the Rust side). A hole's band is reachable from neither
alphabet, and no scene here has one.

The two scenes are the two shapes a full revolve takes. A profile that
CLEARS the axis sweeps to one face per meridian segment, so its bands
stand alone and its latitude rims are whole circles. A profile that
TOUCHES the axis sweeps to a pole, and the kernel splits every band
into its `[0, pi)` and `[pi, 2pi)` halves — which is what `band_pi`
exists for, and why the second scene is here at all.
"""

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
    chain = (
        Open.at((RI * m, 0 * m))
        .line_to((RO * m, 0 * m))
        .line_to((RO * m, H * m))
        .line_to((RI * m, H * m))
        .line_to(Start)
    )
    profile = doc.insert(Node.profile(chain, plane=frame))
    return doc.insert(Node.revolve(profile, axis_of(doc, frame), Expr.angle_in(2 * math.pi, rad)))


def frustum(doc):
    """A truncated cone standing on the axis: `band` 0 is the base
    disc, 1 the cone wall, 2 the top disc, and segment 3 is the
    meridian ON the axis, which sweeps nothing."""
    frame = doc.sketch_frame()
    chain = (
        Open.at((0 * m, 0 * m))
        .line_to((R_BASE * m, 0 * m))
        .line_to((R_TOP * m, H_F * m))
        .line_to((0 * m, H_F * m))
        .line_to(Start)
    )
    profile = doc.insert(Node.profile(chain, plane=frame))
    return doc.insert(Node.revolve(profile, axis_of(doc, frame), Expr.angle_in(2 * math.pi, rad)))


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
        node = ring(doc)
        ev = evaluate(doc)
        bands = of_role(ev, node, EntityKind.Face, SegTag.Band)
        self.assertEqual(len(bands), 4, "one band per meridian segment")
        self.assertEqual(bands, [band(node, seg) for seg in range(4)])
        # No pole, so no `[pi, 2pi)` half exists to name.
        self.assertEqual(of_role(ev, node, EntityKind.Face, SegTag.BandPi), [])
        # A rim per meridian vertex, and a seam vertex under each.
        rims = of_role(ev, node, EntityKind.Edge, SegTag.BandRim)
        self.assertEqual(rims, [band_rim(node, v) for v in range(4)])
        seam = of_role(
            ev, node, EntityKind.Vertex, SegTag.MeridianVertex, MeridianEnd.Seam
        )
        self.assertEqual(
            seam, [meridian_vertex(MeridianEnd.Seam, node, v) for v in range(4)]
        )
        # The names denote what the door says they denote: `band` 2 is
        # the top annulus, and rim 2 the circle standing on it.
        self.assertEqual(
            ev.face_carrier_kind(node, band(node, 2)), SurfaceKind.Plane
        )
        self.assertEqual(
            ev.face_frame(node, band(node, 2)).origin[1].meters, H
        )
        self.assertEqual(ev.edge_frame(node, band_rim(node, 2)).origin[1].meters, H)

    def test_band_pi_is_the_half_a_pole_splits_off(self):
        doc = Doc()
        node = frustum(doc)
        ev = evaluate(doc)
        bands = of_role(ev, node, EntityKind.Face, SegTag.Band)
        halves = of_role(ev, node, EntityKind.Face, SegTag.BandPi)
        self.assertEqual(len(bands), 3, "the fourth segment lies on the axis")
        self.assertEqual(len(halves), 3, "and each band has its pi half")
        self.assertEqual(bands, [band(node, seg) for seg in range(3)])
        self.assertEqual(halves, [band_pi(node, seg) for seg in range(3)])
        # A door answering the other door's text would pass every
        # count above; these are two roles and two texts.
        self.assertNotEqual(band(node, 0), band_pi(node, 0))

    def test_carried_is_the_name_a_survivor_of_the_next_op_wears(self):
        doc = Doc()
        node = ring(doc)
        # The top annulus opened: the other three bands survive the
        # hollowing, and each wears its own name under the shell.
        hollow = doc.insert(Node.shell(node, Expr.length_in(T, m), [band(node, 2)]))
        ev = evaluate(doc)
        survivors = ev.select(
            hollow,
            Selector.of(
                NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.FromTarget))
            ),
        )
        self.assertEqual(
            sorted(survivors),
            sorted(carried(hollow, band(node, seg)) for seg in (0, 1, 3)),
        )
        # The kind is the inner name's, never re-decided: a carried
        # face resolves as a face.
        self.assertEqual(ev.resolve(carried(hollow, band(node, 0))).status, "resolved")

    def test_text_that_is_not_a_name_refuses_at_the_boundary(self):
        doc = Doc()
        node = ring(doc)
        with self.assertRaises(ValueError):
            carried(node, "the bottom face")


class TestASelectionAuthoredBeforeAnyEvaluation(unittest.TestCase):
    """The doors' reason to exist: a fillet and a shell whose
    selections are written with no evaluation in hand, then evaluated,
    with the named entities reached and the result held to its own
    closed form."""

    def test_a_shell_opened_at_a_band_named_before_the_revolve_ran(self):
        doc = Doc()
        node = ring(doc)
        # Authored against the recipe alone — nothing is evaluated
        # until the assertion below.
        hollow = doc.insert(Node.shell(node, Expr.length_in(T, m), [band(node, 2)]))
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
        self.assertIn(band(node, 2), rim[0], "the rim wears the opened face's name")

    def test_a_fillet_on_rims_named_before_the_revolve_ran(self):
        doc = Doc()
        node = ring(doc)
        rolled = doc.insert(
            Node.fillet(node, Expr.length_in(ROLL, m), [band_rim(node, 2), band_rim(node, 3)])
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
