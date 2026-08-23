//! The PATHS profile-authoring lattice, bound state-for-state.
//!
//! The lattice renders as DISTINCT classes, each
//! exposing only its state's legal continuations, so an off-lattice
//! call is an `AttributeError` — the runtime shadow of the Rust
//! typestate's E0599. There are no `isinstance` ladders and no runtime
//! state flags; the class IS the state.
//!
//! The Python layer re-implements NOTHING. Every verb clones its
//! `PartialPath` and calls the SAME generic Rust method, so geometry
//! refusals fire at the call site as the same typed `PathError` the
//! Rust surface returns (§2.4 of the unit spec). Cloning is the
//! documented fork the algebra already permits (`PartialPath` derives
//! `Clone` for motif exploration); it mints no new closure door,
//! because every lowered result still passes the verify layer.
//!
//! One class per Rust type, with two deliberate identifications:
//! `PathDirected` carries BOTH `Directed` flavors (`HasPos<Plain>` and
//! `HasPos<WithIncoming>`), because the Rust impl that gives them
//! their methods is one impl generic over the flavor — the method sets
//! are the same set, so splitting the class would invent an asymmetry
//! Rust does not have; and the entry `Open` is its own class, distinct
//! from a fillet's arrival `PathOpen`, because Rust's `Open` genuinely
//! lacks `.to(Start)` (there is no `Start` yet at the entry).

use pyo3::prelude::*;
use pyo3::types::PyString;

use pncad::document as d;
use pncad::geom_core::Point2;
use pncad::profile as pf;
use pncad::profile::path::{HasAng, HasPos, NoAng, NoPos, Plain, WithIncoming};

use super::quantity::{Angle, Length};
use super::typed_err;
use crate::errors::ErrorClass;
use crate::tags::{path_error_tag, recorded_program_error_tag};
use pncad::tolerance::Tol;

/// The lattice's runtime value, at one state.
type Path<P, A> = pf::PartialPath<f64, P, A>;
type KPathError = pf::PathError<f64>;

/// The kernel's refusal, raised where the verb was written.
fn path_err(py: Python<'_>, err: &KPathError) -> PyErr {
    typed_err(
        py,
        ErrorClass::Path,
        err.to_string(),
        &[(
            "variant",
            PyString::new(py, path_error_tag(err)).unbind().into_any(),
        )],
    )
}

/// A `(Length, Length)` argument as the profile-frame point it is.
fn pt(p: (Length, Length)) -> Point2<f64> {
    Point2::new(p.0.0.meters(), p.1.0.meters())
}

// ------------------------------------------------------------------
// Tokens and results
// ------------------------------------------------------------------

/// The type of the `Start` token — the bound entry, as a value.
///
/// `Start` is a first-class directed point, legal wherever a target
/// goes, and USING it is closing, structurally: the endpoint IS the
/// start point by reference, so closure never depends on re-typed
/// coordinates value-matching. There is deliberately no `close()`.
#[pyclass(frozen, module = "pncad", from_py_object, name = "StartToken")]
#[derive(Clone, Copy)]
pub(crate) struct StartToken;

#[pymethods]
impl StartToken {
    fn __repr__(&self) -> &'static str {
        "Start"
    }
}

/// Travel sense about a centre — structural, never a value.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ArcSweep {
    /// Counterclockwise (positive included angle; positive bulge).
    Ccw,
    /// Clockwise (negative included angle; negative bulge).
    Cw,
}

impl ArcSweep {
    fn to_kernel(self) -> pf::ArcSweep {
        match self {
            Self::Ccw => pf::ArcSweep::Ccw,
            Self::Cw => pf::ArcSweep::Cw,
        }
    }
}

/// Which half-plane of the departure tangent a DERIVED carrier centre
/// sits on — structural, the one discrete bit the endpoint-free and
/// radius modes carry. `Left` of travel curves the arc counterclockwise.
#[pyclass(eq, eq_int, module = "pncad", from_py_object)]
#[derive(Clone, Copy, PartialEq)]
pub(crate) enum ArcSide {
    /// Centre on the left of travel (counterclockwise arc).
    Left,
    /// Centre on the right of travel (clockwise arc).
    Right,
}

impl ArcSide {
    fn to_kernel(self) -> pf::ArcSide {
        match self {
            Self::Left => pf::ArcSide::Left,
            Self::Right => pf::ArcSide::Right,
        }
    }
}

/// A closed loop and the program that produced it.
///
/// Closing verbs return this; `Node.profile` builds the document node
/// from the RECORDED program, so what Python authored and what the
/// document replays are the same steps, not two spellings.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct ClosedLoop(pub(crate) pf::ClosedLoop<f64>);

#[pymethods]
impl ClosedLoop {
    /// How many vertices the lowered loop has — the arc's two tangent
    /// points included, the trimmed-away virtual corner not.
    #[getter]
    fn vertex_count(&self) -> usize {
        self.0.loop_.vertices().len()
    }

    /// How many steps the recorded program has.
    #[getter]
    fn step_count(&self) -> usize {
        self.0.program.len()
    }
}

/// A leg's target: an authored absolute point, or `Start`.
///
/// The Rust surface dispatches these through target TRAITS, so one
/// verb name serves both and the return type follows the target. The
/// extraction here is that dispatch, at the boundary, once.
#[derive(FromPyObject)]
enum PyTarget {
    Point((Length, Length)),
    Start(StartToken),
}

/// A leg landing at a bound position.
fn out_point(
    py: Python<'_>,
    r: Result<Path<HasPos<WithIncoming>, NoAng>, KPathError>,
) -> PyResult<Py<PyAny>> {
    match r {
        Ok(p) => Ok(Py::new(py, PathDirectedPoint(p))?.into_any()),
        Err(err) => Err(path_err(py, &err)),
    }
}

