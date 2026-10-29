//! **A face's carrier read along a sketch piece** (D10 `[coincidence]`,
//! "Constructing a coincidence"): where a line or a circle in a sketch's
//! plane crosses a face's carrier, in which sense, and the carrier's
//! outward normal there.
//!
//! A trace is the carrier cut by the sketch's plane, in closed form; a
//! piece of the sketch lies in that plane, so where the piece meets the
//! trace is where it meets the carrier. This door reads the piece
//! against the carrier itself, through the boolean's certified root
//! doors (a line against a wall, a sphere, a cone or a torus; a circle
//! against a sphere, a wall, a cone or a torus), and a plane in closed
//! form. It reads no face boundary: the trace is the carrier's.
//!
//! The sense of a crossing is the face's: a piece ENTERS the face's
//! material where it runs against the face's outward normal, and LEAVES
//! where it runs along it. A crossing whose sense the band cannot call,
//! a tangency, or two crossings the band cannot order graze the carrier,
//! and refuse.

use geom::{Curve3, Surface, SurfaceKind};
use geom_brep::roots::{Roots, Unsettled};
use geom_core::{Band, Decide, Indeterminate, InfSpeed, Margin, Point3, Real, Sign, Vec3};

use super::BooleanError;
use super::circle_roots::CircleRoots;
use crate::validate::decide;

/// The funnel row a crossing's sense is decided on: the cosine between
/// the piece's direction and the outward normal, levered by the piece's
/// reach.
pub const TRACE_CROSSING_SENSE: &str = "trace_crossing_sense";
/// The funnel row a crossing's place ahead of the pen is decided on: its
/// distance along the piece.
pub const TRACE_CROSSING_AHEAD: &str = "trace_crossing_ahead";
/// The funnel row two crossings are ordered on: their distance apart
/// along the piece.
pub const TRACE_CROSSING_ORDER: &str = "trace_crossing_order";
/// The funnel row a plane carrier's lead is decided on: how far the
/// piece turns off parallel to the plane over its reach.
pub const TRACE_PLANE_LEAD: &str = "trace_plane_lead";
/// The funnel row a circle's depth past a plane carrier is decided on.
pub const TRACE_PLANE_DEPTH: &str = "trace_plane_depth";
/// The funnel row a piece lying on the carrier is decided on: the
/// carrier's residual at the pen.
pub const TRACE_ON_CARRIER: &str = "trace_on_carrier";

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
    /// The carrier's outward normal there, unit.
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
    /// The piece grazes the carrier: a tangency, a crossing at the
    /// apex of a cone, or a double root.
    Grazing,
    /// The piece lies on the carrier.
    OnCarrier,
    /// A sign the band cannot call: the sliver band.
    Escalated(Indeterminate),
    /// A root count the door cannot certify.
    Unsettled(Unsettled),
}

impl core::fmt::Display for TraceFault {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unsupported { carrier } => {
                write!(f, "a {} carrier has no closed-form trace", carrier.name())
            }
            Self::Grazing => f.write_str("the piece grazes the carrier"),
            Self::OnCarrier => f.write_str("the piece lies on the carrier"),
            Self::Escalated(diag) => write!(f, "a crossing in the sliver band ({diag})"),
            Self::Unsettled(why) => write!(f, "a root count the door cannot certify ({why:?})"),
        }
    }
}

impl core::error::Error for TraceFault {}

fn door(error: BooleanError) -> TraceFault {
    match error {
        BooleanError::Escalated { diag, .. } => TraceFault::Escalated(diag),
        other => unreachable!("the trace dispatches each root door on its own kinds: {other:?}"),
    }
}

fn unsettled(why: Unsettled) -> TraceFault {
    match why {
        Unsettled::Tangency | Unsettled::AtApex => TraceFault::Grazing,
        other => TraceFault::Unsettled(other),
    }
}

fn sign(name: &'static str, margin: Margin<impl Decide>, band: Band) -> Result<Sign, TraceFault> {
    decide(name, margin, band).map_err(TraceFault::Escalated)
}

