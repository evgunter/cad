//! **The section certificate: a face pair's section, classified, and
//! every component of it either evidenced by the crossing layer or
//! proved not to lie inside both faces.**
//!
//! # The class it closes
//!
//! A face `F` of one operand can meet a face `G` of the other in a
//! closed loop interior to both faces: a loop that touches no edge of
//! either body. No edge event marks it, the join cuts nothing along it,
//! and on the crossings path face-region propagation carries each face's
//! side across it from regions the crossings elsewhere did cut. On the
//! no-crossings path the vertex probe sees each shell outside the other.
//! Either way the result is a valid body that is wrong: the overlap is
//! counted twice under ∪ and dropped under ∩ and ∖.
//!
//! # The evidence lemma (L1)
//!
//! Let `Σ = carrier(F) ∩ carrier(G)` and `γ` a connected component of
//! it. If `γ ∩ F ∩ G ≠ ∅` and `γ ⊄ int F ∩ int G`, some point `x` of
//! `γ ∩ F ∩ G` lies on an edge of one face inside the other: either
//! `γ ⊂ F ∩ G` and `x` is a point of `γ` on a boundary, or `x` is a
//! relative boundary point of the proper closed subset `γ ∩ F ∩ G` of
//! the connected `γ`. The sweep examines every edge × face pair whose
//! boxes overlap and which the narrow phase (`boolean::separating`)
//! does not certify apart, and records the contact there, or refuses
//! (premise **S**); a pair certified apart has no point to record. So the components no event can evidence are exactly those
//! with `γ ⊂ int F ∩ int G`, and the certificate's job is to prove, per
//! component, that `γ ∩ F ∩ G = ∅` or `γ ⊄ int F ∩ int G`.
//!
//! An event on a pair is a point of SOME component: with `Σ = γ₁ ∪ γ₂`
//! an event on `γ₁` says nothing of `γ₂`. So events are read in two
//! places only: W4, where the section has exactly one component, and
//! the no-event decision.
//!
//! # The witnesses, per component
//!
//! - **W0, apart**: the section has no component: a closed form counts
//!   none, or the march, whose exhaustiveness receipt accounts every
//!   cell of the pose's ball, finds no branch.
//! - **W1, unbounded**: faces are compact, so `γ ⊄ F`.
//! - **W1′, reaching a chart's boundary**: `γ` reaches a side of the
//!   parameter rectangle of `X`'s surface in a channel the chart does
//!   not close. `X` is a region of its one rectangle chart bounded by its
//!   loops' pcurves (C4), and in an open channel no pcurve wraps past the
//!   side, so a point of that side lying in `X` lies on a pcurve, an edge
//!   of `X`, and one lying outside `X` puts `γ` partly outside it. Either
//!   way `γ ⊄ int X`. A side in a closed channel is a seam the face may
//!   lie across with no edge on it (a seamless band, or a patch across
//!   the seam whose loops lift closed), so reaching it clears nothing:
//!   this is read per surface from the chart the loop walk reads
//!   (`pcurves::chart_closed_channels`), never assumed of a kind. Nor
//!   does a collapsed side clear, one the chart maps to a point (a
//!   revolved dome's pole): no pcurve need run along it, so `X` may hold
//!   the point with no edge there and `γ` pass through it inside `X`. A
//!   side clears only where its net row is decided to span more than a
//!   point (`pcurves::chart_side_bounds`), whatever else refuses the
//!   pose first. A `Side` region is certified only to lie within its reach of the
//!   side; it is reported where the side lies within the band of the
//!   other face's plane, so the side's points in `X` lie on an edge of
//!   `X` within the band of that face, a contact the sweep records or
//!   refuses (S).
//! - **W2, essential**: `γ` is essential on the carrier of `X` (it winds
//!   about the axis, or around the tube) and `X` reads [`Sheet::One`]:
//!   every closed curve in `int X` lifts to a closed curve with zero
//!   winding in every periodic channel, read off `X`'s loop walk
//!   (`ops::ChartCache::describes`), so `γ ⊄ int X`. (A section curve
//!   that is not closed — a ruling — is not inside a compact face at
//!   all.) That is all the certificate needs: a `γ` that meets `F ∩ G`
//!   and is not inside `int F ∩ int G` carries an event (L1). This is
//!   checked per FACE, never assumed of a kind: a seamless periodic
//!   band — one face closing on itself with no meridian edge — has two
//!   loops that each wind, and loses W2. A face whose walk refuses
//!   reads [`Sheet::Unread`], and a component nothing else clears
//!   refuses R-unread rather than R-undec.
//! - **W3, a witness point `Out`**: one point of `γ`, certified outside
//!   `F` or outside `G` (by the face's region on a curved face, by
//!   `contfp` on a plane), so `γ ⊄ F ∩ G`. A closed form names the point
//!   on `γ`; a marched loop names its carrier's middle point, within the
//!   loop's pad of `γ` ([`Component::pad`]), and is read out of a face
//!   only past the band padded by twice that.
//! - **W4, a lone evented component**: `Σ` has exactly one component and
//!   the pair has an event. The event's vertex lies on `Σ` within the
//!   band, hence on `γ`, and on an edge of `F` or `G`, so
//!   `γ ⊄ int F ∩ int G`. That is all the crossings path needs, since
//!   the join cuts along every crossing. The no-crossings path needs
//!   more, `γ ∩ int F ∩ int G = ∅`, and it follows there: were `γ` to
//!   meet `int F ∩ int G` and not lie in it, a walk along `γ` from such
//!   a point would leave `F` (or `G`) at a point of an edge of `F` lying
//!   on `G`'s carrier inside `G`. Where the carriers are transverse that
//!   edge crosses `G`, a crossing the sweep records or refuses (S), and
//!   the path would not be the no-crossings one; where they are tangent
//!   along `γ` the classification reads the pinch first and refuses
//!   R-tan, before any witness.
//! - **The no-event decision**: with no event on the pair,
//!   `γ ∩ F ∩ G` has no relative boundary point, so it is empty or all
//!   of `γ`, and one point decides. A witness strictly inside both faces
//!   is a **definite interior loop** (R-loop). Anything else, a witness
//!   on a boundary or with no verdict, refuses undecided (R-undec).
//!
//! One witness point per component is complete on the arms below: a
//! lone component is cleared by W4 or decided by its point, and a
//! component essential or unbounded on a carrier is cleared by W1 or W2.
//! A component's essentiality is the arm's claim, and an arm that does
//! not read it claims none: such a component, like a null one, is
//! cleared only by its witness. A section with a component no claim
//! clears beside others clears each by its witness or, with an event on
//! the pair, refuses R-undec, since
//! the event may lie on another component: cone × sphere, where a ball
//! beside the apex can meet each nappe in one null loop (with the apex
//! outside the ball no nappe can carry essential curves while the other
//! carries a loop); a cone against an oblique cylinder or another cone
//! in general pose, whose section can hold several loops null on both
//! carriers; and torus × square cylinder, where a wall biting the tube's
//! inner side leaves a null loop beside two essential ones and one
//! cutting its top leaves two null loops; and torus × parallel-axis
//! torus, which claims no component essential, so every part it lists
//! is cleared by its witness or refuses (the lily's stem and arch walls
//! meet in two loops, mirror images across the plane of the axes).
//!
//! # The refusals
//!
//! - **R-reach**: a kind pair or pose with no arm (torus against a
//!   cylinder neither parallel nor square to its axis, a torus tilted
//!   to it; a cone against a non-coaxial torus; a NURBS or approximated
//!   face paired with anything but a plane or another spline face, and a
//!   spline pair the whole reading refused short of a tangency). Every
//!   pair with a face that is not a
//!   plane is examined; two planes meet in a line, which W1 clears.
//! - **R-tan**: a classification margin `Zero` or undecided — a
//!   tangency, where components pinch and the count is not certified;
//!   on a spline section, a tangency the march meets, or the section
//!   grazing or running along a side of the wall's rectangle (on two
//!   spline faces, one no shared row or corner handed in covers, or a
//!   handed-in row the walls are tangent along at a point).
//!   A margin decided `Zero` where the carriers touch at one point is a
//!   touch instead (below), and refuses R-tan only when it is not
//!   certified out of either face.
//! - **R-order**: a crossing of the carriers the closed form cannot
//!   place on either side of a line it reads the section by. On the
//!   square wall, a fold or a strip crossing within the band of the
//!   torus's top or bottom parallel, where the tube's sheets join: the
//!   carriers cross there, but which side it falls on decides how the
//!   folds pair.
//! - **R-loop** and **R-undec**: the no-event decision, above.
//! - **R-unread**: as R-undec, on a component W2 would have cleared had
//!   the face's walk read.
//! - **R-lane** and **R-march**: a pair whose components no closed form
//!   counts (a general cylinder × torus or torus × torus pose) is read
//!   from its march; at a scalar with no march it refuses R-lane, naming
//!   the scalar, and where the march or the reading of its loops refuses,
//!   R-march.
//! - **Lone-vertex loops** refuse per pair (below).
//!
//! # A touch
//!
//! Sphere × plane, sphere × sphere (outside one another, or inside with
//! their centres decided apart), sphere × cylinder at its nearest ruling
//! with the girdle decided (the ball clear of the axis), and skew
//! cylinders outside one another touch at one point `at` when their
//! reach margin decides `Zero`. So does a torus whose plane, sphere or parallel-axis wall is
//! tangent to the tube at an elliptic point (the tube's outer half) that
//! is the strict extreme, over the torus, of the partner's level
//! function (the plane's height, the distance from the sphere's centre
//! or the wall's axis), clear of its next critical value by a decided
//! margin: in every pose the band admits, the torus's side of the
//! partner near that level is then one disc about `at` or empty. A
//! tangency at a hyperbolic point, along a whole parallel (a coaxial
//! partner, a plane normal to the axis, a sphere centred on it), or
//! where the elliptic or extreme reading is undecided, is a pinch. The
//! arm does not count the section there. In the exact pose
//! the carriers share `at` alone. In a pose the decided margin admits
//! where they cross, they share one loop `γ` about `at`, bounding a
//! connected region of each carrier inside which they stand within the
//! margin of each other. The argument needs that region connected, not
//! small: a plane near the top parallel cuts a long cap. On a pair with no
//! event, the touch clears when `at` places `Out` of either face. With
//! no event, `γ ∩ F ∩ G` is empty or `γ ⊂ int F ∩ int G` (L1). In the
//! second case, a path on `F`'s carrier from `γ` inward to `at` leaves
//! `F`, or leaves what lies over `G`, at some first point. That point
//! is on an edge of one face, over the other face, within the margin of
//! the other carrier: an edge × face contact the sweep records or
//! refuses (S), so the pair would carry an event. Where the loop is
//! smaller than `at`'s rounding, the path runs outside it by no more
//! than that rounding, where the carriers still stand within the band.
//!
//! A pair with an event refuses R-tan at a touch, since the argument
//! needs the pair's own silence. Any other tangency is a pinch — the
//! components meet, or one becomes two — and refuses R-tan.
//!
//! # Premises, each at its site
//!
//! - **S**, sweep completeness. It holds with the conic × plane lane
//!   (`reduce.rs` `sweep_direction`) examining every root, not only the
//!   first, and giving a conic that lies in the face's plane the line
//!   lane's endpoint treatment: its endpoints are recorded or
//!   certified `Out`, and its interior is evidenced through the
//!   neighbour faces as a coplanar line's is.
//! - **The ring torus**, `R > r > 0` (`geom::require_ring_torus`): the
//!   component counts of the torus arms need genus 1.
//! - **Event vertices lie on `Σ` within the band**, the posture every
//!   contact record takes.
//! - **Witness points are within rounding of `Σ`** on `f64` — of their
//!   own distance from each carrier's anchor, not of a form that squares
//!   it — and enclosures of it on the interval lane; containment is
//!   decided on the band, and a point it cannot decide is not a witness.
//!   A spline section's witness is the wall's point at the middle of
//!   the branch's pcurve, with a point of the section bracketed within a
//!   certified metre bound of it inside the branch's own tube window
//!   (`geom_brep::pair_section::plane_spline`), so the bracketed point
//!   is `γ`'s; that bound is its pad ([`Component::pad`]).
//! - **Lone-vertex loops.** The sweep is edge-driven, so a vertex of a
//!   `LoopBoundary::Empty` loop is never examined. That does not break
//!   L1 on its own: `F` surrounds its lone vertex on every side, so the
//!   vertex is a relative boundary point of `γ ∩ F ∩ G` only where it
//!   also lies on `∂G`, whose edges the sweep does examine; and
//!   W2 reads the face's region as its cycles bound it, so it covers
//!   the vertex too. What the sweep cannot see is a component
//!   in `F ∩ G` whose only boundary point is the lone vertex. That is an
//!   interior loop to every witness here except that it touches a
//!   vertex; the no-event decision would refuse it anyway, but nothing
//!   builds such a face at rest (tier 2 bans empty loops), so a pair
//!   whose face carries one refuses outright rather than resting an
//!   answer on that argument.
//!
//! # The angular margins (the lever and its pivot)
//!
//! Two margins are angles: the tilt between two axes, decided before a
//! parallel or coaxial reading is taken. `decide` meters in metres, so
//! each is levered by the distance over which the tilt can act on the
//! section. The section matters only inside the pair's **reach**
//! ([`Reach`]): the ball about the centre of the overlap of the two
//! faces' certified boxes, radius its half-diagonal.
//!
//! - **The pivot.** An offset between two nearly parallel axes is read
//!   at the partner axis's point nearest the reach's centre, never at
//!   the axis's stored origin: the stored origin can stand anywhere on
//!   the line (a kilometre away), and an axis tilted by `ε` drifts by
//!   `ε` times that distance. Read at the pivot, the same carrier line
//!   classifies the same way whatever origin stores it.
//! - **The lever** is the farthest the reach stands from the pivot,
//!   `|centre − pivot| + radius`, doubled: a tilt decided `Zero` there
//!   routes the pair to the parallel and coaxial readings having moved
//!   the partner by at most half the band across the reach, which leaves
//!   the other half to an offset a coaxial reading can still serve.
//!   Between two walls, whose tilt needs no pivot, the lever is the
//!   reach's diameter.
//! - **The sum.** Routed there, the partner is read as its image under
//!   the turn about the pivot onto the other axis's direction, which
//!   moves it by at most the chord between the axes times
//!   `|centre − pivot| + radius` anywhere the section can matter: the
//!   swing, plus how far the turn moves a point other than the pivot
//!   that the arm reads (a torus's centre, a cone's apex). Between two
//!   walls only a point's station along the turned axis moves it off the
//!   wall at first order, so their swing is the sine times the reach's
//!   radius, plus the radius times the sine squared. The offset at the
//!   pivot, and every row an arm serves on the parallel or coaxial
//!   reading, is decided across the reach beside that swing: the zero
//!   side on `|datum| + swing`, a definite side on the datum shrunk
//!   toward zero by it (the row's `_floor`). Never the two terms one at a
//!   time: each just inside the band would sum to nearly twice it. A
//!   straddle, or an undecided row, refuses R-tan naming it, except where
//!   the ruling charts read the pose with nothing bridged (a cone's
//!   partner off its axis).
//!
//! A LOOSER box widens the reach and lengthens the lever, so a tilt
//! decides `Zero` less readily — the tilted arm, or a refusal on reach,
//! takes more pairs — and a parallel reading that does survive is
//! bounded by the band over a region at least as large as the one the
//! section occupies. A box TIGHTER than its face would be the unsound
//! direction: its reach could miss where the section is.
//!
//! # What the arms say (component structure per kind pair)
//!
//! - **Torus × plane, torus × sphere** (one classification, the
//!   sphere's read in the section value, [`geom_brep::SphereTorusPose`],
//!   the plane's here). In the
//!   meridian plane `Π` holding the axis and the plane's normal (or the
//!   sphere's centre), let `T₊`, `T₋` be the tube circles at the near
//!   and far core points. The section has roots at `u' = 0` iff the
//!   partner cuts `T₊`, and at `u' = π` iff it cuts `T₋`. Both cut: two
//!   `(1,0)` curves (essential on the torus). One cut: a single null
//!   oval, the scrape, whose points on `Π` are the partner's crossings
//!   with that tube circle (one square root). Neither: two `(0,1)` curves
//!   when the partner separates `T₊` from `T₋`, else none. A
//!   two-component section is therefore always essential.
//! - **Torus × coaxial {cylinder, torus}**: parallels, essential on both.
//!
//! Every torus pair but the plane and the cone is the section value's
//! ([`geom_brep::pair_section()`]): its pose decided once, the critical
//! gap read before any count, and its components classified by these
//! arms there, read here by [`of_pair_section`]. The parallel- and
//! square-axis cylinder, the parallel-axis torus and the sphere below
//! are those arms.
//! - **Torus × parallel-axis torus** (axis offset `e`): mirror-symmetric
//!   across the plane `Π` of the axes. Each loop of the curve matching
//!   the two meridian circles' heights meets `Π` in `2k` points, the
//!   crossings of the tori's meridian circles there, and carries `k`
//!   components, each witnessed at one of them; a loop meeting `Π`
//!   nowhere carries two mirror components or none, read at one height
//!   (the section value's arm, read through [`geom_brep::TorusTorusPose`]).
//!   Tube tops or bottoms level with each other are R-tan.
//! - **Torus × parallel-axis cylinder** (axis offset `e`): with
//!   `ρ ∈ [|e − ρc|, e + ρc]` against `[R − r, R + r]`, inside gives two
//!   loops encircling the cylinder, across one bound a single null loop
//!   (witness at the extreme ruling), across both two `(0,1)` loops.
//! - **Torus × square cylinder** (the wall's axis square to the torus
//!   axis: a radial hole through the tube, or one off the axis). In the
//!   meridian plane `Π` normal to the wall's axis, the wall's trace `K`
//!   is a circle, met by the tube circles `T±` and the strip `|z| ≤ r`
//!   in closed form. Each component of the section is the lift of a run
//!   of `K` (on the torus's outer or inner sheet over it) between two
//!   points of `K` on `T±`, through `Π` there: null on the wall, and
//!   winding about the torus axis iff those points lie on different
//!   tube circles. Or it is one of a mirror pair over a run with no such
//!   point: essential on the wall for `K` inside the strip, else null
//!   there and winding about the tube iff the run crosses the strip.
//!   Every component carries a witness (the section value's arm, read
//!   through [`geom_brep::CylinderTorusPose`]).
//! - **Cylinder × cylinder** (common-perpendicular offset `δ₀`):
//!   `|δ₀| + r_min < r_max` gives two loops encircling the thinner
//!   axis; `|r₁ − r₂| < |δ₀| < r₁ + r₂` a single null saddle loop,
//!   witnessed on the arc's middle ruling; parallel axes give rulings
//!   (lines). Equal radii with meeting axes (`δ₀` and `r₁ − r₂` both
//!   `Zero`) give the two bisector-plane ellipses, which cross at two
//!   nodes: one component, essential on both walls, witnessed at a node.
//! - **Cylinder × plane**: an ellipse encircling the wall, or rulings.
//!   Either way the section cannot lie in a wall that describes. The
//!   count is not certified (two rulings, or one ellipse), so W4 is
//!   never read here and the pair clears by W2 or refuses.
//! - **Sphere × {plane, sphere}**: one circle. **Sphere × cylinder**
//!   (axis offset `e`): one loop when `|e − ρc| < ρs < e + ρc`,
//!   witnessed on the nearest ruling; two loops encircling the cylinder
//!   when `e + ρc < ρs`.
//! - **NURBS × plane** (an approximated face read as its fit): the
//!   pair's classification in the section value
//!   ([`geom_brep::pair_section()`], C5), read and never re-decided. Its
//!   hull is W0: the control net's every point decided strictly on one
//!   side of the plane, the patch lying in the net's convex hull (its
//!   weights are positive); a net within the zero band everywhere is
//!   coincident (R-tan, named) and one in band escalates (R-tan). A
//!   collapsed side not decided clear of the plane refuses at the door
//!   (R-reach). Through the march door the boundary pass refuses a graze
//!   or a side held tangent (R-tan, named), and the regularity sweep is
//!   the guard's loop-free stage: the wall's normal bounded off the
//!   plane's, so no component closes in the wall's chart, each ending
//!   on a side the pass found an end on; it clears by W1′ where every
//!   such side bounds the wall, and is otherwise read whole. Any other
//!   pose is the whole reading (C3's `all()`, at `f64`; with no door,
//!   R-lane): a closed branch is a loop, read by W3, W4 or the no-event
//!   decision; an open branch, a `Side` region and a `Corner` region
//!   reach the wall's chart boundary (W1′). The reading accounts every
//!   cell of the wall's rectangle, so the count is certified.
//! - **Cone × plane**: an ellipse on one nappe (essential, one
//!   component), a hyperbola or two lines through the apex (unbounded),
//!   or near the parabola either (essential, no witness), decided on
//!   the aperture margin alone.
//! - **Cone × sphere**: per nappe, two essential curves, one null loop
//!   (the arc of generators that meet the ball) or none; the apex inside
//!   the ball, one essential curve per nappe; the apex on the sphere,
//!   R-tan.
//! - **Cone × coaxial {cylinder, cone, torus}**: parallels, essential on
//!   both. **Cone × parallel-axis cylinder**: two components, one per
//!   nappe, essential on the cylinder, and on the cone iff the cylinder
//!   encloses its axis.
//! - **Cone × oblique cylinder, cone × tilted or parallel-axis cone**
//!   (the ruling charts, `section_cert/ruling.rs`): on each carrier's
//!   ruling chart every maximal arc of lines meeting the partner twice
//!   is one component, null on that carrier, and lines meeting it twice
//!   round the whole turn give two components essential on it; a
//!   component that runs along an asymptotic direction of the partner
//!   is unbounded. The classes are uniform per chart, so the cone's
//!   chart and the partner's give every component both flags, and their
//!   counts must agree. An apex on the other carrier, a double root of
//!   either chart's discriminant or leading coefficient, and (against a
//!   cylinder) a generator along its axis are R-tan; so is a pose whose
//!   reading the `f64` representation cannot resolve at the band (the
//!   charts' `_precision` rows). Two cones of one aperture on parallel
//!   axes share their forms' quadratic part, and their section is a
//!   plane conic, classified as cone × plane is: an ellipse essential on
//!   both, or unbounded branches.
//!
//! Cone components are listed on the DOUBLE cone: one on the other
//! nappe from the face carries a witness the face's trim places `Out`.
//!
//! # What this replaced
//!
//! The no-crossings path's two blanket gates refused a torus face, and
//! a pair of cylinder walls, whose boxes overlapped a partner's. Their
//! arguments are these arms: a plane section of a wall is essential or
//! unbounded (the plane exemption the wall gate made, which rested on
//! every wall carrying a meridian edge — now checked per face by W2); a
//! torus meets a plane in a scrape oval interior to both faces, which a
//! blanket gate could only refuse. The conservative refusal they were
//! known for — a cube standing in a donut's hole — is W0 or W2 here.

