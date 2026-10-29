//! **A chart singularity in a face is a vertex of it**: the pass that
//! makes it one ([`crate::NormalizationKind::ChartSingularityVertex`]
//! states the rule and what each disposition mints).
//!
//! The trigger is incidence, read off the face's region: a sphere's
//! pole or a cone's apex that lies in the face's closure and is not a
//! vertex of it with an edge on each side. Where it lies decides the
//! disposition ([`SingularityFound`]):
//!
//! - **Interior**: a vertex is minted there with the pole form's two
//!   meridians ([`geom::pole_form`]), each running to the first
//!   boundary edge it meets, and the face is cut in two along them.
//! - **Inside an edge**: the edge is split there.
//! - **A valence-1 vertex** (Open CASCADE's seam meridian ending at
//!   the pole or apex): the pole form is completed from the seam with
//!   its half-turn twin, which runs to the first boundary edge it
//!   meets, and the face is cut in two along the pair.
//!
//! Each split of a boundary edge propagates to every face sharing it,
//! so the shell stays closed. A face holding both poles of a sphere is
//! cut at the azimuth of its loop's centroid, through the hole the loop
//! bounds, so the pole met second is reached on a meridian of the first
//! and completed as a valence-1 vertex. Where a meridian lands is
//! decided as the point itself is: at a boundary vertex within the
//! band's zero, splitting the edge past its escalation, and refused
//! between.
//!
//! **Where it lies** is decided at the file's band ([`crate::chart`]):
//! within ε_in of a vertex or an edge it is on it, past the band's
//! escalation distance it is off, and between the two it refuses. Off
//! the boundary, the face's side of each loop is its sense bit, and
//! which side of a loop holds the point is the loop's winding about
//! the surface axis read through the kernel's own chart inverse: a loop
//! that winds separates the sphere's poles (or rings the cone's apex),
//! and one that does not leaves them on the side its chart image does
//! not enclose.

use geom::{Curve3, Surface};
use geom_core::{Point2, Point3, Vec3};

use super::{expand_split_uses, face_boundary_census, split_at_param, use_start, valences};
use crate::entities::{EdgeSpec, EdgeUse, FaceSpec, LoopSpec, SolidSpec};
use crate::error::StepImportError;
use crate::{FaceCensus, NormalizationKind, SingularityFound, StructureNormalization};

/// The surface data the pass reads: the axis through the singular
/// points, the reference azimuth, and the points themselves.
#[derive(Clone, Copy)]
enum Chart {
    /// A sphere: two poles, `[north, south]`.
    Sphere {
        center: Point3<f64>,
        radius: f64,
        axis: Vec3<f64>,
        u_ref: Vec3<f64>,
    },
    /// A cone: the apex.
    Cone {
        apex: Point3<f64>,
        axis: Vec3<f64>,
        half_angle: f64,
        u_ref: Vec3<f64>,
    },
}

impl Chart {
    fn of(surface: &Surface<f64>) -> Option<Self> {
        match *surface {
            Surface::Sphere {
                center,
                radius,
                axis,
                u_ref,
            } => Some(Self::Sphere {
                center,
                radius,
                axis,
                u_ref,
            }),
            Surface::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            } => Some(Self::Cone {
                apex,
                axis,
                half_angle,
                u_ref,
            }),
            _ => None,
        }
    }

    fn axis(self) -> Vec3<f64> {
        match self {
            Self::Sphere { axis, .. } | Self::Cone { axis, .. } => axis,
        }
    }

    fn u_ref(self) -> Vec3<f64> {
        match self {
            Self::Sphere { u_ref, .. } | Self::Cone { u_ref, .. } => u_ref,
        }
    }

    /// A point on the axis.
    fn origin(self) -> Point3<f64> {
        match self {
            Self::Sphere { center, .. } => center,
            Self::Cone { apex, .. } => apex,
        }
    }

    /// The singular points, exact from the surface's own fields.
    fn singular(self) -> Vec<Point3<f64>> {
        match self {
            Self::Sphere {
                center,
                radius,
                axis,
                ..
            } => vec![
                crate::signed_zero::plus_zero_point(center + axis * radius),
                crate::signed_zero::plus_zero_point(center - axis * radius),
            ],
            Self::Cone { apex, .. } => vec![apex],
        }
    }
}

/// What the scan found at one face: the disposition to apply.
enum Action {
    /// The singular point `k` lies inside the face.
    Interior { fi: usize, k: usize },
    /// The singular point lies inside `edge`, at carrier parameter `t`.
    InsideEdge {
        fi: usize,
        edge: u64,
        t: f64,
        point: Point3<f64>,
    },
    /// The singular point is `vertex`, whose one edge the face uses
    /// both ways.
    StrutTip { fi: usize, vertex: u64 },
}

/// The pass (module docs).
pub(super) fn singularity_vertices(
    solid: &mut SolidSpec,
    eps_in: f64,
    chart_band: Option<geom_core::Band>,
    run_band: Option<geom_core::Band>,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    // The coarser of the file's band and the run's: a point the kernel
    // reads as one with a vertex is not cut a sub-tolerance edge away
    // from it, and one the file cannot place is not placed finer.
    let zero = |b: Option<geom_core::Band>| b.map_or(0.0, geom_core::Band::zero);
    let escalate = |b: Option<geom_core::Band>| b.map_or(0.0, geom_core::Band::escalate);
    let (eps, escalate) = (
        eps_in.max(zero(chart_band)).max(zero(run_band)),
        eps_in.max(escalate(chart_band)).max(escalate(run_band)),
    );
    let read = Read {
        eps,
        escalate,
        band: chart_band,
        run: run_band,
        file: geom_core::FileCoincidence::new(eps_in),
    };
    // Each action makes one singular point a vertex with an edge on
    // each side, which no later scan re-finds, so the pass ends; the
    // bound only turns a defect of that argument into a refusal.
    let bound = 4 * solid.faces.len() + 8;
    for _ in 0..bound {
        let mut found = None;
        for fi in 0..solid.faces.len() {
            if let Some(action) = scan(solid, fi, &read)? {
                found = Some(action);
                break;
            }
        }
        let Some(action) = found else {
            return Ok(());
        };
        apply(solid, action, &read, mint, sink)?;
    }
    Err(StepImportError::Topology {
        id: solid.id,
        what: "internal: the chart-singularity pass did not settle",
    })
}

