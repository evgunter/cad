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
//!
//! **Those rows are SAMPLED, not unconditional.** Each carries an
//! `if:` on the drawn `klint_row` — the sweep and its lint on
//! `release-budget`, this crate's derivations on `dev-default`, one of
//! five rows drawn per run from the head SHA. So the SIZING columns —
//! triangle counts and `grid_cells / span_opt_cells`, which is what
//! `compare` reads — are re-measured on about one merge in five, and
//! so is the guard on the split scan's two constants below. Neither
//! quantity drifts between merges: both are functions of this tree
//! alone. What the sampling costs is therefore latency and not
//! staleness — a retune can land unmeasured and be caught by a later
//! draw — which is a weaker thing than the per-merge register the
//! sizing columns have been read as.
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
//! | **span held** | `patch_cells / grid_cells` | the gain TESS-SPAN holds over whole-patch-sup sizing (both sides through the shipped selection). Falls toward 1.0 if the shipped schedule regresses toward the patch sup. |
//! | **split slack** | `grid_cells / span_opt_cells` | grid cells still recoverable by picking a cheaper point on each cell's constraint ellipse. Since TESS-SPLIT the shipped selection IS the cell-minimizing point under the ratified A = 16 aspect cap, so this reads ~1.0 where no constraint is active; the residue above 1.0 is the PRICE of the cap and the sliver snap (the denominator is the UNCONSTRAINED optimum — the anisotropy caveat below), attributed per face by `cap_bands` / `snap_bands`. |
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
//! schedule recovers; since TESS-SPLIT the shipped schedule IS the
//! aspect-capped point (through the first fundamental form, since
//! parameter aspect is not 3-D aspect), so it lands between the strip
//! optimum and the retired AM-GM point, and the gap that remains
//! against `opt_cells`/`span_opt_cells` is the cap's deliberate,
//! indicated price. The span factor carries no such caveat — it
//! changes where divisions go, not how elongated a cell is.
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
//! # Which columns may carry a fallback: none of them
//!
//! **Every column here is read by a DIFFERENTIAL gate, so a reading
//! this crate could not take has two ways to lie and not one.**
//! `tools/tess-lint` fires on `now > was · GROWTH_TOLERANCE`. In the
//! FRESH row an invented in-band number pushes the verdict one way; in
//! the COMMITTED BASELINE the same number pushes it the other, by
//! inflating `was` until a real regression fits underneath. A
//! disposition that reasons about the direction on one of those rows
//! has answered half the question and reads as if it answered all of
//! it, so the question is settled here for every column at once rather
//! than per site.
//!
//! **The line is not which column, it is what the value means.** An
//! unconstrained direction is a READING: `h = ∞`, or a certified
//! `Q(t) = 0`, says this direction constrains nothing, the answer is
//! one division, and that answer is as correct in the gated
//! `span_opt_cells` as in the counterfactual `nu`. A value that could
//! not be read is not a reading, and nothing here answers one: a NaN
//! sup, a negative or zero step, a NaN extent, a cell box with a NaN
//! corner. Those panic. The reason they may not fall back is
//! arithmetic rather than taste — every fallback available is a small
//! count, the gate fires only on GROWTH, and a small count is in band
//! on the fresh row and hides growth on the baseline row. `divisions`'
//! fallback was `1.0`, and **nothing downstream would have caught
//! it**: `tess-lint`'s `Admissible::CellCount` admits
//! `v.is_finite() && v >= 1.0`, so `1.0` is exactly the smallest value
//! it calls a reading. A parse guard bounds what a column may SAY; it
//! cannot know whether the producer measured it.
//!
//! **The counterfactual columns are not an exception to this**, though
//! the argument for one is real: `nu`, `nv`, `patch_cells` and
//! `opt_cells` are diagnostics no rule reads, so a fabricated value
//! there decides nothing today. It would decide something the moment a
//! rule read them, and a fallback whose safety is a property of the
//! consumer roster is a fallback waiting for a consumer.
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
///
/// **The triangle count is a reading of one corpus at one time and
/// nothing re-takes it** — it moves with every scene added to the demo
/// tour and with every δ. It is written to show the ORDER the judgment
/// was made against, not as a figure anyone should compute with; the
/// 28 beside it is arithmetic on this constant (the barycentric lattice
/// at edge samples 6) and follows it. Nothing is guarded here and
/// nothing should be: this constant costs only the deviation pass,
/// which is what `--sizing-only` skips, and the CI gate reads none of
/// the columns it fills (`scripts/tess_budget_sweep.sh` says so at the
/// flag). What a reader chasing the current triangle count wants is a
/// sweep's own output, not this line.
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
    /// `Surface::Approx` — an approximating surface. Its own row: the
    /// lane meshes its FIT, so the cell count is a spline's, but what
    /// the face carries is a description plus a certificate, and a
    /// budget reader that saw `nurbs` here would not know that.
    Approx,
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
            Surface::Approx(_) => Chart::Approx,
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
            Chart::Approx => "approx",
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
    /// COUNTERFACTUAL since TESS-SPAN: the grid divisions a
    /// whole-patch-sup schedule would use over the trim box, `u`
    /// direction — through the SHIPPED point selection
    /// (`FaceMeasure::patch_steps`, since TESS-SPLIT the aspect-capped
    /// one), so the column keeps meaning "what per-cell sizing saves"
    /// as the selection moves.
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
    /// `sup ‖S_u‖` of that bound — the first-fundamental-form sample
    /// the shipped selection's 3-D aspect cap reads (TESS-SPLIT).
    pub mu1: f64,
    /// `sup ‖S_v‖` of that bound.
    pub mv1: f64,
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
    /// Bands the shipped schedule emitted.
    pub bands: usize,
    /// **The constraint-activity indicator (TESS-SPLIT D-3), A-cap
    /// kind**: bands whose step selection the 3-D aspect cap clamped.
    /// Reported by the lane, never re-derived — the selection rule is
    /// not re-spelled outside it.
    pub cap_bands: usize,
    /// The indicator's sliver/snap kind: bands the malign-band snap
    /// projected onto the patch column schedule with changed counts
    /// (either direction — columns added, or columns traded for rows).
    pub snap_bands: usize,
    /// Max over bands of the emitted lattice's post-`ceil` spacing
    /// ratio `s_u/s_v` — the realized aspect `SAFE_ASPECT` judges,
    /// reported so "which faces sit above the sliver line, under which
    /// protection" is read off the CSV.
    pub realized_aspect: f64,
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
                              muu,muv,mvv,mu1,mv1,cells,grid_cells,patch_cells,\
                              opt_cells,span_opt_cells,worst_cert,worst_dev,\
                              dev_samples,bands,cap_bands,snap_bands,realized_aspect";

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
                "{head},{:e},{:e},{:e},{:e},{:e},{:e},{:e},{:e},{:e},{:e},{:e},{},\
                 {:e},{:e},{:e},{:e},{:e},{:e},{},{},{},{},{:e}",
                n.u.0,
                n.u.1,
                n.v.0,
                n.v.1,
                n.nu,
                n.nv,
                n.muu,
                n.muv,
                n.mvv,
                n.mu1,
                n.mv1,
                n.cells,
                n.grid_cells,
                n.patch_cells,
                n.opt_cells,
                n.span_opt_cells,
                n.worst_cert,
                n.worst_dev,
                n.dev_samples,
                n.bands,
                n.cap_bands,
                n.snap_bands,
                n.realized_aspect
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
        mu1: m.mu1,
        mv1: m.mv1,
        cells: m.cells.len(),
        grid_cells,
        patch_cells: nu * nv,
        opt_cells: best_split_cells(patch, du, dv, m.delta_s),
        span_opt_cells: span_opt_cells(&m.cells, m.u, m.v, m.delta_s),
        worst_cert: m.worst_cert,
        worst_dev: m.worst_dev,
        dev_samples: m.dev_samples,
        bands: m.bands,
        cap_bands: m.cap_bands,
        snap_bands: m.snap_bands,
        realized_aspect: m.realized_aspect,
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
/// `muv = 0`) takes one, and `ceil(extent / ∞)` floored at one already
/// says so — the arithmetic answers it, so no arm decides it.
///
/// **It is the second spelling of the lane's `sizing::ceil_count`, and
/// it deliberately does not match it.** They cannot share an import —
/// two cargo roots — so the divergence is stated instead of left to be
/// discovered: `ceil_count` REFUSES a count at or above its
/// `MAX_COUNT` (2^24) with a typed error, because it is about to
/// allocate that many grid points. This one counts and returns,
/// because it sizes nothing: an absurd counterfactual is a large
/// number in a diagnostic column, and turning it into a refusal would
/// make the meter able to fail a tessellation that succeeded.
///
/// The shared part — `ceil(extent / h)`, floored at one — is the part
/// the columns are comparable through, and it is identical. The
/// different NAME is the tell: the lane says *count* for a `usize`
/// division count it is about to allocate for, and this says
/// *divisions* for an `f64` counterfactual that allocates nothing.
///
/// # Panics
///
/// On a step or an extent that is not a reading — a NaN or non-positive
/// step, a non-finite or negative extent. That is this crate's fallback
/// rule (module docs) applied where the number is produced: every
/// column here is read by a differential gate on TWO rows, so an
/// invented in-band value hides a regression on one of them whichever
/// way it leans. Note that without the step assertion `.max(1.0)` would
/// swallow a NaN silently — `f64::max` prefers its non-NaN argument —
/// and hand the gate a fabricated single division, which `tess-lint`
/// ADMITS (`Admissible::CellCount` is `v.is_finite() && v >= 1.0`)
/// rather than refuses.
///
/// This is the meter's second divergence from `ceil_count`, in the
/// stricter direction: the lane refuses a NaN and a zero step (both
/// make its `raw` non-finite) and answers ONE for a negative step,
/// whose `raw` is negative, finite and floored. A negative step is not
/// a smaller counterfactual; it is a reading that did not happen.
///
/// **Nothing in the tree reaches these assertions, and that is a trace
/// rather than a hope.** Two kernel gates run strictly before
/// `mesh::budget::note_face` and admit strictly less than this refuses:
/// `mesh::tessellate` returns `InvalidChordalTolerance` unless
/// `chordal` is finite and positive, and `δ_s` is `chordal · 0.5`; and
/// `nurbs_cert`'s `nurbs_face_bound` and `nurbs_cell_grid` both return
/// `UnsupportedNurbsFace` (*"second-derivative hull is
/// unbounded/poisoned"*) unless every component of the bound is finite.
/// Non-negativity rides along with those components being sups of
/// norms. So the refusals guard the meter against a kernel that stopped
/// doing that, not against inputs it meets today.
pub fn divisions(extent: f64, h: f64) -> f64 {
    assert!(
        h > 0.0,
        "a grid step of {h} is not a reading: the meter has no division count to report"
    );
    assert!(
        extent.is_finite() && extent >= 0.0,
        "an extent of {extent} is not a reading: the meter has no division count to report"
    );
    (extent / h).ceil().max(1.0)
}

