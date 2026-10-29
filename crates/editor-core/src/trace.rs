//! **A profile drawn on a frame, reading the traces of faces** (D10,
//! "Constructing a coincidence"; `[variables.kinds]`).
//!
//! A trace is a face's carrier cut by the frame's `xy` plane, read in
//! closed form from the carrier the evaluation holds at its lane scalar
//! (`topo::boolean::trace`). It is never a variable and never stored:
//! every resolution reads it again, so a moved or resized traced body
//! moves the profile with it.
//!
//! Three document forms read one, and each lowers to a step of the
//! profile's own vocabulary, so the replay and everything after it see
//! an ordinary program:
//!
//! - a straight piece running to [`ProgramTarget::OnTrace`] lowers to
//!   `line(len)`, its length the distance along its heading to the
//!   crossing; [`ProgramTarget::AtX`] and [`ProgramTarget::AtY`] the same
//!   to a written sketch coordinate;
//! - an arc about a centre running to [`ProgramTarget::OnTrace`] lowers
//!   to `arc_to(Center)`, its endpoint the crossing;
//! - [`ProgramStep::AlongTrace`] lowers to `toward(dx, dy)`, the trace's
//!   tangent at the crossing the previous piece reached.
//!
//! A piece's pen and heading are the replay's own: the prefix before
//! the step is replayed ([`profile::pen`]), guided by the first pass's
//! record on a lane's second pass. A prefix that refuses is left to the
//! program's own replay, which refuses at the same step.

use geom_core::{Band, Decide, Margin, Point2, Point3, RigidMotion, Sign, Tol, Vec2, Vec3};
use profile::{ArcSweep, Step};
use topo::boolean::trace::{self as kernel, Passage, TraceFault};

use crate::expr::VarEnv;
use crate::program::{
    ProgramArcData, ProgramStep, ProgramTarget, ProgramResolveFault, TraceCrossing, TracePassage,
    TraceSense, program_index,
};

/// The funnel row a trace's tangent at a crossing is decided on: the
/// in-plane part of the face's outward normal there, levered by the
/// piece's reach. Zero has no tangent: the plane tangent to the carrier
/// there, or a node of the trace.
pub const TRACE_TANGENT: &str = "trace_tangent";
/// The funnel row a piece reaching a written coordinate is decided on:
/// its heading's component across that coordinate's line, levered by
/// the distance to it, and then the distance along the heading.
pub const TRACE_REACHES: &str = "trace_reaches";

/// **One traced face, as the evaluation read it** at its lane scalar:
/// the carrier in its body's coordinates, the face's sense, and the
/// maps between the sketch's coordinates (the frame's) and the body's.
#[derive(Debug, Clone)]
pub struct TracedFace<T: geom_core::Real> {
    /// The face's carrier, in its body's coordinates, or the kind of a
    /// carrier with no closed form, which a step reading it refuses.
    pub surface: Result<geom::Surface<T>, geom::SurfaceKind>,
    /// The face's orientation sense.
    pub sense: bool,
    /// The sketch's coordinates into the body's.
    pub sketch_to_body: RigidMotion<T>,
    /// The body's coordinates into the sketch's.
    pub body_to_sketch: RigidMotion<T>,
}

/// **What a drawn profile's resolution reads**: its traced faces, by
/// place ([`crate::DrawnOn::faces`]), each loop's replay record where a
/// lane's second pass replays guided, and the run's tolerance.
#[derive(Debug, Clone, Copy)]
pub struct Traces<'a, T: geom_core::Real> {
    /// The traced faces.
    pub faces: &'a [TracedFace<T>],
    /// Each loop's record, on a guided pass.
    pub records: Option<&'a [profile::ReplayStructure]>,
    /// The run's tolerance, and its linear band.
    pub tol: Tol,
    /// The band every trace decision is read under.
    pub band: Band,
}

