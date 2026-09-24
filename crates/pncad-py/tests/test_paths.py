"""The PATHS authoring lattice, from Python.

Three things are asserted here, and they are the three halves of the
§L4 type story that a runtime can see:

1. The lattice WALKS — every verb of the current Rust vocabulary is
   reachable from the state that owns it, and the loop it lowers has
   the vertices the algebra says it has. That the set is COMPLETE is
   not this file's claim and cannot be: a corpus walk stays green
   whatever the kernel gains. The crate's `surface_census` is what
   holds the claim, keyed on `Verb::ALL` and `ArcMode::ALL`; what
   lives here is that each verb, once reachable, does what it says.
2. The off-lattice states are ABSENT. This file is the Python analog
   of the Rust E0599 compile-fail probes: a double director,
   `.tangent()` on a plain point, a leading `.fillet`, a leg from a
   half-bound tip, and `close()` are not methods that refuse — they
   are methods that do not exist.
3. Geometry refusals fire AT THE CALL SITE, as `pncad.PathError`
   carrying the kernel's own stable `variant` tag. Nothing is
   pre-checked in Python, so these are the kernel's refusals arriving
   unmodified, one stack frame from the verb that caused them.
"""

import math
import unittest

import pncad
from pncad import (
    ArcSide,
    ArcSweep,
    BooleanOp,
    Bulge,
    Center,
    Doc,
    Expr,
    Node,
    Open,
    Radius,
    Start,
    Via,
    circle,
    circle_split,
    deg,
    evaluate,
    load,
    m,
    rad,
)

ORIGIN = (0 * m, 0 * m)