/// The file's band, as the pass reads it.
#[derive(Clone, Copy)]
struct Read {
    /// ε_in: on a vertex or an edge within this distance.
    eps: f64,
    /// Off it past this distance; between the two, undecided.
    escalate: f64,
    /// The band the chart is read at.
    band: Option<geom_core::Band>,
    /// The run's own linear band, the on-or-off decision's other band.
    run: Option<geom_core::Band>,
    /// The file's ε_in, the door a refusal ends at.
    file: geom_core::FileCoincidence,
}

/// Whether the singular point lies on the boundary feature `d` metres
/// from it, at the coarser of the run's band and the file's
/// (`chart_band`, `(ε_in, K·ε_in)`): on it where either reads zero, off
/// it where both read a clear distance, and otherwise the typed refusal
/// with the margin of the band that could not call it
/// ([`StepImportError::SingularityUndecided`]). The run's band is asked
/// first, so a distance the kernel reads as zero is never refused for
/// the file's.
fn on_boundary(d: f64, read: &Read, id: u64) -> Result<bool, StepImportError> {
    use crate::error::SingularityBand;
    use geom_core::{Margin, Sign, k_stats::decide};
    const NAME: &str = "import_singularity_on_boundary";
    let refuse = |cause, band| StepImportError::SingularityUndecided {
        id,
        cause,
        band,
        file: read.file,
    };
    let run = read.run.map(|band| decide(NAME, Margin::of(d), band));
    if let Some(Ok(Sign::Zero)) = run {
        return Ok(true);
    }
    match read.band.map(|band| decide(NAME, Margin::of(d), band)) {
        Some(Ok(Sign::Zero)) => return Ok(true),
        Some(Err(cause)) => return Err(refuse(cause, SingularityBand::File)),
        Some(Ok(_)) => {}
        None if d <= read.eps => return Ok(true),
        None => {}
    }
    match run {
        Some(Err(cause)) => Err(refuse(cause, SingularityBand::Run)),
        _ => Ok(false),
    }
}

fn unreadable(id: u64) -> StepImportError {
    StepImportError::Topology {
        id,
        what: "a sphere or cone face whose boundary's winding about the surface axis its \
               own geometry cannot state through the chart inverse, so whether the pole or \
               apex lies inside it is a guess, and D7 forbids guessing it",
    }
}

fn foreign_carrier(id: u64) -> StepImportError {
    StepImportError::Topology {
        id,
        what: "a sphere or cone face holding its pole or apex, bounded by an edge on a \
               carrier the chart-singularity normalization reads no crossing of (it reads \
               lines and circles): the meridians it would mint cannot be ended on that \
               boundary; extending its crossing to the carrier is the recourse",
    }
}

fn inverted(id: u64) -> StepImportError {
    StepImportError::Topology {
        id,
        what: "an ORIENTATION-INVERTED face ending a seam at its pole or apex: its stated \
               same_sense puts its material on the side of its loop away from the seam it \
               uses both ways, so the solid it describes is inside out. The re-mint will \
               not re-tessellate it right-side-out, which would launder the inversion, and \
               import returns certified bodies, so the refusal fires here, before any body \
               exists",
    }
}

/// The disposition the face `fi` owes, if any (module docs).
fn scan(solid: &SolidSpec, fi: usize, read: &Read) -> Result<Option<Action>, StepImportError> {
    let face = &solid.faces[fi];
    let Some(chart) = Chart::of(&face.surface) else {
        return Ok(None);
    };
    let valence = valences(solid);
    for (k, x) in chart.singular().into_iter().enumerate() {
        // On a vertex of the face?
        let mut nearest: Option<(u64, f64)> = None;
        for lp in &face.loops {
            for u in &lp.uses {
                let v = use_start(solid, *u);
                let d = (solid.vertices[&v] - x).norm();
                if nearest.is_none_or(|(_, best)| d < best) {
                    nearest = Some((v, d));
                }
            }
        }
        if let Some((v, d)) = nearest
            && on_boundary(d, read, face.id)?
        {
            if valence.get(&v) == Some(&1) && strut_uses(solid, fi, v).is_some() {
                return Ok(Some(Action::StrutTip { fi, vertex: v }));
            }
            continue;
        }
        // Inside an edge of the face?
        for lp in &face.loops {
            for u in &lp.uses {
                let spec = &solid.edges[&u.edge];
                match nearest_on_edge(spec, x, read)? {
                    Near::Off => {}
                    Near::On { t, d } => {
                        if on_boundary(d, read, face.id)? {
                            return Ok(Some(Action::InsideEdge {
                                fi,
                                edge: u.edge,
                                t,
                                point: x,
                            }));
                        }
                    }
                    Near::Unread => return Err(foreign_carrier(face.id)),
                }
            }
        }
        // Inside the face?
        if holds(solid, fi, chart, k, read)? {
            return Ok(Some(Action::Interior { fi, k }));
        }
    }
    Ok(None)
}

/// Where a point lies against one edge.
enum Near {
    /// Farther than the band's escalation distance.
    Off,
    /// Within the band's escalation distance of the edge's closest
    /// interior point `t`, `d` away: on or off is the decision's.
    On { t: f64, d: f64 },
    /// A carrier this pass reads no distance to, near enough to matter.
    Unread,
}

/// **The parameter at which `carrier` passes nearest `x`**, the one
/// reading of it this pass takes: a line's projection, or a circle's
/// angle about its centre in its own frame (the convention
/// [`crate::geometry::endpoint_params`] derives edge intervals in),
/// brought to the representative at or past `from` and short of a turn
/// beyond it. `None` for any other carrier.
fn param_at(carrier: &Curve3<f64>, x: Point3<f64>, from: f64) -> Option<f64> {
    match *carrier {
        Curve3::Line { origin, dir } => Some((x - origin).dot(dir) / dir.dot(dir)),
        Curve3::Circle {
            center,
            axis,
            u_ref,
            ..
        } => {
            let w = x - center;
            let t = w.dot(axis.cross(u_ref)).atan2(w.dot(u_ref));
            let tau = core::f64::consts::TAU;
            Some(from + (t - from).rem_euclid(tau))
        }
        _ => None,
    }
}

