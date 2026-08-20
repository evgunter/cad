//! **The tessellation budget meter** (issue #320): where a mesh's
//! triangles actually go, and how much of the deviation budget that
//! bought.
//!
//! Tessellation sizes its grids from CERTIFIED bounds, and a bound is
//! not an estimate — a face can be honestly certified and still carry
//! orders of magnitude more triangles than its deviation needs. #320 is
//! exactly that shape: one lofted leaf at 261,780 triangles beside
//! swept siblings near 900, with nothing wrong anywhere. This module is
//! the instrument that tells the two cases apart, so "we are
//! over-tessellating" stops being an impression and becomes a number
//! with a decomposition.
//!
//! # Armed, or free
//!
//! Nothing here runs in a normal tessellation: [`arm`] is thread-local
//! (`probe_stats`' rule, same reason — tessellation runs on the calling
//! thread, so armed evidence stays attributable under a parallel test
//! runner) and every recording site is behind an [`armed`] check. The
//! measurement changes no mesh: the recorded quantities are read off
//! the sizing the lane already performed.
//!
//! # The slack factors (re-derived at TESS-SPAN)
//!
//! Per face the meter records what the lane used (the grid, the
//! triangle count, the whole-patch Hessian bound) and what it could
//! have used. The lint arithmetic is deliberately kept OUT of the
//! kernel — the row carries measurements, `tools/tess-lint` names the
//! findings.
//!
//! **TESS-SPAN moved the shipped schedule onto per-knot-span-cell
//! sizing**, so the columns were re-derived to keep both regression
//! kinds visible (spec D-4). The shipped grid is now `grid_cells`,
//! read off the sizing the lane actually ran; `patch_cells` is the
//! retired whole-patch-sup schedule kept as a COUNTERFACTUAL column
//! (the live half recomputes it from `nurbs_face_bound`), so the gain
//! the promotion holds stays a number and a silent revert to
//! whole-patch sizing cannot hide; `span_cells` is the schedule's
//! cell count summed through the SAME `band_schedule` derivation the
//! lane consumes — so the agreement column verifies the lane's
//! REALISATION of the schedule (candidate generation, dedup,
//! counting), not an independent re-derivation. What actually guards
//! `band_schedule` itself: the per-triangle certificate (which reads
//! the raw per-cell bounds and turns an undersizing bug into a typed
//! refusal), the diff-gate on growth, and the committed render cells.
//! The stated blind spot: a schedule bug that makes the grid COARSER
//! while still certifying is invisible to the growth-only gate and to
//! `agree` (docs/TESS-BUDGET.md records it).
//!
//! **Where those guards live, named — and what they do NOT cover**
//! (issue #667's Q6). `ci.yml`'s `k-lint (gate)` job runs `mesh budget
//! meter (feature = budget)` and `mesh certificate falsifier
//! (feature = probe-stats)` — the two gated halves of this module and
//! `crate::probe_stats`, which the default `cargo test -p mesh` row
//! cannot reach — then re-tessellates the whole tour with
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
//! | factor | ratio | what it says |
//! |---|---|---|
//! | **span held** | `patch_cells / grid_cells` | the gain TESS-SPAN holds over whole-patch-sup sizing. Falls toward 1.0 if the shipped schedule regresses toward the patch sup. |
//! | **agreement** | `grid_cells / span_cells` | 1.0 by construction — the lane's realised cell count against the same schedule's sum. Drift means candidate generation/dedup/counting no longer realise the schedule. |
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
//! slack**: the factor
//! by which the deviation budget went unspent. Because a triangle's
//! deviation scales like `h²` and its count like `1/h²`, a
//! first-order extrapolation of "what an oracle-sized uniform grid
//! would have cost" is `triangles · worst_dev / delta`.
//!
//! **That extrapolation is an estimate, and the row says so**: it holds
//! only while the local Hessian is what it is, so it is a sizing
//! signal, never a bound. `worst_dev` is likewise a SAMPLED sup
//! (barycentric samples per triangle, [`DEV_SAMPLES`] per edge), so it
//! under-reports the true deviation and therefore over-reports the
//! available saving. The counted-grid columns carry no such caveat:
//! they are computed from certified bounds and the lane's own
//! `grid_steps` rule, with each cell's `ceil` paid honestly.
//!
//! # What is measured for which chart
//!
//! Every face gets a row (chart kind + triangle count) — the question
//! "which face IS the scene's cost" needs the planar and cylinder rows
//! to be answerable. Only the NURBS lane fills the sizing columns:
//! it is the lane whose grid is Hessian-sized, and the one #320 is
//! about. Non-NURBS rows leave those columns empty rather than
//! reporting a zero that would read as a measurement.

