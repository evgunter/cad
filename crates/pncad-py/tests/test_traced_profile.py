"""`Node.drawn_profile` and `Doc.draw_on_face`: a sketch continues
tangent to a surface's trace (INTENT stage 6 C, D10 "Constructing a
coincidence").

The Python mirror of `crates/editor-core/tests/intent_s6_c_trace.rs`: a
profile drawn on a frame of a cylinder's space runs up until it leaves
the cylinder's wall, then along the wall's trace, and its extrude stands
in the cylinder's space with its wall at the cylinder's radius. A step
whose trace refuses refuses at `evaluate` under the `trace` class. The
same chain drawn on the bottom cap's chart is the mirror of
`crates/editor-core/tests/intent_s6_e_traced_on_a_face.rs`.
"""

import unittest

from pncad import (
    Chain,
    Doc,
    EditError,
    EvaluationError,
    Node,
    PoseDef,
    Start,
    SurfaceKind,
    TraceTarget,
    circle,
    deg,
    evaluate,
    m,
)
from measuring import pinned

R = 0.5


def cylinder(doc):
    """A cylinder of radius `R` and height 1 on its own xy plane."""
    disc = doc.insert(Node.profile(circle((0 * m, 0 * m), R * m)))
    return doc.insert(Node.extrude(disc, 1.0 * m))


def frame_and_wall(doc, cyl):
    """The cylinder's frame at its base, read off its axis and its
    seam's base vertex, and its round wall's name."""
    ev = evaluate(doc)
    wall = next(
        f for f in sorted(ev.all_faces(cyl)) if ev.face_carrier_kind(cyl, f) == SurfaceKind.Cylinder
    )
    base = min(ev.all_vertices(cyl), key=lambda v: ev.vertex_position(cyl, v)[2].meters)
    return PoseDef.through(PoseDef.axis(cyl, wall), PoseDef.point(cyl, base)), wall


def volume(ev, node):
    """`node`'s body's volume, pinned."""
    return pinned(ev.value(node).body()).volume


def caps(ev, cyl):
    """The cylinder's two planar caps."""
    return [f for f in sorted(ev.all_faces(cyl)) if ev.face_carrier_kind(cyl, f) == SurfaceKind.Plane]


def bottom_and_wall(doc, cyl):
    """The cylinder's bottom cap and its round wall."""
    ev = evaluate(doc)
    _, wall = frame_and_wall(doc, cyl)
    bottom = min(caps(ev, cyl), key=lambda f: ev.face_frame(cyl, f).origin[2].meters)
    return bottom, wall


def footprint(passage="leave", nth=1):
    """Up from `(0, -1/4)` until the line leaves the wall, then against
    the trace (its tangent there runs along the frame's x) to `x = 1`,
    down and home."""
    return (
        Chain.at(0 * m, -0.25 * m)
        .toward(0.0, 1.0)
        .line_to(TraceTarget.on_trace(0, passage, nth))
        .along_trace("against")
        .line_to(TraceTarget.at_x(1.0 * m))
        .line_to((1.0 * m, -0.25 * m))
        .line_to(Start)
    )