/// The distance from `x` to the interior of the edge `spec`.
fn nearest_on_edge(spec: &EdgeSpec, x: Point3<f64>, read: &Read) -> Result<Near, StepImportError> {
    let (t0, t1) = (spec.t0, spec.t1);
    let t = match param_at(&spec.carrier, x, t0) {
        Some(t) => t,
        None => {
            // No closed-form foot: sampled, and refused only where a
            // sample comes near enough that the sampling could decide.
            let n = 256;
            let mut best = f64::INFINITY;
            let mut step = 0.0_f64;
            let mut prev = spec.carrier.eval(t0);
            for i in 0..=n {
                let p = spec
                    .carrier
                    .eval(t0 + (t1 - t0) * f64::from(i) / f64::from(n));
                best = best.min((p - x).norm());
                step = step.max((p - prev).norm());
                prev = p;
            }
            return Ok(if best > read.escalate + step {
                Near::Off
            } else {
                Near::Unread
            });
        }
    };
    if !(t > t0 && t < t1) {
        // The closest point of the carrier is outside the edge; the
        // edge's nearest point is an end, which the vertex test read.
        return Ok(Near::Off);
    }
    let d = (spec.carrier.eval(t) - x).norm();
    if d > read.escalate {
        return Ok(Near::Off);
    }
    // Its ends are the face's vertices, which the vertex test read: a
    // point within the band of one was taken or refused there.
    Ok(Near::On { t, d })
}

/// The two uses of the single edge at `vertex` in face `fi`'s loop: the
/// one arriving at the vertex and the one leaving it, when the face
/// uses that edge once each way (a seam ending at a strut tip).
fn strut_uses(solid: &SolidSpec, fi: usize, vertex: u64) -> Option<(usize, usize)> {
    let [lp] = solid.faces[fi].loops.as_slice() else {
        return None;
    };
    let arrive: Vec<usize> = (0..lp.uses.len())
        .filter(|&i| use_end(solid, lp.uses[i]) == vertex)
        .collect();
    let leave: Vec<usize> = (0..lp.uses.len())
        .filter(|&i| use_start(solid, lp.uses[i]) == vertex)
        .collect();
    match (arrive.as_slice(), leave.as_slice()) {
        ([a], [l]) if lp.uses[*a].edge == lp.uses[*l].edge => Some((*a, *l)),
        _ => None,
    }
}

/// The vertex a use ENDS at.
fn use_end(solid: &SolidSpec, use_: EdgeUse) -> u64 {
    let spec = &solid.edges[&use_.edge];
    if use_.forward { spec.end } else { spec.start }
}

/// One loop read through the chart: its winding about the surface axis
/// (in turns), the signed area of its chart image, and its sample of
/// largest `|v|`, whose sign is the cone's nappe the loop runs on.
struct LoopReading {
    winding: i64,
    area: f64,
    /// The chart `v` of the loop's sample farthest from `v = 0`.
    farthest_v: f64,
}