/// A leg that closed the loop.
fn out_closed(py: Python<'_>, r: Result<pf::ClosedLoop<f64>, KPathError>) -> PyResult<Py<PyAny>> {
    match r {
        Ok(loop_) => Ok(Py::new(py, ClosedLoop(loop_))?.into_any()),
        Err(err) => Err(path_err(py, &err)),
    }
}

// ------------------------------------------------------------------
// `ArcData`: the §2c arc-spec modes as standalone value types
// ------------------------------------------------------------------

/// An arc mode's authored endpoint, extracted once at the boundary.
#[derive(Clone, Copy)]
enum Tgt {
    Point(Point2<f64>),
    Start,
}

impl Tgt {
    fn of(t: PyTarget) -> Self {
        match t {
            PyTarget::Point(p) => Self::Point(pt(p)),
            PyTarget::Start(_) => Self::Start,
        }
    }
}

/// Mode `Bulge(p, b)` — chord-relative, so it is admissible exactly
/// where the chord exists: leg targets and the fused verbs' incoming
/// specs. Never an arrival (an arrival has no chord yet).
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Bulge {
    p: Tgt,
    b: f64,
}

#[pymethods]
impl Bulge {
    #[new]
    fn new(p: PyTarget, b: f64) -> Self {
        Self { p: Tgt::of(p), b }
    }
}

/// Mode `Via(q, p)` — the arc THROUGH an authored point.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Via {
    q: Point2<f64>,
    p: Tgt,
}

#[pymethods]
impl Via {
    #[new]
    fn new(q: (Length, Length), p: PyTarget) -> Self {
        Self {
            q: pt(q),
            p: Tgt::of(p),
        }
    }
}

/// Mode `Center(c, winding, p)` — the arc ABOUT an authored centre.
/// From a bare point it supplies the direction retroactively; from a
/// bound direction it is inadmissible, because the derived tangent
/// would have to value-match the direction already authored.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Center {
    c: Point2<f64>,
    winding: ArcSweep,
    p: Tgt,
}

#[pymethods]
impl Center {
    #[new]
    fn new(c: (Length, Length), winding: ArcSweep, p: PyTarget) -> Self {
        Self {
            c: pt(c),
            winding,
            p: Tgt::of(p),
        }
    }
}

/// Mode `Radius(r, side)` — the centre is DERIVED from the directed
/// anchor, so tangency holds by construction and nothing is authored
/// twice. The arrival mode, and the one mode an on-carrier tip takes.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Radius {
    r: Length,
    side: ArcSide,
}

#[pymethods]
impl Radius {
    #[new]
    fn new(r: Length, side: ArcSide) -> Self {
        Self { r, side }
    }
}

impl Radius {
    fn to_kernel(self) -> pf::Radius<f64> {
        pf::Radius {
            r: self.r.0.meters(),
            side: self.side.to_kernel(),
        }
    }
}

/// Endpoint-free mode `Sweep(r, side, angle)` — the arc analog of
/// `line(len)`: tangent-departing, endpoint DERIVED.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Sweep {
    r: Length,
    side: ArcSide,
    angle: Angle,
}

#[pymethods]
impl Sweep {
    #[new]
    fn new(r: Length, side: ArcSide, angle: Angle) -> Self {
        Self { r, side, angle }
    }
}

/// Endpoint-free mode `ArcLen(r, side, len)` — as `Sweep`, with the
/// extent authored as an arc length.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct ArcLen {
    r: Length,
    side: ArcSide,
    len: Length,
}

#[pymethods]
impl ArcLen {
    #[new]
    fn new(r: Length, side: ArcSide, len: Length) -> Self {
        Self { r, side, len }
    }
}

/// The modes a POINT tip admits — as a sharp leg, and as a fused
/// verb's incoming side. Rust makes an inadmissible pair a missing
/// trait impl; here the boundary extraction is that matrix, so a
/// `Sweep`/`ArcLen` written at a point tip is a `TypeError` naming the
/// three modes that belong there, with no kernel call made.
#[derive(FromPyObject, Clone, Copy)]
enum PointSpec {
    Bulge(Bulge),
    Via(Via),
    Center(Center),
}

/// The modes a DIRECTED tip admits: the endpoint-free pair.
#[derive(FromPyObject, Clone, Copy)]
enum TangentSpec {
    Sweep(Sweep),
    ArcLen(ArcLen),
}

/// The modes an ARRIVAL admits (`Bulge` is absent — an arrival has no
/// chord to be relative to).
#[derive(FromPyObject, Clone, Copy)]
enum ArrivalSpec {
    Center(Center),
    Radius(Radius),
    Via(Via),
}

/// The modes a DIRECTED-POINT tip (leg end) admits as a fused
/// incoming: the endpoint-full trio plus `Radius` — ARC EXTENSION, the
/// arc analog of ray extension (the carrier is derived from the tip's
/// own position and tangent; the incoming side's anchor is the tip).
#[derive(FromPyObject, Clone, Copy)]
enum LegEndSpec {
    Bulge(Bulge),
    Via(Via),
    Center(Center),
    Radius(Radius),
}

/// A fused verb's incoming side ends at its own authored anchor, so a
/// `Start` target there names no anchor.
fn incoming_needs_anchor(py: Python<'_>) -> PyErr {
    let _ = py;
    pyo3::exceptions::PyTypeError::new_err(
        "a fused verb's incoming spec authors the incoming side's anchor, so its `p` must be a \
         point; `Start` targets belong to a LEG (arc_to) or to the ARRIVAL spec, which closes",
    )
}

