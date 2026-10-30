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
//! | `distance` | point (a vertex or a point pose) x point, axis or plane pose | `‖q − p‖`, the point's offset from the line, `\|(p − o)·n̂\|` | none |
//! | `distance` | axis pose x axis pose | their separation where parallel, else their common perpendicular's length | `carrier_cyl_axis_parallel` levered over the faces they were read off |
//! | `angle` | plane face x plane face | `atan2(‖n̂_a × n̂_b‖, n̂_a·n̂_b)` | none — a ratio of unit quantities |
//! | `angle` | line edge x line edge | `atan2(‖d̂_a × d̂_b‖, d̂_a·d̂_b)` | none |
//! | `gap` | plane face x plane face | `(o_i − o_o)·ŵ_o` (SIGNED), `ŵ_o` the outer face's OUTWARD normal (S10 sense folded, not the chart normal) | `bool_plane_parallel` |
//! | `gap` | sphere face x sphere face | the inner's touching point, `d = ‖Δc‖` (below) | `gap_round_side`, `gap_round_larger` |
//! | `gap` | cylinder face x cylinder face | the inner's touching point, `d` the axes' separation (below) | `carrier_cyl_axis_parallel`, `gap_round_side`, `gap_round_larger` |
//! | `gap` | plane face x sphere or cylinder face, either seat | `h ∓ r`, `h` the centre's or axis's offset along the plane's outward normal | `gap_round_side`; `gap_axis_parallel_to_plane` for a cylinder |
//! | `gap` | vertex x plane, sphere or cylinder face, either seat | the vertex's signed offset from the face's carrier, positive away from its material | none |
//! | `gap` | line edge x plane face, either seat | the lower end's offset `min((e − o)·ŵ)` | none |
//! | `gap` | line edge x sphere or cylinder face, either seat | the edge's lowest offset from the carrier, `±(d − r)` at the least of `d` along it or an end | none |
//! | `gap` | line edge x vertex, either seat | the vertex's offset from the edge's link (below) | `topo::link`'s door |
//! | `gap` | line edge x line edge | the inner's lowest offset from the outer's link | `topo::link`'s door |
//! | `gap` | plane pose (a split's tool; a frame as its xy plane) x point, either seat | the point's signed offset along the plane's normal | none |
//! | `gap` | vertex x vertex | the inner's offset from the outer's link | `topo::link`'s door |
//!
//! A face pair reads the inner carrier's point that touches the outer
//! at a tangency, along the outer's outward normal. Which point that is
//! is the configuration the two meet in, decided from the values at
//! `gap_round_side` (and, for two nested round carriers, which is the
//! larger at `gap_round_larger`): two round carriers apart or nested, by
//! whether the smaller one's centre or axis lies inside the larger, and
//! a round carrier against a plane by which side of it the centre or
//! axis lies. At every tangency the margin is a radius, so a touch
//! never lands in band. The roles stay authored: the decision reads the
//! configuration, never which face is the socket.
//!
//! A vertex's or an edge's material is its link ([`topo::Link`]): the
//! cone its faces bound at the point, convex, concave or a saddle alike.
//! A point's offset from it is its signed distance from the cone's
//! boundary, negative inside ([`topo::Link::offset`]), and an edge's its
//! lowest along it ([`topo::Link::lowest_on`]). A sphere's or a
//! cylinder's material is its carrier's side ([`topo::Round`]), and the
//! census reads the same carrier at a touch inside such a face. An edge
//! or a vertex beside a curved face has no link yet and refuses, and so
//! does a vertex of a planar face with a curved edge; a line edge of such
//! a face reads it as the disc its other edges clear
//! (`topo::link`'s cleared sector).
//!
//! Everything else — a cone, a torus, a NURBS patch, a curved edge, a
//! whole body, and every mixed pair not listed — refuses.
//!
//! **`min_clearance` is not in that table and never will be** (M10-6).
//! It is the primitive the table's own escape clause names: there is no
//! closed form for the minimum distance between two trimmed regions, so
//! its answer comes from the E7 subdivision engine
//! (`topo::clearance::min_separation`) as a certified BRACKET, and
//! the value is computed in the node's wiring — where the bodies are —
//! rather than here. The wiring (`wire::wire_measure`) intercepts that
//! primitive before this dispatch is consulted, and the arm below says
//! so.
//!
//! # Which trileans this module consumes
//!
//! Two EXISTING funnel predicates called at their existing margin
//! shapes: `bool_plane_parallel` (`Margin::levered(‖n̂_a × n̂_b‖, arm)`,
//! the sine of the normals' disagreement priced at a lever arm) and
//! `carrier_cyl_axis_parallel` (the same shape on the axes). A vertex's
//! or an edge's material decides through the link's own door, every
//! sign a point's distance from a plane (`topo::link`'s module docs). A
//! measure whose decision escalates refuses typed rather than
//! reporting a number whose meaning depends on an undecided fact.
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
//! [`gap`]. `g = 0` is a touch for every pair of cells. Over an OPPOSED
//! pair, each face facing the other's material, `g > 0` is clearance and
//! `g < 0` interference. Over an ALIGNED pair, both facing one way (a
//! flush pair, an aligned tangency, two rods nested), the sign says
//! which face stands outside the other, positive where the inner stands
//! out of the outer's material: no clearance and no interference.

use geom::{Curve3, Surface};
use geom_brep::OutwardNormal;
use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Point3, Sign, Vec3};
use topo::{Body, Link, LinkRefusal, TouchUnread};

use crate::measure::MeasurePrimitive;
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
#[derive(Debug, Clone)]
pub(crate) enum Carrier<'b, T: geom_core::Real> {
    /// A vertex's stored position, and where the material around it is
    /// read.
    Point {
        /// The position.
        at: Point3<T>,
        /// The vertex, its link read when an arm asks.
        cell: CellOf<'b, T>,
    },
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
        /// The face's material, read by its carrier: a pin's or a
        /// bore's, as the face's sense says.
        round: topo::Round<T>,
    },
    /// A spherical face's carrier: centre and radius.
    Sphere {
        /// The centre.
        center: Point3<T>,
        /// The radius.
        radius: T,
        /// The face's material, read by its carrier: a ball's or a
        /// socket's, as the face's sense says.
        round: topo::Round<T>,
    },
    /// A straight edge's carrier: its unit direction, its two ends in
    /// the edge's own direction, and where the material beside it is
    /// read.
    Line {
        /// The unit direction.
        dir: Vec3<T>,
        /// The edge's two vertices' positions, `None` where one does not
        /// read.
        ends: Option<[Point3<T>; 2]>,
        /// The edge, its link read when an arm asks.
        cell: CellOf<'b, T>,
    },
    /// **A pose a reference reads** (D10 `[assertions.quiet]`): a point,
    /// an axis or a plane, read off a cell or a split's tool, and the
    /// reach of the face it was read off, the lever a tilt between two
    /// axes is priced over.
    Pose {
        /// The pose.
        pose: Pose<T>,
        /// An UPPER bound on the distance from the pose's reference point
        /// to any point of the face it was read off ([`reach_of`]);
        /// `None` for a pose read off no face.
        reach: Option<T>,
    },
    /// A carrier outside the v1 table, kept as its CLASS so a refusal
    /// can name it.
    Other(&'static str),
}

/// **A pose as the measure reads it**: where it is, by a point on it.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Pose<T: geom_core::Real> {
    /// A point.
    Point(Point3<T>),
    /// The line through `through` along the unit `dir`.
    Axis {
        /// A point on the line.
        through: Point3<T>,
        /// The unit direction.
        dir: Vec3<T>,
    },
    /// The plane through `through` with the unit `normal`, its sense
    /// the side a signed gap reads positive.
    Plane {
        /// A point on the plane.
        through: Point3<T>,
        /// The unit normal.
        normal: Vec3<T>,
    },
}

impl<T: geom_core::Real> Pose<T> {
    /// The pose of `value`: a point, an axis or a plane, and a frame as
    /// the plane it lies in, its xy (a split's tool read through `Xy`);
    /// `None` for a direction.
    pub(crate) fn of(value: &crate::pose::PoseValue<T>) -> Option<Self> {
        use crate::pose::PoseValue as V;
        match value {
            V::Point { position } => Some(Self::Point(*position)),
            V::Axis(line) => Some(Self::Axis {
                through: line.foot(),
                dir: line.dir().get(),
            }),
            V::Plane { normal, offset } => Some(Self::Plane {
                through: V::plane_foot(*normal, *offset),
                normal: normal.get(),
            }),
            V::Frame(f) => Some(Self::Plane {
                through: f.origin(),
                normal: f.w().get(),
            }),
            V::Direction { .. } => None,
        }
    }

    /// A point on the pose.
    pub(crate) fn at(&self) -> Point3<T> {
        match *self {
            Self::Point(at) | Self::Axis { through: at, .. } | Self::Plane { through: at, .. } => {
                at
            }
        }
    }
}

impl<T: geom_core::Real> Carrier<'_, T> {
    /// The carrier's class, for a refusal's prose.
    pub(crate) fn class(&self) -> &'static str {
        match self {
            Self::Point { .. } => "a vertex",
            Self::Plane { .. } => "a plane face",
            Self::Cylinder { .. } => "a cylinder face",
            Self::Sphere { .. } => "a sphere face",
            Self::Line { .. } => "a line edge",
            Self::Pose {
                pose: Pose::Point(_),
                ..
            } => "a point",
            Self::Pose {
                pose: Pose::Axis { .. },
                ..
            } => "an axis",
            Self::Pose {
                pose: Pose::Plane { .. },
                ..
            } => "a plane",
            Self::Other(what) => what,
        }
    }
}

