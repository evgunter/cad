"""Off-lattice authoring, for `ty` to REJECT.

The compile-fail analog: every line marked `# ty: error` must draw at
least one diagnostic. These are exactly the states PATHS-DESIGN §2
declares unrepresentable, plus the typed-quantity boundary.
"""

from pncad import (
    Settle,
    MeasurePrimitive,
    AssertionRelation,
    AnalysisPolicy,
    analyzed_box,
    McConfig,
    monte_carlo,
    sample_offset,
    ArcSide,
    ChecksConfig,
    Severity,
    Bulge,
    CancelToken,
    Cmp,
    CurveKind,
    Distribution,
    Doc,
    FreeVar,
    Formula,
    DocEdit,
    DocRef,
    EditError,
    PersistError,
    StlError,
    EntityKind,
    EvaluationError,
    ValidationError,
    ValidationFinding,
    Body,
    Frame,
    StationMove,
    FrameError,
    GeomPred,
    NamePat,
    CapEnd,
    Node,
    NodeId,
    NodePick,
    Ray,
    Open,
    VarName,
    VarDecl,
    SegPat,
    SegTag,
    Selector,
    Start,
    Piece,
    SurfaceKind,
    TubeWindow,
    Sweep,
    Workspace,
    enforce_checks,
    import_step,
    run_checks,
    subject_body,
    Mate,
    MateValue,
    PoseDef,
    circle,
    content_pin,
    deg,
    rad,
    band,
    carried,
    evaluate,
    meridian_vertex,
    load,
    m,
    mm,
    WrittenAngle,
    WrittenLength,
    product,
    solve_document,
    split,
    update_references,
)

# A second director on a tip whose angle slot is already full.
Open.at((0 * mm, 0 * mm)).angle(0 * deg).angle(90 * deg)  # ty: error

# `.tangent()` needs an incoming end tangent; a plain point has none.
Open.at((0 * mm, 0 * mm)).tangent()  # ty: error

# A leading fillet would author the seam from the front.
Open.fillet(1 * mm)  # ty: error

# No leg departs a half-bound tip.
Open.at((0 * mm, 0 * mm)).line(1 * mm)  # ty: error
Open.angle(0 * deg).line_to((1 * mm, 0 * mm))  # ty: error

# There is deliberately no `close()`; targeting Start is the mechanism.
Open.at((0 * mm, 0 * mm)).line_to(Start).close()  # ty: error

# A closed loop continues into nothing.
Open.at((0 * mm, 0 * mm)).line_to(Start).line_to(Start)  # ty: error

# `arc_onto` leaves a point tip, and its pending state takes one
# director and nothing else.
Open.at((0 * mm, 0 * mm)).angle(0 * deg).arc_onto((1 * mm, 1 * mm))  # ty: error
Open.at((0 * mm, 0 * mm)).arc_onto((1 * mm, 1 * mm)).tangent()  # ty: error
Open.at((0 * mm, 0 * mm)).arc_onto((1 * mm, 1 * mm)).line_to((2 * mm, 0 * mm))  # ty: error

# A bare number is not a Length, and radians are not a Length either.
Open.at((0.0, 0.0))  # ty: error
Open.at((0 * mm, 0 * mm)).angle(1 * mm)  # ty: error

# LIB-PYBUNDLE. A real id to hang the node doors off — the refusals
# below are about the ARGUMENT types, not about a missing name.
doc = Doc()
solid: NodeId = doc.insert(
    Node.extrude(
        doc.insert(Node.profile(circle((0 * m, 0 * m), 1 * m))),
        Formula.length_in(1, m),
    )
)

# A DIMENSIONED SLOT TAKES ITS OWN DIMENSION'S ROW (Q9): a variable, a
# `Formula`, or a value that measures what the slot measures — a
# `Length` or a `WrittenLength` at a length slot. A bare float is
# dimensionless, so it is not a length.
Node.extrude(solid, 1.0)  # ty: error