/// Barycentric samples per triangle edge when the meter drives the
/// deviation sampling itself (the review probe's own arming uses its
/// denser 12 and its assertion — see `crate::trimmed`). 6 is a
/// judgment call about cost: the leaf of #320 is a quarter-million
/// triangles, and 28 samples each is already 7M surface evaluations.
pub const DEV_SAMPLES: usize = 6;

/// What the meter is armed for. Sizing columns are free (they are read
/// off the sizing the lane already did); deviation costs a dense
/// resampling of every emitted triangle.
//
// Gated with the rest of the ROW types (`NurbsBudget`, `FaceBudget`,
// the CSV): only an armed meter has a mode to be in, and only the live
// half produces rows, so none of this is data a shipped build has any
// use for. What stays ungated is exactly what the tessellation lane's
// shared call sites name — [`Chart`], [`Sizing`], [`DEV_SAMPLES`].
#[cfg(feature = "budget")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Grids, counts, Hessian bounds, per-cell sizing — no resampling.
    Sizing,
    /// Also sample `|S − Π|` per triangle to fill `worst_dev`.
    SizingAndDeviation,
}

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

#[cfg(feature = "budget")]
impl Chart {
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
#[cfg(feature = "budget")]
#[derive(Clone, Copy, Debug)]
pub struct NurbsBudget {
    /// The trim box the grid spans: `u` extent.
    pub u: (f64, f64),
    /// The trim box the grid spans: `v` extent.
    pub v: (f64, f64),
    /// COUNTERFACTUAL since TESS-SPAN: the grid divisions the retired
    /// whole-patch-sup schedule would use over the trim box, `u`
    /// direction (f64 — computed by the meter's `divisions`, no longer
    /// a count the lane held).
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
    /// integral arm, refined cells for the rational one —
    /// `nurbs_cell_bounds`).
    pub cells: usize,
    /// The grid the lane ACTUALLY built (TESS-SPAN: per-cell sizing),
    /// as a cell count read off the sizing hand-off.
    pub grid_cells: f64,
    /// `nu · nv` — the whole-patch counterfactual's cell count (the
    /// pre-TESS-SPAN `uniform_cells` column, kept so the held span
    /// gain stays a number — module docs).
    pub patch_cells: f64,
    /// The cheapest uniform grid the SAME whole-patch bound admits,
    /// over the same box (rides with the counterfactual).
    pub opt_cells: f64,
    /// The schedule's cell count summed through the SAME
    /// `band_schedule` derivation the lane consumes (module docs: the
    /// agreement denominator verifies realisation, not an independent
    /// re-derivation).
    pub span_cells: f64,
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

/// What the NURBS lane sized, handed to the meter in one piece.
///
/// A struct rather than loose positional parameters: the meter's
/// hand-off is the one place the lane's whole sizing decision crosses
/// a module boundary, and `(u, v, grid_cells, delta_s)` as loose
/// arguments is a swap waiting to happen between two `(f64, f64)`
/// pairs.
///
/// The lane BUILDS one either way — the call site is shared — and the
/// inert half then reads none of its fields, which is honest dead code
/// rather than a `#[cfg]` at the call site.
#[cfg_attr(not(feature = "budget"), allow(dead_code))]
#[derive(Clone, Copy, Debug)]
pub(crate) struct Sizing {
    /// The trim box's `u` extent.
    pub u: (f64, f64),
    /// The trim box's `v` extent.
    pub v: (f64, f64),
    /// Grid cells the per-cell schedule actually built (`Σ nuc·nvc`
    /// over the clipped cells — TESS-SPAN).
    pub grid_cells: usize,
    /// The per-face deviation target the grid was sized against.
    pub delta_s: f64,
}

/// One face's budget row.
#[cfg(feature = "budget")]
#[derive(Clone, Copy, Debug)]
pub struct FaceBudget {
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
    pub nurbs: Option<NurbsBudget>,
}

/// The CSV header the sweep writes and `tools/tess-lint` reads. One
/// definition, both sides.
#[cfg(feature = "budget")]
pub const CSV_HEADER: &str = "scene,face,chart,delta,triangles,u0,u1,v0,v1,nu,nv,\
                              muu,muv,mvv,cells,grid_cells,patch_cells,opt_cells,\
                              span_cells,span_opt_cells,worst_cert,worst_dev,dev_samples";

/// How many of [`CSV_HEADER`]'s columns are the NURBS lane's — every
/// column after `triangles`, which is the last one every row fills.
#[cfg(feature = "budget")]
fn nurbs_column_count() -> usize {
    CSV_HEADER.split(',').count()
        - CSV_HEADER
            .split(',')
            .take_while(|c| *c != "triangles")
            .count()
        - 1
}

#[cfg(feature = "budget")]
impl FaceBudget {
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
                 {:e},{:e},{:e},{:e},{:e},{:e},{:e},{}",
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
                n.span_cells,
                n.span_opt_cells,
                n.worst_cert,
                n.worst_dev,
                n.dev_samples
            ),
        }
    }
}

