"""Legal authoring, for `ty` to accept.

This is the guide's PATHS section and the worked journey, typed. It is
never executed — `tests/test_ty.py` hands it to the type checker, and
the equivalent runtime assertions live in `tests/test_paths.py` and in
the guide's own executed blocks.
"""

from pncad import ArcSweep, BooleanOp, Doc, Node, Open, Start, circle, deg, evaluate, mm

outline = (
    Open.at((0 * mm, 0 * mm))
    .line_to((80 * mm, 0 * mm))
    .line_to((80 * mm, 40 * mm))
    .line_to((0 * mm, 40 * mm))
    .line_to(Start)
)
rectangle_vertices: int = outline.vertex_count

rounded = (
    Open.at((0 * mm, 0 * mm))
    .line_to((40 * mm, 0 * mm))
    .toward(0.0, 1.0)
    .fillet(6 * mm)
    .toward(-1.0, 0.0)
    .to((0 * mm, 30 * mm))
    .line_to(Start)
)

# The angle-first entry, the incoming tangent, and the arc modes.
walked = (
    Open.angle(0 * deg)
    .at((0 * mm, 0 * mm))
    .line(20 * mm)
    .turn(90 * deg)
    .line(10 * mm)
    .tangent()
    .tangent_arc_to((0 * mm, 20 * mm))
    .arc_via((-5 * mm, 10 * mm), (0 * mm, 5 * mm))
    .arc_center((0 * mm, 0 * mm), (5 * mm, 0 * mm), ArcSweep.Ccw)
    .line_to(Start)
)

disc = circle((0 * mm, 0 * mm), 10 * mm)

doc = Doc()
plate = doc.insert(Node.extrude(doc.insert(Node.profile(rounded)), 8 * mm))
hole = doc.insert(Node.extrude(doc.insert(Node.profile(disc, elevation=-1 * mm)), 10 * mm))
lightened = doc.insert(Node.boolean(BooleanOp.Subtract, plate, hole))
volume: float = evaluate(doc).value(lightened).body().mass_properties().volume