# `v_degree` is a Count: a count slot takes an `int` (Q9), and a float
# is not one.
Node.loft(solid, [], 2.5)  # ty: error

# A loft section's station is a chain of `StationMove`s, not a
# coordinate triple, and a slide's axis is one of the first section's
# own three, named.
Node.loft(solid, [(solid, (0 * m, 0 * m, 1 * m))], 2)  # ty: error
StationMove.slide("w", 1 * m)  # ty: error
StationMove.spin("z", 1 * m)  # ty: error

# An index reads one member of a family, which is a body: no seat but a
# body seat takes one, and an index is a count.
Node.extrude(solid[0], Formula.length_in(1, m))  # ty: error
solid[0.5]  # ty: error

# A fillet selection is NAMES — the text a materializer answered with,
# never node ids.
Node.fillet(solid, Formula.length_in(1, m), [solid])  # ty: error

# A blend radius is a length, not a bare number.
Node.fillet(solid, 1.0, [])  # ty: error

# The chamfer's setback is a length as well, and its selection is
# names — the twin holds the same two lines.
Node.chamfer(solid, 1.0, [])  # ty: error
Node.chamfer(solid, Formula.length_in(1, m), [solid])  # ty: error

# The shell's wall is a `Formula` too, and its open list is names as text.
Node.shell(solid, 1.0, [])  # ty: error
Node.shell(solid, Formula.length_in(0.01, m), [solid])  # ty: error

# A tube's radii are `Formula`s, its window is a `TubeWindow` and never a
# pair of raw angles, and the hollow kind's WALL IS REQUIRED — the
# three ways a caller reaches for the shape this vocabulary refuses to
# have.
Node.tube(solid, 0.2, TubeWindow.full(), Formula.length_in(0.05, m))  # ty: error
Node.tube(solid, Formula.length_in(0.2, m), (0 * rad, 1 * rad), Formula.length_in(0.05, m))  # ty: error
Node.hollow_tube(solid, Formula.length_in(0.2, m), TubeWindow.full(), Formula.length_in(0.05, m))  # ty: error

# A multi-loop profile is a list of LOOPS, and nothing else.
Node.profile([circle((0 * m, 0 * m), 1 * m), "hole"])  # ty: error

# LIB-PYSEL. The selector vocabulary refuses the same mixings Rust
# refuses at compile time. (There is no "decided where exact is
# required" row: the Rust list type mixes exact and decided atoms
# freely, so the stub mirrors what compiles.)

# A curve-kind set holds CURVE kinds; a surface-kind set holds
# SURFACE kinds — the two families do not stand in for each other.
GeomPred.curve_kind(SurfaceKind.Plane)  # ty: error
GeomPred.adjacent_kinds(CurveKind.Line, SurfaceKind.Sphere)  # ty: error

# A plane-distance comparand is a `Formula`: not a bare float, and not
# a quantity either. That it must be a LENGTH is the kernel's check at
# `select_where`, not the stub's.
GeomPred.plane_distance("a face", Cmp.Approx, 1.0)  # ty: error
GeomPred.plane_distance("a face", Cmp.Approx, 90 * deg)  # ty: error

# The comparison is the trilean value, not a string spelling of one.
GeomPred.plane_distance("a face", ">", Formula.length_in(1, m))  # ty: error

# The face is opaque name TEXT, never the `NodeId` of a body.
GeomPred.plane_distance(solid, Cmp.Approx, Formula.length_in(1, m))  # ty: error

# A selector unions NAME patterns; a segment pattern is not one, and
# `tag`/`group`/`side` take their own vocabularies.
Selector.of(SegPat.any())  # ty: error
SegPat.tag(EntityKind.Edge)  # ty: error
SegPat.group(SegTag.Cap)  # ty: error
SegPat.tag(SegTag.RimEdge).side("top")  # ty: error
NamePat.of_kind(SegTag.Cap)  # ty: error