/// A `Radius` arrival awaiting both binders, in either order.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct PathRadiusArrival(pf::path::RadiusArrival<f64>);

/// A `Radius` arrival with its anchor bound, director pending.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct PathRadiusArrivalAt(pf::path::RadiusArrivalAt<f64>);

/// A `Radius` arrival with its director bound, anchor pending.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct PathRadiusArrivalDir(pf::path::RadiusArrivalDir<f64>);

/// A `Via` arrival: the anchor rides the spec, one director pending.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct PathViaArrival(pf::path::ViaArrival<f64>);

/// A `Via` arrival that CLOSES: one director pending at the entry.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct PathViaArrivalStart(pf::path::ViaArrivalStart<f64>);

#[pymethods]
impl PathRadiusArrival {
    /// Bind the arrival's anchor — a real on-carrier point.
    fn at(&self, p: (Length, Length)) -> PathRadiusArrivalAt {
        PathRadiusArrivalAt(self.0.clone().at(pt(p)))
    }

    /// Bind the arrival direction at the anchor.
    fn angle(&self, theta: Angle) -> PathRadiusArrivalDir {
        PathRadiusArrivalDir(self.0.clone().angle(theta.0.radians()))
    }

    /// Bind the arrival direction as exact components.
    fn toward(&self, py: Python<'_>, dx: f64, dy: f64) -> PyResult<PathRadiusArrivalDir> {
        let tol = Tol::witness();
        self.0
            .clone()
            .toward(dx, dy, tol)
            .map(PathRadiusArrivalDir)
            .map_err(|err| path_err(py, &err))
    }
}

#[pymethods]
impl PathRadiusArrivalAt {
    /// Bind the remaining director; the centre follows from it.
    fn angle(&self, py: Python<'_>, theta: Angle) -> PyResult<PathDirectedPoint> {
        let tol = Tol::witness();
        self.0
            .clone()
            .angle(theta.0.radians(), tol)
            .map(PathDirectedPoint)
            .map_err(|err| path_err(py, &err))
    }

    /// Bind the remaining director as exact components.
    fn toward(&self, py: Python<'_>, dx: f64, dy: f64) -> PyResult<PathDirectedPoint> {
        let tol = Tol::witness();
        self.0
            .clone()
            .toward(dx, dy, tol)
            .map(PathDirectedPoint)
            .map_err(|err| path_err(py, &err))
    }
}

#[pymethods]
impl PathRadiusArrivalDir {
    /// Bind the remaining anchor.
    fn at(&self, py: Python<'_>, p: (Length, Length)) -> PyResult<PathDirectedPoint> {
        let tol = Tol::witness();
        self.0
            .clone()
            .at(pt(p), tol)
            .map(PathDirectedPoint)
            .map_err(|err| path_err(py, &err))
    }
}

#[pymethods]
impl PathViaArrival {
    /// Bind the arrival direction at the spec's anchor.
    fn angle(&self, py: Python<'_>, theta: Angle) -> PyResult<PathDirectedPoint> {
        let tol = Tol::witness();
        self.0
            .clone()
            .angle(theta.0.radians(), tol)
            .map(PathDirectedPoint)
            .map_err(|err| path_err(py, &err))
    }

    /// Bind the arrival direction as exact components.
    fn toward(&self, py: Python<'_>, dx: f64, dy: f64) -> PyResult<PathDirectedPoint> {
        let tol = Tol::witness();
        self.0
            .clone()
            .toward(dx, dy, tol)
            .map(PathDirectedPoint)
            .map_err(|err| path_err(py, &err))
    }
}

#[pymethods]
impl PathViaArrivalStart {
    /// Bind the seam direction, closing the loop.
    fn angle(&self, py: Python<'_>, theta: Angle) -> PyResult<ClosedLoop> {
        let tol = Tol::witness();
        self.0
            .clone()
            .angle(theta.0.radians(), tol)
            .map(ClosedLoop)
            .map_err(|err| path_err(py, &err))
    }

    /// Bind the seam direction as exact components.
    fn toward(&self, py: Python<'_>, dx: f64, dy: f64) -> PyResult<ClosedLoop> {
        let tol = Tol::witness();
        self.0
            .clone()
            .toward(dx, dy, tol)
            .map(ClosedLoop)
            .map_err(|err| path_err(py, &err))
    }
}

/// The ARRIVAL half of a fused verb: one arm per admissible arrival
/// mode, each re-typing the boundary value as the kernel spec its row
/// takes and boxing the state that mode completes into.
macro_rules! arrival {
    ($py:expr, $spec2:expr, |$s:ident| $call:expr) => {
        match $spec2 {
            ArrivalSpec::Center(Center {
                c,
                winding,
                p: Tgt::Point(t),
            }) => {
                let $s = pf::Center {
                    c,
                    winding: winding.to_kernel(),
                    p: t,
                };
                out_point($py, $call)
            }
            ArrivalSpec::Center(Center {
                c,
                winding,
                p: Tgt::Start,
            }) => {
                let $s = pf::Center {
                    c,
                    winding: winding.to_kernel(),
                    p: pf::Start,
                };
                out_closed($py, $call)
            }
            ArrivalSpec::Radius(spec) => {
                let $s = spec.to_kernel();
                match $call {
                    Ok(a) => Ok(Py::new($py, PathRadiusArrival(a))?.into_any()),
                    Err(err) => Err(path_err($py, &err)),
                }
            }
            ArrivalSpec::Via(Via {
                q,
                p: Tgt::Point(t),
            }) => {
                let $s = pf::Via { q, p: t };
                match $call {
                    Ok(a) => Ok(Py::new($py, PathViaArrival(a))?.into_any()),
                    Err(err) => Err(path_err($py, &err)),
                }
            }
            ArrivalSpec::Via(Via { q, p: Tgt::Start }) => {
                let $s = pf::Via { q, p: pf::Start };
                match $call {
                    Ok(a) => Ok(Py::new($py, PathViaArrivalStart(a))?.into_any()),
                    Err(err) => Err(path_err($py, &err)),
                }
            }
        }
    };
}

