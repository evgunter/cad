//! **The whole-document gather, and what the at-rest check reports.**
//!
//! `product` is the gather of the document's world: every copy a
//! placement defines, in the placements' document order, as one body.
//! It answers "what IS this document", which for an assembly is the
//! only useful question — an assembly's nodes are placements, and no
//! single node's value is the assembly.
//!
//! Whether it is valid at rest is the checks registry's `AtRest`
//! resident (`run_checks`): at each `⊔` the census examines its
//! arrivals pairwise, and nothing it finds refuses. The types here are
//! that resident's findings as Python reads them, off a
//! `CheckEvidence`: an overlap (`InterferenceFinding`), a part's
//! overlap carried up (`CarriedInterference`), and why the census could
//! not look (`AtRestEvidence`, a part's own as `CarriedAtRestFinding`).

use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::ErrorClass;
use crate::py::typed_err;
use crate::tags::{product_error_tag, unlocalized_tag};
use pncad::document as d;
use pncad::tolerance::Tol;

use super::doc::{Doc, NodeId, Var, name_text};
use super::value::{Body, Evaluation};

/// **What a row's `__str__` speaks its nodes from**: the evaluated
/// document for this document's own rows and for the first instance of
/// a carried row's route, and the nodes a carried row holds of the
/// document below it came from, for that row's own body.
#[derive(Clone)]
enum SpokenFrom {
    /// The evaluated document, as it holds its nodes now.
    Doc(Arc<d::ProfileDoc>),
    /// A carried row's nodes, as the document they are numbered in
    /// held them.
    Held(Arc<d::HeldNodes>),
}

impl SpokenFrom {
    fn say<T: d::Say>(&self, value: &T) -> String {
        match self {
            Self::Doc(doc) => d::spoken_by(value, doc),
            Self::Held(held) => d::Said(value, d::Speaker::held(held)).to_string(),
        }
    }
}

/// Raise `ProductError` carrying the refusal's stable tag and the
/// arm's own payload, its message each node as `doc`, the gathered
/// document, holds it.
///
/// Every attribute is set on every arm, `None` where the arm does not
/// carry it — the `WorkspaceError` posture: handling reads
/// `err.node` without first branching on `err.variant`.
pub(crate) fn product_err(py: Python<'_>, err: &d::ProductError, doc: &d::ProfileDoc) -> PyErr {
    let (node, through, read) = product_fields(py, err);
    typed_err(
        py,
        ErrorClass::Product,
        err.spoken(doc),
        &[
            (
                "variant",
                PyString::new(py, product_error_tag(err))
                    .unbind()
                    .into_any(),
            ),
            ("node", node),
            ("through", through),
            ("read", read),
        ],
    )
}

/// The gather refusal's payload, flattened: the node it is about, the
/// node it was reached through, and a stranded product slot's read.
fn product_fields(py: Python<'_>, err: &d::ProductError) -> (Py<PyAny>, Py<PyAny>, Py<PyAny>) {
    use d::ProductError as E;
    let id = |n: &d::RecipeNodeId| -> Py<PyAny> {
        Py::new(py, NodeId(*n))
            .map(|v| v.into_any())
            .unwrap_or_else(|_| py.None())
    };
    let none = || py.None();
    match err {
        E::Read(standing) => (
            id(&standing.node()),
            standing.through().map_or_else(none, |through| id(&through)),
            none(),
        ),
        E::Frame { node, .. } | E::Graft { node, .. } => (id(node), none(), none()),
        // The first failing piece, in gather order; every failing
        // piece and output is in the message.
        E::PieceInvalid { findings } => (
            findings.first().map_or_else(none, |first| id(&first.node)),
            none(),
            none(),
        ),
        // The read the product slot holds, as the handle
        // `Doc.set_product` takes.
        E::StrandedProduct { read } | E::ProductOutsideWorld { read } => (
            none(),
            none(),
            Py::new(py, Var(*read, Some(d::VarKind::Body)))
                .map_or_else(|_| py.None(), |v| v.into_any()),
        ),
        // The document-mismatch arm names two DOCUMENTS, which these
        // attributes cannot carry; the message states both.
        E::EmptyProduct
        | E::ProductInvalid { .. }
        | E::ContactLineage { .. }
        | E::EvaluationOfAnotherDocument { .. } => (none(), none(), none()),
    }
}