/// **The crossings of the ray `origin + dir·t`, `t > 0`, with
/// `surface`**, nearest first. `dir` is unit; `reach` is a length the
/// size of the scene the piece runs through, the lever its lead and
/// its senses are read over. `sense` is the face's orientation sense.
///
/// # Errors
///
/// [`TraceFault`].
pub fn ray_crossings<T: Decide>(
    surface: &Surface<T>,
    sense: bool,
    origin: Point3<T>,
    dir: Vec3<T>,
    reach: T,
    band: Band,
) -> Result<Vec<Crossing<T>>, TraceFault> {
    use super::reduce::SpanVerdict;
    let on_carrier = || -> Result<Vec<Crossing<T>>, TraceFault> {
        let residual = geom_brep::implicit_residual(surface, origin);
        match sign(TRACE_ON_CARRIER, Margin::of(residual), band)? {
            Sign::Zero => Err(TraceFault::OnCarrier),
            Sign::Positive | Sign::Negative => Ok(Vec::new()),
        }
    };
    let roots: Vec<T> = match *surface {
        Surface::Plane { origin: o, normal, .. } => {
            let lead = normal.dot(dir);
            match sign(TRACE_PLANE_LEAD, Margin::levered(lead, reach), band)? {
                Sign::Zero => return on_carrier(),
                Sign::Positive | Sign::Negative => {}
            }
            vec![(o - origin).dot(normal) / lead]
        }
        Surface::Nurbs(_) | Surface::Approx(_) => {
            return Err(TraceFault::Unsupported {
                carrier: surface.kind(),
            });
        }
        _ => match super::reduce::line_wall_roots_of(
            origin,
            dir,
            (T::zero(), reach),
            surface,
            band,
        )
        .map_err(door)?
        {
            None => {
                return Err(TraceFault::Unsupported {
                    carrier: surface.kind(),
                });
            }
            Some(Err(SpanVerdict::Constant)) => return on_carrier(),
            Some(Err(_)) => unreachable!("a line's root door answers a constant residual alone"),
            Some(Ok(found)) => roots_of(found)?,
        },
    };
    let mut out = Vec::new();
    for t in roots {
        match sign(TRACE_CROSSING_AHEAD, Margin::of(t), band)? {
            Sign::Positive => {}
            Sign::Zero | Sign::Negative => continue,
        }
        let point = origin + dir * t;
        out.push(crossing(surface, sense, t, point, dir, reach, band)?);
    }
    order(out, |a, b| Margin::of(b - a), band)
}

/// **The crossings of the circle about `center` in the plane normal to
/// `axis`, of `radius`, starting at `center + radius·u_ref` and turning
/// right-handed about `axis`, with `surface`**: each strictly past the
/// start and at most a whole turn on, nearest first. `axis` and `u_ref`
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
    let tau = T::tau();
    let points: Vec<Point3<T>> = match *surface {
        Surface::Plane { origin, normal, .. } => {
            // `A cos θ + B sin θ + K = 0`, solved for the cosine and sine.
            let a = normal.dot(u_ref) * radius;
            let b = normal.dot(v_ref) * radius;
            let k = (center - origin).dot(normal);
            let r2 = a.powi(2) + b.powi(2);
            let r = r2.sqrt();
            match sign(TRACE_PLANE_LEAD, Margin::of(r), band)? {
                Sign::Zero => {
                    return match sign(TRACE_ON_CARRIER, Margin::of(k), band)? {
                        Sign::Zero => Err(TraceFault::OnCarrier),
                        Sign::Positive | Sign::Negative => Ok(Vec::new()),
                    };
                }
                Sign::Positive | Sign::Negative => {}
            }
            match sign(TRACE_PLANE_DEPTH, Margin::of(r - k.abs()), band)? {
                Sign::Zero => return Err(TraceFault::Grazing),
                Sign::Negative => return Ok(Vec::new()),
                Sign::Positive => {}
            }
            let half = (r2 - k.powi(2)).sqrt();
            [T::one(), T::zero() - T::one()]
                .into_iter()
                .map(|s| {
                    let c = (T::zero() - k * a + s * half * b) / r2;
                    let sn = (T::zero() - k * b - s * half * a) / r2;
                    center + u_ref * (radius * c) + v_ref * (radius * sn)
                })
                .collect()
        }
        Surface::Nurbs(_) | Surface::Approx(_) => {
            return Err(TraceFault::Unsupported {
                carrier: surface.kind(),
            });
        }
        _ => {
            let circle = Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            };
            let found = match surface {
                Surface::Torus { .. } => super::circle_torus::circle_torus_roots(
                    &circle,
                    T::zero(),
                    tau,
                    surface,
                    band,
                ),
                _ => super::conic_quadric::conic_quadric_roots(
                    &circle,
                    T::zero(),
                    tau,
                    surface,
                    band,
                ),
            }
            .map_err(door)?;
            roots_of(found)?
                .into_iter()
                .map(|theta| {
                    let (s, c) = theta.sin_cos();
                    center + u_ref * (radius * c) + v_ref * (radius * s)
                })
                .collect()
        }
    };
    let rate = InfSpeed::new(radius);
    let mut out = Vec::new();
    for point in points {
        let w = point - center;
        let (c, s) = (w.dot(u_ref) / radius, w.dot(v_ref) / radius);
        // The turn from the start, in `(0, 2π]`: a sine at or below zero
        // is the far half.
        let half_turn = c.max(T::zero() - T::one()).min(T::one()).acos();
        let theta = s.select_le_zero(tau - half_turn, half_turn);
        // A crossing a whole turn on is the pen itself.
        match sign(TRACE_CROSSING_AHEAD, Margin::metered(tau - theta, rate), band)? {
            Sign::Positive => {}
            Sign::Zero | Sign::Negative => continue,
        }
        match sign(TRACE_CROSSING_AHEAD, Margin::metered(theta, rate), band)? {
            Sign::Positive => {}
            Sign::Zero | Sign::Negative => continue,
        }
        let velocity = v_ref * c - u_ref * s;
        out.push(crossing(surface, sense, theta, point, velocity, radius, band)?);
    }
    order(out, |a, b| Margin::metered(b - a, InfSpeed::new(radius)), band)
}