class TestTheLatticeWalks(unittest.TestCase):
    """Each state's legal continuations, exercised."""

    def test_a_polygon_is_the_straight_run(self):
        loop = (
            Open.at(ORIGIN)
            .line_to((3 * m, 0 * m))
            .line_to((3 * m, 2 * m))
            .line_to((0 * m, 2 * m))
            .line_to(Start)
        )
        self.assertEqual(loop.vertex_count, 4)
        # One step per verb: four binders plus the closing one.
        self.assertEqual(loop.step_count, 5)

    def test_the_fillet_trims_its_two_carriers(self):
        loop = (
            Open.at(ORIGIN)
            .line_to((3 * m, 0 * m))
            .toward(0.0, 1.0)
            .fillet(0.5 * m)
            .toward(-1.0, 0.0)
            .to((0 * m, 2 * m))
            .line_to(Start)
        )
        # Three sharp corners survive; the fourth is replaced by the
        # arc's two tangent points. The virtual corner is never a
        # vertex, because it is never authored.
        self.assertEqual(loop.vertex_count, 5)

    def test_the_angle_first_entry_binds_in_the_other_order(self):
        loop = (
            Open.angle(0 * deg)
            .at(ORIGIN)
            .line(2 * m)
            .turn(90 * deg)
            .line(2 * m)
            .line_to((0 * m, 2 * m))
            .line_to(Start)
        )
        self.assertEqual(loop.vertex_count, 4)

    def test_the_tangent_seam_closes_with_an_arc(self):
        loop = (
            Open.at(ORIGIN)
            .line_to((2 * m, 0 * m))
            .turn(90 * deg)
            .line(1 * m)
            .tangent()
            .tangent_arc_to(Start)
        )
        self.assertEqual(loop.vertex_count, 3)

    def test_the_cusp_verb_authors_the_reverse_tangent_joint(self):
        """Evaluating clean is the assertion that the DECLARATION
        crossed, not merely that a loop lowered.

        The Rust twin (`declared_tangency.rs`) carries an explicit red
        half — the same loop with the declaration stripped refuses
        `UndeclaredTangency` — and this test rides that rather than
        repeating it. What licenses the inference here is that the
        same figure with `turn(180 * deg)` where `cusp()` stands
        refuses `junction_cusp` at the call: the geometry is identical
        and only the declaration differs, so a `cusp()` that failed to
        declare would land on that refusal instead of evaluating.
        """
        # The lune between two internally tangent circles, cut on the
        # y axis — the wedge-0/2π figure. `cusp()` is `tangent()`'s
        # mirror: it departs along the NEGATED incoming ray, so the
        # kiss is exact by construction and DECLARED, which is what
        # the profile gate wants and what no authored value can
        # supply.
        lune = (
            Open.at((0 * m, 4 * m))
            .angle(-90 * deg)
            .line(2 * m)
            .turn(90 * deg)
            .tangent_arc_to(ORIGIN)
            .cusp()
            .tangent_arc_to(Start)
        )
        self.assertEqual(lune.vertex_count, 3)
        doc = Doc()
        node = doc.insert(Node.profile(lune, plane=doc.sketch_frame()))
        self.assertTrue(evaluate(doc).succeeded(node))

    def test_the_three_arc_binding_modes(self):
        # One leg verb, three spec MODES: the mode is the binding, and
        # which modes a state admits is the admissibility matrix.
        via = (
            Open.at((1 * m, 0 * m))
            .arc_to(Via((0 * m, 1 * m), (-1 * m, 0 * m)))
            .line_to(Start)
        )
        centre = (
            Open.at((1 * m, 0 * m))
            .arc_to(Center(ORIGIN, ArcSweep.Ccw, (-1 * m, 0 * m)))
            .line_to(Start)
        )
        bulge = Open.at((1 * m, 0 * m)).arc_to(Bulge((-1 * m, 0 * m), 1.0)).line_to(Start)
        for name, loop in [("via", via), ("centre", centre), ("bulge", bulge)]:
            with self.subTest(mode=name):
                self.assertEqual(loop.vertex_count, 2)

    def test_two_arcs_on_one_carrier_meet_at_a_declared_tangent_joint(self):
        # The half-disc equator's shape at the +y pole: the second arc
        # leaves along the first's tangent (a DECLARED tangent joint —
        # the sixth round's spelling for adjacent same-carrier arcs)
        # and is derived from that tangent and its target. The verb
        # `arc_continue` this shape used to need is removed (BOOL-10).
        loop = (
            Open.at((1 * m, 0 * m))
            .arc_to(Center(ORIGIN, ArcSweep.Ccw, (0 * m, 1 * m)))
            .tangent()
            .tangent_arc_to((-1 * m, 0 * m))
            .line_to(Start)
        )
        self.assertEqual(loop.vertex_count, 3)

    def test_the_fused_verb_authors_both_carriers_and_closes(self):
        # The rocker eye's lens: the entry side rides the left lobe's
        # carrier, one fillet rounds the tip, and the arrival rides the
        # right lobe's carrier back to the entry — ONE authoring act,
        # because an arc and the fillet that trims it are one decision.
        tip = math.sqrt(0.75)
        loop = Open.arc_fillet_arc(
            Center((-0.5 * m, 0 * m), ArcSweep.Ccw, (0 * m, -tip * m)),
            0.25 * m,
            Center((0.5 * m, 0 * m), ArcSweep.Ccw, Start),
        )
        self.assertEqual(loop.vertex_count, 3)

    def test_a_straight_arrival_off_a_fused_arc_incoming(self):
        # The entry side rides the R = 5 circle, the fillet opens
        # against that carrier, and the arrival is the ordinary
        # straight pair. The corner is DERIVED (the ray meets the
        # circle at (±4, 3)); the reach gate discards the root the
        # anchor never came from.
        loop = (
            Open.arc_fillet(Center(ORIGIN, ArcSweep.Ccw, (5 * m, 0 * m)), 0.5 * m)
            .at((0 * m, 3 * m))
            .toward(-1.0, 0.0)
            .line(3 * m)
            .line_to(Start)
        )
        self.assertEqual(loop.vertex_count, 4)

    def test_the_on_carrier_tip_re_authors_its_carrier_by_radius(self):
        # An on-carrier tip carries position and tangent and nothing
        # else, so the verb that continues it AUTHORS the carrier:
        # `Radius` re-derives the centre from those two bits, which is
        # why tangency there needs nothing value-matched. Here the
        # derived centre IS the authored boss centre, exactly.
        loop = (
            Open.at((5.05 * m, -1.6 * m))
            .toward(2.1, 0.8)
            .fillet_arc(0.5 * m, Center((7 * m, 0 * m), ArcSweep.Ccw, (8.5 * m, 0 * m)))
            .arc_fillet(Radius(1.5 * m, ArcSide.Left), 0.5 * m)
            .at((4.05 * m, 1.35 * m))
            .toward(-4.1, 0.3)
            .line(1 * m)
            .line_to(Start)
        )
        self.assertEqual(loop.vertex_count, 6)

    def test_sharp_after_an_arc_arrival_takes_an_ordinary_director(self):
        # The interior arc arrival lands on an ordinary directed point
        # (the run to the hard anchor is emitted at the verb), so a
        # SHARP continuation is an ordinary director + leg — the
        # spelling the retired on-carrier state made unrepresentable.
        loop = (
            Open.at((5.05 * m, -1.6 * m))
            .toward(2.1, 0.8)
            .fillet_arc(0.5 * m, Center((7 * m, 0 * m), ArcSweep.Ccw, (8.5 * m, 0 * m)))
            .angle(2.6 * rad)
            .line(1 * m)
            .line_to(Start)
        )
        # Entry, trim, fillet arc, hard anchor, leg end: five vertices,
        # the authored anchor among them as a genuine sharp corner.
        self.assertEqual(loop.vertex_count, 5)

    def test_the_complete_loop_carrier_forms_are_one_step(self):
        self.assertEqual(circle(ORIGIN, 1 * m).step_count, 1)
        self.assertEqual(circle(ORIGIN, 1 * m).vertex_count, 2)
        split = circle_split(ORIGIN, 1 * m, 3, 0 * rad)
        self.assertEqual(split.step_count, 1)
        self.assertEqual(split.vertex_count, 3)

    def test_a_tip_forks_for_motif_exploration(self):
        tip = Open.at(ORIGIN).line_to((2 * m, 0 * m))
        short = tip.line_to((2 * m, 1 * m)).line_to(Start)
        tall = tip.line_to((2 * m, 3 * m)).line_to(Start)
        self.assertEqual((short.vertex_count, tall.vertex_count), (3, 3))