use geom_core::{Band, Decide, Margin, Point3, Real, Sign, Vec3};

use super::contain::FaceContainment;
use crate::validate::decide;
use geom_brep::intersect::decide_across;
use geom_brep::pair_section::{Pinch, signs, signs_across};
use geom_brep::{AxisPose, ExtentBall, Turn, axis_pose};

/// Which face of the pair a statement is about.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Side {
    /// The pair's first face.
    F,
    /// The pair's second face.
    G,
}

/// The region where a pair's section can matter: the ball about the
/// centre of the overlap of the two faces' certified boxes, radius its
/// half-diagonal (module docs, the lever and its pivot).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Reach<T: Real> {
    /// The overlap box's centre.
    pub centre: Point3<T>,
    /// Its half-diagonal.
    pub radius: T,
}

impl<T: Real> Reach<T> {
    /// The reach as a consumed extent: its foot on an axis is the pivot
    /// a datum on that axis is read at, and its farthest distance from a
    /// point the travel of a turn about it (module docs, the lever and
    /// its pivot).
    fn ball(self) -> ExtentBall<T> {
        ExtentBall::new(self.centre, self.radius)
    }
}

/// One connected component of a pair's section.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Component<T: Real> {
    /// The component is unbounded (a line).
    pub unbounded: bool,
    /// It cannot lie inside `F` if `F` describes: essential on `F`'s
    /// carrier (or, on a cylinder × plane pair, essential or unbounded).
    pub essential_f: bool,
    /// The same for `G`.
    pub essential_g: bool,
    /// One point of the component, in closed form, on a marched loop's
    /// carrier, or a spline section's wall point, within `pad` of the
    /// component.
    pub witness: Option<Point3<T>>,
    /// It reaches a side of the named face's chart rectangle in a
    /// channel that chart does not close, and no collapsed side (W1′);
    /// `F` where it reaches both faces' (one is enough).
    pub reaches: Option<Side>,
    /// **How far the witness may stand from the component**, in metres:
    /// a marched loop's pad ([`geom_brep::CarriedComponent::pad`], where
    /// closeness is argued: the component within it of the carrier, the
    /// carrier within it of each surface), zero for a closed form. W3
    /// reads the witness `w` against the band padded by twice it
    /// (`certify`): a face's placement margins are lengths read at a
    /// point's foot on the face's surface, and for the point `c` of the
    /// component nearest `w`, which lies on the surface (its own foot),
    /// `|foot(w) − c| ≤ |foot(w) − w| + |w − c| ≤ 2·|w − c| ≤ 2·pad`:
    /// `foot(w)` is the surface point nearest `w`, no farther than `c`.
    /// So a margin at `c` is within twice `pad` of the one read at `w`.
    pub pad: f64,
}