/// **Where a vertex's or a line edge's link is read**: off its body,
/// and only when an arm of [`gap`] asks, so a `distance` or an `angle`
/// never walks the cell's faces.
#[derive(Debug, Clone, Copy)]
pub(crate) enum CellOf<'b, T: geom_core::Real> {
    /// Vertex `v` of `body`.
    Vertex(&'b Body<T>, topo::entity::VertexKey),
    /// Line edge `e` of `body`.
    Edge(&'b Body<T>, topo::entity::EdgeKey),
}

impl<T: Decide> CellOf<'_, T> {
    /// The cell's link: at the vertex, or at `at` inside the edge.
    fn link(&self, at: Point3<T>, band: Band) -> Result<Link<T>, TouchUnread> {
        match self {
            Self::Vertex(body, v) => Link::at_point(body, *v, band),
            Self::Edge(body, e) => Link::on_edge(body, *e, at, band),
        }
    }
}

/// **Face `k` as a plane**: a point on it, its chart normal and its
/// outward normal; `None` where it does not read or is not a plane.
fn plane_of<T: Decide>(
    body: &Body<T>,
    k: topo::entity::FaceKey,
) -> Option<(Point3<T>, Vec3<T>, Vec3<T>)> {
    let face = body.get_face(k)?;
    match body.get_surface(face.surface)? {
        Surface::Plane { origin, normal, .. } => Some((
            *origin,
            *normal,
            OutwardNormal::from_chart(*normal, face.sense).vec(),
        )),
        _ => None,
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
pub(crate) fn carrier_of<T: Decide>(body: &Body<T>, ent: EntityRef) -> Carrier<'_, T> {
    match ent.key {
        EntityKey::Body => Carrier::Other("a whole body"),
        EntityKey::Vertex(v) => match topo::readback::vertex_point(body, v) {
            Ok(at) => Carrier::Point {
                at,
                cell: CellOf::Vertex(body, v),
            },
            Err(_) => Carrier::Other("an unreadable vertex"),
        },
        EntityKey::Face(k) => {
            let Some(face) = body.get_face(k) else {
                return Carrier::Other("an unreadable face");
            };
            if let Some((origin, normal, outward)) = plane_of(body, k) {
                let Some(reach) = reach_of(body, k, origin) else {
                    return Carrier::Other("a face with no readable boundary");
                };
                return Carrier::Plane {
                    origin,
                    normal,
                    outward,
                    reach,
                };
            }
            let surface = body.get_surface(face.surface);
            let round = surface.and_then(|s| topo::Round::of(s, face.sense));
            match (surface, round) {
                (
                    Some(Surface::Cylinder {
                        origin,
                        axis,
                        radius,
                        ..
                    }),
                    Some(round),
                ) => {
                    let Some(reach) = reach_of(body, k, *origin) else {
                        return Carrier::Other("a face with no readable boundary");
                    };
                    Carrier::Cylinder {
                        origin: *origin,
                        axis: *axis,
                        radius: *radius,
                        reach,
                        round,
                    }
                }
                (Some(Surface::Sphere { center, radius, .. }), Some(round)) => Carrier::Sphere {
                    center: *center,
                    radius: *radius,
                    round,
                },
                (Some(Surface::Cone { .. }), _) => Carrier::Other("a cone face"),
                (Some(Surface::Torus { .. }), _) => Carrier::Other("a torus face"),
                (Some(_), _) => Carrier::Other("a free-form face"),
                (None, _) => Carrier::Other("an unreadable face"),
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
                Some(Curve3::Line { dir, .. }) => Carrier::Line {
                    dir: *dir,
                    ends: body
                        .half_edge_start_point(edge.he_plus)
                        .zip(body.half_edge_start_point(edge.he_minus))
                        .map(|(start, end)| [start, end]),
                    cell: CellOf::Edge(body, k),
                },
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
pub(crate) fn primitive<T: Decide, R>(
    prim: &MeasurePrimitive<R>,
    a: &Carrier<'_, T>,
    b: &Carrier<'_, T>,
    band: Band,
) -> Result<T, PrimitiveRefusal> {
    match prim {
        MeasurePrimitive::Distance { .. } => distance(a, b, band),
        MeasurePrimitive::Angle { .. } => angle(a, b),
        MeasurePrimitive::Gap { .. } => gap(a, b, band),
        // Not a closed form over carriers at all: its answer is the E7
        // engine's, computed at the node's wiring, which answers this
        // primitive before it reaches here. Announced as the kernel bug
        // it would be rather than measured by some other arm that
        // happens to compile.
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
    a: &Carrier<'_, T>,
    b: &Carrier<'_, T>,
) -> Result<X, PrimitiveRefusal> {
    Err(PrimitiveRefusal::NotParallel {
        verb,
        a: a.class(),
        b: b.class(),
        predicate,
    })
}

/// [`unsupported`], naming a cell's class where the carrier's own
/// would not say why.
fn unsupported_as<X>(
    verb: &'static str,
    a: &'static str,
    b: &'static str,
) -> Result<X, PrimitiveRefusal> {
    Err(PrimitiveRefusal::Unsupported(MeasureUnsupported {
        verb,
        a,
        b,
    }))
}

fn unsupported<T: geom_core::Real, X>(
    verb: &'static str,
    a: &Carrier<'_, T>,
    b: &Carrier<'_, T>,
) -> Result<X, PrimitiveRefusal> {
    Err(PrimitiveRefusal::Unsupported(MeasureUnsupported {
        verb,
        a: a.class(),
        b: b.class(),
    }))
}

/// The parallelism trilean, at an EXISTING funnel predicate and its
/// existing margin shape: the sine of two unit directions'
/// disagreement, levered at the arm over which the verdict is consumed.
///
/// The arm is [`arm`]'s — an upper bound on the EXTENT of the two
/// operands together, with no floor. This paragraph used to say
/// "the operands' own separation … falling back to unit arm when the
/// two carriers pass through one point", which was the pre-E3 reading
/// and stopped being true when E3's amendment landed: there is no
/// fallback constant here any more, and separation is one of three
/// terms rather than the whole of it.
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
/// that reports numbers instead of refusing them — so no carrier that
/// reaches this function may have zero extent.
///
/// **Which carriers reach it, stated rather than assumed.** Only the
/// four call sites below reach `arm`, and each one has already matched
/// its operands to a shape with extent: `distance` and `gap` on a
/// plane-face pair, and the two cylinder-axis sites on a cylinder pair.
/// A [`Carrier::Plane`] or
/// [`Carrier::Cylinder`] carries the `reach` [`carrier_of`] computed
/// for it, and `carrier_of` answers [`Carrier::Other`] — the measure's
/// typed refusal — for a face whose boundary it cannot walk, so a face
/// that reaches a lever site has a walked boundary and a positive
/// reach. A [`Carrier::Line`]'s reach is its own length.
///
/// **What [`reference()`] does for `Other` is a defined answer, not an
/// invariant.** It has no reference point at all, and `reference`
/// answers the world origin with reach zero. That is not a claim that
/// such a carrier cannot arrive; it is what the function returns if one
/// does. No lever site matches one, and this paragraph is the record of
/// why, in place of an invariant nothing enforces. A fifth lever site
/// would have to re-establish it; `reference`'s own comment says so at
/// the arm.
///
/// It is a structural argument rather than a new metered predicate:
/// nothing here decides anything, and no ε enters.
fn arm<T: Decide>(a: &Carrier<'_, T>, b: &Carrier<'_, T>) -> T {
    let (pa, ra) = reference(a);
    let (pb, rb) = reference(b);
    ra + rb + (pb - pa).norm()
}

/// A carrier's reference point and the reach around it — the two halves
/// [`arm`] adds up. A carrier with no extent of its own (a vertex) has
/// reach zero, which is the truth about a point.
fn reference<T: Decide>(c: &Carrier<'_, T>) -> (Point3<T>, T) {
    match c {
        Carrier::Point { at, .. } => (*at, T::zero()),
        Carrier::Plane { origin, reach, .. } | Carrier::Cylinder { origin, reach, .. } => {
            (*origin, *reach)
        }
        Carrier::Pose { pose, reach } => (pose.at(), reach.unwrap_or_else(T::zero)),
        // A sphere's own radius IS its reach, exactly.
        Carrier::Sphere { center, radius, .. } => (*center, *radius),
        // A segment's length bounds its reach from either end.
        Carrier::Line {
            ends: Some([start, end]),
            ..
        } => (*start, (*end - *start).norm()),
        // An unsupported carrier, or an edge whose ends do not read, has
        // no reference point to measure a reach from. The world origin with reach zero is the DEFINED
        // answer for it, not a claim that one cannot arrive — see
        // [`arm`]'s "which carriers reach it". No lever site matches
        // one; a new lever site must check that again rather than
        // inherit it.
        Carrier::Line { ends: None, .. } | Carrier::Other(_) => (Point3::origin(), T::zero()),
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
///
/// Crate-visible because the mate solve's lever reads the same walk
/// for a part body's faces (`mate::reach`): one boundary walk, one
/// bound, read at both lever sites.
pub(crate) fn reach_of<T: Decide>(
    body: &Body<T>,
    k: topo::entity::FaceKey,
    origin: Point3<T>,
) -> Option<T> {
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
        // The semi-axes carry no order and no sign (`geom_brep::Conic`):
        // the reach is the larger MAGNITUDE.
        Curve3::Ellipse {
            center,
            semi_u,
            semi_v,
            ..
        } => Some(from(*center) + geom::Curve3::semi_axes_by_size(*semi_u, *semi_v).0),
        // Every point of the spiric lies on its torus, within `R + r` of
        // the torus centre.
        Curve3::Spiric {
            center,
            major_radius,
            minor_radius,
            ..
        } => Some(from(*center) + *major_radius + *minor_radius),
        Curve3::Nurbs(n) => n.control().iter().map(|p| from(*p)).reduce(|a, b| a.max(b)),
    }
}

/// `distance(a, b)` -> Length. Unsigned throughout: a distance is a
/// magnitude, and the signed object is [`gap`].
fn distance<T: Decide>(
    a: &Carrier<'_, T>,
    b: &Carrier<'_, T>,
    band: Band,
) -> Result<T, PrimitiveRefusal> {
    match (a, b) {
        // Vertex x vertex: an exact norm, no decision.
        (Carrier::Point { at: p, .. }, Carrier::Point { at: q, .. }) => Ok((*q - *p).norm()),
        // A vertex or a point pose x plane face: the point's coordinate
        // along the unit normal, in metres by construction.
        (
            Carrier::Point { at: p, .. }
            | Carrier::Pose {
                pose: Pose::Point(p),
                ..
            },
            Carrier::Plane { origin, normal, .. },
        )
        | (
            Carrier::Plane { origin, normal, .. },
            Carrier::Point { at: p, .. }
            | Carrier::Pose {
                pose: Pose::Point(p),
                ..
            },
        ) => Ok(magnitude((*p - *origin).dot(*normal))),
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
        _ => pose_distance(a, b, band),
    }
}

/// **`distance` over a pose** (D10 `[assertions.quiet]`): a point (a
/// vertex's, or a point pose) from a point, an axis or a plane, and an
/// axis from an axis as one scalar: their separation where the kernel's
/// axis verdict (`carrier_cyl_axis_parallel`, levered over the faces
/// they were read off) reads them parallel, else the length of their
/// common perpendicular. It is zero exactly where the axes are one line
/// or meet. Two axes read off no face have no lever, and refuse.
fn pose_distance<T: Decide>(
    a: &Carrier<'_, T>,
    b: &Carrier<'_, T>,
    band: Band,
) -> Result<T, PrimitiveRefusal> {
    let point = |c: &Carrier<'_, T>| match c {
        Carrier::Point { at, .. }
        | Carrier::Pose {
            pose: Pose::Point(at),
            ..
        } => Some(*at),
        _ => None,
    };
    let pose = |c: &Carrier<'_, T>| match c {
        Carrier::Pose { pose, reach } => Some((*pose, *reach)),
        _ => None,
    };
    if let (Some(p), Some(q)) = (point(a), point(b)) {
        return Ok((q - p).norm());
    }
    for (x, other) in [(a, b), (b, a)] {
        let Some(p) = point(x) else { continue };
        match pose(other) {
            Some((Pose::Axis { through, dir }, _)) => return Ok(axis_offset(through, dir, p)),
            Some((Pose::Plane { through, normal }, _)) => {
                return Ok(magnitude((p - through).dot(normal)));
            }
            _ => {}
        }
    }
    let (
        Some((
            Pose::Axis {
                through: oa,
                dir: da,
            },
            ra,
        )),
        Some((
            Pose::Axis {
                through: ob,
                dir: db,
            },
            rb,
        )),
    ) = (pose(a), pose(b))
    else {
        return unsupported("distance", a, b);
    };
    if ra.is_none() || rb.is_none() {
        return unsupported_as("distance", NO_LEVER, b.class());
    }
    if parallel(AXES_PARALLEL, da, db, arm(a, b), band)? {
        return Ok(axis_offset(oa, da, ob));
    }
    let across = da.cross(db);
    Ok(magnitude((ob - oa).dot(across)) / across.norm())
}

/// The site two axes are decided parallel at, the cylinder pair's own.
const AXES_PARALLEL: &str = "carrier_cyl_axis_parallel";

/// **Whether two axis poses read parallel** at the two-axes arm's own
/// site and lever ([`pose_distance`]): the reading a coaxial row is said
/// by (`checks::at_rest::quiet_row`). `None` for a pair that is not two
/// axes read off faces, or a decision that escalates.
pub(crate) fn axes_parallel<T: Decide>(
    a: &Carrier<'_, T>,
    b: &Carrier<'_, T>,
    band: Band,
) -> Option<bool> {
    let (
        Carrier::Pose {
            pose: Pose::Axis { dir: da, .. },
            reach: Some(_),
        },
        Carrier::Pose {
            pose: Pose::Axis { dir: db, .. },
            reach: Some(_),
        },
    ) = (a, b)
    else {
        return None;
    };
    parallel(AXES_PARALLEL, *da, *db, arm(a, b), band).ok()
}

/// The class a refusal names for two axes with no face to lever their
/// tilt over.
const NO_LEVER: &str = "an axis read off no face, which no tilt can be levered over";

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
fn angle<T: Decide>(a: &Carrier<'_, T>, b: &Carrier<'_, T>) -> Result<T, PrimitiveRefusal> {
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
/// Argument order is the ROLE: `outer` is the face the gap is read
/// from, `inner` the face read. It is authored, never inferred from
/// which radius is larger; what the values decide is the configuration
/// the two meet in ([`round_pair`], [`plane_round`]). **Two faces are
/// read by both material sides**: an OPPOSED pair, each facing the
/// other's material (a rod on a plate, two rods side by side, a pin in
/// a bore), reads the same in either order, its sign clearance or
/// interference; an ALIGNED pair, both facing one way (a flush pair, a
/// stadium's flat into its round end, two rods nested), negates, its
/// sign which face stands outside the other.
///
/// **A vertex or an edge is read by one cell's material.** A face's
/// material is the half-space of its carrier. A vertex's or an edge's is
/// its LINK ([`topo::Link`]): the directions entering its solid at the
/// point, a cone over its faces, convex, concave or a saddle alike. The
/// gap is the other cell's signed distance from that material's
/// boundary, negative inside:
///
/// - a face reads a vertex or a line edge in either seat, so a vertex
///   or an edge on a face reads the same whichever argument it is;
/// - an edge reads a vertex in either seat, through its link;
/// - between two edges, the outer edge's link reads the inner edge;
///   between two vertices, the outer vertex's link reads the inner one.
///
/// An edge reads as its segment, so its gap is its lowest point's: zero
/// where it lies on the other cell's material without entering it.
fn gap<T: Decide>(
    outer: &Carrier<'_, T>,
    inner: &Carrier<'_, T>,
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
        // negates. Every face pair below keeps that rule.
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
        // Two round faces, a plane and a round face: the configuration
        // they meet in is read off their material sides
        // ([`round_pair`], [`plane_round`]).
        (
            Carrier::Sphere {
                center: co,
                round: ro,
                ..
            },
            Carrier::Sphere {
                center: ci,
                round: ri,
                ..
            },
        ) => round_pair(ro, ri, (*ci - *co).norm(), outer, band),
        // Parallel axes only — a skewed "fit" is not a fit, and it
        // refuses typed rather than reporting the closest approach of
        // two lines as though it were a clearance.
        (
            Carrier::Cylinder {
                origin: oo,
                axis: ao,
                round: ro,
                ..
            },
            Carrier::Cylinder {
                origin: oi,
                axis: ai,
                round: ri,
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
            round_pair(ro, ri, axis_offset(*oo, *ao, *oi), outer, band)
        }
        (Carrier::Plane { .. }, Carrier::Sphere { .. } | Carrier::Cylinder { .. }) => {
            plane_round(outer, inner, band).map(|(g, _)| g)
        }
        (Carrier::Sphere { .. } | Carrier::Cylinder { .. }, Carrier::Plane { .. }) => {
            plane_round(inner, outer, band).map(|(g, opposed)| if opposed { g } else { -g })
        }
        // A plane pose (a split's tool) and a point: the point's signed
        // offset along the plane's normal, in either seat.
        (
            Carrier::Pose {
                pose: Pose::Plane { through, normal },
                ..
            },
            Carrier::Point { at, .. }
            | Carrier::Pose {
                pose: Pose::Point(at),
                ..
            },
        )
        | (
            Carrier::Point { at, .. }
            | Carrier::Pose {
                pose: Pose::Point(at),
                ..
            },
            Carrier::Pose {
                pose: Pose::Plane { through, normal },
                ..
            },
        ) => Ok(normal.dot(*at - *through)),
        // An edge whose ends do not read is a body this evaluation
        // built wrong; no cell arm reads it.
        (Carrier::Line { ends: None, .. }, other) | (other, Carrier::Line { ends: None, .. }) => {
            unsupported_as("gap", "a line edge whose ends do not read", other.class())
        }
        // A face and a vertex, or a point pose: the face reads it.
        (
            face @ (Carrier::Plane { .. } | Carrier::Cylinder { .. } | Carrier::Sphere { .. }),
            Carrier::Point { at, .. }
            | Carrier::Pose {
                pose: Pose::Point(at),
                ..
            },
        )
        | (
            Carrier::Point { at, .. }
            | Carrier::Pose {
                pose: Pose::Point(at),
                ..
            },
            face @ (Carrier::Plane { .. } | Carrier::Cylinder { .. } | Carrier::Sphere { .. }),
        ) => face_offset(face, *at),
        // A face and a line edge: the face reads the edge.
        (
            face @ (Carrier::Plane { .. } | Carrier::Cylinder { .. } | Carrier::Sphere { .. }),
            line @ Carrier::Line { .. },
        )
        | (
            line @ Carrier::Line { .. },
            face @ (Carrier::Plane { .. } | Carrier::Cylinder { .. } | Carrier::Sphere { .. }),
        ) => face_reads_edge(face, line),
        // A vertex or a line edge and another: an edge reads a vertex in
        // either seat, and of two edges or two vertices the outer reads
        // the inner, through its link.
        _ => match (Cell::of(outer), Cell::of(inner)) {
            (Some(vertex @ Cell::Vertex(..)), Some(edge @ Cell::Edge(..))) => {
                link_reads(edge, vertex, band)
            }
            (Some(reading), Some(read)) => link_reads(reading, read, band),
            _ => unsupported("gap", outer, inner),
        },
    }
}

/// The funnel site name of the decision which side of a carrier a round
/// face's centre or axis lies: of a plane, along its outward normal; of
/// the larger of two round carriers, outside or inside it.
pub(crate) const GAP_ROUND_SIDE: &str = "gap_round_side";

/// The funnel site name of the decision that a cylinder's axis lies
/// parallel to a plane.
pub(crate) const GAP_AXIS_PARALLEL_TO_PLANE: &str = "gap_axis_parallel_to_plane";

/// The class a refusal names for a round face whose centre or axis the
/// side decision reads on the other carrier.
const ON_THE_CARRIER: &str = "a round face whose centre or axis lies on the other carrier";

/// **Whether a round face's centre or axis lies beyond a carrier**,
/// `margin` its signed offset from it: the configuration a pair meets
/// in, decided at [`GAP_ROUND_SIDE`]. At a touch the margin is the
/// smaller radius, `±r` against a plane and `±r_small` against the
/// larger of two round carriers, so a tangency never lands in band; a
/// centre or an axis on the carrier refuses.
fn beyond<T: Decide>(
    margin: T,
    outer: &Carrier<'_, T>,
    band: Band,
) -> Result<bool, PrimitiveRefusal> {
    match decide(GAP_ROUND_SIDE, Margin::of(margin), band) {
        Ok(Sign::Positive) => Ok(true),
        Ok(Sign::Negative) => Ok(false),
        Ok(Sign::Zero) => unsupported_as("gap", outer.class(), ON_THE_CARRIER),
        Err(source) => Err(PrimitiveRefusal::Escalated {
            predicate: GAP_ROUND_SIDE,
            source,
        }),
    }
}

/// The funnel site name of the decision which of two nested round
/// carriers is the larger, the outer's radius against the inner's.
pub(crate) const GAP_ROUND_LARGER: &str = "gap_round_larger";

/// **Two round faces of one kind**, `d` apart at their centres or axes,
/// read as every face pair is: the inner carrier's point that touches
/// the outer at a tangency, read along the outer's outward normal
/// ([`topo::Round::offset`]'s sign, out of a ball or a pin, into a
/// socket's or a bore's hollow). Which point that is is the
/// configuration, decided from the values and not from the roles:
///
/// - apart, the smaller carrier's centre outside the larger
///   ([`beyond`], margin `d − r_large`): the inner's point nearest the
///   outer's centre, at `d − r_in` from it;
/// - nested, the inner inside the outer: its farthest point, `d + r_in`;
/// - nested, the outer inside the inner: its nearest point, `r_in − d`
///   ([`GAP_ROUND_LARGER`]; of one radius, a fit's convex face is the
///   one inside, and of two faces of one sense the inner).
///
/// A pair whose two faces face each other's material (two rods or two
/// bores side by side, a pin in a bore) reads the same in either role
/// order; an aligned pair (two rods nested, nested hollows, a rod in a
/// bore's wall) negates.
fn round_pair<T: Decide>(
    outer: &topo::Round<T>,
    inner: &topo::Round<T>,
    d: T,
    outer_face: &Carrier<'_, T>,
    band: Band,
) -> Result<T, PrimitiveRefusal> {
    let (r_out, r_in) = (outer.radius(), inner.radius());
    let at = if beyond(d - r_out.max(r_in), outer_face, band)? {
        d - r_in
    } else {
        // One radius: a fit's convex face is the one inside; two of one
        // sense read the inner inside.
        let inner_inside = match decide(GAP_ROUND_LARGER, Margin::of(r_out - r_in), band) {
            Ok(Sign::Positive) => true,
            Ok(Sign::Negative) => false,
            Ok(Sign::Zero) => inner.convex() || !outer.convex(),
            Err(source) => {
                return Err(PrimitiveRefusal::Escalated {
                    predicate: GAP_ROUND_LARGER,
                    source,
                });
            }
        };
        if inner_inside { d + r_in } else { r_in - d }
    };
    Ok(if outer.convex() {
        at - r_out
    } else {
        r_out - at
    })
}

/// **A plane and a round face**, read along the plane's outward normal
/// at the round carrier's point nearest the plane: `h − r` with the
/// centre or axis at `h` beyond the plane, `h + r` behind it
/// ([`beyond`]). A cylinder's axis lies parallel to the plane.
///
/// Also whether the pair is opposed: a ball or a pin beyond the plane,
/// a socket or a bore behind it. An opposed pair reads the same in
/// either seat, and an aligned one (a stadium's flat into its round
/// end, a slot's) negates, as two planes do.
fn plane_round<T: Decide>(
    plane: &Carrier<'_, T>,
    round: &Carrier<'_, T>,
    band: Band,
) -> Result<(T, bool), PrimitiveRefusal> {
    let Carrier::Plane {
        origin, outward, ..
    } = plane
    else {
        unreachable!("gap matched a plane")
    };
    let (centre, radius, convex) = match round {
        Carrier::Sphere { center, round, .. } => (*center, round.radius(), round.convex()),
        Carrier::Cylinder {
            origin: on,
            axis,
            round: r,
            ..
        } => {
            let tilt = Margin::levered(axis.dot(*outward), arm(plane, round));
            match decide(GAP_AXIS_PARALLEL_TO_PLANE, tilt, band) {
                Ok(Sign::Zero) => {}
                Ok(_) => return not_parallel("gap", GAP_AXIS_PARALLEL_TO_PLANE, plane, round),
                Err(source) => {
                    return Err(PrimitiveRefusal::Escalated {
                        predicate: GAP_AXIS_PARALLEL_TO_PLANE,
                        source,
                    });
                }
            }
            (*on, r.radius(), r.convex())
        }
        _ => unreachable!("gap matched a sphere or a cylinder"),
    };
    let h = outward.dot(centre - *origin);
    let out = beyond(h, plane, band)?;
    Ok((if out { h - radius } else { h + radius }, out == convex))
}

/// **The signed [`gap`] of `outer` against `inner`, as the measure
/// reports it** (finite, or refused), for the quieting rule's two
/// halves, which ask whether a holding `Gap ≤ −δ` over two faces
/// bounding an overlap, or a holding `Gap = 0` over a row's two cells,
/// could exist: the measure such an assertion reads, by the one
/// definition of it.
pub(crate) fn gap_value<T: Decide>(
    outer: &Carrier<'_, T>,
    inner: &Carrier<'_, T>,
    band: Band,
) -> Result<T, PrimitiveRefusal> {
    gap(outer, inner, band).and_then(finite)
}

/// **The `Gap` or the `Distance` of `a` and `b`, as the measure reports
/// it** (finite, or refused): what the quieting rule asks whether a
/// holding `= 0` over a row could exist over.
pub(crate) fn value_of<T: Decide>(
    verb: crate::measure::MeasureVerb,
    a: &Carrier<'_, T>,
    b: &Carrier<'_, T>,
    band: Band,
) -> Result<T, PrimitiveRefusal> {
    match verb {
        crate::measure::MeasureVerb::Gap => gap_value(a, b, band),
        crate::measure::MeasureVerb::Distance => distance(a, b, band).and_then(finite),
        crate::measure::MeasureVerb::Angle | crate::measure::MeasureVerb::MinClearance => {
            unreachable!("the quieting rule asks a Gap or a Distance")
        }
    }
}

/// The class a refusal names for a vertex whose link does not read.
const NO_CORNER: &str = "a vertex whose faces do not all read as planes bounded by lines";

/// The class a refusal names for an edge whose link does not read.
const NO_WEDGE: &str = "a line edge whose faces do not both read as planes with closed-form edges";

/// A face reading a line edge at its lowest point: a plane at the
/// edge's lower end, its offset affine along it; a sphere or a cylinder
/// by its carrier's closed form ([`topo::Round::lowest_on`]).
fn face_reads_edge<T: Decide>(
    face: &Carrier<'_, T>,
    line: &Carrier<'_, T>,
) -> Result<T, PrimitiveRefusal> {
    let Carrier::Line {
        ends: Some([start, end]),
        ..
    } = line
    else {
        unreachable!("gap matched a line edge whose ends read")
    };
    match round_of(face) {
        Some(round) => round
            .lowest_on((*start, *end))
            .map_err(|_| at_the_axis(face, AXIS_EDGE)),
        None => Ok(face_offset(face, *start)?.min(face_offset(face, *end)?)),
    }
}

/// A cell read exactly on a round face's axis or at its centre at a
/// scalar that moves it ([`topo::link::AtTheAxis`]): the distance from
/// the axis has no derivative there, so the gap refuses, naming why.
fn at_the_axis<T: geom_core::Real>(face: &Carrier<'_, T>, cell: &'static str) -> PrimitiveRefusal {
    PrimitiveRefusal::Unsupported(MeasureUnsupported {
        verb: "gap",
        a: face.class(),
        b: cell,
    })
}

/// [`at_the_axis`]'s cell, a vertex.
const AXIS_VERTEX: &str = "a vertex on the face's axis or centre, moved across it";

/// [`at_the_axis`]'s cell, a line edge.
const AXIS_EDGE: &str = "a line edge through the face's axis or centre, moved across it";

/// A sphere or a cylinder face's material, as `topo::link` reads it.
fn round_of<T: Decide>(face: &Carrier<'_, T>) -> Option<topo::Round<T>> {
    match face {
        Carrier::Sphere { round, .. } | Carrier::Cylinder { round, .. } => Some(*round),
        _ => None,
    }
}

/// **A vertex, or a line edge whose ends read**, as a cell arm of
/// [`gap`] reads it: where it is, and where its link is read.
#[derive(Clone, Copy)]
enum Cell<'c, 'b, T: geom_core::Real> {
    Vertex(Point3<T>, &'c CellOf<'b, T>),
    Edge([Point3<T>; 2], &'c CellOf<'b, T>),
}

impl<'c, 'b, T: Decide> Cell<'c, 'b, T> {
    fn of(c: &'c Carrier<'b, T>) -> Option<Self> {
        match c {
            Carrier::Point { at, cell } => Some(Self::Vertex(*at, cell)),
            Carrier::Line {
                ends: Some(ends),
                cell,
                ..
            } => Some(Self::Edge(*ends, cell)),
            _ => None,
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::Vertex(..) => "a vertex",
            Self::Edge(..) => "a line edge",
        }
    }
}

/// **`reading`'s link reading `read`**: a vertex's link at the vertex,
/// an edge's at its middle, reading a vertex by [`topo::Link::offset`]
/// and an edge by [`topo::Link::lowest_on`]. A reading in band refuses
/// escalated, carrying the reading; a link that does not read refuses
/// typed, naming why as the reading cell's class.
fn link_reads<T: Decide>(
    reading: Cell<'_, '_, T>,
    read: Cell<'_, '_, T>,
    band: Band,
) -> Result<T, PrimitiveRefusal> {
    let link = match reading {
        Cell::Vertex(at, cell) => cell.link(at, band),
        Cell::Edge([start, end], cell) => cell.link(start + (end - start) * T::from_f64(0.5), band),
    };
    let got = link.map_err(LinkRefusal::from).and_then(|link| match read {
        Cell::Vertex(at, _) => link.offset(at, band),
        Cell::Edge(ends, _) => link.lowest_on((ends[0], ends[1]), band),
    });
    got.or_else(|why| {
        let class = match why {
            LinkRefusal::Undecided(source) => {
                return Err(PrimitiveRefusal::Escalated {
                    predicate: source.predicate.unwrap_or(GAP_LINK),
                    source,
                });
            }
            LinkRefusal::Unread(TouchUnread::Unreadable) => match reading {
                Cell::Vertex(..) => NO_CORNER,
                Cell::Edge(..) => NO_WEDGE,
            },
            LinkRefusal::Unread(TouchUnread::InBand | TouchUnread::PieceInBand) => {
                "a cell whose material does not read at this tolerance"
            }
            LinkRefusal::Unread(TouchUnread::Unanalysed) => {
                "a cell whose faces do not close around it"
            }
            LinkRefusal::Unread(TouchUnread::Degenerate) => {
                "a cell where two of its faces lie on each other"
            }
            LinkRefusal::Unread(TouchUnread::ChordScale) => {
                "a cell with an edge too long or too short to measure"
            }
            LinkRefusal::Unread(TouchUnread::Corrupt) => "a cell whose topology does not read",
        };
        unsupported_as("gap", class, read.class())
    })
}

/// The predicate a link's escalation names where its reading named none.
const GAP_LINK: &str = "gap_link_reading";

/// **How far `at` stands out of a face's material**, along its outward
/// normal: the signed offset from a plane, and from a sphere or a
/// cylinder the signed distance to its carrier, positive away from a
/// ball's or a pin's material and toward a socket's or a bore's centre
/// ([`topo::Round::offset`]).
fn face_offset<T: Decide>(face: &Carrier<'_, T>, at: Point3<T>) -> Result<T, PrimitiveRefusal> {
    match (face, round_of(face)) {
        (_, Some(round)) => round.offset(at).map_err(|_| at_the_axis(face, AXIS_VERTEX)),
        (
            Carrier::Plane {
                origin, outward, ..
            },
            None,
        ) => Ok(outward.dot(at - *origin)),
        _ => unreachable!("the arm matched a face"),
    }
}

/// **A measured value, refused when it is not finite** — the ruled
/// door [`crate::expr::eval`] applies to a document expression,
/// [`crate::expr::refuse_non_finite`], applied to a measure for the same
/// reason: a measure is a number a reader believes, and an assertion
/// over `inf` would report a verdict about nothing.
pub(crate) fn finite<T: Decide>(value: T) -> Result<T, PrimitiveRefusal> {
    crate::expr::refuse_non_finite(value).map_err(PrimitiveRefusal::NonFinite)
}

/// **What an observed variable is worth in one evaluation** (D10): a
/// value at the evaluation's scalar, or the typed absence a measure
/// under it carries at that scalar ([`crate::MeasureUnavailableAt`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Observed<T> {
    /// The value, in kernel units.
    Value(T),
    /// A measure it reads has no value at this scalar.
    Unavailable(crate::measure::MeasureUnavailableAt),
}

/// **Why an observed variable has no reading** in an evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum ObservedRefusal {
    /// A measure defining an output it reads has no value: it failed,
    /// is poisoned, or did not run.
    Measure(super::NodeStanding),
    /// Its definition refused, or it reads a variable with no binding.
    Expr(crate::expr::EvalError),
}

impl core::fmt::Display for ObservedRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Measure(standing) => write!(f, "{standing}"),
            Self::Expr(source) => write!(f, "{source}"),
        }
    }
}

impl core::error::Error for ObservedRefusal {}

/// **`var` read at `dim`, measured values bound**: an observed
/// variable's measure outputs are bound from `results` (the values the
/// measures computed in this evaluation), and every observed definition
/// over them is evaluated over those, at `env`'s scalar; every other
/// variable reads `env`'s binding. A variable that is not observed is
/// `env`'s reading of it. `env` itself is never written: a
/// construction's environment holds no measured value
/// ([`crate::Doc::var_env`]).
///
/// # Errors
///
/// [`ObservedRefusal::Measure`] when a measure under `var` has no value
/// in `results`, and [`ObservedRefusal::Expr`] when an evaluation over
/// the bound values refuses.
pub(crate) fn observe<T: Decide, P>(
    doc: &crate::Doc<P>,
    env: &crate::expr::VarEnv<T>,
    results: &std::collections::BTreeMap<crate::RecipeNodeId, super::NodeResult<T>>,
    var: crate::VarId,
    dim: crate::Dimension,
) -> Result<Observed<T>, ObservedRefusal> {
    use crate::expr::ParamValue;
    use std::collections::BTreeSet;
    let outputs = doc.observed_outputs(var);
    if outputs.is_empty() {
        return crate::expr::eval_var(var, dim, env)
            .map(Observed::Value)
            .map_err(ObservedRefusal::Expr);
    }
    // A measure's output read directly is the value the measure
    // computed, which passed the non-finite door at the measure.
    if outputs == [var] {
        return measured(doc, results, var);
    }
    // The measured values bound over `env`, then every definition under
    // `var` that reads one re-bound in definition order: the same step
    // that binds every definition of a construction's environment.
    let mut local = env.clone();
    let mut touched: BTreeSet<crate::VarId> = BTreeSet::new();
    for &output in &outputs {
        let Some(at_dim) = doc.var(output).and_then(|held| held.kind().dimension()) else {
            continue;
        };
        match measured(doc, results, output)? {
            Observed::Value(value) => {
                local.refused.remove(&output);
                local
                    .bindings
                    .insert(output, ParamValue::Continuous { dim: at_dim, value });
            }
            unavailable @ Observed::Unavailable(_) => return Ok(unavailable),
        }
        touched.insert(output);
    }
    let under = definitions_under(doc, var);
    for id in doc.definition_order() {
        if !under.contains(&id) {
            continue;
        }
        let mut reads = Vec::new();
        if let Some(expr) = doc.var(id).and_then(|held| held.def().defined()) {
            expr.var_reads(&mut reads);
        }
        if reads.iter().any(|(read, _)| touched.contains(read)) {
            doc.bind_definition(id, &mut local);
            touched.insert(id);
        }
    }
    crate::expr::eval_var(var, dim, &local)
        .map(Observed::Value)
        .map_err(ObservedRefusal::Expr)
}

/// **`formula` evaluated with measured values bound**: [`observe`] for
/// each observed variable it reads, every other read `env`'s. The door
/// a reader outside the document takes to a measurement's value that no
/// variable holds — `Recording::measure`'s `value` before an assertion
/// lowers it.
///
/// # Errors
///
/// As [`observe`].
pub(crate) fn observe_formula<T: Decide, P>(
    doc: &crate::Doc<P>,
    env: &crate::expr::VarEnv<T>,
    results: &std::collections::BTreeMap<crate::RecipeNodeId, super::NodeResult<T>>,
    formula: &crate::Formula,
) -> Result<Observed<T>, ObservedRefusal> {
    let mut reads = Vec::new();
    formula.var_reads(&mut reads);
    let mut local = env.clone();
    for (read, dim) in reads {
        if doc.observed_outputs(read).is_empty() {
            continue;
        }
        match observe(doc, env, results, read, dim)? {
            Observed::Value(value) => {
                local
                    .bindings
                    .insert(read, crate::expr::ParamValue::Continuous { dim, value });
            }
            unavailable @ Observed::Unavailable(_) => return Ok(unavailable),
        }
    }
    crate::expr::eval(formula, &local)
        .map(Observed::Value)
        .map_err(ObservedRefusal::Expr)
}

/// **The defined variables `var`'s value is computed through**: `var`
/// itself when it is defined, and every definition its definitions
/// read, transitively.
fn definitions_under<P>(
    doc: &crate::Doc<P>,
    var: crate::VarId,
) -> std::collections::BTreeSet<crate::VarId> {
    let mut under = std::collections::BTreeSet::new();
    let mut stack = vec![var];
    while let Some(at) = stack.pop() {
        let Some(expr) = doc.var(at).and_then(|held| held.def().defined()) else {
            continue;
        };
        if !under.insert(at) {
            continue;
        }
        let mut reads = Vec::new();
        expr.var_reads(&mut reads);
        stack.extend(reads.into_iter().map(|(read, _)| read));
    }
    under
}

/// **The measure an observed variable's absence is said at**: the first
/// measure under `var` with no value at this scalar, which is what
/// [`observe`] answered [`Observed::Unavailable`] for — or, read before
/// any run, the first measure under it.
pub(crate) fn unavailable_at<P>(doc: &crate::Doc<P>, var: crate::VarId) -> crate::RecipeNodeId {
    let measures: Vec<crate::RecipeNodeId> = doc
        .observed_outputs(var)
        .into_iter()
        .filter_map(|out| doc.operation_of(out))
        .collect();
    let clearance = measures.iter().copied().find(|&node| {
        matches!(
            doc.node(node),
            Some(crate::Node::Measure {
                primitive: MeasurePrimitive::MinClearance { .. }
            })
        )
    });
    match clearance.or_else(|| measures.first().copied()) {
        Some(node) => node,
        None => unreachable!(
            "{var:?} is read as a measured value, and every reader asks that it is one"
        ),
    }
}

/// **What the measure defining `output` computed in this run.**
fn measured<T: Decide, P>(
    doc: &crate::Doc<P>,
    results: &std::collections::BTreeMap<crate::RecipeNodeId, super::NodeResult<T>>,
    output: crate::VarId,
) -> Result<Observed<T>, ObservedRefusal> {
    let Some(node) = doc.operation_of(output) else {
        return Err(ObservedRefusal::Expr(
            crate::expr::EvalError::UnresolvedVar { var: output },
        ));
    };
    let value = super::usable_in(results, node, || super::NodeStanding::NotEvaluated { node })
        .map_err(ObservedRefusal::Measure)?;
    match &value.payload {
        super::ValuePayload::Measure { value, .. } => Ok(Observed::Value(*value)),
        super::ValuePayload::MeasureUnavailable { reason, .. } => {
            Ok(Observed::Unavailable(*reason))
        }
        _ => Err(ObservedRefusal::Expr(crate::expr::EvalError::OutputRead {
            var: output,
        })),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::panic)]

    use super::*;

    /// **`curve_reach` bounds an ellipse in any stored frame.** The
    /// semi-axes carry no order and no sign, so an ellipse stored with
    /// `minor` the larger, or `major` negative, reaches as far as its
    /// larger magnitude. Read at the stored `major` it under-reached —
    /// and an under-estimate here certifies a parallelism that does not
    /// hold. Dense samples of the whole ellipse against the bound.
    #[test]
    fn the_reach_bounds_an_ellipse_in_any_stored_frame() {
        let origin = Point3::new(0.3, -0.2, 0.1);
        let center = Point3::new(1.0, 2.0, -0.5);
        for (major, minor) in [(0.5, 3.0), (-3.0, 0.5), (-0.5, -3.0), (3.0, -0.5)] {
            let e = Curve3::Ellipse {
                center,
                axis: geom_core::Vec3::new(0.0, 0.0, 1.0),
                semi_u: major,
                semi_v: minor,
                u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
            };
            let reach = curve_reach(&e, 0.0, core::f64::consts::TAU, origin).expect("a conic");
            let far = (0..=4000)
                .map(|k| (e.eval(core::f64::consts::TAU * f64::from(k) / 4000.0) - origin).norm())
                .fold(0.0_f64, f64::max);
            assert!(
                far <= reach,
                "({major}, {minor}): a point {far} out against a reach of {reach}"
            );
        }
    }

    fn band() -> Band {
        Band::linear(geom_core::Tol::witness()).expect("the witness tolerance is a band")
    }

    fn p(x: f64, y: f64, z: f64) -> Point3<f64> {
        Point3::new(x, y, z)
    }

    fn v(x: f64, y: f64, z: f64) -> Vec3<f64> {
        Vec3::new(x, y, z)
    }

    fn plane(origin: Point3<f64>, outward: Vec3<f64>) -> Carrier<'static, f64> {
        Carrier::Plane {
            origin,
            normal: outward,
            outward,
            reach: 10.0,
        }
    }

    /// A body holding nothing, so a cell's link off it refuses: the face
    /// arms read no cell's link.
    fn nobody() -> &'static Body<f64> {
        Box::leak(Box::new(Body::new()))
    }

    fn line(start: Point3<f64>, end: Point3<f64>) -> Carrier<'static, f64> {
        Carrier::Line {
            dir: (end - start) / (end - start).norm(),
            ends: Some([start, end]),
            cell: CellOf::Edge(nobody(), topo::entity::EdgeKey::default()),
        }
    }

    fn vertex(at: Point3<f64>) -> Carrier<'static, f64> {
        Carrier::Point {
            at,
            cell: CellOf::Vertex(nobody(), topo::entity::VertexKey::default()),
        }
    }

    fn read(outer: &Carrier<'_, f64>, inner: &Carrier<'_, f64>) -> f64 {
        gap(outer, inner, band()).unwrap_or_else(|e| panic!("the gap reads: {e:?}"))
    }

    fn close(got: f64, want: f64, what: &str) {
        assert!((got - want).abs() < 1e-12, "{what}: {got}, not {want}");
    }

    /// **A vertex or a line edge on a face reads the face's signed
    /// offset, in either seat**: zero on it, positive off its material,
    /// negative in it, for a plane, a ball, a socket, a pin and a bore.
    /// An edge reads at its lowest point: an end on a plane, a socket or
    /// a bore, and where it passes nearest the centre or the axis on a
    /// ball or a pin. Red if a seat flips the sign, if a concave face
    /// reads as a convex one, or if an edge reads at its higher end or
    /// across a ball at its ends.
    #[test]
    fn a_face_reads_a_vertex_or_an_edge_by_its_outward_normal() {
        let floor = plane(p(0.0, 0.0, 1.0), v(0.0, 0.0, 1.0));
        for h in [0.0, 0.01, -0.01] {
            let at = vertex(p(0.3, -0.2, 1.0 + h));
            close(
                read(&floor, &at),
                h,
                "a vertex over the floor, outer the face",
            );
            close(
                read(&at, &floor),
                h,
                "a vertex over the floor, outer the vertex",
            );
            let tilted = line(p(0.0, 0.0, 1.0 + h), p(1.0, 0.0, 1.5 + h));
            close(
                read(&floor, &tilted),
                h,
                "a tilted edge reads at its lower end",
            );
            close(read(&tilted, &floor), h, "and in either seat");
        }
        let sphere = |convex| Carrier::Sphere {
            center: p(0.0, 0.0, 0.0),
            radius: 1.0,
            round: topo::Round::sphere(p(0.0, 0.0, 0.0), 1.0, convex),
        };
        let cylinder = |convex| Carrier::Cylinder {
            origin: p(0.0, 0.0, 0.0),
            axis: v(0.0, 0.0, 1.0),
            radius: 1.0,
            reach: 5.0,
            round: topo::Round::cylinder(p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), 1.0, convex),
        };
        let out = vertex(p(1.01, 0.0, 0.0));
        close(read(&sphere(true), &out), 0.01, "outside a ball");
        close(read(&sphere(false), &out), -0.01, "in a socket's material");
        close(read(&cylinder(true), &out), 0.01, "outside a pin");
        close(read(&out, &cylinder(false)), -0.01, "in a bore's material");
        let ruling = line(p(0.99, 0.0, -1.0), p(0.99, 0.0, 1.0));
        close(
            read(&cylinder(true), &ruling),
            -0.01,
            "a ruling inside a pin",
        );
        close(
            read(&ruling, &cylinder(false)),
            0.01,
            "a ruling inside a bore's hollow",
        );
        // An edge across the axis, passing it at 0.5: its least is there
        // against a ball or a pin, and at an end against a socket or a bore.
        let across = line(p(-2.0, 0.5, 0.3), p(2.0, 0.5, 0.3));
        let far = 1.0 - 4.25_f64.sqrt();
        close(read(&cylinder(true), &across), -0.5, "an edge across a pin");
        close(
            read(&across, &cylinder(false)),
            far,
            "an edge across a bore",
        );
        let rel = (0.25_f64 + 0.09).sqrt();
        close(
            read(&sphere(true), &across),
            rel - 1.0,
            "an edge through a ball",
        );
        close(
            read(&across, &sphere(false)),
            1.0 - (4.34_f64).sqrt(),
            "an edge through a socket",
        );
    }

    /// **A vertex or an edge whose link does not read refuses typed**,
    /// naming the reading cell: an edge reading a vertex in either seat,
    /// and a vertex reading a vertex, each off a body holding neither. The links themselves are
    /// `topo::link`'s rows, and the real bodies' are
    /// `gap_reads_vertex_and_edge_cells`. Red if a cell with no link reads
    /// a number, or an edge stops reading a vertex in the vertex's seat.
    #[test]
    fn a_cell_with_no_link_refuses_naming_the_reading_cell() {
        let edge = line(p(0.0, -1.0, 0.0), p(0.0, 1.0, 0.0));
        let tip = vertex(p(0.01, 0.5, -0.5));
        let refused =
            |outer: &Carrier<'_, f64>, inner: &Carrier<'_, f64>| match gap(outer, inner, band()) {
                Err(PrimitiveRefusal::Unsupported(u)) => u.a,
                other => panic!("refused typed: {other:?}"),
            };
        assert_eq!(refused(&edge, &tip), NO_WEDGE, "the edge reads the vertex");
        assert_eq!(refused(&tip, &edge), NO_WEDGE, "in either seat");
        assert_eq!(
            refused(&tip, &vertex(p(0.0, 0.0, 0.0))),
            "a cell whose topology does not read"
        );
        assert_eq!(
            refused(&edge, &line(p(1.0, 0.0, 0.0), p(2.0, 0.0, 0.0))),
            NO_WEDGE
        );
    }

    /// **`angle` reads a line edge whose ends do not read**: the gap's
    /// arms read the ends, the angle only the direction. Red if a line
    /// edge needs its ends to be a [`Carrier::Line`].
    #[test]
    fn an_angle_reads_a_line_edge_without_its_ends() {
        let bare = |dir| Carrier::<f64>::Line {
            dir,
            ends: None,
            cell: CellOf::Edge(nobody(), topo::entity::EdgeKey::default()),
        };
        let got = angle(&bare(v(1.0, 0.0, 0.0)), &bare(v(0.0, 1.0, 0.0)))
            .unwrap_or_else(|e| panic!("the angle reads: {e:?}"));
        close(got, core::f64::consts::FRAC_PI_2, "a right angle");
        assert!(
            matches!(
                gap(&bare(v(1.0, 0.0, 0.0)), &bare(v(0.0, 1.0, 0.0)), band()),
                Err(PrimitiveRefusal::Unsupported(MeasureUnsupported {
                    a: "a line edge whose ends do not read",
                    ..
                }))
            ),
            "the gap refuses typed"
        );
    }

    fn pin(
        origin: Point3<f64>,
        axis: Vec3<f64>,
        radius: f64,
        convex: bool,
    ) -> Carrier<'static, f64> {
        Carrier::Cylinder {
            origin,
            axis,
            radius,
            reach: 10.0,
            round: topo::Round::cylinder(origin, axis, radius, convex),
        }
    }

    fn ball(center: Point3<f64>, radius: f64, convex: bool) -> Carrier<'static, f64> {
        Carrier::Sphere {
            center,
            radius,
            round: topo::Round::sphere(center, radius, convex),
        }
    }

    /// **A curved face reads against a plane or another round face by
    /// both faces' material sides**, each pair at zero where it touches
    /// and by its clearance where it stands off: a rod and a ball on a
    /// plate, a key under a bore's crown, a stadium's flat into its
    /// round end, a slot's flat into its round end; two rods side by
    /// side, two balls, a pin in a bore read either way round, two
    /// coaxial rods end to end and a thin rod inside a fat one. An
    /// opposed pair reads the same in either role order, an aligned one
    /// negates. Red if two touching rods read `−2r`, a plane and a round
    /// face have no arm, a stadium reads `−2r`, or a role swap keeps an
    /// aligned pair's sign.
    #[test]
    fn a_round_face_reads_by_both_material_sides() {
        let x = v(1.0, 0.0, 0.0);
        let z = v(0.0, 0.0, 1.0);
        let top = plane(p(0.0, 0.0, 0.0), z);
        let r = 0.5;
        // (what, the round face for a standoff `s`, opposed)
        type Round = fn(f64, f64) -> Carrier<'static, f64>;
        let on_the_plate: [(&str, Round, bool); 6] = [
            (
                "a rod on a plate",
                |r, s| pin(p(0.0, 0.0, r + s), v(1.0, 0.0, 0.0), r, true),
                true,
            ),
            (
                "a ball on a plate",
                |r, s| ball(p(0.0, 0.0, r + s), r, true),
                true,
            ),
            (
                "a key under a bore's crown",
                |r, s| pin(p(0.0, 0.0, s - r), v(1.0, 0.0, 0.0), r, false),
                true,
            ),
            (
                "a stadium's flat into its round end",
                |r, s| pin(p(0.0, 0.0, s - r), v(1.0, 0.0, 0.0), r, true),
                false,
            ),
            (
                "a slot's flat into its round end",
                |r, s| pin(p(0.0, 0.0, r + s), v(1.0, 0.0, 0.0), r, false),
                false,
            ),
            (
                "a socket's flat into its round",
                |r, s| ball(p(0.0, 0.0, r + s), r, false),
                false,
            ),
        ];
        for (what, round, opposed) in on_the_plate {
            for s in [0.0, 0.1, -0.1] {
                let face = round(r, s);
                // The key, the stadium and the socket stand off as their
                // crown rises; the others as their centre does.
                let want = s;
                close(
                    read(&top, &face),
                    want,
                    &format!("{what} at {s}, the plate outer"),
                );
                let swapped = if opposed { want } else { -want };
                close(
                    read(&face, &top),
                    swapped,
                    &format!("{what} at {s}, the round outer"),
                );
            }
        }
        let (r1, r2) = (0.5, 0.3);
        for s in [0.0, 0.1, -0.1] {
            let d = r1 + r2 + s;
            for (a, b, what) in [
                (
                    pin(p(0.0, 0.0, 0.0), z, r1, true),
                    pin(p(d, 0.0, 0.0), z, r2, true),
                    "two rods side by side",
                ),
                (
                    ball(p(0.0, 0.0, 0.0), r1, true),
                    ball(p(d, 0.0, 0.0), r2, true),
                    "two balls",
                ),
            ] {
                close(read(&a, &b), s, &format!("{what} at {s}"));
                close(read(&b, &a), s, &format!("{what} at {s}, swapped"));
            }
            // A pin of r2 in a bore of r1 + s, off its axis by r1 − r2.
            let bore = pin(p(0.0, 0.0, 0.0), z, r1 + s, false);
            let inside = pin(p(r1 - r2, 0.0, 0.0), z, r2, true);
            close(read(&bore, &inside), s, &format!("a pin in a bore at {s}"));
            close(
                read(&inside, &bore),
                s,
                &format!("a pin in a bore at {s}, swapped"),
            );
            // Nested and aligned: the inner sticks out by `s`.
            let fat = pin(p(0.0, 0.0, 0.0), z, 1.0, true);
            let thin = pin(p(0.7 + s, 0.0, 0.0), z, 0.3, true);
            close(
                read(&fat, &thin),
                s,
                &format!("a thin rod inside a fat one at {s}"),
            );
            let end = pin(p(0.0, 0.0, 0.0), x, r1, true);
            let on = pin(p(3.0, 0.0, 0.0), x, r1 + s, true);
            close(
                read(&end, &on),
                s,
                &format!("two coaxial rods end to end at {s}"),
            );
            close(
                read(&on, &end),
                -s,
                &format!("two coaxial rods end to end at {s}, swapped"),
            );
        }
    }

    /// **A role swap keeps an opposed round pair and negates an aligned
    /// one, off the axis**, for every pair of senses in every
    /// configuration, each read where it neither touches nor sits on one
    /// axis (`d ≠ 0`). Red if an arm reads a pair by its senses alone,
    /// or the swap of a nested pair is read as an external one.
    #[test]
    fn a_role_swap_keeps_an_opposed_round_pair_and_negates_an_aligned_one() {
        let z = v(0.0, 0.0, 1.0);
        let at = |x: f64, r: f64, convex: bool| pin(p(x, 0.0, 0.0), z, r, convex);
        for (what, a, b, opposed) in [
            (
                "two rods apart",
                at(0.0, 0.5, true),
                at(1.0, 0.3, true),
                true,
            ),
            (
                "two bores apart",
                at(0.0, 0.5, false),
                at(1.0, 0.3, false),
                true,
            ),
            (
                "a rod in a bore's wall",
                at(0.0, 0.5, false),
                at(1.0, 0.3, true),
                false,
            ),
            (
                "a thin rod in a fat one",
                at(0.0, 1.0, true),
                at(0.2, 0.3, true),
                false,
            ),
            (
                "a small hollow in a big one",
                at(0.0, 1.0, false),
                at(0.2, 0.3, false),
                false,
            ),
            (
                "a pin in a bore",
                at(0.0, 1.0, false),
                at(0.2, 0.3, true),
                true,
            ),
            (
                "a hole through a rod",
                at(0.0, 1.0, true),
                at(0.2, 0.3, false),
                true,
            ),
        ] {
            let (ab, ba) = (read(&a, &b), read(&b, &a));
            assert!(ab.abs() > 1e-6, "{what}: a reading off the touch, {ab}");
            let want = if opposed { ab } else { -ab };
            close(ba, want, &format!("{what}: swapped against {ab}"));
        }
    }

    /// **A round face's centre or axis on the other carrier refuses**:
    /// which side of it the centre lies is the configuration the gap
    /// reads, and in band it is not decided. Red if a centre on a plane,
    /// or a rod's axis on another rod's wall, reads a number.
    #[test]
    fn a_round_centre_on_the_other_carrier_refuses() {
        let z = v(0.0, 0.0, 1.0);
        let top = plane(p(0.0, 0.0, 0.0), z);
        for (outer, inner, what) in [
            (
                top.clone(),
                ball(p(0.0, 0.0, 1e-12), 0.5, true),
                "a ball centred on a plate",
            ),
            (
                pin(p(0.0, 0.0, 0.0), z, 1.0, true),
                pin(p(1.0, 0.0, 0.0), z, 0.3, true),
                "a rod's axis on another's wall",
            ),
        ] {
            assert!(
                gap(&outer, &inner, band()).is_err(),
                "{what}: {:?}",
                gap(&outer, &inner, band())
            );
        }
    }

    // --- R1 review probes (PR 6103) -----------------------------------

    /// R1 probe: a thin rod internally tangent inside a fat one is an
    /// ALIGNED (nested) pair, so the PR's role-swap rule says reading it
    /// with the thin rod as `outer` negates the fat-outer reading (0 at
    /// the touch). Red: the swapped read classifies the pair "external"
    /// (the fat's axis lies outside the thin's carrier) and reads −2r.
    #[test]
    fn r1_probe_a_nested_rod_read_thin_outer_reads_zero_at_its_touch() {
        let z = v(0.0, 0.0, 1.0);
        let fat = pin(p(0.0, 0.0, 0.0), z, 1.0, true);
        let thin = pin(p(0.7, 0.0, 0.0), z, 0.3, true);
        close(read(&fat, &thin), 0.0, "fat outer");
        close(
            read(&thin, &fat),
            0.0,
            "thin outer: nested is aligned, -0 = 0",
        );
    }

    /// R1 probe: the same for two balls.
    #[test]
    fn r1_probe_a_nested_ball_read_small_outer_reads_zero_at_its_touch() {
        let big = ball(p(0.0, 0.0, 0.0), 1.0, true);
        let small = ball(p(0.7, 0.0, 0.0), 0.3, true);
        close(read(&big, &small), 0.0, "big outer");
        close(read(&small, &big), 0.0, "small outer");
    }

    /// R1 probe: the ruling's margin at a tangency is ±r, so a real
    /// touch never lands in band. A thin rod of r = 0.5 internally
    /// tangent inside a fat one of R = 1 (d = 0.5), read thin-outer:
    /// `gap_round_side`'s margin is d − r_out = 0.5 − 0.5 = 0. Red: a
    /// real tangency refuses in band.
    #[test]
    fn r1_probe_a_real_touch_never_lands_in_band() {
        let z = v(0.0, 0.0, 1.0);
        let fat = pin(p(0.0, 0.0, 0.0), z, 1.0, true);
        let thin = pin(p(0.5, 0.0, 0.0), z, 0.5, true);
        close(read(&fat, &thin), 0.0, "fat outer");
        let swapped = gap(&thin, &fat, band());
        assert!(swapped.is_ok(), "thin outer, a real touch: {swapped:?}");
    }

    /// R1 probe: two bores side by side, tangent (a figure-eight hole):
    /// the concave twin of "two rods side by side". Red: two concave
    /// faces always read `r_out − r_in − d`, so the touch reads −2r_in,
    /// the very −2r the PR fixes for rods.
    #[test]
    fn r1_probe_two_bores_side_by_side_read_zero_at_their_touch() {
        let z = v(0.0, 0.0, 1.0);
        let (r1, r2) = (0.5, 0.3);
        let a = pin(p(0.0, 0.0, 0.0), z, r1, false);
        let b = pin(p(r1 + r2, 0.0, 0.0), z, r2, false);
        close(read(&a, &b), 0.0, "two bores side by side");
        close(read(&b, &a), 0.0, "two bores side by side, swapped");
    }

    /// R1 probe: nested hollows (a small bore internally tangent inside a
    /// big one) are aligned, so the swap negates: 0 at the touch either
    /// way. Red: the small-outer read is r_small − r_big − d = −2d.
    #[test]
    fn r1_probe_nested_hollows_swapped_read_zero_at_their_touch() {
        let z = v(0.0, 0.0, 1.0);
        let big = pin(p(0.0, 0.0, 0.0), z, 1.0, false);
        let small = pin(p(0.7, 0.0, 0.0), z, 0.3, false);
        close(read(&big, &small), 0.0, "big outer");
        close(read(&small, &big), 0.0, "small outer");
    }

    /// R1 probe (claim 3), re-pinned to the ruling: `Distance(axis,
    /// axis)` is the distance between two lines, zero for two axes that
    /// meet at a right angle as for two that are one. A coaxial row is
    /// said only by the parallel reading, which the quieting rule asks
    /// (`atrest_operation_rows_quiet.rs`
    /// `r2_probe_meeting_axes_do_not_say_coaxial`); parallel axes apart
    /// read their separation.
    #[test]
    fn r1_probe_two_meeting_axes_read_zero_and_not_parallel() {
        let axis = |through, dir| Carrier::Pose {
            pose: Pose::Axis { through, dir },
            reach: Some(1.0),
        };
        let a = axis(p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0));
        let b = axis(p(0.0, 0.0, 0.5), v(1.0, 0.0, 0.0));
        let got = distance(&a, &b, band()).expect("reads");
        close(got, 0.0, "two axes that meet");
        assert_eq!(
            axes_parallel(&a, &b, band()),
            Some(false),
            "and not parallel"
        );
        let c = axis(p(0.0, 0.3, 0.0), v(0.0, 0.0, 1.0));
        close(
            distance(&a, &c, band()).expect("reads"),
            0.3,
            "two parallel axes apart",
        );
        assert_eq!(axes_parallel(&a, &c, band()), Some(true), "parallel");
    }

    /// **R2 probe: an aligned nested pair negates under a role swap.**
    /// A thin rod inside a fat one, touching inside (and standing out
    /// by `s`), read with the THIN one as outer: aligned, so the swap of
    /// `a_round_face_reads_by_both_material_sides`'s `s` is `-s`. The
    /// same for two balls nested and two bores nested.
    #[test]
    fn r2_probe_nested_pair_swapped_reads_minus_s() {
        let z = v(0.0, 0.0, 1.0);
        for s in [0.0, 0.1, -0.1] {
            let fat = pin(p(0.0, 0.0, 0.0), z, 1.0, true);
            let thin = pin(p(0.7 + s, 0.0, 0.0), z, 0.3, true);
            let got = gap(&thin, &fat, band());
            assert!(
                matches!(&got, Ok(g) if (g + s).abs() < 1e-12),
                "thin rod inside a fat one at {s}, thin outer: {got:?}, want {}",
                -s
            );
        }
    }

    #[test]
    fn r2_probe_nested_balls_swapped_reads_minus_s() {
        for s in [0.0, 0.1, -0.1] {
            let fat = ball(p(0.0, 0.0, 0.0), 1.0, true);
            let small = ball(p(0.7 + s, 0.0, 0.0), 0.3, true);
            let got = gap(&small, &fat, band());
            assert!(
                matches!(&got, Ok(g) if (g + s).abs() < 1e-12),
                "small ball inside a fat one at {s}, small outer: {got:?}"
            );
        }
    }

    /// Two bores nested (a small hole inside a big hole, tangent inside):
    /// aligned like the thin rod in the fat one; touching reads 0 in
    /// both orders.
    #[test]
    fn r2_probe_nested_bores_touch_reads_zero_either_way() {
        let z = v(0.0, 0.0, 1.0);
        let big = pin(p(0.0, 0.0, 0.0), z, 1.0, false);
        let small = pin(p(0.7, 0.0, 0.0), z, 0.3, false);
        let a = gap(&big, &small, band());
        let b = gap(&small, &big, band());
        assert!(
            matches!((&a, &b), (Ok(x), Ok(y)) if x.abs() < 1e-12 && y.abs() < 1e-12),
            "nested bores touching: big outer {a:?}, small outer {b:?}"
        );
    }

    /// Two bores side by side, their walls touching (the web between
    /// them zero): opposed, locally a face-to-face touch; reads 0.
    #[test]
    fn r2_probe_bores_side_by_side_touch_reads_zero() {
        let z = v(0.0, 0.0, 1.0);
        let a = pin(p(0.0, 0.0, 0.0), z, 0.5, false);
        let b = pin(p(0.8, 0.0, 0.0), z, 0.3, false);
        let x = gap(&a, &b, band());
        let y = gap(&b, &a, band());
        assert!(
            matches!((&x, &y), (Ok(x), Ok(y)) if x.abs() < 1e-12 && y.abs() < 1e-12),
            "two bores side by side touching: {x:?}, swapped {y:?}"
        );
    }

    /// A rod outside a bore, tangent to the hole from within the wall
    /// (the bore's material around it): the configuration the senses
    /// leave open for a convex x concave pair as well.
    #[test]
    fn r2_probe_rod_in_the_wall_touching_the_hole_reads_zero() {
        let z = v(0.0, 0.0, 1.0);
        let bore = pin(p(0.0, 0.0, 0.0), z, 0.5, false);
        let rod = pin(p(0.8, 0.0, 0.0), z, 0.3, true);
        let x = gap(&bore, &rod, band());
        assert!(
            matches!(&x, Ok(x) if x.abs() < 1e-12),
            "a rod in a bore's wall touching the hole: {x:?}"
        );
    }

    /// **R2 probe: a real touch lands in `gap_round_side`'s band.** A
    /// rod of 0.3 inside one of 0.6, touching inside, read with the thin
    /// one as outer: the margin `d − r_out` is `0.3 − 0.3 = 0`, so the
    /// gap refuses at a tangency. Red if it refuses.
    #[test]
    fn r2_probe_a_nested_touch_is_not_in_band() {
        let z = v(0.0, 0.0, 1.0);
        let fat = pin(p(0.0, 0.0, 0.0), z, 0.6, true);
        let thin = pin(p(0.3, 0.0, 0.0), z, 0.3, true);
        let got = gap(&thin, &fat, band());
        assert!(got.is_ok(), "a touch refuses: {got:?}");
    }
}