# The geometric stage takes predicates, not kinds, and the selector
# argument is a Selector, not a bare pattern.
evaluate(doc).select_where(solid, Selector.of(NamePat.any()), [CurveKind.Line])  # ty: error
evaluate(doc).select(solid, NamePat.any())  # ty: error

# Patterns are immutable values: the builder verbs return NEW ones.
NamePat.any().kind = EntityKind.Edge  # ty: error

# The endpoint-FREE modes need a departure tangent to sweep about, so
# they are not among the modes a bare point admits: the pair is a
# missing row of the matrix, not a refusal.
Open.at((0 * mm, 0 * mm)).arc_to(Sweep(1 * mm, ArcSide.Left, 90 * deg))  # ty: error

# `Bulge` is chord-relative, so it is not an ARRIVAL mode: an arrival
# has no chord yet.
_open_fillet = Open.at((0 * mm, 0 * mm)).toward(1.0, 0.0)
_open_fillet.fillet_arc(1 * mm, Bulge((5 * mm, 5 * mm), 0.5))  # ty: error

# D10 `[repetition]`. A row's pitch is a length argument, not a bare
# number, and its axis is one of the target's three words.
Node.linear_pattern(VarName("k"), solid, solid, [("x", 0.5)])  # ty: error
Node.linear_pattern(VarName("k"), solid, solid, [("w", 0.5 * m)])  # ty: error

# An index names the variable by its `VarName`, never bare text.
Node.linear_pattern("k", solid, solid, [("x", 0.5 * m)])  # ty: error

# An index's count is a count, never a dimensioned quantity.
VarDecl.index(2 * m)  # ty: error

# A frame's translation reads back dimensioned, and it is READ-ONLY:
# the value is frozen.
Frame.translation((0 * m, 0 * m, 0 * m)).origin = (1 * m, 0 * m, 0 * m)  # ty: error

# The narrowed structural edit takes a VarName, never bare text, and
# the slot is named by the door, so there is no `slot=` to pass.
DocEdit.bind_v_degree_param(solid, "skin")  # ty: error
name: VarName = VarName("which")
DocEdit.bind_v_degree_param(solid, name, slot="v_degree")  # ty: error

# LIB-EDITS. The CONTINUOUS slot edit takes the slot's word and what a
# slot takes (Q9): its dimension is the word's, which the type cannot
# read, so a value of another dimension refuses at the edit door. And
# the word is TEXT: a slot is a name, so there is no slot type to pass
# and an index is not one either.
DocEdit.set_param(solid, 0, doc.parse_formula("1 m"))  # ty: error

# The name repair takes two NAMES — opaque text, as every other
# name-taking door on this surface does. A node is not one.
DocEdit.rebind(solid, solid)  # ty: error

# A reference is (identity, pin) in that order and neither is the
# other's type: an id is the canonical hex TEXT, a pin is a value.
DocRef(content_pin(doc), doc.id)  # ty: error

# The store resolves a reference, never a bare identity.
Workspace("parts").resolve(doc.id)  # ty: error

# The seam and the memo are KEYWORD-only: `evaluate` takes exactly one
# positional argument, the document, and the two doors are named.
evaluate(doc, Workspace("parts"))  # ty: error

# A resolver is a STORE, not the directory one was opened on: what
# resolves is the scanned object, and a path string has no scan. (The
# scan is not frozen at construction — a `create` through the same
# object is visible to a later `evaluate`; see the snapshot rows in
# `test_assembly_eval.py`. The point here is the TYPE, not timing.)
evaluate(doc, resolver="parts")  # ty: error

# The memo is a prior EVALUATION, not the document it evaluated.
evaluate(doc, prior=doc)  # ty: error

# Neither substitutes for the other: an evaluation resolves nothing.
evaluate(doc, resolver=evaluate(doc))  # ty: error

# LIB-G18b: the assembly vocabulary's own off-lattice lines.

# An instance names a REFERENCE — (id, pin) — never a bare identity.
# The pin is half the value, and that half is what Cargo.lock
# semantics live in.
Node.instantiate_part(doc.id, doc.world)  # ty: error

