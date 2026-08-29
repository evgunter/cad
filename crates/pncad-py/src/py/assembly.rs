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
use crate::tags::{assembly_error_tag, product_error_tag, refused_ref_tag};
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
        E::NoBodyRoots | E::ProductInvalid { .. } | E::ContactLineage { .. } => {
            (none(), none(), none())
        }
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
/// `evaluation` must be an evaluation OF `doc` — the gather looks each
/// root up by node id, and node ids are minted per document, so a
/// foreign evaluation refuses `unknown_node` rather than answering
/// about the wrong document.
///
/// Raises `ProductError`, typed: a root that failed, was poisoned or
/// is absent from this evaluation; a document whose roots denote no
/// body (`no_body_roots`); the kernel's graft and validity refusals.
/// All of the roots or none of them — there are no partial products.
#[pyfunction]
pub(crate) fn product(py: Python<'_>, doc: &Doc, evaluation: &Evaluation) -> PyResult<Body> {
    let tol = Tol::witness();
    d::product(&doc.inner, &evaluation.inner, tol)
        .map(|body| Body {
            inner: Arc::new(body),
        })
        .map_err(|err| product_err(py, &err))
}

/// The product, with the stable names its entities answer to —
/// `product`'s sibling, same gather, one more field.
///
/// The names are the product's OWN alphabet: an instance's entity is
/// named through the instantiate node that placed it, which is what
/// makes "the third post's top cap" one name rather than a coordinate.
/// They cross as opaque text, like every other name in this library.
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
    let (body, names) = d::product_named(&doc.inner, &evaluation.inner, tol)
        .map_err(|err| product_err(py, &err))?;
    let names = names
        .iter()
        .map(|(name, _)| name_text(py, name))
        .collect::<PyResult<Vec<String>>>()?;
    Ok((
        Body {
            inner: Arc::new(body),
        },
        names,
    ))
}

// ---- The gate's payload types ----

/// Why a mate reference named no product face.
///
/// Payload attributes present on every arm, `None` where inapplicable:
/// `width` (how many entities a tie holds) and `kind` (what a
/// non-face reference did name).
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct RefusedRef(d::RefusedRef);

#[pymethods]
impl RefusedRef {
    /// The stable tag: `ref_node_gone`, `ref_vanished`,
    /// `ref_ambiguous`, `ref_not_a_face`.
    #[getter]
    fn variant(&self) -> &'static str {
        refused_ref_tag(&self.0)
    }

    /// How many entities a tie holds. A mate declaration must name
    /// ONE face, and a tie is never broken by picking.
    #[getter]
    fn width(&self) -> Option<u32> {
        match self.0 {
            d::RefusedRef::Ambiguous { width } => Some(width),
            _ => None,
        }
    }

    /// What the reference did name, when it resolved to something
    /// that is not a face: `"face"`, `"edge"`, `"vertex"`, `"body"`.
    #[getter]
    fn kind(&self) -> Option<&'static str> {
        match self.0 {
            d::RefusedRef::NotAFace { kind } => Some(entity_kind_tag(kind)),
            _ => None,
        }
    }

    fn __str__(&self) -> String {
        self.0.to_string()
    }

    fn __repr__(&self) -> String {
        format!("RefusedRef({:?})", self.variant())
    }
}

/// The stable tag for an entity kind. Exhaustive over the kernel
/// enum, so a kind added there stops this build.
fn entity_kind_tag(kind: pncad::prelude::EntityKind) -> &'static str {
    use pncad::prelude::EntityKind as K;
    match kind {
        K::Face => "face",
        K::Edge => "edge",
        K::Vertex => "vertex",
        K::Body => "body",
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
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct Attribution(d::Attribution);

#[pymethods]
impl Attribution {
    /// The stable tag: `refuted`, `declined`, `unattributed`.
    #[getter]
    fn relation(&self) -> &'static str {
        match self.0 {
            d::Attribution::Refuted(_) => "refuted",
            d::Attribution::Declined(_) => "declined",
            d::Attribution::Unattributed => "unattributed",
        }
    }

    /// The declaration named, `None` for `unattributed`.
    #[getter]
    fn declaration(&self) -> Option<MintedDeclaration> {
        self.0.declaration().cloned().map(MintedDeclaration)
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
        format!(
            "AtRestFinding({:?})",
            match self.0.attribution {
                d::Attribution::Refuted(_) => "refuted",
                d::Attribution::Declined(_) => "declined",
                d::Attribution::Unattributed => "unattributed",
            }
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

    /// One declaration per mate the gate minted. Empty for a
    /// mate-less assembly, which is what a disjoint layout is.
    #[getter]
    fn minted(&self) -> Vec<MintedDeclaration> {
        self.minted.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "Assembly({} names, {} minted declaration(s))",
            self.names.len(),
            self.minted.len()
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
    let (mate, side, name, why, class_, findings) = match err {
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
                    ("name", name),
                    ("mate", none()),
                    ("side", none()),
                    ("why", none()),
                    ("class_", none()),
                    ("findings", none()),
                ],
            );
        }
        E::Reference {
            mate,
            side,
            name,
            why,
        } => (
            obj(Py::new(py, NodeId(*mate)).map(|v| v.into_any())),
            obj(Py::new(py, MateSide::from_kernel(*side)).map(|v| v.into_any())),
            obj(name_text(py, name).map(|s| PyString::new(py, &s).unbind().into_any())),
            obj(Py::new(py, RefusedRef(why.clone())).map(|v| v.into_any())),
            none(),
            none(),
        ),
        E::NoAtRestRecord { mate, class, .. } => (
            obj(Py::new(py, NodeId(*mate)).map(|v| v.into_any())),
            none(),
            none(),
            none(),
            obj(super::flush::contact_class(py, *class)
                .and_then(|c| Py::new(py, c).map(|v| v.into_any()))),
            none(),
        ),
        E::AtRest { findings } | E::Uncertified { findings, .. } => (
            none(),
            none(),
            none(),
            none(),
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
            ("name", name),
            ("mate", mate),
            ("side", side),
            ("why", why),
            ("class_", class_),
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
/// The remaining arms refuse before any verdict: `mate_reference_
/// refused` (a mate named no product face — `why` says which way),
/// `no_at_rest_record` (the class mints nothing at rest; ask
/// `class_admission` BEFORE authoring), and the gather's own tags.
#[pyfunction]
pub(crate) fn assemble(py: Python<'_>, doc: &Doc, evaluation: &Evaluation) -> PyResult<Assembly> {
    let tol = Tol::witness();
    let assembly =
        d::assemble(&doc.inner, &evaluation.inner, tol).map_err(|err| assembly_err(py, &err))?;
    let names = assembly
        .names
        .iter()
        .map(|(name, _)| name_text(py, name))
        .collect::<PyResult<Vec<String>>>()?;
    Ok(Assembly {
        body: Body {
            inner: Arc::new(assembly.body),
        },
        names,
        minted: assembly.minted.into_iter().map(MintedDeclaration).collect(),
    })
}

/// Register the gather and gate surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Assembly>()?;
    m.add_class::<MintedDeclaration>()?;
    m.add_class::<Attribution>()?;
    m.add_class::<AtRestFinding>()?;
    m.add_class::<RefusedRef>()?;
    m.add_function(wrap_pyfunction!(product, m)?)?;
    m.add_function(wrap_pyfunction!(product_named, m)?)?;
    m.add_function(wrap_pyfunction!(assemble, m)?)?;
    Ok(())
}
