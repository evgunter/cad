"""Off-lattice authoring, for `ty` to REJECT.

The compile-fail analog: every line marked `# ty: error` must draw at
least one diagnostic. These are exactly the states PATHS-DESIGN §2
declares unrepresentable, plus the typed-quantity boundary.
"""

from pncad import (
    ArcSide,
    Bulge,
    Cmp,
    CurveKind,
    Doc,
    DocEdit,
    EntityKind,
    Frame,
    GeomPred,
    NamePat,
    Node,
    NodeId,
    Open,
    PatternKind,
    SegPat,
    SegTag,
    Selector,
    SketchPlane,
    Start,
    SurfaceKind,
    Sweep,
    circle,
    deg,
    evaluate,
    m,
    mm,
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

# A bare number is not a Length, and radians are not a Length either.
Open.at((0.0, 0.0))  # ty: error
Open.at((0 * mm, 0 * mm)).angle(1 * mm)  # ty: error

# The sketch plane is a VALUE, not a name: there is no string spelling
# of "the yz plane" that the door would guess at.
Node.profile(Open.at((0 * m, 0 * m)).line_to(Start), plane="yz")  # ty: error

# A plane's frame is dimensionless directions and a dimensioned
# origin — not the other way round.
SketchPlane.from_frame((0 * m, 0 * m, 0 * m), (0 * m, 1 * m, 0 * m), (0.0, 0.0, 1.0))  # ty: error

# `v_degree` is a Count: a continuous quantity is not one, and neither
# is a float.
Node.loft([], 2.5)  # ty: error

# LIB-PYBUNDLE. A real id to hang the node doors off — the refusals
# below are about the ARGUMENT types, not about a missing name.
doc = Doc()
solid: NodeId = doc.insert(
    Node.extrude(doc.insert(Node.profile(circle((0 * m, 0 * m), 1 * m))), 1 * m)
)

# A fillet selection is NAMES — the text a materializer answered with,
# never node ids.
Node.fillet(solid, 1 * m, [solid])  # ty: error

# A blend radius is a Length, not a bare number.
Node.fillet(solid, 1.0, [])  # ty: error

# A transform's axis is dimensionless and its angle is an Angle; the
# two do not stand in for each other.
Node.transform(solid, (0 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0), 1 * m)  # ty: error

# A datum plane's normal is a dimensionless triple.
Node.datum_plane((0 * m, 0 * m, 0 * m), (0 * m, 0 * m, 1 * m))  # ty: error

# A multi-loop profile is a list of LOOPS, and nothing else.
Node.profile([circle((0 * m, 0 * m), 1 * m), "hole"])  # ty: error

# The plane's frame components are read-only projections, not slots.
SketchPlane.xy().origin = (0 * m, 0 * m, 0 * m)  # ty: error

# LIB-PYSEL. The selector vocabulary refuses the same mixings Rust
# refuses at compile time. (There is no "decided where exact is
# required" row: the Rust list type mixes exact and decided atoms
# freely, so the stub mirrors what compiles.)

# A curve-kind set holds CURVE kinds; a surface-kind set holds
# SURFACE kinds — the two families do not stand in for each other.
GeomPred.curve_kind(SurfaceKind.Plane)  # ty: error
GeomPred.adjacent_kinds(CurveKind.Line, SurfaceKind.Sphere)  # ty: error

# A datum-distance comparand is a Length: not a bare float, not an
# Angle — the dimension crosses as the type.
GeomPred.datum_distance(solid, Cmp.Approx, 1.0)  # ty: error
GeomPred.datum_distance(solid, Cmp.Approx, 90 * deg)  # ty: error

# The comparison is the trilean value, not a string spelling of one.
GeomPred.datum_distance(solid, ">", 1 * m)  # ty: error

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

# LIB-PYG5. The declare doors take FINDINGS — values from the
# detector — never name text or bare pairs; the detector takes node
# ids; a finding's fields are read-only projections of the report.
doc.declare_all(["some-name-text", "another"])  # ty: error
doc.declare("name-text")  # ty: error
Node.declare([("a", "b")])  # ty: error
evaluate(doc).find_flush_candidates(solid, "not-a-node")  # ty: error

# LIB-PYPU. A spacing is a Length, not a bare number: the typed
# quantity is the whole point of the boundary.
PatternKind.linear((1.0, 0.0, 0.0), 0.5)  # ty: error

# A rule is a PatternKind; a Frame is a placement, not a rule.
Node.placed_union(solid, 5, Frame.translation((0 * m, 0 * m, 0 * m)))  # ty: error

# The explicit door lists FRAMES, never raw coordinate triples.
Node.placed_union_at(solid, [(0 * m, 0 * m, 0 * m)])  # ty: error

# A frame's translation reads back dimensioned, and it is READ-ONLY:
# the value is frozen.
Frame.translation((0 * m, 0 * m, 0 * m)).origin = (1 * m, 0 * m, 0 * m)  # ty: error

# The count is the STRUCTURAL slot's integer, not a Length.
Node.placed_union(solid, 5 * m, PatternKind.linear((1.0, 0.0, 0.0), 0.5 * m))  # ty: error

# The narrowed count edit takes a ParamName, never bare text.
DocEdit.bind_count_param(solid, "fins")  # ty: error
