//! **A face's carrier read along a sketch piece** (D10 `[coincidence]`,
//! "Constructing a coincidence"): where a line or a circle in a sketch's
//! plane crosses a face's carrier, in which sense, and the carrier's
//! outward normal there.
//!
//! A trace is the carrier cut by the sketch's plane, in closed form; a
//! piece of the sketch lies in that plane, so where the piece meets the
//! trace is where it meets the carrier. This door reads the piece
//! against the carrier itself, through the curve × face crossing
//! reading ([`geom_brep::curve_face::cross`]): the pose first (the piece
//! lies on the carrier, passes through a cone's apex, or grazes it), then
//! its meetings, each read for its sense. It reads no face boundary: the
//! trace is the carrier's.
//!
//! The sense of a crossing is the face's: a piece ENTERS the face's
//! material where it runs against the face's outward normal, and LEAVES
//! where it runs along it. A crossing whose sense the band cannot call,
//! a tangency, or two crossings the band cannot order graze the carrier,
//! and refuse.

use geom::{Curve3, Surface, SurfaceKind};
use geom_brep::curve_face::{CrossRefusal, CrossWhy, OnSpan, Pose, PoseRefusal, cross};
use geom_brep::curve_pair::{MeetKind, Span, SpanEnd};
use geom_brep::roots::Unsettled;
use geom_core::{Band, Decide, Indeterminate, Margin, Point3, Real, Sign, Vec3};

use crate::validate::decide;

/// The funnel row a crossing's sense is decided on: the cosine between
/// the piece's direction and the outward normal, levered by the piece's
/// reach.
pub const TRACE_CROSSING_SENSE: &str = "trace_crossing_sense";
/// The funnel row a plane's lead is decided on ([`plane_misses`]): how
/// far it turns off parallel to a plane or a cylinder's axis.
pub const TRACE_PLANE_LEAD: &str = "trace_plane_lead";

/// Which way a piece passes a face's carrier at a crossing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Passage {
    /// Into the face's material: against its outward normal.
    Enter,
    /// Out of the face's material: along its outward normal.
    Leave,
}

/// **One crossing of a piece with a face's carrier.**
#[derive(Debug, Clone, Copy)]
pub struct Crossing<T: Real> {
    /// How far along the piece: a length along a line, an angle along
    /// a circle, from the pen.
    pub along: T,
    /// The point.
    pub point: Point3<T>,
    /// Its sense.
    pub passage: Passage,
    /// The face's outward gradient there: the carrier's implicit
    /// gradient with the face's sense folded in, the outward normal's
    /// direction, unnormalized. The trace's tangent there is it turned a
    /// quarter turn in the sketch's plane, and its ratio is all a
    /// heading reads, so no reader divides by its length.
    pub outward: Vec3<T>,
}

/// **Why a piece cannot be read against a carrier.**
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TraceFault {
    /// A carrier with no closed-form section: a spline, an
    /// approximating surface.
    Unsupported {
        /// The carrier's kind.
        carrier: SurfaceKind,
    },
    /// A closed-form carrier whose data bounds no region
    /// ([`Surface::cover`]'s refusal): a datum that is not a finite
    /// number.
    Unbounded {
        /// The carrier's kind.
        carrier: SurfaceKind,
        /// What its cover did not know.
        what: geom::cover::Unknown,
    },
    /// The piece grazes the carrier: a tangency, or a double root.
    Grazing,
    /// The piece crosses the carrier at a cone's apex, where it has no
    /// tangent plane.
    Apex,
    /// The piece lies on the carrier.
    OnCarrier,
    /// A sign the band cannot call: the sliver band.
    Escalated(Indeterminate),
    /// A root count the door cannot certify: a root door's own
    /// `Unsettled`, past the two this door names itself (a double root
    /// is [`Self::Grazing`], a root at a cone's apex [`Self::Apex`]).
    /// No document row reaches it: those doors' suites exercise each
    /// cause on the pairs that raise it (a meter or a budget refused, a
    /// count the residual cannot have), and no line or circle in a
    /// sketch plane against a closed-form carrier lands on one.
    Unsettled(Unsettled),
    /// A torus carrier outside its ring convention, the convention's
    /// refusal carried whole ([`geom_brep::TorusRefusal`]): its root doors
    /// read only a ring.
    Torus(geom_brep::TorusRefusal),
}