class TestTheOffLatticeStatesAreAbsent(unittest.TestCase):
    """The Python analog of the Rust E0599 probes.

    An off-lattice call is an `AttributeError` because the method does
    not exist on that state's class — not a runtime flag check, and not
    a refusal the geometry layer had to be asked about.
    """

    def test_a_second_director_is_not_a_method(self):
        directed = Open.at(ORIGIN).angle(45 * deg)
        for director in ["angle", "toward", "tangent", "turn"]:
            with self.subTest(director=director):
                self.assertFalse(hasattr(directed, director))

    def test_tangent_needs_a_leg_end_to_inherit_from(self):
        plain = Open.at(ORIGIN)
        self.assertFalse(hasattr(plain, "tangent"))
        self.assertFalse(hasattr(plain, "turn"))
        # A leg end has one, and that asymmetry is the whole point.
        self.assertTrue(hasattr(Open.at(ORIGIN).line_to((1 * m, 0 * m)), "tangent"))

    def test_a_leading_fillet_is_not_a_method(self):
        self.assertFalse(hasattr(Open, "fillet"))
        self.assertFalse(hasattr(Open.at(ORIGIN), "fillet"))
        self.assertFalse(hasattr(Open.angle(0 * deg), "fillet"))

    def test_no_leg_departs_a_half_bound_tip(self):
        for state, name in [
            (Open, "Open"),
            (Open.at(ORIGIN), "PathPoint"),
            (Open.angle(0 * deg), "PathAngle"),
        ]:
            with self.subTest(state=name):
                self.assertFalse(hasattr(state, "line"))
        # A position-only tip can still target: `line_to` needs no
        # bound departure, because the target supplies it.
        self.assertTrue(hasattr(Open.at(ORIGIN), "line_to"))
        self.assertFalse(hasattr(Open.angle(0 * deg), "line_to"))

    def test_there_is_no_close(self):
        states = [
            Open,
            Open.at(ORIGIN),
            Open.angle(0 * deg),
            Open.at(ORIGIN).angle(0 * deg),
            Open.at(ORIGIN).line_to((1 * m, 0 * m)),
        ]
        for state in states:
            with self.subTest(state=type(state).__name__):
                self.assertFalse(hasattr(state, "close"))

    def test_the_entry_cannot_close_on_a_start_that_does_not_exist_yet(self):
        self.assertFalse(hasattr(Open, "to"))


