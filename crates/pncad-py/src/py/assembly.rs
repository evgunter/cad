//! **The whole-document gather and the at-rest gate** (A5).
//!
//! `product` is the gather the document's explicit roots name: every
//! body-denoting root's solids, in root-list order, as one body. It
//! answers "what IS this document", which for an assembly is the only
//! useful question — an assembly's nodes are instances and mates, and
//! no single node's value is the assembly.
//!
//! `assemble` is the gather PLUS the check: it mints every solved
//! mate's declaration into the product's contact-record set and runs
//! the kernel's own at-rest door over the two together. That is the
//! answer to "is this assembly valid at rest", which the authoring
//! vocabulary can otherwise construct and never check.
//!
//! # The two refusal arms are two different facts
//!
//! `AssemblyError` with `variant == "at_rest"` is a verdict AGAINST
//! the document: a declaration the kernel refuted, or a contact
//! nothing declared. `variant == "uncertified"` is the declared
//! direction's FRONTIER: every finding is the census declining to
//! certify a face a declaration names, so nothing was refuted and
//! nothing was decided. A caller must tell them apart, which is why
//! they are two tags on one class rather than one tag with a flag.
//!
//! # What does not cross, and why
//!
//! `Assembly.contacts` — the certified record set — is a
//! `ContactRecords`, the kernel's own record vocabulary, and that
//! whole family stays off the Python surface (the census's
//! deliberately-unbound list). What the assembly answers instead is
//! its BODY, its product names, and its minted declarations: what a
//! consumer measures, exports and reads back.

use std::sync::Arc;

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::ErrorClass;
use crate::py::typed_err;
use crate::tags::{
    assembly_error_tag, attribution_tag, mint_refusal_tag, product_error_tag, refused_ref_tag,
};
use pncad::document as d;
use pncad::tolerance::Tol;

use super::doc::{Doc, NodeId, name_text};
use super::mate::MateSide;
use super::value::{Body, Evaluation};

/// Raise `ProductError` carrying the refusal's stable tag and the
/// arm's own payload.
///
/// Every attribute is set on every arm, `None` where the arm does not
/// carry it — the `WorkspaceError` posture: handling reads
/// `err.node` without first branching on `err.variant`.
pub(crate) fn product_err(py: Python<'_>, err: &d::ProductError) -> PyErr {
    let (node, through, name) = product_fields(py, err);
    typed_err(
        py,
        ErrorClass::Product,
        err.to_string(),
        &[
            (
                "variant",
                PyString::new(py, product_error_tag(err))
                    .unbind()
                    .into_any(),
            ),
            ("node", node),
            ("through", through),
            ("name", name),
        ],
    )
}

/// The gather refusal's payload, flattened.
fn product_fields(py: Python<'_>, err: &d::ProductError) -> (Py<PyAny>, Py<PyAny>, Py<PyAny>) {
    use d::ProductError as E;
    let id = |n: &d::RecipeNodeId| -> Py<PyAny> {
        Py::new(py, NodeId(*n))
            .map(|v| v.into_any())
            .unwrap_or_else(|_| py.None())
    };
    let text = |n: &pncad::prelude::StableName| -> Py<PyAny> {
        name_text(py, n)
            .map(|s| PyString::new(py, &s).unbind().into_any())
            .unwrap_or_else(|_| py.None())
    };
    let none = || py.None();
    match err {
        E::UnknownNode { node }
        | E::RootFailed { node }
        | E::Graft { node, .. }
        | E::SolidInvalid { node, .. } => (id(node), none(), none()),
        E::RootPoisoned { node, through } => (id(node), id(through), none()),
        E::Naming { node, name } => (id(node), none(), text(name)),
        // The placed node is the one the author acts on; the two roots
        // it sits under are in the message.
        E::PlacedUnderTwoRoots { placed, .. } => (id(placed), none(), none()),
        // The document-mismatch arm names two DOCUMENTS, which this
        // node/node/name triple cannot carry; the message states both.
        E::NoBodyRoots
        | E::ProductInvalid { .. }
        | E::ContactLineage { .. }
        | E::EvaluationOfAnotherDocument { .. } => (none(), none(), none()),
    }
}