/// **The armed half — compiled only under the `budget` feature.**
///
/// The gate is HERE, at the module boundary, and deliberately not at
/// the call sites: `crate::tessellate` and `crate::trimmed` call
/// [`armed`] and the recorders unconditionally, and with the feature
/// off those resolve to the `inert` stubs — `armed()` is `false`, so
/// every recording branch folds away and nothing of this module
/// reaches a shipped build. No `#[cfg]` in the tessellation lane means
/// no second version of the lane to keep in step.
///
/// [`arm`] and [`take`] exist ONLY in this half, on purpose: a caller
/// that could arm a meter which records nothing is a fail-quiet, so
/// with the feature off the sweep does not compile rather than
/// producing an empty CSV (the `k-probe` mode's posture, whose gate
/// this mirrors).
#[cfg(feature = "budget")]
mod live {
    use geom_surfaces::NurbsSurface;
    use topo::FaceKey;

    use super::{Chart, FaceBudget, Mode, NurbsBudget, Sizing};
    use crate::nurbs_cert::{CellBound, NurbsFaceBound, nurbs_cell_bounds};
    use crate::types::TessellateError;

    use std::cell::RefCell;

    thread_local! {
        /// This thread's arming, completed rows, and the face in progress.
        static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
    }

    /// The armed accumulator (absent = disarmed, which is every normal
    /// tessellation).
    struct State {
        mode: Mode,
        rows: Vec<FaceBudget>,
        pending: Option<NurbsBudget>,
    }

    /// Arms the meter on THIS thread, discarding anything unread.
    pub fn arm(mode: Mode) {
        STATE.with(|s| {
            *s.borrow_mut() = Some(State {
                mode,
                rows: Vec::new(),
                pending: None,
            });
        });
    }

    /// Is the meter armed on this thread?
    pub fn armed() -> bool {
        STATE.with(|s| s.borrow().is_some())
    }

