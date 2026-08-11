"""Off-lattice authoring, for `ty` to REJECT.

The compile-fail analog: every line marked `# ty: error` must draw at
least one diagnostic. These are exactly the states PATHS-DESIGN §2
declares unrepresentable, plus the typed-quantity boundary.
"""

from pncad import Doc, Node, NodeId, Open, SketchPlane, Start, circle, deg, m, mm

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