/// **Why a step reading a trace refused**, naming the step and, where
/// it reads one, the face.
#[derive(Debug, Clone, PartialEq)]
pub struct TraceRefusal {
    /// The loop.
    pub loop_: u32,
    /// The authored step.
    pub step: u32,
    /// The traced face, by place, where the step reads one.
    pub face: Option<u32>,
    /// Why.
    pub fault: TraceStepFault,
}

/// What [`TraceRefusal`] refuses.
#[derive(Debug, Clone, PartialEq)]
pub enum TraceStepFault {
    /// The profile is drawn on no frame, or was resolved with no trace
    /// to read (a preview with no evaluation).
    Undrawn,
    /// The step reads a face the profile does not trace.
    NoSuchFace {
        /// How many faces it traces.
        traced: u32,
    },
    /// The pen has no position here: the entry, or a direction bound
    /// first.
    NoPen,
    /// The piece has no heading to run along: a straight piece to a
    /// crossing runs along a director (`line_to`) or the incoming
    /// tangent (`continue_to`).
    NoHeading,
    /// The verb does not run along a heading or about a centre to a
    /// point it finds: a trace target in a tangent arc, an arc onto, a
    /// fillet's spec, or a coordinate target in an arc.
    NotARun,
    /// The step heads along a trace the previous piece did not run onto.
    OffTrace,
    /// The frame's plane misses the face's carrier: there is no trace.
    NoTrace,
    /// The piece makes fewer such crossings than the one written.
    NoSuchCrossing {
        /// The crossing written.
        crossing: TraceCrossing,
        /// How many crossings of that passage the piece makes.
        found: u32,
    },
    /// The trace has no tangent at the crossing: a node, a cone's apex,
    /// or the plane tangent to the carrier there.
    NoTangent,
    /// The heading never reaches the written coordinate, or reaches it
    /// behind the pen.
    Unreached,
    /// The carrier could not be read along the piece
    /// ([`TraceFault`]): no closed form, a grazing crossing, a sign in
    /// the sliver band.
    Carrier(TraceFault),
}

impl core::fmt::Display for TraceRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use crate::sentence::Recourse;
        write!(f, "loop {} step {}", self.loop_, self.step)?;
        if let Some(face) = self.face {
            write!(f, " (traced face {face})")?;
        }
        f.write_str(": ")?;
        let recourse = match &self.fault {
            TraceStepFault::Undrawn => {
                f.write_str("reads a trace, and the profile is drawn on no frame")?;
                "draw the profile on a frame of the traced face's space"
            }
            TraceStepFault::NoSuchFace { traced } => {
                write!(f, "reads a face the profile does not trace (it traces {traced})")?;
                "add the face to the faces the profile traces, or read one it traces"
            }
            TraceStepFault::NoPen => {
                f.write_str("the pen has no position here")?;
                "start the chain at a point before it runs to a trace"
            }
            TraceStepFault::NoHeading => {
                f.write_str("the piece has no heading to run along")?;
                "bind a heading before `line_to`, or `continue_to` off a piece's end"
            }
            TraceStepFault::NotARun => {
                f.write_str("this verb does not run along a heading to a point it finds")?;
                "run to it with `line_to`, `continue_to`, or an arc about a centre"
            }
            TraceStepFault::OffTrace => {
                f.write_str("heads along a trace the previous piece did not run onto")?;
                "run the previous piece onto the trace first"
            }
            TraceStepFault::NoTrace => {
                f.write_str("the frame's plane misses the face")?;
                "move the frame so its plane cuts the face"
            }
            TraceStepFault::NoSuchCrossing { crossing, found } => {
                write!(
                    f,
                    "the piece {} the face {found} time(s), not {}",
                    crossing.passage.word(),
                    crossing.nth
                )?;
                "name a crossing the piece makes, or turn its heading"
            }
            TraceStepFault::NoTangent => {
                f.write_str("the trace has no tangent at the crossing")?;
                "run onto the trace at another crossing, or move the frame"
            }
            TraceStepFault::Unreached => {
                f.write_str("the heading never reaches the coordinate")?;
                "turn the heading toward the coordinate"
            }
            TraceStepFault::Carrier(TraceFault::Unsupported { carrier }) => {
                return write!(
                    f,
                    "a {} carrier has no closed-form trace. There is no way through yet",
                    carrier.name()
                );
            }
            // The escalation states its own recourse.
            TraceStepFault::Carrier(fault @ TraceFault::Escalated(_)) => {
                return write!(f, "{fault}");
            }
            TraceStepFault::Carrier(fault) => {
                write!(f, "{fault}")?;
                "turn the piece so it crosses the face, or move the frame"
            }
        };
        write!(f, ". {}", Recourse(recourse))
    }
}