/// The document's **product**: the body its product slot reads
/// (`Doc.set_product`), a `⊔`'s copies side by side in its members'
/// order.
///
/// This is what a document IS, and for an assembly it is the only
/// useful reading: what is gathered is what the slot reads — read
/// through `Doc.product`, written through `Doc.set_product` — and
/// nothing enters it as a side effect of another edit.
///
/// A pure function of the slot and the evaluation: no ambient state,
/// so two evaluations of an edit that moves no copy yield the same
/// solid order.
///
/// `evaluation` must be an evaluation OF `doc`, and the door CHECKS
/// it: an evaluation carries the id of the document it was run on,
/// and a foreign one raises `ProductError` with tag
/// `evaluation_of_another_document` before the slot is read.
/// Node ids alone could not decide this — they are minted by a
/// per-document counter, so two documents built from one recipe carry
/// the same ids for the same nodes, and a gather over the wrong one
/// would succeed, in full, about other geometry.
///
/// **One gather per evaluation.** The document's product is a pure
/// function of the (document, evaluation) pair `evaluation` captured
/// at `evaluate` and the run's tolerance, so it is gathered on the
/// first ask and shared with every other door that wants one
/// ([`crate::product_memo`]). Reusing an `Evaluation` is therefore
/// how a caller asks several questions for the price of one gather.
///
/// Raises `ProductError`, typed: an operation the slot reads that
/// failed, was poisoned or is absent from this evaluation; an empty
/// slot (`empty_product`); a slot whose body is gone
/// (`stranded_product`, carrying the read); the kernel's graft and
/// validity refusals. All of it or none of it — there are no partial
/// products.
#[pyfunction]
pub(crate) fn product(py: Python<'_>, doc: &Doc, evaluation: &Evaluation) -> PyResult<Body> {
    let tol = Tol::witness();
    // The body DECLARES NOTHING: it is the copies' solids side by
    // side, with no records, so `Body.validate_pseudomanifold` reports
    // any seam between two copies as undeclared. An instance of the
    // document carries the records its census minted.
    evaluation
        .paired_with(doc)
        .map_err(|m| mispaired_product(py, m, evaluation))?;
    evaluation
        .gathered(|memo, doc, ev| crate::product_memo::body(memo, doc, ev, tol))
        .map(|body| Body::plain(Arc::new(body)))
        .map_err(|err| product_err(py, &err, evaluation.doc()))
}

/// A mispaired `(doc, evaluation)` as the gather's own refusal — the
/// one the memo path cannot inherit from a gather it does not reach.
fn mispaired_product(py: Python<'_>, m: d::Mispaired, evaluation: &Evaluation) -> PyErr {
    product_err(py, &m.into(), evaluation.doc())
}

/// The product, with the stable names its entities answer to —
/// `product`'s sibling, same gather, one more field.
///
/// The names are the product's OWN alphabet: a copy's entity is named
/// through the placement that defines the copy, and an instance's
/// through the instantiate node besides, which is what makes "the
/// third post's top cap" one name rather than a coordinate.
/// They cross as opaque text, like every other name in this library.
///
/// **One gather per evaluation.** The document's product is a pure
/// function of the (document, evaluation) pair `evaluation` captured
/// at `evaluate` and the run's tolerance, so it is gathered on the
/// first ask and shared with every other door that wants one
/// ([`crate::product_memo`]). Reusing an `Evaluation` is therefore
/// how a caller asks several questions for the price of one gather.
///
/// Raises `ProductError`, typed, as `product` does.
#[pyfunction]
pub(crate) fn product_named(
    py: Python<'_>,
    doc: &Doc,
    evaluation: &Evaluation,
) -> PyResult<(Body, Vec<String>)> {
    let tol = Tol::witness();
    evaluation
        .paired_with(doc)
        .map_err(|m| mispaired_product(py, m, evaluation))?;
    let (body, names) = evaluation
        .gathered(|memo, doc, ev| crate::product_memo::body_and_names(memo, doc, ev, tol))
        .map_err(|err| product_err(py, &err, evaluation.doc()))?;
    let names = names
        .iter()
        .map(|name| name_text(py, name))
        .collect::<PyResult<Vec<String>>>()?;
    Ok((Body::plain(Arc::new(body)), names))
}

