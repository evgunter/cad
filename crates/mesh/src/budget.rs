//! **The tessellation budget meter's kernel half** (issue #320): the
//! per-face measurements only the tessellation lane can observe.
//!
//! Tessellation sizes its grids from CERTIFIED bounds, and a bound is
//! not an estimate — a face can be honestly certified and still carry
//! orders of magnitude more triangles than its deviation needs. #320 is
//! exactly that shape: one lofted leaf at 261,780 triangles beside
//! swept siblings near 900, with nothing wrong anywhere.
//!
//! # What is here, and what is deliberately not
//!
//! Here: the numbers the lane HOLDS and nothing downstream can
//! recover — the trim box, the cell count the schedule actually built,
//! the certified bounds the sizing read, the worst per-triangle
//! certificate, and (when armed for it) the sampled deviation. One
//! struct, handed over once per NURBS face, however that face ended —
//! a REFUSED face is measured too, because a certificate that failed
//! on a mesh the lane then threw away has still failed.
//!
//! Not here: the CSV schema, the slack factors, the counterfactual
//! schedules, the split optimizer, and the reading of any of it. Those
//! are derivations over these measurements plus the body and the mesh,
//! so they need no kernel privileges and they live with the consumer
//! (`tools/tess-meter`, and `docs/TESS-BUDGET.md` for the reading).
//! Faces the meter says nothing about — planar caps, cylinders — get
//! their rows there too, from the body and the mesh, which is where
//! their chart and triangle count already are.
//!
//! **Nothing here asserts at run time**, in either configuration —
//! the only `assert!` in this module is a `const` one in the inert
//! half, evaluated by the compiler and unable to reach a running
//! tessellation. The deviation samples are reduced instead to
//! `worst_ratio` — the largest `|S − Π| / (cert + ε)` any sample
//! reached — which is the per-TRIANGLE falsification stated as one
//! number per face: `worst_ratio ≤ 1` is exactly "every sample was
//! dominated by its own triangle's certificate". The suite that drives
//! the meter asserts on it.
//!
//! **The precise claim, because the sweeping one is false**: no
//! `assert!` in this crate is reachable only under a feature, so no
//! FEATURE can add a panic to the tessellation path. That is not "the
//! path cannot panic" — `NurbsCellGrid::from_cells` asserts its tensor
//! invariant on every NURBS face in every build, and `walk` has an
//! `unreachable!`. Those are unconditional fail-loud kernel-bug
//! assertions, present in and identical across every configuration,
//! which is a different thing from an instrument that a build flag
//! switches on.
//!
//! # Armed, or free — and what a DEFAULT build of this module is
//!
//! Two configurations, and the docs below name items that exist in
//! only one of them. **Without the `budget` feature this module is
//! [`CellMeasure`], [`FaceMeasure`], `armed()` (a `const fn` answering
//! `false`) and `deviation_samples()` (a `const fn` answering `None`)
//! — and nothing else.** There is no `Mode`, no `arm`, no `take`, and
//! so no way for any caller to switch on an instrument that is not in
//! the build. `live::arm` and `live::take` below are the armed half's,
//! and a default build does not have them to link against.
//!
//! **Why the module is `pub` in both configurations, since the
//! opposite was once argued here.** It used to be `pub(crate)` without
//! the feature, on the reasoning that `pub` "would leave a permanently
//! visible surface on the kernel crate whose only consumer is a
//! diagnostic". That reasoning is answered rather than dropped: the
//! surface is now two plain-data structs and two `const fn`s that
//! answer "not in this build", and it is visible **because it is the
//! contract** — `tools/tess-meter` reads [`FaceMeasure`] to derive
//! every column of the budget CSV, and it must do so without the
//! instrument compiled in or depending on it would turn the meter on
//! for everything that depends on IT. The old objection was to
//! exporting an *instrument*; what is exported is a *record type*.
//!
//! A measured face is therefore not a meshed face: the caller's own
//! `tessellate` result says whether the body was built, and `take()`
//! can carry rows for a body that refused. Every consumer in tree
//! reads them after an `Ok`, so nothing downstream changes.
//!
//! Armed, nothing here runs in a normal tessellation: arming is
//! thread-local (tessellation runs on the calling thread, so armed
//! evidence stays attributable under a parallel test runner) and every
//! recording site is behind an [`armed`] check. The measurement
//! changes no mesh: the recorded quantities are read off the sizing
//! the lane already performed, and the deviation pass only samples
//! what was emitted.

use topo::FaceKey;