    /// Is the meter armed for the (expensive) deviation resampling?
    pub fn deviation_armed() -> bool {
        STATE.with(|s| {
            s.borrow()
                .as_ref()
                .is_some_and(|st| st.mode == Mode::SizingAndDeviation)
        })
    }

    /// Takes the accumulated rows and DISARMS. Empty when never armed —
    /// which is a real answer ("nothing was tessellated"), not an error.
    pub fn take() -> Vec<FaceBudget> {
        STATE.with(|s| s.borrow_mut().take().map(|st| st.rows).unwrap_or_default())
    }

    /// The NURBS lane's sizing hand-off, called once per face before
    /// the grid is built: computes the per-cell prediction and the
    /// whole-patch counterfactual (module docs) and holds them until
    /// [`finish_face`] closes the row.
    ///
    /// Both analysis arms — `nurbs_cell_bounds` for the prediction and
    /// `nurbs_face_bound` for the counterfactual — are called from
    /// HERE rather than from the lane, so that everything the meter
    /// needs compiles only in this half; since TESS-SPAN the lane
    /// itself never computes the whole-patch bound. The lane's call
    /// site is one line and identical in both halves.
    ///
    /// # Errors
    ///
    /// As `nurbs_cell_bounds`/`nurbs_face_bound` — the same gates the
    /// lane's own cell grid passed, so a face that tessellates cannot
    /// fail here.
    pub(crate) fn note_nurbs_sizing(
        payload: &NurbsSurface<f64>,
        fk: FaceKey,
        sizing: Sizing,
    ) -> Result<(), TessellateError> {
        if !armed() {
            return Ok(());
        }
        let Sizing {
            u,
            v,
            grid_cells,
            delta_s,
        } = sizing;
        let cells = &nurbs_cell_bounds(payload, fk)?;
        let cell_grid = crate::nurbs_cert::nurbs_cell_grid(payload, fk)?;
        let bound = crate::nurbs_cert::nurbs_face_bound(payload, fk)?;
        let (span_cells, span_opt_cells) =
            span_sized_cells(cells, &cell_grid, bound, u, v, delta_s)?;
        STATE.with(|s| {
            let mut s = s.borrow_mut();
            let Some(st) = s.as_mut() else { return };
            let (du, dv) = (u.1 - u.0, v.1 - v.0);
            // The retired whole-patch schedule, re-run as the
            // counterfactual (TESS-SPAN D-4): same steps, same ceil.
            let (phu, phv) = bound.grid_steps(delta_s);
            let (nu, nv) = (divisions(du, phu), divisions(dv, phv));
            #[allow(clippy::cast_precision_loss)]
            let grid_cells = grid_cells as f64;
            st.pending = Some(NurbsBudget {
                u,
                v,
                nu,
                nv,
                muu: bound.muu,
                muv: bound.muv,
                mvv: bound.mvv,
                cells: cells.len(),
                grid_cells,
                patch_cells: nu * nv,
                opt_cells: best_split_cells(bound, du, dv, delta_s),
                span_cells,
                span_opt_cells,
                worst_cert: f64::NAN,
                worst_dev: f64::NAN,
                dev_samples: 0,
            });
        });
        Ok(())
    }

    /// The NURBS lane's per-face certificate hand-off (the max over the
    /// triangles it emitted), called once the face's emit pass has
    /// settled.
    pub(crate) fn note_worst_cert(cert: f64) {
        STATE.with(|s| {
            if let Some(p) = s.borrow_mut().as_mut().and_then(|st| st.pending.as_mut()) {
                p.worst_cert = cert;
            }
        });
    }

    /// Drops the face-in-progress's deviation samples, called when the
    /// lane restarts its emit pass (the grid-on-constraint retry). Without
    /// this, a discarded attempt's triangles would contribute to a
    /// `worst_dev` that is supposed to describe the mesh that was KEPT.
    pub(crate) fn reset_face_dev() {
        STATE.with(|s| {
            if let Some(p) = s.borrow_mut().as_mut().and_then(|st| st.pending.as_mut()) {
                p.worst_dev = f64::NAN;
                p.dev_samples = 0;
            }
        });
    }