/// A fused verb's INCOMING half from a point tip: the endpoint-full
/// modes, whose `p` is the incoming side's own anchor.
macro_rules! point_incoming {
    ($py:expr, $spec:expr, |$s:ident| $call:expr) => {
        match $spec {
            PointSpec::Bulge(Bulge {
                p: Tgt::Point(t),
                b,
            }) => {
                let $s = pf::Bulge { p: t, b };
                $call
            }
            PointSpec::Via(Via {
                q,
                p: Tgt::Point(t),
            }) => {
                let $s = pf::Via { q, p: t };
                $call
            }
            PointSpec::Center(Center {
                c,
                winding,
                p: Tgt::Point(t),
            }) => {
                let $s = pf::Center {
                    c,
                    winding: winding.to_kernel(),
                    p: t,
                };
                $call
            }
            _ => return Err(incoming_needs_anchor($py)),
        }
    };
}

/// A fused verb's INCOMING half from a DIRECTED-POINT tip: the
/// endpoint-full modes plus `Radius` (arc extension).
macro_rules! leg_end_incoming {
    ($py:expr, $spec:expr, |$s:ident| $call:expr) => {
        match $spec {
            LegEndSpec::Bulge(Bulge {
                p: Tgt::Point(t),
                b,
            }) => {
                let $s = pf::Bulge { p: t, b };
                $call
            }
            LegEndSpec::Via(Via {
                q,
                p: Tgt::Point(t),
            }) => {
                let $s = pf::Via { q, p: t };
                $call
            }
            LegEndSpec::Center(Center {
                c,
                winding,
                p: Tgt::Point(t),
            }) => {
                let $s = pf::Center {
                    c,
                    winding: winding.to_kernel(),
                    p: t,
                };
                $call
            }
            LegEndSpec::Radius(spec) => {
                let $s = spec.to_kernel();
                $call
            }
            _ => return Err(incoming_needs_anchor($py)),
        }
    };
}

/// A fused verb's INCOMING half from a directed tip: the endpoint-free
/// pair, which derives its own endpoint from the bound departure.
macro_rules! tangent_incoming {
    ($spec:expr, |$s:ident| $call:expr) => {
        match $spec {
            TangentSpec::Sweep(spec) => {
                let $s = pf::Sweep {
                    r: spec.r.0.meters(),
                    side: spec.side.to_kernel(),
                    angle: spec.angle.0.radians(),
                };
                $call
            }
            TangentSpec::ArcLen(spec) => {
                let $s = pf::ArcLen {
                    r: spec.r.0.meters(),
                    side: spec.side.to_kernel(),
                    len: spec.len.0.meters(),
                };
                $call
            }
        }
    };
}

/// A `Point` state = {position}: the two flavors Rust distinguishes by
/// type, written once.
///
/// Both flavors get the same director pair and the same four
/// target-taking legs, from impls Rust writes once and generically;
/// the directed flavor adds what only an incoming tangent can answer.
/// The macro generates the whole `#[pymethods]` block, because a
/// `macro_rules!` invocation INSIDE one would not be expanded before
/// PyO3 reads the impl.
macro_rules! point_state {
    ($cls:ident, $flavor:ident, $doc:literal, { $($extra:tt)* }) => {
        #[doc = $doc]
        #[pyclass(frozen, module = "pncad")]
        pub(crate) struct $cls(Path<HasPos<$flavor>, NoAng>);

        #[pymethods]
        impl $cls {
            /// Bind the outgoing direction (`Point -> Directed`).
            fn angle(&self, py: Python<'_>, theta: Angle) -> PyResult<PathDirected> {
                let tol = Tol::witness();
                self.0
                    .clone()
                    .angle(theta.0.radians(), tol)
                    .map(|d| PathDirected(Directed::$flavor(d)))
                    .map_err(|err| path_err(py, &err))
            }

            /// Bind the outgoing direction as exact components.
            fn toward(&self, py: Python<'_>, dx: f64, dy: f64) -> PyResult<PathDirected> {
                let tol = Tol::witness();
                self.0
                    .clone()
                    .toward(dx, dy, tol)
                    .map(|d| PathDirected(Directed::$flavor(d)))
                    .map_err(|err| path_err(py, &err))
            }

            /// A straight leg to `target`.
        fn line_to(&self, py: Python<'_>, target: PyTarget) -> PyResult<Py<PyAny>> {
            let tol = Tol::witness();
            match target {
                PyTarget::Point(p) => out_point(py, self.0.clone().line_to(pt(p), tol)),
                PyTarget::Start(_) => out_closed(py, self.0.clone().line_to(pf::Start, tol)),
            }
        }

        /// The SHARP arc leg, one verb over the endpoint-full modes:
        /// `Bulge(p, b)` chord-relative, `Via(q, p)` through a point,
        /// `Center(c, winding, p)` about a centre. `p=Start` closes.
        fn arc_to(&self, py: Python<'_>, spec: PointSpec) -> PyResult<Py<PyAny>> {
            let tol = Tol::witness();
            let path = self.0.clone();
            match spec {
                PointSpec::Bulge(Bulge { p: Tgt::Point(t), b }) => {
                    out_point(py, path.arc_to(pf::Bulge { p: t, b }, tol))
                }
                PointSpec::Bulge(Bulge { p: Tgt::Start, b }) => {
                    out_closed(py, path.arc_to(pf::Bulge { p: pf::Start, b }, tol))
                }
                PointSpec::Via(Via { q, p: Tgt::Point(t) }) => {
                    out_point(py, path.arc_to(pf::Via { q, p: t }, tol))
                }
                PointSpec::Via(Via { q, p: Tgt::Start }) => {
                    out_closed(py, path.arc_to(pf::Via { q, p: pf::Start }, tol))
                }
                PointSpec::Center(Center { c, winding, p: Tgt::Point(t) }) => out_point(
                    py,
                    path.arc_to(pf::Center { c, winding: winding.to_kernel(), p: t }, tol),
                ),
                PointSpec::Center(Center { c, winding, p: Tgt::Start }) => out_closed(
                    py,
                    path.arc_to(pf::Center { c, winding: winding.to_kernel(), p: pf::Start }, tol),
                ),
            }
        }

            $($extra)*
        }
    };
}