// ---- The at-rest check's findings ----

/// The document a foreign row is of, and the instances this document
/// reached it through — the two halves of a [`d::Route`], as the
/// Python surface spells them.
///
/// ONE rule at every door that carries a foreign mate: `of` and `via`
/// travel with it. A bare node id in another document's space is not
/// something a caller can look up, so a door that hands one over
/// without the document has not answered the question.
/// `via` is the WHOLE route, nearest first: the instantiating node of
/// the document that was gathered, then one per intervening
/// sub-assembly in its own document's id space. One list rather than a
/// head plus a tail, because a caller walking it walks one thing.
pub(super) fn route_fields(py: Python<'_>, route: &d::Route) -> (Py<PyAny>, Py<PyAny>) {
    let of = PyString::new(py, &route.of.to_string()).unbind().into_any();
    let via = core::iter::once(route.through)
        .chain(route.via.iter().map(d::SpokenNode::id))
        .map(NodeId)
        .collect::<Vec<_>>()
        .into_pyobject(py)
        .map(|v| v.unbind().into_any())
        .unwrap_or_else(|_| py.None());
    (of, via)
}

/// **Why the at-rest check could not look** (DS6): one finding, loud,
/// unquietable, and refusing at no severity. Read off
/// `CheckEvidence.at_rest`.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct AtRestEvidence(d::AtRestEvidence, SpokenFrom);

impl AtRestEvidence {
    /// This document's own evidence, said as `doc` holds its nodes.
    pub(crate) fn of(evidence: d::AtRestEvidence, doc: &Arc<d::ProfileDoc>) -> Self {
        Self(evidence, SpokenFrom::Doc(Arc::clone(doc)))
    }

    fn payload(&self) -> crate::check_payload::AtRestPayload {
        crate::check_payload::at_rest_payload(&self.0)
    }
}

#[pymethods]
impl AtRestEvidence {
    /// Why, as a stable word: `no_lane` (a pair the census had no lane
    /// for or left undecided), `malformed` (the gathered body fails a
    /// check of its own structure or geometry), `inside_arrival` (a
    /// touch or overlap inside one arrival nothing backs),
    /// `not_gathered` (a `⊔` the product does not read whose operands
    /// did not gather), `not_run` (this evaluation's scalar runs no
    /// census) or `carried` (a part's own, with its route).
    #[getter]
    fn variant(&self) -> &'static str {
        self.payload().variant
    }

    /// The inner refusal's word: the validator's finding on `no_lane`,
    /// `malformed` and `inside_arrival` (`ValidationError`'s
    /// vocabulary), the gather's refusal on `not_gathered`
    /// (`ProductError`'s); `None` on `not_run` and `carried`.
    #[getter]
    fn inner_variant(&self) -> Option<&'static str> {
        self.payload().inner_variant
    }

    /// The part's own finding and its route, on `carried` alone.
    #[getter]
    fn carried(&self) -> Option<CarriedAtRestFinding> {
        match &self.0 {
            d::AtRestEvidence::Carried(row) => {
                Some(CarriedAtRestFinding((**row).clone(), self.1.clone()))
            }
            _ => None,
        }
    }

    /// The evidence in the library's own words. The kernel's messages
    /// carry their own recourse, so nothing is appended here.
    fn __str__(&self) -> String {
        self.1.say(&self.0)
    }

    /// The same evidence; how its nodes are said is not compared.
    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    fn __repr__(&self) -> String {
        format!("AtRestEvidence({:?})", self.payload().variant)
    }
}

/// An overlap a document BELOW this one reported between two of its own
/// copies, with the route this document reached it by: an `AtRest`
/// finding's `CheckEvidence.carried_interference`, never raised, quiet
/// or loud as the part's own assertions judged it.
///
/// The finding is the part's: its copies are variables of `of`, and its
/// faces are named as `of` names them.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct CarriedInterference {
    finding: InterferenceFinding,
    route: d::Route,
    pin: d::ContentPin,
}

impl CarriedInterference {
    pub(crate) fn read(py: Python<'_>, row: &d::CarriedInterference) -> PyResult<Self> {
        Ok(Self {
            finding: InterferenceFinding::carried(py, &row.finding)?,
            route: row.route.clone(),
            pin: row.pin,
        })
    }
}