/// [`LoopReading`] of one loop. A sample at a singular point has no
/// chart preimage and is skipped: the step across it is read as the
/// nearest representative, which stands for a detour round that point
/// on one side or the other — and so leaves the side of every OTHER
/// singular point as it is, which is all this reading is asked.
fn read_loop(
    solid: &SolidSpec,
    surface: &Surface<f64>,
    lp: &LoopSpec,
    singular: &[Point3<f64>],
    read: &Read,
    id: u64,
) -> Result<LoopReading, StepImportError> {
    use core::f64::consts::{PI, TAU};
    let wrap = |d: f64| {
        let (s, c) = d.sin_cos();
        s.atan2(c)
    };
    let mut charted: Vec<Point2<f64>> = Vec::new();
    for u in &lp.uses {
        let spec = &solid.edges[&u.edge];
        let (a, b) = if u.forward {
            (spec.t0, spec.t1)
        } else {
            (spec.t1, spec.t0)
        };
        // Parameter samples, refined where the chart azimuth steps by
        // more than a sixteenth of a turn. The end is the next use's
        // start, so it is left to that use.
        let at = |t: f64| -> Result<Option<Point2<f64>>, StepImportError> {
            let p = spec.carrier.eval(t);
            match crate::chart::uv_of(surface, p, read.band) {
                Some(uv) => Ok(Some(uv)),
                None if singular.iter().any(|s| (*s - p).norm() <= read.escalate) => Ok(None),
                None => Err(unreadable(id)),
            }
        };
        let n = 16;
        let mut ts: Vec<f64> = (0..=n)
            .map(|i| a + (b - a) * f64::from(i) / f64::from(n))
            .collect();
        let mut uvs: Vec<Option<Point2<f64>>> =
            ts.iter().map(|&t| at(t)).collect::<Result<_, _>>()?;
        for _ in 0..60 {
            let mut refined = false;
            let mut i = 0;
            while i + 1 < ts.len() {
                if let (Some(p), Some(q)) = (uvs[i], uvs[i + 1])
                    && wrap(q.x - p.x).abs() > PI / 8.0
                    && (ts[i + 1] - ts[i]).abs() > 1e-15 * (1.0 + ts[i].abs())
                {
                    let tm = 0.5 * (ts[i] + ts[i + 1]);
                    ts.insert(i + 1, tm);
                    uvs.insert(i + 1, at(tm)?);
                    refined = true;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            if !refined || ts.len() > 1 << 16 {
                break;
            }
        }
        charted.extend(uvs[..uvs.len() - 1].iter().flatten());
    }
    if charted.len() < 3 {
        return Err(unreadable(id));
    }
    let mut lifted = Vec::with_capacity(charted.len());
    let mut total = 0.0;
    for (i, p) in charted.iter().enumerate() {
        if i == 0 {
            lifted.push(*p);
            continue;
        }
        let step = wrap(p.x - charted[i - 1].x);
        total += step;
        lifted.push(Point2::new(lifted[i - 1].x + step, p.y));
    }
    total += wrap(charted[0].x - charted[charted.len() - 1].x);
    let turns = total / TAU;
    #[allow(clippy::cast_possible_truncation)] // a turn count, checked whole below
    let winding = turns.round() as i64;
    if (turns - turns.round()).abs() > 1e-6 || winding.abs() > 1 {
        return Err(unreadable(id));
    }
    let mut area = 0.0;
    for i in 0..lifted.len() {
        let (p, q) = (lifted[i], lifted[(i + 1) % lifted.len()]);
        area += p.x * q.y - q.x * p.y;
    }
    let v_extreme = charted
        .iter()
        .map(|p| p.y)
        .fold(0.0_f64, |acc, v| if v.abs() > acc.abs() { v } else { acc });
    Ok(LoopReading {
        winding,
        area: area / 2.0,
        farthest_v: v_extreme,
    })
}

/// Whether the singular point `k` of `chart` lies on the face's side of
/// every one of its loops (module docs), the point being off them.
fn holds(
    solid: &SolidSpec,
    fi: usize,
    chart: Chart,
    k: usize,
    read: &Read,
) -> Result<bool, StepImportError> {
    let face = &solid.faces[fi];
    let singular = chart.singular();
    for lp in &face.loops {
        let r = read_loop(solid, &face.surface, lp, &singular, read, face.id)?;
        if r.winding == 0 && r.area == 0.0 {
            return Err(unreadable(face.id));
        }
        let left = match chart {
            Chart::Sphere { .. } => match r.winding {
                0 => r.area < 0.0,
                w => (w == 1) == (k == 0),
            },
            // A loop that does not ring the apex leaves it on the side
            // that runs to infinity, which bounds no face.
            Chart::Cone { .. } => match r.winding {
                0 => return Ok(false),
                w => {
                    if r.farthest_v == 0.0 {
                        return Err(unreadable(face.id));
                    }
                    (w == 1) == (r.farthest_v < 0.0)
                }
            },
        };
        if left != face.sense {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Applies one disposition, reporting it.
fn apply(
    solid: &mut SolidSpec,
    action: Action,
    read: &Read,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    let fi = match action {
        Action::Interior { fi, .. }
        | Action::InsideEdge { fi, .. }
        | Action::StrutTip { fi, .. } => fi,
    };
    let face_id = solid.faces[fi].id;
    let (entry_edges, entry_vertices) = face_boundary_census(solid, fi);
    let (found, faces) = match action {
        Action::InsideEdge { edge, t, point, .. } => {
            split_at_point(solid, edge, t, point, mint)?;
            (SingularityFound::InsideEdge, 1)
        }
        Action::Interior { k, .. } => (
            SingularityFound::Interior,
            interior(solid, fi, k, read, mint)?,
        ),
        Action::StrutTip { vertex, .. } => (
            SingularityFound::ValenceOneVertex,
            strut_tip(solid, fi, vertex, read, mint)?,
        ),
    };
    let (mut edges, mut vertices) = (
        std::collections::BTreeSet::new(),
        std::collections::BTreeSet::new(),
    );
    for face in &solid.faces[fi..fi + faces] {
        for lp in &face.loops {
            for u in &lp.uses {
                edges.insert(u.edge);
                let spec = &solid.edges[&u.edge];
                vertices.insert(spec.start);
                vertices.insert(spec.end);
            }
        }
    }
    sink.push(StructureNormalization {
        face: face_id,
        kind: NormalizationKind::ChartSingularityVertex { found },
        file_census: FaceCensus {
            faces: 1,
            edges: entry_edges,
            vertices: entry_vertices,
        },
        kernel_census: FaceCensus {
            faces,
            edges: edges.len(),
            vertices: vertices.len(),
        },
    });
    Ok(())
}

/// Splits `edge` at its interior parameter `t`, the new vertex placed
/// at `point` exactly and both halves' intervals re-derived from their
/// endpoints, as a re-import derives them; the split reaches every face
/// that uses the edge. Answers the new vertex.
fn split_at_point(
    solid: &mut SolidSpec,
    edge: u64,
    t: f64,
    point: Point3<f64>,
    mint: &mut dyn FnMut() -> u64,
) -> Result<u64, StepImportError> {
    let (first, second, v) = split_at_param(solid, edge, t, mint);
    solid.vertices.insert(v, point);
    for half in [first, second] {
        let Some(spec) = solid.edges.get_mut(&half) else {
            unreachable!("a split half minted by `split_at_param` is in `solid.edges`")
        };
        let self_loop = spec.start == spec.end;
        let (ps, pe) = (solid.vertices[&spec.start], solid.vertices[&spec.end]);
        let (t0, t1) = crate::geometry::endpoint_params(half, &spec.carrier, ps, pe, self_loop)?;
        spec.t0 = t0;
        spec.t1 = t1;
    }
    expand_split_uses(solid, edge, first, second, &[]);
    Ok(v)
}

/// A ray: a carrier walked from the parameter `from` in the direction
/// `sign`, at most `reach` of parameter.
struct Ray {
    carrier: Curve3<f64>,
    from: f64,
    sign: f64,
    reach: f64,
}

/// Where a ray first meets the face's boundary.
enum Hit {
    /// At an existing vertex.
    Vertex(u64),
    /// Inside an edge, at its parameter `t`.
    Edge { edge: u64, t: f64 },
    /// At another singular point, off the boundary.
    Singular(Point3<f64>),
    /// Where a landing beside a boundary vertex is undecided: the refusal,
    /// raised only if the ray meets nothing sooner.
    Refused(StepImportError),
}

/// The carrier length per unit parameter (the radius for a circle).
fn speed(carrier: &Curve3<f64>) -> f64 {
    match *carrier {
        Curve3::Circle { radius, .. } => radius,
        Curve3::Line { dir, .. } => dir.norm(),
        _ => 1.0,
    }
}

/// The first point past its start at which `ray` meets the boundary of
/// face `fi`, ignoring `skip`'s edge (a seam the ray leaves from), or
/// reaches one of `singular` off the boundary.
fn first_hit(
    solid: &SolidSpec,
    fi: usize,
    ray: &Ray,
    skip: Option<u64>,
    singular: &[Point3<f64>],
    read: &Read,
) -> Result<(Hit, Point3<f64>), StepImportError> {
    let face = &solid.faces[fi];
    let lead = read.escalate / speed(&ray.carrier);
    let mut best: Option<(f64, Hit, Point3<f64>)> = None;
    let offer =
        |delta: f64, hit: Hit, p: Point3<f64>, best: &mut Option<(f64, Hit, Point3<f64>)>| {
            if delta > lead
                && delta <= ray.reach + lead
                && best.as_ref().is_none_or(|(b, ..)| delta < *b)
            {
                *best = Some((delta, hit, p));
            }
        };
    let delta_of = |t: f64| (t - ray.from) * ray.sign;
    // How far along the ray its parameter `t` lies: on a circle, the
    // representative past the start by at most a turn.
    let walked = |t: f64| {
        let mut delta = delta_of(t);
        if matches!(ray.carrier, Curve3::Circle { .. }) {
            let tau = core::f64::consts::TAU;
            delta = delta.rem_euclid(tau);
            if delta <= lead {
                delta += tau;
            }
        }
        delta
    };
    let mut edges: Vec<u64> = face
        .loops
        .iter()
        .flat_map(|lp| lp.uses.iter().map(|u| u.edge))
        .collect();
    edges.sort_unstable();
    edges.dedup();
    for edge in edges {
        if Some(edge) == skip {
            continue;
        }
        let spec = &solid.edges[&edge];
        // A boundary vertex on the ray is met there, whatever the
        // carriers do between (a boundary arc running along the ray
        // crosses it nowhere, and meets it at its ends).
        for v in [spec.start, spec.end] {
            let p = solid.vertices[&v];
            if let Some(t) = foot(&ray.carrier, p, read) {
                offer(walked(t), Hit::Vertex(v), p, &mut best);
            }
        }
        for (t_ray, p) in
            crossings(&ray.carrier, &spec.carrier, read).ok_or(foreign_carrier(face.id))?
        {
            let delta = walked(t_ray);
            let te = param_at(&spec.carrier, p, spec.t0).ok_or(foreign_carrier(face.id))?;
            // At one of the edge's ends, decided at the pass's band as
            // the singular point itself is: within it, the landing is
            // that vertex; past it, the edge is split there; between,
            // the landing is undecided and refuses if it is the one the
            // ray meets first.
            let ends = [spec.start, spec.end].map(|v| (v, solid.vertices[&v]));
            let (v, pv) = if (p - ends[0].1).norm() <= (p - ends[1].1).norm() {
                ends[0]
            } else {
                ends[1]
            };
            match on_boundary((p - pv).norm(), read, face.id) {
                Ok(true) => offer(delta, Hit::Vertex(v), pv, &mut best),
                Ok(false) if te > spec.t0 && te < spec.t1 => {
                    offer(delta, Hit::Edge { edge, t: te }, p, &mut best);
                }
                Ok(false) => {}
                Err(refused) => offer(delta, Hit::Refused(refused), p, &mut best),
            }
        }
    }
    for &s in singular {
        if let Some(t) = foot(&ray.carrier, s, read) {
            let delta = walked(t);
            if best.as_ref().is_none_or(|(b, ..)| delta < *b - lead) {
                offer(delta, Hit::Singular(s), s, &mut best);
            }
        }
    }
    match best {
        Some((_, Hit::Refused(refused), _)) => Err(refused),
        Some((_, hit, p)) => Ok((hit, p)),
        None => Err(StepImportError::Topology {
            id: face.id,
            what: "internal: a pole-form meridian minted inside a face meets none of its \
               boundary",
        }),
    }
}

/// The parameter of `carrier` at `x`, when `x` lies on it within ε_in.
fn foot(carrier: &Curve3<f64>, x: Point3<f64>, read: &Read) -> Option<f64> {
    let t = param_at(carrier, x, 0.0)?;
    ((carrier.eval(t) - x).norm() <= read.eps).then_some(t)
}

/// Where the carriers `ray` and `other` cross: the ray's parameter and
/// the point, for each crossing. `None` for a pair this pass reads no
/// crossing of. Where `other` runs along the ray, its points ON the ray
/// are its crossings' candidates — its vertices are found by the
/// caller's end tests, so a coincident carrier offers the ray points
/// at its own two ends.
fn crossings(
    ray: &Curve3<f64>,
    other: &Curve3<f64>,
    read: &Read,
) -> Option<Vec<(f64, Point3<f64>)>> {
    // `other` as a plane-and-distance test (a circle) or a line.
    match (ray, other) {
        (
            &Curve3::Circle {
                center: c,
                axis: n,
                radius: r,
                u_ref: u,
            },
            &Curve3::Circle {
                center: ce,
                axis: ne,
                radius: re,
                ..
            },
        ) => {
            let w = n.cross(u);
            // ne · (C(t) − ce) = a cos t + b sin t + k.
            let (a, b, k) = (r * ne.dot(u), r * ne.dot(w), ne.dot(c - ce));
            let rho = a.hypot(b);
            if rho <= read.eps {
                // Parallel planes: one circle (on one sphere) or none.
                return Some(Vec::new());
            }
            let phase = b.atan2(a);
            let cos = -k / rho;
            if cos.abs() > 1.0 {
                return Some(Vec::new());
            }
            let spread = cos.acos();
            Some(
                [phase + spread, phase - spread]
                    .into_iter()
                    .map(|t| (t, ray.eval(t)))
                    .filter(|(_, p)| ((*p - ce).norm() - re).abs() <= read.escalate)
                    .collect(),
            )
        }
        (
            &Curve3::Line { origin: o, dir: g },
            &Curve3::Circle {
                center: ce,
                axis: ne,
                radius: re,
                ..
            },
        ) => {
            let den = ne.dot(g);
            if den.abs() <= 1e-12 * g.norm() {
                return Some(Vec::new());
            }
            let t = ne.dot(ce - o) / den;
            let p = ray.eval(t);
            Some(if ((p - ce).norm() - re).abs() <= read.escalate {
                vec![(t, p)]
            } else {
                Vec::new()
            })
        }
        (
            &Curve3::Line { origin: o, dir: g },
            &Curve3::Line {
                origin: oe,
                dir: ge,
            },
        ) => {
            // The closest approach of two lines; generators of one cone
            // meet only at its apex, which the caller's lead skips.
            let wv = o - oe;
            let (aa, bb, cc, dd, ee) = (g.dot(g), g.dot(ge), ge.dot(ge), g.dot(wv), ge.dot(wv));
            let den = aa * cc - bb.powi(2);
            if den.abs() <= 1e-12 * aa * cc {
                return Some(Vec::new());
            }
            let t = (bb * ee - cc * dd) / den;
            let s = (aa * ee - bb * dd) / den;
            let (p, q) = (ray.eval(t), other.eval(s));
            Some(if (p - q).norm() <= read.escalate {
                vec![(t, p)]
            } else {
                Vec::new()
            })
        }
        (
            &Curve3::Circle {
                center: c,
                axis: n,
                radius: r,
                ..
            },
            &Curve3::Line {
                origin: oe,
                dir: ge,
            },
        ) => {
            let den = n.dot(ge);
            if den.abs() <= 1e-12 * ge.norm() {
                return Some(Vec::new());
            }
            let s = n.dot(c - oe) / den;
            let p = other.eval(s);
            if ((p - c).norm() - r).abs() > read.escalate {
                return Some(Vec::new());
            }
            let t = foot(
                ray,
                p,
                &Read {
                    eps: read.escalate,
                    ..*read
                },
            )?;
            Some(vec![(t, p)])
        }
        _ => None,
    }
}

/// The pole form's meridian rays from singular point `k` of `chart`,
/// at the azimuths `d` and `−d`, walked into the face (`nappe` is the
/// sign of the cone face's chart `v`).
fn meridian_rays(chart: Chart, k: usize, d: Vec3<f64>, nappe: f64) -> [Ray; 2] {
    geom::pole_form::azimuths(d).map(|d| match chart {
        Chart::Sphere {
            center,
            radius,
            axis,
            ..
        } => Ray {
            carrier: crate::signed_zero::plus_zero_curve(geom::pole_form::sphere_meridian(
                center, radius, axis, d,
            )),
            from: if k == 0 { 0.0 } else { core::f64::consts::PI },
            sign: if k == 0 { 1.0 } else { -1.0 },
            reach: core::f64::consts::PI,
        },
        Chart::Cone {
            apex,
            axis,
            half_angle,
            ..
        } => Ray {
            carrier: crate::signed_zero::plus_zero_curve(geom::pole_form::cone_generator(
                apex, axis, half_angle, d,
            )),
            from: 0.0,
            sign: nappe,
            reach: f64::INFINITY,
        },
    })
}

/// Requires every boundary carrier of face `fi` to be one this pass
/// reads a crossing of.
fn require_read_carriers(solid: &SolidSpec, fi: usize) -> Result<(), StepImportError> {
    let face = &solid.faces[fi];
    for lp in &face.loops {
        for u in &lp.uses {
            if !matches!(
                solid.edges[&u.edge].carrier,
                Curve3::Line { .. } | Curve3::Circle { .. }
            ) {
                return Err(foreign_carrier(face.id));
            }
        }
    }
    Ok(())
}

/// Resolves a hit to a vertex of the boundary, splitting the edge it
/// lies inside or minting the singular point it reaches.
fn land(
    solid: &mut SolidSpec,
    hit: Hit,
    p: Point3<f64>,
    mint: &mut dyn FnMut() -> u64,
) -> Result<u64, StepImportError> {
    match hit {
        Hit::Vertex(v) => Ok(v),
        Hit::Edge { edge, t } => split_at_point(solid, edge, t, p, mint),
        Hit::Singular(s) => {
            let v = mint();
            solid.vertices.insert(v, s);
            Ok(v)
        }
        Hit::Refused(refused) => Err(refused),
    }
}

/// Mints the edge of `ray` between the vertices `from` (at the ray's
/// start) and `to`, in the carrier's increasing direction. Answers the
/// edge and whether it runs from `from` to `to`.
fn mint_spoke(
    solid: &mut SolidSpec,
    carrier: Curve3<f64>,
    sign: f64,
    from: u64,
    to: u64,
    mint: &mut dyn FnMut() -> u64,
) -> Result<(u64, bool), StepImportError> {
    let id = mint();
    let forward = sign > 0.0;
    let (start, end) = if forward { (from, to) } else { (to, from) };
    let (t0, t1) = crate::geometry::endpoint_params(
        id,
        &carrier,
        solid.vertices[&start],
        solid.vertices[&end],
        false,
    )?;
    solid.edges.insert(
        id,
        EdgeSpec {
            start,
            end,
            carrier,
            t0,
            t1,
            // A minted edge: the importer states its own
            // start → end, so there is no sense to compose.
            reversed: false,
        },
    );
    Ok((id, forward))
}

/// **Interior**: mints the singular point `k` as a vertex with the pole
/// form's two meridians and cuts the face along them. Answers how many
/// faces the face became.
fn interior(
    solid: &mut SolidSpec,
    fi: usize,
    k: usize,
    read: &Read,
    mint: &mut dyn FnMut() -> u64,
) -> Result<usize, StepImportError> {
    require_read_carriers(solid, fi)?;
    let face_id = solid.faces[fi].id;
    let Some(chart) = Chart::of(&solid.faces[fi].surface) else {
        unreachable!("the scan reads only sphere and cone faces")
    };
    let singular = chart.singular();
    let x = singular[k];
    let others: Vec<Point3<f64>> = singular
        .iter()
        .enumerate()
        .filter(|&(j, _)| j != k)
        .map(|(_, s)| *s)
        .collect();
    let (axis, origin) = (chart.axis(), chart.origin());
    // The azimuth: the surface's own u_ref, unless the face holds the
    // other pole too. Then its loop bounds a pole-free hole, and the
    // azimuth is that of the loop's centroid, which the hole surrounds:
    // the meridian there crosses the hole through it, meeting the loop at
    // two distinct points, rather than grazing it at one (a boundary
    // vertex's azimuth can be tangent to the hole there). A centroid on
    // the axis falls back to a boundary vertex's azimuth.
    let mut d = chart.u_ref();
    if let Some(&y) = others.first()
        && holds(solid, fi, chart, 1 - k, read)?
    {
        let lp = &solid.faces[fi].loops[0];
        let radial = |p: Point3<f64>| {
            let w = p - origin;
            let r = w - axis * w.dot(axis);
            (r.norm() > read.escalate).then(|| r / r.norm())
        };
        let mut sum = Vec3::new(0.0, 0.0, 0.0);
        let mut count = 0.0;
        for u in &lp.uses {
            let spec = &solid.edges[&u.edge];
            for i in 0..8 {
                let t = spec.t0 + (spec.t1 - spec.t0) * f64::from(i) / 8.0;
                sum = sum + (spec.carrier.eval(t) - origin);
                count += 1.0;
            }
        }
        let centroid = origin + sum * (1.0 / count);
        let vertex = || {
            lp.uses.iter().find_map(|u| {
                let p = solid.vertices[&use_start(solid, *u)];
                if (p - y).norm() > read.escalate {
                    radial(p)
                } else {
                    None
                }
            })
        };
        d = radial(centroid)
            .or_else(vertex)
            .ok_or(unreadable(face_id))?;
    }
    let nappe = match chart {
        Chart::Cone { .. } => {
            let r = read_loop(
                solid,
                &solid.faces[fi].surface,
                &solid.faces[fi].loops[0],
                &singular,
                read,
                face_id,
            )?;
            if r.farthest_v == 0.0 {
                return Err(unreadable(face_id));
            }
            r.farthest_v.signum()
        }
        Chart::Sphere { .. } => 1.0,
    };
    let pole = mint();
    solid.vertices.insert(pole, x);
    let mut spokes = Vec::with_capacity(2);
    // Each ray is read against the boundary as the previous one left
    // it, so a split the first made is an edge the second can meet.
    for ray in meridian_rays(chart, k, d, nappe) {
        let (hit, p) = first_hit(solid, fi, &ray, None, &others, read)?;
        let far = land(solid, hit, p, mint)?;
        spokes.push(mint_spoke(solid, ray.carrier, ray.sign, pole, far, mint)?);
    }
    cut(solid, fi, pole, &spokes, None)
}

/// **A valence-1 vertex**: completes the pole form at `vertex` with the
/// half-turn twin of the seam that ends there, and cuts the face along
/// the pair. Answers how many faces the face became.
fn strut_tip(
    solid: &mut SolidSpec,
    fi: usize,
    vertex: u64,
    read: &Read,
    mint: &mut dyn FnMut() -> u64,
) -> Result<usize, StepImportError> {
    let face_id = solid.faces[fi].id;
    let Some(chart) = Chart::of(&solid.faces[fi].surface) else {
        unreachable!("the scan reads only sphere and cone faces")
    };
    let singular = chart.singular();
    let x = solid.vertices[&vertex];
    let Some(k) = singular.iter().position(|s| (*s - x).norm() <= read.eps) else {
        unreachable!("the scan finds a strut tip at a singular point")
    };
    // The face's material must hold the seam it uses both ways: a seam
    // outside it is an inverted face (the module docs' orientation
    // doctrine).
    if !holds(solid, fi, chart, k, read)? {
        return Err(inverted(face_id));
    }
    require_read_carriers(solid, fi)?;
    let Some((arrive, _)) = strut_uses(solid, fi, vertex) else {
        unreachable!("the scan finds a strut tip only where the face uses its seam both ways")
    };
    let seam = solid.faces[fi].loops[0].uses[arrive].edge;
    let spec = &solid.edges[&seam];
    let Some(twin) = geom::pole_form::half_turn_curve(
        &spec.carrier,
        chart.origin(),
        chart.axis(),
        crate::signed_zero::plus_zero_point,
        crate::signed_zero::plus_zero,
    ) else {
        return Err(foreign_carrier(face_id));
    };
    // The twin leaves the tip the way the seam does, turned.
    let (from, sign) = if spec.start == vertex {
        (spec.t0, 1.0)
    } else {
        (spec.t1, -1.0)
    };
    let reach = match twin {
        Curve3::Circle { .. } => core::f64::consts::TAU,
        _ => f64::INFINITY,
    };
    let ray = Ray {
        carrier: twin,
        from,
        sign,
        reach,
    };
    let others: Vec<Point3<f64>> = singular
        .iter()
        .enumerate()
        .filter(|&(j, _)| j != k)
        .map(|(_, s)| *s)
        .collect();
    let (hit, p) = first_hit(solid, fi, &ray, Some(seam), &others, read)?;
    let far = land(solid, hit, p, mint)?;
    let spoke = mint_spoke(solid, ray.carrier, ray.sign, vertex, far, mint)?;
    cut(solid, fi, vertex, &[spoke], Some(seam))
}

/// Cuts face `fi` along the spokes leaving `pole`, each an edge and
/// whether it runs from the pole, and the seam `seam` the face already
/// uses both ways at the pole. The face's loop and the spokes, each
/// walked both ways, are re-linked into cycles: a spoke enters the face
/// where it meets the loop, each walk at the pole leaves along the
/// other edge there, and a spoke whose far end is off the loop is
/// walked out and back. Each cycle is a face. Answers how many.
fn cut(
    solid: &mut SolidSpec,
    fi: usize,
    pole: u64,
    spokes: &[(u64, bool)],
    seam: Option<u64>,
) -> Result<usize, StepImportError> {
    let face_id = solid.faces[fi].id;
    let internal = |what| StepImportError::Topology { id: face_id, what };
    let [lp] = solid.faces[fi].loops.as_slice() else {
        return Err(internal(
            "internal: a chart singularity cut on a face with rings, which the face gate \
             admits on no sphere or cone",
        ));
    };
    let mut all: Vec<EdgeUse> = lp.uses.clone();
    let n = all.len();
    for &(edge, from_pole) in spokes {
        // Out of the pole, then back.
        all.push(EdgeUse {
            edge,
            forward: from_pole,
        });
        all.push(EdgeUse {
            edge,
            forward: !from_pole,
        });
    }
    let start = |i: usize| use_start(solid, all[i]);
    let end = |i: usize| use_end(solid, all[i]);
    let loop_leaving = |v: u64| -> Result<usize, StepImportError> {
        let found: Vec<usize> = (0..n).filter(|&i| start(i) == v).collect();
        match found.as_slice() {
            [i] => Ok(*i),
            _ => Err(internal(
                "a pole-form meridian ends at a vertex the face's loop passes more than once; \
                 the cut there has no one reading",
            )),
        }
    };
    // Spoke `s` leaves the pole as `n + 2s` and returns as `n + 2s + 1`.
    let out = |s: usize| n + 2 * s;
    let back = |s: usize| n + 2 * s + 1;
    let far = |s: usize| end(out(s));
    let link = |i: usize| -> Result<usize, StepImportError> {
        let v = end(i);
        Ok(if i < n {
            if v == pole {
                // The seam arriving at its tip leaves along the twin.
                out(0)
            } else if let Some(s) = (0..spokes.len()).find(|&s| far(s) == v) {
                back(s)
            } else {
                (i + 1) % n
            }
        } else if (i - n).is_multiple_of(2) {
            // A spoke arriving at its far end: into the loop, on along
            // another spoke ending there, or back along itself.
            let s = (i - n) / 2;
            if (0..n).any(|j| start(j) == v) {
                loop_leaving(v)?
            } else if let Some(o) = (0..spokes.len()).find(|&o| o != s && far(o) == v) {
                back(o)
            } else {
                back(s)
            }
        } else {
            // A spoke arriving at the pole leaves along the other edge
            // there: the other spoke, or the seam.
            let s = (i - n) / 2;
            match (seam, spokes.len()) {
                (Some(_), 1) => loop_leaving(pole)?,
                (None, 2) => out(1 - s),
                _ => {
                    return Err(internal(
                        "internal: a pole vertex with other than two edges",
                    ));
                }
            }
        })
    };
    let next = (0..all.len()).map(link).collect::<Result<Vec<_>, _>>()?;
    let mut seen = vec![false; all.len()];
    let mut cycles: Vec<Vec<EdgeUse>> = Vec::new();
    for s in 0..all.len() {
        if seen[s] {
            continue;
        }
        let mut cycle = Vec::new();
        let mut i = s;
        while !seen[i] {
            seen[i] = true;
            cycle.push(all[i]);
            i = next[i];
        }
        if i != s {
            return Err(internal(
                "internal: a chart singularity cut's walk does not close",
            ));
        }
        cycles.push(cycle);
    }
    if cycles.len() > 2 {
        return Err(internal(
            "a sphere face whose pole-form meridians close round both poles without meeting \
             its boundary, which would leave the boundary a ring of one half: this re-mint \
             adopts no ring on a curved chart",
        ));
    }
    let template = &solid.faces[fi];
    let (surface, sense) = (template.surface.clone(), template.sense);
    let faces: Vec<FaceSpec> = cycles
        .into_iter()
        .map(|uses| FaceSpec {
            id: face_id,
            surface: surface.clone(),
            sense,
            loops: vec![LoopSpec { outer: true, uses }],
            band: false,
        })
        .collect();
    let count = faces.len();
    solid.faces.splice(fi..=fi, faces);
    Ok(count)
}

#[cfg(test)]
mod review_on_boundary {
    use super::{Read, on_boundary};

    /// The three-step decision, swept over d at the eps rows with ε_in
    /// above and below ε: "on" only within the coarser zero, "off" only
    /// past both escalations, and a refusal only inside a band.
    #[test]
    fn on_boundary_decides_only_where_a_band_reads() {
        let k = 10.0;
        for eps in [1e-6, 1e-9, 1e-12] {
            for eps_in in [eps * 1e-3, eps * 0.3, eps, eps * 3.0, eps * 1e3, 0.0] {
                let run = geom_core::Band::new(eps, k * eps).ok();
                let file = if eps_in == 0.0 {
                    Some(geom_core::Band::least())
                } else {
                    geom_core::Band::new(eps_in, k * eps_in).ok()
                };
                let zero = |b: Option<geom_core::Band>| b.map_or(0.0, geom_core::Band::zero);
                let esc = |b: Option<geom_core::Band>| b.map_or(0.0, geom_core::Band::escalate);
                let read = Read {
                    eps: eps_in.max(zero(file)).max(zero(run)),
                    escalate: eps_in.max(esc(file)).max(esc(run)),
                    band: file,
                    run,
                    file: geom_core::FileCoincidence::new(eps_in),
                };
                let top = 100.0 * eps.max(eps_in);
                for i in 0..=4000 {
                    let d = top * f64::from(i) / 4000.0;
                    match on_boundary(d, &read, 1) {
                        Ok(true) => assert!(
                            d <= zero(run).max(zero(file)),
                            "eps {eps} eps_in {eps_in}: on at d = {d}"
                        ),
                        Ok(false) => assert!(
                            d >= esc(run).max(esc(file)) || d >= esc(run) && d >= esc(file),
                            "eps {eps} eps_in {eps_in}: off at d = {d}"
                        ),
                        Err(e) => assert!(
                            (d > zero(run) && d < esc(run)) || (d > zero(file) && d < esc(file)),
                            "eps {eps} eps_in {eps_in}: refused at d = {d}: {e}"
                        ),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod side_through_a_pole {
    use std::collections::BTreeMap;

    use geom::{Curve3, Surface};
    use geom_core::{Point3, Vec3};

    use super::{Chart, Read, holds};
    use crate::entities::{EdgeSpec, EdgeUse, FaceSpec, LoopSpec, SolidSpec};

    /// **`read_loop`'s claim, pinned**: a loop passing THROUGH one pole
    /// (its samples there skipped) still reads the side of the OTHER.
    /// The half-cap of a unit sphere above latitude 0.5 in `y ≥ 0`,
    /// bounded by its rim half and the meridian arc over the north pole
    /// (a vertex there): under `same_sense` the face is the half-cap and
    /// the south pole lies outside it; flipped, the face is its
    /// complement and holds the south pole. A skipped step read as the
    /// wrong detour would put the south pole on the same side both ways.
    #[test]
    fn a_loop_through_one_pole_reads_the_side_of_the_other() {
        let b = 0.5_f64;
        let (a, n, bb) = (
            Point3::new(b.cos(), 0.0, b.sin()),
            Point3::new(0.0, 0.0, 1.0),
            Point3::new(-b.cos(), 0.0, b.sin()),
        );
        let rim = Curve3::Circle {
            center: Point3::new(0.0, 0.0, b.sin()),
            axis: Vec3::new(0.0, 0.0, 1.0),
            radius: b.cos(),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        // θ runs from +x up through the north pole (θ = π/2) to −x.
        let meridian = Curve3::Circle {
            center: Point3::new(0.0, 0.0, 0.0),
            axis: Vec3::new(0.0, -1.0, 0.0),
            radius: 1.0,
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let edge = |start, end, carrier: &Curve3<f64>, t0, t1| EdgeSpec {
            start,
            end,
            carrier: carrier.clone(),
            t0,
            t1,
            reversed: false,
        };
        let pi = core::f64::consts::PI;
        let edges = || {
            BTreeMap::from([
                (11, edge(1, 3, &rim, 0.0, pi)),
                (12, edge(1, 2, &meridian, b, pi / 2.0)),
                (13, edge(2, 3, &meridian, pi / 2.0, pi - b)),
            ])
        };
        let vertices = BTreeMap::from([(1, a), (2, n), (3, bb)]);
        let surface = Surface::Sphere {
            center: Point3::new(0.0, 0.0, 0.0),
            radius: 1.0,
            axis: Vec3::new(0.0, 0.0, 1.0),
            u_ref: Vec3::new(1.0, 0.0, 0.0),
        };
        let use_ = |edge, forward| EdgeUse { edge, forward };
        let read = Read {
            eps: 1e-9,
            escalate: 1e-8,
            band: geom_core::Band::new(1e-9, 1e-8).ok(),
            run: geom_core::Band::new(1e-9, 1e-8).ok(),
            file: geom_core::FileCoincidence::new(1e-9),
        };
        let chart = Chart::of(&surface).unwrap();
        for (sense, south_inside) in [(true, false), (false, true)] {
            let solid = SolidSpec {
                id: 100,
                faces: vec![FaceSpec {
                    id: 101,
                    surface: surface.clone(),
                    sense,
                    loops: vec![LoopSpec {
                        outer: true,
                        uses: vec![use_(11, true), use_(13, false), use_(12, false)],
                    }],
                    band: false,
                }],
                edges: edges(),
                vertices: vertices.clone(),
                band_seams: Default::default(),
            };
            assert_eq!(
                holds(&solid, 0, chart, 1, &read).unwrap(),
                south_inside,
                "sense {sense}: the south pole's side, read through a loop over the north pole"
            );
        }
    }
}