class TestRefusalsFireAtTheCallSite(unittest.TestCase):
    """The kernel's typed refusals, raised where the verb was written."""

    def refuses(self, variant, thunk):
        with self.assertRaises(pncad.PathError) as caught:
            thunk()
        self.assertEqual(caught.exception.variant, variant)
        self.assertIsInstance(caught.exception, pncad.PncadError)

    def test_the_junction_checks(self):
        east = Open.at(ORIGIN).line_to((1 * m, 0 * m))
        self.refuses("junction_tangent", lambda: east.angle(0 * deg))
        self.refuses("junction_cusp", lambda: east.angle(180 * deg))
        # `turn(0)` lands in the tangent band by construction.
        self.refuses("junction_tangent", lambda: east.turn(0 * deg))

    def test_the_collinear_tangent_arc_close_refuses(self):
        # Carrier identity is no longer the reason (ruled 2026-09-02:
        # every zero-turn joint is a declared tangent joint). What
        # refuses is the geometry: Start is collinear with the declared
        # departure and BEHIND it, so the tangent-chord angle is pi and
        # no arc spans the chord.
        self.refuses(
            "degenerate_arc_chord",
            lambda: Open.at(ORIGIN)
            .line_to((1 * m, 0 * m))
            .tangent()
            .tangent_arc_to(Start),
        )

    def test_a_fillet_needs_a_corner(self):
        self.refuses(
            "no_corner_for_fillet",
            lambda: Open.at(ORIGIN)
            .line_to((1 * m, 0 * m))
            .toward(0.0, 1.0)
            .fillet(0.2 * m)
            .toward(0.0, 1.0)
            .to((0 * m, 3 * m)),
        )

    def test_a_fillet_refusal_names_every_corner_it_tried(self):
        # The envelope: a refusal about a carrier PAIR reports every
        # corner that refused at the answering stage, each with its own
        # reason and its own point. A straight pair derives one corner,
        # so the list is one row; the reason is reachable without
        # parsing the sentence.
        with self.assertRaises(pncad.PathError) as caught:
            (
                Open.at(ORIGIN)
                .toward(1.0, 0.0)
                .fillet(2.5 * m)
                .toward(0.0, 1.0)
                .to((3 * m, 2 * m))
            )
        err = caught.exception
        self.assertEqual(err.variant, "no_corner_of_pair")
        self.assertEqual(len(err.corners), 1)
        (x, y, reason) = err.corners[0]
        self.assertEqual(reason, "anchor_outside_trimmed_extent")
        self.assertAlmostEqual(x, 3.0)
        self.assertAlmostEqual(y, 0.0)
        # The sentence names the corner it is about.
        self.assertIn("at corner (", str(err))
        # Every other refusal carries the attribute too, empty.
        with self.assertRaises(pncad.PathError) as other:
            circle(ORIGIN, 0 * m)
        self.assertIsNone(other.exception.corners)

    def test_the_sign_gates(self):
        self.refuses("nonpositive_circle_radius", lambda: circle(ORIGIN, 0 * m))
        self.refuses("circle_split_count", lambda: circle_split(ORIGIN, 1 * m, 1, 0 * rad))
        self.refuses("zero_direction", lambda: Open.toward(0.0, 0.0))
        # A director past the ~1e154 overflow band is NOT a zero
        # direction and does not get that word: the norm overflows to
        # infinity, which reads maximally definite to the sign gate,
        # and the door used to return a stored ray of (0, 0) through
        # this very call.
        self.refuses("non_finite_direction", lambda: Open.toward(1e200, 0.0))
        self.refuses(
            "nonpositive_fillet_radius",
            lambda: Open.at(ORIGIN)
            .line_to((1 * m, 0 * m))
            .toward(0.0, 1.0)
            .fillet(0 * m),
        )

    def test_coordinates_are_typed_quantities(self):
        # A bare number is a boundary refusal, not an ambiguous unit.
        with self.assertRaises(TypeError):
            Open.at((0.0, 0.0))
        with self.assertRaises(TypeError):
            Open.at(ORIGIN).line_to((1 * m, 0 * m)).turn(1.0)


