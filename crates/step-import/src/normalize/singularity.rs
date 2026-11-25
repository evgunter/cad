//! **A chart singularity in a face is a vertex of it**: the pass that
//! makes it one ([`crate::NormalizationKind::ChartSingularityVertex`]
//! states the rule and what each disposition mints).
//!
//! The trigger is incidence, read off the face's region: a sphere's
//! pole or a cone's apex that lies in the face's closure and is not a
//! vertex of it. Where it lies decides the disposition
//! ([`SingularityFound`]):
//!
//! - **Interior**: a vertex is minted there with one wrap edge to it,
//!   the surface's meridian ([`geom::pole_form`]) run to the first
//!   boundary edge it meets, which the face's loop walks out and back.
//! - **Inside an edge**: the edge is split there.
//!
//! A vertex there already is settled. Where it is the tip of a wrap
//! edge the file states (Open CASCADE's seam meridian ending at the
//! pole or apex, a valence-1 vertex the face's loop walks out and
//! back), the face is adopted as stated, once its material is read to
//! hold the seam: a seam outside it is an inverted face.
//!
//! Each split of a boundary edge propagates to every face sharing it,
//! so the shell stays closed. Each wrap edge runs at the azimuth of the
//! face's boundary point nearest its tip ([`wrap_azimuth`]), so it meets
//! the boundary there before anything farther, the other pole of a
//! sphere face holding both included, however the file splits that
//! boundary into edges. Where a meridian lands is the
//! kernel's meeting reading of it against each boundary edge
//! ([`geom_brep::curve_pair::meet`]) at the pass's band: at a boundary
//! vertex where the meeting reads at the edge's end, splitting the edge
//! where it reads inside it, and refused where the reading refuses and
//! the ray meets nothing sooner.
//!
//! **Where it lies** is the kernel's one reading of it,
//! [`geom_brep::chart_singularity::locate`], whose boundary placement
//! tier 3 shares (tier 3 reads the side through the face's point-in-face
//! door, which this pass is filed to follow:
//! `work/importgate/import-reads-a-chart-singularitys-side-by-the-chart-walk-tier-3-is-to-leave.md`);
//! this pass keeps its dispositions and the band it reads
//! at, the coarser of the file's and the run's, so a point the kernel
//! reads as one with a vertex is not cut a sub-tolerance edge away from
//! it, and one the file cannot place is not placed finer.

use geom::{ChartInverse, Curve3, Surface};
use geom_brep::chart_singularity::{self as home, Lies, Unread};
use geom_brep::curve_pair::{self, Span, SpanEnd};
use geom_brep::props::LoopEdge;
use geom_core::{Point3, Vec3};

use super::surgery::{
    self, CutLine, Unlinked, boundary_census, split_at_point, use_end, use_start,
};
use super::valences;
use crate::entities::{EdgeSpec, EdgeUse, LoopSpec, SolidSpec};
use crate::error::StepImportError;
use crate::interpret::FileBand;
use crate::{FaceCensus, NormalizationKind, SingularityFound, StructureNormalization};

/// The surface data the pass reads: the axis through the singular
/// points, the reference azimuth, and the points themselves.
#[derive(Clone, Copy)]
enum Chart {
    /// A sphere: two poles, `[north, south]`.
    Sphere {
        /// The stored frame, carried back by [`Chart::surface`].
        frame: geom::Frame<f64>,
        center: Point3<f64>,
        radius: f64,
        axis: Vec3<f64>,
        u_ref: Vec3<f64>,
    },
    /// A cone: the apex.
    Cone {
        /// The stored frame, carried back by [`Chart::surface`].
        frame: geom::Frame<f64>,
        apex: Point3<f64>,
        axis: Vec3<f64>,
        half_angle: f64,
        u_ref: Vec3<f64>,
    },
}

