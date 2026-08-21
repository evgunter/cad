//! **The tessellation budget meter, consumer half** (issue #320):
//! where a mesh's triangles actually go, and how much of the deviation
//! budget that bought.
//!
//! `mesh::budget` measures — the trim box, the cells the schedule
//! built, the certified bounds the sizing read, the worst certificate,
//! the sampled deviation. This crate does everything downstream of
//! that: the counterfactual schedules, the split optimizer, the row
//! for every face (including the charts the meter says nothing about,
//! whose chart and triangle count are in the body and the mesh), and
//! the CSV `tools/tess-lint` reads.
//!
//! The split is where the privilege is. Nothing here needs to run
//! inside a tessellation, so nothing here is in the kernel: a change
//! to how the numbers are READ cannot reach the lane that produces
//! them.
//!
//! **The rule, with its one exception.** The rule is: the kernel reports what
//! nothing downstream can recover. `FaceMeasure::patch_steps` and
//! `CellMeasure::steps` BREAK it — they are `grid_steps(delta_s)` over
//! `(muu, muv, mvv)`, and all four values ride in the same struct, so
//! this crate could compute them. They are reported anyway, on a
//! narrower rule: **the lane's own schedule rule is not re-spelled
//! outside the lane.** `grid_steps` is the point selection the shipped
//! sizing uses; a second copy here would be a second schedule that
//! could drift from the one being measured, and a column comparing
//! the two would then be reporting on the copy. Reporting the answer
//! keeps one derivation. Everything else in the row is derived here.
//!
//! # The slack factors
//!
//! Per face the meter records what the lane used (the grid, the
//! triangle count, the certified bounds) and this crate derives what
//! it could have used. The arithmetic is deliberately kept OUT of the
//! kernel — the row carries measurements, `tools/tess-lint` names the
//! findings.
//!
//! **TESS-SPAN moved the shipped schedule onto per-knot-span-cell
//! sizing**, so the columns are derived to keep both regression kinds
//! visible (spec D-4). The shipped grid is `grid_cells`, the cell
//! count the lane actually built; `patch_cells` is the retired
//! whole-patch-sup schedule kept as a COUNTERFACTUAL column, computed
//! here from the whole-patch bound the lane still holds, so the gain
//! the promotion holds stays a number and a silent revert to
//! whole-patch sizing cannot hide.
//!
//! **Where the guards live, named — and what they do NOT cover**
//! (issue #667's Q6). `ci.yml`'s `k-lint (gate)` job runs `mesh budget
//! meter + certificate falsifier (feature = budget)` — the gated half
//! of `mesh::budget`, which the default `cargo test -p mesh` row
//! cannot reach — and `tess-meter tool fmt + clippy + tests`, this
//! crate's own derivations; then it re-tessellates the whole tour with
//! `tessellation-budget sweep (every tour scene, per face)`
//! (`scripts/tess_budget_sweep.sh`) and lints the fresh CSV against
//! `docs/tess-budget-data/tess-budget-baseline.csv` in
//! `tessellation-budget lint (gate — a grown budget fails this row)`.
//! The job is unconditional on anything that builds, so the SIZING
//! columns — triangle counts and `grid_cells / span_opt_cells`, which
//! is what `compare` reads — are a scheduled register, re-measured per
//! merge.
//!
//! **The deviation half is not.** CI runs that sweep with
//! `--sizing-only`, which skips the |S - Pi| resample, so `worst_dev`
//! is empty on every fresh row and `tess_lint::Row::total_slack` is
//! `None` for all of them. `docs/TESS-BUDGET.md`'s `total` column and
//! its total-slack factors therefore come from a `--deviation` run
//! nothing re-takes: that document is a one-shot writeup wrapped
//! around a re-measured sizing gate, not a register end to end. Read
//! its sizing columns as live and its deviation columns as dated.
//!
//! **What guards `band_schedule` itself, and the blind spot that is
//! left** (it moved here with the columns, because this is where their
//! meaning now lives): the per-triangle certificate reads the raw
//! per-cell bounds independent of the schedule, so an undersizing bug
//! ends in refinement then a typed refusal; `tools/tess-lint`'s growth
//! rules against the committed baseline; and the committed render
//! cells. **The blind spot: a schedule bug that makes the grid COARSER
//! while still certifying is invisible to a growth-only gate.**
//! Accepted because the certificate is the guarantee; stated so the
//! gate is not read as more than it is.
//!
//! **No column reports the lane's REALISATION of the schedule**, and
//! that is deliberate rather than owed — `docs/TESS-BUDGET.md`, "Why
//! there is no realisation column". The short form: such a ratio
//! divides what the lane built by what the schedule asked for, so it
//! is blind to the schedule bug above by construction; a lane that
//! realises the schedule too coarsely fails the per-triangle
//! certificate exactly, and one that realises it too densely grows
//! the triangle count, which the gate bounds at a scene total rather
//! than catches; and a realised point count matches no stated value
//! anyway, because a shared band cut carries the union of both bands'
//! columns, so a ratio built on it could not be given a tighter
//! tolerance than the one already there.
//!
//! | factor | ratio | what it says |
//! |---|---|---|
//! | **span held** | `patch_cells / grid_cells` | the gain TESS-SPAN holds over whole-patch-sup sizing. Falls toward 1.0 if the shipped schedule regresses toward the patch sup. |
//! | **split slack** | `grid_cells / span_opt_cells` | grid cells still recoverable by picking a cheaper point on each cell's constraint ellipse. The `2·a_u·a_v ≤ a_u² + a_v²` decoupling is unchanged by TESS-SPAN (the aspect-policy question is the split unit's), and a ruled wall still pays for it — see the anisotropy caveat below. |
//! | **budget slack** | `delta / worst_cert` | the sizing heuristic's headroom — two-cells-per-axis budgeting, the `ceil`, and trim boxes smaller than a full grid cell. |
//! | **certificate slack** | `worst_cert / worst_dev` | how far the Hessian interpolation bound sits above the deviation actually attained. Irreducible in part (a bound must dominate). |
//!
//! `opt_cells` (cheapest split under the WHOLE-PATCH bound) rides
//! along with the counterfactual for continuity with the #547
//! measurement; `span_opt_cells` (per-cell sizing AND the cheapest
//! split per cell) is the recoverable-slack denominator the gate
//! compares.
//!
//! **The anisotropy caveat on split slack, stated because the number
//! is otherwise too flattering**: the cheapest point on the constraint
//! curve is genuinely certified, but on a ruled wall it is a STRIP —
//! one division across the flat direction and thousands along the
//! curved one (measured on #320's leaf: `70 × 328` becomes `1 × 4905`,
//! a parameter aspect near 5·10³). Nothing in the certificate objects,
//! and nothing downstream of it has been asked whether it minds.
//! `opt_cells` is therefore an UPPER BOUND on what a practical
//! schedule would recover; a schedule that caps aspect would land
//! between it and the shipped grid, and capping it HONESTLY needs the
//! surface's first fundamental form, since parameter aspect is not
//! 3-D aspect. The span factor carries no such caveat — it changes
//! where divisions go, not how elongated a cell is.
//!
//! The product of the last two, `delta / worst_dev`, is the **total
//! slack**: the factor by which the deviation budget went unspent.
//! Because a triangle's deviation scales like `h²` and its count like
//! `1/h²`, a first-order extrapolation of "what an oracle-sized
//! uniform grid would have cost" is `triangles · worst_dev / delta`.
//!
//! **That extrapolation is an estimate, and the row says so**: it holds
//! only while the local Hessian is what it is, so it is a sizing
//! signal, never a bound. `worst_dev` is likewise a SAMPLED sup
//! (barycentric samples per triangle, at the density the meter was
//! armed with), so it under-reports the true deviation and therefore
//! over-reports the available saving. The counted-grid columns carry
//! no such caveat: they are computed from certified bounds and the
//! lane's own step rule, with each cell's `ceil` paid honestly.
//!
//! # What is measured for which chart
//!
//! Every face gets a row (chart kind + triangle count) — the question
//! "which face IS the scene's cost" needs the planar and cylinder rows
//! to be answerable. Only the NURBS lane fills the sizing columns:
//! it is the lane whose grid is Hessian-sized, and the one #320 is
//! about. Non-NURBS rows leave those columns empty rather than
//! reporting a zero that would read as a measurement.

