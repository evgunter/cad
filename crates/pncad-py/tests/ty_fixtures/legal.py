"""Legal authoring, for `ty` to accept.

This is the guide's PATHS section and the worked journey, typed. It is
never executed — `tests/test_ty.py` hands it to the type checker, and
the equivalent runtime assertions live in `tests/test_paths.py` and in
the guide's own executed blocks.
"""

from typing import Literal

from pncad import (
    Var,
    IndexedRead,
    MeasurePrimitive,
    Measured,
    Measurement,
    MassMeasurement,
    Settle,
    AssertionRelation,
    Advisory,
    AnalysisPolicy,
    AnalyzedBox,
    AnalyzedParam,
    analyzed_box,
    McAssertion,
    McConfig,
    McMeasure,
    McReport,
    monte_carlo,
    sample_offset,
    Coord,
    Angle,
    AngleUnit,
    ArcSweep,
    AtRestEvidence,
    Bulge,
    ClosedLoop,
    CancelToken,
    CapEnd,
    Center,
    CheckId,
    CheckKind,
    ChecksConfig,
    ChecksReport,
    Cmp,
    ContentPin,
    CurveKind,
    CheckFinding,
    Maintenance,
    FreeVar,
    Denotation,
    Distribution,
    HitTestError,
    Formula,
    TubeWindow,
    Doc,
    DocEdit,
    EditError,
    PersistError,
    StlError,
    DocRef,
    FlattenOutcome,
    InlineOutcome,
    InterfaceRecord,
    EntityKind,
    Evaluation,
    EvaluationError,
    Frame,
    FrameError,
    GeomPred,
    Length,
    LengthUnit,
    Body,
    CurvePromotion,
    FaceCensus,
    ImportReport,
    MassProperties,
    PlacedInstance,
    StructureNormalization,
    Mesh,
    MateFault,
    MateValue,
    Placed,
    Mate,
    MateRef,
    ConstraintId,
    NamePat,
    MeridianEnd,
    Node,
    PoseDef,
    NodeId,
    NodePick,
    NodePickError,
    PickHit,
    Pose,
    Resolution,
    PinMultiplicity,
    VarName,
    Recognised,
    Value,
    VarDecl,
    SolvedPoses,
    SplitOutcome,
    Via,
    Open,
    SegPat,
    SegTag,
    Ray,
    Selector,
    Severity,
    SketchedOnFace,
    Start,
    AuthoredStep,
    Piece,
    Role,
    RoleRead,
    StepId,
    SurfaceKind,
    Workspace,
    StationMove,
    InterferenceFinding,
    CarriedCoincidence,
    CarriedInterference,
    canonical_bytes,
    circle,
    content_pin,
    deg,
    rad,
    enforce_checks,
    band,
    band_pi,
    band_rim,
    band_rim_pi,
    carried,
    evaluate,
    meridian_vertex,
    import_step,
    load,
    header_document_id,
    flatten,
    inline,
    m,
    mixed_pins,
    mm,
    WrittenAngle,
    WrittenLength,
    pi_rad,
    product,
    product_named,
    random_document_id,
    relative_freedom_components,
    run_checks,
    solve_document,
    split,
    subject_body,
    update_references,
    ValidationError,
    ValidationFinding,
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
    .arc_to(Via((-5 * mm, 10 * mm), (0 * mm, 5 * mm)))
    .arc_to(Center((0 * mm, 0 * mm), ArcSweep.Ccw, (5 * mm, 0 * mm)))
    .arc_to(Bulge((-5 * mm, 0 * mm), 0.5))
    .line_to(Start)
)

disc = circle((0 * mm, 0 * mm), 10 * mm)

doc = Doc()
plate = doc.insert(Node.extrude(doc.insert(Node.profile(rounded)), Formula.length_in(8, mm)))
# A construction lies on its own xy and goes up its own z; where it
# stands beside another body is a placement into that body's space,
# by values on the space's axes.
hole_body = doc.insert(Node.extrude(doc.insert(Node.profile(disc)), Formula.length_in(10, mm)))
plate_space: Var | None = doc.space(plate)
assert plate_space is not None
hole: NodeId = doc.place(hole_body, plate_space, Placed.unmated([MateValue(Coord.slide("z"), -1 * mm)]))
lightened = doc.insert(Node.subtract(plate, hole))
volume: float = (
    evaluate(doc).value(lightened).body().mass_properties(Settle.resolution(1.024e-6 * m)).enclosure.volume
)

# A profile is 2-D: its points are length pairs on its own xy, and a
# world placement stands the construction where it goes, by values.
upright: NodeId = doc.insert(
    Node.extrude(
        doc.insert(
            Node.polygon(
                [
                    (Formula.length_in(0, m), Formula.length_in(0, m)),
                    (Formula.length_in(1, m), Formula.length_in(0, m)),
                    (Formula.length_in(1, m), Formula.length_in(1, m)),
                ],
            )
        ),
        Formula.length_in(2, m),
    )
)
stood: NodeId = doc.place(
    upright, doc.world, Placed.unmated([MateValue(Coord.spin("y"), Formula.turn_over(4))])
)
sideways: NodeId = doc.insert(Node.extrude(doc.insert(Node.profile(circle((0 * m, 0 * m), 1 * m))), Formula.length_in(4, m)))

# A revolve's axis is written in the profile's own two coordinates: an
# origin pair of lengths and a bare direction pair, on the node itself.
turned: NodeId = doc.insert(
    Node.revolve(
        doc.insert(Node.profile(circle((3 * m, 0 * m), 1 * m))),
        (Formula.length_in(0, m), Formula.length_in(0, m)),
        Formula.turn_over(4),
        Formula.angle_in(360, deg),
    )
)

# A plane-distance rule: a face of the selected body, named by its
# text, and a stated length off it.
near_here: GeomPred = GeomPred.plane_distance(evaluate(doc).all_faces(upright)[0], Cmp.Less, Formula.length_in(1, m))