# A placement's target is a space, not a pose: where the copy stands
# in it is its bundle's.
doc.place(solid, Frame.translation((0 * m, 0 * m, 1 * m)))  # ty: error

# A placement's target is always written.
Node.place(solid, [])  # ty: error

# A bundle is its values, never a coordinate triple.
doc.place(solid, doc.world, (0 * m, 0 * m, 1 * m))  # ty: error

# A mate equates two POSES, each read off geometry. Name text where a
# pose goes names nothing.
Mate("a-name", PoseDef.plane(solid, "a-name"))  # ty: error

# Which freedom a value sets is a `Coord`, never its word.
MateValue("slide_x", 1 * m)  # ty: error

# The gather takes a document AND an evaluation of it: neither is
# derivable from the other, and the pair is the signature.
product(doc)  # ty: error
product(evaluate(doc), doc)  # ty: error

# The solve is a whole-DOCUMENT fold; there is no per-mate door.
solve_document(doc, solid)  # ty: error

# A split's cut is a SET OF NODES and its new identity is the
# canonical hex text; identity is never defaulted, so there is no
# one-argument form.
split(doc, solid, "fresh")  # ty: error

# `update_references` takes the pin as a VALUE. Passing its text would
# make the door do the parsing the type already did.
update_references(doc, doc.id, content_pin(doc).hex)  # ty: error

# A read-back door takes the NODE the name was minted against and the
# name's opaque text, in that order — a name alone does not say which
# evaluation it should be read on.
evaluate(doc).face_frame(solid)  # ty: error

# A pose's origin is three LENGTHS: reading it as bare floats is a
# dimension mistake.
pose = evaluate(doc).face_frame(solid, "a-name")
origin_floats: tuple[float, float, float] = pose.origin  # ty: error

# `vertex_position` answers a POSITION, not a pose — there is no frame
# at a point, and no `axis` to read off one.
no_axis: object = evaluate(doc).vertex_position(solid, "a-name").axis  # ty: error
# DS6's waiver rule is a TYPE here, not a comment asking callers not
# to reach: the unproven-coincidence lint ships no acknowledgment record
# for every row it reports, so its knob is an `Advisory` and `Error` is
# not a position it has.
ChecksConfig(unproven_coincidence=Severity.Error)  # ty: error

# The report door takes a document AND an evaluation of it, in that
# order; the gate door takes the finished REPORT. Neither is derivable
# from the other, and swapping them is the mistake the pair exists to
# catch.
run_checks(doc)  # ty: error
enforce_checks(doc)  # ty: error

# A subject is named by a node and an OUTPUT INDEX — the same
# attribution a finding carries — never by the finding's rendering.
subject_body(evaluate(doc), solid)  # ty: error

# A ray's ORIGIN is a position and carries `Length`s; writing it as
# bare floats is the same dimension mistake `Pose.origin` catches
# above, and the surface refuses to guess a unit.
Ray((0.0, 0.0, 10.0), (0.0, 0.0, -1.0))  # ty: error

# ...and its DIRECTION is dimensionless. Handing it lengths claims a
# direction has a unit, which is the mistake in the other direction.
Ray((0 * m, 0 * m, 10 * m), (0 * m, 0 * m, -1 * m))  # ty: error

# A pick index is built against the EVALUATION that minted the body,
# not against the document: a `(node, body)` pairing only means
# something as of one run, which is the whole reason this type exists.
NodePick.build(doc, solid, 0, 1 * mm)  # ty: error

# The chordal budget is a DISTANCE (δ), never a bare float — the same
# closed-set rule `Body.tessellate` states.
NodePick.build(evaluate(doc), solid, 0, 0.001)  # ty: error

# `pick_face` takes the pre-paired targets, not the node they were
# built for. There is no spelling of a raw target here, and that is
# what stops a confidently wrong name.
evaluate(doc).pick_face([solid], Ray((0 * m, 0 * m, 1 * m), (0.0, 0.0, -1.0)))  # ty: error