#[pymethods]
impl CarriedInterference {
    /// The part's own finding, its copies variables of `of`.
    #[getter]
    fn finding(&self) -> InterferenceFinding {
        self.finding.clone()
    }

    /// The instantiating node OF THIS DOCUMENT the finding came through.
    #[getter]
    fn through(&self) -> NodeId {
        NodeId(self.route.through)
    }

    /// Which version of `of` found it.
    #[getter]
    fn pin(&self) -> super::store::ContentPin {
        super::store::ContentPin(self.pin)
    }

    /// The document whose copies overlap, as opaque id text.
    #[getter]
    fn of(&self, py: Python<'_>) -> Py<PyAny> {
        route_fields(py, &self.route).0
    }

    /// The instances this document reached it through, nearest first.
    #[getter]
    fn via(&self, py: Python<'_>) -> Py<PyAny> {
        route_fields(py, &self.route).1
    }

    fn __repr__(&self) -> String {
        format!(
            "CarriedInterference(of={}, loud={})",
            self.route.of,
            self.finding.quiet_by.is_none()
        )
    }
}

/// What a document BELOW this one's census could not look at, with the
/// route this document reached it by: `AtRestEvidence.carried`, loud
/// and never refusing.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct CarriedAtRestFinding(d::CarriedAtRestFinding, SpokenFrom);

#[pymethods]
impl CarriedAtRestFinding {
    /// The part's own evidence, said as the part holds its nodes.
    #[getter]
    fn finding(&self) -> AtRestEvidence {
        AtRestEvidence(
            self.0.finding.clone(),
            SpokenFrom::Held(Arc::clone(&self.0.held)),
        )
    }

    /// The instantiating node OF THIS DOCUMENT the finding came through.
    #[getter]
    fn through(&self) -> NodeId {
        NodeId(self.0.route.through)
    }

    /// Which version of `of` found it.
    #[getter]
    fn pin(&self) -> super::store::ContentPin {
        super::store::ContentPin(self.0.pin)
    }

    /// The document whose census found it, as opaque id text.
    #[getter]
    fn of(&self, py: Python<'_>) -> Py<PyAny> {
        route_fields(py, &self.0.route).0
    }

    /// The instances this document reached it through, nearest first.
    #[getter]
    fn via(&self, py: Python<'_>) -> Py<PyAny> {
        route_fields(py, &self.0.route).1
    }

    /// The row in the library's own words: which document, and what its
    /// census could not look at.
    fn __str__(&self) -> String {
        self.1.say(&self.0)
    }

    fn __repr__(&self) -> String {
        format!("CarriedAtRestFinding(of={})", self.0.route.of)
    }
}

/// A face of a copy, as the copy's own name table spells it: the name
/// a selection of the copy reads.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct FaceSite {
    copy: Var,
    member: u32,
    face: String,
}

#[pymethods]
impl FaceSite {
    /// The copy: its placement's output variable.
    #[getter]
    fn copy(&self) -> Var {
        self.copy
    }

    /// Which member of the placement's value the copy is (0 for a
    /// placement of one body).
    #[getter]
    fn member(&self) -> u32 {
        self.member
    }

    /// The face's name, as opaque text.
    #[getter]
    fn face(&self) -> String {
        self.face.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "FaceSite(copy=Var({}), member={}, face={})",
            self.copy.0.full(),
            self.member,
            self.face
        )
    }
}

/// An overlap of two copies' material at rest: an `AtRest` finding's
/// `CheckEvidence.interference`, never raised, and quiet only under a
/// holding one-sided `gap` assertion at its site (`quiet_by`).
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct InterferenceFinding {
    a: (Var, u32),
    b: (Var, u32),
    faces: Option<Vec<FaceSite>>,
    unlocalized: Option<&'static str>,
    quiet_by: Option<NodeId>,
    quiet_at: Vec<(Var, u32)>,
    quietable: bool,
}

impl InterferenceFinding {
    pub(crate) fn of(
        py: Python<'_>,
        doc: &d::ProfileDoc,
        finding: &d::InterferenceFinding,
    ) -> PyResult<Self> {
        Self::read(py, &|id| Var::of(doc, id), finding)
    }