/// How many `t = h_v / h_u` aspect ratios [`best_split_cells`] tries,
/// and over how many decades either side of square. A SCAN, not a
/// closed form, and deliberately: the closed-form interior optimum
/// (`h_v/h_u = √(muu/mvv)`, from `∇(h_u·h_v) ∥ ∇Q`) degenerates
/// exactly where these walls live — a ruled direction has `muu = 0`
/// and pushes the optimum onto the `h_u ≤ extent` boundary — and the
/// two `ceil`s make the true objective a step function anyway.
///
/// # What boxes these two, and what nothing can box
///
/// **What they guarantee is a resolution in aspect ratio, and not a
/// bound on the answer** — so that is what carries the guard, and
/// `tests/derivations.rs` boxes its two failure modes separately: a
/// RANGE too narrow to bracket the optimum (the scan's argmin lands on
/// an endpoint) and a STEP too coarse to resolve it
/// ([`unfloored_worst_excess`] over the ceiling the split column's
/// consumer can absorb).
///
/// **The guarantee is analytic on both classes, and the difference
/// between them is `divisions`' one-division floor.** Where the optimum
/// is the interior stationary point ([`optimum_is_unfloored`]) the
/// excess is bounded by [`unfloored_worst_excess`]; where the floor
/// binds the objective has a KINK instead, the excess grows linearly
/// rather than quadratically in the distance to the nearest sample, and
/// [`floored_worst_excess`] bounds that from the kink's two exact
/// branch ratios. At the shipped pair they are 0.16573% and 2.09180%,
/// and the second is a supremum rather than a sample: two independent
/// random searches over the class, 400,000 bounds and 4.6 M, found
/// 2.0768% and 2.0918% under it.
///
/// # What these two constants do NOT hold, and the lever that would
///
/// **Both bounds are on the CONTINUOUS objective**, which is what these
/// constants govern smoothly. `tools/tess-lint` divides by
/// `span_opt_cells`, which is the `ceil`'d one, and there the
/// instrument is already outside its consumer's margin: an anisotropic
/// bound with a live cross term (`muu = 0.1, muv = 1, mvv = 50`) scores
/// **5.8824%** against `GROWTH_TOLERANCE − 1 = 5%`, and a single smooth
/// geometry change through it — `mvv` scaled 1× to 100×, counts in the
/// thousands — runs the scan-to-true ratio from 1.00000 to 1.0588. So
/// the meter's own resolution can move a face across the gate's
/// threshold with no schedule change at all.
///
/// **The lever, recorded so the next taker does not re-derive it**: the
/// one-sided envelope `10^(decades/(samples − 1)) − 1` drops under 5%
/// at `SPLIT_SCAN_SAMPLES ≥ 379` for `SPLIT_SCAN_DECADES = 8`. That
/// costs no range and it is cheap. It is deliberately NOT taken here —
/// raising the sample count moves every committed budget number and
/// re-cuts `docs/tess-budget-data/`, which is its own unit rather than
/// a guard's fix pass.
///
/// **The other lever is narrowing the range, and that question is
/// open**: 3.7 decades would bring the continuous excess to 2.70% and
/// every claim in the derivations suite stays green, because no family
/// member's optimum lives above `t = 1`. Nothing in this tree
/// characterises what `muu/mvv` ratios real certified bounds produce,
/// so narrowing to the family's spread would be fitting the constant to
/// the test — the range question needs that characterisation first, and
/// the resolution question has the cheaper answer above in the
/// meantime.
///
/// **The cell count these columns report cannot carry a guard, and
/// nobody should re-attempt one.** The two `ceil`s in [`divisions`]
/// make it DISCONTINUOUS in the parameters a guard would be written
/// against: the worst relative excess moves ~4 percentage points
/// between ADJACENT sample counts (321: 5.88%, 322: 3.64%, 323: 5.24%,
/// 324: 1.79%, 325: 3.94%) and does not converge — 2,000 samples is
/// still 0.79%. A tolerance wide enough to survive the jumps catches
/// nothing; one tight enough to catch a degradation is a lottery on
/// which lattice the count lands. Two instruments were built against
/// that quantity and both failed, `323` being the witness that killed
/// the second. The `ceil` quantisation sits on TOP of the resolution
/// these constants buy and is not theirs to control, which is why the
/// shipped pair is not even locally best on the cell count.
///
/// **The guard on this pair runs on the merge that moves it.** What
/// boxes these two is this crate's own derivations suite, and the only
/// k-lint unification that runs that suite is `dev-default` — one of
/// five, drawn per run. A change anywhere under `tools/` now PINS that
/// row rather than sampling it (`KLINT_PATH_ROWS` in
/// `scripts/ci-filter.py`), so a retune here is gated by the guard it
/// is about instead of by whichever row a hash picked.
pub const SPLIT_SCAN_DECADES: f64 = 8.0;
/// Samples per scan (fixed, so the answer is deterministic — D9).
/// SAMPLES, not steps: a step in this crate's vocabulary is a UV
/// increment, and these are trial aspect ratios.
///
/// **Provenance and guard are the PAIR's, at [`SPLIT_SCAN_DECADES`]
/// directly above** — what the two buy is one quantity (the resolution
/// of the aspect-ratio scan), no derivation reaches either alone, and
/// the boxing test asserts them together. Read that paragraph before
/// retuning this: it also records why the cell count these constants
/// feed cannot carry a guard, and that a change under `tools/` now
/// forces the CI row that runs the box rather than sampling it. Stated
/// as a pointer and not a second copy, because the two constants moving
/// apart in their documentation is the first step to their moving apart
/// in fact.
pub const SPLIT_SCAN_SAMPLES: usize = 321;