# A three-section loft: the first section and each later one with its
# station, a chain of moves in the first section's own axes; the
# v-degree a plain int (a Count, structurally).
def square() -> NodeId:
    return doc.insert(Node.polygon([
        (Formula.length_in(0, m), Formula.length_in(0, m)),
        (Formula.length_in(1, m), Formula.length_in(0, m)),
        (Formula.length_in(1, m), Formula.length_in(1, m)),
        (Formula.length_in(0, m), Formula.length_in(1, m)),
    ]))


first_section: NodeId = square()
stations: list[tuple[NodeId, list[StationMove]]] = [
    (square(), [StationMove.slide("z", Formula.length_in(z, m))])
    for z in (1, 2)
]
stations.append(
    (square(), [StationMove.slide("z", 3 * m), StationMove.spin("z", 15 * deg)])
)
skinned = doc.insert(Node.loft(first_section, stations, Formula.count(2)))
station: StationMove = stations[0][1][0]
station_kind: str = station.kind
station_axis: str = station.axis
# A deciding predicate's answer: why it stopped, and what it decided.
_skinned: MassMeasurement = evaluate(doc).value(skinned).body().mass_properties(Settle.sign())
skinned_stop: Literal["settled", "met", "floor", "wall", "band"] = _skinned.stop
skinned_verdict: Literal["outer", "void"] | bool | None = _skinned.verdict
skinned_volume: float = _skinned.enclosure.volume
_asked: list[Settle] = [
    Settle.at_most(1.0),
    Settle.at_least(0.0),
    Settle.contains(0.0, 1.0),
]

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
                ],
            )
        ),
        Formula.length_in(1, m),
    )
)

# Fillet by NAME: the materializer answers as of this evaluation, the
# selection is stored, and from then on it is frozen.
blend_edges: list[str] = evaluate(doc).all_edges(upright)
blended: NodeId = doc.insert(Node.fillet(upright, Formula.length_in(0.05, m), blend_edges))

# Chamfer by NAME: the fillet's twin, and the SETBACK is a Length too.
chamfered: NodeId = doc.insert(Node.chamfer(upright, Formula.length_in(0.05, m), blend_edges))
open_faces: list[str] = evaluate(doc).all_faces(upright)[:1]
hollowed: NodeId = doc.insert(Node.shell(upright, Formula.length_in(0.01, m), open_faces))
sealed: NodeId = doc.insert(Node.shell(upright, Formula.length_in(0.01, m), []))

# The tube pair. The window is a VALUE with two spellings, and the
# hollow kind's wall is a required Length — there is no `wall=None`
# that quietly makes it the solid door.
donut: NodeId = doc.insert(
    Node.tube(Formula.length_in(0.2, m), TubeWindow.full(), Formula.length_in(0.05, m))
)
elbow: NodeId = doc.insert(
    Node.hollow_tube(
        Formula.length_in(0.2, m),
        TubeWindow.arc(Formula.angle_in(0, rad), Formula.angle_in(1.5, rad)),
        Formula.length_in(0.05, m),
        Formula.length_in(0.01, m),
    )
)

# Split by a plane read off geometry; the value is a split, read as
# two optional bodies rather than one.
cutter: PoseDef = PoseDef.plane(plate_with_holes, evaluate(doc).all_faces(plate_with_holes)[0])
halves = evaluate(doc).value(doc.insert(Node.split(plate_with_holes, cutter))).split()

# A pose defined at the seat that reads it (D10): a face read as its
# plane, slid along its outward normal, and a frame read off an edge
# and a vertex, slid along its own z and read through its xy plane. Every constructor answers a `PoseDef`,
# a pose seat takes one or a `RoleRead`, and an operand takes one.
_plate_face: str = evaluate(doc).all_faces(plate)[0]
face_plane: PoseDef = PoseDef.plane(plate, _plate_face)
inset: PoseDef = PoseDef.slide(
    face_plane, RoleRead(face_plane, "normal"), Formula.length_in(-0.01, m)
)
flipped: PoseDef = PoseDef.flip(inset)
authored_frame: PoseDef = PoseDef.through(
    PoseDef.axis(plate, evaluate(doc).all_edges(plate)[0]),
    PoseDef.point(plate, evaluate(doc).all_vertices(plate)[0]),
)
framed: PoseDef = PoseDef.slide(authored_frame, RoleRead(authored_frame, "z"), 0.5 * m)
projected: RoleRead = RoleRead(framed, "xy")
crossing: PoseDef = PoseDef.meet(face_plane, projected)
turned: PoseDef = PoseDef.spin(face_plane, RoleRead(framed, "x"), 15 * deg)
corner: PoseDef = PoseDef.point_in(authored_frame, 0 * m, 0.5 * m, 0 * m)
quarter: Formula = Formula.turn_over(4)
nothing: Formula = Formula.zero("length")
through_edge: PoseDef = PoseDef.through(
    PoseDef.axis(plate, evaluate(doc).all_edges(plate)[0]),
    PoseDef.point(plate, evaluate(doc).all_vertices(plate)[0]),
)
pose_cut: NodeId = doc.insert(Node.split(plate, inset))

# A rigid placement: a spin about the world's z, then a slide — the
# values on the world's axes, written whole.
placed: NodeId = doc.place(
    plate_with_holes,
    doc.world,
    Placed.unmated([MateValue(Coord.spin("z"), 90 * deg), MateValue(Coord.slide("z"), 2 * m)]),
)

# LIB-PYSEL: the selector language, typed. The diecomposed filters —
# the box edges by carrier kind, the pip rims by adjacent-surface
# pair — feeding the fillet's selection with no name text read.
edges: Selector = Selector.of(NamePat.of_kind(EntityKind.Edge))
ev = evaluate(doc)
straight: list[str] = ev.select_where(
    lightened, edges, [GeomPred.curve_kind(CurveKind.Line)]
)
rims: list[str] = ev.select_where(
    lightened,
    edges,
    [GeomPred.adjacent_kinds(SurfaceKind.Plane, [SurfaceKind.Sphere, SurfaceKind.Torus])],
)
narrowed_blend: NodeId = doc.insert(Node.fillet(lightened, Formula.length_in(0.01, m), straight))