point_state!(
    PathPoint,
    Plain,
    "A plain point: position bound, no incoming carrier — the entry \
     point, or a fillet arrival stopped at its anchor.\n\n\
     `.tangent()` is absent here, and that absence is what makes \
     \"fillets sit between defined geometry\" structural rather than a \
     rule: a plain point has no direction to inherit.",
    {
        /// The FUSED verb: author the incoming arc (mode per `spec`,
        /// its `p` the incoming side's anchor) and fillet off it in
        /// ONE act — the arc and the trim that shapes it are one
        /// authoring decision, so they are one call. Line arrival.
        fn arc_fillet(
            &self,
            py: Python<'_>,
            spec: PointSpec,
            radius: Length,
        ) -> PyResult<PathOpen> {
            let tol = Tol::witness();
            let path = self.0.clone();
            let r = radius.0.meters();
            let out = point_incoming!(py, spec, |s| path.arc_fillet(s, r, tol));
            out.map(PathOpen).map_err(|err| path_err(py, &err))
        }

        /// The fused verb with an ARC arrival too: the arrival's own
        /// mode decides what the chain becomes (a `Center` anchor
        /// lands at a directed point, `Center(p=Start)` closes,
        /// `Radius` and `Via` await their binders).
        fn arc_fillet_arc(
            &self,
            py: Python<'_>,
            spec: PointSpec,
            radius: Length,
            spec2: ArrivalSpec,
        ) -> PyResult<Py<PyAny>> {
            let tol = Tol::witness();
            let path = self.0.clone();
            let r = radius.0.meters();
            point_incoming!(py, spec, |si| arrival!(py, spec2, |s2| path
                .clone()
                .arc_fillet_arc(si, r, s2, tol)))
        }
    }
);

point_state!(
    PathDirectedPoint,
    WithIncoming,
    "A leg end: position bound, and the leg's incoming end tangent \
     available as read-only intrinsic data.\n\n\
     The incoming direction is never a slot — it is settable by \
     nothing, and consultable by `.tangent()`, `.turn()` and the \
     junction check.",
    {
        /// Re-use the incoming end tangent as the departure — exact by
        /// construction — and DECLARE the joint tangent on lowering.
        fn tangent(&self) -> PathDirected {
            PathDirected(Directed::WithIncoming(self.0.clone().tangent()))
        }

        /// Depart at the incoming tangent rotated by `delta`. A zero
        /// turn lands in the tangent band and refuses (use
        /// `tangent()`); a half turn refuses as a cusp.
        fn turn(&self, py: Python<'_>, delta: Angle) -> PyResult<PathDirected> {
            let tol = Tol::witness();
            self.0
                .clone()
                .turn(delta.0.radians(), tol)
                .map(|d| PathDirected(Directed::WithIncoming(d)))
                .map_err(|err| path_err(py, &err))
        }

        /// Open a fillet directly on this leg end: the incoming side
        /// is the TANGENT RAY ahead of the point, extended as a real
        /// line leg (uniformly, whatever leg arrived here — the state
        /// carries position and tangent, and nothing else is knowable).
        fn fillet(&self, py: Python<'_>, radius: Length) -> PyResult<PathOpen> {
            let tol = Tol::witness();
            self.0
                .clone()
                .fillet(radius.0.meters(), tol)
                .map(PathOpen)
                .map_err(|err| path_err(py, &err))
        }

        /// Ray-extension fillet with an ARC arrival.
        fn fillet_arc(
            &self,
            py: Python<'_>,
            radius: Length,
            spec: ArrivalSpec,
        ) -> PyResult<Py<PyAny>> {
            let tol = Tol::witness();
            let path = self.0.clone();
            let r = radius.0.meters();
            arrival!(py, spec, |s| path.clone().fillet_arc(r, s, tol))
        }

        /// The FUSED verb from a leg end: the endpoint-full modes
        /// (junction-checked at the tip) plus `Radius` — ARC
        /// EXTENSION: the carrier is derived from this tip's own
        /// position and tangent, so tangency there holds by
        /// construction; the same carrier extends the arriving leg,
        /// a different one departs on a new tangent carrier. Line
        /// arrival.
        fn arc_fillet(
            &self,
            py: Python<'_>,
            spec: LegEndSpec,
            radius: Length,
        ) -> PyResult<PathOpen> {
            let tol = Tol::witness();
            let path = self.0.clone();
            let r = radius.0.meters();
            let out = leg_end_incoming!(py, spec, |s| path.arc_fillet(s, r, tol));
            out.map(PathOpen).map_err(|err| path_err(py, &err))
        }

        /// The fused verb with an ARC arrival too.
        fn arc_fillet_arc(
            &self,
            py: Python<'_>,
            spec: LegEndSpec,
            radius: Length,
            spec2: ArrivalSpec,
        ) -> PyResult<Py<PyAny>> {
            let tol = Tol::witness();
            let path = self.0.clone();
            let r = radius.0.meters();
            leg_end_incoming!(py, spec, |si| arrival!(py, spec2, |s2| path
                .clone()
                .arc_fillet_arc(si, r, s2, tol)))
        }

        /// Continue the incoming ARC carrier to an authored on-carrier
        /// point, minting a STRUCTURAL subdivision vertex. The junction
        /// is a same-carrier identity, so no junction check runs and
        /// nothing is declared tangent.
        fn arc_continue(
            &self,
            py: Python<'_>,
            target: (Length, Length),
        ) -> PyResult<PathDirectedPoint> {
            let tol = Tol::witness();
            self.0
                .clone()
                .arc_continue(pt(target), tol)
                .map(PathDirectedPoint)
                .map_err(|err| path_err(py, &err))
        }
    }
);

