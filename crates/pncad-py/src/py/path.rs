//! The PATHS profile-authoring lattice, bound state-for-state.
//!
//! LIBRARY-DESIGN §L4: the lattice renders as DISTINCT classes, each
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
        self.0.loop_.vertices.len()
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
                self.0
                    .clone()
                    .angle(theta.0.radians())
                    .map(|d| PathDirected(Directed::$flavor(d)))
                    .map_err(|err| path_err(py, &err))
            }

            /// Bind the outgoing direction as exact components.
            fn toward(&self, py: Python<'_>, dx: f64, dy: f64) -> PyResult<PathDirected> {
                self.0
                    .clone()
                    .toward(dx, dy)
                    .map(|d| PathDirected(Directed::$flavor(d)))
                    .map_err(|err| path_err(py, &err))
            }

            /// A straight leg to `target`.
        fn line_to(&self, py: Python<'_>, target: PyTarget) -> PyResult<Py<PyAny>> {
            match target {
                PyTarget::Point(p) => out_point(py, self.0.clone().line_to(pt(p))),
                PyTarget::Start(_) => out_closed(py, self.0.clone().line_to(pf::Start)),
            }
        }

        /// An arc leg to `target` with an AUTHORED bulge (the M2
        /// convention: tan of a quarter of the included angle).
        fn arc_to(&self, py: Python<'_>, target: PyTarget, bulge: f64) -> PyResult<Py<PyAny>> {
            match target {
                PyTarget::Point(p) => out_point(py, self.0.clone().arc_to(pt(p), bulge)),
                PyTarget::Start(_) => out_closed(py, self.0.clone().arc_to(pf::Start, bulge)),
            }
        }

        /// The arc THROUGH an authored point: `via` and the target
        /// bind the carrier, and the bulge is derived.
        fn arc_via(
            &self,
            py: Python<'_>,
            via: (Length, Length),
            target: PyTarget,
        ) -> PyResult<Py<PyAny>> {
            match target {
                PyTarget::Point(p) => out_point(py, self.0.clone().arc_via(pt(via), pt(p))),
                PyTarget::Start(_) => out_closed(py, self.0.clone().arc_via(pt(via), pf::Start)),
            }
        }

        /// The arc ABOUT an authored centre, swept `winding`.
        fn arc_center(
            &self,
            py: Python<'_>,
            centre: (Length, Length),
            target: PyTarget,
            winding: ArcSweep,
        ) -> PyResult<Py<PyAny>> {
            let w = winding.to_kernel();
            match target {
                PyTarget::Point(p) => {
                    out_point(py, self.0.clone().arc_center(pt(centre), pt(p), w))
                }
                PyTarget::Start(_) => {
                    out_closed(py, self.0.clone().arc_center(pt(centre), pf::Start, w))
                }
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
    {}
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
            self.0
                .clone()
                .turn(delta.0.radians())
                .map(|d| PathDirected(Directed::WithIncoming(d)))
                .map_err(|err| path_err(py, &err))
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
            self.0
                .clone()
                .arc_continue(pt(target))
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
        pf::Open
            .toward(dx, dy)
            .map(PathAngle)
            .map_err(|err| path_err(py, &err))
    }

    /// Bind position AND the carrier's tangent there in one act: the
    /// anchor `p` lies on the circle about `centre`, travelled
    /// `winding` (G2's carrier-bound anchor).
    #[staticmethod]
    fn at_on(
        py: Python<'_>,
        p: (Length, Length),
        centre: (Length, Length),
        winding: ArcSweep,
    ) -> PyResult<PathDirected> {
        pf::Open
            .at_on(pt(p), pt(centre), winding.to_kernel())
            .map(|d| PathDirected(Directed::Plain(d)))
            .map_err(|err| path_err(py, &err))
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
        self.0
            .clone()
            .at(pt(p))
            .map(PathPoint)
            .map_err(|err| path_err(py, &err))
    }

    /// Bind the arrival direction (angle-first order).
    fn angle(&self, py: Python<'_>, theta: Angle) -> PyResult<PathAngle> {
        self.0
            .clone()
            .angle(theta.0.radians())
            .map(PathAngle)
            .map_err(|err| path_err(py, &err))
    }

    /// Bind the arrival direction as exact components.
    fn toward(&self, py: Python<'_>, dx: f64, dy: f64) -> PyResult<PathAngle> {
        self.0
            .clone()
            .toward(dx, dy)
            .map(PathAngle)
            .map_err(|err| path_err(py, &err))
    }

    /// The SEAM FILLET close: the fillet arc becomes the closing
    /// segment and the entry vertex is retrimmed. Both carriers are
    /// bound, nothing is pending, the loop is closed.
    fn to(&self, py: Python<'_>, target: StartToken) -> PyResult<ClosedLoop> {
        let _ = target;
        self.0
            .clone()
            .to(pf::Start)
            .map(ClosedLoop)
            .map_err(|err| path_err(py, &err))
    }

    /// Bind the arrival anchor AND its carrier tangent in one act.
    fn at_on(
        &self,
        py: Python<'_>,
        p: (Length, Length),
        centre: (Length, Length),
        winding: ArcSweep,
    ) -> PyResult<PathDirected> {
        self.0
            .clone()
            .at_on(pt(p), pt(centre), winding.to_kernel())
            .map(|d| PathDirected(Directed::Plain(d)))
            .map_err(|err| path_err(py, &err))
    }

    /// Bind a STRAIGHT arrival's anchor AND its exact director in one
    /// act, off a departure bound on an arc carrier (LB10 route 3).
    fn at_toward(
        &self,
        py: Python<'_>,
        p: (Length, Length),
        dx: f64,
        dy: f64,
    ) -> PyResult<PathDirected> {
        self.0
            .clone()
            .at_toward(pt(p), dx, dy)
            .map(|d| PathDirected(Directed::Plain(d)))
            .map_err(|err| path_err(py, &err))
    }

    /// Close with the arrival running on a carrier that differs from
    /// side 1's, so the entry vertex is KEPT as a genuine two-carrier
    /// junction (G2).
    fn to_on(
        &self,
        py: Python<'_>,
        target: StartToken,
        centre: (Length, Length),
        winding: ArcSweep,
    ) -> PyResult<ClosedLoop> {
        let _ = target;
        self.0
            .clone()
            .to_on(pf::Start, pt(centre), winding.to_kernel())
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
        self.0
            .clone()
            .at(pt(p))
            .map(|d| PathDirected(Directed::Plain(d)))
            .map_err(|err| path_err(py, &err))
    }

    /// The FAR-END anchor: bind the arrival side's position and END
    /// the side there, at the authored far vertex.
    fn to(&self, py: Python<'_>, anchor: (Length, Length)) -> PyResult<PathDirectedPoint> {
        self.0
            .clone()
            .to(pt(anchor))
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
        let out = match &self.0 {
            Directed::Plain(p) => p.clone().line(len.0.meters()),
            Directed::WithIncoming(p) => p.clone().line(len.0.meters()),
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
        let out = match &self.0 {
            Directed::Plain(p) => p.clone().fillet(radius.0.meters()),
            Directed::WithIncoming(p) => p.clone().fillet(radius.0.meters()),
        };
        out.map(PathOpen).map_err(|err| path_err(py, &err))
    }

    /// The unique arc leaving TANGENT to the bound direction and
    /// reaching `target`.
    fn tangent_arc_to(&self, py: Python<'_>, target: PyTarget) -> PyResult<Py<PyAny>> {
        match (&self.0, target) {
            (Directed::Plain(p), PyTarget::Point(t)) => {
                out_point(py, p.clone().tangent_arc_to(pt(t)))
            }
            (Directed::Plain(p), PyTarget::Start(_)) => {
                out_closed(py, p.clone().tangent_arc_to(pf::Start))
            }
            (Directed::WithIncoming(p), PyTarget::Point(t)) => {
                out_point(py, p.clone().tangent_arc_to(pt(t)))
            }
            (Directed::WithIncoming(p), PyTarget::Start(_)) => {
                out_closed(py, p.clone().tangent_arc_to(pf::Start))
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
    pf::circle(pt(centre), radius.0.meters())
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
    pf::circle_split(pt(centre), radius.0.meters(), n, phase.0.radians())
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
    m.add_function(wrap_pyfunction!(circle, m)?)?;
    m.add_function(wrap_pyfunction!(circle_split, m)?)?;
    // `Start` is a VALUE in the Rust prelude, so it is a value here:
    // `line_to(Start)` reads the same in both languages.
    m.add("Start", Py::new(m.py(), StartToken)?)?;
    Ok(())
}