# A miss is `None`, so a hit is OPTIONAL: reading `.name` off the
# answer without asking whether there was one is the mistake the typed
# miss exists to make visible.
maybe_hit = evaluate(doc).pick_face([], Ray((0 * m, 0 * m, 1 * m), (0.0, 0.0, -1.0)))
hit_name: str = maybe_hit.name  # ty: error

# `resolve` reads a name in the variable holding its body: the read
# comes first and is never optional, so a name alone is refused.
evaluate(doc).resolve("a face")  # ty: error

# A name crosses as opaque TEXT, never as the `NodeId` that minted it.
evaluate(doc).resolve(solid, solid)  # ty: error

# The verdict is a value, and its location attributes are OPTIONAL
# because two of the three states have no location. Binding one to a
# bare `NodeId` claims a verdict always resolved, which is exactly the
# assumption the three states exist to stop.
where: NodeId = evaluate(doc).resolve("a face").node  # ty: error

# ...and the same on the failure half: `detail` is prose that is
# `None` on a resolved verdict, so it is not a `str`.
reason: str = evaluate(doc).resolve("a face").detail  # ty: error

# `variant` is the ARM under the state, and a resolved verdict has
# none — so binding it to a bare `str` makes the same claim `node`
# above does, one vocabulary over.
arm: str = evaluate(doc).resolve("a face").variant  # ty: error

# `offers` is a list of NAMES — opaque texts — not of parsed
# structures, and it is `None` where suggestions do not apply.
rebinds: list[str] = evaluate(doc).resolve("a face").offers  # ty: error

# The status is a stable tag STRING, not the kind enum: "which of the
# three states" and "what kind of entity" are different questions.
tag: EntityKind = evaluate(doc).resolve("a face").status  # ty: error

# A sketch on a face. The SPIN is an angle slot, so a bare number
# cannot mean radians by convention...
doc.sketch_on_face(solid, "a face", circle((0 * m, 0 * m), 1 * m), 1 * m, 0.3)  # ty: error

# ...and a length cannot stand for an angle.
doc.sketch_on_face(solid, "a face", circle((0 * m, 0 * m), 1 * m), 1 * m, 1 * m)  # ty: error

# The face is opaque TEXT, never the `NodeId` that minted it — the
# same confusion the read doors refuse, on the authoring side.
doc.sketch_on_face(solid, solid, circle((0 * m, 0 * m), 1 * m), 1 * m)  # ty: error

# The carrier-kind read answers the `SurfaceKind` enum, not the tag as
# a string: "which surface variant" is a value to compare, not prose
# to parse.
kind_text: str = evaluate(doc).face_carrier_kind(solid, "a face")  # ty: error

# ...and it is node-scoped like the frame doors, so it takes the node
# the name was minted for. Dropping it is the `resolve` confusion in
# the other direction.
evaluate(doc).face_carrier_kind("a face")  # ty: error

# The orientation sense is a BOOL beside the axis, never a signed
# direction: folding it in is exactly what the field exists to stop.
pose_here = evaluate(doc).face_frame(solid, "a face")
signed: tuple[float, float, float] = pose_here.sense  # ty: error
# The two evaluators are not interchangeable and neither takes text:
# an expression is a VALUE, built by the document that declares the
# parameters it references.
doc.eval("width / 2.0")  # ty: error
doc.eval_count("4")  # ty: error

# `eval` answers a quantity where the expression is dimensioned, so
# reading it as a bare float is the same dimension mistake the
# quantity boundary catches elsewhere.
plain: float = doc.eval(doc.parse_formula("1 m"))  # ty: error

# NOT here, deliberately: an expression is unhashable on purpose
# (equality is an IEEE comparison of the literals inside it), and `ty`
# does not check hashability of a set member, so the pin would be a
# comment wearing a marker. `tests/test_expressions.py` executes it.