// ------------------------------------------------------------------
// Open = {} — the entry
// ------------------------------------------------------------------

/// The entry point of the algebra: nothing bound yet.
///
/// `Open` is a token, not a value you construct — write `Open.at(p)`
/// exactly as Rust does. It is distinct from `PathOpen` (a fillet's
/// freshly opened arrival side) because the entry cannot close: there
/// is no `Start` to target until the entry has authored one.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Open;

#[pymethods]
impl Open {
    /// Bind the entry position (`Open -> Point`, plain flavor — the
    /// entry has no incoming carrier; its junction check happens at
    /// the seam).
    #[staticmethod]
    fn at(p: (Length, Length)) -> PathPoint {
        PathPoint(pf::Open.at(pt(p)))
    }

    /// Bind the entry direction first (`Open -> Angle`).
    #[staticmethod]
    fn angle(theta: Angle) -> PathAngle {
        PathAngle(pf::Open.angle(theta.0.radians()))
    }

    /// Bind the entry direction as exact COMPONENTS: only the RATIO
    /// carries meaning, and the unit ray is stored verbatim, so an
    /// axis-aligned or Pythagorean direction is exact.
    #[staticmethod]
    fn toward(py: Python<'_>, dx: f64, dy: f64) -> PyResult<PathAngle> {
        let tol = Tol::witness();
        pf::Open
            .toward(dx, dy, tol)
            .map(PathAngle)
            .map_err(|err| path_err(py, &err))
    }

    /// The ENTRY fused verb: author the entry side ON an arc carrier
    /// — the spec's `p` is the entry anchor and the direction is the
    /// carrier's tangent there, derived rather than authored — and
    /// open a fillet of `radius` off it. Line arrival.
    #[staticmethod]
    fn arc_fillet(py: Python<'_>, spec: Center, radius: Length) -> PyResult<PathOpen> {
        let tol = Tol::witness();
        let Tgt::Point(p) = spec.p else {
            return Err(incoming_needs_anchor(py));
        };
        pf::Open
            .arc_fillet(
                pf::Center {
                    c: spec.c,
                    winding: spec.winding.to_kernel(),
                    p,
                },
                radius.0.meters(),
                tol,
            )
            .map(PathOpen)
            .map_err(|err| path_err(py, &err))
    }

    /// The entry fused verb with an ARC arrival.
    #[staticmethod]
    fn arc_fillet_arc(
        py: Python<'_>,
        spec: Center,
        radius: Length,
        spec2: ArrivalSpec,
    ) -> PyResult<Py<PyAny>> {
        let tol = Tol::witness();
        let Tgt::Point(p) = spec.p else {
            return Err(incoming_needs_anchor(py));
        };
        let si = pf::Center {
            c: spec.c,
            winding: spec.winding.to_kernel(),
            p,
        };
        let r = radius.0.meters();
        arrival!(py, spec2, |s2| pf::Open.arc_fillet_arc(si, r, s2, tol))
    }
}

// ------------------------------------------------------------------
// PathOpen = {} — a fillet's arrival side
// ------------------------------------------------------------------

/// A fillet's freshly opened arrival side: nothing bound, in either
/// order, and `Start` reachable because the entry is behind us.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct PathOpen(Path<NoPos, NoAng>);

#[pymethods]
impl PathOpen {
    /// Bind the arrival side's anchor — a real on-path point.
    fn at(&self, py: Python<'_>, p: (Length, Length)) -> PyResult<PathPoint> {
        let tol = Tol::witness();
        self.0
            .clone()
            .at(pt(p), tol)
            .map(PathPoint)
            .map_err(|err| path_err(py, &err))
    }

    /// Bind the arrival direction (angle-first order).
    fn angle(&self, py: Python<'_>, theta: Angle) -> PyResult<PathAngle> {
        let tol = Tol::witness();
        self.0
            .clone()
            .angle(theta.0.radians(), tol)
            .map(PathAngle)
            .map_err(|err| path_err(py, &err))
    }

    /// Bind the arrival direction as exact components.
    fn toward(&self, py: Python<'_>, dx: f64, dy: f64) -> PyResult<PathAngle> {
        let tol = Tol::witness();
        self.0
            .clone()
            .toward(dx, dy, tol)
            .map(PathAngle)
            .map_err(|err| path_err(py, &err))
    }

