//! **The v1 measurement primitives** (ERROR-DESIGN E3, CONTACT-DESIGN
//! C5): closed forms over the carriers the referenced entities sit on.
//!
//! # The scope, stated honestly
//!
//! Every arm below is a CLOSED FORM with its lever named. There is no
//! general point-set distance here and no sampling: a carrier pair
//! this table has no row for refuses typed
//! ([`MeasureUnsupported`]) naming the pair class, and the general
//! pair arrives with the clearance machinery, not by silently
//! degrading to the nearest arm that compiles.
//!
//! | primitive | pair | closed form | lever |
//! |-----------|------|-------------|-------|
//! | `distance` | vertex x vertex | `‖p_b − p_a‖` | none — an exact norm |
//! | `distance` | vertex x plane face | `\|(p − o)·n̂\|` | none — a metre projection on a unit normal |
//! | `distance` | plane face x plane face | `\|(o_b − o_a)·n̂_a\|`, PARALLEL required | `bool_plane_parallel` at the centre separation |
//! | `distance` | cylinder face x cylinder face | axis-line distance minus nothing: `‖Δo − (Δo·â)â‖`, PARALLEL required | `carrier_cyl_axis_parallel` at the axis separation |
//! | `angle` | plane face x plane face | `atan2(‖n̂_a × n̂_b‖, n̂_a·n̂_b)` | none — a ratio of unit quantities |
//! | `angle` | line edge x line edge | `atan2(‖d̂_a × d̂_b‖, d̂_a·d̂_b)` | none |
//! | `gap` | plane face x plane face | `(o_i − o_o)·ŵ_o` (SIGNED), `ŵ_o` the outer face's OUTWARD normal (S10 sense folded, not the chart normal) | `bool_plane_parallel` |
//! | `gap` | sphere face x sphere face | `R − r − ‖Δc‖` | none |
//! | `gap` | cylinder face x cylinder face | `r_bore − r_pin − d` | `carrier_cyl_axis_parallel` |
//!
//! Everything else — a cone, a torus, a NURBS patch, a curved edge, a
//! whole body, and every mixed pair not listed — refuses.
//!
//! **`min_clearance` is not in that table and never will be** (M10-6).
//! It is the primitive the table's own escape clause names: there is no
//! closed form for the minimum distance between two trimmed regions, so
//! its answer comes from the E7 subdivision engine
//! ([`crate::clearance::min_separation`]) as a certified BRACKET, and
//! the value is computed in the node's wiring — where the bodies are —
//! rather than here. [`eval_measure`] intercepts that primitive before
//! this dispatch is consulted, and the arm below says so.
//!
//! # Which trileans this module consumes
//!
//! Two, and both are EXISTING funnel predicates called at their
//! existing margin shapes: `bool_plane_parallel`
//! (`Margin::levered(‖n̂_a × n̂_b‖, arm)`, the sine of the normals'
//! disagreement priced at a lever arm) and
//! `carrier_cyl_axis_parallel` (the same shape on the axes). No third
//! margined compare is minted here; a distance whose parallelism
//! escalates refuses typed rather than reporting a number whose
//! meaning depends on an undecided fact.
//!
//! **That arm is an UPPER BOUND ON THE EXTENT of the two operands
//! together, with no floor** (ERROR-DESIGN E3's amendment): the faces'
//! own reaches plus the separation of their reference points, which
//! contains their union. A parallelism verdict is consumed as "the
//! separation is constant ACROSS THESE FACES", and that is the length
//! the tilt has to be priced over — so the margin is scale-aware, a
//! sub-millimetre part is not measured against a metre it does not
//! span, and no absolute constant appears anywhere. See [`arm`] for the
//! bound, for why over-refusal is the safe direction, and for why a
//! carrier with no extent is refused structurally rather than floored.
//!
//! The C5 sign convention is binding and lives in exactly one place:
//! [`gap`]. `g > 0` is clearance, `g = 0` contact, `g < 0`
//! interference, for all three carrier pairs.

use geom::{Curve3, Surface};
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};
use topo::Body;

use crate::measure::{MeasureExpr, MeasureKind, MeasurePrimitive};
use crate::names::{EntityKey, EntityRef};

/// A carrier pair this unit has no closed form for, named by the pair
/// it was asked about (E3's honest refusal, never a guess).
///
/// The pair CLASS is what the message carries — "a cone and a plane",
/// not two arena keys — because that is the fact a reader can act on:
/// the general pair is the clearance machinery's, and no rewording of
/// the reference changes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MeasureUnsupported {
    /// Which primitive was asked.
    pub verb: &'static str,
    /// The first operand's carrier class.
    pub a: &'static str,
    /// The second operand's carrier class.
    pub b: &'static str,
}

