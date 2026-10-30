"""`Node.drawn_profile`: a sketch continues tangent to a surface's trace
(INTENT stage 6 C, D10 "Constructing a coincidence").

The Python mirror of `crates/editor-core/tests/intent_s6_c_trace.rs`: a
profile drawn on a frame of a cylinder's space runs up until it leaves
the cylinder's wall, then along the wall's trace, and its extrude stands
in the cylinder's space with its wall at the cylinder's radius. A step
whose trace refuses refuses at `evaluate` under the `trace` class.
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

    def test_the_traced_frame_stands_on_the_walls_axis_in_the_caps_plane(self):
        doc = Doc()
        cyl = cylinder(doc)
        _, wall = frame_and_wall(doc, cyl)
        ev = evaluate(doc)
        caps = [f for f in sorted(ev.all_faces(cyl)) if ev.face_carrier_kind(cyl, f) == SurfaceKind.Plane]
        bottom = min(caps, key=lambda f: ev.face_frame(cyl, f).origin[2].meters)
        frame = ev.traced_frame((cyl, bottom), (cyl, wall))
        sketch = doc.insert(Node.drawn_profile(frame, [(cyl, wall)], footprint()))
        boss = doc.insert(Node.extrude(sketch, 0.5 * m))
        ev = evaluate(doc)
        at = [ev.vertex_position(boss, v) for v in ev.all_vertices(boss)]
        self.assertAlmostEqual(max(abs(p[1].meters) for p in at), R, delta=1e-12)
        self.assertAlmostEqual(min(p[2].meters for p in at), 0.0, delta=1e-12)
        with self.assertRaisesRegex(ValueError, "axis"):
            ev.traced_frame((cyl, bottom), (cyl, bottom))

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

    def test_the_words_a_chain_takes(self):
        with self.assertRaises(ValueError):
            TraceTarget.on_trace(0, "across")
        with self.assertRaises(ValueError):
            Chain.at(0 * m, 0 * m).along_trace("sideways")
        self.assertEqual(len(footprint()), 7)


if __name__ == "__main__":
    unittest.main()