# The structural half: role-path shape, sides, sub-name arguments, the
# union — and `matches` on a materialized name, the binding reading
# the text so your code never does.
top_rim: Selector = Selector.of(
    NamePat.of_kind(EntityKind.Edge).seg(SegPat.tag(SegTag.RimEdge).side(CapEnd.End))
)
cap_side: NamePat = NamePat.any().seg(
    SegPat.tag(SegTag.Seam).of([NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap))])
)
both: Selector = top_rim.or_(cap_side)
structural: list[str] = ev.select(lightened, both)
hit: bool = both.matches(blend_edges[0])

# The decided atom: a plane-relative position rule, its comparand a
# Length, its comparison the sign trilean.
near_cutter: GeomPred = GeomPred.plane_distance(structural[0], Cmp.Approx, Formula.length_in(0, m))
low_faces: list[str] = ev.select_where(
    lightened,
    Selector.of(NamePat.of_kind(EntityKind.Face)),
    [GeomPred.surface_kind(SurfaceKind.Plane), near_cutter],
)

# The fused verb: the entry side rides an arc carrier the verb itself
# authors, and the arrival is the ordinary straight pair.
carrier_corner = (
    Open.arc_fillet(Center((0 * m, 0 * m), ArcSweep.Ccw, (5 * m, 0 * m)), 0.5 * m)
    .at((0 * m, 3 * m))
    .toward(-1.0, 0.0)
    .line(3 * m)
    .line_to(Start)
)
carrier_corner_vertices: int = carrier_corner.vertex_count

# The arrival mode decides what the chain becomes: a `Center` anchored
# at `Start` CLOSES, so this expression is a loop, not a tip.
lens = Open.arc_fillet_arc(
    Center((-0.5 * m, 0 * m), ArcSweep.Ccw, (0 * m, -0.866 * m)),
    0.25 * m,
    Center((0.5 * m, 0 * m), ArcSweep.Ccw, Start),
)
lens_vertices: int = lens.vertex_count

# The seam's declared tangent arrival closes like bare `Start`, on
# every closer that takes it.
declared_d: ClosedLoop = (
    Open.at((0 * m, 0 * m))
    .angle(90 * deg)
    .line(2 * m)
    .arc_to(Bulge((0 * m, -2 * m), 1.0))
    .line_to(Start.arrives_tangent())
)
declared_bulge: ClosedLoop = (
    Open.at((0 * m, 0 * m))
    .line_to((1 * m, 0 * m))
    .arc_to(Bulge(Start.arrives_tangent(), 1.0))
)
declared_via: ClosedLoop = (
    Open.at((0 * m, 0 * m))
    .line_to((1 * m, 0 * m))
    .arc_to(Via((1 * m, 1 * m), Start.arrives_tangent()))
)
declared_center: ClosedLoop = (
    Open.at((0 * m, 0 * m))
    .line_to((1 * m, 0 * m))
    .arc_to(Center((0 * m, 1 * m), ArcSweep.Ccw, Start.arrives_tangent()))
)
declared_via_arrival: ClosedLoop = (
    Open.at((0 * m, 0 * m))
    .toward(1.0, 0.0)
    .line(2 * m)
    .toward(-1.0, 1.0)
    .fillet_arc(0.2 * m, Via((0 * m, 4 * m), Start.arrives_tangent()))
)
onto_landed: ClosedLoop = (
    Open.at((0 * m, 0 * m))
    .line_to((2 * m, 0 * m))
    .arc_onto((3 * m, 2 * m))
    .toward(0.0, 1.0)
    .tangent()
    .line(1 * m)
    .line_to(Start)
)
onto_sharp_seam: ClosedLoop = (
    Open.at((0 * m, 0 * m))
    .line_to((2 * m, 0 * m))
    .arc_onto(Start)
    .angle(90 * deg)
)
onto_declared: ClosedLoop = (
    Open.at((0 * m, 0 * m)).line_to((1 * m, 0 * m)).arc_onto(Start.arrives_tangent())
)
declared_stadium: ClosedLoop = (
    Open.at((0 * m, 0 * m))
    .angle(0 * deg)
    .line(2 * m)
    .tangent()
    .tangent_arc_to(Start.arrives_tangent())
)

# LIB-PYPU: the group boolean and its placement vocabulary. Lengths
# and angles are typed; the count is a plain int (the structural-slot
# exception); a frame reads back as the dimensioned triple it took.
here: Frame = Frame.translation((0 * m, 0 * m, 0 * m))
turned: Frame = Frame.rotate_then_translate((0.0, 0.0, 1.0), 90 * deg, (1 * m, 0 * m, 0 * m))
aimed: Frame = Frame.point_at((0 * m, 0 * m, 0 * m), (0 * m, 0 * m, 1 * m), (0.0, 1.0, 0.0))
swept: Frame = Frame.path_start_frame((0 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0))
frame_origin: tuple[Length, Length, Length] = here.origin
frame_det: float = here.determinant

spin_axis: PoseDef = PoseDef.axis(plate, evaluate(doc).all_edges(plate)[0])