impl core::fmt::Display for MeasureUnsupported {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "`{}` has no closed form for {} against {} — the v1 vocabulary is the pair table \
             in `eval::measure`, and the general pair belongs to the clearance machinery, \
             not to a degraded version of one of these arms",
            self.verb, self.a, self.b
        )
    }
}

impl core::error::Error for MeasureUnsupported {}

/// A referenced entity, resolved to the carrier the closed forms read.
///
/// This is a VALUE read of stored geometry — `topo::readback`'s rule 1
/// verbatim: nothing here decides what kind of carrier something is,
/// it reports what the body stores. The decisions happen in the arms
/// below, at named funnel predicates.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Carrier<T: geom_core::Real> {
    /// A vertex's stored position.
    Point(Point3<T>),
    /// A planar face's carrier: a point on it, its UNIT chart normal,
    /// and the face's OUTWARD normal with the S10 sense folded in.
    ///
    /// Both are carried because the two questions really are
    /// different, and each door picks the one it means. `angle` reads
    /// the CHART normal: it reports the angle between two carriers,
    /// and `topo::readback` states the rule that a chart direction is
    /// not silently sense-corrected. `gap` reads `outward`: C5's gap
    /// is a MATERIAL separation, positive toward separation, and a
    /// material question needs the material side — the same reason
    /// `carrier_eq`'s plane arm folds sense before deciding.
    Plane {
        /// A point on the plane.
        origin: Point3<T>,
        /// The unit chart normal, uncorrected.
        normal: Vec3<T>,
        /// The face's outward normal: `normal` when the face's
        /// material side agrees with the chart, `-normal` when it is
        /// reversed.
        outward: Vec3<T>,
        /// An UPPER bound on the distance from `origin` to any point of
        /// the face — the face's own extent, and the lever [`arm`] is
        /// built from ([`reach_of`]).
        reach: T,
    },
    /// A cylindrical face's carrier: a point on the axis, the unit
    /// axis direction, and the radius.
    Cylinder {
        /// A point on the axis.
        origin: Point3<T>,
        /// The unit axis direction.
        axis: Vec3<T>,
        /// The radius, positive by the surface's own convention.
        radius: T,
        /// An UPPER bound on the distance from `origin` to any point of
        /// the face — the face's own extent, and the lever [`arm`] is
        /// built from ([`reach_of`]).
        reach: T,
    },
    /// A spherical face's carrier: centre and radius.
    Sphere {
        /// The centre.
        center: Point3<T>,
        /// The radius.
        radius: T,
    },
    /// A straight edge's carrier: its unit direction.
    ///
    /// The line's own origin is deliberately absent. The v1 table's
    /// only line consumer is `angle`, which reads directions; carrying
    /// a point nothing reads would be a field a reader has to check
    /// the uses of to know is inert.
    Line {
        /// The unit direction.
        dir: Vec3<T>,
    },
    /// A carrier outside the v1 table, kept as its CLASS so a refusal
    /// can name it.
    Other(&'static str),
    /// A reference the expression never indexes, so its carrier was
    /// never read. Unreachable from any closed form (the node door
    /// bounds every index), and announced as a kernel bug if reached.
    Unread,
}

impl<T: geom_core::Real> Carrier<T> {
    /// The carrier's class, for a refusal's prose.
    pub(crate) fn class(&self) -> &'static str {
        match self {
            Self::Point(_) => "a vertex",
            Self::Plane { .. } => "a plane face",
            Self::Cylinder { .. } => "a cylinder face",
            Self::Sphere { .. } => "a sphere face",
            Self::Line { .. } => "a line edge",
            Self::Other(what) => what,
            Self::Unread => "an unread reference",
        }
    }
}

