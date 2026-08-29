"""Legal authoring, for `ty` to accept.

This is the guide's PATHS section and the worked journey, typed. It is
never executed — `tests/test_ty.py` hands it to the type checker, and
the equivalent runtime assertions live in `tests/test_paths.py` and in
the guide's own executed blocks.
"""

from pncad import (
    Advisory,
    Alignment,
    ArcSweep,
    Assembly,
    AxisSense,
    Bulge,
    BooleanOp,
    CapEnd,
    Center,
    CheckId,
    CheckKind,
    ChecksConfig,
    ChecksReport,
    Cmp,
    ContactClass,
    ContentPin,
    CurveKind,
    CheckFinding,
    ClassAdmission,
    ClusterMaintenance,
    Denotation,
    Doc,
    DocEdit,
    DocRef,
    InlineOutcome,
    InterfaceRecord,
    EntityKind,
    Evaluation,
    FlushFinding,
    FlushRung,
    Frame,
    GeomPred,
    Length,
    Body,
    MateFault,
    MateFrame,
    MatePrimitive,
    MateRole,
    NamePat,
    Node,
    NodeId,
    Pose,
    PinMultiplicity,
    ParamName,
    PatternKind,
    SolvedPoses,
    SplitOutcome,
    Via,
    Open,
    PlaneRelation,
    SegPat,
    SegTag,
    Selector,
    Severity,
    SketchPlane,
    Start,
    SurfaceKind,
    Workspace,
    assemble,
    canonical_bytes,
    circle,
    class_admission,
    clusters,
    content_pin,
    deg,
    enforce_checks,
    evaluate,
    gauge_of,
    header_document_id,
    inline,
    m,
    mixed_pins,
    mm,
    product,
    product_named,
    random_document_id,
    reading_edges,
    relative_freedom_components,
    run_checks,
    solve_document,
    split,
    subject_body,
    update_references,
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

# Chamfer by NAME: the fillet's twin, and the SETBACK is a Length too.
chamfered: NodeId = doc.insert(Node.chamfer(upright, 0.05 * m, blend_edges))

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
narrowed_blend: NodeId = doc.insert(Node.fillet(lightened, 0.01 * m, straight))

# The structural half: role-path shape, sides, sub-name prefixes, the
# union — and `matches` on a materialized name, the binding reading
# the text so your code never does.
top_rim: Selector = Selector.of(
    NamePat.of_kind(EntityKind.Edge).seg(SegPat.tag(SegTag.RimEdge).side(CapEnd.Top))
)
from_a: NamePat = NamePat.any().seg(
    SegPat.tag(SegTag.Seam).of([NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap))])
)
both: Selector = top_rim.or_(from_a)
structural: list[str] = ev.select(lightened, both)
hit: bool = both.matches(blend_edges[0])

# The decided atom: a datum-relative position rule, its comparand a
# Length, its comparison the sign trilean.
near_cutter: GeomPred = GeomPred.datum_distance(cutter, Cmp.Approx, 0 * m)
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

# LIB-PYG5: the detect/declare protocol, typed end to end. Findings
# are values; the declare doors consume THEM, not name text; the id
# feeds the boolean's declare= input.
findings: list[FlushFinding] = ev.find_flush_candidates(plate, lightened)
first_relation: PlaneRelation = findings[0].relation
first_class: ContactClass = findings[0].class_
first_rung: FlushRung = findings[0].rung
opaque_a: str = findings[0].a
opaque_b: str = findings[0].b
decl_one: NodeId = doc.declare(findings[0])
decl_many: NodeId = doc.declare_all(findings)
decl_node: NodeId = doc.insert(Node.declare(findings))
glued: NodeId = doc.insert(
    Node.boolean(BooleanOp.Union, plate, lightened, declare=decl_many)
)

# LIB-PYPU: the group boolean and its placement vocabulary. Lengths
# and angles are typed; the count is a plain int (the structural-slot
# exception); a frame reads back as the dimensioned triple it took.
here: Frame = Frame.translation((0 * m, 0 * m, 0 * m))
turned: Frame = Frame.rotate_then_translate((0.0, 0.0, 1.0), 90 * deg, (1 * m, 0 * m, 0 * m))
aimed: Frame = Frame.point_at((0 * m, 0 * m, 0 * m), (0 * m, 0 * m, 1 * m), (0.0, 1.0, 0.0))
swept: Frame = Frame.path_start_frame((0 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0))
flipped: Frame = Frame.mirror_across_plane((0 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0))
frame_origin: tuple[Length, Length, Length] = here.origin
frame_det: float = here.determinant

