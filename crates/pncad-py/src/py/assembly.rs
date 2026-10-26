//! **The whole-document gather and the at-rest gate** (A5).
//!
//! `product` is the gather of the document's world: every copy a
//! placement defines, in the placements' document order, as one body.
//! It answers "what IS this document", which for an assembly is the
//! only useful question — an assembly's nodes are placements, and no
//! single node's value is the assembly.
//!
//! `assemble` is the gather PLUS the check: the product's census
//! records every touch between copies, mated or not, as a row and a
//! record citing it, and the gate reads the census's verdict. That is
//! the answer to "is this assembly valid at rest", which the authoring
//! vocabulary can otherwise construct and never check.
//!
//! # What does not cross, and why
//!
//! `Assembly.contacts` — the certified record set — is a
//! `ContactRecords`, the kernel's own record vocabulary, and that
//! whole family stays off the Python surface (the census's
//! deliberately-unbound list). What the assembly answers instead is
//! its BODY and its product names: what a consumer measures, exports
//! and reads back.

use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::ErrorClass;
use crate::py::typed_err;
use crate::tags::{assembly_error_tag, product_error_tag, unlocalized_tag};
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
        E::Root(standing) => (
            id(&standing.node()),
            standing.through().map_or_else(none, |through| id(&through)),
            none(),
        ),
        E::Graft { node, .. } => (id(node), none(), none()),
        // The first failing placement, in gather order; every failing
        // placement and output is in the message.
        E::RootInvalid { findings } => (
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
    // The gather DECLARES NOTHING: it is the copies' solids side
    // by side, with no gate and no minted records, so the product
    // body is plain and `Body.validate_pseudomanifold` will report
    // any seam between two copies as undeclared. `assemble` is the
    // door that mints declarations over the same geometry.
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

// ---- The gate's payload types ----

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
fn route_fields(py: Python<'_>, route: &d::Route) -> (Py<PyAny>, Py<PyAny>) {
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

/// One at-rest refusal: the kernel's own finding, verbatim.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct AtRestFinding(d::AtRestFinding, SpokenFrom);

#[pymethods]
impl AtRestFinding {
    /// The kernel's finding, in its own words. The kernel's messages
    /// carry their own recourse, so nothing is appended here.
    fn __str__(&self) -> String {
        self.1.say(&self.0)
    }

    fn __repr__(&self) -> String {
        "AtRestFinding(..)".to_owned()
    }
}

/// An overlap a document BELOW this one reported between two of its own
/// copies, with the route this document reached it by: reported on
/// `Assembly.carried_interference`, never raised, quiet or loud as the
/// part's own assertions judged it.
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

/// One finding of an at-rest refusal a document BELOW this one raised,
/// with the route this document reached it by: a row of
/// `AssemblyError.findings` on the `carried_at_rest` arm.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct CarriedAtRestFinding(d::CarriedAtRestFinding, SpokenFrom);

#[pymethods]
impl CarriedAtRestFinding {
    /// The part's own finding, said as the part holds its nodes.
    #[getter]
    fn finding(&self) -> AtRestFinding {
        AtRestFinding(
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
    /// census found.
    fn __str__(&self) -> String {
        self.1.say(&self.0)
    }

    fn __repr__(&self) -> String {
        format!("CarriedAtRestFinding(of={})", self.0.route.of)
    }
}

/// A validated assembly: the gathered body and its product names.
///
/// Reaching one means the kernel's at-rest door PASSED over the
/// product and its records together. What that door checked is stated
/// at [`assemble`]; what this value carries is what a consumer then
/// measures, exports and names.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Assembly {
    body: Body,
    names: Vec<String>,
    interference: Vec<InterferenceFinding>,
    carried_interference: Vec<CarriedInterference>,
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

/// An overlap of two copies' material at rest: reported on
/// `Assembly.interference`, never raised, and quiet only under a
/// holding one-sided `gap` assertion at its site (`quiet_by`).
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct InterferenceFinding {
    a: (Var, u32),
    b: (Var, u32),
    faces: Option<Vec<FaceSite>>,
    unlocalized: Option<&'static str>,
    quiet_by: Option<NodeId>,
}

impl InterferenceFinding {
    fn of(py: Python<'_>, doc: &d::ProfileDoc, finding: &d::InterferenceFinding) -> PyResult<Self> {
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

    /// Whether no assertion quiets it.
    #[getter]
    fn loud(&self) -> bool {
        self.quiet_by.is_none()
    }

    fn __repr__(&self) -> String {
        format!(
            "InterferenceFinding({}, {} face(s))",
            if self.loud() { "loud" } else { "quiet" },
            self.faces.as_ref().map_or(0, Vec::len)
        )
    }
}

#[pymethods]
impl Assembly {
    /// The gathered aggregate body.
    #[getter]
    fn body(&self) -> Body {
        self.body.clone()
    }

    /// Its stable names, as opaque text — the product's own alphabet.
    #[getter]
    fn names(&self) -> Vec<String> {
        self.names.clone()
    }

    /// Every overlap between two copies' material, quiet or loud, in
    /// the census's order. An overlap is reported here and never
    /// raised.
    #[getter]
    fn interference(&self) -> Vec<InterferenceFinding> {
        self.interference.clone()
    }

    /// Every overlap a document BELOW this one reported between two of
    /// its own copies, once per part row, in gather order: the part's
    /// findings, reported here and never raised.
    #[getter]
    fn carried_interference(&self) -> Vec<CarriedInterference> {
        self.carried_interference.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "Assembly({} names, {} interference, {} carried interference)",
            self.names.len(),
            self.interference.len(),
            self.carried_interference.len()
        )
    }
}

/// Raise `AssemblyError` carrying the refusal's stable tag and the
/// arm's own payload.
fn assembly_err(py: Python<'_>, err: &d::AssemblyError, doc: Arc<d::ProfileDoc>) -> PyErr {
    use d::AssemblyError as E;
    // A gather refusal is not wrapped: the caller wants the gather's
    // own answer, and the wrapper adds nothing they can act on. It
    // still raises on THIS class — the door they called was the gate.
    let none = || py.None();
    let obj = |v: PyResult<Py<PyAny>>| v.unwrap_or_else(|_| py.None());
    let from = SpokenFrom::Doc(Arc::clone(&doc));
    let node = none();
    let findings = match err {
        // Each row carries its own route: `of` is the document to open
        // and `via` the instances this document reached it through. The
        // gate's own `through` stays `None`, because a list of rows has
        // no single route.
        E::CarriedAtRest { findings } => obj(findings
            .iter()
            .map(|f| CarriedAtRestFinding(f.clone(), from.clone()))
            .collect::<Vec<_>>()
            .into_pyobject(py)
            .map(|v| v.unbind().into_any())),
        E::Product(inner) => {
            let (node, through, name) = product_fields(py, inner);
            // The gather's payload rides under the gather's own
            // attribute names; the gate's `findings` stays `None`, so a
            // caller who branched on `variant` reads the fields that
            // tag actually carries.
            return typed_err(
                py,
                ErrorClass::Assembly,
                err.spoken(&*doc),
                &[
                    (
                        "variant",
                        PyString::new(py, assembly_error_tag(err))
                            .unbind()
                            .into_any(),
                    ),
                    ("node", node),
                    ("through", through),
                    ("name", name),
                    ("findings", none()),
                ],
            );
        }
        // The `⊔` is read by its output, not a node: it and its own
        // gather refusal are in the message.
        E::Assemble { .. } => none(),
        E::AtRest { findings } => obj(findings
            .iter()
            .map(|f| AtRestFinding(f.clone(), from.clone()))
            .collect::<Vec<_>>()
            .into_pyobject(py)
            .map(|v| v.unbind().into_any())),
    };
    typed_err(
        py,
        ErrorClass::Assembly,
        err.spoken(&*doc),
        &[
            (
                "variant",
                PyString::new(py, assembly_error_tag(err))
                    .unbind()
                    .into_any(),
            ),
            ("node", node),
            ("through", none()),
            ("name", none()),
            ("findings", findings),
        ],
    )
}

/// **The at-rest assembly gate**: gather the document's product, whose
/// census records every touch between copies, mated or not, and read
/// the census's verdict.
///
/// The answer to "is this assembly valid at rest" — the check the
/// authoring vocabulary can otherwise construct and never make. A
/// disjoint assembly (a flat-pack layout, say) passes outright with
/// nothing recorded.
///
/// `evaluation` must be an evaluation OF `doc`, and one that
/// RESOLVED: an instantiate node with no resolver produced no body,
/// so the gather refuses `root_failed` before the gate runs.
///
/// **One gather per evaluation.** The document's product is a pure
/// function of the (document, evaluation) pair `evaluation` captured
/// at `evaluate` and the run's tolerance, so it is gathered on the
/// first ask and shared with every other door that wants one
/// ([`crate::product_memo`]). Reusing an `Evaluation` is therefore
/// how a caller asks several questions for the price of one gather.
///
/// Raises `AssemblyError`, typed. `at_rest` is the census's verdict
/// against the product, `findings` every one of its findings in the
/// kernel's own sweep order; `carried_at_rest` is the same for a
/// document below this one, raised first; the remaining arms are the
/// gather's own tags.
#[pyfunction]
pub(crate) fn assemble(py: Python<'_>, doc: &Doc, evaluation: &Evaluation) -> PyResult<Assembly> {
    let tol = Tol::witness();
    evaluation.paired_with(doc).map_err(|m| {
        assembly_err(
            py,
            &d::AssemblyError::Product(Box::new(m.into())),
            evaluation.doc_shared(),
        )
    })?;
    let assembly = evaluation
        .gathered(|memo, doc, ev| crate::product_memo::assembly(memo, doc, ev, tol))
        .map_err(|err| assembly_err(py, &err, evaluation.doc_shared()))?;
    let names = assembly
        .names
        .iter()
        .map(|(name, _)| name_text(py, name))
        .collect::<PyResult<Vec<String>>>()?;
    let read = evaluation.doc_shared();
    let interference = assembly
        .interference
        .iter()
        .map(|finding| InterferenceFinding::of(py, &read, finding))
        .collect::<PyResult<Vec<_>>>()?;
    let carried_interference = assembly
        .carried_interference
        .iter()
        .map(|row| {
            Ok(CarriedInterference {
                finding: InterferenceFinding::carried(py, &row.finding)?,
                route: row.route.clone(),
                pin: row.pin,
            })
        })
        .collect::<PyResult<Vec<_>>>()?;
    Ok(Assembly {
        // The at-rest body keeps the record set the gate certified it
        // against — the copies' carried records plus those the census
        // recorded between them. Reaching an
        // `Assembly` at all means tier 3′ ALREADY passed over exactly
        // this pair, so `Assembly.body.validate_pseudomanifold()` is
        // the same verdict re-taken, un-attributed.
        body: Body::recorded(Arc::new(assembly.body), Arc::new(assembly.contacts)),
        names,
        interference,
        carried_interference,
    })
}

/// Register the gather and gate surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Assembly>()?;
    m.add_class::<InterferenceFinding>()?;
    m.add_class::<CarriedInterference>()?;
    m.add_class::<CarriedAtRestFinding>()?;
    m.add_class::<FaceSite>()?;
    m.add_class::<AtRestFinding>()?;
    m.add_function(wrap_pyfunction!(product, m)?)?;
    m.add_function(wrap_pyfunction!(product_named, m)?)?;
    m.add_function(wrap_pyfunction!(assemble, m)?)?;
    Ok(())
}