# D10 `[repetition]`: a pattern is an index variable read by one
# placement's values, so its copy is a family; one member is
# `family[i]` (`family[i, j]` for a grid), the family whole a union's
# one members argument.
index_k: DocEdit = DocEdit.declare_var(VarName("k"), VarDecl.index(5))
index_j: DocEdit = DocEdit.declare_var(VarName("j"), VarDecl.index(Formula.count(2), within=VarName("k")))
index_count: Formula | None = VarDecl.index(5).count
family: NodeId = doc.insert(Node.linear_pattern(VarName("k"), plate, doc.world, [("x", Formula.length_in(0.5, m))]))
grid: Node = Node.grid((VarName("k"), "x", 0.5 * m), (VarName("j"), "y", 0.5 * m), plate, doc.world)
ring: Node = Node.circular_pattern(doc, VarName("k"), plate, doc.world, (spin_axis, spin_axis, "z"), step=90 * deg)
bolts: Node = Node.bolt_circle(doc, VarName("k"), 5, plate, doc.world, (spin_axis, spin_axis, "z"))
seen: Recognised | None = doc.recognise(family)
grid_member: IndexedRead = family[1, 2]
members_of: list[tuple[list[int], Value]] = evaluate(doc).value(family).members()
one_copy: NodeId = doc.place(family[2], doc.world, Placed.identity())
# A union or an intersect takes a list of operands or ONE family read
# whole; a subtract names its two operands, `from_` (`from` is a
# keyword) and `tool`.
whole_family: Node = Node.union(family)
common: Node = Node.intersect([plate, one_copy])
named_cut: Node = Node.subtract(from_=plate, tool=hole)
# One member of a family is `family[i]`, taken at every body seat: in a
# spelled list, at a subtract's seats, at a split's target.
members: IndexedRead = family[2]
spelled_family: Node = Node.union([family[0], family[1], family[2]])
member_cut: Node = Node.subtract(from_=family[1], tool=hole)
member_split: Node = Node.split(family[1], cutter)
cut: NodeId = doc.insert(Node.split(plate, spin_axis))
upper_half: Var | None = doc.output(cut, 0)

# LIB-G15: the workspace store. Identity crosses as the canonical hex
# text, the pin as a value, and a reference as the pair of them.
pin: ContentPin = content_pin(doc)
pin_text: str = pin.hex
same_pin: ContentPin = ContentPin(pin_text)
reference: DocRef = DocRef(doc.id, pin)
reference_id: str = reference.id
reference_pin: ContentPin = reference.pin
minted: str = random_document_id()
preimage: bytes = canonical_bytes(doc)
scanned: str = header_document_id(doc.save())

store: Workspace = Workspace("parts")
store_root: str = store.root
listing: dict[str, str] = store.documents()
written: str = store.create(doc)
rewritten: str = store.resave(doc)
saved_at: str = store.save_at(doc, "part.pncad")
forked: tuple[str, str] = store.save_as_new_document(doc)
resolved: Doc = store.resolve(reference)
current: ContentPin = store.current_pin(doc.id)
held: int = len(store)

# LIB-G18a: the document seam and the memo, `evaluate`'s two keyword
# doors. A store is passed AS the resolver — it IS one — and a prior
# evaluation is the memo, whose reuse the two counters make readable.
seamed: Evaluation = evaluate(doc, resolver=store)
memoized: Evaluation = evaluate(doc, prior=seamed)
both: Evaluation = evaluate(doc, resolver=store, prior=memoized)
reused_nodes: int = both.reused
recomputed_nodes: int = both.recomputed
crossings: int = both.part_evaluations

# LIB-G18b: the assembly authoring vocabulary. A reference becomes an
# instance, a bundle of constraints places each copy, a mate says how
# two copies meet, and the gate says whether the result is valid at
# rest.
instance: NodeId = doc.insert(Node.instantiate_part(reference, doc.world))
gesture: Placed = Placed.identity()
gesture_values: list[MateValue] = gesture.values
gesture_entries: list[Mate | MateValue] = gesture.bundle()
aside: NodeId = doc.new_space(label="aside")
aside_instance: Node = Node.instantiate_part(reference, aside)
space_node: Node = Node.space()
in_space: Var | None = doc.space(instance)
repinned: DocEdit = DocEdit.update_reference(instance, pin)
placed: NodeId = doc.place(sideways, doc.world, gesture, label="the post")
pair: NodeId = doc.assemble([sideways, sideways])
twice: NodeId = doc.place(pair, doc.world)
doc.set_product(placed)
the_product: Var | None = doc.product
world: list[NodeId] = doc.placements()
the_world: NodeId = doc.world
held: list[tuple[MateRef, Mate | MateValue]] = doc.bundle(placed)
pin_ref: MateRef = held[0][0]
pin_id: ConstraintId = pin_ref.constraint
pin_holder: NodeId = pin_ref.placement
unpinned: DocEdit = DocEdit.remove_constraint(pin_ref)
carried_reference: DocRef | None = doc.reference(instance)
seam_record: InterfaceRecord | None = doc.interface(instance)
after_edit: list[Maintenance] = doc.last_maintenance

# The authored mate: two poses of one kind, each read off geometry,
# and the values that set what the mates leave free.
floor: PoseDef = PoseDef.flip(PoseDef.plane(plate, "c-name"))
mate: Mate = Mate(PoseDef.plane(instance, "a-name"), floor)
between: Mate = Mate(
    PoseDef.plane(instance, "a-name"), PoseDef.flip(PoseDef.plane(sideways, "b-name"))
)
mate_on: PoseDef = mate.on
mate_to: PoseDef = mate.to
pinned_on: DocEdit = DocEdit.add_constraint(instance, mate)
placer: Node = Node.place(
    sideways, doc.world, [mate, MateValue(Coord.slide("x"), 0.1 * m)]
)
rewritten: list[MateRef] = doc.set_constraints(instance, [mate, MateValue(Coord.spin(), 0)])
rewrite: DocEdit = DocEdit.set_constraints(instance, [mate])
bundled: Placed = Placed.of(doc, [mate], [MateValue(Coord.slide("x"), 0.1 * m)])
rebundled: DocEdit = DocEdit.set_constraints(instance, bundled)
unmated: Placed = Placed.unmated([MateValue(Coord.spin("x"), Formula.turn_over(4))])
joint: MateRef = doc.constrain(instance, mate)
freed: list[MateRef] = doc.remove_mate(joint)
set_value: MateValue = MateValue(Coord.slide("y"), 0)
value_coord: str = set_value.coord.variant