class TestTheSeamArrival(unittest.TestCase):
    """`Start.arrives_tangent()`: the seam's joint DECLARED tangent.

    The seam is the one joint whose arriving leg is authored last, so
    its declaration rides the target. Each closer below is shown in a
    pair: the undeclared close of the same figure refuses
    `seam_tangent` at the call, and the declared one closes and
    evaluates — the declaration, not the geometry, is what differs.
    """

    def closes_only_declared(self, close):
        with self.assertRaises(pncad.PathError) as caught:
            close(Start)
        self.assertEqual(caught.exception.variant, "seam_tangent")
        loop = close(Start.arrives_tangent())
        doc = Doc()
        node = doc.insert(Node.profile(loop, plane=doc.sketch_frame()))
        self.assertTrue(evaluate(doc).succeeded(node))
        return loop

    def test_a_straight_closer_declares_the_seam(self):
        # A D: the straight closing leg runs up the entry's own line.
        d = (
            Open.at(ORIGIN)
            .angle(90 * deg)
            .line(2 * m)
            .arc_to(Bulge((0 * m, -2 * m), 1.0))
        )
        loop = self.closes_only_declared(d.line_to)
        self.assertEqual(loop.vertex_count, 3)

    def test_a_tangent_arc_closer_declares_the_seam(self):
        stadium = (
            Open.at(ORIGIN)
            .angle(0 * deg)
            .line(2 * m)
            .tangent()
            .tangent_arc_to((2 * m, 2 * m))
            .tangent()
            .line(2 * m)
            .tangent()
        )
        loop = self.closes_only_declared(stadium.tangent_arc_to)
        self.assertEqual(loop.vertex_count, 4)

    def test_a_bulge_closer_declares_the_seam(self):
        # Three sides of a square, then a quarter-bulge arc whose end
        # tangent lands on the entry's outgoing direction.
        tip = (
            Open.at(ORIGIN)
            .angle(0 * deg)
            .line(2 * m)
            .turn(90 * deg)
            .line(2 * m)
            .turn(90 * deg)
            .line(2 * m)
            .turn(90 * deg)
            .line(1 * m)
        )
        loop = self.closes_only_declared(lambda target: tip.arc_to(Bulge(target, 1.0)))
        self.assertEqual(loop.vertex_count, 5)

    def test_the_declaration_is_checked(self):
        # A square's seam is a right angle; declaring it tangent is
        # contradicted by the geometry, and the kernel says so.
        tip = (
            Open.at(ORIGIN)
            .line_to((1 * m, 0 * m))
            .line_to((1 * m, 1 * m))
            .line_to((0 * m, 1 * m))
        )
        with self.assertRaises(pncad.PathError) as caught:
            tip.line_to(Start.arrives_tangent())
        self.assertEqual(caught.exception.variant, "seam_arrival_off_direction")

    def test_via_and_center_do_not_take_the_declaration(self):
        # The kernel's `Via` and `Center` closers take bare `Start`
        # only, so the declared token is refused at construction — the
        # Python face of Rust's missing trait impl. Bare `Start` builds,
        # so the token is the only difference.
        Via((1 * m, 1 * m), Start)
        Center(ORIGIN, ArcSweep.Ccw, Start)
        arriving = Start.arrives_tangent()
        with self.assertRaises(TypeError):
            Via((1 * m, 1 * m), arriving)
        with self.assertRaises(TypeError):
            Center(ORIGIN, ArcSweep.Ccw, arriving)

    def test_the_token_spells_itself(self):
        self.assertEqual(repr(Start.arrives_tangent()), "Start.arrives_tangent()")