/// Reads the carrier a resolved entity sits on, out of the body its
/// reference lands in.
///
/// A dangling arena key is a KERNEL BUG, not a document fault: the
/// entity came out of the very table this evaluation emitted for this
/// body. It comes back as [`Carrier::Other`] naming the state rather
/// than as a panic, so the refusal ladder above still gets to say
/// which measurement could not be taken.
pub(crate) fn carrier_of<T: Decide>(body: &Body<T>, ent: EntityRef) -> Carrier<T> {
    match ent.key {
        EntityKey::Body => Carrier::Other("a whole body"),
        EntityKey::Vertex(v) => match topo::readback::vertex_point(body, v) {
            Ok(p) => Carrier::Point(p),
            Err(_) => Carrier::Other("an unreadable vertex"),
        },
        EntityKey::Face(k) => {
            let Some(face) = body.get_face(k) else {
                return Carrier::Other("an unreadable face");
            };
            match body.get_surface(face.surface) {
                Some(Surface::Plane { origin, normal, .. }) => {
                    let Some(reach) = reach_of(body, k, *origin) else {
                        return Carrier::Other("a face with no readable boundary");
                    };
                    Carrier::Plane {
                        origin: *origin,
                        normal: *normal,
                        // S10's sense bit, folded once, here: `true`
                        // means the material side agrees with the chart
                        // normal.
                        outward: if face.sense { *normal } else { -*normal },
                        reach,
                    }
                }
                Some(Surface::Cylinder {
                    origin,
                    axis,
                    radius,
                    ..
                }) => {
                    let Some(reach) = reach_of(body, k, *origin) else {
                        return Carrier::Other("a face with no readable boundary");
                    };
                    Carrier::Cylinder {
                        origin: *origin,
                        axis: *axis,
                        radius: *radius,
                        reach,
                    }
                }
                Some(Surface::Sphere { center, radius, .. }) => Carrier::Sphere {
                    center: *center,
                    radius: *radius,
                },
                Some(Surface::Cone { .. }) => Carrier::Other("a cone face"),
                Some(Surface::Torus { .. }) => Carrier::Other("a torus face"),
                Some(_) => Carrier::Other("a free-form face"),
                None => Carrier::Other("an unreadable face"),
            }
        }
        EntityKey::Edge(k) => {
            let Some(edge) = body.get_edge(k) else {
                return Carrier::Other("an unreadable edge");
            };
            let carrier = body
                .get_curve_geom(edge.curve)
                .and_then(|g| g.certified())
                .map(topo::EdgeCurve::carrier);
            match carrier {
                Some(Curve3::Line { dir, .. }) => Carrier::Line { dir: *dir },
                Some(Curve3::Circle { .. }) => Carrier::Other("a circular edge"),
                Some(_) => Carrier::Other("a curved edge"),
                None => Carrier::Other("an unreadable edge"),
            }
        }
    }
}

/// How a primitive could not be taken.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum PrimitiveRefusal {
    /// No closed form for this carrier pair.
    Unsupported(MeasureUnsupported),
    /// The pair IS in the v1 table, but its arm needs the two carriers
    /// PARALLEL and they are decidedly not.
    ///
    /// Its own refusal rather than `Unsupported`, because the two say
    /// different things to a reader: `Unsupported` means "this
    /// vocabulary has no arm for that pair", which for two cylinder
    /// walls is FALSE and sent the author looking for a missing
    /// feature instead of at their tilt.
    NotParallel {
        /// Which primitive was asked.
        verb: &'static str,
        /// The pair class, for the message.
        a: &'static str,
        /// The second operand's class.
        b: &'static str,
        /// The funnel predicate that decided them non-parallel.
        predicate: &'static str,
    },
    /// A parallelism or coaxiality trilean landed in the band: the
    /// measurement's MEANING depends on a fact the run cannot decide,
    /// so no number is reported.
    Escalated {
        /// The existing funnel predicate that escalated.
        predicate: &'static str,
        /// The escalation, unaltered.
        source: geom_core::Indeterminate,
    },
    /// The measured value is not finite. The SAME ruled door
    /// [`crate::expr::eval`] applies to a document expression, applied
    /// here for the same reason and by the same function: a measure is
    /// a number a reader believes, and an assertion over `inf` would
    /// report a verdict about nothing.
    NonFinite(crate::expr::EvalError),
}

/// **The v1 primitive evaluator.** One closed form per row of the
/// module's table; every other pair refuses.
pub(crate) fn primitive<T: Decide>(
    prim: MeasurePrimitive,
    a: &Carrier<T>,
    b: &Carrier<T>,
    band: Band,
) -> Result<T, PrimitiveRefusal> {
    match prim {
        MeasurePrimitive::Distance { .. } => distance(a, b, band),
        MeasurePrimitive::Angle { .. } => angle(a, b),
        MeasurePrimitive::Gap { .. } => gap(a, b, band),
        // Not a closed form over carriers at all: its answer is the E7
        // engine's, computed at the node's wiring and read off the
        // pre-order vector in `eval_measure_inner`, which intercepts
        // this primitive before it reaches here. Announced as the
        // kernel bug it would be rather than measured by some other
        // arm that happens to compile.
        MeasurePrimitive::MinClearance { .. } => unreachable!(
            "`min_clearance` is answered by the clearance engine before the closed-form table \
             is consulted; reaching this arm means the interception and this dispatch disagree"
        ),
    }
}