# The solve's read side, and the admission table a tool asks first.
poses: SolvedPoses = solve_document(doc)
refusal: MateFault | None = poses.fault(instance)
faulted_mate: MateRef | None = refusal.mate if refusal is not None else None
relative: Frame | None = poses.pose(instance)
world_pose: Frame = poses.placement(doc, instance)
partition: list[list[NodeId]] = relative_freedom_components(doc)

# The gather.
gathered: Body = product(doc, seamed)
named_gather: tuple[Body, list[str]] = product_named(doc, seamed)

# The refactorings and the pin-update door.
cut: SplitOutcome = split(doc, [instance], minted)
spliced: InlineOutcome = inline(cut.remainder, cut.instance, store)
inlined_at: NodeId = spliced.placement
inlined_into: Var = spliced.space
flattened: FlattenOutcome = flatten(spliced.doc, spliced.placement)
lint: list[PinMultiplicity] = mixed_pins(doc)
moves: list[DocEdit] = update_references(doc, doc.id, pin)
to_store: list[DocEdit] = store.update_to_store(doc, doc.id)

# The read-back doors: a name in, VALUES out. The origin is
# dimensioned and the directions are not — a position carries a
# `Length`, a direction is a bare triple — and `u_ref` is OPTIONAL
# because a carrier that fixes no reference direction says so.
cap_name: str = seamed.all_faces(upright)[0]
where: Pose = seamed.face_frame(upright, cap_name)
sits_at: tuple[Length, Length, Length] = where.origin
normal: tuple[float, float, float] = where.axis
clocking: tuple[float, float, float] | None = where.u_ref
edge_pose: Pose = seamed.edge_frame(upright, seamed.all_edges(upright)[0])
corner: tuple[Length, Length, Length] = seamed.vertex_position(
    upright, seamed.all_vertices(upright)[0]
)
denotes: Denotation = seamed.denotation(upright, cap_name)
tied: bool = denotes.tied
# The face's ORIENTATION SENSE rides beside the axis as a plain bool,
# never folded into it, so the outward normal is formed by the reader.
faces_along_axis: bool = where.sense
outward: tuple[float, float, float] = (
    where.axis if where.sense else (-where.axis[0], -where.axis[1], -where.axis[2])
)
# The carrier-kind read: a face name in, the stored tag out — a
# `SurfaceKind`, the same enum `GeomPred.surface_kind` matches on, and
# not a string.
carrier: SurfaceKind = seamed.face_carrier_kind(upright, cap_name)
is_flat: bool = carrier == SurfaceKind.Plane
# A sketch on that face: the body, an opaque name, the outline, the
# depth and a DIMENSIONED spin, written as one action that answers its
# four nodes.
on_the_face: SketchedOnFace = doc.sketch_on_face(
    upright, cap_name, circle((0 * m, 0 * m), 0.1 * m), 0.1 * m, Formula.angle_in(0.3, rad), label="boss"
)
boss_profile: NodeId = on_the_face.profile
boss_extrude: NodeId = on_the_face.extrude
boss_placement: NodeId = on_the_face.placement
boss_union: NodeId = on_the_face.union
on_the_face_level: SketchedOnFace = doc.sketch_on_face(upright, cap_name, circle((0 * m, 0 * m), 0.1 * m), 0.1 * m)
# The advisory checks: a report out of one door, a gate the caller
# opens at the other, and the subject a finding names.
report: ChecksReport = run_checks(doc, seamed)
strict: ChecksConfig = ChecksConfig(
    connectedness=Severity.Error,
    expected_components=[(instance, 0, 2)],
    unproven_coincidence=Advisory.Warn,
    at_rest=Severity.Error,
)
findings: list[CheckFinding] = run_checks(doc, seamed, strict).findings
label: CheckKind = CheckId.Connectedness.kind
# What the at-rest check reports, read off a finding: the census it is
# under, the assertion that quiets it, and the evidence's payloads.
at_rest: list[CheckFinding] = [f for f in findings if f.check == CheckId.AtRest]
census: Var | None = at_rest[0].census
quieted_by: NodeId | None = at_rest[0].quiet
overlap: InterferenceFinding | None = at_rest[0].evidence.interference
carried_overlap: CarriedInterference | None = at_rest[0].evidence.carried_interference
carried_glue: CarriedCoincidence | None = at_rest[0].evidence.carried_coincidence
could_not_look: AtRestEvidence | None = at_rest[0].evidence.at_rest
why: str | None = could_not_look.inner_variant if could_not_look is not None else None
enforce_checks(report, strict)
flagged: Body | None = subject_body(seamed, instance, 0)

# Picking, the fourth door onto a name. A ray's origin is a POSITION
# and carries `Length`s; its direction is dimensionless and is a bare
# triple — the same split `Pose` draws. `t` is neither: it is in units
# of the ray's own direction, so it is a float, and `point` is where
# the dimensioned answer lives.
index: NodePick = NodePick.build(seamed, upright, 0, 1 * mm)
every_body: list[NodePick] = NodePick.build_all(seamed, upright, 1 * mm)
aimed: Ray = Ray((0 * mm, 0 * mm, 10 * mm), (0.0, 0.0, -1.0))
from_here: tuple[Length, Length, Length] = aimed.origin
along: tuple[float, float, float] = aimed.direction
struck: PickHit | None = seamed.pick_face([index], aimed)
drawn: Mesh = index.mesh
# The model's edges, in the position-index alphabet the triangles
# speak — one polyline per edge, paired entry for entry with
# `boundary_names` below.
wireframe: list[list[int]] = drawn.boundaries
paired_with: NodeId = index.node
which_body: int = index.body
# The per-slot inversion: a name, or the loud arm as a VALUE in the
# slot it concerns.
per_patch: list[str | HitTestError] = index.patch_names(seamed)
per_edge: list[str | HitTestError] = index.boundary_names(seamed)
# The pick refusal's payload, every attribute present and each typed.
# `patch`, `triangle` and `index` are the index arm's three numbers —
# `None` on every other arm, which is a value the stub types and not a
# missing attribute.
try:
    NodePick.build(seamed, upright, 99, 1 * mm)