use std::collections::HashMap;

use geom::Surface;
use mesh::Mesh;
use mesh::budget::{CellMeasure, FaceMeasure};
use topo::Body;

/// Barycentric samples per triangle edge for a sizing sweep's
/// deviation pass. 6 is a judgment call about cost: the leaf of #320
/// is a quarter-million triangles, and 28 samples each is already 7M
/// surface evaluations.
pub const DEV_SAMPLES: usize = 6;

/// The chart a face was tessellated on. Names the LANE's view, which
/// is what a budget reader needs (`Nurbs` is one row whether the face
/// was integral or rational — the cell count says which).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chart {
    /// `Surface::Plane`.
    Plane,
    /// `Surface::Cylinder`.
    Cylinder,
    /// `Surface::Cone`.
    Cone,
    /// `Surface::Sphere`.
    Sphere,
    /// `Surface::Torus`.
    Torus,
    /// `Surface::Nurbs` (described; the placeholder never reaches a row
    /// — it refuses upstream).
    Nurbs,
}

impl Chart {
    /// The chart of a face's surface.
    pub fn of(surface: &Surface<f64>) -> Self {
        match *surface {
            Surface::Plane { .. } => Chart::Plane,
            Surface::Cylinder { .. } => Chart::Cylinder,
            Surface::Cone { .. } => Chart::Cone,
            Surface::Sphere { .. } => Chart::Sphere,
            Surface::Torus { .. } => Chart::Torus,
            Surface::Nurbs(_) => Chart::Nurbs,
        }
    }