/// One knot-span cell's certified bound and the grid steps it admits,
/// as the lane's own sizing read them.
///
/// Data only, and compiled either way — the lane builds these only
/// inside an [`armed`] check, so an unarmed build never reaches the
/// code that would allocate one.
#[cfg_attr(not(feature = "budget"), allow(dead_code))]
#[derive(Clone, Copy, Debug)]
pub struct CellMeasure {
    /// The cell's `u` extent.
    pub u: (f64, f64),
    /// The cell's `v` extent.
    pub v: (f64, f64),
    /// `sup ‖S_uu‖` over the cell.
    pub muu: f64,
    /// `sup ‖S_uv‖` over the cell.
    pub muv: f64,
    /// `sup ‖S_vv‖` over the cell.
    pub mvv: f64,
    /// The `(h_u, h_v)` the cell's own bound admits at the face's
    /// sizing target — the schedule's input, reported rather than
    /// re-derived.
    pub steps: (f64, f64),
}

/// One NURBS face's measurements ([module docs](self)).
#[cfg_attr(not(feature = "budget"), allow(dead_code))]
#[derive(Clone, Debug)]
pub struct FaceMeasure {
    /// The face these measurements are about. Keyed rather than
    /// positional: a row attributed to the wrong face is worse than a
    /// missing row.
    pub face: FaceKey,
    /// The trim box the grid spans: `u` extent.
    pub u: (f64, f64),
    /// The trim box the grid spans: `v` extent.
    pub v: (f64, f64),
    /// The per-face deviation target the grid was sized against.
    pub delta_s: f64,
    /// Grid cells the per-cell schedule actually built (`Σ nuc·nvc`
    /// over the clipped bands).
    pub grid_cells: usize,
    /// `sup ‖S_uu‖` of the whole-patch bound.
    pub muu: f64,
    /// `sup ‖S_uv‖` of the whole-patch bound.
    pub muv: f64,
    /// `sup ‖S_vv‖` of the whole-patch bound.
    pub mvv: f64,
    /// The whole-patch bound's `(h_u, h_v)` at `delta_s`.
    pub patch_steps: (f64, f64),
    /// The analysis cells the per-cell bound reported (knot spans for
    /// the integral arm, refined cells for the rational one), in the
    /// order the assembly emits them.
    pub cells: Vec<CellMeasure>,
    /// The largest per-triangle certificate of the attempt the face
    /// exited on. **`0.0` can mean two things**: a genuinely tight
    /// face, or one that refused before the emit pass ever ran (a
    /// failed insert, an empty realised constraint, a self-touching
    /// trim loop) and certified nothing — the caller's own
    /// `tessellate` result is what separates them, and
    /// [`Self::dev_samples`] is `0` in the second case whenever the
    /// meter was armed for deviation.
    pub worst_cert: f64,
    /// The largest SAMPLED `|S − Π|`, or `f64::NAN` when the meter was
    /// not armed for deviation.
    pub worst_dev: f64,
    /// The certificate of the triangle [`Self::worst_dev`] was sampled
    /// on (`f64::NAN` when there was no deviation pass).
    pub worst_dev_cert: f64,
    /// The largest `|S − Π| / (cert + ε)` any sample reached — the
    /// per-triangle falsification as one number ([module docs](self)).
    /// `f64::NAN` when there was no deviation pass.
    ///
    /// **Taken independently of [`Self::worst_dev`]**, so the two need
    /// not come from the same sample: the largest deviation and the
    /// largest ratio-to-its-own-certificate are different questions on
    /// a face whose triangles carry different certificates. Do not
    /// print the `worst_dev` / `worst_dev_cert` pair as "the violating
    /// sample" — it is headroom, and this is the verdict.
    ///
    /// **Accumulated over every attempt the face made**, retries that
    /// were discarded included, unlike [`Self::worst_dev`] and
    /// [`Self::dev_samples`], which describe the attempt the face
    /// exited on. A certificate that failed on a triangle the lane
    /// then threw away has still failed — which is also why a face
    /// that ends in a typed REFUSAL is handed over rather than
    /// dropped: it is the likeliest carrier of a violating sample
    /// there is, and its triangles are all discarded by definition.
    pub worst_ratio: f64,
    /// How many deviation samples [`Self::worst_dev`] is over, on the
    /// attempt the face exited on — the accepted one, or, for a
    /// refusal, the one that lost (0 when not armed for deviation).
    /// [`Self::worst_ratio`] is over more than these — see there.
    pub dev_samples: u64,
}