except NodePickError as pick_refusal:
    which_pick_arm: str = pick_refusal.variant
    which_index_arm: str | None = pick_refusal.index_variant
    bad_patch: int | None = pick_refusal.patch
    bad_triangle: int | None = pick_refusal.triangle
    bad_position: int | None = pick_refusal.index

# Name resolution across re-evaluation. The verdict is a VALUE — a
# name that no longer denotes is an answer, not a raise — so every
# attribute is optional and the caller reads them after branching on
# `status`, which is always a `str`.
stored_name: str = seamed.all_faces(upright)[0]
standing: Resolution = seamed.resolve(upright, stored_name)
state: str = standing.status
which_arm: str | None = standing.variant
carried_by: NodeId | None = standing.node
in_body: int | None = standing.body
denotes: EntityKind | None = standing.kind
why: str | None = standing.detail
suggested: list[str] | None = standing.offers
# The expression READ side: text in through the document that declares
# the parameters, a dimension-checked tree out, and a DIMENSIONED
# value back. `eval` answers the quantity the expression measures, so
# the union is narrowed by what the caller knows about the source.
derived: Formula = doc.parse_formula("1 m + 2 mm")
reads_back: str = derived.text
measures: str = derived.dimension
depends_on: list[VarName] = derived.params
bare: float | None = derived.literal_value
worth: Length | Angle | float = doc.eval(derived)
# LIB-EDITS: the two edits addressed the way a refusal answers — a
# slot by its own WORD, a name by its own TEXT. Neither takes a class
# of its own: the alphabet a caller writes at is the alphabet
# `EditError.slot` reads back, and a stable name is opaque text on
# this surface everywhere else too.
slot_moved: DocEdit = DocEdit.set_param(plate, "distance", derived)
repaired: DocEdit = DocEdit.rebind(open_faces[0], open_faces[0])
how_many: int = doc.eval_count(doc.parse_formula("4"))
# The display formatter: TEXT out, in the unit asked for, from the
# quantity that carries the dimension. The sibling `in_unit` answers a
# float and is the door this one exists beside — the pair, typed, is
# what makes the difference between them legible at a glance.
shown: str = (25 * mm).format(mm)
turned: str = (90 * deg).format(pi_rad)
magnitude: float = (25 * mm).in_unit(mm)
# Cooperative cancellation: the token in, a PARTIAL evaluation out.
# The two `canceled` are different questions with the same answer
# type — the token's says the flag is set, the run's says a run
# observed it — and the run remains an ordinary `Evaluation`, because
# a stop is a partial answer and not a refusal.
stop: CancelToken = CancelToken()
stop.cancel()
flagged: bool = stop.canceled
stopped: Evaluation = evaluate(doc, cancel=stop)
was_stopped: bool = stopped.canceled
still_ordered: list[NodeId] = stopped.order()

# The validator ladder, all four rungs. Each answers NOTHING and
# raises on failure, which is what makes `-> None` the honest return:
# a verdict a caller could forget to read would be the wrong shape for
# a gate. The fourth takes no arguments either, because the contacts
# it certifies against ride with the body.
gathered.validate()
gathered.validate_closed()
gathered.validate_geometric()
gathered.validate_pseudomanifold()

# The node-kind read door: an id in, one stable word out. It is the
# NODE's kind and not its value's, so it is answerable with no
# evaluation in hand at all.
which_kind: str = doc.node_kind(upright)

# Parameter uncertainty and the analysis lane. The offsets are typed
# quantities in the parameter's own dimension, so the annotation and
# the declaration agree by construction here and a mismatch is a
# refusal at the door.
spread: Distribution = Distribution.normal(1 * mm)
window: Distribution = Distribution.truncated_normal(1 * mm, -2 * mm, 2 * mm)
declared_form: str = spread.kind
annotated: FreeVar = FreeVar.length(4 * mm, spread)
unannotated: FreeVar = FreeVar.length(4 * mm)
carried_distribution: Distribution | None = annotated.distribution
read_back: FreeVar | None = doc.params.get(VarName("bore_r"))

# The box is derived on request from a document and a policy, and the
# policy is optional because the ±3σ convention is the default.
policy: AnalysisPolicy = AnalysisPolicy(0.99)
boxed: AnalyzedBox = analyzed_box(doc, policy)
default_boxed: AnalyzedBox = analyzed_box(doc)
one_axis: AnalyzedParam | None = boxed.get(VarName("bore_r"))
axis_names: list[VarName] = boxed.names

# Both mass columns answer `None` for a name the document does not
# declare, so the caller's variable is optional whichever way it goes.
tail: float | None = boxed.tail_mass(VarName("bore_r"))
leaf: float | None = boxed.box_mass(VarName("bore_r"), -1 * mm, 1 * mm)