/// The document's **product**: every body-denoting root's solids,
/// gathered in root-list order into one body.
///
/// This is what a document IS, and for an assembly it is the only
/// useful reading: an assembly's nodes are instances and mates, and no
/// single node's value is the assembly. Which roots are gathered is
/// the document's own ordered root list — read through `Doc.roots`,
/// set through `DocEdit.set_roots`.
///
/// A pure function of the root list and the evaluation: no ambient
/// state, so two evaluations of a root-neutral edit yield the same
/// solid order.
///
/// `evaluation` must be an evaluation OF `doc`, and the door CHECKS
/// it: an evaluation carries the id of the document it was run on,
/// and a foreign one raises `ProductError` with tag
/// `evaluation_of_another_document` before the first root is read.
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
/// Raises `ProductError`, typed: a root that failed, was poisoned or
/// is absent from this evaluation; a document whose roots denote no
/// body (`no_body_roots`); the kernel's graft and validity refusals.
/// All of the roots or none of them — there are no partial products.
#[pyfunction]
pub(crate) fn product(py: Python<'_>, doc: &Doc, evaluation: &Evaluation) -> PyResult<Body> {
    let tol = Tol::witness();
    // The gather DECLARES NOTHING: it is the root list's solids side
    // by side, with no gate and no minted records, so the product
    // body is plain and `Body.validate_pseudomanifold` will report
    // any seam between two roots as undeclared. `assemble` is the
    // door that mints declarations over the same geometry.
    evaluation
        .paired_with(doc)
        .map_err(|m| mispaired_product(py, m))?;
    evaluation
        .gathered(|memo, doc, ev| crate::product_memo::body(memo, doc, ev, tol))
        .map(|body| Body::plain(Arc::new(body)))
        .map_err(|err| product_err(py, &err))
}

/// A mispaired `(doc, evaluation)` as the gather's own refusal — the
/// one the memo path cannot inherit from a gather it does not reach.
fn mispaired_product(py: Python<'_>, m: d::Mispaired) -> PyErr {
    product_err(py, &m.into())
}

/// The product, with the stable names its entities answer to —
/// `product`'s sibling, same gather, one more field.
///
/// The names are the product's OWN alphabet: an instance's entity is
/// named through the instantiate node that placed it, which is what
/// makes "the third post's top cap" one name rather than a coordinate.
/// They cross as opaque text, like every other name in this library.
///
/// **One gather per evaluation.** The document's product is a pure
/// function of the (document, evaluation) pair `evaluation` captured
/// at `evaluate` and the run's tolerance, so it is gathered on the
/// first ask and shared with every other door that wants one
/// ([`crate::product_memo`]). Reusing an `Evaluation` is therefore
/// how a caller asks several questions for the price of one gather.
///
/// Raises `ProductError`, typed — including `product_naming` when two
/// roots' rows would name one aggregate entity.
#[pyfunction]
pub(crate) fn product_named(
    py: Python<'_>,
    doc: &Doc,
    evaluation: &Evaluation,
) -> PyResult<(Body, Vec<String>)> {
    let tol = Tol::witness();
    evaluation
        .paired_with(doc)
        .map_err(|m| mispaired_product(py, m))?;
    let (body, names) = evaluation
        .gathered(|memo, doc, ev| crate::product_memo::body_and_names(memo, doc, ev, tol))
        .map_err(|err| product_err(py, &err))?;
    let names = names
        .iter()
        .map(|name| name_text(py, name))
        .collect::<PyResult<Vec<String>>>()?;
    Ok((Body::plain(Arc::new(body)), names))
}

// ---- The gate's payload types ----

/// Why a mate reference named no product face.
///
/// Payload attributes present on every arm, `None` where inapplicable:
/// `at` (the operand a reference is read at when it is spelled there
/// but the operand is not a product root) and `width` (how many
/// entities a tie holds). There is no `kind`: a mate head is a face
/// by its type, so no refusal here reports what a head named
/// instead.
///
/// Each accessor is an exhaustive match with no wildcard, so a
/// refusal arm added kernel-side is a compile error here rather than
/// a reference every accessor silently answers `None` about.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct RefusedRef(d::RefusedRef);