    /// The CSV token.
    pub fn tag(self) -> &'static str {
        match self {
            Chart::Plane => "plane",
            Chart::Cylinder => "cylinder",
            Chart::Cone => "cone",
            Chart::Sphere => "sphere",
            Chart::Torus => "torus",
            Chart::Nurbs => "nurbs",
        }
    }
}

/// The Hessian-sized lane's own columns (module docs).
#[derive(Clone, Copy, Debug)]
pub struct NurbsColumns {
    /// The trim box the grid spans: `u` extent.
    pub u: (f64, f64),
    /// The trim box the grid spans: `v` extent.
    pub v: (f64, f64),
    /// COUNTERFACTUAL since TESS-SPAN: the grid divisions the retired
    /// whole-patch-sup schedule would use over the trim box, `u`
    /// direction.
    pub nu: f64,
    /// The whole-patch counterfactual's `v` divisions.
    pub nv: f64,
    /// `sup ‖S_uu‖` of the whole-patch bound (the counterfactual's
    /// input; still the chord pass's boundary schedule).
    pub muu: f64,
    /// `sup ‖S_uv‖` of that bound.
    pub muv: f64,
    /// `sup ‖S_vv‖` of that bound.
    pub mvv: f64,
    /// Analysis cells the per-cell bound reported (knot spans for the
    /// integral arm, refined cells for the rational one).
    pub cells: usize,
    /// The grid the lane ACTUALLY built (TESS-SPAN: per-cell sizing),
    /// as a cell count.
    pub grid_cells: f64,
    /// `nu · nv` — the whole-patch counterfactual's cell count (the
    /// pre-TESS-SPAN `uniform_cells` column, kept so the held span
    /// gain stays a number — module docs).
    pub patch_cells: f64,
    /// The cheapest uniform grid the SAME whole-patch bound admits,
    /// over the same box (rides with the counterfactual).
    pub opt_cells: f64,
    /// Per-cell sizing AND the cheapest split in each cell — the two
    /// recoverable factors together, which is not their product.
    pub span_opt_cells: f64,
    /// The largest per-triangle certificate the face emitted.
    pub worst_cert: f64,
    /// The largest SAMPLED `|S − Π|`, or `f64::NAN` when the meter was
    /// not armed for deviation.
    pub worst_dev: f64,
    /// How many deviation samples that maximum is over (0 when not
    /// armed for deviation).
    pub dev_samples: u64,
}

/// One face's budget row.
#[derive(Clone, Copy, Debug)]
pub struct FaceRow {
    /// The face's ordinal in the body's face arena (D9 order — stable
    /// for a given body, and printable, which a slotmap key is not).
    pub face: usize,
    /// The chart its lane used.
    pub chart: Chart,
    /// The δ the mesh was requested at.
    pub delta: f64,
    /// Triangles the face contributed.
    pub triangles: usize,
    /// The Hessian-sized lane's columns, when that is the lane.
    pub nurbs: Option<NurbsColumns>,
}

/// The CSV header the sweep writes and `tools/tess-lint` reads.
pub const CSV_HEADER: &str = "scene,face,chart,delta,triangles,u0,u1,v0,v1,nu,nv,\
                              muu,muv,mvv,cells,grid_cells,patch_cells,opt_cells,\
                              span_opt_cells,worst_cert,worst_dev,dev_samples";

