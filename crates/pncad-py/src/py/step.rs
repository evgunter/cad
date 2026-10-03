//! **Authored step handles and the typed piece vocabulary**
//! (`names/README.md`, "N1, the profile pieces").
//!
//! Every chain state's `.step`, and a closed loop's, is an
//! [`AuthoredStep`]: the address of the step its verb just recorded,
//! which a profile binds to the id it minted (`Doc.step`,
//! `Doc.piece`). A handle's role accessors are not written here per
//! verb. One property per word the kernel's role lists answer
//! (`profile::RoleList::ALL`) is installed at module load, and each
//! answers only on a handle whose verb's list holds it, so a handle
//! answers exactly the roles its verb draws: `.leg` on a leg,
//! `.run_in`, `.arc` and `.run_out` on a fillet or a fused verb,
//! `.piece(k)` on a carrier form, and nothing on a binder.

use std::hash::{DefaultHasher, Hash, Hasher};

use pyo3::exceptions::PyAttributeError;
use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::ErrorClass;
use crate::py::typed_err;
use crate::tags::step_handle_refusal_tag;
use pncad::document as d;
use pncad::profile as pf;

/// A minted profile step id: what a piece's name spells, what
/// `Doc.step` answers and what `DocEdit.set_program` keeps a step by.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct StepId(pub(crate) d::StepId);

#[pymethods]
impl StepId {
    fn __repr__(&self) -> String {
        format!("StepId({})", self.0.full())
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        self.0.0
    }
}

/// **Which of its step's pieces a piece is**: `Role.Leg`,
/// `Role.RunIn`, `Role.Arc`, `Role.RunOut`, or `Role.piece(k)` on a
/// carrier form.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Role(pub(crate) pf::PieceRole);

#[pymethods]
impl Role {
    /// The one segment a single-segment verb draws.
    #[classattr]
    #[allow(non_snake_case, reason = "a class attribute spelled as its variant")]
    fn Leg() -> Self {
        Self(pf::PieceRole::Leg)
    }

    /// The run into a fillet.
    #[classattr]
    #[allow(non_snake_case, reason = "a class attribute spelled as its variant")]
    fn RunIn() -> Self {
        Self(pf::PieceRole::RunIn)
    }

    /// A fillet's own arc.
    #[classattr]
    #[allow(non_snake_case, reason = "a class attribute spelled as its variant")]
    fn Arc() -> Self {
        Self(pf::PieceRole::Arc)
    }

    /// The run out of a fillet.
    #[classattr]
    #[allow(non_snake_case, reason = "a class attribute spelled as its variant")]
    fn RunOut() -> Self {
        Self(pf::PieceRole::RunOut)
    }

    /// Piece `k` of a carrier form (`circle`, `circle_split`).
    #[staticmethod]
    fn piece(k: u32) -> Self {
        Self(pf::PieceRole::Piece(k))
    }

    fn __repr__(&self) -> String {
        match self.0 {
            pf::PieceRole::Piece(k) => format!("Role.piece({k})"),
            other => format!("Role.{other:?}"),
        }
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        digest(&self.0)
    }
}

/// **A profile piece**: the step that drew it, by its minted id, and
/// its role in that step's list. `str()` is the kernel's text for it,
/// the alphabet a name's text is written in.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone, Copy)]
pub(crate) struct Piece {
    step: d::StepId,
    role: pf::PieceRole,
}

impl Piece {
    /// The kernel's locator for this piece.
    pub(crate) fn edge(&self) -> pncad::select::ProfileEdgeRef {
        pncad::select::ProfileEdgeRef::Piece {
            step: self.step,
            role: self.role,
        }
    }

    /// The piece a kernel locator names.
    pub(crate) fn of_edge(edge: &pncad::select::ProfileEdgeRef) -> PyResult<Self> {
        match *edge {
            pncad::select::ProfileEdgeRef::Piece { step, role } => Ok(Self { step, role }),
            pncad::select::ProfileEdgeRef::Section { .. } => {
                Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "{} is a kernel-built section's piece, which no authored step drew",
                    super::doc::piece_text(edge)?
                )))
            }
        }
    }
}

#[pymethods]
impl Piece {
    #[new]
    fn new(step: StepId, role: Role) -> Self {
        Self {
            step: step.0,
            role: role.0,
        }
    }

    /// The minted id of the step that drew the piece.
    #[getter]
    fn step(&self) -> StepId {
        StepId(self.step)
    }