#[pymethods]
impl RefusedRef {
    /// The stable tag: `ref_vanished`, `ref_read_below_a_root`,
    /// `ref_ambiguous`.
    #[getter]
    fn variant(&self) -> &'static str {
        refused_ref_tag(&self.0)
    }

    /// The operand the reference is read at, when its own table
    /// spells the name but it is not a root of the product — the
    /// product spells that entity at its roots, under a pattern as
    /// the instance row at the pattern node.
    #[getter]
    fn at(&self) -> Option<NodeId> {
        match self.0 {
            d::RefusedRef::ReadBelowARoot { at } => Some(NodeId(at)),
            d::RefusedRef::Vanished | d::RefusedRef::Ambiguous { .. } => None,
        }
    }

    /// How many entities a tie holds. A mate declaration must name
    /// ONE face, and a tie is never broken by picking.
    #[getter]
    fn width(&self) -> Option<u32> {
        match self.0 {
            d::RefusedRef::Ambiguous { width } => Some(width),
            d::RefusedRef::Vanished | d::RefusedRef::ReadBelowARoot { .. } => None,
        }
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("RefusedRef({:?})", self.variant())
    }
}

/// One declaration the gate minted from a solved mate: which mate,
/// which two references, and the class it asserts.
///
/// The face keys the kernel matched them to do NOT cross — arena keys
/// never leave the document layer — so what identifies the pair here
/// is the two stable names, which is the alphabet the mate was
/// authored in.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct MintedDeclaration(d::MintedDeclaration);

#[pymethods]
impl MintedDeclaration {
    /// The mate this declaration came from.
    #[getter]
    fn mate(&self) -> NodeId {
        NodeId(self.0.mate)
    }

    /// The `a` reference, as opaque name text.
    #[getter]
    fn a(&self, py: Python<'_>) -> PyResult<String> {
        name_text(py, &self.0.a)
    }

    /// The `b` reference, as opaque name text.
    #[getter]
    fn b(&self, py: Python<'_>) -> PyResult<String> {
        name_text(py, &self.0.b)
    }

    /// The declared contact class (trailing underscore: `class` is a
    /// Python keyword — the `FlushFinding.class_` precedent).
    #[getter]
    fn class_(&self, py: Python<'_>) -> PyResult<super::flush::ContactClass> {
        super::flush::contact_class(py, self.0.class)
    }

    fn __repr__(&self) -> String {
        format!("MintedDeclaration(mate={})", self.0.mate.0)
    }
}

/// **What a kernel finding says about the document's declarations.**
///
/// One value rather than a declaration plus a flag: the relation and
/// the declaration it names are decided together and cannot disagree.
///
/// `relation` is `"refuted"` (the kernel says the faces do not meet as
/// declared — a finding against the document), `"declined"` (the
/// census has no certifier lane for a face the declaration names, so
/// nothing was decided either way), or `"unattributed"` (no
/// declaration answers for the finding — an UNDECLARED contact, which
/// is by definition the hard error).
///
/// A declaration a document BELOW this one authored answers under
/// `"carried_refuted"` and `"carried_declined"`: the same two
/// relations, and a separate pair of tags because `declaration.mate`
/// is then a node of THAT document, not of the one the caller
/// gathered. `of` and `via` are what say which document and by what
/// path, so the file to open is readable and not just printable.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct Attribution(d::Attribution);

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
        .chain(route.via.iter().copied())
        .map(NodeId)
        .collect::<Vec<_>>()
        .into_pyobject(py)
        .map(|v| v.unbind().into_any())
        .unwrap_or_else(|_| py.None());
    (of, via)
}

#[pymethods]
impl Attribution {
    /// The stable tag: `refuted`, `declined`, `carried_refuted`,
    /// `carried_declined`, `unattributed`.
    #[getter]
    fn relation(&self) -> &'static str {
        attribution_tag(&self.0)
    }

    /// The declaration named, `None` for `unattributed`. Under a
    /// `carried_*` relation its `mate` is a node of `of`, not of the
    /// document that was gathered.
    #[getter]
    fn declaration(&self) -> Option<MintedDeclaration> {
        self.0.declaration().cloned().map(MintedDeclaration)
    }

    /// The document whose mate authored the declaration, as opaque
    /// id text. `None` where the declaration is the gathered
    /// document's own, and `None` for `unattributed`.
    #[getter]
    fn of(&self, py: Python<'_>) -> Option<Py<PyAny>> {
        self.0.route().map(|r| route_fields(py, r).0)
    }

    /// The instances this document reached it through, nearest first
    /// (`route_fields`). `None` where `of` is.
    #[getter]
    fn via(&self, py: Python<'_>) -> Option<Py<PyAny>> {
        self.0.route().map(|r| route_fields(py, r).1)
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("Attribution({:?})", self.relation())
    }
}

/// One at-rest refusal: what it says about the declarations, and the
/// kernel's own finding verbatim.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct AtRestFinding(d::AtRestFinding);