    /// The SEAM FILLET close: the fillet arc becomes the closing
    /// segment and the entry vertex is retrimmed. Both carriers are
    /// bound, nothing is pending, the loop is closed.
    fn to(&self, py: Python<'_>, target: StartToken) -> PyResult<ClosedLoop> {
        let tol = Tol::witness();
        let _ = target;
        self.0
            .clone()
            .to(pf::Start, tol)
            .map(ClosedLoop)
            .map_err(|err| path_err(py, &err))
    }
}

// ------------------------------------------------------------------
// Angle = {angle}
// ------------------------------------------------------------------

/// Direction bound, position pending. Its only continuations bind the
/// position: `.at(p)` leaves the side open past its anchor, `.to(p)`
/// ENDS the side there.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct PathAngle(Path<NoPos, HasAng>);

#[pymethods]
impl PathAngle {
    /// Bind the position, leaving the side Directed and continuable.
    /// On a fillet arrival this completes both carriers, so the
    /// corner construction and the anchor-fit gates run HERE.
    fn at(&self, py: Python<'_>, p: (Length, Length)) -> PyResult<PathDirected> {
        let tol = Tol::witness();
        self.0
            .clone()
            .at(pt(p), tol)
            .map(|d| PathDirected(Directed::Plain(d)))
            .map_err(|err| path_err(py, &err))
    }

    /// The FAR-END anchor: bind the arrival side's position and END
    /// the side there, at the authored far vertex.
    fn to(&self, py: Python<'_>, anchor: (Length, Length)) -> PyResult<PathDirectedPoint> {
        let tol = Tol::witness();
        self.0
            .clone()
            .to(pt(anchor), tol)
            .map(PathDirectedPoint)
            .map_err(|err| path_err(py, &err))
    }
}

// ------------------------------------------------------------------
// Directed = {position, angle}
// ------------------------------------------------------------------

/// Which position the bound direction sits on. Rust distinguishes the
/// two by type but gives them their methods through ONE impl generic
/// over the flavor, so the method SETS are equal and one Python class
/// is the faithful mirror; this enum is the phantom parameter, erased.
enum Directed {
    Plain(Path<HasPos<Plain>, HasAng>),
    WithIncoming(Path<HasPos<WithIncoming>, HasAng>),
}

/// Both bits bound — the only state legs and `fillet` consume.
///
/// The outgoing angle is a slot filled at most once, so there is no
/// second director here: `angle`, `toward`, `tangent` and `turn` are
/// absent, and a double director is an `AttributeError`.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct PathDirected(Directed);

#[pymethods]
impl PathDirected {
    /// A straight leg of the given length along the bound direction.
    fn line(&self, py: Python<'_>, len: Length) -> PyResult<PathDirectedPoint> {
        let tol = Tol::witness();
        let out = match &self.0 {
            Directed::Plain(p) => p.clone().line(len.0.meters(), tol),
            Directed::WithIncoming(p) => p.clone().line(len.0.meters(), tol),
        };
        out.map(PathDirectedPoint).map_err(|err| path_err(py, &err))
    }

    /// Open a fillet of radius `radius`: this side's departure ray is
    /// consumed, and the arrival side opens unbound.
    ///
    /// The corner is never authored — it exists only as the carrier
    /// intersection, and the arc is fitted there and trims both sides
    /// once the arrival is Directed too.
    fn fillet(&self, py: Python<'_>, radius: Length) -> PyResult<PathOpen> {
        let tol = Tol::witness();
        let out = match &self.0 {
            Directed::Plain(p) => p.clone().fillet(radius.0.meters(), tol),
            Directed::WithIncoming(p) => p.clone().fillet(radius.0.meters(), tol),
        };
        out.map(PathOpen).map_err(|err| path_err(py, &err))
    }

    /// Open a fillet with an ARC arrival: line incoming (this side's
    /// departure ray), the arrival carrier authored by `spec`.
    fn fillet_arc(&self, py: Python<'_>, radius: Length, spec: ArrivalSpec) -> PyResult<Py<PyAny>> {
        let tol = Tol::witness();
        let r = radius.0.meters();
        match &self.0 {
            Directed::Plain(p) => {
                let path = p.clone();
                arrival!(py, spec, |s| path.clone().fillet_arc(r, s, tol))
            }
            Directed::WithIncoming(p) => {
                let path = p.clone();
                arrival!(py, spec, |s| path.clone().fillet_arc(r, s, tol))
            }
        }
    }

    /// The SHARP arc leg from a bound direction: the endpoint-free
    /// modes, the arc analogs of `line(len)` — tangent-departing, the
    /// endpoint DERIVED from radius, side and extent.
    fn arc_to(&self, py: Python<'_>, spec: TangentSpec) -> PyResult<Py<PyAny>> {
        let tol = Tol::witness();
        match &self.0 {
            Directed::Plain(p) => {
                let path = p.clone();
                out_point(py, tangent_incoming!(spec, |s| path.arc_to(s, tol)))
            }
            Directed::WithIncoming(p) => {
                let path = p.clone();
                out_point(py, tangent_incoming!(spec, |s| path.arc_to(s, tol)))
            }
        }
    }

    /// The fused verb from a bound direction: author the incoming arc
    /// (endpoint-free mode) and fillet off it in one act.
    fn arc_fillet(&self, py: Python<'_>, spec: TangentSpec, radius: Length) -> PyResult<PathOpen> {
        let tol = Tol::witness();
        let r = radius.0.meters();
        let out = match &self.0 {
            Directed::Plain(p) => {
                let path = p.clone();
                tangent_incoming!(spec, |s| path.arc_fillet(s, r, tol))
            }
            Directed::WithIncoming(p) => {
                let path = p.clone();
                tangent_incoming!(spec, |s| path.arc_fillet(s, r, tol))
            }
        };
        out.map(PathOpen).map_err(|err| path_err(py, &err))
    }