# The advisory lane. The config is optional because the shipped dials
# are the kernel's; the box is not, because which parameters vary and
# what counts as outside is the analysis's knob and never a default
# hidden inside the run.
dials: McConfig = McConfig(samples=64, seed=7, parallel=False)
estimate: McReport = monte_carlo(doc, boxed, dials)
shipped: McReport = monte_carlo(doc, boxed)
per_measure: list[McMeasure] = estimate.measures
per_assertion: list[McAssertion] = estimate.assertions
labeled: str = estimate.render()
# The fraction is over the DECIDED samples, so it is optional: a run
# that decided none has no fraction rather than a zero.
fraction: float | None = per_assertion[0].violation_fraction if per_assertion else None
# One draw. The offset carries the distribution's own dimension, so a
# Length annotation answers a Length.
drawn: Length | Angle | float = sample_offset(VarName("bore_r"), spread, 0.5)
# Authored notation: the value and the unit it was WRITTEN in, kept
# together. `in_unit` multiplies (`25 * mm` that remembers the `mm`);
# `canonical_in` takes a quantity whose arithmetic has already
# happened and says which notation to record it in. The unit reads
# back as the typed unit, and the parameter as its symbol.
thickness: WrittenLength = WrittenLength.in_unit(25.0, mm)
computed: WrittenLength = WrittenLength.canonical_in((20 * mm) + (5 * mm), mm)
plain: Length = thickness.length
notation: LengthUnit = thickness.unit
turned: WrittenAngle = WrittenAngle.in_unit(90.0, deg)
turn_notation: AngleUnit = turned.unit
declared: FreeVar = FreeVar.written_length(thickness)
spun: FreeVar = FreeVar.written_angle(turned)
symbol: str | None = declared.unit
table: dict[VarName, FreeVar] = doc.params

# Authoring a measurement. The verb vocabulary is a value class over
# its two references — each a node and a name, the pair `Node.mate`
# already takes each of its two sides as — and its arithmetic is an
# ordinary formula over the measures' outputs.
reach: MeasurePrimitive = MeasurePrimitive.distance((upright, cap_name), (upright, cap_name))
which_verb: str = reach.verb
which_pair: tuple[tuple[NodeId, str] | PoseDef | Var, tuple[NodeId, str] | PoseDef | Var] = reach.refs
measured_kind: str = reach.dimension
recorded: Measured = doc.measure([reach])
spans: list[NodeId] = recorded.measures
spanned: list[Var] = recorded.outputs
lone: NodeId = doc.insert(Node.measure(reach))
# The bound is an EXPRESSION, because its dimension is the value's and
# a slot address cannot fix it.
requirement: NodeId = doc.insert(
    Node.assertion(recorded.outputs[0], AssertionRelation.AtLeast, doc.parse_formula("0.5 mm"))
)
read_back: Measurement = evaluate(doc).reading(recorded.outputs[0])
which_way: str = AssertionRelation.AtMost.symbol
equality: NodeId = doc.insert(
    Node.assertion(
        recorded.outputs[0], relation=AssertionRelation.Equal, bound=doc.parse_formula("0.5 mm")
    )
)

# The two words a refusal carries, typed. Both are OPTIONAL strings and
# the stub says so: `kind` is which door refused, `inner_kind` the arm
# of the refusal that door holds, and a caller that has not narrowed
# either is holding `str | None`.
try:
    evaluate(doc).value(upright)
except EvaluationError as node_refusal:
    which_door: str | None = node_refusal.kind
    which_arm: str | None = node_refusal.inner_kind
    if which_arm is not None:
        narrowed: str = which_arm
try:
    doc.apply(DocEdit.delete_node(upright))
except EditError as edit_refusal:
    which_edit: str = edit_refusal.variant
    which_edit_arm: str | None = edit_refusal.inner_variant
    # ...and the arm's PAYLOAD beside them, every attribute present and
    # each typed. An attribute the arm does not carry is `None`, which
    # is a value the stub types and not a missing attribute.
    dangling: NodeId | None = edit_refusal.node
    operand: NodeId | None = edit_refusal.input
    which_slot: str | None = edit_refusal.slot
    which_param: str | None = edit_refusal.param
    which_name: str | None = edit_refusal.name
    which_key: str | None = edit_refusal.key
    wanted_dim: str | None = edit_refusal.expected
    offered_dim: str | None = edit_refusal.found
    which_attr: str | None = edit_refusal.kind
    rebound_from: EntityKind | None = edit_refusal.from_kind
    rebound_to: EntityKind | None = edit_refusal.to_kind
    members: int | None = edit_refusal.count
    first_at: int | None = edit_refusal.first
    again_at: int | None = edit_refusal.again
    refused_eps: float | None = edit_refusal.value
    refused_value: float | int | None = edit_refusal.offered
    address: tuple[int, ...] | None = edit_refusal.path
    inside_meta: str | None = edit_refusal.value_path
    already: ContentPin | None = edit_refusal.pin
    if dangling is not None:
        narrowed_node: NodeId = dangling

# The one refusal on this surface whose discriminant is a SEQUENCE.
# `findings` is a list, its length is `failure_count`, and each entry's
# `variant` is a plain `str` while its five payload words are optional
# — the shape a caller reads without narrowing on `variant` first.
try:
    gathered.validate_pseudomanifold()
except ValidationError as validation_refusal:
    which_rung: str = validation_refusal.door
    how_many: int = validation_refusal.failure_count
    every_finding: list[ValidationFinding] = validation_refusal.findings
    first_finding: ValidationFinding = every_finding[0]
    which_arm_failed: str = first_finding.variant
    about: str | None = first_finding.subject_kind
    carrier: str | None = first_finding.entity_kind
    coincidence: str | None = first_finding.contact_kind
    unwitnessed: str | None = first_finding.stale_kind
    where_the_ring_meets: str | None = first_finding.ring_contact_kind
    if coincidence is not None:
        declarable: str = coincidence
    if unwitnessed is not None:
        withdraw: str = unwitnessed
    if where_the_ring_meets is not None:
        move_the_ring: str = where_the_ring_meets


# The three doors LIB-DOORS-2 projected, typed. Every payload
# attribute is `Optional[...]`, so a caller that has not narrowed one
# is holding `X | None`; only the discriminant itself is a plain
# `str`.
try:
    load("id: 00000000000000000000000000000000\n{}")