    /// One sampled `|S − Π|`. Called from the trimmed lane's sampling
    /// block, which the review probe and this meter share.
    pub(crate) fn note_dev(d: f64) {
        STATE.with(|s| {
            if let Some(p) = s.borrow_mut().as_mut().and_then(|st| st.pending.as_mut()) {
                // A first sample replaces the NaN seed; after that, max.
                if p.dev_samples == 0 || d > p.worst_dev {
                    p.worst_dev = d;
                }
                p.dev_samples += 1;
            }
        });
    }

    /// Opens one face, discarding any sizing left over from the
    /// previous one.
    ///
    /// [`finish_face`] normally consumes the pending sizing, but a face
    /// whose lane REFUSES never reaches it — `tessellate` propagates
    /// the error with `?` from inside the dispatch loop — and the
    /// leftover would then be attached to whichever face closed next,
    /// reporting one face's grid under another face's name. Clearing on
    /// entry makes each row depend only on its own face's fate, which
    /// is the property a diagnostic has to have to be believed.
    pub(crate) fn begin_face() {
        STATE.with(|s| {
            if let Some(st) = s.borrow_mut().as_mut() {
                st.pending = None;
            }
        });
    }

    /// Closes one face's row (called from the dispatcher, which is the one
    /// place that knows every lane's triangle count). A face whose lane
    /// noted nothing gets a row with empty sizing columns.
    pub(crate) fn finish_face(face: usize, chart: Chart, delta: f64, triangles: usize) {
        STATE.with(|s| {
            let mut s = s.borrow_mut();
            let Some(st) = s.as_mut() else { return };
            let nurbs = st.pending.take();
            st.rows.push(FaceBudget {
                face,
                chart,
                delta,
                triangles,
                nurbs,
            });
        });
    }