class TracedProfileTest(unittest.TestCase):
    def test_the_boss_runs_tangent_to_the_cylinders_wall(self):
        doc = Doc()
        cyl = cylinder(doc)
        frame, wall = frame_and_wall(doc, cyl)
        sketch = doc.insert(Node.drawn_profile(frame, [(cyl, wall)], footprint()))
        boss = doc.insert(Node.extrude(sketch, 0.5 * m))
        ev = evaluate(doc)
        ev.value(boss).body().validate()
        ys = [abs(ev.vertex_position(boss, v)[1].meters) for v in ev.all_vertices(boss)]
        self.assertAlmostEqual(max(ys), R, delta=1e-12)
        self.assertEqual(doc.space(boss), doc.space(cyl), "the boss is in the cylinder's space")

    def test_a_boss_traced_off_the_wall_on_the_bottom_caps_chart_stands_outward(self):
        """`Doc.draw_on_face` on the bottom cap, tracing the wall: the
        cap's chart has `z` along its outward normal (the space's `-z`)
        and `y` along the space's `-y`, so the footprint's wall stands at
        `y = -r`, a `+` depth stands below the base and a `-` depth runs
        up along the wall."""
        for depth, (low, high) in ((0.5, (-0.5, 0.0)), (-0.5, (0.0, 0.5))):
            with self.subTest(depth=depth):
                doc = Doc()
                cyl = cylinder(doc)
                bottom, wall = bottom_and_wall(doc, cyl)
                sketch = doc.draw_on_face(cyl, bottom, footprint(), traced=[(cyl, wall)])
                boss = doc.insert(Node.extrude(sketch, depth * m))
                ev = evaluate(doc)
                at = [ev.vertex_position(boss, v) for v in ev.all_vertices(boss)]
                self.assertAlmostEqual(min(p[1].meters for p in at), -R, delta=1e-12)
                self.assertAlmostEqual(min(p[2].meters for p in at), low, delta=1e-12)
                self.assertAlmostEqual(max(p[2].meters for p in at), high, delta=1e-12)
                self.assertEqual(doc.space(boss), doc.space(cyl), "the boss is in the cylinder's space")
                # Outward the boss shares no material with the cylinder,
                # so it unites whole; up the wall it is a pocket's tool,
                # and the pocket and what it takes make the cylinder.
                if depth > 0:
                    combined = doc.insert(Node.union([cyl, boss]))
                    want = volume(ev, cyl) + volume(ev, boss)
                else:
                    combined = doc.insert(Node.subtract(cyl, boss))
                    taken = doc.insert(Node.intersect([cyl, boss]))
                    ev = evaluate(doc)
                    want = volume(ev, cyl) - volume(ev, taken)
                    self.assertGreater(volume(ev, taken), 0.0, "the pocket takes material")
                self.assertAlmostEqual(volume(evaluate(doc), combined), want, delta=1e-9)

    def test_a_face_to_trace_with_no_axis_is_drawn_and_refuses_as_its_trace(self):
        """The chart is read off the face drawn on, never off a traced
        face, so tracing the top cap (a plane, no axis) is drawn; its
        evaluation refuses as the trace, the bottom cap's plane missing
        the top."""
        doc = Doc()
        cyl = cylinder(doc)
        bottom, _ = bottom_and_wall(doc, cyl)
        ev = evaluate(doc)
        top = max(caps(ev, cyl), key=lambda f: ev.face_frame(cyl, f).origin[2].meters)
        sketch = doc.draw_on_face(cyl, bottom, footprint(), traced=[(cyl, top)])
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(sketch)
        self.assertEqual((caught.exception.kind, caught.exception.inner_kind), ("trace", "no_trace"))

    def test_a_crossing_the_piece_does_not_make_refuses_as_a_trace(self):
        doc = Doc()
        cyl = cylinder(doc)
        frame, wall = frame_and_wall(doc, cyl)
        sketch = doc.insert(Node.drawn_profile(frame, [(cyl, wall)], footprint("leave", 2)))
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(sketch)
        self.assertEqual(caught.exception.kind, "trace")
        self.assertEqual(caught.exception.inner_kind, "no_such_crossing")

    def test_an_argument_of_the_wrong_dimension_names_its_loop(self):
        """The façade checks each argument at its slot, at its loop and
        step: an angle written for the second loop's start refuses as the
        insert door would, naming loop 1."""
        doc = Doc()
        cyl = cylinder(doc)
        frame, wall = frame_and_wall(doc, cyl)
        hole = Chain.at(0 * m, 1 * deg).line_to(Start)
        with self.assertRaises(EditError) as caught:
            Node.drawn_profile(frame, [(cyl, wall)], [footprint(), hole])
        refusal = caught.exception
        self.assertEqual(refusal.variant, "slot_dimension_mismatch")
        self.assertEqual(refusal.expected, "length")
        self.assertEqual(refusal.found, "angle")
        self.assertIn("loop 1", str(refusal))

    def test_a_chain_drawn_on_a_face_beside_a_loop_is_checked_at_its_place(self):
        """The door takes closed loops and chains in one list, and checks
        each chain's arguments at its own place in it: an angle written
        for a chain's start refuses naming that loop, first, middle or
        last, and nothing lands."""
        hole = Chain.at(0 * m, 1 * deg).line_to(Start)
        for at in range(3):
            with self.subTest(at=at):
                doc = Doc()
                cyl = cylinder(doc)
                bottom, wall = bottom_and_wall(doc, cyl)
                outline = [footprint(), circle((0.5 * m, 0 * m), 0.05 * m), footprint()]
                outline[at] = hole
                before = doc.node_count
                with self.assertRaises(EditError) as caught:
                    doc.draw_on_face(cyl, bottom, outline, traced=[(cyl, wall)])
                refusal = caught.exception
                self.assertEqual(refusal.variant, "slot_dimension_mismatch")
                named = [i for i in range(3) if f"loop {i}" in str(refusal)]
                self.assertEqual(named, [at], str(refusal))
                self.assertEqual(doc.node_count, before)

    def test_an_outline_item_that_is_neither_kind_names_both(self):
        doc = Doc()
        cyl = cylinder(doc)
        bottom, _ = bottom_and_wall(doc, cyl)
        with self.assertRaisesRegex(TypeError, r"outline\[1\] is a str, neither a ClosedLoop nor a Chain"):
            doc.draw_on_face(cyl, bottom, [footprint(), "a loop"])
        with self.assertRaisesRegex(TypeError, "neither a ClosedLoop, a Chain nor a list of them"):
            doc.draw_on_face(cyl, bottom, 1.0)

    def test_the_words_a_chain_takes(self):
        with self.assertRaises(ValueError):
            TraceTarget.on_trace(0, "across")
        with self.assertRaises(ValueError):
            Chain.at(0 * m, 0 * m).along_trace("sideways")
        self.assertEqual(len(footprint()), 7)


if __name__ == "__main__":
    unittest.main()