impl core::fmt::Display for TraceFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unsupported { carrier } => {
                write!(f, "a {} carrier has no closed-form trace", carrier.name())
            }
            Self::Unbounded { carrier, what } => write!(
                f,
                "the {} carrier bounds no region ({})",
                carrier.name(),
                match what {
                    geom::cover::Unknown::Unbounded => "its locus is unbounded",
                    geom::cover::Unknown::PoisonedNet => "its control net carries poison",
                    geom::cover::Unknown::NonFinite => "a datum of it is not a finite number",
                }
            ),
            Self::Grazing => f.write_str("the piece grazes the carrier"),
            Self::Apex => f.write_str("the piece crosses the cone at its apex"),
            Self::OnCarrier => f.write_str("the piece lies on the carrier"),
            Self::Escalated(diag) => write!(f, "a crossing in the sliver band ({diag})"),
            Self::Unsettled(why) => write!(
                f,
                "a root count the door cannot certify ({})",
                why.sentence()
            ),
            Self::Torus(refusal) => f.write_str(&refusal.said("the carrier's", None)),
        }
    }
}

impl core::error::Error for TraceFault {}

/// **A crossing reading's refusal, as the trace's**: its first window's.
/// A pose in band escalates, a torus outside the ring convention is the
/// convention's, an unbuilt or a spline arm has no closed-form trace, a
/// meeting reading in band grazes, and a limit is the door's own
/// (a tangency grazes, a root at a cone's apex is the apex's).
///
/// # Panics
///
/// On a kernel fault: the pose's or the method's readings contradict.
fn refused<T: Real>(surface: &Surface<T>, refusal: &CrossRefusal<T>) -> TraceFault {
    use geom_brep::curve_pair::MeetFault;
    let Some(window) = refusal.windows.first() else {
        unreachable!("a crossing refusal holds a window");
    };
    match window.why {
        CrossWhy::Pose(PoseRefusal::InBand { diag, .. }) => TraceFault::Escalated(diag),
        CrossWhy::Pose(PoseRefusal::Torus(torus)) => TraceFault::Torus(torus),
        CrossWhy::Pose(PoseRefusal::Unbuilt(_) | PoseRefusal::Enclosed) => {
            TraceFault::Unsupported {
                carrier: surface.kind(),
            }
        }
        CrossWhy::Pose(PoseRefusal::Meter { row, .. }) => {
            TraceFault::Unsettled(Unsettled::Meter { row })
        }
        CrossWhy::Pose(PoseRefusal::Fault(what)) => {
            unreachable!("the curve × face pose faulted: {what}")
        }
        CrossWhy::Method { fault, .. } => match (fault, fault.kind()) {
            (_, MeetKind::Escalated(diag)) => TraceFault::Escalated(diag),
            (_, MeetKind::Touch(_)) | (MeetFault::Unsettled(Unsettled::Tangency), _) => {
                TraceFault::Grazing
            }
            (MeetFault::Unsettled(Unsettled::AtApex), _) => TraceFault::Apex,
            (MeetFault::Unsettled(why), _) => TraceFault::Unsettled(why),
            (_, MeetKind::Unread(_)) => unreachable!("an unread meeting reading of no limit"),
            (_, MeetKind::Fault { what }) => {
                unreachable!("a curve × face method faulted: {what}")
            }
        },
    }
}