    /// Grid divisions an extent needs at step `h`. An unconstrained
    /// direction (`h = ∞`, e.g. the ruled direction of a wall with
    /// `muv = 0`) takes one, exactly as `ceil_count` gives the shipped
    /// path.
    pub(super) fn divisions(extent: f64, h: f64) -> f64 {
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
    const SPLIT_SCAN_DECADES: f64 = 8.0;
    /// Samples per scan (fixed, so the answer is deterministic — D9).
    const SPLIT_SCAN_STEPS: usize = 321;

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
    pub(super) fn best_split_cells(bound: NurbsFaceBound, du: f64, dv: f64, delta_s: f64) -> f64 {
        best_split_steps(bound, du, dv, delta_s).0
    }

    /// [`best_split_cells`] with the steps it chose: `(cells, h_u, h_v)`.
    /// Split out so the constraint can be asserted on the ANSWER and not
    /// merely on the formula that produced it (see this module's tests).
    pub(super) fn best_split_steps(
        bound: NurbsFaceBound,
        du: f64,
        dv: f64,
        delta_s: f64,
    ) -> (f64, f64, f64) {
        let (muu, muv, mvv) = (bound.muu, bound.muv, bound.mvv);
        // The steps at aspect ratio `t = h_v / h_u`: the constraint is
        // homogeneous of degree 2 in h_u, so h_u falls straight out.
        let steps = |t: f64| -> (f64, f64) {
            // `powi(2)`, not `t * t`: the discipline the CI lint
            // enforces tree-wide (ci.yml's "interval-square powi(2)
            // allowlist").
            // These are f64, so the two agree bit for bit — which is
            // exactly why there is no reason to be the exception.
            let q = mvv.mul_add(t.powi(2), 2.0f64.mul_add(muv * t, muu));
            let hu = if q > 0.0 {
                (delta_s / q).sqrt()
            } else {
                f64::INFINITY
            };
            (hu, t * hu)
        };
        let (lane_u, lane_v) = bound.grid_steps(delta_s);
        let mut best = (
            divisions(du, lane_u) * divisions(dv, lane_v),
            lane_u,
            lane_v,
        );
        for k in 0..SPLIT_SCAN_STEPS {
            #[allow(clippy::cast_precision_loss)]
            let f = k as f64 / (SPLIT_SCAN_STEPS - 1) as f64;
            let (hu, hv) = steps(10.0f64.powf(SPLIT_SCAN_DECADES * f.mul_add(2.0, -1.0)));
            let n = divisions(du, hu) * divisions(dv, hv);
            if n < best.0 {
                best = (n, hu, hv);
            }
        }
        best
    }

    /// The shipped schedule's cell count and the pure per-cell ideal,
    /// over the trim box.
    ///
    /// Returns `(lane, cheapest split)`:
    ///
    /// * `lane` sums the SAME [`crate::nurbs_cert::NurbsCellGrid::band_schedule`] the
    ///   lane consumes — deliberately one derivation, so the agreement
    ///   column verifies the lane's REALISATION of it (candidate
    ///   generation, dedup, counting), and a bug in `band_schedule`
    ///   itself is caught elsewhere (module docs: certificate refusal,
    ///   growth gate, committed render cells).
    /// * `opt` keeps each cell's own RAW bound through
    ///   [`best_split_cells`], clipped to the box with the `ceil` paid
    ///   per cell — the pure per-cell-and-cheapest-split ideal, which
    ///   is the recoverable-slack denominator; the banding's
    ///   max-across-u cost stays visible in it.
    ///
    /// **Why aligning rows to the band cuts is enough for the
    /// certificate**: with rows on the band boundaries, every grid
    /// triangle's UV box lies inside one band, so the band's own
    /// certified bounds are the ones its certificate uses. Cells are
    /// half-open — a knot is exactly where a C¹ surface's second
    /// derivative jumps — but the shared boundary is measure-zero, and
    /// the Taylor remainder the certificate is built on needs only an
    /// a.e. bound. That is the same fact the shipped whole-patch
    /// assembly already rests on at its own interior knots
    /// (`nurbs_cert` docs).
    pub(super) fn span_sized_cells(
        cells: &[CellBound],
        grid: &crate::nurbs_cert::NurbsCellGrid,
        patch: NurbsFaceBound,
        u: (f64, f64),
        v: (f64, f64),
        delta_s: f64,
    ) -> Result<(f64, f64), TessellateError> {
        // The shipped schedule — the ONE derivation the lane also
        // consumes (`NurbsCellGrid::band_schedule`), so the agreement
        // column verifies the lane's REALISATION of the schedule
        // (candidate generation, dedup, counting) rather than
        // re-deriving the arithmetic.
        let mut lane = 0.0;
        for b in grid.band_schedule(patch, u, v, delta_s)? {
            #[allow(clippy::cast_precision_loss)]
            {
                lane += (b.nuc * b.nvc) as f64;
            }
        }
        // The pure per-cell ideal at the cheapest split per cell.
        let mut opt = 0.0;
        for c in cells {
            let du = c.u.1.min(u.1) - c.u.0.max(u.0);
            let dv = c.v.1.min(v.1) - c.v.0.max(v.0);
            if !(du > 0.0 && dv > 0.0) {
                continue; // cell outside the trim box
            }
            opt += best_split_cells(c.bound, du, dv, delta_s);
        }
        Ok((lane, opt))
    }
}

/// **The disarmed half — compiled when the `budget` feature is off**,
/// which is every shipped build and every default `cargo test`.
///
/// `armed()` is a constant `false`, so the optimizer deletes the
/// recording branches in `tessellate` and `trimmed` outright — the
/// meter costs nothing, structurally, rather than by argument. The
/// recorders keep their signatures so the call sites are shared with
/// the live half; the types they take are compiled either way (they are
/// data, and unreferenced data costs nothing).
#[cfg(not(feature = "budget"))]
mod inert {
    use geom_surfaces::NurbsSurface;
    use topo::FaceKey;