impl<T: Real> Component<T> {
    /// A component a closed form reads: unbounded or not, essential on
    /// `[F, G]` or not, its witness on it, reaching no chart boundary.
    pub(crate) fn closed_form(
        unbounded: bool,
        [essential_f, essential_g]: [bool; 2],
        witness: Option<Point3<T>>,
    ) -> Self {
        Self {
            unbounded,
            essential_f,
            essential_g,
            witness,
            reaches: None,
            pad: 0.0,
        }
    }
}

/// A pair's section, classified.
#[derive(Clone, Debug)]
pub(crate) enum Section<T: Real> {
    /// The components, and whether their count is CERTIFIED to be one
    /// (`single`), which W4 needs.
    Components {
        /// Every component.
        parts: Vec<Component<T>>,
        /// The section has exactly one component.
        single: bool,
    },
    /// A classification margin is `Zero` or undecided: R-tan, naming the
    /// predicate.
    Tangent(&'static str),
    /// A crossing of the carriers the classification cannot place on
    /// either side of a line it reads the section by: R-order, naming the
    /// predicate.
    Unordered(&'static str),
    /// A classification margin decided `Zero` where the carriers touch
    /// at one point: the section, if the carriers meet at all, lies in
    /// the touch's ball.
    Touch(Touch<T>),
    /// No arm for the kind pair or the pose: R-reach.
    Intractable,
    /// The components are the march's, and the scalar, named, has no
    /// march: R-lane.
    NoLane(&'static str),
    /// The components are the march's, and the march, or the reading of
    /// a marched loop, refused: R-march.
    Unmarched,
}

/// **A touch of two carriers**: a margin decided `Zero` on an arm whose
/// tangent pose meets in one point. The carriers share that point in
/// the exact pose and, in a pose the decided margin admits where they
/// cross, one loop about it bounding a connected region of each carrier
/// where they stand within the margin (not necessarily a small one).
#[derive(Clone, Copy, Debug)]
pub(crate) struct Touch<T: Real> {
    /// The margin decided `Zero`.
    pub name: &'static str,
    /// The touch: inside the loop of a crossing pose, where the carriers
    /// stand deepest in each other, and on both carriers within the
    /// margin. Each arm says which point it reads.
    pub at: Point3<T>,
}

/// Why a component was cleared.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Cleared {
    /// W1.
    Unbounded,
    /// W1′: the component reaches the named face's chart boundary.
    Reaches(Side),
    /// W2, on the named face.
    Essential(Side),
    /// W3, on the named face.
    Out(Side),
    /// W4.
    LoneEvented,
    /// A touch on a pair with no event, its centre placed `Out` of the
    /// named face.
    TouchOut(Side),
}

/// Why a pair refused.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// The roster the refusal censuses read (test builds only).
#[cfg_attr(test, derive(strum::EnumIter))]
pub(crate) enum Refusal {
    /// R-reach: no arm.
    Reach,
    /// R-tan, with the margin's predicate.
    Tangent(&'static str),
    /// R-order, with the margin's predicate.
    Unordered(&'static str),
    /// R-loop: a component certified inside both faces, no event.
    Loop,
    /// R-undec: no witness cleared a component and none proved it
    /// interior.
    Undecided,
    /// A face of the pair carries a lone-vertex loop.
    LoneVertex,
    /// R-unread: a component essential on a face whose loop walk
    /// refused for a numerical or unbuilt reason ([`Sheet::Unread`]),
    /// and no other witness cleared it.
    Unread,
    /// R-lane: the pair's components are the march's, and the scalar,
    /// named, has none.
    NoLane(&'static str),
    /// R-march: the pair's components are the march's, and the march or
    /// the reading of its loops refused.
    Unmarched,
}

/// **W2's answer about one face**: whether every closed curve in its
/// interior lifts to a closed curve with zero winding
/// (`ops::ChartCache::describes`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Sheet {
    /// It does: W2 clears a component essential on the face.
    One,
    /// The walk read the face and cannot certify it: a loop winds, two
    /// loops meet the apex, or a cone face spans both nappes. W2 does
    /// not clear.
    Not,
    /// The walk refused before reading it, for a numerical or unbuilt
    /// reason. W2 does not clear, and a component nothing else clears
    /// refuses [`Refusal::Unread`].
    Unread,
}

impl Refusal {
    /// The sentence the no-crossings path's refusal carries.
    pub(crate) fn what(self) -> &'static str {
        match self {
            Self::Reach => {
                "a curved face's box overlaps a face of the other solid, no crossing layer saw \
                 an event, and the pair has no section classification (an oblique or \
                 non-coaxial pose, or a spline face) to prove the two faces do not meet \
                 in a closed loop interior to both"
            }
            Self::Tangent(_) => {
                "a curved face and a face of the other solid have tangent or near-tangent \
                 carriers and no crossing layer saw an event; the section's component count is \
                 not certified there"
            }
            Self::Unordered(_) => {
                "a curved face and a face of the other solid cross where the section's closed \
                 form cannot place the crossing on either side of a line it reads the section \
                 by, and no crossing layer saw an event; the section's component count is not \
                 certified there"
            }
            Self::Loop => {
                "a curved face meets a face of the other solid in a closed loop interior to \
                 both faces, which no edge event and no vertex probe can see; the loop is \
                 certified, and cutting it in is not built"
            }
            Self::Undecided => {
                "a curved face's section with a face of the other solid has a component no \
                 witness could place in or out of both faces, and no crossing layer saw an \
                 event on it"
            }
            Self::LoneVertex => {
                "a face of the pair carries a lone-vertex loop, which the crossing layer does \
                 not examine"
            }
            Self::Unread => {
                "a curved face's section with a face of the other solid has a component that \
                 winds about the face's surface, and no crossing layer saw an event on it; the \
                 face's boundary, which would show the component leaves it, could not be read, \
                 most often because a vertex of it stands too near the surface's axis or apex \
                 (moving that vertex away lets it be read)"
            }
            Self::NoLane(_) => {
                "a curved face's section with a face of the other solid has components no \
                 closed form counts, and the march that lists them is written at f64, not at \
                 this scalar; run the operation at f64"
            }
            Self::Unmarched => {
                "a curved face's section with a face of the other solid has components no \
                 closed form counts, and the march that lists them refused, so no crossing \
                 layer's silence can be read against them"
            }
        }
    }
}

/// The unit vector along `v`.
fn unit<T: Real>(v: Vec3<T>) -> Vec3<T> {
    v / v.norm()
}

use geom_brep::square_part as perp_to;

/// **The meridian half-plane `Π` of the torus `(c, a)` toward `v`**: the
/// norm `s` of `v`'s part square to `a`, and the map from `Π`'s
/// coordinates — `x` along that part, `y` along `a` — to space.
fn meridian<T: Real>(c: Point3<T>, a: Vec3<T>, v: Vec3<T>) -> (T, impl Fn(T, T) -> Point3<T>) {
    let (perp, s) = perp_to(v, a);
    (s, move |x: T, y: T| c + perp / s * x + a * y)
}

/// A margin's sign, `None` when undecided.
fn sign<T: Decide>(name: &'static str, m: Margin<T>, band: Band) -> Option<Sign> {
    decide(name, m, band).ok()
}

impl<T: Real> From<Pinch> for Section<T> {
    fn from(p: Pinch) -> Self {
        Section::Tangent(p.name)
    }
}

/// A [`Touch`] at `at()` when the pinch `p` is one of `names` decided
/// `Zero`, the one reading under which the carriers share at most one
/// loop about it, bounding a connected region where they stand within
/// the margin; R-tan otherwise.
fn touch<T: Real>(p: Pinch, names: &[&'static str], at: impl FnOnce() -> Point3<T>) -> Section<T> {
    if p.undecided.is_none() && names.contains(&p.name) {
        Section::Touch(Touch {
            name: p.name,
            at: at(),
        })
    } else {
        p.into()
    }
}

fn none<T: Real>() -> Section<T> {
    Section::Components {
        parts: Vec::new(),
        single: false,
    }
}

fn lone<T: Real>(witness: Point3<T>) -> Section<T> {
    Section::Components {
        parts: vec![Component::closed_form(false, [false, false], Some(witness))],
        single: true,
    }
}

fn essential_pair<T: Real>(on_f: bool, on_g: bool) -> Section<T> {
    let c = Component::closed_form(false, [on_f, on_g], None);
    Section::Components {
        parts: vec![c, c],
        single: false,
    }
}

/// The same classification with the roles of `F` and `G` exchanged.
fn swapped<T: Real>(s: Section<T>) -> Section<T> {
    match s {
        Section::Components { parts, single } => Section::Components {
            parts: parts
                .into_iter()
                .map(|c| Component {
                    essential_f: c.essential_g,
                    essential_g: c.essential_f,
                    ..c
                })
                .collect(),
            single,
        },
        other => other,
    }
}

/// **The classifier: two carriers in, the section's components out.**
/// Pure: surfaces, the pair's reach (module docs) and the band.
#[cfg(any(test, feature = "test-support"))]
pub(crate) fn classify<T: Decide>(
    f: &geom::Surface<T>,
    g: &geom::Surface<T>,
    reach: Reach<T>,
    lane: Option<geom_brep::SsiLane<T>>,
    band: Band,
) -> Section<T> {
    classify_known(f, g, reach, lane, band, Known::default())
}

/// **What the boolean hands a spline pair's reading as known**: the rows
/// and vertices the two faces share, each in the order `[F, G]`
/// ([`geom_brep::spline_pair_section`]).
#[derive(Clone, Debug)]
pub(crate) struct Known<T: Real> {
    /// The shared rows.
    pub rows: Vec<geom_brep::KnownRow<T>>,
    /// The shared corners.
    pub corners: Vec<geom_brep::ssi::PairCorner>,
}

impl<T: Real> Default for Known<T> {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            corners: Vec::new(),
        }
    }
}

/// [`classify`] with what the boolean knows of a spline pair handed in
/// (`known`, read by the spline pair's arm alone).
pub(crate) fn classify_known<T: Decide>(
    f: &geom::Surface<T>,
    g: &geom::Surface<T>,
    reach: Reach<T>,
    lane: Option<geom_brep::SsiLane<T>>,
    band: Band,
    known: Known<T>,
) -> Section<T> {
    use geom::Surface as S;
    let marched = move |s: &geom_brep::PairSection<T>| of_carried(s.carry(lane, band).as_ref());
    match (f, g) {
        (S::Sphere { .. } | S::Cylinder { .. } | S::Torus { .. }, S::Torus { .. })
        | (S::Torus { .. }, S::Sphere { .. } | S::Cylinder { .. })
        | (S::Sphere { .. }, S::Cylinder { .. })
        | (S::Cylinder { .. }, S::Sphere { .. }) => {
            match geom_brep::pair_section(f, g, None, None, band) {
                Ok(geom_brep::SectionRead::Built(section)) => {
                    of_pair_section(&Ok(section), band, marched)
                }
                Ok(geom_brep::SectionRead::Elsewhere(_)) => Section::Intractable,
                Err(e) => of_pair_section(&Err(e), band, marched),
            }
        }
        (S::Torus { .. }, S::Plane { .. } | S::Cone { .. }) => torus_pair(f, g, reach, band),
        (S::Plane { .. } | S::Cone { .. }, S::Torus { .. }) => {
            swapped(torus_pair(g, f, reach, band))
        }
        (
            S::Cone { .. },
            S::Plane { .. } | S::Sphere { .. } | S::Cylinder { .. } | S::Cone { .. },
        ) => cone_pair(f, g, reach, band),
        (S::Plane { .. } | S::Sphere { .. } | S::Cylinder { .. }, S::Cone { .. }) => {
            swapped(cone_pair(g, f, reach, band))
        }
        (
            &S::Cylinder {
                origin: o1,
                axis: d1,
                radius: r1,
                ..
            },
            &S::Cylinder {
                origin: o2,
                axis: d2,
                radius: r2,
                ..
            },
        ) => cylinder_cylinder(o1, unit(d1), r1, o2, unit(d2), r2, reach, band),
        (S::Cylinder { .. }, S::Plane { .. }) => cylinder_plane(),
        (S::Plane { .. }, S::Cylinder { .. }) => swapped(cylinder_plane()),
        (&S::Sphere { center, radius, .. }, &S::Plane { origin, normal, .. }) => {
            sphere_plane(center, radius, origin, unit(normal), band)
        }
        (&S::Plane { origin, normal, .. }, &S::Sphere { center, radius, .. }) => {
            sphere_plane(center, radius, origin, unit(normal), band)
        }
        (
            &S::Sphere {
                center: c1,
                radius: r1,
                ..
            },
            &S::Sphere {
                center: c2,
                radius: r2,
                ..
            },
        ) => sphere_sphere(c1, r1, c2, r2, band),
        (S::Nurbs(_) | S::Approx(_), S::Plane { .. })
        | (S::Plane { .. }, S::Nurbs(_) | S::Approx(_)) => spline_plane(f, g, lane, band),
        (S::Nurbs(_) | S::Approx(_), S::Nurbs(_) | S::Approx(_)) => {
            spline_pair(f, g, known, lane, band)
        }
        _ => Section::Intractable,
    }
}

/// **Two spline faces, read whole** ([`geom_brep::spline_pair_section`]
/// through `lane`, module docs): C3's reading of the pair with the rows
/// and corners the faces share handed in, its components listed (a closed
/// branch a loop with its witness; an open branch, a `Side` region and a
/// `Corner` region reaching the sides they end on, cleared by W1′ where
/// one bounds its face), its refusals mapped by name. With no lane it is
/// R-lane.
fn spline_pair<T: Decide>(
    f: &geom::Surface<T>,
    g: &geom::Surface<T>,
    known: Known<T>,
    lane: Option<geom_brep::SsiLane<T>>,
    band: Band,
) -> Section<T> {
    let read = geom_brep::spline_pair_section(f, g, (known.rows, known.corners), lane, band);
    of_pair_section(&read, band, |_| Section::Unmarched)
}

/// **A spline face against a plane, read off the pair's
/// classification** ([`geom_brep::pair_section()`] through `lane`, module
/// docs): the hull's apart is W0 and its coincidence R-tan; the
/// door's and the boundary pass's refusals map by name; the regularity
/// sweep's loop-free verdict ([`geom_brep::Components::Reaching`])
/// clears by W1′ where every side it names bounds the wall, and is
/// otherwise read whole; any other pose is C3's whole reading. With no
/// lane a pose the hull does not decide is R-lane.
fn spline_plane<T: Decide>(
    f: &geom::Surface<T>,
    g: &geom::Surface<T>,
    lane: Option<geom_brep::SsiLane<T>>,
    band: Band,
) -> Section<T> {
    let listed = move |s: &geom_brep::PairSection<T>| {
        of_pair_section(&s.read_whole(lane, band), band, |_| Section::Unmarched)
    };
    match geom_brep::pair_section(f, g, None, lane, band) {
        Ok(geom_brep::SectionRead::Built(section)) => of_pair_section(&Ok(section), band, listed),
        Ok(geom_brep::SectionRead::Elsewhere(_)) => Section::Intractable,
        Err(e) => of_pair_section(&Err(e), band, listed),
    }
}

/// The torus arms the section value does not build: `t` is a torus, `p`
/// a plane or a cone.
fn torus_pair<T: Decide>(
    t: &geom::Surface<T>,
    p: &geom::Surface<T>,
    reach: Reach<T>,
    band: Band,
) -> Section<T> {
    let &geom::Surface::Torus {
        center: c,
        axis,
        major_radius: big_r,
        minor_radius: r,
        ..
    } = t
    else {
        return Section::Intractable;
    };
    if geom::require_ring_torus(big_r, r, band).is_err() {
        return Section::Intractable;
    }
    let a = unit(axis);
    match *p {
        geom::Surface::Plane { origin, normal, .. } => {
            torus_plane(c, a, big_r, r, origin, unit(normal), band)
        }
        geom::Surface::Cone {
            apex,
            axis: d,
            half_angle,
            ..
        } => {
            let d = unit(d);
            // `z` is read along the cone's axis from its own apex: the
            // turn onto the torus axis moves it by the torus centre's
            // station from the pivot times `1 − cos` (at most `sin²`),
            // and by the offset's part along the tilted axis.
            let turn = |t: &Turn<T>| (c - t.at).dot(a).abs() * t.tilt.powi(2) + t.offset * t.tilt;
            match axis_pose((c, a), (apex, d), turn, reach.ball(), band) {
                AxisPose::Coaxial { slack, .. } => {
                    // In the meridian half-plane about the apex, each
                    // nappe's generator is a line through it at `α` from
                    // the axis; the tube circle, centred `(R, z)`, lies in
                    // `ρ > 0`, which only the nappe's own half of the line
                    // reaches.
                    let (s, co) = half_angle.sin_cos();
                    let z = (c - apex).dot(d);
                    match signs_across(
                        [
                            (
                                "section_torus_coaxial_cone_near",
                                "section_torus_coaxial_cone_near_floor",
                                r - (big_r * co - z * s).abs(),
                            ),
                            (
                                "section_torus_coaxial_cone_far",
                                "section_torus_coaxial_cone_far_floor",
                                r - (big_r * co + z * s).abs(),
                            ),
                        ],
                        slack,
                        band,
                    ) {
                        Ok([false, false]) => none(),
                        Ok(_) => essential_pair(true, true),
                        Err(tan) => tan.into(),
                    }
                }
                AxisPose::Undecided(diag) => {
                    Section::Tangent(diag.predicate.unwrap_or("section_axes_offset"))
                }
                AxisPose::Parallel { .. } | AxisPose::Other => Section::Intractable,
            }
        }
        _ => Section::Intractable,
    }
}

/// **Torus × plane**, the plane through `p0` with unit normal `n`.
fn torus_plane<T: Decide>(
    c: Point3<T>,
    a: Vec3<T>,
    big_r: T,
    r: T,
    p0: Point3<T>,
    n: Vec3<T>,
    band: Band,
) -> Section<T> {
    let h = n.dot(p0 - c);
    let na = n.dot(a);
    let (s, in_pi) = meridian(c, a, n);
    // In Π, with x along ê = n⊥/s and y along a: the tube circle T_σ is
    // centred (σR, 0), radius r, and the plane is the line
    // s·x + n_a·y = h, whose unit normal is (s, n_a); `d` is the
    // plane's offset from T_σ's centre and `(σR + d·s, d·n_a)` its foot.
    let offset = |sigma: T| h - sigma * s * big_r;
    match plane_cut(big_r, r, h, s, band) {
        // Tangent to T_σ at the foot, an elliptic point when the foot
        // stands on the tube's outer half: the extreme `σ(sR + r)` of
        // the height `n·(x − c)`, whose other critical values `±(sR − r)`
        // stand `2·min(sR, r) ≥ 2·σ·d·s` away.
        PlaneCut::Pinch(tan) if tan.name == NEAR || tan.name == FAR => {
            let sigma = if tan.name == NEAR {
                T::one()
            } else {
                -T::one()
            };
            let d = offset(sigma);
            if sign(
                "section_torus_plane_touch_elliptic",
                Margin::of(sigma * d * s),
                band,
            ) != Some(Sign::Positive)
            {
                return tan.into();
            }
            touch(tan, &[NEAR, FAR], || in_pi(sigma * big_r + d * s, d * na))
        }
        PlaneCut::Pinch(tan) => tan.into(),
        PlaneCut::AroundAxis | PlaneCut::AroundTube => essential_pair(true, false),
        PlaneCut::Empty => none(),
        PlaneCut::Null { near } => {
            let sigma = if near { T::one() } else { -T::one() };
            let d = offset(sigma);
            let (fx, fy) = (sigma * big_r + d * s, d * na);
            let half = ((r - d.abs()) * (r + d.abs())).sqrt();
            lone(in_pi(fx - na * half, fy + s * half))
        }
    }
}

/// [`torus_plane`]'s near-tube margin: the plane cuts the tube circle
/// `T₊` of `Π`.
const NEAR: &str = "section_torus_plane_near_tube";
/// [`torus_plane`]'s far-tube margin: the plane cuts `T₋`.
const FAR: &str = "section_torus_plane_far_tube";

/// **How a plane cuts a ring torus**, by [`torus_plane`]'s margins in
/// the meridian half-plane `Π` toward the plane's normal: the plane at
/// height `h` over the centre, `s` the norm of its normal's part square
/// to the axis.
enum PlaneCut {
    /// It cuts both tube circles of `Π`, so it crosses every meridian:
    /// two loops, each round the axis.
    AroundAxis,
    /// It passes between the tube circles: two loops, each round the
    /// tube.
    AroundTube,
    /// It cuts one tube circle, `T₊` where `near`: one null oval.
    Null { near: bool },
    /// It misses the torus.
    Empty,
    /// A margin decided `Zero` or undecided: a tangency.
    Pinch(Pinch),
}

fn plane_cut<T: Decide>(big_r: T, r: T, h: T, s: T, band: Band) -> PlaneCut {
    let [near, far] = match signs(
        [
            (NEAR, r - (h - s * big_r).abs()),
            (FAR, r - (h + s * big_r).abs()),
        ],
        band,
    ) {
        Ok(x) => x,
        Err(tan) => return PlaneCut::Pinch(tan),
    };
    match (near, far) {
        (true, true) => PlaneCut::AroundAxis,
        (false, false) => match signs(
            [("section_torus_plane_between", s * big_r - r - h.abs())],
            band,
        ) {
            Ok([true]) => PlaneCut::AroundTube,
            Ok([false]) => PlaneCut::Empty,
            Err(tan) => PlaneCut::Pinch(tan),
        },
        (true, false) | (false, true) => PlaneCut::Null { near },
    }
}

/// **The loops a plane cuts on a ring torus**, by [`torus_plane`]'s own
/// margins, so a refusal naming them and the certificate read one
/// decision: `Ok(None)` where those margins decide a tangency or no
/// section, and the diagnostics of the first margin they leave
/// undecided. The plane runs through `p0` with normal `n`, the torus is
/// `(c, a, R, r)`; neither direction need be unit.
pub(super) fn torus_plane_loops<T: Decide>(
    (c, a, big_r, r): (Point3<T>, Vec3<T>, T, T),
    (p0, n): (Point3<T>, Vec3<T>),
    band: Band,
) -> Result<Option<super::SpiricLoops>, geom_core::Indeterminate> {
    let (a, n) = (unit(a), unit(n));
    let (_, s) = perp_to(n, a);
    match plane_cut(big_r, r, n.dot(p0 - c), s, band) {
        PlaneCut::AroundAxis => Ok(Some(super::SpiricLoops::AroundAxis)),
        PlaneCut::AroundTube => Ok(Some(super::SpiricLoops::AroundTube)),
        PlaneCut::Null { .. } => Ok(Some(super::SpiricLoops::Null)),
        PlaneCut::Empty => Ok(None),
        PlaneCut::Pinch(tan) => tan.undecided.map_or(Ok(None), Err),
    }
}

/// The cone arms: `k` is a cone, `p` its partner. Components are listed
/// on the DOUBLE cone: one on the other nappe from the face has a
/// witness the face's trim places `Out`, so `classify` never needs the
/// face's nappe.
fn cone_pair<T: Decide>(
    k: &geom::Surface<T>,
    p: &geom::Surface<T>,
    reach: Reach<T>,
    band: Band,
) -> Section<T> {
    let &geom::Surface::Cone {
        apex,
        axis,
        half_angle,
        u_ref,
    } = k
    else {
        return Section::Intractable;
    };
    let cone = Cone::new(apex, axis, half_angle, u_ref);
    match *p {
        geom::Surface::Plane { origin, normal, .. } => {
            cone_plane(&cone, origin, unit(normal), reach, band)
        }
        geom::Surface::Sphere { center, radius, .. } => cone_sphere(&cone, center, radius, band),
        geom::Surface::Cylinder {
            origin,
            axis: d,
            radius: rc,
            ..
        } => match axis_pose(
            (apex, cone.a),
            (origin, unit(d)),
            |_| T::zero(),
            reach.ball(),
            band,
        ) {
            // The meridians meet at `h = ±r_c/τ`: a parallel per nappe.
            AxisPose::Coaxial { .. } => essential_pair(true, true),
            AxisPose::Parallel { e, toward, slack } => {
                cone_parallel_cylinder(&cone, (e, slack), toward, rc, band)
            }
            // The ruling charts read the pose as it stands, with nothing
            // bridged, so an offset neither reading claims is theirs.
            AxisPose::Undecided(_) | AxisPose::Other => {
                cone_cylinder(&cone, origin, unit(d), rc, reach, band)
            }
        },
        geom::Surface::Cone {
            apex: apex2,
            axis: d,
            half_angle: half_angle2,
            u_ref: u_ref2,
        } => match axis_pose(
            (apex, cone.a),
            (apex2, unit(d)),
            // The apexes' gap is read off `apex2` itself, which the turn
            // onto the first axis moves along it by its station from the
            // pivot times `1 − cos`, at most `sin²`.
            |t| (apex2 - t.at).dot(unit(d)).abs() * t.tilt.powi(2),
            reach.ball(),
            band,
        ) {
            // The meridians `ρ = |h|τ₁` and `ρ = |h − d|τ₂` meet in one or
            // two parallels, each essential on both; a common apex is the
            // cones touching there or coinciding.
            AxisPose::Coaxial { slack, .. } => match signs_across(
                [(
                    "section_cone_coaxial_apexes",
                    "section_cone_coaxial_apexes_floor",
                    (apex2 - apex).dot(cone.a).abs(),
                )],
                slack,
                band,
            ) {
                Ok(_) => essential_pair(true, true),
                Err(tan) => tan.into(),
            },
            AxisPose::Parallel { .. } => {
                let other = Cone::new(apex2, d, half_angle2, u_ref2);
                cone_cone(&cone, &other, true, reach, band)
            }
            // As against a cylinder, the ruling charts take an offset
            // neither reading claims.
            AxisPose::Undecided(_) | AxisPose::Other => {
                let other = Cone::new(apex2, d, half_angle2, u_ref2);
                cone_cone(&cone, &other, false, reach, band)
            }
        },
        _ => Section::Intractable,
    }
}

/// A cone's carrier, read once: apex, unit axis, half-angle `α` and
/// `(sin α, cos α)`, and the unit seam direction square to the axis.
struct Cone<T: Real> {
    apex: Point3<T>,
    a: Vec3<T>,
    alpha: T,
    sc: (T, T),
    u_ref: Vec3<T>,
}

impl<T: Real> Cone<T> {
    fn new(apex: Point3<T>, axis: Vec3<T>, alpha: T, u_ref: Vec3<T>) -> Self {
        let frame = geom::square_frame(axis, u_ref);
        Self {
            apex,
            a: frame.axis,
            alpha,
            sc: alpha.sin_cos(),
            u_ref: frame.u_ref,
        }
    }

    /// The unit direction of the generator line at the radial direction
    /// `r` (unit, `⊥ a`): `t > 0` along it is the `v > 0` nappe.
    fn generator(&self, r: Vec3<T>) -> Vec3<T> {
        let (s, c) = self.sc;
        self.a * c + r * s
    }

    /// A point's signed distance from the double carrier, in metres.
    fn elevation(&self, p: Point3<T>) -> T {
        geom_brep::cone_elevation(self.apex, self.a, self.alpha, None, p)
    }

    /// The farthest the pair's reach stands from the apex.
    fn reach_lever(&self, reach: Reach<T>) -> T {
        (reach.centre - self.apex).norm() + reach.radius
    }

    /// The generator lines as a ruling chart: the azimuth `θ` from
    /// `u_ref`, `t` along the generator. Its reach speed is the turn's at
    /// the reach's farthest distance from the apex.
    fn chart(&self, reach: Reach<T>) -> ruling::Chart<T> {
        let (s, c) = self.sc;
        let zero = Vec3::new(T::zero(), T::zero(), T::zero());
        let lever = self.reach_lever(reach);
        ruling::Chart {
            base: self.apex,
            offset: ruling::Swing::fixed(zero),
            dir: ruling::Swing::round([self.a * c, self.u_ref * s, self.a.cross(self.u_ref) * s]),
            reach_speed: lever * s,
            lever: lever + lever,
        }
    }

    fn quadric(&self) -> ruling::Quadric<T> {
        ruling::Quadric::Cone {
            apex: self.apex,
            a: self.a,
            sc: self.sc,
        }
    }
}

/// The section as two ruling charts read it, one on each carrier: the
/// bounded components with the first chart's witnesses and each
/// chart's class, and the unbounded ones. Charts that disagree on a
/// count refuse under `row`.
fn charted<T: Real>(
    first: Result<ruling::Reading<T>, &'static str>,
    second: Result<ruling::Reading<T>, &'static str>,
    row: &'static str,
) -> Section<T> {
    let (first, second) = match (first, second) {
        (Ok(x), Ok(y)) => (x, y),
        (Err(name), _) | (_, Err(name)) => return Section::Tangent(name),
    };
    if first.bounded.len() != second.bounded.len() || first.unbounded != second.unbounded {
        return Section::Tangent(row);
    }
    let line = Component::closed_form(true, [false, false], None);
    let parts: Vec<_> = first
        .bounded
        .iter()
        .map(|&w| Component::closed_form(false, [first.essential, second.essential], Some(w)))
        .chain(core::iter::repeat_n(line, first.unbounded))
        .collect();
    let single = parts.len() == 1;
    Section::Components { parts, single }
}

/// **Cone × a cylinder in any pose but coaxial or parallel**, the wall
/// about the unit `d` through `o`, radius `rc` (the ruling charts,
/// `section_cert/ruling.rs`). The apex off the wall and no generator
/// along the axis are decided first: the one makes `p₀` a nonzero
/// constant on the cone's chart, the other keeps `p₂ > 0` on it and a
/// nonzero constant on the wall's, so neither chart has an unbounded
/// component. The wall's chart is anchored at the axis point nearest
/// the apex.
fn cone_cylinder<T: Decide>(
    cone: &Cone<T>,
    o: Point3<T>,
    d: Vec3<T>,
    rc: T,
    reach: Reach<T>,
    band: Band,
) -> Section<T> {
    let (_, c) = cone.sc;
    let (off, dist) = perp_to(cone.apex - o, d);
    let foot = cone.apex - off;
    let lever = cone.reach_lever(reach);
    if let Err(tan) = signs([("section_cone_cylinder_apex", dist - rc)], band) {
        return tan.into();
    }
    match sign(
        "section_cone_cylinder_aperture",
        Margin::levered(d.dot(cone.a).abs() - c, lever + lever),
        band,
    ) {
        Some(Sign::Positive | Sign::Negative) => {}
        _ => return Section::Tangent("section_cone_cylinder_aperture"),
    }
    // A chart of the cylinder for the ruling reader, picked off the axes.
    // No output keeps it, but the reader's verdict may read it
    // (`work/issues/a-section-certificates-witness-point-is-picked-off-the-axes.md`).
    let (f1, f2) = d.orthonormal_basis();
    let zero = Vec3::new(T::zero(), T::zero(), T::zero());
    let wall = ruling::Chart {
        base: foot,
        offset: ruling::Swing::round([zero, f1 * rc, f2 * rc]),
        dir: ruling::Swing::fixed(d),
        reach_speed: rc,
        lever: lever + lever,
    };
    let on_cone = ruling::read(
        &cone.chart(reach),
        &ruling::Quadric::Cylinder { o: foot, d, r: rc },
        &ruling::ChartRows {
            fold: "section_cone_cylinder_fold",
            asymptote: "section_cone_cylinder_asymptote",
            precision: "section_cone_cylinder_precision",
            walk: RULING_WALK,
        },
        band,
    );
    let on_wall = ruling::read(
        &wall,
        &cone.quadric(),
        &ruling::ChartRows {
            fold: "section_cylinder_cone_fold",
            asymptote: "section_cylinder_cone_asymptote",
            precision: "section_cylinder_cone_precision",
            walk: RULING_WALK,
        },
        band,
    );
    charted(on_cone, on_wall, "section_cone_cylinder_charts")
}

/// **Cone × cone in any pose but coaxial** (the ruling charts,
/// `section_cert/ruling.rs`), each cone's generators against the
/// other; `parallel`, their axes are parallel. Each apex is decided off
/// the other carrier first (a common apex is both): an apex on the
/// other cone is a node of the section.
///
/// Parallel axes and one aperture (`cos²α₁ − cos²α₂`, levered by the
/// charts' longer lever, in the band's zero) give the two forms one
/// quadratic part, so the section is a plane conic ([`cone_plane_pair`]);
/// the charts would read `p₂ ≡ 0` there, every root of `E = p₁²` double.
fn cone_cone<T: Decide>(
    one: &Cone<T>,
    two: &Cone<T>,
    parallel: bool,
    reach: Reach<T>,
    band: Band,
) -> Section<T> {
    const APEX: &str = "section_cone_pair_apex";
    const APERTURE: &str = "section_cone_pair_aperture";
    if let Err(tan) = signs(
        [
            (APEX, two.elevation(one.apex)),
            (APEX, one.elevation(two.apex)),
        ],
        band,
    ) {
        return tan.into();
    }
    if parallel {
        let lever = one.reach_lever(reach).max(two.reach_lever(reach));
        let gap = one.sc.1.powi(2) - two.sc.1.powi(2);
        match sign(APERTURE, Margin::levered(gap, lever + lever), band) {
            Some(Sign::Zero) => return cone_plane_pair(one, two, reach, band),
            Some(Sign::Positive | Sign::Negative) => {}
            None => return Section::Tangent(APERTURE),
        }
    }
    let rows = ruling::ChartRows {
        fold: "section_cone_pair_fold",
        asymptote: "section_cone_pair_asymptote",
        precision: "section_cone_pair_precision",
        walk: RULING_WALK,
    };
    let first = ruling::read(&one.chart(reach), &two.quadric(), &rows, band);
    let second = ruling::read(&two.chart(reach), &one.quadric(), &rows, band);
    charted(first, second, "section_cone_pair_charts")
}

/// **Two cones of one aperture on parallel axes.** With `M = ââᵀ −
/// cos²α I` their forms `(X − Aᵢ)ᵀM(X − Aᵢ)` differ by the linear
/// `−2w·(X − m)`, `w = M(A₁ − A₂)` and `m` the apexes' midpoint, so the
/// section is either cone's section by the plane `w·(X − m) = 0`
/// ([`cone_plane`] on the first). `w` is not zero: `M` is invertible and
/// the apexes are apart. The plane misses both apexes (it meets `A₁`
/// exactly where `(A₁ − A₂)ᵀM(A₁ − A₂) = 0`, the other apex on the
/// cone), and its aperture margin `|n·â| − sin α` is the same against
/// both cones, so each component is essential on both or on neither.
fn cone_plane_pair<T: Decide>(
    one: &Cone<T>,
    two: &Cone<T>,
    reach: Reach<T>,
    band: Band,
) -> Section<T> {
    let delta = one.apex - two.apex;
    let w = one.a * one.a.dot(delta) - delta * one.sc.1.powi(2);
    let mid = two.apex + delta * T::from_f64(0.5);
    match cone_plane(one, mid, unit(w), reach, band) {
        Section::Components { parts, single } => Section::Components {
            parts: parts
                .into_iter()
                .map(|p| Component {
                    essential_g: p.essential_f,
                    ..p
                })
                .collect(),
            single,
        },
        other => other,
    }
}

/// The subdivision rows every ruling chart walks under.
const RULING_WALK: geom_brep::half_angle::SubdivisionRows =
    geom_brep::half_angle::SubdivisionRows {
        clear: "section_ruling_clear",
        monotone: "section_ruling_monotone",
        side: "section_ruling_side",
        width: "section_ruling_width",
    };

/// **Cone × plane**, the plane through `p0` with unit normal `n`.
///
/// The aperture margin `μ = |n·a| − sin α` decides the conic: Positive,
/// the plane's directions miss the cone's asymptotic ones and it meets
/// one nappe in a closed curve, met once by every generator of that
/// nappe, so essential (an ellipse, or at the apex a point); Negative,
/// a hyperbola or two lines through the apex, every component
/// unbounded; Zero, a parabola or anything near one, each component
/// unbounded or essential. The apex's offset from the plane is never
/// read: every class holds at any offset, the ellipse's witness
/// included, and a small offset does not bound the ellipse (near the
/// parabola it runs `m_A / sin γ` along a generator).
///
/// `μ` is levered by how far a tilt can act within the pair's reach:
/// the reach's farthest distance from the apex (the generator's pivot)
/// plus its farthest distance from the plane (the plane's pivot is its
/// point nearest the reach's centre).
fn cone_plane<T: Decide>(
    cone: &Cone<T>,
    p0: Point3<T>,
    n: Vec3<T>,
    reach: Reach<T>,
    band: Band,
) -> Section<T> {
    let (s, _) = cone.sc;
    let lever = (reach.centre - cone.apex).norm()
        + (reach.centre - p0).dot(n).abs()
        + reach.radius
        + reach.radius;
    let na = n.dot(cone.a);
    match sign(
        "section_cone_plane_aperture",
        Margin::levered(na.abs() - s, lever),
        band,
    ) {
        Some(Sign::Negative) => {
            let c = Component::closed_form(true, [false, false], None);
            Section::Components {
                parts: vec![c, c],
                single: false,
            }
        }
        Some(Sign::Zero) => Section::Components {
            parts: vec![Component::closed_form(false, [true, false], None)],
            single: false,
        },
        None => Section::Tangent("section_cone_plane_aperture"),
        Some(Sign::Positive) => {
            // The witness is the vertex nearer the apex: the generator in
            // the meridian plane of `n` that leans toward it, which the
            // frame facing `a` names. Any generator meets the plane on the
            // ellipse's own nappe or its line's other half; this one does
            // at `g·n = cos(α − β)`, the largest over the meridian, which
            // on the ellipse's range `0 ≤ β < π/2 − α` is at least
            // `min(cos α, sin 2α)`. Only the witness's position divides
            // by it; no margin does.
            let facing = match sign(
                "section_cone_plane_facing",
                Margin::levered(na, lever),
                band,
            ) {
                Some(Sign::Positive) => n,
                Some(Sign::Negative) => -n,
                _ => return Section::Tangent("section_cone_plane_facing"),
            };
            let (across, width) = perp_to(facing, cone.a);
            let r = match sign(
                "section_cone_plane_meridian",
                Margin::levered(width, lever),
                band,
            ) {
                Some(Sign::Positive) => across / width,
                Some(Sign::Zero) => cone.u_ref,
                _ => return Section::Tangent("section_cone_plane_meridian"),
            };
            let g = cone.generator(r);
            let t = (p0 - cone.apex).dot(facing) / g.dot(facing);
            Section::Components {
                parts: vec![Component::closed_form(
                    false,
                    [true, false],
                    Some(cone.apex + g * t),
                )],
                single: true,
            }
        }
    }
}

/// **Cone × sphere**, the ball `(cs, rho)`.
///
/// With `δ = A − c_s`, the generator line `A + t·w` meets the sphere at
/// `t² + 2bt + k = 0`, `b = w·δ`, `k = |δ|² − ρ²`; `t > 0` is the `v > 0`
/// nappe. Over the azimuth `b` is extreme on the two generators in the
/// meridian plane of the centre, `w₊` (largest `b`) and `w₋`.
///
/// - The apex inside the ball: every generator line crosses it once on
///   each side of the apex, so each nappe carries one essential
///   component.
/// - The apex outside: a line's two roots share `sign(−b)`. Nappe `+`
///   is met on the generators with `b < −√k`: all of them (two essential
///   curves) when `w₊`'s line meets the ball with `b₊ < 0`, an arc of
///   them — ONE null loop — when only `w₋`'s does, none otherwise; nappe
///   `−` alike with the signs and the two lines exchanged.
///
/// Each margin is a length: the apex's distance to the sphere, and each
/// extreme line's distance to the centre against the radius (Zero is a
/// generator tangent to the ball, where an arc ends in a pinch). Every
/// component carries a witness on its extreme line.
fn cone_sphere<T: Decide>(cone: &Cone<T>, cs: Point3<T>, rho: T, band: Band) -> Section<T> {
    let delta = cone.apex - cs;
    let apex_out = match signs([("section_cone_sphere_apex", delta.norm() - rho)], band) {
        Ok([out]) => out,
        Err(tan) => return tan.into(),
    };
    let (across, offset) = perp_to(delta, cone.a);
    let r = match sign("section_cone_sphere_axis", Margin::of(offset), band) {
        Some(Sign::Positive) => across / offset,
        Some(Sign::Zero) => cone.u_ref,
        _ => return Section::Tangent("section_cone_sphere_axis"),
    };
    let (hi, lo) = (cone.generator(r), cone.generator(-r));
    // The line `w`'s `b`, and its roots `−b ± √(ρ² − d²)` with `d` the
    // line's distance from the centre.
    let roots = |w: Vec3<T>| {
        let b = delta.dot(w);
        let d = (delta - w * b).norm();
        let half = ((rho - d) * (rho + d)).sqrt();
        (b, d, [-b - half, -b + half])
    };
    let part = |w: Vec3<T>, t: T, essential: bool| {
        Component::closed_form(false, [essential, false], Some(cone.apex + w * t))
    };
    let (b_hi, d_hi, t_hi) = roots(hi);
    let (b_lo, d_lo, t_lo) = roots(lo);
    if !apex_out {
        return Section::Components {
            parts: vec![part(hi, t_hi[1], true), part(hi, t_hi[0], true)],
            single: false,
        };
    }
    // Whether each extreme line meets the ball, and on which side of the
    // apex: its roots' side is `−b`'s, and `|b| > √k > 0` wherever it
    // meets.
    let meets = |b: T, d: T, name: &'static str| -> Result<Option<bool>, Section<T>> {
        match signs([("section_cone_sphere_generator", rho - d)], band)? {
            [false] => Ok(None),
            [true] => match signs([(name, b)], band)? {
                [up] => Ok(Some(!up)),
            },
        }
    };
    let (on_hi, on_lo) = match (
        meets(b_hi, d_hi, "section_cone_sphere_side"),
        meets(b_lo, d_lo, "section_cone_sphere_side"),
    ) {
        (Ok(x), Ok(y)) => (x, y),
        (Err(tan), _) | (_, Err(tan)) => return tan,
    };
    let mut parts = Vec::new();
    // Nappe `+`: `w₊` on it means every generator is; `w₋` alone, an arc.
    match (on_hi == Some(true), on_lo == Some(true)) {
        (true, true) => {
            parts.extend([part(hi, t_hi[0], true), part(hi, t_hi[1], true)]);
        }
        (false, true) => parts.push(part(lo, t_lo[1], false)),
        (false, false) => {}
        (true, false) => return Section::Tangent("section_cone_sphere_generator"),
    }
    // Nappe `−`: `w₋`'s line meets it (roots negative) only if every
    // generator line does; `w₊`'s alone, an arc.
    match (on_lo == Some(false), on_hi == Some(false)) {
        (true, true) => {
            parts.extend([part(lo, t_lo[0], true), part(lo, t_lo[1], true)]);
        }
        (false, true) => parts.push(part(hi, t_hi[0], false)),
        (false, false) => {}
        (true, false) => return Section::Tangent("section_cone_sphere_generator"),
    }
    let single = parts.len() == 1;
    Section::Components { parts, single }
}

/// **Cone × a cylinder whose axis is parallel to the cone's**, at offset
/// `e` in the direction `toward`. Each ruling, at distance
/// `ρ₀ ∈ [|e − r_c|, e + r_c]` from the cone's axis, meets each nappe
/// once, at `h = ±ρ₀/τ`: two components, each a graph over the
/// cylinder's azimuth (essential on it), essential on the cone iff the
/// cylinder encloses the cone's axis. `e − r_c` Zero is a ruling through
/// the apex, read across the `slack` the pose admitted. The witnesses
/// sit on the ruling nearest the axis.
fn cone_parallel_cylinder<T: Decide>(
    cone: &Cone<T>,
    (e, slack): (T, T),
    toward: Vec3<T>,
    rc: T,
    band: Band,
) -> Section<T> {
    let (s, c) = cone.sc;
    let outside = match signs_across(
        [(
            "section_cone_cylinder_apex",
            "section_cone_cylinder_apex_floor",
            e - rc,
        )],
        slack,
        band,
    ) {
        Ok([outside]) => outside,
        Err(tan) => return tan.into(),
    };
    let foot = cone.apex + toward * (e - rc);
    let h = (e - rc).abs() * c / s;
    let part = |h: T| Component::closed_form(false, [!outside, true], Some(foot + cone.a * h));
    Section::Components {
        parts: vec![part(h), part(-h)],
        single: false,
    }
}

/// **Cylinder × cylinder.**
#[allow(clippy::too_many_arguments)]
fn cylinder_cylinder<T: Decide>(
    o1: Point3<T>,
    d1: Vec3<T>,
    r1: T,
    o2: Point3<T>,
    d2: Vec3<T>,
    r2: T,
    reach: Reach<T>,
    band: Band,
) -> Section<T> {
    let cross = d1.cross(d2);
    let sin = cross.norm();
    match sign(
        "section_cylinder_axes_tilt",
        Margin::levered(sin, reach.radius + reach.radius),
        band,
    ) {
        // Parallel axes: common rulings, or nothing — unless the two
        // walls are ONE carrier, where the section is the surface itself
        // rather than a curve. L1 is an argument about curves, so a
        // coincident pair refuses as the torus analog does (R-tan),
        // read across the reach. The second wall is read as its image
        // turned about the pivot `at` onto the first's direction. A point
        // of the wall in the reach stands at a station `s` along its axis
        // from `at`, `|s|` at most the reach's radius (`at` is the
        // centre's foot), and off the image's axis by `|s|·sin` plus its
        // radius times between `cos` and one: radial motion slides along
        // the wall, so the swing is `sin·radius + r₂·sin²`.
        Some(Sign::Zero) => {
            const COINCIDENT: &str = "section_cylinder_pair_coincident";
            let at = reach.ball().foot_on(o2, d2);
            let (_, offset) = perp_to(at - o1, d1);
            return match decide_across(
                [COINCIDENT, "section_cylinder_pair_coincident_floor"],
                offset + (r1 - r2).abs(),
                sin * reach.radius + r2 * sin.powi(2),
                band,
            ) {
                Ok(Sign::Positive) => Section::Components {
                    parts: vec![Component::closed_form(true, [false, false], None)],
                    single: false,
                },
                Ok(Sign::Zero) => Section::Tangent(COINCIDENT),
                Ok(Sign::Negative) => unreachable!(
                    "`{COINCIDENT}` read a sum of a norm and an `abs` Negative on its floor, \
                     which is shrunk toward zero, never past it"
                ),
                Err(diag) => Section::Tangent(diag.predicate.unwrap_or(COINCIDENT)),
            };
        }
        Some(Sign::Positive) => {}
        // A near-parallel pair can carry a long null saddle loop.
        Some(Sign::Negative) | None => return Section::Tangent("section_cylinder_axes_tilt"),
    }
    let m = cross / sin;
    let delta = (o2 - o1).dot(m);
    const REACH: &str = "section_cylinder_pair_reach";
    let [reach, nest] = match signs(
        [
            (REACH, r1 + r2 - delta.abs()),
            ("section_cylinder_pair_nest", delta.abs() - (r1 - r2).abs()),
        ],
        band,
    ) {
        Ok(x) => x,
        // Touching outside, on the common perpendicular from axis 1's
        // foot, `r₁ / (r₁ + r₂)` of the way to axis 2: the walls' deepest
        // point when they cross, which the half-turn about that line (it
        // maps each wall to itself) fixes inside the loop. Touching
        // inside (the nest margin) is a pinch: the thinner wall leaves
        // the fatter one on both sides of the touch — unless the radii
        // are equal and the axes meet ([`equal_crossing_axes`]).
        Err(tan) if tan.name == "section_cylinder_pair_nest" && tan.undecided.is_none() => {
            return equal_crossing_axes((o1, d1, r1), (o2, d2, r2), (m, delta), band)
                .unwrap_or_else(|| tan.into());
        }
        Err(tan) => {
            return touch(tan, &[REACH], || {
                axis_foot((o1, d1), (o2, d2), sin.powi(2)) + m * (delta * r1 / (r1 + r2))
            });
        }
    };
    match (reach, nest) {
        (false, _) => none(),
        // Two loops, each encircling the thinner wall's axis.
        (true, false) => match signs([("section_cylinder_pair_thin", r1 - r2)], band) {
            Ok([true]) => essential_pair(false, true),
            Ok([false]) => essential_pair(true, false),
            Err(tan) => tan.into(),
        },
        // The single null saddle loop, witnessed on the arc's middle
        // ruling of cylinder 2: the one on cylinder 1's side of it,
        // `cos(θ − φ) = −sign(δ₀)`. `nest` puts `δ₀` off zero, so its
        // sign decides; a `Zero` or undecided one refuses R-tan all
        // the same.
        (true, true) => {
            let q = match signs([("section_cylinder_pair_side", delta)], band) {
                Ok([true]) => o2 - m * r2,
                Ok([false]) => o2 + m * r2,
                Err(tan) => return tan.into(),
            };
            let (av, bv) = (d2.cross(d1), (q - o1).cross(d1));
            let (qa, qb, qc) = (av.dot(av), av.dot(bv), bv.dot(bv) - r1.powi(2));
            let t = (-qb + (qb.powi(2) - qa * qc).sqrt()) / qa;
            lone(q + d2 * t)
        }
    }
}

/// **Two equal cylinders whose axes meet**: the nest margin
/// `|δ₀| − |r₁ − r₂|` decided `Zero` because both terms are, the radii
/// equal (`section_cylinder_pair_equal`) and the axes coplanar
/// (`section_cylinder_pair_meet`, `δ₀` itself). The section is the two
/// bisector-plane ellipses, crossing at the nodes `p ± r·n̂` (`p` the
/// axes' meeting point, `n̂` along `m`, the common perpendicular): ONE
/// connected component, essential on both walls, since each ellipse is
/// a plane section encircling each axis. Witnessed at a node. `None`
/// where either margin is not `Zero`: a nest pinch at unequal radii
/// (the thinner wall inside the fatter, touching), refused R-tan by the
/// caller.
fn equal_crossing_axes<T: Decide>(
    (o1, d1, r1): (Point3<T>, Vec3<T>, T),
    (o2, d2, r2): (Point3<T>, Vec3<T>, T),
    (m, delta): (Vec3<T>, T),
    band: Band,
) -> Option<Section<T>> {
    let zero = |name, m| sign(name, Margin::of(m), band) == Some(Sign::Zero);
    if !(zero("section_cylinder_pair_equal", r1 - r2) && zero("section_cylinder_pair_meet", delta))
    {
        return None;
    }
    let p = axis_foot((o1, d1), (o2, d2), d1.cross(d2).norm_squared());
    Some(Section::Components {
        parts: vec![Component::closed_form(
            false,
            [true, true],
            Some(p + m * r1.abs()),
        )],
        single: true,
    })
}

/// The foot on axis 1 `(o1, d1)` of its common perpendicular with the
/// non-parallel axis 2 `(o2, d2)`, both directions unit and `sin2` the
/// squared sine between them: where the axes meet, when they do.
fn axis_foot<T: geom_core::Real>(
    (o1, d1): (Point3<T>, Vec3<T>),
    (o2, d2): (Point3<T>, Vec3<T>),
    sin2: T,
) -> Point3<T> {
    let w = o1 - o2;
    let b = d1.dot(d2);
    o1 + d1 * ((b * d2.dot(w) - d1.dot(w)) / sin2)
}

/// **Cylinder × plane**: an ellipse encircling the wall, or rulings.
/// Neither lies inside a wall that describes, and the count is not
/// certified, so W4 is never read.
fn cylinder_plane<T: Real>() -> Section<T> {
    Section::Components {
        parts: vec![Component::closed_form(false, [true, false], None)],
        single: false,
    }
}

/// **Sphere × plane**: one circle.
fn sphere_plane<T: Decide>(
    cs: Point3<T>,
    rho: T,
    p0: Point3<T>,
    n: Vec3<T>,
    band: Band,
) -> Section<T> {
    let d = (cs - p0).dot(n);
    const REACH: &str = "section_sphere_plane_reach";
    match signs([(REACH, rho - d.abs())], band) {
        Ok([true]) => {
            let rad = ((rho - d.abs()) * (rho + d.abs())).sqrt();
            // A witness picked off the axes. No output keeps it, but W3
            // and the no-event decision read it, so which point decides
            // can follow the computing frame
            // (`work/issues/a-section-certificates-witness-point-is-picked-off-the-axes.md`).
            let (b1, _) = n.orthonormal_basis();
            lone(cs - n * d + b1 * rad)
        }
        Ok([false]) => none(),
        // The foot of the centre: the circle's centre.
        Err(tan) => touch(tan, &[REACH], || cs - n * d),
    }
}

/// **Sphere × sphere**: one circle.
fn sphere_sphere<T: Decide>(c1: Point3<T>, r1: T, c2: Point3<T>, r2: T, band: Band) -> Section<T> {
    let dd = (c2 - c1).norm();
    const REACH: &str = "section_sphere_pair_reach";
    const NEST: &str = "section_sphere_pair_nest";
    match signs([(REACH, r1 + r2 - dd), (NEST, dd - (r1 - r2).abs())], band) {
        Ok([true, true]) => {
            let k = (c2 - c1) / dd;
            let x = (dd.powi(2) + r1.powi(2) - r2.powi(2)) / (dd + dd);
            let rad = ((r1 - x) * (r1 + x)).sqrt();
            // Picked off the axes, as `sphere_plane`'s witness is.
            let (b1, _) = k.orthonormal_basis();
            lone(c1 + k * x + b1 * rad)
        }
        Ok(_) => none(),
        // Touching outside or inside, the circle's centre on the line of
        // centres. Inside, the touch is the extreme of the distance from
        // one centre over the other sphere only while the centres stand
        // apart: the next critical value is `2·dd` away.
        Err(tan)
            if tan.name == NEST
                && sign("section_sphere_pair_apart", Margin::of(dd), band)
                    != Some(Sign::Positive) =>
        {
            tan.into()
        }
        Err(tan) => touch(tan, &[REACH, NEST], || {
            let k = (c2 - c1) / dd;
            c1 + k * ((dd.powi(2) + r1.powi(2) - r2.powi(2)) / (dd + dd))
        }),
    }
}

/// **A pair the section value builds** ([`geom_brep::pair_section()`]),
/// classified from it, its first surface `F`: a tangency it decided is a
/// [`Touch`] at the point where the section is one point and R-tan
/// otherwise; a pose question undecided, a closed-form count that
/// pinches, and two tori decided one, R-tan, named by the decision; a
/// crossing the closed form cannot order R-order; a radius not definitely
/// positive, decided or not, and a torus off its ring convention,
/// R-reach: an operand's size is no tangency, and the pair has no arm
/// for it. A section a closed form counts is its components, each with
/// whether it winds about each carrier and its witness, certified single
/// when there is one, and the face whose chart boundary it reaches in a
/// channel the chart does not close, on a side that is not collapsed
/// (W1′, `pcurves::chart_side_bounds`), read at `band` as the loop walk
/// reads it; one no closed form counts is the march's, read by
/// `marched` ([`of_carried`]).
pub(super) fn of_pair_section<T: Decide>(
    read: &Result<geom_brep::PairSection<T>, geom_brep::SectionError>,
    band: Band,
    marched: impl FnOnce(&geom_brep::PairSection<T>) -> Section<T>,
) -> Section<T> {
    use geom_brep::{ComponentShape, Components, SectionError};
    let section = match read {
        Ok(section) => section,
        // Every row the value decides goes through `k_stats`, which names
        // the predicate it escalates; one with no name is the value's
        // door itself.
        Err(SectionError::Escalated(diag)) => {
            return Section::Tangent(diag.predicate.unwrap_or("pair_section"));
        }
        Err(SectionError::Pinched { predicate }) => return Section::Tangent(predicate),
        Err(SectionError::Unordered { predicate }) => return Section::Unordered(predicate),
        // Two surfaces decided one share a carrier: no component count
        // is certified, as at a tangency. Two tori (`tt_coincident`), two
        // cylinders and a spline wall on a plane
        // (`section_nurbs_plane_coincident`) are decided one; the refusal
        // carries no pose, so it is named by the value.
        Err(SectionError::CoincidentSurfaces) => {
            return Section::Tangent("pair_section_coincident");
        }
        // A kind pair the value serves refuses these on no arm of its
        // own: an operand's size (decided or not), a torus off its ring,
        // and the refusals of the arms it does not build.
        Err(
            SectionError::RadiusEscalated { .. }
            | SectionError::DegenerateOperand { .. }
            | SectionError::DegenerateTorus(_)
            | SectionError::WrongLane { .. }
            | SectionError::RoutesToGeneralRung { .. }
            | SectionError::NoRegion { .. }
            | SectionError::BeyondOperandExtent { .. }
            | SectionError::Carrier(_)
            | SectionError::Spiric(_)
            | SectionError::Unaccounted { .. },
        ) => return Section::Intractable,
        Err(SectionError::NoMarch { scalar }) => return Section::NoLane(scalar),
    };
    let (f, g) = section.pose().operands();
    let components = match section.components() {
        Components::Counted(components) => components,
        Components::Marched => return marched(section),
        // Every component is open, ending on some of these sides (some
        // side, by the variant's contract): each clears by W1′ where
        // every side listed bounds its surface; otherwise the list is the
        // reader's to take.
        &Components::Reaching(reached) => {
            let [on_f, on_g] = reached.rims();
            if !(bounding(on_f, f, band).all(|b| b) && bounding(on_g, g, band).all(|b| b)) {
                return marched(section);
            }
            let side = if on_f.any() { Side::F } else { Side::G };
            return Section::Components {
                parts: vec![Component {
                    unbounded: false,
                    essential_f: false,
                    essential_g: false,
                    witness: None,
                    reaches: Some(side),
                    pad: 0.0,
                }],
                single: false,
            };
        }
    };
    if let Some(zero) = section.tangency() {
        return match components.as_slice() {
            [c] if c.shape == ComponentShape::Point => match c.witness {
                Some(at) => Section::Touch(Touch {
                    name: zero.decision.name(),
                    at,
                }),
                None => Section::Tangent(zero.decision.name()),
            },
            _ => Section::Tangent(zero.decision.name()),
        };
    }
    Section::Components {
        parts: components
            .iter()
            .map(|c| Component {
                unbounded: false,
                essential_f: c.winds[0],
                essential_g: c.winds[1],
                witness: c.witness,
                reaches: match c.shape {
                    // Only rim ends reach a chart side; a cut end or a
                    // node end clears nothing here, so such a component
                    // is read as a loop is, by its witness.
                    ComponentShape::Open {
                        ends:
                            geom_brep::ComponentEnds {
                                rims: [on_f, on_g], ..
                            },
                    } => {
                        if bounding(on_f, f, band).any(|b| b) {
                            Some(Side::F)
                        } else if bounding(on_g, g, band).any(|b| b) {
                            Some(Side::G)
                        } else {
                            None
                        }
                    }
                    ComponentShape::Loop | ComponentShape::Point => None,
                },
                pad: c.pad,
            })
            .collect(),
        single: components.len() == 1,
    }
}

/// **Whether each side of `surface`'s chart `reach` names bounds a face
/// on it** (W1′, [`crate::pcurves::chart_side_bounds`]), side by side.
fn bounding<'a, T: Decide>(
    reach: geom_brep::ChartReach,
    surface: &'a geom::Surface<T>,
    band: Band,
) -> impl Iterator<Item = bool> + 'a {
    reach
        .sides()
        .map(move |side| crate::pcurves::chart_side_bounds(surface, side, band))
}

