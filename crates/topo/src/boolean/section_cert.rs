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
//! - **W0, apart**: the classification returns no component.
//! - **W1, unbounded**: faces are compact, so `γ ⊄ F`.
//! - **W2, essential**: `γ` is essential on the carrier of `X` (it winds
//!   about the axis, or around the tube) and `chart_boundary(X)` returns
//!   `Ok`: `X`'s loops lift into one sheet of the chart's cover, so every
//!   closed curve in `int X` lifts to a closed curve with zero winding in
//!   every periodic channel, and `γ ⊄ int X`. (A section curve that is
//!   not closed — a ruling — is not inside a compact face at all.) That
//!   is all the certificate needs: a `γ` that meets `F ∩ G` and is not
//!   inside `int F ∩ int G` carries an event (L1). This is checked per FACE, never
//!   assumed of a kind: a seamless periodic band — one face closing on
//!   itself with no meridian edge — fails the check and loses W2.
//! - **W3, a witness point `Out`**: one closed-form point of `γ`,
//!   certified outside `F` or outside `G` (by the chart trim on a curved
//!   face, by `contfp` on a plane), so `γ ⊄ F ∩ G`.
//! - **W4, a lone evented component**: `Σ` has exactly one component and
//!   the pair has an event. The event's vertex lies on `Σ` within the
//!   band, hence on `γ`, and on an edge of `F` or `G`, so
//!   `γ ⊄ int F ∩ int G`.
//! - **The no-event decision**: with no event on the pair,
//!   `γ ∩ F ∩ G` has no relative boundary point, so it is empty or all
//!   of `γ`, and one point decides. A witness strictly inside both faces
//!   is a **definite interior loop** (R-loop). Anything else, a witness
//!   on a boundary or with no verdict, refuses undecided (R-undec).
//!
//! One witness point per component is complete on the arms below: a
//! lone component is cleared by W4 or decided by its point, and a
//! component essential or unbounded on a carrier is cleared by W1 or W2.
//! A section with a null component beside others clears each null loop
//! by its witness or, with an event on the pair, refuses R-undec, since
//! the event may lie on another component: cone × sphere, where a ball
//! beside the apex can meet each nappe in one null loop (with the apex
//! outside the ball no nappe can carry essential curves while the other
//! carries a loop); a cone against an oblique cylinder or another cone
//! in general pose, whose section can hold several loops null on both
//! carriers; and torus × square cylinder, where a wall biting the tube's
//! inner side leaves a null loop beside two essential ones and one
//! cutting its top leaves two null loops.
//!
//! # The refusals
//!
//! - **R-reach**: a kind pair or pose with no arm (torus against a
//!   cylinder neither parallel nor square to its axis, a non-coaxial
//!   torus; a cone against a non-coaxial torus; a NURBS or approximated
//!   face paired with anything but a plane, and a NURBS face whose
//!   control net a plane cuts). Every pair with a face that is not a
//!   plane is examined; two planes meet in a line, which W1 clears.
//! - **R-tan**: a classification margin `Zero` or undecided — a
//!   tangency, where components pinch and the count is not certified.
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
//! - **Witness points are within rounding of `Σ`** on `f64`, and
//!   enclosures of it on the interval lane; containment is decided on the
//!   band, and a point it cannot decide is not a witness.
//! - **Lone-vertex loops.** The sweep is edge-driven, so a vertex of a
//!   `LoopBoundary::Empty` loop is never examined. That does not break
//!   L1 on its own: `F` surrounds its lone vertex on every side, so the
//!   vertex is a relative boundary point of `γ ∩ F ∩ G` only where it
//!   also lies on `∂G`, whose edges the sweep does examine; and
//!   `chart_boundary` reads the face's region as its cycles bound it, so
//!   W2 covers the vertex too. What the sweep cannot see is a component
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
//! - **Torus × plane, torus × sphere** (one classification). In the
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
//!   Every component carries a witness ([`torus_square_cylinder`]).
//! - **Cylinder × cylinder** (common-perpendicular offset `δ₀`):
//!   `|δ₀| + r_min < r_max` gives two loops encircling the thinner
//!   axis; `|r₁ − r₂| < |δ₀| < r₁ + r₂` a single null saddle loop,
//!   witnessed on the arc's middle ruling; parallel axes give rulings
//!   (lines).
//! - **Cylinder × plane**: an ellipse encircling the wall, or rulings.
//!   Either way the section cannot lie in a wall that describes. The
//!   count is not certified (two rulings, or one ellipse), so W4 is
//!   never read here and the pair clears by W2 or refuses.
//! - **Sphere × {plane, sphere}**: one circle. **Sphere × cylinder**
//!   (axis offset `e`): one loop when `|e − ρc| < ρs < e + ρc`,
//!   witnessed on the nearest ruling; two loops encircling the cylinder
//!   when `e + ρc < ρs`.
//! - **NURBS × plane**: W0 when the control net's every point is
//!   certified strictly on one side of the plane — the patch lies in the
//!   net's convex hull, its weights being positive. Any other pose has
//!   no arm: nothing counts the section's components on a spline.
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

use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};

use super::carrier_eq::line_tilt;
use super::contain::FaceContainment;
use crate::validate::decide;
use geom_brep::ExtentBall;
use geom_brep::intersect::decide_across;

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
    /// One point of the component, in closed form.
    pub witness: Option<Point3<T>>,
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
        }
    }
}

/// The unit vector along `v`.
fn unit<T: Real>(v: Vec3<T>) -> Vec3<T> {
    v / v.norm()
}

/// `v`'s part perpendicular to the unit `a`, and its norm. Taken as
/// `a × (v × a)`, which stands square to `a` to the rounding of its own
/// size: `v − a·(a·v)` keeps a component along `a` of `v`'s size times
/// the rounding, which dividing by a small norm (a plane or a centre
/// near the axis) turns into a direction off the plane square to `a`.
fn perp_to<T: Real>(v: Vec3<T>, a: Vec3<T>) -> (Vec3<T>, T) {
    let perp = a.cross(v.cross(a));
    (perp, perp.norm())
}

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

/// A margin [`signs`] could not read as a side: decided `Zero`, or
/// undecided.
#[derive(Clone, Copy, Debug)]
struct Pinch {
    name: &'static str,
    /// The margin's place in the list [`signs`] decided.
    index: usize,
    /// `None` where the margin decided `Zero`; its diagnostics where it
    /// was undecided.
    undecided: Option<geom_core::Indeterminate>,
}

impl<T: Real> From<Pinch> for Section<T> {
    fn from(p: Pinch) -> Self {
        Section::Tangent(p.name)
    }
}