impl core::error::Error for TraceRefusal {}

/// Whether `t` is a target only a trace-reading resolution reads.
fn reads_a_trace<S>(t: &ProgramTarget<S>) -> bool {
    matches!(
        t,
        ProgramTarget::OnTrace { .. } | ProgramTarget::AtX(_) | ProgramTarget::AtY(_)
    )
}

/// **Whether `step` reads anything only a trace-reading resolution
/// reads**: a trace, or a written coordinate along a heading.
pub(crate) fn step_reads_a_trace<S>(step: &ProgramStep<S>) -> bool {
    match step {
        ProgramStep::AlongTrace { .. } => true,
        ProgramStep::LineTo(t)
        | ProgramStep::ContinueTo(t)
        | ProgramStep::TangentArcTo(t)
        | ProgramStep::ArcOnto(t) => reads_a_trace(t),
        ProgramStep::ArcTo(spec)
        | ProgramStep::FilletArc { spec, .. }
        | ProgramStep::ArcFillet { spec, .. } => spec_reads(spec),
        ProgramStep::ArcFilletArc { spec, spec2, .. } => spec_reads(spec) || spec_reads(spec2),
        ProgramStep::At(_)
        | ProgramStep::Angle(_)
        | ProgramStep::Toward { .. }
        | ProgramStep::Tangent
        | ProgramStep::Cusp
        | ProgramStep::Turn(_)
        | ProgramStep::Line(_)
        | ProgramStep::Fillet(_)
        | ProgramStep::FarEndTo(_)
        | ProgramStep::CloseTo => false,
    }
}

fn spec_reads<S>(spec: &ProgramArcData<S>) -> bool {
    match spec {
        ProgramArcData::Bulge { target, .. }
        | ProgramArcData::Via { target, .. }
        | ProgramArcData::Center { target, .. } => reads_a_trace(target),
        ProgramArcData::Radius { .. }
        | ProgramArcData::Sweep { .. }
        | ProgramArcData::ArcLen { .. } => false,
    }
}