# The display formatter takes the unit of the dimension it is a method
# ON. That is the whole reason it is a method: `quantity`'s free
# `fmt_length` takes a bare `f64` of metres, and a free binding of it
# would accept an angle's radians without complaint. The receiver
# carries the dimension, so the mis-pairing has no spelling.
(1 * m).format(deg)  # ty: error
(1 * rad).format(mm)  # ty: error

# It answers TEXT, not a number — the door beside it, `in_unit`, is
# the one that answers a float, and the difference between them is
# what the family closed.
digits: float = (1 * m).format(m)  # ty: error

# The cancel token is a keyword, and a keyword of its own TYPE: the
# three optional arguments of `evaluate` are not interchangeable, and
# a bare bool is not a handle onto a flag anybody else can set.
evaluate(doc, cancel=True)  # ty: error
evaluate(doc, CancelToken())  # ty: error
evaluate(doc, prior=CancelToken())  # ty: error

# The stop is ONE-WAY. `canceled` is a read-only property on both the
# token and the run — a settable one would let a caller un-observe a
# cancelation, which is the state neither object has.
CancelToken().canceled = False  # ty: error
evaluate(doc).canceled = False  # ty: error

# A canceled run is a partial `Evaluation`, not a boolean and not a
# raise: reading the run where the TOKEN was meant, or the other way
# round, is the confusion the two names invite and the types refuse.
finished: CancelToken = evaluate(doc, cancel=CancelToken())  # ty: error
# The fourth rung takes NO contacts argument. `ContactRecords` has no
# Python spelling at all — it is minted by the ops that certify
# geometry, never built by a caller — so the two shapes a reader might
# reach for from the Rust signature are both unspellable: there is no
# value to pass, and the door would not take one.
product(doc, evaluate(doc)).validate_pseudomanifold(doc)  # ty: error

# And it answers nothing. A rung that returned a verdict would be a
# gate a caller could pass without reading; every rung raises instead.
verdict: bool = product(doc, evaluate(doc)).validate_pseudomanifold()  # ty: error

# `node_kind` reads a document's node BY ID and answers TEXT. Both ends
# invite the same confusion, because `Node` and `NodeId` are two types
# one sentence apart: the constructor value is not a handle onto an
# inserted node, and the word that comes back is not the node.
doc.node_kind(Node.extrude(solid, Formula.length_in(1, m)))  # ty: error
which: Node = doc.node_kind(solid)  # ty: error

# A structural count is FIXED under any error analysis, so the count
# constructor takes no annotation — the one `FreeVar` door that does
# not, and the type is what says so rather than a runtime check.
FreeVar.count(4, Distribution.normal(1.0))  # ty: error

# A distribution is a frozen value: an annotation is restated by
# building a new one, never by editing the one a document handed back.
Distribution.normal(1 * mm).kind = "band"  # ty: error

# The mass doors are keyed by a `VarName`, not by its text — the
# same distinction `Doc.doc_param` draws, and the reason a name is a
# type here at all.
analyzed_box(doc).tail_mass("bore_r")  # ty: error

# The policy is the ANALYSIS's knob and takes a bare mass, not a
# quantity: a share of a distribution's mass is dimensionless.
AnalysisPolicy(1 * mm)  # ty: error

# The advisory run needs the BOX it is measured against — which
# parameters vary and what counts as outside is not something the
# document alone answers, so there is no one-argument spelling.
monte_carlo(doc)  # ty: error

# A sample count is a count and a seed is a seed: neither is a
# quantity, and the type is what says so rather than a runtime check.
McConfig(samples=1 * mm)  # ty: error
# A report is a frozen value: a run is restated by running again,
# never by editing the answer.
monte_carlo(doc, analyzed_box(doc)).samples = 4  # ty: error

# The draw door is keyed by a `VarName` like every other door in
# this vocabulary, and its quantile is a bare float in `[0, 1)` — a
# share of a law's mass is dimensionless.
sample_offset("bore_r", Distribution.normal(1 * mm), 0.5)  # ty: error
sample_offset(VarName("bore_r"), Distribution.normal(1 * mm), 1 * mm)  # ty: error