/// The aspect ratios `t = h_v / h_u` a scan of `decades` either side of
/// square visits at `samples` points, log-uniformly and in order.
///
/// **The one derivation of the scan's lattice**, driven at the shipped
/// pair by [`shipped_split_scan_aspects`] and at other pairs by the
/// derivations suite, to measure what those two constants buy. A second
/// spelling of the placement would turn that measurement into a
/// statement about the copy.
///
/// # Panics
///
/// If `samples < 2`: a scan of one point has no sampling step, and the
/// resolution these constants exist to set is undefined without one.
pub fn split_scan_aspects(decades: f64, samples: usize) -> impl Iterator<Item = f64> {
    let spans = split_scan_spans(samples);
    // Spelled `decades·(2k/spans − 1)` rather than through the step, so
    // the lattice is bit-identical to the loop this was factored out
    // of. The two groupings differ by up to tens of ulps at 95 of 321
    // points, which is invisible to the continuous objective and is
    // exactly the kind of thing a `ceil` turns into a whole division.
    (0..samples).map(move |k| {
        #[allow(clippy::cast_precision_loss)]
        let f = k as f64 / spans;
        10.0f64.powf(decades * f.mul_add(2.0, -1.0))
    })
}

/// The aspect lattice the shipped optimizer scans.
///
/// **One call site for the pair, and it is this one.** The derivations
/// suite drives [`split_scan`] through this same function, so the
/// lattice the guard measures is the lattice [`best_split_steps`] uses.
/// A second call site carrying its own literals would be invisible to
/// that guard — the constants would still be boxed and the scan would
/// still be wrong — which is the failure this function exists to make
/// unspellable.
pub fn shipped_split_scan_aspects() -> impl Iterator<Item = f64> {
    split_scan_aspects(SPLIT_SCAN_DECADES, SPLIT_SCAN_SAMPLES)
}