except PersistError as persist_refusal:
    which_stage: str = persist_refusal.variant
    which_inner: str | None = persist_refusal.inner_variant
    where: str | None = persist_refusal.site
    which_param: str | None = persist_refusal.name
    reporter_said: str | None = persist_refusal.detail
    at_line: int | None = persist_refusal.line
    recorded_epsilon: float | None = persist_refusal.document
    if at_line is not None:
        line_number: int = at_line

try:
    Frame.path_start_frame((0 * m, 0 * m, 0 * m), (0.0, 0.0, 0.0))
except FrameError as frame_refusal:
    which_input: str = frame_refusal.variant
    which_band_arm: str | None = frame_refusal.inner_variant
    in_band: float | None = frame_refusal.margin
    lower: float | None = frame_refusal.margin_low
    coincidence_at: float | None = frame_refusal.zero
    escalation_at: float | None = frame_refusal.escalate
    deciding: str | None = frame_refusal.predicate
    if in_band is not None:
        margin_value: float = in_band

try:
    gathered.tessellate(1 * mm).to_stl_binary(header="x" * 81)
except StlError as stl_refusal:
    which_stl_arm: str = stl_refusal.variant
    facet: tuple[int, int, int] | None = stl_refusal.triangle
    offending_char: str | None = stl_refusal.character
    header_bytes: int | None = stl_refusal.len
    sink_said: str | None = stl_refusal.detail
    if header_bytes is not None:
        how_long: int = header_bytes


# The import report, read attribute by attribute. Every field of the
# importer's success value is here, and the reads that MUST be narrowed
# are exactly the ones the record leaves absent: a normalization's
# promoted kind and residual, and the four entity fields of an assembly
# row a file that places nothing does not state.
_report: ImportReport = import_step(
    evaluate(doc).step_string(lightened, product_name="plate")
)
imported_body: Body = _report.body
# The gate's own certificate, measured — continuing its rounds rather
# than running the quadrature a second time.
enclosure: MassProperties = _report.measure(Settle.resolution(1.024e-6 * m)).enclosure
imported_volume: float = enclosure.volume
imported_area: float = enclosure.surface_area
imported_volume_pad: float = enclosure.volume_pad
imported_area_pad: float = enclosure.area_pad
file_tolerance: float = _report.eps_in

for _normalization in _report.normalizations:
    _n: StructureNormalization = _normalization
    remint_face: int = _n.face
    remint_kind: str = _n.kind
    promoted_to: str | None = _n.promoted_to
    promotion_residual: float | None = _n.residual
    stated: FaceCensus = _n.file_census
    minted: FaceCensus = _n.kernel_census
    face_delta: int = minted.faces - stated.faces
    edge_delta: int = minted.edges - stated.edges
    vertex_delta: int = minted.vertices - stated.vertices
    if promotion_residual is not None:
        how_far: float = promotion_residual

for _promotion in _report.promotions:
    _p: CurvePromotion = _promotion
    carrier: int = _p.curve
    carrier_kind: str = _p.kind
    carrier_residual: float = _p.residual

for _instance in _report.instances:
    _i: PlacedInstance = _instance
    solid_index: int = _i.index
    msb: int = _i.solid
    component: int = _i.component
    occurrence: int | None = _i.occurrence
    relationship: int | None = _i.relationship
    transform: int | None = _i.transform
    placement: Frame | None = _i.placement
    if placement is not None:
        placed_at: tuple[Length, Length, Length] = placement.origin


# The five role-name doors: a name MINTED from a role, answering the
# same `str` a materializer answers, and handed to the two doors that
# take an authored selection. Nothing here evaluates anything — that
# is what the doors are for.
_names_doc = Doc()
_names_profile: NodeId = _names_doc.insert(
    Node.profile(circle((2 * m, 1 * m), 0.5 * m))
)
_revolved: NodeId = _names_doc.insert(
    Node.revolve(
        _names_profile,
        (Formula.length_in(0, m), Formula.length_in(0, m)),
        Formula.turn_over(4),
        Formula.angle_in(360, deg),
    )
)
# A piece is spelled from the handle its authoring call returned, in
# the loop the author states; `Doc.pieces` is the positional reading.
_pieces: list[list[Piece]] = _names_doc.pieces(_names_profile)
_step_ids: list[list[StepId]] = _names_doc.step_ids(_names_profile)
_carrier: AuthoredStep = circle((2 * m, 1 * m), 0.5 * m).step
_first: Piece = _names_doc.piece(_names_profile, 0, _carrier.piece(0))
_second: Piece = Piece(_names_doc.step(_names_profile, 0, _carrier), Role.piece(1))
minted_band: str = band(_revolved, _first)
minted_half: str = band_pi(_revolved, _pieces[0][0])
minted_rim: str = band_rim(_revolved, _second)
minted_rim_half: str = band_rim_pi(_revolved, _second)
minted_vertex: str = meridian_vertex(MeridianEnd.Seam, _revolved, _second)
_revolved_read = _names_doc.output(_revolved, 0)
assert _revolved_read is not None
minted_survivor: str = carried(_revolved, _revolved_read, minted_band)
_blended: NodeId = _names_doc.insert(
    Node.fillet(_revolved, Formula.length_in(0.1, m), [minted_rim])
)
_hollowed: NodeId = _names_doc.insert(
    Node.shell(_revolved, Formula.length_in(0.1, m), [minted_band, minted_half])
)

# INTENT-LITERALS Q9: a slot takes a variable, a formula, or a value of
# the slot's own dimension — written, or bare in its canonical unit.
q9_solid: NodeId = plate
Node.extrude(q9_solid, 1 * m)
Node.revolve(q9_solid, (0 * m, 0 * m), Formula.turn_over(4), 90 * deg)
Node.fillet(q9_solid, 1 * mm, [])
Node.loft(q9_solid, [(q9_solid, [StationMove.slide("z", 1 * m)])], 2)
DocEdit.set_param(q9_solid, "distance", 1 * m)
held: Var | None = doc.slot(q9_solid, "distance")
if held is not None:
    Node.extrude(q9_solid, held)