class TestTheProfileNode(unittest.TestCase):
    """`Node.profile` builds the document node from the RECORDED
    program, so authoring and replay are one program."""

    def rounded(self):
        return (
            Open.at(ORIGIN)
            .line_to((3 * m, 0 * m))
            .toward(0.0, 1.0)
            .fillet(0.5 * m)
            .toward(-1.0, 0.0)
            .to((0 * m, 2 * m))
            .line_to(Start)
        )

    def test_an_arc_bearing_profile_evaluates(self):
        doc = Doc()
        solid = doc.insert(Node.extrude(doc.insert(Node.profile(self.rounded(), plane=doc.sketch_frame())), Expr.length_in(1, m)))
        ev = evaluate(doc)
        self.assertTrue(ev.succeeded(solid))
        body = ev.value(solid).body()
        body.validate()
        # 3 x 2 rectangle, one r = 0.5 corner rounded away.
        expected = 6.0 - (0.25 - math.pi / 16.0)
        self.assertAlmostEqual(body.mass_properties().volume, expected, delta=1e-12)

    def test_the_carrier_form_lands_as_its_own_program_arm(self):
        doc = Doc()
        solid = doc.insert(
            Node.extrude(doc.insert(Node.profile(circle((0 * m, 0 * m), 0.5 * m), plane=doc.sketch_frame())), Expr.length_in(2, m))
        )
        ev = evaluate(doc)
        self.assertTrue(ev.succeeded(solid))
        volume = ev.value(solid).body().mass_properties().volume
        self.assertAlmostEqual(volume, math.pi * 0.25 * 2.0, delta=1e-12)

    def test_the_elevation_is_the_polygon_plane_story_unchanged(self):
        # A boss on a plate, raised half its own height: the fused
        # volume names where `elevation` put the boss — at z = 0 the
        # boss would be swallowed whole and the union would be 4.0.
        def rect(x0, x1, y0, y1):
            return (
                Open.at((x0 * m, y0 * m))
                .line_to((x1 * m, y0 * m))
                .line_to((x1 * m, y1 * m))
                .line_to((x0 * m, y1 * m))
                .line_to(Start)
            )

        doc = Doc()
        plate = doc.insert(
            Node.extrude(doc.insert(Node.profile(rect(0, 2, 0, 2), plane=doc.sketch_frame())), Expr.length_in(1, m))
        )
        boss = doc.insert(
            Node.extrude(
                doc.insert(
                    Node.profile(rect(0.5, 1.5, 0.5, 1.5), plane=doc.sketch_frame(elevation=Expr.length_in(0.5, m)))
                ),
                Expr.length_in(1, m),
            )
        )
        fused = doc.insert(Node.boolean(BooleanOp.Union, plate, boss))
        ev = evaluate(doc)
        self.assertTrue(ev.succeeded(fused))
        volume = ev.value(fused).body().mass_properties().volume
        self.assertAlmostEqual(volume, 4.5, delta=1e-12)

    def test_the_program_survives_persistence_bit_for_bit(self):
        doc = Doc()
        doc.insert(Node.extrude(doc.insert(Node.profile(self.rounded(), plane=doc.sketch_frame())), Expr.length_in(1, m)))
        replayed = load(doc.save()).doc
        self.assertTrue(doc.bit_eq(replayed), "replay is bit-identical, not merely close")


class TestTheWorkedExample(unittest.TestCase):
    """`examples/bracket.py` is the front-door script; running it is
    the only way it cannot rot."""

    def test_the_bracket_example_runs(self):
        import contextlib
        import io
        import runpy
        from pathlib import Path

        script = (
            Path(__file__).resolve().parents[1] / "examples" / "bracket.py"
        )
        self.assertTrue(script.is_file(), f"missing example: {script}")
        # The script asserts its own oracles and its own STEP
        # round-trip; this only has to let it speak.
        with contextlib.redirect_stdout(io.StringIO()) as out:
            runpy.run_path(str(script), run_name="__main__")
        self.assertIn("re-imported OK", out.getvalue())


if __name__ == "__main__":
    unittest.main()