/// What the meter is armed for. Sizing costs nothing beyond the
/// hand-off; deviation costs a dense resampling of every emitted
/// triangle.
#[cfg(feature = "budget")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Grids, counts, certified bounds — no resampling.
    Sizing,
    /// Also sample `|S − Π|` on every emitted NURBS triangle, at
    /// `samples_per_edge` barycentric samples per triangle edge.
    ///
    /// The density is the caller's, because the two questions want
    /// different ones: 6 is a sizing signal cheap enough for a
    /// quarter-million-triangle face, 12 is the falsifier's own
    /// density and costs ~3.3× that.
    Deviation {
        /// Barycentric samples per triangle edge.
        samples_per_edge: usize,
    },
}

/// **The armed half — compiled only under the `budget` feature.**
///
/// The gate is HERE, at the module boundary, and deliberately not at
/// the call sites: `crate::trimmed` calls [`armed`],
/// [`deviation_samples`] and [`note_face`] unconditionally, and with
/// the feature off those resolve to the `inert` stubs — `armed()` is
/// `false`, so every recording branch folds away and nothing of this
/// module reaches a shipped build. No `#[cfg]` in the tessellation
/// lane means no second version of the lane to keep in step.
///
/// [`arm`] and [`take`] exist ONLY in this half, on purpose: a caller
/// that could arm a meter which records nothing is a fail-quiet, so
/// with the feature off the sweep does not compile rather than
/// producing an empty CSV.
///
/// [`arm`]: arm
/// [`take`]: take
#[cfg(feature = "budget")]
mod live {
    use super::{FaceMeasure, Mode};

    use std::cell::RefCell;

    thread_local! {
        /// This thread's arming and the measurements taken since.
        static STATE: RefCell<Option<State>> = const { RefCell::new(None) };
    }

    /// The armed accumulator (absent = disarmed, which is every normal
    /// tessellation).
    struct State {
        mode: Mode,
        faces: Vec<FaceMeasure>,
    }

    /// Arms the meter on THIS thread, discarding anything unread.
    pub fn arm(mode: Mode) {
        STATE.with(|s| {
            *s.borrow_mut() = Some(State {
                mode,
                faces: Vec::new(),
            });
        });
    }

    /// Is the meter armed on this thread?
    pub fn armed() -> bool {
        STATE.with(|s| s.borrow().is_some())
    }

    /// Barycentric samples per triangle edge the meter wants, or
    /// `None` when it is disarmed or armed for sizing only.
    pub fn deviation_samples() -> Option<usize> {
        STATE.with(|s| {
            s.borrow().as_ref().and_then(|st| match st.mode {
                Mode::Sizing => None,
                Mode::Deviation { samples_per_edge } => Some(samples_per_edge),
            })
        })
    }

    /// Takes the accumulated measurements and DISARMS. Empty when
    /// never armed — which is a real answer ("nothing was
    /// tessellated"), not an error.
    pub fn take() -> Vec<FaceMeasure> {
        STATE.with(|s| s.borrow_mut().take().map(|st| st.faces).unwrap_or_default())
    }

    /// The lane's one hand-off: this face's measurements, once,
    /// whichever way the face ended.
    pub(crate) fn note_face(m: FaceMeasure) {
        STATE.with(|s| {
            if let Some(st) = s.borrow_mut().as_mut() {
                st.faces.push(m);
            }
        });
    }
}

/// **The disarmed half — compiled when the `budget` feature is off**,
/// which is every shipped build and every default `cargo test`.
///
/// [`armed`][inert::armed] is a constant `false` and
/// [`deviation_samples`][inert::deviation_samples] a constant `None`,
/// so the optimizer deletes the recording branches in `crate::trimmed`
/// outright — the meter costs nothing, structurally, rather than by
/// argument. [`note_face`][inert::note_face] keeps its signature so the
/// call site is shared with the live half.
#[cfg(not(feature = "budget"))]
mod inert {
    use super::FaceMeasure;

    /// Always false: the meter is not in this build.
    ///
    /// INVARIANT: `const fn`, so "folds away" is a fact the compiler
    /// holds at every call site rather than a hope about inlining —
    /// checked by the static assertion below.
    pub const fn armed() -> bool {
        false
    }

    /// Always `None` — and it is what removes the trimmed lane's
    /// per-triangle resampling block from a shipped build.
    pub const fn deviation_samples() -> Option<usize> {
        None
    }

    /// The gate, proved at COMPILE TIME: this item fails to build if
    /// either stub above ever stops folding to its constant.
    const _: () = assert!(!armed() && deviation_samples().is_none());

    /// No-op. Unreachable in this build (its one call site is behind
    /// [`armed`]); it exists so the lane has one shared call site
    /// instead of a `#[cfg]`.
    pub(crate) fn note_face(_m: FaceMeasure) {}
}

#[cfg(not(feature = "budget"))]
pub use inert::*;
#[cfg(feature = "budget")]
pub use live::*;