/// How many of [`CSV_HEADER`]'s columns are the NURBS lane's — every
/// column after `triangles`, which is the last one every row fills.
fn nurbs_column_count() -> usize {
    CSV_HEADER.split(',').count()
        - CSV_HEADER
            .split(',')
            .take_while(|c| *c != "triangles")
            .count()
        - 1
}

impl FaceRow {
    /// This row as CSV under `scene`, in [`CSV_HEADER`] order. NURBS
    /// columns are EMPTY (not zero) on a non-NURBS row — a zero there
    /// would read as a measured zero.
    ///
    /// Floats print `{:e}`, which round-trips through `str::parse`.
    pub fn csv_row(&self, scene: &str) -> String {
        let head = format!(
            "{scene},{},{},{:e},{}",
            self.face,
            self.chart.tag(),
            self.delta,
            self.triangles
        );
        match self.nurbs {
            // The empty tail is COUNTED from the header rather than
            // written as a run of commas, so a new column cannot make
            // the two arms disagree about the row's width.
            None => format!("{head}{}", ",".repeat(nurbs_column_count())),
            Some(n) => format!(
                "{head},{:e},{:e},{:e},{:e},{:e},{:e},{:e},{:e},{:e},{},\
                 {:e},{:e},{:e},{:e},{:e},{:e},{}",
                n.u.0,
                n.u.1,
                n.v.0,
                n.v.1,
                n.nu,
                n.nv,
                n.muu,
                n.muv,
                n.mvv,
                n.cells,
                n.grid_cells,
                n.patch_cells,
                n.opt_cells,
                n.span_opt_cells,
                n.worst_cert,
                n.worst_dev,
                n.dev_samples
            ),
        }
    }
}

/// One row per face of `body`, in face-arena order (D9), from the mesh
/// it produced and the measurements the meter took while producing it.
///
/// The head columns come from the body and the mesh, not from the
/// kernel: a face's chart and triangle count are already in what
/// `tessellate` returned, so the meter is not asked to report them.
///
/// # Panics
///
/// If `mesh` did not come from tessellating `body` — a patch naming a
/// face the body does not have is harness breakage, not a measurement.
pub fn face_rows(
    delta: f64,
    body: &Body<f64>,
    mesh: &Mesh,
    measures: &[FaceMeasure],
) -> Vec<FaceRow> {
    let by_face: HashMap<topo::FaceKey, &FaceMeasure> =
        measures.iter().map(|m| (m.face, m)).collect();
    mesh.patches
        .iter()
        .enumerate()
        .map(|(ordinal, patch)| {
            let surface = body
                .get_face(patch.face)
                .and_then(|f| body.get_surface(f.surface))
                .expect("the mesh's patches name this body's faces");
            FaceRow {
                face: ordinal,
                chart: Chart::of(surface),
                delta,
                triangles: patch.triangles.len(),
                nurbs: by_face.get(&patch.face).map(|m| columns(m)),
            }
        })
        .collect()
}

/// The whole CSV for one scene, header included.
pub fn csv(
    scene: &str,
    delta: f64,
    body: &Body<f64>,
    mesh: &Mesh,
    measures: &[FaceMeasure],
) -> String {
    let mut out = String::from(CSV_HEADER);
    out.push('\n');
    for row in face_rows(delta, body, mesh, measures) {
        out.push_str(&row.csv_row(scene));
        out.push('\n');
    }
    out
}

/// The NURBS columns of one face's measurements: the counterfactual
/// schedules and the cheapest splits, derived from the certified
/// bounds the lane read.
fn columns(m: &FaceMeasure) -> NurbsColumns {
    let (du, dv) = (m.u.1 - m.u.0, m.v.1 - m.v.0);
    // The retired whole-patch schedule, re-derived as the
    // counterfactual (TESS-SPAN D-4): same steps, same ceil.
    let (nu, nv) = (
        divisions(du, m.patch_steps.0),
        divisions(dv, m.patch_steps.1),
    );
    let patch = Bound {
        muu: m.muu,
        muv: m.muv,
        mvv: m.mvv,
        steps: m.patch_steps,
    };
    #[allow(clippy::cast_precision_loss)]
    let grid_cells = m.grid_cells as f64;
    NurbsColumns {
        u: m.u,
        v: m.v,
        nu,
        nv,
        muu: m.muu,
        muv: m.muv,
        mvv: m.mvv,
        cells: m.cells.len(),
        grid_cells,
        patch_cells: nu * nv,
        opt_cells: best_split_cells(patch, du, dv, m.delta_s),
        span_opt_cells: span_opt_cells(&m.cells, m.u, m.v, m.delta_s),
        worst_cert: m.worst_cert,
        worst_dev: m.worst_dev,
        dev_samples: m.dev_samples,
    }
}