/// The pair is in the table; the arm's parallelism precondition is
/// not met. Names the tilt, not a missing feature (see
/// [`PrimitiveRefusal::NotParallel`]).
fn not_parallel<T: geom_core::Real, X>(
    verb: &'static str,
    predicate: &'static str,
    a: &Carrier<T>,
    b: &Carrier<T>,
) -> Result<X, PrimitiveRefusal> {
    Err(PrimitiveRefusal::NotParallel {
        verb,
        a: a.class(),
        b: b.class(),
        predicate,
    })
}

fn unsupported<T: geom_core::Real, X>(
    verb: &'static str,
    a: &Carrier<T>,
    b: &Carrier<T>,
) -> Result<X, PrimitiveRefusal> {
    Err(PrimitiveRefusal::Unsupported(MeasureUnsupported {
        verb,
        a: a.class(),
        b: b.class(),
    }))
}

/// The parallelism trilean, at an EXISTING funnel predicate and its
/// existing margin shape: the sine of two unit directions'
/// disagreement, levered at the arm over which the verdict is
/// consumed. The arm is the operands' own separation — the distance
/// the misalignment is about to be quoted over — falling back to unit
/// arm when the two carriers pass through one point, where there is
/// no separation to price.
fn parallel<T: Decide>(
    predicate: &'static str,
    u: Vec3<T>,
    v: Vec3<T>,
    arm: T,
    band: Band,
) -> Result<bool, PrimitiveRefusal> {
    match decide(predicate, Margin::levered(u.cross(v).norm(), arm), band) {
        Ok(Sign::Zero) => Ok(true),
        Ok(Sign::Positive | Sign::Negative) => Ok(false),
        Err(source) => Err(PrimitiveRefusal::Escalated { predicate, source }),
    }
}

/// **The lever arm a parallelism verdict is consumed over: an UPPER
/// bound on the EXTENT of the two operands together, with no floor**
/// (ERROR-DESIGN E3's amendment, ratified at revision E12).
///
/// A `bool_plane_parallel` verdict is not consumed as "these normals
/// agree"; it is consumed as "the separation is CONSTANT across the
/// faces", by [`gap`]'s sign and by the mate consumers. What that costs
/// is a deviation `sin θ · L` where `L` is the length the faces span,
/// so the honest lever is the diameter of the region the verdict is
/// read over — here bounded above by
/// `reach(a) + reach(b) + ‖ref(b) − ref(a)‖`, which contains the union
/// of the two carriers by the triangle inequality. The bound is an
/// UPPER one because over-refusal is the safe direction: an arm larger
/// than the truth prices a tilt higher and refuses sooner.
///
/// **What it replaces, and why.** The shipped arm was
/// `max(separation, 1 m)`. For every part smaller than a metre the
/// separation never participated and the lever was the CONSTANT 1 m —
/// an ad-hoc absolute standing in for a model scale nobody asked for,
/// exactly the class the rejected lever-arm unification names. Two
/// walls 10 mm apart tilted by 1e-8 rad deviate 1e-10 m across their
/// own extent, a tenth of a default ε, and the measure refused them as
/// non-parallel because the tilt was priced across a metre they do not
/// span. Two planes crossing within ε of the reference point at 45°
/// certified as parallel under the separation-only reading, and still
/// do not here: their extent is real and `sin 45° · L` is enormous.
///
/// **No floor, and no predicate minted to replace one.** A zero lever
/// would price every tilt at zero and call it parallel — the direction
/// that reports numbers instead of refusing them — so a carrier with no
/// extent must not reach this function. It cannot: [`carrier_of`]
/// refuses a face whose boundary it cannot walk
/// ([`Carrier::Other`] → the measure's typed refusal), and a face that
/// PASSES tier-3 validation has positive extent by construction. That
/// is the argument for having no floor rather than a small one, and it
/// is a structural refusal rather than a new metered predicate: nothing
/// here decides anything, and no ε enters.
fn arm<T: Decide>(a: &Carrier<T>, b: &Carrier<T>) -> T {
    let (pa, ra) = reference(a);
    let (pb, rb) = reference(b);
    ra + rb + (pb - pa).norm()
}