impl Pinch {
    /// A [`Touch`] at `at()` when the pinch is one of `names` decided
    /// `Zero`, the one reading under which the carriers share at most one
    /// loop about it, bounding a connected region where they stand within
    /// the margin; R-tan otherwise.
    fn touch<T: Real>(self, names: &[&'static str], at: impl FnOnce() -> Point3<T>) -> Section<T> {
        if self.undecided.is_none() && names.contains(&self.name) {
            Section::Touch(Touch {
                name: self.name,
                at: at(),
            })
        } else {
            self.into()
        }
    }
}

/// Reads every row by `read`, which names it and gives its verdict; the
/// first `Zero` or undecided one is the pair's tangency, named by the row
/// left undecided where one was.
fn first_pinch<R, const N: usize>(
    rows: [R; N],
    read: impl Fn(R) -> (&'static str, Result<Sign, Indeterminate>),
) -> Result<[bool; N], Pinch> {
    let mut out = [false; N];
    for (index, (slot, row)) in out.iter_mut().zip(rows).enumerate() {
        let (name, verdict) = read(row);
        let (name, undecided) = match verdict {
            Ok(Sign::Positive) => {
                *slot = true;
                continue;
            }
            Ok(Sign::Negative) => continue,
            Ok(Sign::Zero) => (name, None),
            Err(diag) => (diag.predicate.unwrap_or(name), Some(diag)),
        };
        return Err(Pinch {
            name,
            index,
            undecided,
        });
    }
    Ok(out)
}

/// Decides every margin in `ms` ([`first_pinch`]).
fn signs<T: Decide, const N: usize>(
    ms: [(&'static str, T); N],
    band: Band,
) -> Result<[bool; N], Pinch> {
    first_pinch(ms, |(name, m)| (name, decide(name, Margin::of(m), band)))
}

/// [`signs`], each row `(far, floor, datum)` read across the reach beside
/// `slack` (module docs, the sum; [`decide_across`]).
fn signs_across<T: Decide, const N: usize>(
    rows: [(&'static str, &'static str, T); N],
    slack: T,
    band: Band,
) -> Result<[bool; N], Pinch> {
    first_pinch(rows, |(far, floor, m)| {
        (far, decide_across([far, floor], m, slack, band))
    })
}

fn none<T: Real>() -> Section<T> {
    Section::Components {
        parts: Vec::new(),
        single: false,
    }
}

fn lone<T: Real>(witness: Point3<T>) -> Section<T> {
    Section::Components {
        parts: vec![Component {
            unbounded: false,
            essential_f: false,
            essential_g: false,
            witness: Some(witness),
        }],
        single: true,
    }
}

fn essential_pair<T: Real>(on_f: bool, on_g: bool) -> Section<T> {
    let c = Component {
        unbounded: false,
        essential_f: on_f,
        essential_g: on_g,
        witness: None,
    };
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
pub(crate) fn classify<T: Decide>(
    f: &geom::Surface<T>,
    g: &geom::Surface<T>,
    reach: Reach<T>,
    band: Band,
) -> Section<T> {
    use geom::Surface as S;
    match (f, g) {
        (
            S::Torus { .. },
            S::Plane { .. }
            | S::Sphere { .. }
            | S::Cylinder { .. }
            | S::Cone { .. }
            | S::Torus { .. },
        ) => torus_pair(f, g, reach, band),
        (
            S::Plane { .. } | S::Sphere { .. } | S::Cylinder { .. } | S::Cone { .. },
            S::Torus { .. },
        ) => swapped(torus_pair(g, f, reach, band)),
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
        (
            &S::Sphere { center, radius, .. },
            &S::Cylinder {
                origin,
                axis,
                radius: rc,
                ..
            },
        ) => sphere_cylinder(center, radius, origin, unit(axis), rc, band),
        (
            &S::Cylinder {
                origin,
                axis,
                radius: rc,
                ..
            },
            &S::Sphere { center, radius, .. },
        ) => swapped(sphere_cylinder(
            center,
            radius,
            origin,
            unit(axis),
            rc,
            band,
        )),
        (S::Nurbs(patch), &S::Plane { origin, normal, .. })
        | (&S::Plane { origin, normal, .. }, S::Nurbs(patch)) => {
            nurbs_plane(patch, origin, unit(normal), band)
        }
        _ => Section::Intractable,
    }
}

/// **NURBS × plane: W0 or no arm.** A patch lies in the convex hull of
/// its control net (the weights are strictly positive), so a net whose
/// every point is certified strictly on one side of the plane meets it
/// nowhere. A net the plane cuts, touches or cannot place has no arm:
/// the section's components are not classified, and the pair refuses on
/// reach.
fn nurbs_plane<T: Decide>(
    patch: &geom::NurbsSurface<T>,
    origin: Point3<T>,
    n: Vec3<T>,
    band: Band,
) -> Section<T> {
    let sides: Vec<_> = patch
        .control()
        .iter()
        .map(|&c| {
            sign(
                "section_nurbs_plane_hull",
                Margin::of((c - origin).dot(n)),
                band,
            )
        })
        .collect();
    let apart = |s| !sides.is_empty() && sides.iter().all(|&t| t == Some(s));
    if apart(Sign::Positive) || apart(Sign::Negative) {
        none()
    } else {
        Section::Intractable
    }
}

/// The torus arms: `t` is a torus, `p` its partner.
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
        geom::Surface::Sphere { center, radius, .. } => {
            torus_sphere(c, a, big_r, r, center, radius, band)
        }
        geom::Surface::Cylinder {
            origin,
            axis: d,
            radius: rc,
            ..
        } => match axis_pose(c, a, origin, unit(d), |_| T::zero(), reach, band) {
            Pose::Coaxial { slack } => {
                // The cylinder's meridian is the line ρ = ρc against the
                // tube circle.
                match signs_across(
                    [(
                        "section_torus_coaxial_wall",
                        "section_torus_coaxial_wall_floor",
                        r - (rc - big_r).abs(),
                    )],
                    slack,
                    band,
                ) {
                    Ok([true]) => essential_pair(true, true),
                    Ok([false]) => none(),
                    Err(tan) => tan.into(),
                }
            }
            Pose::Parallel { e, toward, slack } => {
                torus_parallel_cylinder(c, a, big_r, r, (e, slack), toward, rc, band)
            }
            Pose::Undecided(name) => Section::Tangent(name),
            Pose::Other => match square_pose(c, a, origin, unit(d), reach, band) {
                Some(wall) => torus_square_cylinder(c, a, big_r, r, &wall, rc, band),
                None => Section::Intractable,
            },
        },
        geom::Surface::Torus {
            center: c2,
            axis: a2,
            major_radius: big_r2,
            minor_radius: r2,
            ..
        } => {
            if geom::require_ring_torus(big_r2, r2, band).is_err() {
                return Section::Intractable;
            }
            let a2 = unit(a2);
            // The centre's height is read off `c2` itself, which the turn
            // onto the torus axis lifts by its station from the pivot
            // times `1 − cos`, at most `sin²`.
            let turn = |t: &Turn<T>| (c2 - t.at).dot(a2).abs() * t.tilt.powi(2);
            match axis_pose(c, a, c2, a2, turn, reach, band) {
                Pose::Coaxial { slack } => {
                    let z2 = (c2 - c).dot(a);
                    let d = hypot(big_r2 - big_r, z2);
                    match signs_across(
                        [
                            (
                                "section_torus_coaxial_tube_reach",
                                "section_torus_coaxial_tube_reach_floor",
                                r + r2 - d,
                            ),
                            (
                                "section_torus_coaxial_tube_nest",
                                "section_torus_coaxial_tube_nest_floor",
                                d - (r - r2).abs(),
                            ),
                        ],
                        slack,
                        band,
                    ) {
                        Ok([true, true]) => essential_pair(true, true),
                        Ok(_) => none(),
                        Err(tan) => tan.into(),
                    }
                }
                Pose::Undecided(name) => Section::Tangent(name),
                Pose::Parallel { .. } | Pose::Other => Section::Intractable,
            }
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
            match axis_pose(c, a, apex, d, turn, reach, band) {
                Pose::Coaxial { slack } => {
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
                Pose::Undecided(name) => Section::Tangent(name),
                Pose::Parallel { .. } | Pose::Other => Section::Intractable,
            }
        }
        _ => Section::Intractable,
    }
}

/// How a partner's axis stands to the torus axis.
enum Pose<T: Real> {
    /// One axis line: the arm's coaxial reading stands at most `slack` off
    /// the partner anywhere the section can matter.
    Coaxial { slack: T },
    /// Parallel, at offset `e` read at the pivot, with `toward` the unit
    /// direction from the torus axis to the partner's: the parallel
    /// reading stands at most `slack` off the partner anywhere the section
    /// can matter.
    Parallel { e: T, toward: Vec3<T>, slack: T },
    /// The tilt routes the axes parallel, and the offset reads neither
    /// coaxial nor parallel across the reach: the row left undecided.
    Undecided(&'static str),
    /// Tilted, or the tilt undecided.
    Other,
}

/// The lever of a tilt read about the pivot `at` (module docs, the lever
/// and its pivot): the reach's farthest distance from it, doubled.
fn lever<T: Real>(at: Point3<T>, reach: Reach<T>) -> T {
    reach.ball().lever_from(at) * T::from_f64(2.0)
}

/// Where the turn onto the torus axis is read: the pivot, the axes' tilt
/// (the sine between them), and the partner axis's offset at the pivot.
struct Turn<T: Real> {
    at: Point3<T>,
    tilt: T,
    offset: T,
}

/// Decides the axis pose (module docs, the lever and its pivot, the sum).
/// The tilt, levered from the pivot `at`, only routes. Below it the
/// partner is read as its image under the turn about `at` that takes its
/// axis onto `a`'s direction, which moves it by at most the chord between
/// the axes ([`line_tilt`]) times the reach's farthest distance from `at`
/// anywhere the section can matter: the swing, plus `turn`, how far the
/// arm's own reading moves with the image where it reads a point other
/// than the pivot (zero for a reading that is the image). The offset at
/// the pivot is decided across the reach beside that swing: coaxial on
/// the far row, parallel on the floor.
fn axis_pose<T: Decide>(
    c: Point3<T>,
    a: Vec3<T>,
    o: Point3<T>,
    d: Vec3<T>,
    turn: impl FnOnce(&Turn<T>) -> T,
    reach: Reach<T>,
    band: Band,
) -> Pose<T> {
    let at = reach.ball().foot_on(o, d);
    let tilt = a.cross(d).norm();
    if sign(
        "section_axes_tilt",
        Margin::levered(tilt, lever(at, reach)),
        band,
    ) != Some(Sign::Zero)
    {
        return Pose::Other;
    }
    let (perp, e) = perp_to(at - c, a);
    let swing = line_tilt(a, d) * reach.ball().lever_from(at)
        + turn(&Turn {
            at,
            tilt,
            offset: e,
        });
    const OFFSET: &str = "section_axes_offset";
    match decide_across([OFFSET, "section_axes_offset_floor"], e, swing, band) {
        Ok(Sign::Zero) => Pose::Coaxial { slack: e + swing },
        Ok(Sign::Positive) => Pose::Parallel {
            e,
            toward: perp / e,
            slack: swing,
        },
        Ok(Sign::Negative) => unreachable!(
            "`{OFFSET}` read a distance's floor Negative: the offset is a norm, and its floor \
             is shrunk toward zero, never past it"
        ),
        Err(diag) => Pose::Undecided(diag.predicate.unwrap_or(OFFSET)),
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
            tan.touch(&[NEAR, FAR], || in_pi(sigma * big_r + d * s, d * na))
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

/// **Torus × sphere**, the ball `(cs, rho)`.
fn torus_sphere<T: Decide>(
    c: Point3<T>,
    a: Vec3<T>,
    big_r: T,
    r: T,
    cs: Point3<T>,
    rho: T,
    band: Band,
) -> Section<T> {
    const NEAR: &str = "section_torus_sphere_near_tube";
    const FAR: &str = "section_torus_sphere_far_tube";
    let w = cs - c;
    let ca = w.dot(a);
    let (s, in_pi) = meridian(c, a, w);
    let dist = |x: T| hypot(x, ca);
    let (d_near, d_far) = (dist(s - big_r), dist(s + big_r));
    let cuts = |d: T| (d + r - rho).min(rho - (d - r).abs());
    let [near, far] = match signs([(NEAR, cuts(d_near)), (FAR, cuts(d_far))], band) {
        Ok(x) => x,
        Err(tan) => {
            return torus_sphere_touch(tan, [big_r, r], (s, ca), rho, [d_near, d_far], band)
                .map_or_else(
                    || tan.into(),
                    |(x, y)| tan.touch(&[NEAR, FAR], || in_pi(x, y)),
                );
        }
    };
    match (near, far) {
        (true, true) => essential_pair(true, false),
        (false, false) => match signs(
            [(
                "section_torus_sphere_between",
                (rho - d_near - r).min(d_far - r - rho),
            )],
            band,
        ) {
            Ok([true]) => essential_pair(true, false),
            Ok([false]) => none(),
            Err(tan) => tan.into(),
        },
        (true, false) | (false, true) => {
            let sigma = if near { T::one() } else { -T::one() };
            let d = if near { d_near } else { d_far };
            // Circle against circle in Π: T_σ centred (σR, 0) radius r,
            // the sphere's great circle centred (s, c_a) radius ρ.
            let (ex, ey) = ((s - sigma * big_r) / d, ca / d);
            let along = (d.powi(2) + r.powi(2) - rho.powi(2)) / (d + d);
            let half = ((r - along) * (r + along)).sqrt();
            lone(in_pi(
                sigma * big_r + ex * along - ey * half,
                ey * along + ex * half,
            ))
        }
    }
}

/// **Where a sphere tangent to the tube circle `T_σ` the pinch names
/// touches the torus, if at one point**: `(x, y)` in `Π`. The torus is
/// `[R, r]`, the sphere's centre `(s, ca)` in `Π`, and `d_near`, `d_far`
/// its distances from the tube centres `(±R, 0)`.
///
/// The section is the level `ρ` of `f = |x − cs|` on the torus. With
/// `cs` off the axis (`s > 0`), `f`'s critical points are the four
/// points of `T₊` and `T₋` on their normals through `cs`: on `T_σ`, its
/// nearest point to `cs` at `|d_σ − r|` and its farthest at `d_σ + r`.
/// The sphere is tangent at whichever of the two stands nearer `ρ`. That
/// point is a touch when its value is `f`'s strict minimum or maximum
/// over the torus, clear of the other three by a decided margin: in
/// every pose the band admits, `{f ≤ ρ}` (or `{f ≥ ρ}`) is then one disc
/// about it or empty, on which the carriers stand within the band of
/// each other. It must also stand on the tube's outer half, where the
/// torus is elliptic.
///
/// Which of the two is a choice, not a decision: `ρ` stands within the
/// band of one of them and they are `2·min(d_σ, r)` apart, so wherever
/// the choice is in doubt the other stands within the band too and the
/// extreme margin refuses.
fn torus_sphere_touch<T: Decide>(
    tan: Pinch,
    [big_r, r]: [T; 2],
    (s, ca): (T, T),
    rho: T,
    [d_near, d_far]: [T; 2],
    band: Band,
) -> Option<(T, T)> {
    let (sigma, d, d_other) = if tan.index == 0 {
        (T::one(), d_near, d_far)
    } else {
        (-T::one(), d_far, d_near)
    };
    let positive = |name, m: T| sign(name, Margin::of(m), band) == Some(Sign::Positive);
    // The midpoint of T_σ's two values is max(d_σ, r); `kappa` is +1 at
    // the nearest point, −1 at the farthest.
    let split = rho - d.max(r);
    let (nearest, farthest) = ((d - r).abs(), d + r);
    let kappa = split.select_le_zero(T::one(), -T::one());
    let value = split.select_le_zero(nearest, farthest);
    let [o1, o2, o3] = [
        split.select_le_zero(farthest, nearest),
        (d_other - r).abs(),
        d_other + r,
    ]
    .map(|v| kappa * (v - value));
    // T_σ's point along ±(cs − C_σ), C_σ = (σR, 0).
    let (ux, uy) = ((s - sigma * big_r) / d, ca / d);
    let (x, y) = (sigma * big_r + kappa * r * ux, kappa * r * uy);
    (positive("section_torus_sphere_touch_extreme", o1.min(o2).min(o3))
        && positive("section_torus_sphere_touch_elliptic", sigma * x - big_r))
    .then_some((x, y))
}

/// **Torus × a cylinder whose axis is parallel to the torus's**, at
/// offset `e` in the direction `toward`, each row read across the
/// `slack` the pose admitted.
#[allow(clippy::too_many_arguments)]
fn torus_parallel_cylinder<T: Decide>(
    c: Point3<T>,
    a: Vec3<T>,
    big_r: T,
    r: T,
    (e, slack): (T, T),
    toward: Vec3<T>,
    rc: T,
    band: Band,
) -> Section<T> {
    let (rho_min, rho_max) = ((e - rc).abs(), e + rc);
    const NEAR_OUTER: &str = "section_torus_offset_wall_near_outer";
    let (lo, hi) = (big_r - r, big_r + r);
    let [min_over_lo, min_under_hi, max_over_lo, max_under_hi] = match signs_across(
        [
            (
                "section_torus_offset_wall_near_inner",
                "section_torus_offset_wall_near_inner_floor",
                rho_min - lo,
            ),
            (
                NEAR_OUTER,
                "section_torus_offset_wall_near_outer_floor",
                hi - rho_min,
            ),
            (
                "section_torus_offset_wall_far_inner",
                "section_torus_offset_wall_far_inner_floor",
                rho_max - lo,
            ),
            (
                "section_torus_offset_wall_far_outer",
                "section_torus_offset_wall_far_outer_floor",
                hi - rho_max,
            ),
        ],
        slack,
        band,
    ) {
        Ok(x) => x,
        // The wall's nearest ruling on the outer equator, the rest of
        // the wall beyond it: the extreme of the distance from the wall's
        // axis over the torus (its minimum beside the torus, its maximum
        // about it), at an elliptic point, clear of the next critical
        // value by `2r` beside the torus and by `min(2r, 2e)` about it,
        // both decided upstream: `r` by `torus_pair`'s
        // `geom::require_ring_torus`, `e` by `axis_pose`'s `Positive`
        // offset floor, the one pose that reaches this arm.
        Err(tan) if tan.name == NEAR_OUTER => {
            return tan.touch(&[NEAR_OUTER], || c + toward * (e - rc));
        }
        Err(tan) => return tan.into(),
    };
    let height = |rho: T| {
        let dr = rho - big_r;
        ((r - dr.abs()) * (r + dr.abs())).sqrt()
    };
    match (min_over_lo, min_under_hi, max_over_lo, max_under_hi) {
        // The whole ρ-range inside the tube's shadow: two loops, each
        // encircling the cylinder.
        (true, true, true, true) => essential_pair(false, true),
        // Wholly inside the hole, or wholly outside the torus.
        (_, _, false, _) | (_, false, _, _) => none(),
        // Across both bounds: two `(0,1)` loops on the torus.
        (false, true, true, false) => essential_pair(true, false),
        // Across the inner bound only: the null loop at θ = 0.
        (false, true, true, true) => lone(c + toward * rho_max + a * height(rho_max)),
        // Across the outer bound only: the null loop at θ = π.
        (true, true, true, false) => lone(c + toward * (e - rc) + a * height(rho_min)),
    }
}

/// **A cylinder whose axis stands square to the torus axis**, read in the
/// meridian plane `Π` through the torus centre normal to it: `d` the
/// wall's unit axis, `across = a × d`, and `(e, z0)` the axis's trace in
/// `Π`, `e` along `across` and `z0` along `a`. `slop` bounds how far the
/// rounding leaves `e` and `z0` from their exact values.
#[derive(Clone, Copy, Debug)]
struct SquareWall<T: Real> {
    d: Vec3<T>,
    across: Vec3<T>,
    e: T,
    z0: T,
    slop: T,
}

/// The ulps of its operands' size a coordinate of the trace carries.
const SQUARE_TRACE_ULPS: f64 = 2.0;

/// Decides whether the partner axis `(o, d)` stands square to the torus
/// axis `a`: `a·d` levered as [`axis_pose`] levers a tilt. Square, the
/// wall is read with its axis turned square to `a` about the pivot, which
/// moves it by less than the band anywhere the section can matter. An
/// undecided margin is not a pose this arm claims.
fn square_pose<T: Decide>(
    c: Point3<T>,
    a: Vec3<T>,
    o: Point3<T>,
    d: Vec3<T>,
    reach: Reach<T>,
    band: Band,
) -> Option<SquareWall<T>> {
    if sign(
        "section_torus_square_wall_tilt",
        Margin::levered(a.dot(d), lever(reach.ball().foot_on(o, d), reach)),
        band,
    ) != Some(Sign::Zero)
    {
        return None;
    }
    // The pivot's offset from the torus centre, summed from the stored
    // operands so that neither's distance from the origin is rounded in.
    let reach_along = (reach.centre - o).dot(d);
    let w = (o - c) + d * reach_along;
    let (along, n) = perp_to(d, a);
    let d = along / n;
    let across = a.cross(d);
    Some(SquareWall {
        d,
        across,
        e: w.dot(across),
        z0: w.dot(a),
        slop: ((o - c).norm() + reach_along.abs() + w.norm())
            * T::from_f64(SQUARE_TRACE_ULPS * f64::EPSILON),
    })
}

/// A point where the wall's trace `K` crosses a tube circle, `T₊` at
/// `y = R` (`plus`) or `T₋`, on its outer half (`|y| > R`) or its inner.
#[derive(Clone, Copy, Debug)]
struct Fold<T: Real> {
    plus: bool,
    y: T,
    z: T,
    outer: bool,
}

/// A point where `K` crosses an edge of the strip `|z| ≤ r` (`top`:
/// `z = r`), whether it stands over the torus's parallel there
/// (`|y| < R`), and its side of `K`'s centre (`right`: along `across`).
#[derive(Clone, Copy, Debug)]
struct Edge {
    top: bool,
    over: bool,
    right: bool,
}

const SQUARE_WALK: &str = "section_torus_square_wall_walk";
const SQUARE_ORDER: &str = "section_torus_square_wall_order";
const SQUARE_FOLD_HALF: &str = "section_torus_square_wall_fold_half";
const SQUARE_EDGE_OVER: &str = "section_torus_square_wall_edge_over";

/// **Torus × a cylinder whose axis is square to the torus's**, the wall
/// `wall` of radius `rc`.
///
/// `Π` holds the torus axis and stands normal to the wall's, so the wall
/// is the union of the lines `P + t·d` normal to `Π` through the points
/// `P = (Y, Z)` of its trace `K`, the circle of radius `rc` about
/// `(e, z0)`. Such a line stays at height `Z` and at distance
/// `√(t² + Y²)` from the torus axis, so it meets the torus where
/// `√(t² + Y²) = R ± √(r² − Z²)`: on each SHEET `ρ = R ± √(r² − Z²)` over
/// the strip `|Z| ≤ r`, at `t = ±√(ρ² − Y²)` where `ρ ≥ |Y|`. The sheets
/// join on the strip's edges, at `ρ = R`. So the section is the double
/// cover, `t ↦ −t`, of the curve `γ` of points `(P, sheet)` with
/// `ρ ≥ |Y|`, folded where `ρ = |Y|`: where `P` lies on a tube circle
/// `T±` (centred `(±R, 0)` in `Π`), on the outer sheet on the circle's
/// outer half and on the inner sheet on its inner half.
///
/// `γ`'s components are `K`'s arcs in the strip, each an oval running
/// out along the outer sheet and back along the inner, or with `K` inside
/// the open strip the two sheets as two loops. On each, `ρ ≥ |Y|` holds on
/// alternate runs between folds; a run between two folds lifts to ONE
/// closed component through `t = 0` at those two folds, and a fold-free
/// component wholly in `ρ > |Y|` lifts to TWO, one at `t > 0` and its
/// mirror. Their classes:
///
/// - a component through two folds is its own mirror, so the mirror acts
///   on it as a reflection with those two folds fixed. It is null on the
///   wall (an essential curve the mirror fixes would lie in `t = 0`), and
///   its tube angle retraces itself (`q = 0`). It winds once about the
///   torus axis iff its folds lie on different tube circles: it crosses
///   `Π` only there, on the half-planes `u = ±π/2`;
/// - a fold-free lift lies in one half-space `±t > 0`, where the torus is
///   a tube segment (`p = 0`). A loop of a sheet is essential on the
///   wall and stays on one half of the tube (null on the torus); an oval
///   is null on the wall, and winds once about the tube iff it runs from
///   one strip edge to the other.
///
/// Every fold and strip crossing is a circle meeting a circle or a line
/// in `Π`, in closed form; each component's witness is one of its folds,
/// or for a fold-free lift a point where its sheets stand apart. A margin
/// `Zero` or undecided where the carriers are tangent — `K` tangent to a
/// tube circle, or to a strip edge over the parallel, a fold too shallow
/// to place within the band, two crossings in one place along `K` —
/// refuses R-tan; a fold or strip crossing at a tube circle's top or
/// bottom, where they cross, refuses R-order.
fn torus_square_cylinder<T: Decide>(
    c: Point3<T>,
    a: Vec3<T>,
    big_r: T,
    r: T,
    wall: &SquareWall<T>,
    rc: T,
    band: Band,
) -> Section<T> {
    let &SquareWall {
        d,
        across,
        e,
        z0,
        slop,
    } = wall;
    let at = |y: T, z: T, t: T| c + across * y + a * z + d * t;
    let k = (e, z0, rc);
    let parts = (|| {
        let top = reaches_edge(
            "section_torus_square_wall_top",
            z0 + rc - r,
            (e, big_r),
            band,
        )?;
        let bottom = reaches_edge(
            "section_torus_square_wall_bottom",
            rc - z0 - r,
            (e, big_r),
            band,
        )?;
        let folds = square_folds([big_r, r], k, slop, band)?;
        if top || bottom {
            across_strip([big_r, r], k, [top, bottom], &folds, at, band)
        } else {
            inside_strip([big_r, r], k, &folds, at, band)
        }
    })();
    match parts {
        Ok(parts) => {
            let single = parts.len() == 1;
            Section::Components { parts, single }
        }
        Err(s) => s,
    }
}

/// `K`'s crossings with the tube circles `T±`: none or two each.
///
/// A fold is where `K` crosses a tube circle at the angle `θ` between
/// them, `sin θ = dist·half / (r·rc)` for centres `dist` apart and the
/// crossings `half` off their line. Where the angle is shallow the
/// rounding `slop` of `K`'s centre moves the crossing `slop / sin θ` along
/// both circles while it stays on each: a fold whose drift the band does
/// not decide `Zero` is a near-tangency at the fold (the carriers' normals
/// there stand `θ` apart) and refuses R-tan. A fold within the band of a
/// tube circle's top or bottom point lies on neither half; the carriers
/// cross there, and it refuses [`Section::Unordered`].
fn square_folds<T: Decide>(
    [big_r, r]: [T; 2],
    (e, z0, rc): (T, T, T),
    slop: T,
    band: Band,
) -> Result<Vec<Fold<T>>, Section<T>> {
    let mut out = Vec::new();
    for plus in [true, false] {
        let sigma = if plus { T::one() } else { -T::one() };
        let dy = e - sigma * big_r;
        let dist = hypot(dy, z0);
        let [cut] = signs(
            [(
                "section_torus_square_wall_tube",
                (rc + r - dist).min(dist - (rc - r).abs()),
            )],
            band,
        )?;
        if !cut {
            continue;
        }
        // Circle against circle: `T_σ` about `(σR, 0)` radius `r`, `K`
        // about `(e, z0)` radius `rc`, `dist` apart.
        let (uy, uz) = (dy / dist, z0 / dist);
        let along = (dist.powi(2) + (r - rc) * (r + rc)) / (dist + dist);
        let half = ((r - along) * (r + along)).sqrt();
        let drift = (slop + (e.abs() + big_r + rc) * T::from_f64(SQUARE_TRACE_ULPS * f64::EPSILON))
            * r
            * rc
            / (dist * half);
        if sign(
            "section_torus_square_wall_fold_drift",
            Margin::of(drift),
            band,
        ) != Some(Sign::Zero)
        {
            return Err(Section::Tangent("section_torus_square_wall_fold_drift"));
        }
        for s in [half, -half] {
            let (y, z) = (sigma * big_r + uy * along - uz * s, uz * along + uy * s);
            let [outer] = signs([(SQUARE_FOLD_HALF, sigma * y - big_r)], band)
                .map_err(|_| Section::Unordered(SQUARE_FOLD_HALF))?;
            out.push(Fold { plus, y, z, outer });
        }
    }
    Ok(out)
}

/// `(x² + y²)^½`.
fn hypot<T: Real>(x: T, y: T) -> T {
    (x.powi(2) + y.powi(2)).sqrt()
}

/// Whether `K` reaches past a strip edge's line, by the margin `m` of its
/// reach there. `K` within the band of tangent to the line touches it at
/// `(e, ±r)`: over the parallel (`|e| ≤ R`) the wall is tangent to the
/// torus there, a pinch, R-tan; beyond it no line of the wall near that
/// point meets the torus, and the section is the one of `K` held just
/// short of the line, a pose the band admits.
fn reaches_edge<T: Decide>(
    name: &'static str,
    m: T,
    (e, big_r): (T, T),
    band: Band,
) -> Result<bool, Section<T>> {
    match signs([(name, m)], band) {
        Ok([past]) => Ok(past),
        Err(_)
            if sign(
                "section_torus_square_wall_edge_clear",
                Margin::of(e.abs() - big_r),
                band,
            ) == Some(Sign::Positive) =>
        {
            Ok(false)
        }
        Err(pinch) => Err(pinch.into()),
    }
}

/// The radius of the sheet `ρ = R ± √(r² − z²)` at height `z`.
fn sheet<T: Real>([big_r, r]: [T; 2], z: T, outer: bool) -> T {
    let h = ((r - z.abs()) * (r + z.abs())).sqrt();
    if outer { big_r + h } else { big_r - h }
}

/// The two lifts of a fold-free component of `γ` through `(y, z)` on the
/// sheet of radius `rho > |y|`.
fn lifts<T: Real>(
    (y, z, rho): (T, T, T),
    essential: [bool; 2],
    at: impl Fn(T, T, T) -> Point3<T>,
) -> [Component<T>; 2] {
    let t = ((rho - y) * (rho + y)).sqrt();
    [t, -t].map(|t| Component {
        unbounded: false,
        essential_f: essential[0],
        essential_g: essential[1],
        witness: Some(at(y, z, t)),
    })
}

fn through<T: Real>(
    f: Fold<T>,
    essential_f: bool,
    at: impl Fn(T, T, T) -> Point3<T>,
) -> Component<T> {
    Component {
        unbounded: false,
        essential_f,
        essential_g: false,
        witness: Some(at(f.y, f.z, T::zero())),
    }
}

/// **`K` inside the open strip**: the sheets are two loops over all of
/// `K`, the outer meeting the folds on outer halves, the inner those on
/// inner halves. A tube circle's two folds share a half: `K` meets the
/// segment `y = ±R` of the strip only inside that circle's disc. On the
/// outer loop each tube circle's folds bound `K`'s arc inside its disc,
/// in `ρ ≥ |Y|`: one component each. On the inner loop, one circle's
/// folds bound the rest of `K`, which then stands between the discs;
/// both circles' folds bound the two arcs of `K` between the discs, each
/// from `T₊` to `T₋` and winding about the torus axis. (Folds on both
/// outer halves would need `K` to reach past `y = ±R` inside `|z| < r`,
/// `rc > R > r`.) On a fold-free loop `ρ − |Y|` keeps one sign, its zeros
/// being folds, so any point of `K` decides it: the arm reads the point
/// it witnesses with, `(e − rc, z0)`.
fn inside_strip<T: Decide>(
    ring: [T; 2],
    (e, z0, rc): (T, T, T),
    folds: &[Fold<T>],
    at: impl Fn(T, T, T) -> Point3<T> + Copy,
    band: Band,
) -> Result<Vec<Component<T>>, Section<T>> {
    let mut parts = Vec::new();
    for outer in [true, false] {
        let on = |plus: bool| -> Vec<Fold<T>> {
            folds
                .iter()
                .copied()
                .filter(|f| f.outer == outer && f.plus == plus)
                .collect()
        };
        match (on(true).as_slice(), on(false).as_slice()) {
            ([], []) => {
                let (y, rho) = (e - rc, sheet(ring, z0, outer));
                if signs([("section_torus_square_wall_sheet", rho - y.abs())], band)? == [true] {
                    parts.extend(lifts((y, z0, rho), [false, true], at));
                }
            }
            ([f, _], []) | ([], [f, _]) => parts.push(through(*f, false, at)),
            ([p1, p2], [_, _]) if !outer => {
                parts.push(through(*p1, true, at));
                parts.push(through(*p2, true, at));
            }
            _ => return Err(Section::Tangent(SQUARE_WALK)),
        }
    }
    Ok(parts)
}

/// **`K` across a strip edge**: `γ` is one oval per arc of `K` in the
/// strip. Read along `K` from a point outside the strip (its top when it
/// crosses `z = r`, else its bottom), the strip crossings alternate in
/// and out, and with two arcs (`K` across both edges) they are `K`'s
/// left and right flanks. Every fold lies inside its arc, in the open
/// strip, so folds are ordered among themselves only, never against a
/// strip crossing. An arc's oval runs out along the outer sheet
/// through its outer-half folds and back along the inner through its
/// inner-half ones; at its strip crossings both sheets lie in `ρ ≥ |Y|`
/// iff the crossing stands over the parallel (`|y| < R`).
fn across_strip<T: Decide>(
    ring: [T; 2],
    (e, z0, rc): (T, T, T),
    [top, bottom]: [bool; 2],
    folds: &[Fold<T>],
    at: impl Fn(T, T, T) -> Point3<T> + Copy,
    band: Band,
) -> Result<Vec<Component<T>>, Section<T>> {
    let [big_r, r] = ring;
    if top != bottom {
        let m = if top { r - (z0 - rc) } else { z0 + rc + r };
        if !reaches_edge("section_torus_square_wall_meets", m, (e, big_r), band)? {
            return Ok(Vec::new());
        }
    }
    let kappa = if top { T::one() } else { -T::one() };
    // The angle along `K` from the cut, `(−π, π)` everywhere off it.
    let psi = |y: T, z: T| (kappa * (y - e)).atan2(-kappa * (z - z0));
    let mut edges = Vec::new();
    for (edge, crossed) in [(true, top), (false, bottom)] {
        if !crossed {
            continue;
        }
        let z = if edge { r } else { -r };
        let h = (z - z0).abs();
        let w = ((rc - h) * (rc + h)).sqrt();
        for right in [true, false] {
            let y = if right { e + w } else { e - w };
            let [over] = signs([(SQUARE_EDGE_OVER, big_r - y.abs())], band)
                .map_err(|_| Section::Unordered(SQUARE_EDGE_OVER))?;
            edges.push((
                psi(y, z),
                Edge {
                    top: edge,
                    over,
                    right,
                },
            ));
        }
    }
    along_k(&mut edges, rc, band)?;
    let arcs: Vec<[Edge; 2]> = edges.chunks(2).map(|p| [p[0].1, p[1].1]).collect();
    let mut on_arc: Vec<Vec<(T, Fold<T>)>> = vec![Vec::new(); arcs.len()];
    for &f in folds {
        let k = if arcs.len() == 1 {
            0
        } else {
            let [right] = signs([("section_torus_square_wall_fold_side", f.y - e)], band)?;
            usize::from(arcs[0][0].right != right)
        };
        on_arc[k].push((psi(f.y, f.z), f));
    }
    let mut parts = Vec::new();
    for ([start, end], mut on) in arcs.into_iter().zip(on_arc) {
        along_k(&mut on, rc, band)?;
        // Out along the outer sheet, back along the inner.
        let (outward, inner): (Vec<Fold<T>>, Vec<Fold<T>>) =
            on.into_iter().map(|(_, f)| f).partition(|f| f.outer);
        let crossed_out = outward.len();
        let ring_order: Vec<Fold<T>> = outward.into_iter().chain(inner.into_iter().rev()).collect();
        if (crossed_out % 2 == 0) != (start.over == end.over) || ring_order.len() % 2 == 1 {
            return Err(Section::Tangent(SQUARE_WALK));
        }
        if ring_order.is_empty() {
            if start.over {
                // Edge to edge: `K`'s point at `z = 0`, where the sheets
                // stand `2r` apart; else the arc's extreme.
                let essential = start.top != end.top;
                let (y, z) = if essential {
                    let h = z0.abs();
                    let w = ((rc - h) * (rc + h)).sqrt();
                    (if start.right { e + w } else { e - w }, T::zero())
                } else {
                    (e, z0 - kappa * rc)
                };
                parts.extend(lifts((y, z, sheet(ring, z, true)), [essential, false], at));
            }
            continue;
        }
        // The runs in `ρ ≥ |Y|` pair the folds, the first run wrapping
        // through `start` when it stands over the parallel.
        let n = ring_order.len();
        let skip = usize::from(start.over);
        for k in (0..n).step_by(2) {
            let (f1, f2) = (ring_order[(k + skip) % n], ring_order[(k + skip + 1) % n]);
            parts.push(through(f1, f1.plus != f2.plus, at));
        }
    }
    Ok(parts)
}

/// Sorts points of `K` by their angle along it, every final neighbour
/// pair decided apart (as arc length): two in one place within the band
/// refuse R-tan.
fn along_k<T: Decide, X: Copy>(xs: &mut [(T, X)], rc: T, band: Band) -> Result<(), Section<T>> {
    for i in 1..xs.len() {
        let mut j = i;
        while j > 0 {
            match sign(
                SQUARE_ORDER,
                Margin::levered(xs[j - 1].0 - xs[j].0, rc),
                band,
            ) {
                Some(Sign::Positive) => xs.swap(j - 1, j),
                Some(Sign::Negative) => break,
                _ => return Err(Section::Tangent(SQUARE_ORDER)),
            }
            j -= 1;
        }
    }
    Ok(())
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
        } => match axis_pose(apex, cone.a, origin, unit(d), |_| T::zero(), reach, band) {
            // The meridians meet at `h = ±r_c/τ`: a parallel per nappe.
            Pose::Coaxial { .. } => essential_pair(true, true),
            Pose::Parallel { e, toward, slack } => {
                cone_parallel_cylinder(&cone, (e, slack), toward, rc, band)
            }
            // The ruling charts read the pose as it stands, with nothing
            // bridged, so an offset neither reading claims is theirs.
            Pose::Undecided(_) | Pose::Other => {
                cone_cylinder(&cone, origin, unit(d), rc, reach, band)
            }
        },
        geom::Surface::Cone {
            apex: apex2,
            axis: d,
            half_angle: half_angle2,
            u_ref: u_ref2,
        } => match axis_pose(
            apex,
            cone.a,
            apex2,
            unit(d),
            // The apexes' gap is read off `apex2` itself, which the turn
            // onto the first axis moves along it by its station from the
            // pivot times `1 − cos`, at most `sin²`.
            |t| (apex2 - t.at).dot(unit(d)).abs() * t.tilt.powi(2),
            reach,
            band,
        ) {
            // The meridians `ρ = |h|τ₁` and `ρ = |h − d|τ₂` meet in one or
            // two parallels, each essential on both; a common apex is the
            // cones touching there or coinciding.
            Pose::Coaxial { slack } => match signs_across(
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
            Pose::Parallel { .. } => {
                let other = Cone::new(apex2, d, half_angle2, u_ref2);
                cone_cone(&cone, &other, true, reach, band)
            }
            // As against a cylinder, the ruling charts take an offset
            // neither reading claims.
            Pose::Undecided(_) | Pose::Other => {
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
        let a = unit(axis);
        let (across, width) = perp_to(u_ref, a);
        Self {
            apex,
            a,
            alpha,
            sc: alpha.sin_cos(),
            u_ref: across / width,
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
    let line = Component {
        unbounded: true,
        essential_f: false,
        essential_g: false,
        witness: None,
    };
    let parts: Vec<_> = first
        .bounded
        .iter()
        .map(|&w| Component {
            unbounded: false,
            essential_f: first.essential,
            essential_g: second.essential,
            witness: Some(w),
        })
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
const RULING_WALK: crate::boolean::circle_roots::SubdivisionRows =
    crate::boolean::circle_roots::SubdivisionRows {
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
            let c = Component {
                unbounded: true,
                essential_f: false,
                essential_g: false,
                witness: None,
            };
            Section::Components {
                parts: vec![c, c],
                single: false,
            }
        }
        Some(Sign::Zero) => Section::Components {
            parts: vec![Component {
                unbounded: false,
                essential_f: true,
                essential_g: false,
                witness: None,
            }],
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
                parts: vec![Component {
                    unbounded: false,
                    essential_f: true,
                    essential_g: false,
                    witness: Some(cone.apex + g * t),
                }],
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
    let part = |w: Vec3<T>, t: T, essential: bool| Component {
        unbounded: false,
        essential_f: essential,
        essential_g: false,
        witness: Some(cone.apex + w * t),
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
    let part = |h: T| Component {
        unbounded: false,
        essential_f: !outside,
        essential_g: true,
        witness: Some(foot + cone.a * h),
    };
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
                    parts: vec![Component {
                        unbounded: true,
                        essential_f: false,
                        essential_g: false,
                        witness: None,
                    }],
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
        // the fatter one on both sides of the touch.
        Err(tan) => {
            return tan.touch(&[REACH], || {
                let w = o1 - o2;
                let b = d1.dot(d2);
                let foot = o1 + d1 * ((b * d2.dot(w) - d1.dot(w)) / sin.powi(2));
                foot + m * (delta * r1 / (r1 + r2))
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

/// **Cylinder × plane**: an ellipse encircling the wall, or rulings.
/// Neither lies inside a wall that describes, and the count is not
/// certified, so W4 is never read.
fn cylinder_plane<T: Real>() -> Section<T> {
    Section::Components {
        parts: vec![Component {
            unbounded: false,
            essential_f: true,
            essential_g: false,
            witness: None,
        }],
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
            let (b1, _) = n.orthonormal_basis();
            lone(cs - n * d + b1 * rad)
        }
        Ok([false]) => none(),
        // The foot of the centre: the circle's centre.
        Err(tan) => tan.touch(&[REACH], || cs - n * d),
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
        Err(tan) => tan.touch(&[REACH, NEST], || {
            let k = (c2 - c1) / dd;
            c1 + k * ((dd.powi(2) + r1.powi(2) - r2.powi(2)) / (dd + dd))
        }),
    }
}

/// **Sphere × cylinder** (the sphere is `F`).
fn sphere_cylinder<T: Decide>(
    cs: Point3<T>,
    rho: T,
    o: Point3<T>,
    d: Vec3<T>,
    rc: T,
    band: Band,
) -> Section<T> {
    let w = cs - o;
    let foot = o + d * w.dot(d);
    let (perp, e) = perp_to(w, d);
    let q_min = (e - rc).abs();
    const REACH: &str = "section_sphere_cylinder_reach";
    let [reach, girdle] = match signs(
        [
            (REACH, rho - q_min),
            ("section_sphere_cylinder_girdle", e + rc - rho),
        ],
        band,
    ) {
        Ok(x) => x,
        // The touch is on the nearest ruling, at the foot's height: the
        // loop's centre, which the reflections through the plane of the
        // axis and the centre and across the foot fix. It is the extreme
        // of the distance from the axis over the sphere only while the
        // girdle margin, `2·min(e, ρc)` there, is decided: a ball about
        // the axis touches the wall all round. A girdle pinch is not a
        // touch.
        Err(tan) => {
            if tan.name == REACH
                && sign(
                    "section_sphere_cylinder_girdle",
                    Margin::of(e + rc - rho),
                    band,
                ) != Some(Sign::Positive)
            {
                return tan.into();
            }
            return tan.touch(&[REACH], || foot + perp / e * rc);
        }
    };
    match (reach, girdle) {
        (false, _) => none(),
        // The ball holds every ruling's nearest point: two loops, each
        // encircling the cylinder.
        (true, false) => essential_pair(false, true),
        // One loop, witnessed on the ruling nearest the centre.
        (true, true) => {
            let t = ((rho - q_min) * (rho + q_min)).sqrt();
            lone(foot + perp / e * rc + d * t)
        }
    }
}

/// **The per-pair rule** over a classified section.
///
/// `evented` says whether the reduction recorded an event on this pair;
/// `describes` asks `chart_boundary` of a face (cached by the caller);
/// `place` places a witness point in `[F, G]`, `None` for no verdict.
///
/// # Errors
///
/// The pair's [`Refusal`].
pub(crate) fn certify<T: Decide>(
    section: &Section<T>,
    evented: bool,
    mut describes: impl FnMut(Side) -> bool,
    mut place: impl FnMut(Point3<T>) -> [Option<FaceContainment>; 2],
) -> Result<Vec<Cleared>, Refusal> {
    let (parts, single) = match section {
        Section::Components { parts, single } => (parts, *single),
        Section::Tangent(name) => return Err(Refusal::Tangent(name)),
        Section::Unordered(name) => return Err(Refusal::Unordered(name)),
        Section::Touch(t) if !evented => {
            let placed = place(t.at);
            return [Side::F, Side::G]
                .into_iter()
                .zip(placed)
                .find(|&(_, at)| at == Some(FaceContainment::Out))
                .map(|(side, _)| vec![Cleared::TouchOut(side)])
                .ok_or(Refusal::Tangent(t.name));
        }
        Section::Touch(t) => return Err(Refusal::Tangent(t.name)),
        Section::Intractable => return Err(Refusal::Reach),
    };
    let mut out = Vec::with_capacity(parts.len());
    for c in parts {
        if c.unbounded {
            out.push(Cleared::Unbounded);
            continue;
        }
        if c.essential_f && describes(Side::F) {
            out.push(Cleared::Essential(Side::F));
            continue;
        }
        if c.essential_g && describes(Side::G) {
            out.push(Cleared::Essential(Side::G));
            continue;
        }
        if single && evented {
            out.push(Cleared::LoneEvented);
            continue;
        }
        let Some(p) = c.witness else {
            return Err(Refusal::Undecided);
        };
        let [at_f, at_g] = place(p);
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
        return Err(Refusal::Undecided);
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
#[path = "section_cert_rows.rs"]
pub(super) mod section_cert_rows;
#[cfg(test)]
#[path = "section_cert_square_rows.rs"]
mod section_cert_square_rows;
#[cfg(test)]
mod square_search;