    /// The piece's role in its step's list.
    #[getter]
    fn role(&self) -> Role {
        Role(self.role)
    }

    fn __str__(&self) -> PyResult<String> {
        super::doc::piece_text(&self.edge())
    }

    fn __repr__(&self) -> String {
        format!(
            "Piece({}, {})",
            StepId(self.step).__repr__(),
            Role(self.role).__repr__()
        )
    }

    fn __eq__(&self, other: &Self) -> bool {
        (self.step, self.role) == (other.step, other.role)
    }

    fn __hash__(&self) -> u64 {
        digest(&(self.step, self.role))
    }
}

/// **The address of an authored step**: its index in its loop and the
/// loop's shape up to it, values erased. Every chain state's `.step`
/// and a closed loop's is one.
///
/// Its role accessors are its verb's role list: `.leg`; `.run_in`,
/// `.arc` and `.run_out`; `.piece(k)` on a carrier form; none on a
/// binder. Each answers a [`StepRole`] for `Doc.piece`.
#[pyclass(frozen, module = "pncad", from_py_object)]
#[derive(Clone)]
pub(crate) struct AuthoredStep(pub(crate) d::AuthoredStep);

/// The handle of the last step `recorded` holds — the one the verb
/// that produced a chain state recorded.
pub(crate) fn last_step(recorded: &[pf::Step<f64>]) -> AuthoredStep {
    match d::AuthoredStep::after(recorded) {
        Some(h) => AuthoredStep(h),
        None => unreachable!("every chain state has recorded the verb that produced it"),
    }
}

/// The attribute a role is read through on a handle.
fn role_word(role: pf::PieceRole) -> &'static str {
    match role {
        pf::PieceRole::Leg => "leg",
        pf::PieceRole::RunIn => "run_in",
        pf::PieceRole::Arc => "arc",
        pf::PieceRole::RunOut => "run_out",
        pf::PieceRole::Piece(_) => "piece",
    }
}

impl AuthoredStep {
    /// The accessor `name` on this handle: a role on its verb's list,
    /// or `piece` on a carrier form; an `AttributeError` naming the
    /// accessors it has otherwise.
    fn accessor(&self, py: Python<'_>, name: &str) -> PyResult<Py<PyAny>> {
        let step = self.0.step();
        if let Some(role) = step.roles().named().iter().find(|r| role_word(**r) == name) {
            return Ok(Py::new(
                py,
                StepRole {
                    step: self.0.clone(),
                    role: *role,
                },
            )?
            .into_any());
        }
        if step.roles() == pf::RoleList::Carrier && name == role_word(pf::PieceRole::Piece(0)) {
            return Ok(Py::new(py, CarrierPieces(self.0.clone()))?.into_any());
        }
        let words = self.role_words();
        Err(PyAttributeError::new_err(format!(
            "a `{}` step has no attribute {name:?}; its role accessors are {}",
            step.verb(),
            if words.is_empty() {
                "none, because it draws nothing".to_owned()
            } else {
                words.join(", ")
            }
        )))
    }

    /// The attributes this handle's role list answers.
    fn role_words(&self) -> Vec<&'static str> {
        let list = self.0.step().roles();
        match list {
            pf::RoleList::Carrier => vec![role_word(pf::PieceRole::Piece(0))],
            _ => list.named().iter().map(|r| role_word(*r)).collect(),
        }
    }
}

#[pymethods]
impl AuthoredStep {
    /// The step's index in its loop.
    #[getter]
    fn index(&self) -> u32 {
        self.0.index()
    }

    /// The verb the step names, in its authoring spelling.
    #[getter]
    fn verb(&self) -> String {
        self.0.step().verb().to_string()
    }

    fn __repr__(&self) -> String {
        format!(
            "AuthoredStep(index={}, verb={:?})",
            self.0.index(),
            self.0.step().verb().to_string()
        )
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __hash__(&self) -> u64 {
        digest(&self.0)
    }
}

/// `h.piece` on a carrier form's handle: `h.piece(k)` is its piece `k`,
/// checked against the form's count.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct CarrierPieces(d::AuthoredStep);

#[pymethods]
impl CarrierPieces {
    fn __call__(&self, py: Python<'_>, k: u32) -> PyResult<StepRole> {
        let role = pf::PieceRole::Piece(k);
        if !self.0.step().admits(role) {
            return Err(handle_err(
                py,
                &d::StepHandleRefusal::RoleNotDrawn {
                    verb: self.0.step().verb(),
                    role,
                },
            ));
        }
        Ok(StepRole {
            step: self.0.clone(),
            role,
        })
    }
}

/// **One role of an authored step**, as a handle's accessor answers it:
/// what `Doc.piece` binds to a [`Piece`].
#[pyclass(frozen, module = "pncad")]
pub(crate) struct StepRole {
    pub(crate) step: d::AuthoredStep,
    pub(crate) role: pf::PieceRole,
}

#[pymethods]
impl StepRole {
    /// The step.
    #[getter]
    fn step(&self) -> AuthoredStep {
        AuthoredStep(self.step.clone())
    }