/// A carrier's reference point and the reach around it — the two halves
/// [`arm`] adds up. A carrier with no extent of its own (a vertex) has
/// reach zero, which is the truth about a point.
fn reference<T: Decide>(c: &Carrier<T>) -> (Point3<T>, T) {
    match c {
        Carrier::Point(p) => (*p, T::zero()),
        Carrier::Plane { origin, reach, .. } | Carrier::Cylinder { origin, reach, .. } => {
            (*origin, *reach)
        }
        // A sphere's own radius IS its reach, exactly.
        Carrier::Sphere { center, radius } => (*center, *radius),
        // A line carries no origin (its type docs say why), so it has
        // no reference point to measure a reach from. It never reaches
        // this door: `angle` is the v1 table's only line consumer and
        // takes no arm.
        Carrier::Line { .. } | Carrier::Other(_) | Carrier::Unread => (Point3::origin(), T::zero()),
    }
}

/// **An upper bound on the distance from `origin` to any point of face
/// `k`** — the face's reach ([`arm`]).
///
/// The walk is the face's own boundary: every loop, every edge, each
/// edge bounded from `origin` by [`curve_reach`]. A trimmed region is
/// contained in the region its boundary bounds for every carrier in the
/// v1 table (a plane patch and a cylinder patch are both bounded by
/// their own rims), so the maximum over the boundary bounds the face.
///
/// `None` when the boundary cannot be walked or an edge's carrier has
/// no bound this module can state — a refusal, never a guess, because
/// an under-estimate here would price a tilt too low and certify a
/// parallelism that does not hold.
fn reach_of<T: Decide>(body: &Body<T>, k: topo::entity::FaceKey, origin: Point3<T>) -> Option<T> {
    let face = body.get_face(k)?;
    let mut reach = T::zero();
    for lk in core::iter::once(face.outer).chain(face.rings.iter().copied()) {
        let (edges, _) = topo::props::loop_edges(body, lk).ok()?;
        for e in &edges {
            reach = reach.max(curve_reach(&e.carrier, e.t0, e.t1, origin)?);
        }
    }
    Some(reach)
}

/// An upper bound on `‖c(t) − origin‖` over `t ∈ [t0, t1]`.
///
/// Exact for a line (the maximum of its two ends); the containing
/// circle or ellipse for the conics, which bounds any TRIM of them; the
/// control polygon for a NURBS, which contains the curve by the convex
/// hull property (positive weights, the same assumption
/// `geom::surfaces::boxes::nurbs_surface_aabb` states). `None` for a
/// NURBS with an empty control net, which describes no locus to bound.
fn curve_reach<T: Decide>(c: &Curve3<T>, t0: T, t1: T, origin: Point3<T>) -> Option<T> {
    let from = |p: Point3<T>| (p - origin).norm();
    match c {
        Curve3::Line { .. } => Some(from(c.eval(t0)).max(from(c.eval(t1)))),
        Curve3::Circle { center, radius, .. } => Some(from(*center) + *radius),
        Curve3::Ellipse { center, major, .. } => Some(from(*center) + *major),
        Curve3::Nurbs(n) => n.control().iter().map(|p| from(*p)).reduce(|a, b| a.max(b)),
    }
}

/// `distance(a, b)` -> Length. Unsigned throughout: a distance is a
/// magnitude, and the signed object is [`gap`].
fn distance<T: Decide>(a: &Carrier<T>, b: &Carrier<T>, band: Band) -> Result<T, PrimitiveRefusal> {
    match (a, b) {
        // Vertex x vertex: an exact norm, no decision.
        (Carrier::Point(p), Carrier::Point(q)) => Ok((*q - *p).norm()),
        // Vertex x plane face: the point's coordinate along the unit
        // normal, in metres by construction.
        (Carrier::Point(p), Carrier::Plane { origin, normal, .. })
        | (Carrier::Plane { origin, normal, .. }, Carrier::Point(p)) => {
            Ok(magnitude((*p - *origin).dot(*normal)))
        }
        // Plane x plane: defined ONLY between parallel planes — the
        // distance between non-parallel planes is zero along their
        // line of intersection and unbounded away from it, so there is
        // no number to report rather than a number to approximate.
        (
            Carrier::Plane {
                origin: oa,
                normal: na,
                ..
            },
            Carrier::Plane {
                origin: ob,
                normal: nb,
                ..
            },
        ) => {
            if !parallel("bool_plane_parallel", *na, *nb, arm(a, b), band)? {
                return not_parallel("distance", "bool_plane_parallel", a, b);
            }
            Ok(magnitude((*ob - *oa).dot(*na)))
        }
        // Cylinder x cylinder: the distance between the two AXIS LINES
        // — the worked example's web is this minus the two radii, and
        // the subtraction is the author's arithmetic, not a hidden
        // convention. Parallel axes only: skew axes have a distance
        // this formula computes but a common perpendicular no wall
        // pair shares, which is `gap`'s refusal for the same reason.
        (
            Carrier::Cylinder {
                origin: oa,
                axis: aa,
                ..
            },
            Carrier::Cylinder {
                origin: ob,
                axis: ab,
                ..
            },
        ) => {
            if !parallel("carrier_cyl_axis_parallel", *aa, *ab, arm(a, b), band)? {
                return not_parallel("distance", "carrier_cyl_axis_parallel", a, b);
            }
            Ok(axis_offset(*oa, *aa, *ob))
        }
        _ => unsupported("distance", a, b),
    }
}