/// **A section no closed form counts, read from its march** (W0 to W4
/// on the march's components): no branch is W0, its receipt being the
/// march's exhaustiveness; each loop winds as its turns about each
/// surface's axis and core read off the loop do (W2,
/// `geom_brep::pair_section::winds`); its witness is its carrier's
/// middle point, within its tube of the component, so W3 places it with
/// that pad ([`Component::pad`]); and the march's count is certified, so
/// a lone loop is W4's. At
/// a scalar with no march it is R-lane; a march or loop reading that
/// refused is R-march.
pub(super) fn of_carried<T: Decide>(
    carried: Result<&geom_brep::CarriedSection<T>, &geom_brep::CarryError>,
) -> Section<T> {
    use geom_brep::CarryError;
    let section = match carried {
        Ok(section) => section,
        Err(CarryError::NoLane { scalar }) => return Section::NoLane(scalar),
        Err(
            CarryError::Unminted(_)
            | CarryError::March(_)
            | CarryError::Winds(_)
            | CarryError::PadShort { .. }
            | CarryError::Escalated(_)
            | CarryError::Desync(_)
            | CarryError::Seat(_),
        ) => return Section::Unmarched,
    };
    let components = section.components();
    Section::Components {
        parts: components
            .iter()
            .map(|c| Component {
                unbounded: false,
                essential_f: c.winds[0],
                essential_g: c.winds[1],
                witness: Some(c.witness),
                reaches: None,
                pad: c.pad,
            })
            .collect(),
        single: components.len() == 1,
    }
}