# Two refusals in one line, and both are the point. A name the
# document does not declare is not an axis, so `get` answers an
# OPTION that has to be narrowed; and an axis speaks its parameter's
# dimension, so its nominal is a quantity rather than a bare float.
nominal_as_float: float = analyzed_box(doc).get(VarName("h")).nominal  # ty: error
# THE MIS-DIMENSIONED WRITTEN VALUE IS UNREPRESENTABLE, not refused.
# A `WrittenLength` holds a LENGTH unit, so "a length written in
# degrees" is not a value the type can hold and no door has to refuse
# one — the illegal state is excluded one layer out, at the table.
WrittenLength.in_unit(25.0, deg)  # ty: error
WrittenAngle.in_unit(90.0, mm)  # ty: error
FreeVar.written_length(WrittenAngle.in_unit(90.0, deg))  # ty: error

# The authored pair has NO arithmetic: there is no answer to what
# notation the sum of a millimetre and an inch is written in. Compute
# on the `Length` inside instead.
WrittenLength.in_unit(25.0, mm) + WrittenLength.in_unit(1.0, mm)  # ty: error

# `canonical_in` takes the QUANTITY, not bare canonical metres — the
# crossing rule this whole boundary follows.
WrittenLength.canonical_in(0.025, mm)  # ty: error

# A DIRECTION IS NOT A BOUND, and a bound is not a direction. The two
# sit side by side on `Node.assertion` and the types are what keep the
# order from being a thing to remember.
_value = doc.output(solid)
assert _value is not None
Node.assertion(_value, doc.parse_formula("1 m"), AssertionRelation.AtLeast)  # ty: error
Node.assertion(_value, AssertionRelation.AtLeast, AssertionRelation.AtMost)  # ty: error

# A PRIMITIVE IS NOT A VALUE. It is what a measure is built FROM;
# handing it where the value it measures belongs confuses the two
# halves the measurement vocabulary keeps apart.
_span = MeasurePrimitive.distance((solid, "a face"), (solid, "another"))
Node.assertion(_span, AssertionRelation.AtLeast, doc.parse_formula("1 m"))  # ty: error
doc.insert(_span)  # ty: error

# A reference is a PAIR — the name alone does not say where its
# carrier is read, which is the half that makes a measure report
# placed geometry.
MeasurePrimitive.distance("a face", "another")  # ty: error

# The bound takes the expression door and not the quantity one: a
# typed length cannot be an angle bound, and the whole point of the
# `Formula` seat is that the dimension is the value's.
Node.assertion(_value, AssertionRelation.AtLeast, 1 * mm)  # ty: error

# The verb vocabulary is a frozen value: a primitive is restated by
# building a new one, never by editing one in place.
MeasurePrimitive.distance(0, 1).verb = "gap"  # ty: error

# The two refusal words are OPTIONAL strings. `inner_kind` is `None`
# wherever the refusal has no arms, so reading it as a `str` is the
# narrowing the caller has not done — the `resolve().variant` rule
# above, at the refusal carrier.
try:
    evaluate(doc).value(solid)
except EvaluationError as _refusal:
    _arm: str = _refusal.inner_kind  # ty: error

# ...and the same at the edit door, where the CARRIER's word is always
# a string and only the inner one is optional: the two are not
# interchangeable however alike they read.
try:
    doc.apply(DocEdit.delete_node(solid))
except EditError as _refused:
    _edit_arm: str = _refused.inner_variant  # ty: error
    # The arm's payload is optional for the same reason and reads the
    # same way: every attribute is present on every arm, so the one a
    # caller wants is `None` wherever the arm does not carry it and a
    # bare read has not narrowed anything. The node id is the payload's
    # most-reached attribute and the one worth pinning.
    _dangling: NodeId = _refused.node  # ty: error
    _which_slot: str = _refused.slot  # ty: error