/// The roots a certified answer holds; one on the carrier refuses.
fn roots_of<T: Real>(found: Result<CircleRoots<T>, Unsettled>) -> Result<Vec<T>, TraceFault> {
    match found.map_err(unsettled)? {
        Roots::Miss => Ok(Vec::new()),
        Roots::On(_) => Err(TraceFault::OnCarrier),
        Roots::Certified(found) => Ok(found.roots().to_vec()),
    }
}

/// One crossing at `point`, `along` the piece, its sense read off the
/// piece's unit `velocity` against the outward normal.
fn crossing<T: Decide>(
    surface: &Surface<T>,
    sense: bool,
    along: T,
    point: Point3<T>,
    velocity: Vec3<T>,
    lever: T,
    band: Band,
) -> Result<Crossing<T>, TraceFault> {
    let outward = geom_brep::implicit_outward_normal(surface, sense, point).vec();
    let passage = match sign(
        TRACE_CROSSING_SENSE,
        Margin::levered(outward.dot(velocity), lever),
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

/// The crossings nearest first, each pair ordered under
/// [`TRACE_CROSSING_ORDER`] on `gap(a, b)`, the length from `a` on to
/// `b`; two the band cannot part graze.
fn order<T: Decide>(
    mut crossings: Vec<Crossing<T>>,
    gap: impl Fn(T, T) -> Margin<T>,
    band: Band,
) -> Result<Vec<Crossing<T>>, TraceFault> {
    for i in 1..crossings.len() {
        let mut j = i;
        while j > 0 {
            match sign(
                TRACE_CROSSING_ORDER,
                gap(crossings[j - 1].along, crossings[j].along),
                band,
            )? {
                Sign::Positive => break,
                Sign::Zero => return Err(TraceFault::Grazing),
                Sign::Negative => crossings.swap(j - 1, j),
            }
            j -= 1;
        }
    }
    Ok(crossings)
}

/// The funnel row a plane's miss of a carrier is decided on: the
/// plane's distance from the carrier's anchor less the carrier's reach
/// across it.
pub const TRACE_PLANE_MISSES: &str = "trace_plane_misses";

/// **Whether the plane through `point` with unit `normal` misses
/// `surface`**: its trace is empty. A plane misses a parallel plane off
/// it, a sphere or a torus it passes beyond, and a cylinder whose axis
/// it is parallel to and passes beyond; it meets every cone, whose
/// locus is double, and every cylinder it tilts across.
///
/// # Errors
///
/// [`TraceFault::Unsupported`] for a carrier with no closed form;
/// [`TraceFault::Escalated`] where the band cannot call the miss.
pub fn plane_misses<T: Decide>(
    surface: &Surface<T>,
    point: Point3<T>,
    normal: Vec3<T>,
    band: Band,
) -> Result<bool, TraceFault> {
    // The plane's distance from `anchor`, less how far the carrier
    // reaches from it along the normal: positive is a miss.
    let beyond = |anchor: Point3<T>, extent: T| {
        let gap = (anchor - point).dot(normal).abs() - extent;
        sign(TRACE_PLANE_MISSES, Margin::of(gap), band).map(|s| s == Sign::Positive)
    };
    match *surface {
        Surface::Plane {
            origin,
            normal: n,
            ..
        } => {
            // A tilted plane meets it; a parallel one misses it off it.
            let tilt = n.cross(normal).norm();
            match sign(TRACE_PLANE_LEAD, Margin::levered(tilt, (origin - point).norm()), band)? {
                Sign::Zero => beyond(origin, T::zero()),
                Sign::Positive | Sign::Negative => Ok(false),
            }
        }
        Surface::Sphere { center, radius, .. } => beyond(center, radius),
        Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => match sign(
            TRACE_PLANE_LEAD,
            Margin::levered(axis.dot(normal), radius),
            band,
        )? {
            Sign::Zero => beyond(origin, radius),
            Sign::Positive | Sign::Negative => Ok(false),
        },
        Surface::Cone { .. } => Ok(false),
        Surface::Torus {
            center,
            axis,
            major_radius,
            minor_radius,
            ..
        } => beyond(
            center,
            major_radius * axis.cross(normal).norm() + minor_radius,
        ),
        Surface::Nurbs(_) | Surface::Approx(_) => Err(TraceFault::Unsupported {
            carrier: surface.kind(),
        }),
    }
}