/// **|d|, as a bracket that cannot contain a negative.**
///
/// `d.max(-d)` alone is SOUND at every scalar — it contains the true
/// magnitude — but at `Interval` it is loose in a way that
/// contradicts the thing it computes: for `d = [-1, 2]` it yields
/// `[-1, 2]`, so a value this module documents as a magnitude brackets
/// negative lengths. Clamping at zero afterwards costs one operation,
/// tightens the bracket to `[0, 2]`, and makes the documented
/// contract true rather than merely defensible.
///
/// It is a value operation, not a decision: `max` is `Real`'s, the
/// same lattice `Expr::max` uses, and no comparison is reified.
fn magnitude<T: Decide>(d: T) -> T {
    d.max(-d).max(T::zero())
}

/// The perpendicular distance from a point to a line — the component
/// of the separation orthogonal to a UNIT direction. A metre quantity
/// throughout.
fn axis_offset<T: Decide>(origin: Point3<T>, axis: Vec3<T>, point: Point3<T>) -> T {
    let rel = point - origin;
    (rel - axis * rel.dot(axis)).norm()
}

/// `angle(a, b)` -> Angle: the unsigned angle in `[0, pi]` between two
/// plane normals or two line directions.
///
/// `atan2(‖u x v‖, u·v)` rather than `acos(u·v)`: the four-quadrant
/// form is accurate at both ends of the range, where the cosine form
/// loses every digit. No decision is taken — an angle is defined for
/// every pair of unit directions, including parallel ones.
fn angle<T: Decide>(a: &Carrier<T>, b: &Carrier<T>) -> Result<T, PrimitiveRefusal> {
    let pair = match (a, b) {
        (Carrier::Plane { normal: u, .. }, Carrier::Plane { normal: v, .. })
        | (Carrier::Line { dir: u, .. }, Carrier::Line { dir: v, .. }) => (*u, *v),
        _ => return unsupported("angle", a, b),
    };
    let (u, v) = pair;
    Ok(u.cross(v).norm().atan2(u.dot(v)))
}