/// **The piece's meetings with the carrier, by the pose first**: one
/// lying on it, through a cone's apex or grazing it refuses; apart, none;
/// otherwise each meeting's parameter and point, ascending, past the
/// ends `skip` names.
fn meetings<T: Decide>(
    span: &Span<'_, T>,
    surface: &Surface<T>,
    skip: &[SpanEnd],
    band: Band,
) -> Result<Vec<(T, Point3<T>)>, TraceFault> {
    let found = cross(span, surface, band).map_err(|refusal| refused(surface, &refusal))?;
    match found.pose {
        Pose::Lies => return Err(TraceFault::OnCarrier),
        Pose::Through(_) => return Err(TraceFault::Apex),
        Pose::Tangent(_) => return Err(TraceFault::Grazing),
        Pose::Apart | Pose::Isolated(_) => {}
    }
    let (t0, t1) = span.params();
    Ok(found
        .meetings
        .into_iter()
        .filter_map(|m| match m.on_span {
            OnSpan::Inside { t } => Some((t, m.point)),
            OnSpan::End(end) if skip.contains(&end) => None,
            OnSpan::End(SpanEnd::Stop) => Some((t1, m.point)),
            OnSpan::End(SpanEnd::Start) => Some((t0, m.point)),
        })
        .collect())
}

fn sign_as(
    name: &'static str,
    margin: Margin<impl Decide>,
    band: Band,
) -> Result<Sign, TraceFault> {
    decide(name, margin, band).map_err(TraceFault::Escalated)
}

/// **The crossings of `ray` with `surface`**, nearest first, each ahead of
/// its start. Its `dir` is unit; `reach` is a length the size of the
/// scene the piece runs through, the run the crossing reading reads and
/// the lever its senses are read over. `sense` is the face's orientation
/// sense.
///
/// # Errors
///
/// [`TraceFault`].
pub fn ray_crossings<T: Decide>(
    surface: &Surface<T>,
    sense: bool,
    ray: &geom::Ray<T>,
    reach: T,
    band: Band,
) -> Result<Vec<Crossing<T>>, TraceFault> {
    let dir = ray.line().dir;
    // Unsourced lever: the ray's length, a scene size the caller hands
    // rather than geometry the extent home reads.
    let arm = geom_core::Lever::unsourced(reach);
    let carrier = Curve3::Line(ray.line());
    let span = Span::new(&carrier, (ray.start(), ray.from_start(reach)));
    meetings(&span, surface, &[SpanEnd::Start], band)?
        .into_iter()
        .map(|(t, point)| crossing(surface, sense, t, point, dir, arm, band))
        .collect()
}

/// **The crossings of the circle about `center` in the plane normal to
/// `axis`, of `radius`, starting at `center + radius·u_ref` and turning
/// right-handed about `axis`, with `surface`**: each strictly past the
/// start and short of a whole turn on, nearest first. `axis` and `u_ref`
/// are unit and orthogonal.
///
/// # Errors
///
/// [`TraceFault`].
pub fn circle_crossings<T: Decide>(
    surface: &Surface<T>,
    sense: bool,
    (center, axis, radius, u_ref): (Point3<T>, Vec3<T>, T, Vec3<T>),
    band: Band,
) -> Result<Vec<Crossing<T>>, TraceFault> {
    let v_ref = axis.cross(u_ref);
    // The circle's period: its parameter is its angle.
    let tau = geom::conic_period::<T>().length();
    let carrier = Curve3::Circle(geom::Circle::new(
        geom::Frame {
            origin: center,
            axis,
            u_ref,
        },
        radius,
    ));
    let reach = geom_brep::ExtentBall::of_covered(
        geom::cover::Cover::circle(geom::cover::Core {
            axis: geom::Line {
                origin: center,
                dir: axis,
            },
            radius,
        })
        .as_ref(),
    )
    .lever();
    let span = Span::new(&carrier, (T::zero(), tau));
    meetings(&span, surface, &[SpanEnd::Start, SpanEnd::Stop], band)?
        .into_iter()
        .map(|(theta, point)| {
            let (s, c) = theta.sin_cos();
            let velocity = v_ref * c - u_ref * s;
            crossing(surface, sense, theta, point, velocity, reach, band)
        })
        .collect()
}