    use super::{Chart, Sizing};
    use crate::types::TessellateError;

    /// Always false: the meter is not in this build.
    ///
    /// INVARIANT: `const fn`, so "folds away" is a fact the compiler
    /// holds at every call site rather than a hope about inlining —
    /// checked by the static assertion below (#558's shape, adopted
    /// here so the standing telemetry rule is enforced for BOTH gated
    /// instruments and not just the newer one).
    pub const fn armed() -> bool {
        false
    }

    /// Always false — and it is what removes the trimmed lane's
    /// per-triangle resampling block from a shipped build.
    pub const fn deviation_armed() -> bool {
        false
    }

    /// The gate, proved at COMPILE TIME: this item fails to build if
    /// either stub above ever stops folding to `false`.
    const _: () = assert!(!armed() && !deviation_armed());

    /// No-op (see the module docs). Keeps the `Result` so the lane's
    /// single call site is identical in both halves — and so the `?`
    /// there stays honest about the live half, which really can refuse.
    ///
    /// # Errors
    ///
    /// Never, in this half.
    pub(crate) fn note_nurbs_sizing(
        _payload: &NurbsSurface<f64>,
        _fk: FaceKey,
        _sizing: Sizing,
    ) -> Result<(), TessellateError> {
        Ok(())
    }

    /// No-op.
    pub(crate) fn note_worst_cert(_cert: f64) {}

    /// No-op.
    pub(crate) fn note_dev(_d: f64) {}

    /// No-op.
    pub(crate) fn reset_face_dev() {}

    /// No-op.
    pub(crate) fn begin_face() {}

    /// No-op.
    pub(crate) fn finish_face(_face: usize, _chart: Chart, _delta: f64, _triangles: usize) {}
}

#[cfg(not(feature = "budget"))]
pub use inert::*;
#[cfg(feature = "budget")]
pub use live::*;

#[cfg(all(test, feature = "budget"))]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::live::{best_split_cells, best_split_steps, divisions};
    use super::*;
    use crate::nurbs_cert::NurbsFaceBound;
    use test_utils::fuzz;

    /// The claim the whole "split slack" column rests on: the grid
    /// [`best_split_steps`] picks satisfies the SAME certificate the
    /// lane checks, `muu·h_u² + 2·muv·h_u·h_v + mvv·h_v² ≤ δ_s`.
    ///
    /// That inequality is the per-triangle bound `Q/4` at the lane's
    /// own two-cells-per-axis budgeting (`a_u ≤ 2h_u`, `a_v ≤ 2h_v`),
    /// so a grid satisfying it certifies exactly as the shipped one
    /// does. Asserted on the ANSWER over random bounds — including the
    /// degenerate corners (a ruled direction's `muu = 0`, a zero cross
    /// term) where the closed-form optimum does not exist.
    #[test]
    fn the_cheapest_split_still_satisfies_the_certificate() {
        let mut rng = fuzz::start("budget::cheapest_split");
        // Log-uniform magnitudes, with a zero in each slot often
        // enough to exercise the degenerate corners (a ruled
        // direction's muu = 0, a dead cross term).
        fn mag(r: &mut fuzz::Rng) -> f64 {
            if r.unit() < 0.2 {
                0.0
            } else {
                10.0f64.powf(r.range(-6.0, 4.0))
            }
        }
        for _ in 0..fuzz::scaled(400) {
            let bound = NurbsFaceBound {
                muu: mag(&mut rng),
                muv: mag(&mut rng),
                mvv: mag(&mut rng),
            };
            let (du, dv) = (
                10.0f64.powf(rng.range(-2.0, 2.0)),
                10.0f64.powf(rng.range(-2.0, 2.0)),
            );
            let delta_s = 10.0f64.powf(rng.range(-6.0, -1.0));
            let (cells, hu, hv) = best_split_steps(bound, du, dv, delta_s);
            // An infinite step means "one division", which is the
            // whole extent — that is what the certificate must be
            // checked at, not at ∞.
            let (hu, hv) = (hu.min(du), hv.min(dv));
            let q = bound.muu * hu.powi(2) + 2.0 * bound.muv * hu * hv + bound.mvv * hv.powi(2);
            assert!(
                q <= delta_s * (1.0 + 1e-9),
                "cheapest split violates the certificate: q={q:e} > delta_s={delta_s:e} \
                 at h=({hu:e},{hv:e}) for {bound:?} — {}",
                fuzz::replay()
            );
            // …and it never loses to the schedule it is compared with.
            let (lu, lv) = bound.grid_steps(delta_s);
            assert!(
                cells <= divisions(du, lu) * divisions(dv, lv),
                "cheapest split is worse than the lane's for {bound:?} — {}",
                fuzz::replay()
            );
        }
    }