/// **C5's signed gap.** `g > 0` clearance, `g = 0` contact, `g < 0`
/// interference — the ONE sign convention, defined here and nowhere
/// else.
///
/// Argument order is the mating ROLE: `outer` contains, `inner` is
/// contained. That is why the sphere arm is `R − r − ‖Δc‖` and not its
/// negation, and it is authored rather than inferred from which radius
/// is larger — inferring it would be a decided comparison dressed as a
/// definition, and it would silently flip the sign of an interference
/// fit, which is exactly the case C6 exists for.
fn gap<T: Decide>(
    outer: &Carrier<T>,
    inner: &Carrier<T>,
    band: Band,
) -> Result<T, PrimitiveRefusal> {
    match (outer, inner) {
        // Parallel planes: the signed MATERIAL separation, measured
        // along the outer face's OUTWARD normal — linear in the
        // authored offsets, so smooth everywhere (C5's ideal M10
        // citizen).
        //
        // **The outward normal, not the chart normal.** A gap is a
        // statement about material sides, so it is read off the side
        // the material is on — the same discipline `carrier_eq`'s
        // plane arm keeps (S10) and for the same reason. Against the
        // raw chart normal the sign was an artifact of which way each
        // face's surface happened to be charted: over two disjoint
        // slabs, half of the parallel pairs read NEGATIVE — C5
        // "interference" where nothing interferes — and half gave
        // `gap(a, b) == gap(b, a)`, so the mating roles carried no
        // information at all.
        //
        // With the sense folded in, `g` is how far `inner`'s face lies
        // along the direction `outer`'s material faces: positive when
        // the two material sides face each other across empty space,
        // negative when they have passed through one another.
        //
        // **What a role swap does, stated exactly, because the obvious
        // guess is wrong.** For an OPPOSED pair — the mating
        // configuration C5 is written for, outward normals pointing at
        // each other — `g` is SYMMETRIC, and that is correct: the
        // clearance between two facing faces is one distance, not two
        // signed ones, and reporting `−2 m` for the far side of a 2 m
        // gap would be a new lie in place of the old one. For an
        // ALIGNED pair (both material sides facing the same way — a
        // flush/containment configuration, not a mate) the swap
        // negates. So the roles are informative for the curved arms,
        // where `R − r` genuinely asks which is the socket, and
        // vacuous for a planar mate — a fact about planes, not a
        // missing feature.
        (
            Carrier::Plane {
                origin: oo,
                outward: wo,
                ..
            },
            Carrier::Plane {
                origin: oi,
                outward: wi,
                ..
            },
        ) => {
            if !parallel("bool_plane_parallel", *wo, *wi, arm(outer, inner), band)? {
                return not_parallel("gap", "bool_plane_parallel", outer, inner);
            }
            Ok((*oi - *oo).dot(*wo))
        }
        // Concentric spheres, ball r in socket R: g = R − r − ‖Δc‖.
        // The norm kink at Δc = 0 is C5's stated semismooth point, and
        // it is the nominal operating point of every real fit — the
        // ratified `Dual<Interval>` straddle-hull treatment applies
        // there with no new machinery.
        (
            Carrier::Sphere {
                center: co,
                radius: r_out,
            },
            Carrier::Sphere {
                center: ci,
                radius: r_in,
            },
        ) => Ok(*r_out - *r_in - (*ci - *co).norm()),
        // Coaxial cylinders, pin r_p in bore r_b: g = r_b − r_p − d.
        // Parallel axes only — a skewed "fit" is not a fit, and it
        // refuses typed rather than reporting the closest approach of
        // two lines as though it were a clearance.
        (
            Carrier::Cylinder {
                origin: oo,
                axis: ao,
                radius: r_bore,
                ..
            },
            Carrier::Cylinder {
                origin: oi,
                axis: ai,
                radius: r_pin,
                ..
            },
        ) => {
            if !parallel(
                "carrier_cyl_axis_parallel",
                *ao,
                *ai,
                arm(outer, inner),
                band,
            )? {
                return not_parallel("gap", "carrier_cyl_axis_parallel", outer, inner);
            }
            Ok(*r_bore - *r_pin - axis_offset(*oo, *ao, *oi))
        }
        _ => unsupported("gap", outer, inner),
    }
}

/// **Evaluates a whole measured expression, and refuses a non-finite
/// result** — [`crate::expr::eval`]'s two-part shape, deliberately
/// mirrored: a recursive `Real` core with no decisions in it, then the
/// ONE ruled door on the final value.
///
/// The door is not a second implementation. It is
/// [`crate::expr::refuse_non_finite`], the very function
/// [`crate::expr::eval`] calls, because this language's `Div` is the
/// same partial operation that language's is: `13 / 0` is `inf` in
/// both, and only one of them used to say so.
pub(crate) fn eval_measure<T: Decide>(
    expr: &MeasureExpr,
    carriers: &[Carrier<T>],
    leaves: &[T],
    cursor: &mut usize,
    clearances: &[T],
    clearance_cursor: &mut usize,
    band: Band,
) -> Result<T, PrimitiveRefusal> {
    let value = eval_measure_inner(
        expr,
        carriers,
        leaves,
        cursor,
        clearances,
        clearance_cursor,
        band,
    )?;
    crate::expr::refuse_non_finite(value).map_err(PrimitiveRefusal::NonFinite)
}