/// One crossing at `point`, `along` the piece, its sense read off the
/// piece's unit `velocity` against the outward normal: the cosine
/// between them, the outward gradient's component along the velocity
/// over its length.
fn crossing<T: Decide>(
    surface: &Surface<T>,
    sense: bool,
    along: T,
    point: Point3<T>,
    velocity: Vec3<T>,
    lever: geom_core::Lever<T>,
    band: Band,
) -> Result<Crossing<T>, TraceFault> {
    let outward = crate::face_normal::surface_outward_normal(surface, point, sense).vec();
    let passage = match sign_as(
        TRACE_CROSSING_SENSE,
        Margin::levered(outward.dot(velocity) / outward.norm(), lever),
        band,
    )? {
        Sign::Negative => Passage::Enter,
        Sign::Positive => Passage::Leave,
        Sign::Zero => return Err(TraceFault::Grazing),
    };
    Ok(Crossing {
        along,
        point,
        passage,
        outward,
    })
}

/// The funnel row a plane's miss of a carrier is decided on: how far the
/// plane stands beyond the carrier, past its support on the plane's
/// nearer side, or off a plane or a cylinder it is parallel to.
pub const TRACE_PLANE_MISSES: &str = "trace_plane_misses";

/// **Whether the plane through `point` with unit `normal` is shown to
/// miss `surface`**: `true` grants that its trace is empty, and `false`
/// grants nothing (the crossings a caller computes next decide for
/// themselves). The plane is shown to miss a parallel plane off it, a
/// sphere or a torus it passes beyond (past the support of the carrier's
/// cover along the normal on the plane's side, [`Surface::cover`]), and a
/// cylinder whose axis it is parallel to and passes beyond. It is never
/// shown to miss a cone, whose locus is double, or a cylinder it tilts
/// across.
///
/// # Errors
///
/// [`TraceFault::Unsupported`] for a carrier with no closed form;
/// [`TraceFault::Unbounded`] for a sphere or a torus whose data bounds no
/// region; [`TraceFault::Escalated`] where the band cannot call the miss.
pub fn plane_misses<T: Decide>(
    surface: &Surface<T>,
    point: Point3<T>,
    normal: Vec3<T>,
    band: Band,
) -> Result<bool, TraceFault> {
    let unsupported = || TraceFault::Unsupported {
        carrier: surface.kind(),
    };
    let beyond =
        |gap: T| sign_as(TRACE_PLANE_MISSES, Margin::of(gap), band).map(|s| s == Sign::Positive);
    // How far the plane stands off a parallel carrier reaching `reach`
    // from `anchor` across it.
    let off = |anchor: Point3<T>, reach: T| beyond((anchor - point).dot(normal).abs() - reach);
    match *surface {
        Surface::Plane {
            origin, normal: n, ..
        } => {
            // A tilted plane meets it; a parallel one misses it off it.
            let tilt = n.cross(normal).norm();
            match sign_as(
                TRACE_PLANE_LEAD,
                // Unsourced lever: the plane carrier's stored origin's
                // distance from `point`, a carrier term and no extent.
                Margin::levered(tilt, geom_core::Lever::unsourced((origin - point).norm())),
                band,
            )? {
                Sign::Zero => off(origin, T::zero()),
                Sign::Positive | Sign::Negative => Ok(false),
            }
        }
        Surface::Sphere { .. } | Surface::Torus { .. } => {
            let cover = surface.cover().map_err(|refusal| {
                let (_, what) = refusal.unknowns()[0];
                TraceFault::Unbounded {
                    carrier: surface.kind(),
                    what,
                }
            })?;
            let level = (point - Point3::origin()).dot(normal);
            let (lo, hi) = cover.range_along(normal);
            let (above, below) = (level - hi, lo - level);
            beyond(above.max(below))
        }
        // A tilted cylinder runs across the plane; one parallel to it
        // misses it beyond its radius.
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => match sign_as(
            TRACE_PLANE_LEAD,
            Margin::levered(axis.dot(normal), geom_core::Lever::curvature_radius(radius)),
            band,
        )? {
            Sign::Zero => off(origin, radius),
            Sign::Positive | Sign::Negative => Ok(false),
        },
        Surface::Cone { .. } => Ok(false),
        Surface::Spindle { .. } | Surface::Nurbs(_) | Surface::Approx(_) => Err(unsupported()),
    }
}