    /// The role.
    #[getter]
    fn role(&self) -> Role {
        Role(self.role)
    }

    fn __repr__(&self) -> String {
        format!(
            "StepRole({}, {})",
            AuthoredStep(self.step.clone()).__repr__(),
            Role(self.role).__repr__()
        )
    }
}

/// A stable hash of a hashable kernel value: `DefaultHasher::new` is
/// keyed with constants, so equal values hash equal in every process.
fn digest<T: Hash>(value: &T) -> u64 {
    let mut h = DefaultHasher::new();
    value.hash(&mut h);
    h.finish()
}

/// Raise a handle that does not bind as `StepHandleError`.
pub(crate) fn handle_err(py: Python<'_>, refusal: &d::StepHandleRefusal) -> PyErr {
    let (loop_, index, role) = match refusal {
        d::StepHandleRefusal::OffProgram { loop_, index } => {
            (Some(*loop_), Some(*index), py.None())
        }
        d::StepHandleRefusal::RoleNotDrawn { role, .. } => match Py::new(py, Role(*role)) {
            Ok(role) => (None, None, role.into_any()),
            Err(failed) => return failed,
        },
        d::StepHandleRefusal::Unminted | d::StepHandleRefusal::StepIds(_) => {
            (None, None, py.None())
        }
    };
    let int = |v: Option<u32>| match v {
        Some(v) => match v.into_pyobject(py) {
            Ok(v) => v.unbind().into_any(),
            Err(never) => match never {},
        },
        None => py.None(),
    };
    typed_err(
        py,
        ErrorClass::StepHandle,
        refusal.to_string(),
        &[
            (
                "variant",
                PyString::new(py, step_handle_refusal_tag(refusal))
                    .unbind()
                    .into_any(),
            ),
            ("loop_", int(loop_)),
            ("index", int(index)),
            ("role", role),
        ],
    )
}

/// Every accessor word some role list answers: each list's named
/// roles, and `piece` for the indexed carrier list.
fn accessor_words() -> Vec<&'static str> {
    let mut words = Vec::new();
    for list in pf::RoleList::ALL {
        let listed: Vec<pf::PieceRole> = match list {
            pf::RoleList::Carrier => vec![pf::PieceRole::Piece(0)],
            _ => list.named().to_vec(),
        };
        for role in listed {
            let word = role_word(role);
            if !words.contains(&word) {
                words.push(word);
            }
        }
    }
    words
}

/// **Installs the role accessors on `AuthoredStep`**, one property per
/// word the role lists answer ([`accessor_words`]). Each answers only on
/// a handle whose verb's list holds it and raises `AttributeError`
/// elsewhere, so the accessors a handle has are its list's and nothing
/// here names a role per verb.
fn install_accessors(py: Python<'_>) -> PyResult<()> {
    let class = py.get_type::<AuthoredStep>();
    let property = py.import("builtins")?.getattr("property")?;
    for word in accessor_words() {
        let fget = pyo3::types::PyCFunction::new_closure(
            py,
            None,
            None,
            move |args: &Bound<'_, pyo3::types::PyTuple>,
                  _: Option<&Bound<'_, pyo3::types::PyDict>>|
                  -> PyResult<Py<PyAny>> {
                let handle = args.get_item(0)?;
                let handle = handle.cast::<AuthoredStep>()?;
                handle.get().accessor(args.py(), word)
            },
        )?;
        class.setattr(word, property.call1((fget,))?)?;
    }
    Ok(())
}

pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<StepId>()?;
    m.add_class::<Role>()?;
    m.add_class::<Piece>()?;
    m.add_class::<AuthoredStep>()?;
    install_accessors(m.py())?;
    m.add_class::<StepRole>()?;
    m.add_class::<CarrierPieces>()?;
    Ok(())
}