/// **One chain loop resolved, reading its traces** at the lane scalar.
/// Each step that reads no trace resolves as every step does; each that
/// reads one lowers as the module docs say.
///
/// # Errors
///
/// [`ProgramResolveFault::Expr`] for an expression that does not resolve, and
/// [`ProgramResolveFault::Trace`] for a step whose trace refuses.
pub(crate) fn resolve_chain<T: profile::ArcCarrierScalar>(
    steps: &[ProgramStep],
    env: &VarEnv<T>,
    loop_: u32,
    traces: Option<&Traces<'_, T>>,
) -> Result<Vec<Step<T>>, ProgramResolveFault> {
    let mut out: Vec<Step<T>> = Vec::with_capacity(steps.len());
    // The crossing the previous piece ran onto: its face, and the point.
    let mut reached: Option<(u32, Point2<T>)> = None;
    for (i, s) in steps.iter().enumerate() {
        let step = program_index(i);
        let ran_onto = reached.take();
        if !step_reads_a_trace(s) {
            out.push(crate::program::res_chain_step(s, env, loop_, step)?);
            continue;
        }
        let refuse = |face: Option<u32>, fault| {
            ProgramResolveFault::Trace(TraceRefusal {
                loop_,
                step,
                face,
                fault,
            })
        };
        let Some(traces) = traces else {
            return Err(refuse(None, TraceStepFault::Undrawn));
        };
        let band = traces.band;
        let face = |at: u32| {
            traces.faces.get(at as usize).ok_or_else(|| {
                refuse(
                    Some(at),
                    TraceStepFault::NoSuchFace {
                        traced: u32::try_from(traces.faces.len()).unwrap_or(u32::MAX),
                    },
                )
            })
        };
        // The pen after the prefix; a prefix that refuses is the
        // replay's to report, at the step it refuses.
        let record = traces.records.and_then(|r| r.get(loop_ as usize));
        let pen = match profile::pen(&out, record, traces.tol) {
            Ok(pen) => pen,
            Err(_) => return Ok(out),
        };
        let Some(pen) = pen else {
            return Err(refuse(None, TraceStepFault::NoPen));
        };
        let res = |e: &crate::VarId, arg: crate::StepArg| {
            crate::expr::eval_var::<T>(*e, arg.dimension(), env).map_err(|source| {
                ProgramResolveFault::Expr {
                    slot: crate::SlotId::Profile { loop_, step, arg },
                    source,
                }
            })
        };
        let lowered = match s {
            ProgramStep::AlongTrace { sense } => {
                let Some((at, point)) = ran_onto else {
                    return Err(refuse(None, TraceStepFault::OffTrace));
                };
                let tangent = tangent(face(at)?, point, *sense, &pen, band)
                    .map_err(|fault| refuse(Some(at), fault))?;
                Step::Toward {
                    dx: tangent.x,
                    dy: tangent.y,
                }
            }
            ProgramStep::LineTo(target) | ProgramStep::ContinueTo(target) => {
                // A director binds `line_to`'s heading; `continue_to`
                // runs along the incoming tangent.
                let wants_director = matches!(s, ProgramStep::LineTo(_));
                let heading = match pen.heading {
                    Some(h) if pen.directed == wants_director => h,
                    _ => return Err(refuse(None, TraceStepFault::NoHeading)),
                };
                let len = match target {
                    ProgramTarget::OnTrace { face: at, crossing } => {
                        let (len, point) =
                            ray_to(face(*at)?, pen.at, heading, *crossing, band)
                                .map_err(|fault| refuse(Some(*at), fault))?;
                        reached = Some((*at, point));
                        len
                    }
                    ProgramTarget::AtX(x) => {
                        let x = res(x, crate::StepArg::TargetX)?;
                        reach(x - pen.at.x, heading.x, band).map_err(|f| refuse(None, f))?
                    }
                    ProgramTarget::AtY(y) => {
                        let y = res(y, crate::StepArg::TargetY)?;
                        reach(y - pen.at.y, heading.y, band).map_err(|f| refuse(None, f))?
                    }
                    ProgramTarget::Point(_)
                    | ProgramTarget::Start
                    | ProgramTarget::StartArriving => {
                        unreachable!("a step reading a trace reads one of the three forms")
                    }
                };
                Step::Line(len)
            }
            ProgramStep::ArcTo(ProgramArcData::Center {
                c,
                winding,
                target: ProgramTarget::OnTrace { face: at, crossing },
            }) => {
                let centre = Point2::new(
                    res(&c[0], crate::StepArg::CenterX)?,
                    res(&c[1], crate::StepArg::CenterY)?,
                );
                let point = arc_to(face(*at)?, pen.at, centre, *winding, *crossing, band)
                    .map_err(|fault| refuse(Some(*at), fault))?;
                reached = Some((*at, point));
                Step::ArcTo(profile::ArcData::Center {
                    c: centre,
                    winding: *winding,
                    target: profile::Target::Point(point),
                })
            }
            _ => return Err(refuse(None, TraceStepFault::NotARun)),
        };
        out.push(lowered);
    }
    Ok(out)
}