#[pymethods]
impl AtRestFinding {
    /// Which declaration the finding names, and in what relation.
    #[getter]
    fn attribution(&self) -> Attribution {
        Attribution(self.0.attribution.clone())
    }

    /// The finding composed the way the library renders one: the
    /// subject (the mate a user can act on) followed by the kernel's
    /// own story. The kernel's messages carry their own recourse, so
    /// nothing is appended here.
    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("AtRestFinding({:?})", attribution_tag(&self.0.attribution))
    }
}

/// One mate whose declaration the gather could not mint: which mate,
/// and why — a reference that named no product face (`why`), or a
/// class that carries no kernel record at rest (`class_`).
///
/// A row of `AssemblyError.refusals`, which is the WHOLE list the
/// gather recorded: a document with two broken mates answers with two
/// rows, so an author repairs both from one evaluation.
///
/// `variant` is the stable tag, and the other three are the arms'
/// payloads — present on the arm that carries them, `None` on the
/// other, so `getattr` never raises.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct MintRefusal(d::MintRefusal);

#[pymethods]
impl MintRefusal {
    /// The stable tag: `mate_reference_refused`, `no_at_rest_record`.
    #[getter]
    fn variant(&self) -> &'static str {
        mint_refusal_tag(&self.0)
    }

    /// The mate that did not mint. Both arms carry one.
    #[getter]
    fn mate(&self) -> NodeId {
        NodeId(self.0.mate())
    }

    /// Which side of the mate the refused reference is on, for a
    /// reference refusal.
    #[getter]
    fn side(&self) -> Option<MateSide> {
        match &self.0 {
            d::MintRefusal::Reference { side, .. } => Some(MateSide::from_kernel(*side)),
            d::MintRefusal::NoAtRestRecord { .. } => None,
        }
    }

    /// The reference that named no product face, as its stable name.
    #[getter]
    fn name(&self, py: Python<'_>) -> PyResult<Option<String>> {
        match &self.0 {
            d::MintRefusal::Reference { name, .. } => name_text(py, name).map(Some),
            d::MintRefusal::NoAtRestRecord { .. } => Ok(None),
        }
    }

    /// Why the reference did not resolve.
    ///
    /// `None` on the `no_at_rest_record` arm, which has a reason of a
    /// different KIND: not a refused reference but the class's own
    /// entry in the admission table. That reason is not re-minted
    /// here, because the table is where it is sourced and
    /// `class_admission(refusal.class_).why` is the same string the
    /// message carries. One home, asked by the door that owns it.
    #[getter]
    fn why(&self) -> Option<RefusedRef> {
        match &self.0 {
            d::MintRefusal::Reference { why, .. } => Some(RefusedRef(why.clone())),
            d::MintRefusal::NoAtRestRecord { .. } => None,
        }
    }

    /// The class that carries no kernel record at rest.
    #[getter]
    fn class_(&self, py: Python<'_>) -> PyResult<Option<super::flush::ContactClass>> {
        match &self.0 {
            d::MintRefusal::NoAtRestRecord { class, .. } => {
                super::flush::contact_class(py, *class).map(Some)
            }
            d::MintRefusal::Reference { .. } => Ok(None),
        }
    }

    /// The refusal in the library's own words, its own recourse
    /// included.
    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!(
            "MintRefusal({:?}, mate={})",
            self.variant(),
            self.0.mate().0
        )
    }
}

/// One mate a document BELOW this one could not mint, with the route
/// this document reached it by.
///
/// The mate is a node of `of`, not of the document that was gathered
/// — the same rule every foreign-mate row follows — so `of` is the
/// file to open and `via` the instances in between, nearest first.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct CarriedRefusal(d::CarriedRefusal);

#[pymethods]
impl CarriedRefusal {
    /// The inner document's own refusal, its `mate` a node of `of`.
    #[getter]
    fn refusal(&self) -> MintRefusal {
        MintRefusal(self.0.refusal.clone())
    }

    /// The instantiating node OF THIS DOCUMENT the row came through.
    #[getter]
    fn through(&self) -> NodeId {
        NodeId(self.0.route.through)
    }

    /// The document whose mate did not mint, as opaque id text.
    #[getter]
    fn of(&self, py: Python<'_>) -> Py<PyAny> {
        route_fields(py, &self.0.route).0
    }

    /// The instances this document reached it through, nearest first.
    #[getter]
    fn via(&self, py: Python<'_>) -> Py<PyAny> {
        route_fields(py, &self.0.route).1
    }

