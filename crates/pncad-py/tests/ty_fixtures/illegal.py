"""Off-lattice authoring, for `ty` to REJECT.

The compile-fail analog: every line marked `# ty: error` must draw at
least one diagnostic. These are exactly the states PATHS-DESIGN §2
declares unrepresentable, plus the typed-quantity boundary.
"""

from pncad import Node, Open, SketchPlane, Start, deg, m, mm

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