/// Sampling intervals in a scan of `samples` points, as an `f64` — the
/// one place the scan's shape is checked.
///
/// # Panics
///
/// If `samples < 2`: a scan of one point has no sampling step, and the
/// resolution these constants exist to set is undefined without one.
fn split_scan_spans(samples: usize) -> f64 {
    assert!(
        samples >= 2,
        "a split scan of {samples} sample(s) has no sampling step"
    );
    #[allow(clippy::cast_precision_loss)]
    let spans = (samples - 1) as f64;
    spans
}

/// Half the scan's sampling step, in decades of aspect ratio: the
/// furthest any aspect can sit from the nearest sample, and the only
/// thing [`SPLIT_SCAN_DECADES`] and [`SPLIT_SCAN_SAMPLES`] jointly fix.
fn split_scan_half_step(decades: f64, samples: usize) -> f64 {
    decades / split_scan_spans(samples)
}

/// The worst relative excess a scan at `(decades, samples)` can leave on
/// the CONTINUOUS objective — [`best_split_cells`]'s cost with the two
/// `ceil`s of [`divisions`] removed — **over the bounds whose optimum
/// lies strictly above the one-division floor**, which is the domain
/// [`optimum_is_unfloored`] answers and NOT every bound.
///
/// **Derivation, and the domain is where it comes from.** Above the
/// floor the continuous cost at aspect `t` is
/// `U·V·(muu/t + 2·muv + mvv·t) / δ_s`, whose interior stationary point
/// is `t* = √(muu/mvv)`. Writing `t = t*·10^x`, the ratio to the
/// optimum is `1 + (cosh(x·ln 10) − 1) / (1 + muv/√(muu·mvv))`, so a
/// live cross term only ever shrinks it and the worst case is
/// `muv = 0`; no aspect sits further than half a sampling step from a
/// sample, and half the step is `decades/(samples − 1)` decades. **Every
/// line of that assumes `t*` is the optimum**, and it is not when
/// `divisions`' floor binds there: the objective then has a KINK rather
/// than a smooth minimum, its excess grows linearly in the distance to
/// the nearest sample rather than quadratically, and this value bounds
/// nothing. For `muv = 0` and a unit box the condition is
/// `muu ≥ δ_s/2` and `mvv ≥ δ_s/2`.
///
/// **On its own domain it is attained, not conservative**: an isotropic
/// bound puts `t*` at exactly `1`, and at an even `samples` the lattice
/// straddles `1` half a step either side. The derivations suite
/// measures a family against it for that reason — a closed form nothing
/// witnesses is theory, not a guard — and carries the floored class as
/// its own members, measured rather than bounded.
///
/// It bounds nothing either when the optimum lies OUTSIDE `10^±decades`;
/// that is the range failure, guarded separately.
///
/// # Panics
///
/// If `samples < 2`.
#[must_use]
pub fn unfloored_worst_excess(decades: f64, samples: usize) -> f64 {
    (split_scan_half_step(decades, samples) * std::f64::consts::LN_10).cosh() - 1.0
}