stepped: PatternKind = PatternKind.linear((1.0, 0.0, 0.0), 0.5 * m)
spin_axis: NodeId = doc.insert(Node.datum_axis((0 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0)))
around: PatternKind = PatternKind.circular(spin_axis, 90 * deg)
listed: PatternKind = PatternKind.explicit([here, turned])

fin_group: NodeId = doc.insert(Node.placed_union(plate, 5, stepped))
listed_group: NodeId = doc.insert(Node.placed_union_at(plate, [here, turned]))
count_bound: DocEdit = DocEdit.bind_count_param(fin_group, ParamName("fins"))

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
# instance, an edit places its cluster, a mate says how two instances
# meet, and the gate says whether the result is valid at rest.
instance: NodeId = doc.insert(Node.instantiate_part(reference))
placed: DocEdit = DocEdit.set_placement(instance, here)
designated: DocEdit = DocEdit.set_roots([instance])
repinned: DocEdit = DocEdit.update_reference(instance, pin)
product_roots: list[NodeId] = doc.roots
cluster_frame: Frame = doc.placement(instance)
registry: dict[NodeId, Frame] = doc.placements()
carried: DocRef | None = doc.reference(instance)
seam_record: InterfaceRecord | None = doc.interface(instance)
after_edit: list[ClusterMaintenance] = doc.last_maintenance

# The authored mate datum: two frames, a primitive, an axis sense, and
# an optional clocking rider.
side_a: MateFrame = MateFrame((0 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0), (1.0, 0.0, 0.0))
side_b: MateFrame = MateFrame((0 * m, 0 * m, 1 * m), (0.0, 0.0, 1.0), (1.0, 0.0, 0.0))
seated: MatePrimitive = MatePrimitive.planar_rest(0 * m)
datum: Alignment = Alignment(side_a, side_b, seated, AxisSense.Aligned)
clocked: Alignment = Alignment(
    side_a, side_b, MatePrimitive.coaxial(), AxisSense.Opposed, 90 * deg
)
arm: Length = datum.lever_arm
seat_pose: Frame = side_a.placement()
joint: NodeId = doc.insert(
    Node.mate("a-name", "b-name", ContactClass.Rest, datum)
)

# The solve's read side, and the admission table a tool asks first.
poses: SolvedPoses = solve_document(doc)
gauge: NodeId | None = poses.gauge(instance)
role: MateRole | None = poses.role(joint)
refusal: MateFault | None = poses.fault(joint)
world: Frame = poses.placement(doc, instance)
groups: list[list[NodeId]] = clusters(doc)
keyed_by: NodeId = gauge_of(doc, instance)
edges: list[tuple[NodeId, NodeId]] = reading_edges(doc)
partition: list[list[NodeId]] = relative_freedom_components(doc)
admission: ClassAdmission = class_admission(ContactClass.Rest)
mintable: bool = admission.mints

# The gather and the gate.
gathered: Body = product(doc, seamed)
named_gather: tuple[Body, list[str]] = product_named(doc, seamed)
checked: Assembly = assemble(doc, seamed)
declared: int = len(checked.minted)

# The refactorings and the pin-update door.
cut: SplitOutcome = split(doc, [instance], minted)
spliced: InlineOutcome = inline(cut.remainder, cut.instance, store)
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
# The advisory checks: a report out of one door, a gate the caller
# opens at the other, and the subject a finding names.
report: ChecksReport = run_checks(doc, seamed)
strict: ChecksConfig = ChecksConfig(
    connectedness=Severity.Error,
    expected_components=[(instance, 0, 2)],
    separation=Advisory.Warn,
)
findings: list[CheckFinding] = run_checks(doc, seamed, strict).findings
label: CheckKind = CheckId.Connectedness.kind
enforce_checks(report, strict)
flagged: Body | None = subject_body(seamed, instance, 0)