    /// The fused verb with an arc arrival too.
    fn arc_fillet_arc(
        &self,
        py: Python<'_>,
        spec: TangentSpec,
        radius: Length,
        spec2: ArrivalSpec,
    ) -> PyResult<Py<PyAny>> {
        let tol = Tol::witness();
        let r = radius.0.meters();
        match &self.0 {
            Directed::Plain(p) => {
                let path = p.clone();
                tangent_incoming!(spec, |si| arrival!(py, spec2, |s2| path
                    .clone()
                    .arc_fillet_arc(si, r, s2, tol)))
            }
            Directed::WithIncoming(p) => {
                let path = p.clone();
                tangent_incoming!(spec, |si| arrival!(py, spec2, |s2| path
                    .clone()
                    .arc_fillet_arc(si, r, s2, tol)))
            }
        }
    }

    /// The unique arc leaving TANGENT to the bound direction and
    /// reaching `target`.
    fn tangent_arc_to(&self, py: Python<'_>, target: PyTarget) -> PyResult<Py<PyAny>> {
        let tol = Tol::witness();
        match (&self.0, target) {
            (Directed::Plain(p), PyTarget::Point(t)) => {
                out_point(py, p.clone().tangent_arc_to(pt(t), tol))
            }
            (Directed::Plain(p), PyTarget::Start(_)) => {
                out_closed(py, p.clone().tangent_arc_to(pf::Start, tol))
            }
            (Directed::WithIncoming(p), PyTarget::Point(t)) => {
                out_point(py, p.clone().tangent_arc_to(pt(t), tol))
            }
            (Directed::WithIncoming(p), PyTarget::Start(_)) => {
                out_closed(py, p.clone().tangent_arc_to(pf::Start, tol))
            }
        }
    }
}

// ------------------------------------------------------------------
// The complete-loop program forms
// ------------------------------------------------------------------

/// The circle primitive: a one-step COMPLETE-LOOP program form, not a
/// chain — `circle(centre, r)` IS the whole loop, so there is nothing
/// to continue, close or bind, and it authors NO seam.
#[pyfunction]
fn circle(py: Python<'_>, centre: (Length, Length), radius: Length) -> PyResult<ClosedLoop> {
    let tol = Tol::witness();
    pf::circle(pt(centre), radius.0.meters(), tol)
        .map(ClosedLoop)
        .map_err(|err| path_err(py, &err))
}

/// The declared-subdivision closed carrier: one circle authored WITH
/// its seam structure — `n` arcs of equal sweep, the first vertex at
/// `phase` from the +x axis, counterclockwise.
///
/// `n` is a STRUCTURAL count and crosses as a plain integer, because
/// that is the type the kernel door takes; a negative one is refused
/// by the boundary extraction itself.
#[pyfunction]
fn circle_split(
    py: Python<'_>,
    centre: (Length, Length),
    radius: Length,
    n: usize,
    phase: Angle,
) -> PyResult<ClosedLoop> {
    let tol = Tol::witness();
    pf::circle_split(pt(centre), radius.0.meters(), n, phase.0.radians(), tol)
        .map(ClosedLoop)
        .map_err(|err| path_err(py, &err))
}

// ------------------------------------------------------------------
// The recorded program, lifted to the document vocabulary
// ------------------------------------------------------------------

/// Lift the RECORDED program through the document layer's own door.
///
/// The lift is `LoopProgram::from_recorded` (editor-core, beside the
/// literal authoring helpers): one seam between the two authoring
/// surfaces, shared by both host languages. Only the refusal mapping
/// is binding work.
pub(crate) fn loop_program(py: Python<'_>, closed: &ClosedLoop) -> PyResult<d::LoopProgram> {
    d::LoopProgram::from_recorded(&closed.0.program).map_err(|err| {
        typed_err(
            py,
            ErrorClass::Literal,
            err.to_string(),
            &[(
                "variant",
                PyString::new(py, recorded_program_error_tag(&err))
                    .unbind()
                    .into_any(),
            )],
        )
    })
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Open>()?;
    m.add_class::<PathOpen>()?;
    m.add_class::<PathAngle>()?;
    m.add_class::<PathPoint>()?;
    m.add_class::<PathDirectedPoint>()?;
    m.add_class::<PathDirected>()?;
    m.add_class::<StartToken>()?;
    m.add_class::<ClosedLoop>()?;
    m.add_class::<ArcSweep>()?;
    m.add_class::<ArcSide>()?;
    m.add_class::<PathRadiusArrival>()?;
    m.add_class::<PathRadiusArrivalAt>()?;
    m.add_class::<PathRadiusArrivalDir>()?;
    m.add_class::<PathViaArrival>()?;
    m.add_class::<PathViaArrivalStart>()?;
    m.add_class::<Bulge>()?;
    m.add_class::<Via>()?;
    m.add_class::<Center>()?;
    m.add_class::<Radius>()?;
    m.add_class::<Sweep>()?;
    m.add_class::<ArcLen>()?;
    m.add_function(wrap_pyfunction!(circle, m)?)?;
    m.add_function(wrap_pyfunction!(circle_split, m)?)?;
    // `Start` is a VALUE in the Rust prelude, so it is a value here:
    // `line_to(Start)` reads the same in both languages.
    m.add("Start", Py::new(m.py(), StartToken)?)?;
    Ok(())
}