fn lift<T: Decide>(p: Point2<T>) -> Point3<T> {
    Point3::new(p.x, p.y, T::zero())
}

fn lift_vec<T: Decide>(v: Vec2<T>) -> Vec3<T> {
    Vec3::new(v.x, v.y, T::zero())
}

/// The frame plane's point and normal, in the body's coordinates.
fn plane_in_body<T: Decide>(face: &TracedFace<T>) -> (Point3<T>, Vec3<T>) {
    (
        face.sketch_to_body.transform_point(Point3::origin()),
        face.sketch_to_body.transform_vec(Vec3::new(T::zero(), T::zero(), T::one())),
    )
}

/// A length the size of what a piece from `from` runs through: the lever
/// its decisions about `face` are read over.
fn reach_of<T: Decide>(face: &TracedFace<T>, surface: &geom::Surface<T>, from: Point3<T>) -> T {
    use geom::Surface as S;
    let (anchor, size) = match *surface {
        S::Plane { origin, .. } => (origin, T::zero()),
        S::Cylinder { origin, radius, .. } => (origin, radius),
        S::Cone { apex, .. } => (apex, T::zero()),
        S::Sphere { center, radius, .. } => (center, radius),
        S::Torus {
            center,
            major_radius,
            minor_radius,
            ..
        } => (center, major_radius + minor_radius),
        S::Nurbs(_) | S::Approx(_) => (from, T::zero()),
    };
    (anchor - from).norm() + size + (from - plane_in_body(face).0).norm()
}

/// The face's carrier, where it has a closed form.
fn carrier<T: geom_core::Real>(face: &TracedFace<T>) -> Result<&geom::Surface<T>, TraceStepFault> {
    face.surface
        .as_ref()
        .map_err(|&carrier| TraceStepFault::Carrier(TraceFault::Unsupported { carrier }))
}

/// The face's carrier, refusing where the frame's plane misses it.
fn traced<T: Decide>(face: &TracedFace<T>, band: Band) -> Result<&geom::Surface<T>, TraceStepFault> {
    let surface = carrier(face)?;
    let (point, normal) = plane_in_body(face);
    match kernel::plane_misses(surface, point, normal, band) {
        Ok(true) => Err(TraceStepFault::NoTrace),
        Ok(false) => Ok(surface),
        Err(fault) => Err(TraceStepFault::Carrier(fault)),
    }
}

/// The written crossing among `crossings`, or how many there were.
fn pick<T: geom_core::Real>(
    crossings: Vec<kernel::Crossing<T>>,
    crossing: TraceCrossing,
) -> Result<kernel::Crossing<T>, TraceStepFault> {
    let passage = match crossing.passage {
        TracePassage::Enter => Passage::Enter,
        TracePassage::Leave => Passage::Leave,
    };
    let of: Vec<_> = crossings
        .into_iter()
        .filter(|c| c.passage == passage)
        .collect();
    let found = u32::try_from(of.len()).unwrap_or(u32::MAX);
    crossing
        .nth
        .checked_sub(1)
        .and_then(|k| of.get(k as usize).copied())
        .ok_or(TraceStepFault::NoSuchCrossing { crossing, found })
}

/// The length along `heading` from `pen` to the written crossing, and
/// the crossing, in sketch coordinates.
fn ray_to<T: Decide>(
    face: &TracedFace<T>,
    pen: Point2<T>,
    heading: Vec2<T>,
    crossing: TraceCrossing,
    band: Band,
) -> Result<(T, Point2<T>), TraceStepFault> {
    let surface = traced(face, band)?;
    let origin = face.sketch_to_body.transform_point(lift(pen));
    let dir = face.sketch_to_body.transform_vec(lift_vec(heading));
    let crossings = kernel::ray_crossings(
        surface,
        face.sense,
        origin,
        dir,
        reach_of(face, surface, origin),
        band,
    )
    .map_err(TraceStepFault::Carrier)?;
    let hit = pick(crossings, crossing)?;
    let at = face.body_to_sketch.transform_point(hit.point);
    Ok((hit.along, Point2::new(at.x, at.y)))
}