    /// The scan is a fixed grid of aspect ratios (D9: structure, never
    /// a data-dependent iteration), so the answer is reproducible.
    #[test]
    fn the_split_scan_is_deterministic() {
        let bound = NurbsFaceBound {
            muu: 0.0,
            muv: 2.4,
            mvv: 51.3,
        };
        let a = best_split_cells(bound, 1.0, 1.0, 1e-3);
        let b = best_split_cells(bound, 1.0, 1.0, 1e-3);
        assert_eq!(a.to_bits(), b.to_bits());
    }

    /// A ruled wall is the case #320 is about: `muu = 0` with a live
    /// cross term. The lane's symmetric split charges the cross term
    /// to BOTH directions and grids the flat one anyway; the cheapest
    /// split spends its divisions where the curvature is.
    #[test]
    fn a_ruled_wall_pays_for_its_flat_direction() {
        let bound = NurbsFaceBound {
            muu: 0.0,
            muv: 2.4,
            mvv: 51.3,
        };
        let (lu, lv) = bound.grid_steps(1e-3);
        let lane = divisions(1.0, lu) * divisions(1.0, lv);
        let (best, hu, _) = best_split_steps(bound, 1.0, 1.0, 1e-3);
        assert!(
            lane / best > 3.0,
            "expected the ruled wall's split slack to be several-fold, got {:.2}x",
            lane / best
        );
        assert!(
            hu >= 1.0,
            "the cheapest split should stop dividing the FLAT direction, got h_u = {hu:e}"
        );
    }

    /// The empty-tail arm and the filled arm must agree about the
    /// row's width, or every consumer's column indices are off by the
    /// difference.
    #[test]
    fn both_row_shapes_have_the_headers_width() {
        let cols = CSV_HEADER.split(',').count();
        let plane = FaceBudget {
            face: 0,
            chart: Chart::Plane,
            delta: 1e-3,
            triangles: 2,
            nurbs: None,
        };
        assert_eq!(plane.csv_row("s/b").split(',').count(), cols);
        let nurbs = FaceBudget {
            chart: Chart::Nurbs,
            nurbs: Some(NurbsBudget {
                u: (0.0, 1.0),
                v: (0.0, 1.0),
                nu: 4.0,
                nv: 5.0,
                muu: 1.0,
                muv: 2.0,
                mvv: 3.0,
                cells: 6,
                grid_cells: 12.0,
                patch_cells: 20.0,
                opt_cells: 10.0,
                span_cells: 12.0,
                span_opt_cells: 8.0,
                worst_cert: 1e-4,
                worst_dev: 5e-5,
                dev_samples: 7,
            }),
            ..plane
        };
        assert_eq!(nurbs.csv_row("s/b").split(',').count(), cols);
    }
}