/// The worst relative excess a scan at `(decades, samples)` can leave
/// on the FLOORED class — the bounds whose continuous optimum is a kink
/// on [`divisions`]' one-division floor rather than the interior
/// stationary point [`unfloored_worst_excess`] assumes.
///
/// # The derivation
///
/// Take `muv = 0` and a unit box, and let the `u` floor bind, so
/// `r = muu/δ_s ∈ (0, ½)`. The optimum is the kink at
/// `t₁ = √((δ_s − muu)/mvv)`, where `Q(t₁) = δ_s` and `h_u` is exactly
/// the extent. Writing `u = t/t₁`, the cost RATIO to that optimum is
/// exact on each side and needs no slope approximation:
///
/// * left of the kink the `u` divisions are floored, so the cost is the
///   `v` count alone and `R(u) = √(r + (1 − r)·u²)/u`;
/// * right of it neither floor binds, so the cost is the product and
///   `R(u) = (r + (1 − r)·u²)/u`.
///
/// The scan sees whichever of the two neighbouring samples is cheaper,
/// so the worst placement equalises the two branch ratios across one
/// sampling step, and the worst bound maximises that over `r`. Both are
/// solved here — a bisection on the placement inside a sweep over `r` —
/// because the equalisation is transcendental. The linearised form,
/// `10^(step·r(1−2r)/(1−r)) − 1`, has its maximum at
/// `r = (2 − √2)/2 = 0.29289` and is worth knowing as the anchor: at
/// the shipped pair it gives 1.995% where the exact value below gives
/// **2.0918%**, the curvature of the two branches being the difference.
///
/// **It is a supremum, not a sample.** Two independent random searches
/// over the class — 400,000 bounds here, 4.6 M in review — found
/// 2.0768% and 2.0918% against this 2.09180%, and the family member
/// `floored, cross-term-free` sits at `r = 0.29808`, which is the
/// analytic argmax. The `muv > 0` case only dilutes the ratio, exactly
/// as in the unfloored derivation, and the mirrored `v`-floor case is
/// the same expression with the extents exchanged; the sweeps cover
/// both and found no exceedance.
///
/// # Panics
///
/// If `samples < 2`.
#[must_use]
pub fn floored_worst_excess(decades: f64, samples: usize) -> f64 {
    // Converged: the value is stable to eight significant figures from
    // 1,024 `r` samples upward, and D9 wants a fixed structure rather
    // than a tolerance-driven loop.
    //
    // THAT CONVERGENCE IS A ONE-TIME READING, TAKEN BY RAISING THIS
    // CONSTANT AND WATCHING THE ANSWER, AND NOTHING RE-TAKES IT. It can
    // be re-taken in one edit — raise `RATIOS`, run this crate's
    // derivations suite, compare — which is why it earns a note rather
    // than a guard: a test pinning the value to eight figures would pin
    // the ARITHMETIC of this function, not its convergence, and would
    // red on any legitimate refinement of the bound. The margin the
    // constant is chosen against is generous by a factor of four
    // deliberately, so a reader retuning it is moving away from the
    // measured plateau rather than toward its edge.
    const RATIOS: usize = 4096;
    const PLACEMENT_STEPS: usize = 100;
    let step = 2.0 * split_scan_half_step(decades, samples) * std::f64::consts::LN_10;
    // Left branch at distance `a` below the kink, right branch at `a`
    // above it, both as ratios to the optimum.
    let left = |r: f64, a: f64| (2.0 * a).exp().mul_add(r, 1.0 - r).sqrt();
    let right = |r: f64, a: f64| (-a).exp().mul_add(r, (1.0 - r) * a.exp());
    let mut worst: f64 = 0.0;
    for i in 1..RATIOS {
        #[allow(clippy::cast_precision_loss)]
        let r = 0.5 * i as f64 / RATIOS as f64;
        let (mut lo, mut hi) = (0.0, step);
        for _ in 0..PLACEMENT_STEPS {
            let mid = 0.5 * (lo + hi);
            if left(r, mid) < right(r, step - mid) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        worst = worst.max(left(r, lo).min(right(r, step - lo)));
    }
    worst - 1.0
}

/// Whether `bound`'s continuous optimum over `du × dv` at `delta_s` is
/// the interior stationary point `t* = √(muu/mvv)` rather than a kink
/// on [`divisions`]' one-division floor — i.e. whether
/// [`unfloored_worst_excess`] says anything about it.
///
/// It takes `delta_s` and the box because the condition is on them: the
/// steps at `t*` are `h_u = √(δ_s/Q(t*))` and `h_v = t*·h_u`, and the
/// floor binds exactly when either exceeds its extent. A bound with no
/// interior stationary point at all — a ruled direction's `muu = 0`, or
/// a vanishing `mvv` — is outside the domain by the same test.
#[must_use]
pub fn optimum_is_unfloored(bound: Bound, du: f64, dv: f64, delta_s: f64) -> bool {
    let (muu, muv, mvv) = (bound.muu, bound.muv, bound.mvv);
    if !(muu > 0.0 && mvv > 0.0) {
        return false;
    }
    let t = (muu / mvv).sqrt();
    let q = mvv.mul_add(t * t, 2.0f64.mul_add(muv * t, muu));
    let hu = (delta_s / q).sqrt();
    hu <= du && t * hu <= dv
}

/// One split scan's answer: the cheapest grid it found, the steps that
/// give it, and which sample won — `None` when the seed did.
#[derive(Clone, Copy, Debug)]
pub struct SplitScan {
    /// The counted grid at [`SplitScan::steps`], in whatever `count`
    /// the scan was driven with.
    pub cells: f64,
    /// The `(h_u, h_v)` that count belongs to.
    pub steps: (f64, f64),
    /// The index into `aspects` that won, or `None` for the seed.
    pub sample: Option<usize>,
}

/// **The split scan, once, with its counting function as a parameter.**
///
/// [`best_split_steps`] is this over [`shipped_split_scan_aspects`],
/// counting with [`divisions`] and seeded with the lane's own grid. The
/// derivations suite is this over the same aspects, counting with the
/// same expression MINUS its `ceil`, and unseeded — which is the only
/// way a guard on the scan's resolution can be a guard on THIS scan.
/// Everything that could drift between the two is derived once, here:
/// the lattice, `Q(t)`, the step the constraint fixes, the running
/// minimum. A guard that re-spelled any of them would be measuring its
/// own copy, which is this crate's own rule about the lane's schedule
/// applied to itself.
///
/// # Panics
///
/// On a bound or a sizing target that is not a reading (module docs).
/// The certified sups are non-negative by construction — they are sups
/// of norms — so a NEGATIVE one is a sign error rather than a
/// measurement, and it is checked HERE rather than through `Q(t)`,
/// which a negative `muv` passes at every sampled `t` whenever
/// `muv² ≤ muu·mvv`.
pub fn split_scan<F: Fn(f64, f64) -> f64>(
    bound: Bound,
    du: f64,
    dv: f64,
    delta_s: f64,
    aspects: impl Iterator<Item = f64>,
    seed: Option<(f64, f64)>,
    count: F,
) -> SplitScan {
    let (muu, muv, mvv) = (bound.muu, bound.muv, bound.mvv);
    assert!(
        muu >= 0.0
            && muv >= 0.0
            && mvv >= 0.0
            && muu.is_finite()
            && muv.is_finite()
            && mvv.is_finite(),
        "a certified bound of muu={muu}, muv={muv}, mvv={mvv} is not a reading: \
         the meter has no cheapest split to report"
    );
    assert!(
        delta_s > 0.0 && delta_s.is_finite(),
        "a sizing target of {delta_s} is not a reading: \
         the meter has no cheapest split to report"
    );
    let mut best = seed.map(|(hu, hv)| SplitScan {
        cells: count(du, hu) * count(dv, hv),
        steps: (hu, hv),
        sample: None,
    });
    for (k, t) in aspects.enumerate() {
        // The steps at aspect ratio `t = h_v / h_u`: the constraint is
        // homogeneous of degree 2 in h_u, so h_u falls straight out.
        // `Q(t) = 0` is a reading — a certified-flat cell constrains
        // nothing and takes one division per axis, which is what an
        // infinite step gives.
        let q = mvv.mul_add(t * t, 2.0f64.mul_add(muv * t, muu));
        assert!(
            q.is_finite(),
            "the certificate overflows at t={t}: a scan this wide cannot be \
             evaluated on muu={muu}, muv={muv}, mvv={mvv}"
        );
        let hu = if q > 0.0 {
            (delta_s / q).sqrt()
        } else {
            f64::INFINITY
        };
        let (hu, hv) = (hu, t * hu);
        let cells = count(du, hu) * count(dv, hv);
        if best.is_none_or(|b| cells < b.cells) {
            best = Some(SplitScan {
                cells,
                steps: (hu, hv),
                sample: Some(k),
            });
        }
    }
    best.expect("a split scan visits at least two aspects")
}

/// The cheapest uniform grid a bound admits over one box: minimize
/// `divisions(U, h_u) · divisions(V, h_v)` subject to the SAME
/// certificate the lane checks,
/// `muu·h_u² + 2·muv·h_u·h_v + mvv·h_v² ≤ δ_s`.
///
/// That constraint is the per-triangle bound `Q/4` at the lane's own
/// two-cells-per-axis budgeting (`a_u ≤ 2h_u`), so a grid found here
/// certifies EXACTLY as the shipped one does — the difference is only
/// which `(h_u, h_v)` on the constraint ellipse gets picked. Since
/// TESS-SPLIT the shipped selection is the cell minimizer under the
/// A = 16 first-fundamental-form aspect cap, so this UNCONSTRAINED
/// optimum differs from it exactly where the cap (or the sliver snap)
/// binds — which is what the split column now measures.
///
/// The lane's own steps are evaluated too, so the answer can never
/// come out worse than what the lane already does.
pub fn best_split_cells(bound: Bound, du: f64, dv: f64, delta_s: f64) -> f64 {
    best_split_steps(bound, du, dv, delta_s).0
}

/// [`best_split_cells`] with the steps it chose: `(cells, h_u, h_v)`.
/// Split out so the constraint can be asserted on the ANSWER and not
/// merely on the formula that produced it (see this crate's tests).
///
/// It is [`split_scan`] over [`shipped_split_scan_aspects`], counted
/// with [`divisions`] and seeded with the lane's own grid so the answer
/// can never come out worse than what the lane already does. Written as
/// that composition and not as its own loop, so the derivations suite
/// can drive the same scan with the same lattice and a different count.
///
/// # Panics
///
/// On a bound or a sizing target that is not a reading — see
/// [`split_scan`] and [`divisions`].
pub fn best_split_steps(bound: Bound, du: f64, dv: f64, delta_s: f64) -> (f64, f64, f64) {
    let best = best_split_scan(bound, du, dv, delta_s);
    (best.cells, best.steps.0, best.steps.1)
}

/// [`best_split_steps`] with the scan's own answer, sample index and
/// all — the SHIPPED composition, named so a test can hold it against
/// the composition it is supposed to be.
///
/// **Boxing the constants is not enough and this is why.** A guard that
/// checks [`shipped_split_scan_aspects`] checks a helper; the three
/// retunes that matter live at the CALL SITE below — a different sample
/// count, a different range, a dropped seed — and each leaves both the
/// constants and the helper untouched. Measured on the shipped `ceil`'d
/// count over 200,000 random bounds, a 21-sample call site alone moves
/// the reported cell count by +14.93% on average and +100% at worst,
/// several times the growth margin `tools/tess-lint` allows.
///
/// **PROVENANCE OF THAT PAIR, since it is what makes the guard below
/// worth its cost.** It was measured once, off-CI, by driving the
/// retuned call site against the shipped one over drawn bounds; nothing
/// re-takes it, no register carries it, and no run would go red if it
/// drifted — a sampling statistic over random bounds is not a property
/// this crate exposes. What IS re-taken is the thing it argued for: the
/// derivations suite pins the composition exactly, on a family chosen so
/// each of the three retunes moves an assertion, and a change anywhere
/// under `tools/` now PINS the k-lint row that runs that suite rather
/// than sampling it (`KLINT_PATH_ROWS` in `scripts/ci-filter.py`). So
/// the number is history and the guard is live, which is the right way
/// round. The figure is stated HERE and nowhere else — the derivations
/// row that holds this composition points at this paragraph rather than
/// restating the pair, so the two cannot part.
///
/// So the derivations suite asserts this function EQUALS
/// `split_scan(bound, du, dv, delta_s, shipped_split_scan_aspects(),
/// Some(bound.steps), divisions)`, bit for bit and sample index
/// included, on bounds that tell the three retunes apart.
#[must_use]
pub fn best_split_scan(bound: Bound, du: f64, dv: f64, delta_s: f64) -> SplitScan {
    split_scan(
        bound,
        du,
        dv,
        delta_s,
        shipped_split_scan_aspects(),
        Some(bound.steps),
        divisions,
    )
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
///
/// # Panics
///
/// On a trim box or a cell box that is not a reading. The overlap test
/// below is an EMPTINESS test and nothing else: written as
/// `!(du > 0.0 && dv > 0.0)` it also swallows a NaN extent, dropping
/// the cell from a sum the gate divides by — which lowers the
/// denominator, raises the reported slack, and so hides a regression
/// wherever it lands in the committed baseline.
fn span_opt_cells(cells: &[CellMeasure], u: (f64, f64), v: (f64, f64), delta_s: f64) -> f64 {
    assert!(
        u.0.is_finite() && u.1.is_finite() && v.0.is_finite() && v.1.is_finite(),
        "a trim box of {u:?} x {v:?} is not a reading: the meter has no per-cell ideal to report"
    );
    let mut opt = 0.0;
    for c in cells {
        assert!(
            c.u.0.is_finite() && c.u.1.is_finite() && c.v.0.is_finite() && c.v.1.is_finite(),
            "a cell box of {:?} x {:?} is not a reading: the meter has no per-cell ideal to report",
            c.u,
            c.v
        );
        let du = c.u.1.min(u.1) - c.u.0.max(u.0);
        let dv = c.v.1.min(v.1) - c.v.0.max(v.0);
        if du <= 0.0 || dv <= 0.0 {
            continue; // cell outside the trim box
        }
        opt += best_split_cells(c.into(), du, dv, delta_s);
    }
    opt
}