    /// The row in the library's own words: which document, what it
    /// could not mint, and the repair.
    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!(
            "CarriedRefusal(mate={}, of={})",
            self.0.refusal.mate().0,
            self.0.route.of
        )
    }
}

/// A validated assembly: the gathered body, its product names, and one
/// minted declaration per solved mate.
///
/// Reaching one means the kernel's at-rest door PASSED over the
/// product and its records together. What that door checked is stated
/// at [`assemble`]; what this value carries is what a consumer then
/// measures, exports and names.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Assembly {
    body: Body,
    names: Vec<String>,
    minted: Vec<MintedDeclaration>,
    carried: Vec<CarriedDeclaration>,
}

/// One declaration a document BELOW this one authored, certified here
/// with everything else the gate was given.
///
/// The same rule as the foreign-mate arms above: the mate is a node of
/// `of`, so `of` and `via` travel with it.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct CarriedDeclaration(d::CarriedDeclaration);

#[pymethods]
impl CarriedDeclaration {
    /// The declaration, its `mate` a node of `of`.
    #[getter]
    fn declaration(&self) -> MintedDeclaration {
        MintedDeclaration(self.0.declaration.clone())
    }

    /// The document whose mate authored it, as opaque id text.
    #[getter]
    fn of(&self, py: Python<'_>) -> Py<PyAny> {
        route_fields(py, &self.0.route).0
    }

    /// The instances this document reached it through, nearest first.
    #[getter]
    fn via(&self, py: Python<'_>) -> Py<PyAny> {
        route_fields(py, &self.0.route).1
    }