/// The recursive core: the primitives against the resolved carriers,
/// the arithmetic in between, and the value leaves read off the vector
/// the node's leaf stage already evaluated. No decisions inside —
/// poison FLOWS through values per the kernel policy, and the single
/// refusal door is [`eval_measure`]'s.
///
/// Both vectors arrive INDEX-ALIGNED with this walk — the carriers
/// with the node's references, the leaves with
/// [`MeasureExpr::value_leaves`]'s pre-order — so nothing here
/// resolves a name or re-evaluates an expression: resolution ran once,
/// leaf evaluation ran once, and the content key saw exactly the
/// values this arithmetic sees.
fn eval_measure_inner<T: Decide>(
    expr: &MeasureExpr,
    carriers: &[Carrier<T>],
    leaves: &[T],
    cursor: &mut usize,
    clearances: &[T],
    clearance_cursor: &mut usize,
    band: Band,
) -> Result<T, PrimitiveRefusal> {
    let binary =
        |a: &MeasureExpr, b: &MeasureExpr, cursor: &mut usize, clearance_cursor: &mut usize| {
            let x = eval_measure_inner(
                a,
                carriers,
                leaves,
                cursor,
                clearances,
                clearance_cursor,
                band,
            )?;
            let y = eval_measure_inner(
                b,
                carriers,
                leaves,
                cursor,
                clearances,
                clearance_cursor,
                band,
            )?;
            Ok::<(T, T), PrimitiveRefusal>((x, y))
        };
    match expr.kind() {
        MeasureKind::Primitive(p) => {
            let [ia, ib] = p.refs();
            // Both indices were bounds-checked at the node door and
            // re-checked at load, so a miss is a kernel bug: it is
            // announced as one rather than carried as a refusal a
            // caller could believe in.
            let (Some(a), Some(b)) = (carriers.get(ia as usize), carriers.get(ib as usize)) else {
                unreachable!(
                    "`{}` reads references {ia} and {ib} of {} resolved carriers, yet \
                     `Node::measure_fault` bounds every index against the node's reference \
                     list at both the construction and the load door",
                    p.verb(),
                    carriers.len()
                )
            };
            // `Unread` is filled in for references no primitive
            // indexes; reaching one from a primitive means the read
            // set and this walk disagree, which is a kernel bug.
            if matches!(a, Carrier::Unread) || matches!(b, Carrier::Unread) {
                unreachable!(
                    "`{}` reads references {ia}/{ib}, which the resolver marked unread — the \
                     read set is computed from these very primitives",
                    p.verb()
                )
            }
            // **The one primitive whose value did not come from a
            // carrier.** `min_clearance` measures between two
            // SELECTIONS through the E7 engine, which wants bodies and
            // face scopes; its answers were computed once, in the
            // node's wiring, and arrive here in this very pre-order —
            // the same arrangement the value leaves have, for the same
            // reason (one walk, one order, no chance of two consumers
            // disagreeing about which leaf is which).
            if matches!(p, MeasurePrimitive::MinClearance { .. }) {
                let i = *clearance_cursor;
                *clearance_cursor += 1;
                return match clearances.get(i) {
                    Some(v) => Ok(*v),
                    None => unreachable!(
                        "`min_clearance` leaf {i} of {} computed answers is missing, yet that \
                         vector is this expression's own primitives walked in this very order",
                        clearances.len()
                    ),
                };
            }
            primitive(*p, a, b, band)
        }
        MeasureKind::Value(_) => {
            let i = *cursor;
            *cursor += 1;
            // Same reasoning: the leaf vector IS this expression's own
            // `value_leaves` walked in this very order, so a short
            // vector is a kernel bug, not a document fault.
            match leaves.get(i) {
                Some(v) => Ok(*v),
                None => unreachable!(
                    "measure value leaf {i} of {} evaluated leaves is missing, yet the leaf \
                     vector is this expression's own `value_leaves` in this very order",
                    leaves.len()
                ),
            }
        }
        MeasureKind::Neg(a) => Ok(-eval_measure_inner(
            a,
            carriers,
            leaves,
            cursor,
            clearances,
            clearance_cursor,
            band,
        )?),
        MeasureKind::Add(a, b) => {
            let (x, y) = binary(a, b, cursor, clearance_cursor)?;
            Ok(x + y)
        }
        MeasureKind::Sub(a, b) => {
            let (x, y) = binary(a, b, cursor, clearance_cursor)?;
            Ok(x - y)
        }
        MeasureKind::Mul(a, b) => {
            let (x, y) = binary(a, b, cursor, clearance_cursor)?;
            Ok(x * y)
        }
        MeasureKind::Div(a, b) => {
            let (x, y) = binary(a, b, cursor, clearance_cursor)?;
            Ok(x / y)
        }
        MeasureKind::Min(a, b) => {
            let (x, y) = binary(a, b, cursor, clearance_cursor)?;
            Ok(x.min(y))
        }
        MeasureKind::Max(a, b) => {
            let (x, y) = binary(a, b, cursor, clearance_cursor)?;
            Ok(x.max(y))
        }
    }
}
