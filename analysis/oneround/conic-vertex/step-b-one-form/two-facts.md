# Two facts about step B's moved rows

Measured 2026-11-21 with temporary prints in the two rows, on the
one-form commit `e04a87227` and on step A's head `7f8faa67c`, where a
conic built from a centre is stored with it as on `main`. The raw lines
are the two `two-facts-*.txt` beside this file. Facts only.

## 1. `arc_ball_rows::a_short_arc_of_a_long_ellipse_has_a_ball_of_its_own_size` (a = 1e7, b = 1)

The ellipse is `Ellipse::new(Frame(centre (1, 0, 1e7), axis x, u_ref −z), 1e7, 1)`,
so its points are `(1, sin t, 1e7·(1 − cos t))`. At `t = −0.00045`:

| | z of the point | off the true z |
|---|---|---|
| true, in exact rationals | 1.012499982914062559905665 | |
| one form (`e04a87227`): the evaluator | 1.0124999829140626 | +6.4e-17 |
| centre form (step A, as `main`): the evaluator | 1.0124999824911356 | −4.23e-10 |
| the centre-form sum written out by hand, on either tree | 1.0124999824911356 | −4.23e-10 |

- The centre each tree reads is `(1, 0, 10000000)` exactly. On the
  one-form tree it is derived, `anchor − u·a` with the anchor
  `(1, 0, 0)`; the derivation is exact here.
- The cover's ball is the same on both trees: middle
  `(1, 0, 0.5062499912455678)`, radius `0.5062501912455183`.
- The true point lies `0.506250191668445` from that middle, 4.2e-10 past
  the radius. The centre-form evaluator's point lies `0.5062501912455183`
  from it, the radius to the bit. The one-form evaluator's lies
  `0.5062501916684451`.

So the ball does not hold the arc's true end on either tree. The row
passes in the centre form because the evaluator makes the same 4.2e-10
error as the cover's own arithmetic (`1e7 − 1e7·cos t`); on the one-form
tree the evaluator is right to 6e-17 and the cover is the one that is
off.

## 2. `swept::tests::the_placed_rim_chains_through_the_sketch_rim_to_the_radius` (the symbolic scalar)

The row reads `center = circle.frame().origin()`,
`rim = (q_from − center).norm()`, and decides `rim − radius`.

| reading | centre form (step A) | one form |
|---|---|---|
| `center.x`, node | `SymId(9662604709541122910758549267056833588)` | `SymId(4882613072376908171245050256024296805)` |
| `center.x`, enclosure | [2.3170, 2.4431] | [2.0349, 2.7252] |
| `rim`, node | `SymId(194752228669135623874558203394335317549)` | `SymId(23722191224643525062354919022121087443)` |
| `rim`, enclosure | [0.5920, 0.6681] | [0.3000, 1.0093] |
| `radius`, node | `SymId(336671177450829572653835067550136860562)` | the same |
| sign of `rim − radius`, shipped rules | `Zero` | `None` |
| sign of `(q_from + u·r) − u·r − q_from`, each component | `Zero` | `Zero` |
| sign of `point_at(0).x − q_from.x`, shipped rules | `Zero` | `Zero` |

- `c + u·r − u·r` does reduce: asked directly, the sign decides `Zero`
  on both trees.
- The anchored evaluator is not in the failing reading: `rim` is built
  from `frame().origin()`, and no `sin²(θ/2)` enters it. The evaluator's
  own point at zero decides `Zero` against `q_from` on the one-form tree.
- The failing run's counts are `checks_frozen: 0`, `frozen: 8`,
  `checks_expanded: 8`; the passing run's counts were not printed, so
  whether `frozen` differs is not shown.
- What differs is the node. In the centre form `center` is the node the
  sweep registered the rim against (`register_rigidity`, whose comment
  says the rim is "bound out of the expression … one subtraction, one
  node"), and `rim − radius` decides by that registration. In one form
  `frame().origin()` is a new node, `anchor − u·r`, so `rim` is a new
  node the registration does not name, and its numeric enclosure is
  wider ([0.30, 1.01] against a radius of [0.55, 0.72]), so neither
  channel decides.