# The validator's findings are a SEQUENCE, not a scalar word. Reading
# one as a `str` is the mistake the exception's shape exists to make
# impossible to hold quietly: one raise carries N failures, so there is
# no single word for it to be.
try:
    product(doc, evaluate(doc)).validate_pseudomanifold()
except ValidationError as _validation:
    _one_word: str = _validation.findings  # ty: error

# A finding's payload words are OPTIONAL: an arm that carries no census
# subject has `None` there, so reading one as a `str` is the narrowing
# the caller has not done — the `inner_kind` rule at the validator.
try:
    product(doc, evaluate(doc)).validate_pseudomanifold()
except ValidationError as _refusal_again:
    _subject: str = _refusal_again.findings[0].subject_kind  # ty: error

# And a finding is a frozen VALUE: it is restated by re-running the
# validator, never by editing one in place — the `MeasurePrimitive`
# rule at the refusal side of the surface.
try:
    product(doc, evaluate(doc)).validate_pseudomanifold()
except ValidationError as _frozen:
    _finding: ValidationFinding = _frozen.findings[0]
    _finding.variant = "something_else"  # ty: error


# The three projected doors' payloads are OPTIONAL on every arm — the
# whole point of "present on every arm, `None` where the arm does not
# carry one" is that reading one as its bare type is a narrowing the
# caller has not done.
try:
    load("id: 00000000000000000000000000000000\n{}")
except PersistError as _persist:
    _at_line: int = _persist.line  # ty: error

try:
    Frame.path_start_frame((0 * m, 0 * m, 0 * m), (0.0, 0.0, 0.0))
except FrameError as _frame:
    _margin: float = _frame.margin  # ty: error

try:
    product(doc, evaluate(doc)).tessellate(1 * mm).to_stl_binary(header="x" * 81)
except StlError as _stl:
    _header_bytes: int = _stl.len  # ty: error


# The import door answers a REPORT, not a body. The whole point of the
# value is that the body is one field of it beside the gate's own
# measurement, so a caller that treats the report as the handle is
# reaching past the field it wanted.
_import = import_step("ISO-10303-21;\nEND-ISO-10303-21;\n")
_as_body: Body = _import  # ty: error
_measured = _import.mass_properties(Settle.sign())  # ty: error

# And a report is FROZEN, like every other value on this surface: the
# import is restated by importing again, never by editing one in place.
_import.eps_in = 1.0  # ty: error

# The record's absent fields are `Optional` on every row — reading one
# as its bare type is a narrowing the caller has not done.
for _row in _import.normalizations:
    _residual: float = _row.residual  # ty: error
for _placed in _import.instances:
    _frame: Frame = _placed.placement  # ty: error


# A role-name door mints a name from a NODE and a profile `Piece`. A
# name is not a node, a piece is not an index nor its text, the piece
# is not optional, `carried`'s read is a `Var` and not a node, and the end vocabulary is closed: `MeridianEnd` names a
# revolve's ends and a `CapEnd` is an extrude's.
_names_doc = Doc()
_names_node: NodeId = _names_doc.insert(Node.profile(circle((0 * m, 0 * m), 1 * m)))
_names_read = _names_doc.output(_names_node, 0)
assert _names_read is not None
_piece: Piece = _names_doc.pieces(_names_node)[0][0]
_text: str = str(_piece)
_from_text: str = band(_names_node, _text)  # ty: error
_minted: str = band(_names_node, _piece)
_from_a_name: str = band(_minted, _piece)  # ty: error
_from_an_index: str = band(_names_node, 0)  # ty: error
_the_piece_is_not_optional: str = band(_names_node)  # ty: error
_wrong_vocabulary: str = meridian_vertex(CapEnd.End, _names_node, _piece)  # ty: error
_a_node_is_not_a_name: str = carried(_names_node, _names_read, _names_node)  # ty: error
_a_name_is_not_a_node: str = carried(_minted, _names_read, _minted)  # ty: error
_a_node_is_not_a_read: str = carried(_names_node, _names_node, _minted)  # ty: error