    fn __repr__(&self) -> String {
        format!(
            "CarriedDeclaration(mate={}, of={})",
            self.0.declaration.mate.0, self.0.route.of
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

    /// One declaration per mate THIS document's gate minted. Empty for
    /// a mate-less assembly, which is what a disjoint layout is.
    #[getter]
    fn minted(&self) -> Vec<MintedDeclaration> {
        self.minted.clone()
    }

    /// One row per declaration a document BELOW this one authored, so
    /// a certified assembly can say which inner mates its verdict
    /// answered for.
    #[getter]
    fn carried(&self) -> Vec<CarriedDeclaration> {
        self.carried.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "Assembly({} names, {} minted declaration(s), {} carried)",
            self.names.len(),
            self.minted.len(),
            self.carried.len()
        )
    }
}

/// Raise `AssemblyError` carrying the refusal's stable tag and the
/// arm's own payload.
fn assembly_err(py: Python<'_>, err: &d::AssemblyError) -> PyErr {
    use d::AssemblyError as E;
    // A gather refusal is not wrapped: the caller wants the gather's
    // own answer, and the wrapper adds nothing they can act on. It
    // still raises on THIS class — the door they called was the gate.
    let none = || py.None();
    let obj = |v: PyResult<Py<PyAny>>| v.unwrap_or_else(|_| py.None());
    let (refusals, findings) = match err {
        // The carried arm carries FOREIGN mates, so every row carries
        // its own route: `of` is the document to open and `via` the
        // instances this document reached it through, and the row's
        // `mate` is that document's node — a bare id with no document
        // is not something a caller can look up, and the route is what
        // makes it one. The gate's own `of`/`via`/`through` stay
        // `None`, because a list of rows has no single route.
        E::CarriedMintRefusal { refusals } => (
            obj(refusals
                .iter()
                .map(|r| CarriedRefusal(r.clone()))
                .collect::<Vec<_>>()
                .into_pyobject(py)
                .map(|v| v.unbind().into_any())),
            none(),
        ),
        E::Product(inner) => {
            let (node, through, name) = product_fields(py, inner);
            // The gather's payload rides under the gather's own
            // attribute names; the gate's five stay `None`, so a
            // caller who branched on `variant` reads the fields that
            // tag actually carries.
            return typed_err(
                py,
                ErrorClass::Assembly,
                err.to_string(),
                &[
                    (
                        "variant",
                        PyString::new(py, assembly_error_tag(err))
                            .unbind()
                            .into_any(),
                    ),
                    ("node", node),
                    ("through", through),
                    ("of", none()),
                    ("via", none()),
                    ("name", name),
                    ("refusals", none()),
                    ("findings", none()),
                ],
            );
        }
        E::Mint { refusals } => (
            obj(refusals
                .iter()
                .map(|r| MintRefusal(r.clone()))
                .collect::<Vec<_>>()
                .into_pyobject(py)
                .map(|v| v.unbind().into_any())),
            none(),
        ),
        E::AtRest { findings } | E::Uncertified { findings, .. } => (
            none(),
            obj(findings
                .iter()
                .map(|f| AtRestFinding(f.clone()))
                .collect::<Vec<_>>()
                .into_pyobject(py)
                .map(|v| v.unbind().into_any())),
        ),
    };
    typed_err(
        py,
        ErrorClass::Assembly,
        err.to_string(),
        &[
            (
                "variant",
                PyString::new(py, assembly_error_tag(err))
                    .unbind()
                    .into_any(),
            ),
            ("node", none()),
            ("through", none()),
            ("of", none()),
            ("via", none()),
            ("name", none()),
            ("refusals", refusals),
            ("findings", findings),
        ],
    )
}

/// **The at-rest assembly gate**: gather the document's product, mint
/// every solved mate's declaration into its contact records, and run
/// the kernel's own at-rest door over the two together.
///
/// The answer to "is this assembly valid at rest" — the check the
/// authoring vocabulary can otherwise construct and never make. A
/// disjoint assembly (a flat-pack layout, say) passes outright with
/// nothing minted; a mated one is checked against what its mates
/// declared.
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
/// Raises `AssemblyError`, typed. Read `variant` before anything else
/// — the two verdict arms are different facts:
///
/// * `at_rest` — a finding AGAINST the document. At least one
///   declaration was refuted, or a contact nothing declared was
///   found. `findings` carries every one, in the kernel's own sweep
///   order. A mixed refusal lands here: one refuted declaration makes
///   this a refusal of the document however many declines ride along.
/// * `uncertified` — the declared direction's FRONTIER. Nothing was
///   refuted and nothing was undeclared; every finding is the census
///   declining to certify a face a declaration names, so NOTHING was
///   decided about this geometry either way. Today a declared
///   cross-instance pair whose two descriptions share no structural
///   chart ends here whatever its geometry.
///
/// The remaining arms refuse before any verdict: `unminted_mates`
/// (this document's own mates that did not mint), `carried_mint_
/// refusal` (the same for mates of documents below it), and the
/// gather's own tags. Both mint arms answer with `refusals` — EVERY
/// mate that did not mint, in document order, each row carrying its
/// own word: `mate_reference_refused` (the mate named no product face
/// — `why` says which way) or `no_at_rest_record` (the class mints
/// nothing at rest; ask `class_admission` BEFORE authoring).
#[pyfunction]
pub(crate) fn assemble(py: Python<'_>, doc: &Doc, evaluation: &Evaluation) -> PyResult<Assembly> {
    let tol = Tol::witness();
    evaluation
        .paired_with(doc)
        .map_err(|m| assembly_err(py, &d::AssemblyError::Product(Box::new(m.into()))))?;
    let assembly = evaluation
        .gathered(|memo, doc, ev| crate::product_memo::assembly(memo, doc, ev, tol))
        .map_err(|err| assembly_err(py, &err))?;
    let names = assembly
        .names
        .iter()
        .map(|(name, _)| name_text(py, name))
        .collect::<PyResult<Vec<String>>>()?;
    Ok(Assembly {
        // The at-rest body keeps the record set the gate certified it
        // against — the parts' own carried declarations (D-1) plus
        // this document's minted mate declarations (D-2). Reaching an
        // `Assembly` at all means tier 3′ ALREADY passed over exactly
        // this pair, so `Assembly.body.validate_pseudomanifold()` is
        // the same verdict re-taken, un-attributed.
        body: Body::declared(Arc::new(assembly.body), Arc::new(assembly.contacts)),
        names,
        minted: assembly.minted.into_iter().map(MintedDeclaration).collect(),
        carried: assembly
            .carried
            .into_iter()
            .map(CarriedDeclaration)
            .collect(),
    })
}

/// Register the gather and gate surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Assembly>()?;
    m.add_class::<MintedDeclaration>()?;
    m.add_class::<CarriedDeclaration>()?;
    m.add_class::<Attribution>()?;
    m.add_class::<AtRestFinding>()?;
    m.add_class::<RefusedRef>()?;
    m.add_class::<MintRefusal>()?;
    m.add_class::<CarriedRefusal>()?;
    m.add_function(wrap_pyfunction!(product, m)?)?;
    m.add_function(wrap_pyfunction!(product_named, m)?)?;
    m.add_function(wrap_pyfunction!(assemble, m)?)?;
    Ok(())
}