impl Chart {
    fn of(surface: &Surface<f64>) -> Option<Self> {
        match *surface {
            Surface::Sphere { frame, radius } => {
                let (center, axis, u_ref) = (frame.origin(), frame.axis(), frame.u_ref());
                Some(Self::Sphere {
                    frame,
                    center,
                    radius,
                    axis,
                    u_ref,
                })
            }
            Surface::Cone { frame, half_angle } => {
                let (apex, axis, u_ref) = (frame.origin(), frame.axis(), frame.u_ref());
                Some(Self::Cone {
                    frame,
                    apex,
                    axis,
                    half_angle,
                    u_ref,
                })
            }
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

    /// The surface the chart reads: its stored frame carried, not
    /// squared again.
    fn surface(self) -> Surface<f64> {
        match self {
            Self::Sphere { frame, radius, .. } => Surface::Sphere { frame, radius },
            Self::Cone {
                frame, half_angle, ..
            } => Surface::Cone { frame, half_angle },
        }
    }

    /// The singular points, the kernel's one list of them
    /// ([`geom_brep::chart_singularity::singular_points`]), in its order,
    /// with signed zeros normalized as every point this pass mints is.
    fn singular(self) -> Vec<Point3<f64>> {
        home::singular_points(&self.surface())
            .into_iter()
            .map(|(_, p)| crate::signed_zero::plus_zero_point(p))
            .collect()
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
}

/// A point the pass reads coincident with another (a singular point, a
/// boundary vertex, a carrier's foot): Zero at the pass's band.
const STEP_SINGULARITY_POINT: &str = "step_singularity_point";

/// The pass (module docs).
pub(super) fn singularity_vertices(
    solid: &mut SolidSpec,
    file: FileBand,
    mint: &mut dyn FnMut() -> u64,
    sink: &mut Vec<StructureNormalization>,
) -> Result<(), StepImportError> {
    // The file's band, floored at the run's: a point the kernel reads as
    // one with a vertex is not cut a sub-tolerance edge away from it, and
    // one the file cannot place is not placed finer.
    let read = Read::of(file);
    // Each action makes one singular point a vertex of the face (an
    // edge's split point, or a wrap edge's tip), which no later scan
    // re-finds, so the pass ends; the bound only turns a defect of that
    // argument into a refusal.
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

/// The pass's band, as it reads it: the file's band, floored at the
/// run's ([`FileBand`]), which every on-or-off and side decision is taken
/// at and every refusal ends at.
#[derive(Clone, Copy)]
struct Read {
    file: FileBand,
}

impl Read {
    /// The pass's reading of `file`'s band.
    fn of(file: FileBand) -> Self {
        Self { file }
    }

    /// The band itself, for a kernel reading that takes one.
    fn band(&self) -> geom_core::Band {
        self.file.band()
    }

    /// Whether the finite distance `d` does not read apart at the pass's
    /// band under `name` (Zero or in band): a candidate the band keeps.
    fn not_apart(&self, name: &'static str, d: f64) -> bool {
        d.is_finite() && self.file.read(name, d) != crate::interpret::Read::Apart
    }

    /// The typed refusal of a distance `d` the band could not call,
    /// naming whose band it was: the file's where its declared ε_in is at
    /// least the run's ε, the run's otherwise.
    fn refuse(&self, cause: geom_core::Indeterminate, id: u64) -> StepImportError {
        use crate::error::SingularityBand;
        StepImportError::SingularityUndecided {
            id,
            cause,
            band: if self.file.eps_in() >= self.file.run_eps() {
                SingularityBand::File
            } else {
                SingularityBand::Run
            },
            file: self.coincidence(),
            run_eps: self.file.run_eps(),
        }
    }

    /// The file's ε_in, as the import door reads a refusal at it.
    fn coincidence(&self) -> geom_core::FileCoincidence {
        geom_core::FileCoincidence::new(self.file.eps_in())
    }

    /// The kernel's reading's failure to place a point, as this pass
    /// refuses it. A side is no distance either band decides, so its
    /// refusal names no band ([`StepImportError::SingularitySideUndecided`]).
    fn unread(&self, why: Unread<f64>, id: u64) -> StepImportError {
        match why {
            Unread::Boundary { cause, .. } => self.refuse(cause, id),
            Unread::Side { why, .. } => StepImportError::SingularitySideUndecided {
                id,
                side: why,
                file: self.coincidence(),
            },
            Unread::Carrier { .. } => foreign_carrier(id),
            Unread::Loop { .. } | Unread::SplineSide => unreadable(id),
        }
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
               carrier other than a line or a circle: the kernel's meeting reading reads \
               where a meridian meets it, but the chart-singularity normalization cuts \
               only faces bounded by lines and circles, and does not yet cut this one",
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

/// The disposition the face `fi` owes, if any (module docs): the first
/// singular point, in the kernel's order, that the kernel's reading
/// places inside an edge or inside the face. A wrap tip the file states
/// (a valence-1 vertex the face walks its one edge out and back to) owes
/// none, and refuses where the face's material does not hold it.
fn scan(solid: &SolidSpec, fi: usize, read: &Read) -> Result<Option<Action>, StepImportError> {
    let face = &solid.faces[fi];
    let Some(chart) = Chart::of(solid.surface_of(face)) else {
        return Ok(None);
    };
    let valence = valences(solid);
    let loops = loop_edges(solid, fi);
    // Each vertex's stored point: what "at a vertex" reads.
    let corners: Vec<Vec<Point3<f64>>> = face
        .loops
        .iter()
        .map(|lp| {
            lp.uses
                .iter()
                .map(|u| solid.vertices[&use_start(solid, *u)])
                .collect()
        })
        .collect();
    let views: Vec<home::FaceLoop<'_, f64>> = loops
        .iter()
        .zip(&corners)
        .map(|(edges, corners)| home::FaceLoop { edges, corners })
        .collect();
    let images = image_edges(solid, fi);
    let images: Vec<&[LoopEdge<f64>]> = images.iter().map(Vec::as_slice).collect();
    for (k, located) in home::locate(
        solid.surface_of(face),
        &views,
        &images,
        face.sense,
        read.band(),
    )
    .into_iter()
    .enumerate()
    {
        match located.lies.map_err(|why| read.unread(why, face.id))? {
            Lies::AtVertex { r#loop, edge } => {
                let v = use_start(solid, face.loops[r#loop].uses[edge]);
                if valence.get(&v) == Some(&1)
                    && walks_out_and_back(solid, fi, v)
                    && !holds(solid, fi, chart, k, read)?
                {
                    return Err(inverted(face.id));
                }
            }
            Lies::InsideEdge { r#loop, edge, t } => {
                // This pass cuts lines and circles alone.
                if !matches!(
                    loops[r#loop][edge].carrier,
                    Curve3::Line(_) | Curve3::Circle(..)
                ) {
                    return Err(foreign_carrier(face.id));
                }
                return Ok(Some(Action::InsideEdge {
                    fi,
                    edge: face.loops[r#loop].uses[edge].edge,
                    t,
                    point: crate::signed_zero::plus_zero_point(located.point),
                }));
            }
            Lies::Interior => return Ok(Some(Action::Interior { fi, k })),
            Lies::Outside => {}
        }
    }
    Ok(None)
}

/// Face `fi`'s loops as the kernel's reading takes them: each use's
/// carrier, its stored interval and its traversal, with loop-local
/// vertex tags.
fn loop_edges(solid: &SolidSpec, fi: usize) -> Vec<Vec<LoopEdge<f64>>> {
    solid.faces[fi]
        .loops
        .iter()
        .map(|lp| use_edges(solid, &lp.uses))
        .collect()
}

/// Face `fi`'s loops' chart images, loop for loop
/// ([`geom_brep::chart_image::without_reversals`]): a use followed by a
/// use of the same `EDGE_CURVE` the other way walks it straight back (a
/// spur), the reading a body's half-edges and mates give
/// (`topo::surface_boundary::chart_image`). A loop that is all spurs is
/// empty.
fn image_edges(solid: &SolidSpec, fi: usize) -> Vec<Vec<LoopEdge<f64>>> {
    solid.faces[fi]
        .loops
        .iter()
        .map(|lp| {
            let kept = geom_brep::chart_image::without_reversals(&lp.uses, |a, b| {
                a.edge == b.edge && a.forward != b.forward
            });
            use_edges(solid, &kept)
        })
        .collect()
}

/// A run of uses as the kernel's reading takes them, with run-local
/// vertex tags.
fn use_edges(solid: &SolidSpec, uses: &[EdgeUse]) -> Vec<LoopEdge<f64>> {
    let mut tags: Vec<u64> = Vec::new();
    let mut tag = |v: u64| {
        let i = tags.iter().position(|&t| t == v).unwrap_or_else(|| {
            tags.push(v);
            tags.len() - 1
        });
        u32::try_from(i).unwrap_or(u32::MAX)
    };
    uses.iter()
        .map(|u| {
            let spec = &solid.edges[&u.edge];
            let (start, end) = (use_start(solid, *u), use_end(solid, *u));
            LoopEdge::hand_built(
                spec.carrier.clone(),
                spec.t0,
                spec.t1,
                u.forward,
                tag(start),
                tag(end),
            )
        })
        .collect()
}

/// Whether face `fi`'s one loop arrives at `vertex` once and leaves it
/// once, along one edge: the tip of a wrap edge the face walks out and
/// back.
fn walks_out_and_back(solid: &SolidSpec, fi: usize, vertex: u64) -> bool {
    let [lp] = solid.faces[fi].loops.as_slice() else {
        return false;
    };
    let arrive: Vec<usize> = (0..lp.uses.len())
        .filter(|&i| use_end(solid, lp.uses[i]) == vertex)
        .collect();
    let leave: Vec<usize> = (0..lp.uses.len())
        .filter(|&i| use_start(solid, lp.uses[i]) == vertex)
        .collect();
    matches!((arrive.as_slice(), leave.as_slice()), ([a], [l]) if lp.uses[*a].edge == lp.uses[*l].edge)
}

/// Whether the singular point `k` of `chart` lies on the face's side of
/// every one of its loops, the point being off them: the kernel's
/// reading ([`geom_brep::chart_singularity::holds`]). The loops are read
/// as their chart images ([`image_edges`]), which drop an edge walked
/// out and back, so a wrap tip the file states is off them and its side
/// is read.
fn holds(
    solid: &SolidSpec,
    fi: usize,
    chart: Chart,
    k: usize,
    read: &Read,
) -> Result<bool, StepImportError> {
    let face = &solid.faces[fi];
    let surface = chart.surface();
    let Some(&(singularity, _)) = home::singular_points(&surface).get(k) else {
        return Err(unreadable(face.id));
    };
    let images = image_edges(solid, fi);
    let views: Vec<&[LoopEdge<f64>]> = images.iter().map(Vec::as_slice).collect();
    home::holds(&surface, &views, face.sense, singularity, read.band())
        .map_err(|why| read.unread(why, face.id))
}

/// The nappe face `fi`'s outer loop runs on, `±1`, read off its walk
/// ([`geom_brep::chart_singularity::nappe`]).
fn nappe(solid: &SolidSpec, fi: usize, chart: Chart, read: &Read) -> Result<f64, StepImportError> {
    let face_id = solid.faces[fi].id;
    let Ok(inverse) = ChartInverse::of(&chart.surface()) else {
        return Err(unreadable(face_id));
    };
    let points = home::singular_points(&chart.surface());
    let images = image_edges(solid, fi);
    let walk =
        home::walk_loop(&inverse, &points, &images[0], read.band()).ok_or(unreadable(face_id))?;
    match home::nappe(&walk) {
        Ok(side) => Ok(side.unit()),
        Err(why) => Err(read.unread(Unread::Side { r#loop: 0, why }, face_id)),
    }
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
        Action::Interior { fi, .. } | Action::InsideEdge { fi, .. } => fi,
    };
    let face_id = solid.faces[fi].id;
    let (entry_edges, entry_vertices) = boundary_census(solid, &[fi]);
    let found = match action {
        Action::InsideEdge { edge, t, point, .. } => {
            split_at_point(solid, edge, t, point, mint)?;
            SingularityFound::InsideEdge
        }
        Action::Interior { k, .. } => {
            interior(solid, fi, k, read, mint)?;
            SingularityFound::Interior
        }
    };
    let (edges, vertices) = boundary_census(solid, &[fi]);
    sink.push(StructureNormalization {
        face: face_id,
        kind: NormalizationKind::ChartSingularityVertex { found },
        file_census: FaceCensus {
            faces: 1,
            edges: entry_edges,
            vertices: entry_vertices,
        },
        kernel_census: FaceCensus {
            faces: 1,
            edges,
            vertices,
        },
    });
    Ok(())
}

/// A ray: a carrier walked from the parameter `from` in the direction
/// `sign`, at most `reach` of parameter.
struct Ray {
    carrier: Curve3<f64>,
    from: f64,
    sign: f64,
    reach: f64,
}

impl Ray {
    /// The span the ray walks to `delta` of parameter past its start.
    fn span(&self, delta: f64) -> Span<'_, f64> {
        let to = self.from + self.sign * delta;
        Span::new(&self.carrier, (self.from.min(to), self.from.max(to)))
    }

    /// The end of [`Self::span`] the ray starts at.
    fn start(&self) -> SpanEnd {
        if self.sign > 0.0 {
            SpanEnd::Start
        } else {
            SpanEnd::Stop
        }
    }

    /// How far along the ray `hit` on its `span` lies.
    fn walked(&self, span: &Span<'_, f64>, hit: curve_pair::Hit<f64>) -> f64 {
        (span.at(hit) - self.from) * self.sign
    }

    /// The parameter the ray is read out to on face `fi`: its own reach,
    /// or, unbounded (a cone's generator), past the farthest any boundary
    /// edge of the face lies from its start by twice the band's
    /// escalation, so its far end reads definitely off every edge.
    fn reach_on(&self, solid: &SolidSpec, edges: &[u64], read: &Read) -> f64 {
        if self.reach.is_finite() {
            return self.reach;
        }
        let o = self.carrier.eval(self.from);
        let far = edges
            .iter()
            .map(|e| {
                let spec = &solid.edges[e];
                let (_, hi) = geom::carrier_speed::speed_range(&spec.carrier);
                (spec.carrier.eval(spec.t0) - o).norm() + hi.to_meters(spec.t1 - spec.t0)
            })
            .fold(0.0, f64::max);
        let Some(speed) = geom::carrier_speed::speed_at(&self.carrier, self.from) else {
            unreachable!(
                "an unbounded ray runs on a cone's generator line (`meridian_ray`), \
                 whose speed `speed_at` reads in closed form"
            )
        };
        (far + 2.0 * read.band().escalate()) / speed
    }
}

/// Where a ray first meets the face's boundary.
enum Hit {
    /// At an existing vertex.
    Vertex(u64),
    /// Inside an edge, at its parameter `t`.
    Edge { edge: u64, t: f64 },
}

/// A landing: how far along the ray, where, and the point.
type Landing = (f64, Hit, Point3<f64>);

/// The first point past its start at which `ray` meets the boundary of
/// face `fi`. (The ray runs to the boundary point nearest its start,
/// `wrap_azimuth`, so it meets the boundary before any other singular
/// point of the surface.)
///
/// Each edge is read against the ray by the kernel's one meeting reader
/// ([`curve_pair::meet`]) at the pass's band. The landings it decides,
/// a refused reading's included, settle the nearest; each edge whose
/// reading refused is then read once more on the ray from its start to
/// that landing. A refusal there is the pass's; one past it is no
/// question the pass asks. That span is fixed before the second reads:
/// one of them may decide a nearer landing, and a refusal lying between
/// the two still refuses, which errs toward a refusal, never a landing.
fn first_hit(
    solid: &SolidSpec,
    fi: usize,
    ray: &Ray,
    read: &Read,
) -> Result<(Hit, Point3<f64>), StepImportError> {
    let face = &solid.faces[fi];
    let mut edges: Vec<u64> = face
        .loops
        .iter()
        .flat_map(|lp| lp.uses.iter().map(|u| u.edge))
        .collect();
    edges.sort_unstable();
    edges.dedup();
    let reach = ray.reach_on(solid, &edges, read);
    let span = ray.span(reach);
    let mut best: Option<Landing> = None;
    let offer = |landings: Vec<Landing>, best: &mut Option<Landing>| {
        for (delta, hit, p) in landings {
            if best.as_ref().is_none_or(|(b, ..)| delta < *b) {
                *best = Some((delta, hit, p));
            }
        }
    };
    let mut refused = Vec::new();
    for &edge in &edges {
        let spec = &solid.edges[&edge];
        let other = Span::new(&spec.carrier, (spec.t0, spec.t1));
        let (met, mut undecided) = match curve_pair::meet(&span, &other, read.band()) {
            Ok(met) => (met, false),
            Err(why) => (
                curve_pair::Meetings {
                    shared: why.shared,
                    points: why.found,
                },
                true,
            ),
        };
        let (decided, end) = landings(solid, ray, &span, edge, &met, read, face.id);
        undecided |= end.is_some();
        offer(decided, &mut best);
        if undecided {
            refused.push(edge);
        }
    }
    let near = ray.span(best.as_ref().map_or(reach, |b| b.0));
    let mut reread = Vec::with_capacity(refused.len());
    for edge in refused {
        let spec = &solid.edges[&edge];
        let other = Span::new(&spec.carrier, (spec.t0, spec.t1));
        let met = curve_pair::meet(&near, &other, read.band())
            .map_err(|why| meet_refused(why.fault, read, face.id))?;
        let (decided, end) = landings(solid, ray, &near, edge, &met, read, face.id);
        if let Some(refusal) = end {
            return Err(refusal);
        }
        reread.push(decided);
    }
    for decided in reread {
        offer(decided, &mut best);
    }
    best.map(|(_, hit, p)| (hit, p))
        .ok_or(StepImportError::Topology {
            id: face.id,
            what: "internal: a wrap edge's meridian minted inside a face meets none of its \
                   boundary",
        })
}

/// What `met`, the meetings of the ray's `span` with `edge`, offer as
/// landings: each meeting at the vertex or inside the edge, where it sits
/// on the edge; where the edge runs along the ray, each of its ends the
/// ray reads on itself, the run's first point being one of them. A
/// meeting at the ray's start is where it leaves, no landing. Beside
/// them, the refusal of an end the ray's span could not read, which, as
/// a meeting's refusal does, counts only where it is met first.
fn landings(
    solid: &SolidSpec,
    ray: &Ray,
    span: &Span<'_, f64>,
    edge: u64,
    met: &curve_pair::Meetings<f64>,
    read: &Read,
    id: u64,
) -> (Vec<Landing>, Option<StepImportError>) {
    let spec = &solid.edges[&edge];
    let mut out = Vec::new();
    let mut undecided = None;
    if met.shared.is_some() {
        for v in [spec.start, spec.end] {
            let p = solid.vertices[&v];
            let at = match span.read(p, ray.from, read.band()) {
                Ok(curve_pair::Reading::On { t, margin }) => curve_pair::Hit::On { t, margin },
                Ok(curve_pair::Reading::End(end)) => curve_pair::Hit::End(end),
                Ok(curve_pair::Reading::Off(_)) => continue,
                Err(why) => {
                    undecided = Some(span_refused(why, read, id));
                    continue;
                }
            };
            if at != curve_pair::Hit::End(ray.start()) {
                out.push((ray.walked(span, at), Hit::Vertex(v), p));
            }
        }
        return (out, undecided);
    }
    for m in &met.points {
        if m.on_a == curve_pair::Hit::End(ray.start()) {
            continue;
        }
        let (hit, p) = match m.on_b {
            curve_pair::Hit::End(SpanEnd::Start) => {
                (Hit::Vertex(spec.start), solid.vertices[&spec.start])
            }
            curve_pair::Hit::End(SpanEnd::Stop) => {
                (Hit::Vertex(spec.end), solid.vertices[&spec.end])
            }
            curve_pair::Hit::On { t, .. } => (Hit::Edge { edge, t }, m.point),
        };
        out.push((ray.walked(span, m.on_a), hit, p));
    }
    (out, undecided)
}

/// The pass's refusal of a meeting the kernel's reader did not decide,
/// by its group ([`curve_pair::MeetKind`]): a decision in the band, or a
/// meeting in it, is the distance the pass could not call, named by
/// whose band it was ([`Read::refuse`]); a reading that did not run to an
/// answer ends as its own standing does
/// ([`StepImportError::SingularityMeetUnread`]); a kernel fault names
/// what faulted.
fn meet_refused(fault: curve_pair::MeetFault, read: &Read, id: u64) -> StepImportError {
    match fault.kind() {
        curve_pair::MeetKind::Escalated(cause) | curve_pair::MeetKind::Touch(cause) => {
            read.refuse(cause, id)
        }
        curve_pair::MeetKind::Unread(why) => StepImportError::SingularityMeetUnread {
            id,
            why,
            file: read.coincidence(),
        },
        curve_pair::MeetKind::Fault { what } => StepImportError::Topology { id, what },
    }
}

/// [`meet_refused`] for a point read on the ray's span: its escalation,
/// or the piece walk's refusal where the walk could not run.
fn span_refused(why: curve_pair::SpanRefusal, read: &Read, id: u64) -> StepImportError {
    match why.escalation() {
        Ok(cause) => read.refuse(cause, id),
        Err(walk) => meet_refused(curve_pair::MeetFault::Unwalked(walk), read, id),
    }
}

/// The wrap edge's ray from singular point `k` of `chart`: the
/// surface's meridian at the azimuth `d`, walked into the face (`nappe`
/// is the sign of the cone face's chart `v`).
fn meridian_ray(chart: Chart, k: usize, d: Vec3<f64>, nappe: f64) -> Ray {
    match chart {
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
                geom::Line {
                    origin: apex,
                    dir: axis,
                },
                half_angle,
                d,
            )),
            from: 0.0,
            sign: nappe,
            reach: f64::INFINITY,
        },
    }
}

/// Requires every boundary carrier of face `fi` to be a line or a
/// circle, the boundaries the pass cuts.
fn require_read_carriers(solid: &SolidSpec, fi: usize) -> Result<(), StepImportError> {
    let face = &solid.faces[fi];
    for lp in &face.loops {
        for u in &lp.uses {
            if !matches!(
                solid.edges[&u.edge].carrier,
                Curve3::Line(_) | Curve3::Circle(..)
            ) {
                return Err(foreign_carrier(face.id));
            }
        }
    }
    Ok(())
}

/// Resolves a hit to a vertex of the boundary, splitting the edge it
/// lies inside.
fn land(
    solid: &mut SolidSpec,
    hit: Hit,
    p: Point3<f64>,
    mint: &mut dyn FnMut() -> u64,
) -> Result<u64, StepImportError> {
    match hit {
        Hit::Vertex(v) => Ok(v),
        Hit::Edge { edge, t } => split_at_point(solid, edge, t, p, mint),
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
            curve: None,
        },
    );
    Ok((id, forward))
}

/// **Interior**: mints the singular point `k` as a vertex with one wrap
/// edge to it, and splices the edge into the face's loop, walked out
/// from where it lands on the boundary and back.
fn interior(
    solid: &mut SolidSpec,
    fi: usize,
    k: usize,
    read: &Read,
    mint: &mut dyn FnMut() -> u64,
) -> Result<(), StepImportError> {
    require_read_carriers(solid, fi)?;
    let Some(chart) = Chart::of(solid.surface(fi)) else {
        unreachable!("the scan reads only sphere and cone faces")
    };
    let x = chart.singular()[k];
    let nappe = match chart {
        Chart::Cone { .. } => nappe(solid, fi, chart, read)?,
        Chart::Sphere { .. } => 1.0,
    };
    let d = wrap_azimuth(solid, fi, chart, x, read)? * nappe;
    let pole = mint();
    solid.vertices.insert(pole, x);
    let ray = meridian_ray(chart, k, d, nappe);
    let (hit, p) = first_hit(solid, fi, &ray, read)?;
    let far = land(solid, hit, p, mint)?;
    let spoke = mint_spoke(solid, ray.carrier, ray.sign, pole, far, mint)?;
    splice(solid, fi, spoke)
}

/// The azimuth of the wrap edge to the singular point `x` of `chart`
/// on face `fi`: that of the face's boundary point nearest `x`. Answers
/// the unit radial direction of the chosen point.
///
/// Along a meridian from `x` (a sphere's meridian from its pole, a
/// cone's generator from its apex) the distance from `x` only grows, so
/// the meridian through the nearest boundary point meets the boundary
/// there first, and meets no other singular point before it.
///
/// The choice reads the boundary's geometry alone, not how the file
/// splits it into edges. Each edge offers the nearest point of its own
/// span: a line's foot, a circle's closest point, or an end. An arc
/// every point of which lies at one distance from `x` (a rim coaxial
/// with the surface) offers its point at the `u_ref` azimuth where its
/// span holds one, and its ends otherwise. Offered points within the
/// band of the nearest one tie, and the tie goes to the least turn from
/// the surface's `u_ref`, either way round, the positive turn first.
fn wrap_azimuth(
    solid: &SolidSpec,
    fi: usize,
    chart: Chart,
    x: Point3<f64>,
    read: &Read,
) -> Result<Vec3<f64>, StepImportError> {
    let face_id = solid.faces[fi].id;
    let (axis, origin, u_ref) = (chart.axis(), chart.origin(), chart.u_ref());
    let frame = match chart.surface() {
        Surface::Sphere { frame, .. } | Surface::Cone { frame, .. } => frame,
        _ => unreachable!("a chart is a sphere's or a cone's"),
    };
    let radial = |p: Point3<f64>| geom::Line { origin, dir: axis }.offset(p);
    // How far round the axis `p` lies from `u_ref`: the turn's size,
    // then its sign, so a point a rounding either side of `u_ref` reads
    // as at it.
    let turn = |p: Point3<f64>| {
        let a = frame.azimuth(p).near(0.0);
        (a.abs(), a)
    };
    let mut offered: Vec<Point3<f64>> = Vec::new();
    let mut edges: Vec<u64> = solid.faces[fi]
        .loops
        .iter()
        .flat_map(|lp| lp.uses.iter().map(|u| u.edge))
        .collect();
    edges.sort_unstable();
    edges.dedup();
    for e in edges {
        let spec = &solid.edges[&e];
        offered.push(spec.carrier.eval(spec.t0));
        offered.push(spec.carrier.eval(spec.t1));
        // Whether the point `q` of the carrier lies inside the edge's span.
        let spans = |q: Point3<f64>| {
            let mid = geom::mid_param(spec.t0, spec.t1);
            spec.carrier
                .azimuth(q)
                .is_some_and(|a| (a.near(mid) - mid).abs() < 0.5 * (spec.t1 - spec.t0).abs())
        };
        match &spec.carrier {
            Curve3::Line(geom::Line { origin: o, dir }) => {
                let t = (x - *o).dot(*dir) / dir.dot(*dir);
                if t > spec.t0.min(spec.t1) && t < spec.t0.max(spec.t1) {
                    offered.push(spec.carrier.eval(t));
                }
            }
            Curve3::Circle(c) => {
                let (o, r, n) = (c.frame().origin(), c.radius(), c.frame().axis());
                // `x`'s direction in the circle's plane: the nearest point
                // lies along it, and where it has no length every point of
                // the circle is at one distance (a circle about the axis),
                // which offers its point in `u_ref`'s direction instead.
                let w = (x - o) - n * (x - o).dot(n);
                // |p − x|² runs over A ± 2r|w| round the circle, so the
                // distance itself spans the difference of their roots.
                let (a, b) = ((x - o).norm_squared() + r.powi(2), 2.0 * r * w.norm());
                let spread = 2.0 * b / ((a + b).sqrt() + (a - b).max(0.0).sqrt());
                let toward = if read.not_apart(STEP_SINGULARITY_POINT, spread) {
                    u_ref - n * u_ref.dot(n)
                } else {
                    w
                };
                if !read.not_apart(STEP_SINGULARITY_POINT, toward.norm()) {
                    let q = o + toward.normalize() * r;
                    if spans(q) {
                        offered.push(q);
                    }
                }
            }
            _ => return Err(foreign_carrier(face_id)),
        }
    }
    let nearest = offered
        .iter()
        .map(|p| (*p - x).norm())
        .fold(f64::INFINITY, f64::min);
    let chosen = offered
        .into_iter()
        .filter(|p| read.not_apart(STEP_SINGULARITY_POINT, (*p - x).norm() - nearest))
        .filter(|p| !read.not_apart(STEP_SINGULARITY_POINT, radial(*p).norm()))
        .min_by(|p, q| {
            let (a, b) = (turn(*p), turn(*q));
            a.0.total_cmp(&b.0).then(a.1.total_cmp(&b.1))
        })
        .ok_or(unreadable(face_id))?;
    Ok(radial(chosen).normalize())
}

/// Splices the wrap edge `spoke` (an edge, and whether it runs from the
/// pole) into face `fi`'s one loop, as a cut whose far end is attached
/// where the loop leaves it and whose pole end is free
/// ([`surgery::relink`]): the loop arrives at the far end, walks the
/// edge out to the pole and back, and goes on.
fn splice(solid: &mut SolidSpec, fi: usize, spoke: (u64, bool)) -> Result<(), StepImportError> {
    let face_id = solid.faces[fi].id;
    let internal = |what| StepImportError::Topology { id: face_id, what };
    let [lp] = solid.faces[fi].loops.as_slice() else {
        return Err(internal(
            "internal: a chart singularity reached on a face with rings, which the face \
             gate admits on no sphere or cone",
        ));
    };
    let loops = vec![lp.uses.clone()];
    let (edge, from_pole) = spoke;
    let first = EdgeUse {
        edge,
        forward: from_pole,
    };
    let far = use_end(solid, first);
    let leaving: Vec<usize> = (0..loops[0].len())
        .filter(|&i| use_start(solid, loops[0][i]) == far)
        .collect();
    let at = match leaving.as_slice() {
        [i] => (0, *i),
        // The wrap edge runs to the boundary point nearest the pole
        // (`wrap_azimuth`), which it meets before anything farther,
        // the other pole included.
        [] => {
            return Err(internal(
                "internal: a wrap edge ends off its face's loop, though it runs to the \
                 loop's point nearest its tip",
            ));
        }
        _ => {
            return Err(internal(
                "a wrap edge ends at a vertex the face's loop passes more than once; \
                 where the loop walks it out and back has no one reading",
            ));
        }
    };
    let cut = CutLine {
        first,
        ends: [None, Some(at)],
    };
    let cycles = surgery::relink(solid, &loops, &[cut], &[]).map_err(|why| match why {
        Unlinked::Branch => internal("internal: a wrap tip with other than one edge"),
        Unlinked::Open => internal("internal: a wrap edge's splice does not close"),
    })?;
    let [uses] = <[Vec<EdgeUse>; 1]>::try_from(cycles)
        .map_err(|_| internal("internal: a wrap edge's splice is not one cycle"))?;
    solid.faces[fi].loops = vec![LoopSpec { outer: true, uses }];
    Ok(())
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod side_through_a_pole {
    use std::collections::BTreeMap;

    use geom::{Curve3, Surface};
    use geom_core::{Point3, Vec3};

    use super::{Chart, Read, holds};
    use crate::entities::{EdgeSpec, EdgeUse, FaceSpec, LoopSpec, SolidSpec};

    /// **The kernel's walk's claim, pinned**: a loop passing THROUGH one pole
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
        let rim = Curve3::circle(
            Point3::new(0.0, 0.0, b.sin()),
            Vec3::new(0.0, 0.0, 1.0),
            b.cos(),
            Vec3::new(1.0, 0.0, 0.0),
        );
        // θ runs from +x up through the north pole (θ = π/2) to −x.
        let meridian = Curve3::circle(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            1.0,
            Vec3::new(1.0, 0.0, 0.0),
        );
        let edge = |start, end, carrier: &Curve3<f64>, t0, t1| EdgeSpec {
            start,
            end,
            carrier: carrier.clone(),
            t0,
            t1,
            reversed: false,
            curve: None,
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
            frame: geom::Frame::new(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            radius: 1.0,
        };
        let use_ = |edge, forward| EdgeUse { edge, forward };
        let read = Read::of(crate::interpret::FileBand::with_run(1e-9, 1e-9, 10.0).unwrap());
        let chart = Chart::of(&surface).unwrap();
        for (sense, south_inside) in [(true, false), (false, true)] {
            let solid = SolidSpec {
                id: 100,
                faces: vec![FaceSpec {
                    id: 101,
                    record: 900,
                    sense,
                    loops: vec![LoopSpec {
                        outer: true,
                        uses: vec![use_(11, true), use_(13, false), use_(12, false)],
                    }],
                    band: false,
                }],
                edges: edges(),
                vertices: vertices.clone(),
                surfaces: crate::entities::records_of([(900, surface.clone())]),
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

/// **The pass reports a distance in band ahead of an edge that does not
/// settle**, the order `geom_brep::chart_singularity::place_on_boundary`
/// ranks them in, in either loop order: the north pole of a unit sphere
/// lies `5ε` inside a circle's edge (the plane `x = 5ε`) and `3ε` from a
/// grazing ellipse whose refinement does not settle. At the default band
/// the pass refuses the distance (`SingularityUndecided`) in both
/// orders; at a coarser one the first edge places the pole. Refusing the ellipse as a
/// foreign carrier at the default band would hide a placement a coarser
/// band reads definite.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod an_undecided_distance_ahead_of_an_unsettled_edge {
    use std::collections::BTreeMap;

    use geom::{Curve3, Surface};
    use geom_core::{Point3, Vec3};

    use super::{Action, Read, scan};
    use crate::entities::{EdgeSpec, EdgeUse, FaceSpec, LoopSpec, SolidSpec};
    use crate::error::StepImportError;

    fn read(eps: f64, escalate: f64) -> Read {
        Read::of(crate::interpret::FileBand::with_run(eps, eps, escalate / eps).unwrap())
    }

    fn solid(rim_first: bool) -> SolidSpec {
        let d = 5e-9_f64;
        let rim = Curve3::circle(
            Point3::new(d, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            (1.0 - d * d).sqrt(),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let (delta, major) = (3e-9, 1e7);
        let graze = Curve3::Ellipse(geom::Ellipse::new(
            geom::Frame::new(
                Point3::new(0.0, 0.0, 1.0 + delta - major),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ),
            major,
            1.0,
        ));
        let edges = BTreeMap::from([
            (
                11,
                EdgeSpec {
                    start: 1,
                    end: 1,
                    carrier: rim.clone(),
                    t0: 0.0,
                    t1: core::f64::consts::TAU,
                    reversed: false,
                    curve: None,
                },
            ),
            (
                12,
                EdgeSpec {
                    start: 2,
                    end: 3,
                    carrier: graze.clone(),
                    t0: -0.5,
                    t1: 0.5,
                    reversed: false,
                    curve: None,
                },
            ),
        ]);
        let vertices = BTreeMap::from([
            (1, rim.eval(0.0)),
            (2, graze.eval(-0.5)),
            (3, graze.eval(0.5)),
        ]);
        let lp = |edge| LoopSpec {
            outer: false,
            uses: vec![EdgeUse {
                edge,
                forward: true,
            }],
        };
        let loops = if rim_first {
            vec![lp(11), lp(12)]
        } else {
            vec![lp(12), lp(11)]
        };
        SolidSpec {
            id: 100,
            faces: vec![FaceSpec {
                id: 101,
                record: 900,
                sense: true,
                loops,
                band: false,
            }],
            edges,
            vertices,
            surfaces: crate::entities::records_of([(
                900,
                Surface::Sphere {
                    frame: geom::Frame::new(
                        Point3::new(0.0, 0.0, 0.0),
                        Vec3::new(0.0, 0.0, 1.0),
                        Vec3::new(1.0, 0.0, 0.0),
                    ),
                    radius: 1.0,
                },
            )]),
            band_seams: Default::default(),
        }
    }

    /// What the pass did, for a message.
    fn shown(got: &Result<Option<Action>, StepImportError>) -> String {
        match got {
            Ok(Some(Action::InsideEdge { edge, t, .. })) => format!("inside edge {edge} at {t}"),
            Ok(Some(_)) => "another disposition".to_owned(),
            Ok(None) => "none".to_owned(),
            Err(e) => format!("{e:?}"),
        }
    }

    #[test]
    fn the_pass_refuses_the_distance_and_a_coarser_band_cuts_the_edge() {
        for rim_first in [true, false] {
            let got = scan(&solid(rim_first), 0, &read(1e-9, 1e-8));
            assert!(
                matches!(got, Err(StepImportError::SingularityUndecided { .. })),
                "rim first {rim_first}, default band: {}",
                shown(&got)
            );
            // At the coarser band the first edge to place the pole is
            // its answer: the circle, cut there; or the ellipse, which
            // settles on the pole and whose carrier the pass reads no
            // crossing of.
            let got = scan(&solid(rim_first), 0, &read(1e-8, 1e-7));
            let placed = if rim_first {
                matches!(got, Ok(Some(Action::InsideEdge { edge: 11, .. })))
            } else {
                matches!(got, Err(StepImportError::Topology { id: 101, .. }))
            };
            assert!(
                placed,
                "rim first {rim_first}, coarser band: {}",
                shown(&got)
            );
        }
    }
}

/// **Where a minted meridian ends is the kernel's meeting reading at the
/// pass's band** ([`geom_brep::curve_pair::meet`]), not a reading of the
/// pass's own: the landing beside a boundary vertex is decided at the
/// file's band, and a crossing at a grazing angle is a crossing.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod meetings_are_the_kernels {
    use geom::{Curve3, Surface};
    use geom_core::{Point3, Vec3};

    use super::{Chart, Hit, Read, first_hit, meridian_ray};
    use crate::entities::{EdgeSpec, EdgeUse, FaceSpec, LoopSpec, SolidSpec};
    use crate::error::{SingularityBand, StepImportError};

    /// One face on `surface`, each of `edges` a loop of its own. These
    /// single-edge loops pin the meeting reader, not a face the pass is
    /// handed: `first_hit` reads the face's edges and nothing of its
    /// loops' structure.
    fn face(
        surface: Surface<f64>,
        edges: Vec<(u64, EdgeSpec)>,
        vertices: Vec<(u64, Point3<f64>)>,
    ) -> SolidSpec {
        let loops = edges
            .iter()
            .map(|&(edge, _)| LoopSpec {
                outer: false,
                uses: vec![EdgeUse {
                    edge,
                    forward: true,
                }],
            })
            .collect();
        SolidSpec {
            id: 100,
            faces: vec![FaceSpec {
                id: 101,
                record: 900,
                sense: true,
                loops,
                band: false,
            }],
            edges: edges.into_iter().collect(),
            vertices: vertices.into_iter().collect(),
            surfaces: crate::entities::records_of([(900, surface)]),
            band_seams: Default::default(),
        }
    }

    /// A closed circle edge `id` from and to the vertex `v` at its `t = 0`.
    fn closed(id: u64, v: u64, carrier: Curve3<f64>) -> ((u64, EdgeSpec), (u64, Point3<f64>)) {
        let at = carrier.eval(0.0);
        let spec = EdgeSpec {
            start: v,
            end: v,
            carrier,
            t0: 0.0,
            t1: core::f64::consts::TAU,
            reversed: false,
            curve: None,
        };
        ((id, spec), (v, at))
    }

    fn shown(got: &Result<(Hit, Point3<f64>), StepImportError>) -> String {
        match got {
            Ok((Hit::Vertex(v), p)) => format!("vertex {v} at {p:?}"),
            Ok((Hit::Edge { edge, t }, p)) => format!("edge {edge} at t {t} ({p:?})"),
            Err(e) => format!("{e:?}"),
        }
    }

    /// The north pole's meridian at azimuth `x` meets the equator at
    /// `(1, 0, 0)`, `phi` of arc from the equator edge's vertex: a vertex
    /// landing at the band's zero, refused in the band, an edge split past
    /// its escalation, all at the FILE's band where its ε_in is coarser
    /// than the run's.
    #[test]
    fn a_landing_beside_a_vertex_is_decided_at_the_files_band() {
        let (z, x) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        let sphere = Surface::Sphere {
            frame: geom::Frame::new(Point3::new(0.0, 0.0, 0.0), z, x),
            radius: 1.0,
        };
        let Some(chart) = Chart::of(&sphere) else {
            unreachable!("a sphere has a chart")
        };
        let ray = meridian_ray(chart, 0, x, 1.0);
        let at = |phi: f64, eps_in: f64| {
            let (edge, vertex) = closed(
                11,
                1,
                Curve3::circle(
                    Point3::new(0.0, 0.0, 0.0),
                    z,
                    1.0,
                    Vec3::new(phi.cos(), phi.sin(), 0.0),
                ),
            );
            let solid = face(sphere.clone(), vec![edge], vec![vertex]);
            let read = Read::of(crate::interpret::FileBand::with_run(eps_in, 1e-9, 10.0).unwrap());
            first_hit(&solid, 0, &ray, &read)
        };
        // ε_in 1e-6 over a run of 1e-9: the band (1e-6, 1e-5).
        let got = at(3e-7, 1e-6);
        assert!(
            matches!(got, Ok((Hit::Vertex(1), _))),
            "phi 3e-7 at the file's zero: {}",
            shown(&got)
        );
        let got = at(3e-6, 1e-6);
        assert!(
            matches!(
                got,
                Err(StepImportError::SingularityUndecided {
                    id: 101,
                    band: SingularityBand::File,
                    ..
                })
            ),
            "phi 3e-6 in the file's band: {}",
            shown(&got)
        );
        let got = at(3e-5, 1e-6);
        let Ok((Hit::Edge { edge: 11, t }, p)) = got else {
            panic!("phi 3e-5 past the file's band: {}", shown(&got))
        };
        assert!(
            (t - (core::f64::consts::TAU - 3e-5)).abs() < 1e-12
                && (p - Point3::new(1.0, 0.0, 0.0)).norm() < 1e-12,
            "phi 3e-5: the split lands at the meridian's crossing, t {t}, p {p:?}"
        );
        // The same landing at the run's own band, the file declaring no
        // coarser: past its escalation, so the edge splits.
        let got = at(3e-6, 1e-9);
        assert!(
            matches!(got, Ok((Hit::Edge { edge: 11, .. }, _))),
            "phi 3e-6 at the run's band: {}",
            shown(&got)
        );
    }

    /// A cone's generator ray from the apex crosses a boundary line at
    /// slant `1` at a grazing angle (`sin² = 1e-14`), before a rim circle
    /// at slant `2`: the line is where the ray first meets the boundary.
    /// A reading that takes lines this near parallel for no crossing lands
    /// on the rim.
    #[test]
    fn a_grazing_crossing_is_a_crossing() {
        let (z, x, y) = (
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        let half_angle = core::f64::consts::FRAC_PI_4;
        let cone = Surface::Cone {
            frame: geom::Frame::new(Point3::new(0.0, 0.0, 0.0), z, x),
            half_angle,
        };
        let Some(chart) = Chart::of(&cone) else {
            unreachable!("a cone has a chart")
        };
        let ray = meridian_ray(chart, 0, x, 1.0);
        let crossing = ray.carrier.eval(1.0);
        let beta: f64 = 1e-7;
        let Curve3::Line(generator) = ray.carrier else {
            unreachable!("a cone's ray is its generator line")
        };
        let g = generator.dir * beta.cos() + y * beta.sin();
        let graze = Curve3::line(crossing - g * 0.5, g);
        let (s, h) = half_angle.sin_cos();
        let (rim, rim_vertex) = closed(
            12,
            3,
            Curve3::circle(Point3::new(0.0, 0.0, 2.0 * h), z, 2.0 * s, -x),
        );
        let solid = face(
            cone,
            vec![
                (
                    11,
                    EdgeSpec {
                        start: 1,
                        end: 2,
                        carrier: graze.clone(),
                        t0: 0.0,
                        t1: 1.0,
                        reversed: false,
                        curve: None,
                    },
                ),
                rim,
            ],
            vec![(1, graze.eval(0.0)), (2, graze.eval(1.0)), rim_vertex],
        );
        let read = Read::of(crate::interpret::FileBand::with_run(1e-9, 1e-9, 10.0).unwrap());
        let got = first_hit(&solid, 0, &ray, &read);
        let Ok((Hit::Edge { edge: 11, t }, p)) = got else {
            panic!("the grazing line is met first: {}", shown(&got))
        };
        assert!(
            (t - 0.5).abs() < 1e-6 && (p - crossing).norm() < 1e-9,
            "the grazing line is met at the crossing: t {t}, p {p:?}"
        );
    }

    fn unit_sphere() -> Surface<f64> {
        Surface::Sphere {
            frame: geom::Frame::new(
                Point3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            radius: 1.0,
        }
    }

    fn read_at(eps: f64) -> Read {
        Read::of(crate::interpret::FileBand::with_run(eps, eps, 10.0).unwrap())
    }

    /// **A refusal past the first landing is no question the pass asks.**
    /// The north pole's meridian meets the equator (edge 11) inside it,
    /// and a latitude circle (edge 12) whose crossing lies `3ε` from its
    /// vertex, in band. Below the equator the latitude's refusal lies
    /// past the landing and the ray splits the equator; above it, it is
    /// met first and refuses.
    #[test]
    fn a_refusal_counts_only_where_it_is_met_first() {
        let (z, x) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        let sphere = unit_sphere();
        let Some(chart) = Chart::of(&sphere) else {
            unreachable!("a sphere has a chart")
        };
        let ray = meridian_ray(chart, 0, x, 1.0);
        for (height, met_first) in [(-0.5_f64, false), (0.5, true)] {
            let r = (1.0 - height * height).sqrt();
            let phi = 3e-9 / r;
            let (equator, v1) = closed(
                11,
                1,
                Curve3::circle(
                    Point3::new(0.0, 0.0, 0.0),
                    z,
                    1.0,
                    Vec3::new(1.0_f64.cos(), 1.0_f64.sin(), 0.0),
                ),
            );
            let (latitude, v2) = closed(
                12,
                2,
                Curve3::circle(
                    Point3::new(0.0, 0.0, height),
                    z,
                    r,
                    Vec3::new(phi.cos(), phi.sin(), 0.0),
                ),
            );
            let solid = face(sphere.clone(), vec![equator, latitude], vec![v1, v2]);
            let got = first_hit(&solid, 0, &ray, &read_at(1e-9));
            if met_first {
                assert!(
                    matches!(
                        got,
                        Err(StepImportError::SingularityUndecided { id: 101, .. })
                    ),
                    "latitude {height}, met first: {}",
                    shown(&got)
                );
            } else {
                assert!(
                    matches!(got, Ok((Hit::Edge { edge: 11, .. }, _))),
                    "latitude {height}, past the equator: {}",
                    shown(&got)
                );
            }
        }
    }

    /// **A cone's generator is read out to the farthest the boundary
    /// reaches.** Its one boundary edge, a short line, crosses it at slant
    /// `2`; a ray read short of that meets nothing.
    #[test]
    fn a_cone_ray_reads_as_far_as_the_boundary_reaches() {
        let (x, y, z) = (
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        );
        let cone = Surface::Cone {
            frame: geom::Frame::new(Point3::new(0.0, 0.0, 0.0), z, x),
            half_angle: core::f64::consts::FRAC_PI_4,
        };
        let Some(chart) = Chart::of(&cone) else {
            unreachable!("a cone has a chart")
        };
        let ray = meridian_ray(chart, 0, x, 1.0);
        let crossing = ray.carrier.eval(2.0);
        let line = Curve3::line(crossing - y * 1e-3, y * 2e-3);
        let solid = face(
            cone,
            vec![(
                11,
                EdgeSpec {
                    start: 1,
                    end: 2,
                    carrier: line.clone(),
                    t0: 0.0,
                    t1: 1.0,
                    reversed: false,
                    curve: None,
                },
            )],
            vec![(1, line.eval(0.0)), (2, line.eval(1.0))],
        );
        let got = first_hit(&solid, 0, &ray, &read_at(1e-9));
        let Ok((Hit::Edge { edge: 11, t }, _)) = got else {
            panic!("the line at slant 2: {}", shown(&got))
        };
        assert!((t - 0.5).abs() < 1e-9, "met at the line's middle: t {t}");
    }

    /// **A boundary arc along the ray is met at its first end.** The arc
    /// lies on the north pole's meridian from polar angle `1` on: the ray
    /// lands at that end's vertex, whether the arc's far end lies well
    /// inside the ray or `3ε` past its stop at the south pole, where the
    /// first reading of the shared run refuses and the second, out to the
    /// landing, does not.
    #[test]
    fn a_boundary_arc_along_the_ray_is_met_at_its_first_end() {
        let (z, x) = (Vec3::new(0.0, 0.0, 1.0), Vec3::new(1.0, 0.0, 0.0));
        let sphere = unit_sphere();
        let Some(chart) = Chart::of(&sphere) else {
            unreachable!("a sphere has a chart")
        };
        let ray = meridian_ray(chart, 0, x, 1.0);
        let meridian = geom::pole_form::sphere_meridian(Point3::new(0.0, 0.0, 0.0), 1.0, z, x);
        for stop in [1.5, core::f64::consts::PI + 3e-9] {
            let solid = face(
                sphere.clone(),
                vec![(
                    11,
                    EdgeSpec {
                        start: 1,
                        end: 2,
                        carrier: meridian.clone(),
                        t0: 1.0,
                        t1: stop,
                        reversed: false,
                        curve: None,
                    },
                )],
                vec![(1, meridian.eval(1.0)), (2, meridian.eval(stop))],
            );
            let got = first_hit(&solid, 0, &ray, &read_at(1e-9));
            assert!(
                matches!(got, Ok((Hit::Vertex(1), _))),
                "an arc along the ray to {stop}: {}",
                shown(&got)
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::panic)]
mod a_wrap_edge_landing_where_the_loop_passes_twice {
    use std::collections::BTreeMap;

    use geom::{Curve3, Surface};
    use geom_core::{Point3, Vec3};

    use crate::entities::{EdgeSpec, EdgeUse, FaceSpec, LoopSpec, SolidSpec};
    use crate::error::StepImportError;

    /// **A wrap edge landing at a vertex the loop passes twice refuses
    /// typed.** A unit sphere's cap about an axis tilted 0.2 from the
    /// pole, of polar radius 0.6: its rim's point nearest the north pole
    /// is its vertex `P`, on the `+x` meridian. A spur leaves `P` along
    /// the parallel through it, inside the cap, every point of it as far
    /// from the pole as `P`; the tie goes to `P`, the least turn from
    /// `u_ref`, and the loop passes `P` twice (round the rim, out along
    /// the spur), so where it walks the wrap edge out and back has no
    /// one reading.
    #[test]
    fn a_wrap_edge_landing_where_the_loop_passes_twice_refuses_typed() {
        let (tau, beta) = (0.2_f64, 0.6_f64);
        let c = Vec3::new(-tau.sin(), 0.0, tau.cos());
        let theta = beta - tau;
        let p = Point3::new(theta.sin(), 0.0, theta.cos());
        let centre = Point3::new(0.0, 0.0, 0.0) + c * beta.cos();
        let rim = Curve3::circle(centre, c, beta.sin(), (p - centre) * (1.0 / beta.sin()));
        let parallel = Curve3::circle(
            Point3::new(0.0, 0.0, theta.cos()),
            Vec3::new(0.0, 0.0, 1.0),
            theta.sin(),
            Vec3::new(1.0, 0.0, 0.0),
        );
        let edge = |start, end, carrier: &Curve3<f64>, t1| EdgeSpec {
            start,
            end,
            carrier: carrier.clone(),
            t0: 0.0,
            t1,
            reversed: false,
            curve: None,
        };
        let use_ = |edge, forward| EdgeUse { edge, forward };
        let mut solid = SolidSpec {
            id: 100,
            faces: vec![FaceSpec {
                id: 101,
                record: 900,
                sense: true,
                loops: vec![LoopSpec {
                    outer: true,
                    uses: vec![use_(10, true), use_(11, true), use_(11, false)],
                }],
                band: false,
            }],
            edges: BTreeMap::from([
                (10, edge(1, 1, &rim, core::f64::consts::TAU)),
                (11, edge(1, 2, &parallel, 0.3)),
            ]),
            vertices: BTreeMap::from([(1, p), (2, parallel.eval(0.3))]),
            surfaces: crate::entities::records_of([(
                900,
                Surface::Sphere {
                    frame: geom::Frame::new(
                        Point3::new(0.0, 0.0, 0.0),
                        Vec3::new(0.0, 0.0, 1.0),
                        Vec3::new(1.0, 0.0, 0.0),
                    ),
                    radius: 1.0,
                },
            )]),
            band_seams: Default::default(),
        };
        let file = crate::interpret::FileBand::with_run(1e-9, 1e-9, 10.0).unwrap();
        let mut next = 1000;
        let mut mint = || {
            next += 1;
            next
        };
        match super::singularity_vertices(&mut solid, file, &mut mint, &mut Vec::new()) {
            Err(StepImportError::Topology { id: 101, what }) => {
                assert!(what.contains("passes more than once"), "{what}");
            }
            other => panic!("want the typed refusal, got {other:?}"),
        }
    }
}
