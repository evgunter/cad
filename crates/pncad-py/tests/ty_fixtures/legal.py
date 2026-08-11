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
    Length,
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

# LIB-PYBUNDLE: a multi-loop profile is a LIST of closed loops — the
# outer boundary first, then the holes, in description order.
plate_with_holes: NodeId = doc.insert(
    Node.extrude(
        doc.insert(
            Node.profile(
                [
                    circle((0 * m, 0 * m), 3 * m),
                    circle((-1 * m, 0 * m), 0.5 * m),
                    circle((1 * m, 0 * m), 0.5 * m),
                ]
            )
        ),
        1 * m,
    )
)

# Fillet by NAME: the materializer answers as of this evaluation, the
# selection is stored, and from then on it is frozen.
blend_edges: list[str] = evaluate(doc).all_edges(upright)
blended: NodeId = doc.insert(Node.fillet(upright, 0.05 * m, blend_edges))

# Split by a datum plane; the value is a split, read as two optional
# bodies rather than one.
cutter: NodeId = doc.insert(Node.datum_plane((0 * m, 0 * m, 1 * m), (0.0, 0.0, 1.0)))
halves = evaluate(doc).value(doc.insert(Node.split(plate_with_holes, cutter))).split()

# A rigid placement: rotate about an axis through the world origin,
# then translate.
placed: NodeId = doc.insert(
    Node.transform(plate_with_holes, (0 * m, 0 * m, 2 * m), (0.0, 0.0, 1.0), 90 * deg)
)

# The plane's frame, read back; and the bit-exact equality the read-back
# supports.
plane_origin: tuple[Length, Length, Length] = offset_frame.origin
plane_normal: tuple[float, float, float] = offset_frame.normal
same_plane: bool = offset_frame == SketchPlane.from_frame(
    offset_frame.origin, offset_frame.u, offset_frame.v
)