/// A certified Hessian bound and the grid steps it admits at the
/// face's sizing target — the shape [`best_split_steps`] optimizes
/// around, whether it came from the whole patch or from one cell.
#[derive(Clone, Copy, Debug)]
pub struct Bound {
    /// `sup ‖S_uu‖`.
    pub muu: f64,
    /// `sup ‖S_uv‖`.
    pub muv: f64,
    /// `sup ‖S_vv‖`.
    pub mvv: f64,
    /// The lane's own `(h_u, h_v)` for this bound, as the kernel
    /// reported them — the schedule the optimizer must never lose to.
    pub steps: (f64, f64),
}

impl From<&CellMeasure> for Bound {
    fn from(c: &CellMeasure) -> Self {
        Bound {
            muu: c.muu,
            muv: c.muv,
            mvv: c.mvv,
            steps: c.steps,
        }
    }
}

/// Grid divisions an extent needs at step `h`. An unconstrained
/// direction (`h = ∞`, e.g. the ruled direction of a wall with
/// `muv = 0`) takes one.
///
/// **It is the second spelling of the lane's `sizing::ceil_count`, and
/// it deliberately does not match it.** They cannot share an import —
/// two cargo roots — so the divergences are stated instead of left to
/// be discovered:
///
/// * `ceil_count` REFUSES a count at or above its `MAX_COUNT` (2^24) with
///   a typed error, because it is about to allocate that many grid
///   points. This one counts and returns, because it sizes nothing:
///   an absurd counterfactual is a number in a diagnostic column, and
///   turning it into a refusal would make the meter able to fail a
///   tessellation that succeeded.
/// * `ceil_count` treats a non-positive step as the caller's error.
///   This one answers 1, because an unconstrained direction is a
///   normal thing for a counterfactual to be asked about.
///
/// The shared part — `ceil(extent / h)`, floored at one — is the part
/// the columns are comparable through, and it is identical. The
/// different NAME is the tell: the lane says *count* for a `usize`
/// division count it is about to allocate for, and this says
/// *divisions* for an `f64` counterfactual that allocates nothing.
pub fn divisions(extent: f64, h: f64) -> f64 {
    if h.is_finite() && h > 0.0 {
        (extent / h).ceil().max(1.0)
    } else {
        1.0
    }
}

/// How many `t = h_v / h_u` aspect ratios [`best_split_cells`] tries,
/// and over how many decades either side of square. A SCAN, not a
/// closed form, and deliberately: the closed-form interior optimum
/// (`h_v/h_u = √(muu/mvv)`, from `∇(h_u·h_v) ∥ ∇Q`) degenerates
/// exactly where these walls live — a ruled direction has `muu = 0`
/// and pushes the optimum onto the `h_u ≤ extent` boundary — and the
/// two `ceil`s make the true objective a step function anyway.
///
/// # Why these two carry no mechanical guard
///
/// **Not an omission: the quantity anyone would guard — the cell count
/// this scan produces — is DISCONTINUOUS in the parameters they would
/// guard it against.** Moving the sample count by one moves the worst
/// relative excess by percentage points in either direction, with no
/// convergence, so no tolerance on it can admit every refinement and
/// exclude every degradation: wide enough to survive the jumps is too
/// weak to catch anything, tight enough to catch a degradation is a
/// lottery on which lattice the count lands.
///
/// **The discontinuity is the two `ceil`s, not the scan.** The same
/// worst-excess computed WITHOUT them — the cost as a continuous
/// function of `t` — falls smoothly with resolution and depends on the
/// sampling step `2·DECADES/(SAMPLES−1)` and the range, which is what
/// these two constants actually set. A guard on THAT quantity is
/// continuous where a guard on the cell count cannot be; it is not
/// written here because it measures something these columns do not
/// report.
///
/// **So what these constants guarantee is a resolution in aspect
/// ratio, and not a bound on the answer.** The `ceil` quantisation on
/// top of it is real and is not theirs to control. Anyone re-tuning
/// them should know that the shipped pair is not even locally best on
/// the cell count, and that this is exactly the kind of fact a step
/// function produces and no amount of tuning removes.
const SPLIT_SCAN_DECADES: f64 = 8.0;
/// Samples per scan (fixed, so the answer is deterministic — D9).
/// SAMPLES, not steps: a step in this crate's vocabulary is a UV
/// increment, and these are trial aspect ratios.
const SPLIT_SCAN_SAMPLES: usize = 321;

