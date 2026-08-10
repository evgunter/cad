"""Legal authoring, for `ty` to accept.

This is the guide's PATHS section and the worked journey, typed. It is
never executed — `tests/test_ty.py` hands it to the type checker, and
the equivalent runtime assertions live in `tests/test_paths.py` and in
the guide's own executed blocks.
"""

from pncad import (
    ArcSweep,
    BooleanOp,
    Doc,
    Node,
    NodeId,
    Open,
    SketchPlane,
    Start,
    circle,
    deg,
    evaluate,
    m,
    mm,
)

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

# The plane vocabulary: a named cyclic frame, and the general rigid
# frame that carries an origin. Extrusion runs along the plane's
# normal, so the plane is what chooses the axis.
upright: NodeId = doc.insert(
    Node.extrude(
        doc.insert(Node.polygon([(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m)], plane=SketchPlane.yz())),
        2 * m,
    )
)
offset_frame = SketchPlane.from_frame((0 * m, -0.5 * m, 0 * m), (0.0, 0.0, 1.0), (1.0, 0.0, 0.0))
sideways: NodeId = doc.insert(Node.extrude(doc.insert(Node.profile(circle((0 * m, 0 * m), 1 * m), plane=offset_frame)), 4 * m))

# A three-section loft: the sections are NodeIds in skin order, the
# v-degree a plain int (a Count, structurally), and each section's
# placement rides its own profile's plane.
sections: list[NodeId] = [
    doc.insert(Node.polygon([(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m), (0 * m, 1 * m)], elevation=z * m))
    for z in (0, 1, 2)
]
skinned = doc.insert(Node.loft(sections, 2))
skinned_volume: float = evaluate(doc).value(skinned).body().mass_properties().volume