/// The written crossing of the arc about `centre` from `pen`, in sketch
/// coordinates.
fn arc_to<T: Decide>(
    face: &TracedFace<T>,
    pen: Point2<T>,
    centre: Point2<T>,
    winding: ArcSweep,
    crossing: TraceCrossing,
    band: Band,
) -> Result<Point2<T>, TraceStepFault> {
    let surface = traced(face, band)?;
    let radial = pen - centre;
    let radius = radial.norm();
    let up = match winding {
        ArcSweep::Ccw => T::one(),
        ArcSweep::Cw => T::zero() - T::one(),
    };
    let map = &face.sketch_to_body;
    let circle = (
        map.transform_point(lift(centre)),
        map.transform_vec(Vec3::new(T::zero(), T::zero(), up)),
        radius,
        map.transform_vec(lift_vec(radial / radius)),
    );
    let crossings = kernel::circle_crossings(surface, face.sense, circle, band)
        .map_err(TraceStepFault::Carrier)?;
    let hit = pick(crossings, crossing)?;
    let at = face.body_to_sketch.transform_point(hit.point);
    Ok(Point2::new(at.x, at.y))
}

/// The trace's unit tangent at `point`, in `sense`: the face's outward
/// normal there, in the sketch's plane, turned a quarter turn the
/// sketch's way (material on the left), or against it.
fn tangent<T: Decide>(
    face: &TracedFace<T>,
    point: Point2<T>,
    sense: TraceSense,
    pen: &profile::Pen<T>,
    band: Band,
) -> Result<Vec2<T>, TraceStepFault> {
    let surface = carrier(face)?;
    let at = face.sketch_to_body.transform_point(lift(point));
    let outward = geom_brep::implicit_outward_normal(surface, face.sense, at).vec();
    let normal = face.body_to_sketch.transform_vec(outward);
    let in_plane = Vec2::new(normal.x, normal.y);
    let norm = in_plane.norm();
    let lever = reach_of(face, surface, at) + (pen.at - point).norm();
    match geom_core::k_stats::decide(TRACE_TANGENT, Margin::levered(norm, lever), band) {
        Ok(Sign::Positive) => {}
        Ok(Sign::Zero | Sign::Negative) => return Err(TraceStepFault::NoTangent),
        Err(diag) => return Err(TraceStepFault::Carrier(TraceFault::Escalated(diag))),
    }
    let with = Vec2::new(T::zero() - in_plane.y, in_plane.x) / norm;
    Ok(match sense {
        TraceSense::With => with,
        TraceSense::Against => Vec2::new(T::zero() - with.x, T::zero() - with.y),
    })
}

/// The length along a heading whose component across a coordinate's
/// line is `rate` to the point `gap` along that coordinate.
fn reach<T: Decide>(gap: T, rate: T, band: Band) -> Result<T, TraceStepFault> {
    let escalated = |diag| TraceStepFault::Carrier(TraceFault::Escalated(diag));
    match geom_core::k_stats::decide(TRACE_REACHES, Margin::levered(rate, gap.abs()), band) {
        Ok(Sign::Positive | Sign::Negative) => {}
        Ok(Sign::Zero) => return Err(TraceStepFault::Unreached),
        Err(diag) => return Err(escalated(diag)),
    }
    let len = gap / rate;
    match geom_core::k_stats::decide(TRACE_REACHES, Margin::of(len), band) {
        Ok(Sign::Positive) => Ok(len),
        Ok(Sign::Zero | Sign::Negative) => Err(TraceStepFault::Unreached),
        Err(diag) => Err(escalated(diag)),
    }
}