/// The cheapest uniform grid a bound admits over one box: minimize
/// `divisions(U, h_u) · divisions(V, h_v)` subject to the SAME
/// certificate the lane checks,
/// `muu·h_u² + 2·muv·h_u·h_v + mvv·h_v² ≤ δ_s`.
///
/// That constraint is the per-triangle bound `Q/4` at the lane's own
/// two-cells-per-axis budgeting (`a_u ≤ 2h_u`), so a grid found here
/// certifies EXACTLY as the shipped one does — the difference is only
/// which `(h_u, h_v)` on the constraint ellipse gets picked. The
/// shipped schedule picks the point the decoupling `2·a_u·a_v ≤
/// a_u² + a_v²` leaves it at, which is not the cheapest point when the
/// two directions are anisotropic.
///
/// The lane's own steps are evaluated too, so the answer can never
/// come out worse than what the lane already does.
pub fn best_split_cells(bound: Bound, du: f64, dv: f64, delta_s: f64) -> f64 {
    best_split_steps(bound, du, dv, delta_s).0
}

/// [`best_split_cells`] with the steps it chose: `(cells, h_u, h_v)`.
/// Split out so the constraint can be asserted on the ANSWER and not
/// merely on the formula that produced it (see this crate's tests).
pub fn best_split_steps(bound: Bound, du: f64, dv: f64, delta_s: f64) -> (f64, f64, f64) {
    let (muu, muv, mvv) = (bound.muu, bound.muv, bound.mvv);
    // The steps at aspect ratio `t = h_v / h_u`: the constraint is
    // homogeneous of degree 2 in h_u, so h_u falls straight out.
    let steps = |t: f64| -> (f64, f64) {
        let q = mvv.mul_add(t.powi(2), 2.0f64.mul_add(muv * t, muu));
        let hu = if q > 0.0 {
            (delta_s / q).sqrt()
        } else {
            f64::INFINITY
        };
        (hu, t * hu)
    };
    let (lane_u, lane_v) = bound.steps;
    let mut best = (
        divisions(du, lane_u) * divisions(dv, lane_v),
        lane_u,
        lane_v,
    );
    for k in 0..SPLIT_SCAN_SAMPLES {
        #[allow(clippy::cast_precision_loss)]
        let f = k as f64 / (SPLIT_SCAN_SAMPLES - 1) as f64;
        let (hu, hv) = steps(10.0f64.powf(SPLIT_SCAN_DECADES * f.mul_add(2.0, -1.0)));
        let n = divisions(du, hu) * divisions(dv, hv);
        if n < best.0 {
            best = (n, hu, hv);
        }
    }
    best
}

/// The pure per-cell ideal over the trim box: each cell's own RAW
/// bound through [`best_split_cells`], clipped to the box with the
/// `ceil` paid per cell.
///
/// This is the recoverable-slack denominator; the banding's
/// max-across-u cost stays visible in it.
///
/// **Why aligning rows to the band cuts is enough for the
/// certificate**: with rows on the band boundaries, every grid
/// triangle's UV box lies inside one band, so the band's own certified
/// bounds are the ones its certificate uses. Cells are half-open — a
/// knot is exactly where a C¹ surface's second derivative jumps — but
/// the shared boundary is measure-zero, and the Taylor remainder the
/// certificate is built on needs only an a.e. bound. That is the same
/// fact the shipped whole-patch assembly already rests on at its own
/// interior knots (`mesh::nurbs_cert` docs).
fn span_opt_cells(cells: &[CellMeasure], u: (f64, f64), v: (f64, f64), delta_s: f64) -> f64 {
    let mut opt = 0.0;
    for c in cells {
        let du = c.u.1.min(u.1) - c.u.0.max(u.0);
        let dv = c.v.1.min(v.1) - c.v.0.max(v.0);
        if !(du > 0.0 && dv > 0.0) {
            continue; // cell outside the trim box
        }
        opt += best_split_cells(c.into(), du, dv, delta_s);
    }
    opt
}