    /// A part's own finding, carried up: its copies are variables of
    /// the part, which this document does not hold, so their handles
    /// carry no kind.
    fn carried(py: Python<'_>, finding: &d::InterferenceFinding) -> PyResult<Self> {
        Self::read(py, &|id| Var(id, None), finding)
    }

    fn read(
        py: Python<'_>,
        var: &dyn Fn(d::VarId) -> Var,
        finding: &d::InterferenceFinding,
    ) -> PyResult<Self> {
        let copy = |c: &d::CopyRef| (var(c.var), c.member);
        let (faces, unlocalized) = match &finding.overlap {
            d::Overlap::Bounded { faces } => (
                Some(
                    faces
                        .iter()
                        .map(|site| {
                            Ok(FaceSite {
                                copy: var(site.copy.var),
                                member: site.copy.member,
                                face: name_text(py, &site.face)?,
                            })
                        })
                        .collect::<PyResult<Vec<_>>>()?,
                ),
                None,
            ),
            d::Overlap::Unlocalized(why) => (None, Some(unlocalized_tag(why))),
        };
        Ok(Self {
            a: copy(&finding.a),
            b: copy(&finding.b),
            faces,
            unlocalized,
            quiet_by: finding.quiet.map(NodeId),
            quiet_at: finding
                .quiet_at
                .iter()
                .map(|&(k, at)| (var(k), at))
                .collect(),
            quietable: finding.quietable,
        })
    }
}

#[pymethods]
impl InterferenceFinding {
    /// The copy earlier in gather order: its placement's output
    /// variable.
    #[getter]
    fn a(&self) -> Var {
        self.a.0
    }

    /// Which member of `a`'s placement value it is.
    #[getter]
    fn a_member(&self) -> u32 {
        self.a.1
    }

    /// The other copy.
    #[getter]
    fn b(&self) -> Var {
        self.b.0
    }

    /// Which member of `b`'s placement value it is.
    #[getter]
    fn b_member(&self) -> u32 {
        self.b.1
    }

    /// The faces of both copies bounding the overlap, or `None` when
    /// the kernel could not bound it (`unlocalized` says why).
    #[getter]
    fn faces(&self) -> Option<Vec<FaceSite>> {
        self.faces.clone()
    }

    /// Why the overlap has no site — `refused`, `invalid`, `empty`,
    /// `unnamed`, `containment` or `lane` — or `None` when `faces` names it. An
    /// unlocalized overlap is loud and nothing quiets it.
    #[getter]
    fn unlocalized(&self) -> Option<&'static str> {
        self.unlocalized
    }

    /// The assertion that quiets it, or `None` when it is loud.
    #[getter]
    fn quiet_by(&self) -> Option<NodeId> {
        self.quiet_by
    }

    /// The member of `quiet_by`'s family that quiets it, each index and
    /// its value (D10 `[repetition]`): each member of an assertion whose
    /// reads reach an index quiets its own finding. Empty for any other.
    #[getter]
    fn quiet_at(&self) -> Vec<(Var, u32)> {
        self.quiet_at.clone()
    }

    /// Whether no assertion quiets it.
    #[getter]
    fn loud(&self) -> bool {
        self.quiet_by.is_none()
    }

    /// Whether an assertion the quieting rule accepts could quiet it, as
    /// the rule itself answers: some holding `gap` assertion over two of
    /// its bounding faces would quiet it. One no assertion could quiet
    /// refuses at no severity.
    #[getter]
    fn quietable(&self) -> bool {
        self.quietable
    }

    fn __repr__(&self) -> String {
        format!(
            "InterferenceFinding({}, {} face(s))",
            if self.loud() { "loud" } else { "quiet" },
            self.faces.as_ref().map_or(0, Vec::len)
        )
    }
}

/// Register the gather and the at-rest findings on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<InterferenceFinding>()?;
    m.add_class::<CarriedInterference>()?;
    m.add_class::<CarriedAtRestFinding>()?;
    m.add_class::<FaceSite>()?;
    m.add_class::<AtRestEvidence>()?;
    m.add_function(wrap_pyfunction!(product, m)?)?;
    m.add_function(wrap_pyfunction!(product_named, m)?)?;
    Ok(())
}