/// **The per-pair rule** over a classified section.
///
/// `evented` says whether the reduction recorded an event on this pair;
/// `describes` asks W2's question of a face (cached by the caller);
/// `place` places a witness point in `[F, G]` against the band it is
/// handed, `None` for no verdict: `band` itself for a closed-form point,
/// and `band` padded by twice a marched witness's pad
/// ([`Component::pad`]).
///
/// # Errors
///
/// The pair's [`Refusal`].
pub(crate) fn certify<T: Decide>(
    section: &Section<T>,
    evented: bool,
    band: Band,
    mut describes: impl FnMut(Side) -> Sheet,
    mut place: impl FnMut(Point3<T>, Band) -> [Option<FaceContainment>; 2],
) -> Result<Vec<Cleared>, Refusal> {
    let (parts, single) = match section {
        Section::Components { parts, single } => (parts, *single),
        Section::Tangent(name) => return Err(Refusal::Tangent(name)),
        Section::Unordered(name) => return Err(Refusal::Unordered(name)),
        Section::Touch(t) if !evented => {
            let placed = place(t.at, band);
            return [Side::F, Side::G]
                .into_iter()
                .zip(placed)
                .find(|&(_, at)| at == Some(FaceContainment::Out))
                .map(|(side, _)| vec![Cleared::TouchOut(side)])
                .ok_or(Refusal::Tangent(t.name));
        }
        Section::Touch(t) => return Err(Refusal::Tangent(t.name)),
        Section::Intractable => return Err(Refusal::Reach),
        Section::NoLane(scalar) => return Err(Refusal::NoLane(scalar)),
        Section::Unmarched => return Err(Refusal::Unmarched),
    };
    let mut out = Vec::with_capacity(parts.len());
    for c in parts {
        if c.unbounded {
            out.push(Cleared::Unbounded);
            continue;
        }
        if let Some(side) = c.reaches {
            out.push(Cleared::Reaches(side));
            continue;
        }
        let mut unread = false;
        let mut essential = |on: bool, side: Side| {
            on && match describes(side) {
                Sheet::One => true,
                Sheet::Not => false,
                Sheet::Unread => {
                    unread = true;
                    false
                }
            }
        };
        if essential(c.essential_f, Side::F) {
            out.push(Cleared::Essential(Side::F));
            continue;
        }
        if essential(c.essential_g, Side::G) {
            out.push(Cleared::Essential(Side::G));
            continue;
        }
        let undecided = if unread {
            Refusal::Unread
        } else {
            Refusal::Undecided
        };
        if single && evented {
            out.push(Cleared::LoneEvented);
            continue;
        }
        let Some(p) = c.witness else {
            return Err(undecided);
        };
        let [at_f, at_g] = match band.padded(2.0 * c.pad) {
            Ok(padded) => place(p, padded),
            Err(_) => [None, None],
        };
        if at_f == Some(FaceContainment::Out) {
            out.push(Cleared::Out(Side::F));
            continue;
        }
        if at_g == Some(FaceContainment::Out) {
            out.push(Cleared::Out(Side::G));
            continue;
        }
        if !evented && at_f == Some(FaceContainment::In) && at_g == Some(FaceContainment::In) {
            return Err(Refusal::Loop);
        }
        return Err(undecided);
    }
    Ok(out)
}

mod ruling;

#[cfg(test)]
mod cone_pair_search;
#[cfg(test)]
mod cone_search;
#[cfg(test)]
mod search_kit;
#[cfg(test)]
#[path = "section_cert_axis_pose_rows.rs"]
mod section_cert_axis_pose_rows;
#[cfg(test)]
#[path = "section_cert_cone_pair_rows.rs"]
mod section_cert_cone_pair_rows;
#[cfg(test)]
#[path = "section_cert_cone_rows.rs"]
mod section_cert_cone_rows;
#[cfg(test)]
#[path = "section_cert_parallel_tori_rows.rs"]
mod section_cert_parallel_tori_rows;
#[cfg(test)]
#[path = "section_cert_rows.rs"]
pub(super) mod section_cert_rows;
#[cfg(test)]
#[path = "section_cert_spline_pair_rows.rs"]
mod section_cert_spline_pair_rows;
#[cfg(test)]
#[path = "section_cert_spline_rows.rs"]
mod section_cert_spline_rows;
#[cfg(test)]
#[path = "section_cert_square_rows.rs"]
mod section_cert_square_rows;
#[cfg(test)]
mod square_search;
