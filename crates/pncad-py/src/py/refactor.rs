//! **The recorded refactorings** (`split`, `inline`) and **the
//! pin-update door** (`update_references`, `mixed_pins`).
//!
//! Both families are PURE. `split` and `inline` hand back the new
//! document VALUES plus the ordinary recorded edits that produce them,
//! and mutate nothing; `update_references` hands back an edit list and
//! applies none of it. That is what makes each of them atomic at the
//! caller's single step — there is no partially applied state to roll
//! back from, because the caller applies the whole list or none of it.
//!
//! Persisting a result is the store's write side (`Workspace.create`
//! for a new part document, `Workspace.resave` for a rewritten one).
//!
//! # What store state each door reads, and when
//!
//! Stated at the door rather than left to be discovered, because a
//! door whose answer depends on a snapshot nobody named is a door
//! whose contract has a hole in it (issue #1185's class — "an
//! argument that silently voids another argument's gate"):
//!
//! * `update_references(doc, id, new_pin)` reads **no store at all**.
//!   It is a pure function of the document and the pin the CALLER
//!   supplies; whether that pin names content anything holds is not
//!   asked here and cannot be. Where the pin came from, and how stale
//!   it is, is the caller's fact to keep.
//! * `Workspace.update_to_store(doc, id)` reads the store **once, at
//!   the moment of the call**, recomputing the current pin from disk.
//!   The edits it returns carry that pin as a literal, so a resave
//!   between this call and the caller's `apply` leaves the applied pin
//!   naming the older version — the edits are a snapshot, not a
//!   subscription.
//! * `mixed_pins(doc)` reads no store either: it reports what the
//!   DOCUMENT says, never what a store holds.
//! * `inline(doc, instance, resolver)` is the one door here that
//!   crosses the seam, and it crosses it AT THE CALL: the referenced
//!   document is resolved from the workspace as it stands, under the
//!   full pin gate, so a stale pin refuses (`part_pin_mismatch`)
//!   rather than splicing the version on disk.
//!
//! Applying a pin-update edit never re-checks anything against a
//! store: a pin is recipe data, and whether it resolves is
//! evaluation's question, through the seam vocabulary that names both
//! pins.

use std::collections::BTreeSet;

use pyo3::prelude::*;
use pyo3::types::PyString;

use crate::errors::ErrorClass;
use crate::py::typed_err;
use crate::tags::{inline_error_tag, split_error_tag, update_error_tag};
use pncad::document as d;
use pncad::tolerance::Tol;

use super::doc::{Doc, DocEdit, NodeId, name_text};
use super::store::ContentPin;

/// Parse the canonical 32-hex-digit identity spelling — the store
/// module's door, shared so an id is one alphabet everywhere.
fn document_id(text: &str) -> PyResult<d::DocumentId> {
    d::DocumentId::parse_hex(text).ok_or_else(|| {
        pyo3::exceptions::PyValueError::new_err(format!(
            "not a document id: {text:?} — an id is exactly 32 lowercase \
             hex digits, the spelling `Doc.id` answers"
        ))
    })
}

// ---- The split seam's record ----

/// One declaration that crossed a split's cut: a mate whose two ends
/// landed on opposite sides.
///
/// `outer` is the reference that stayed in the remainder; `inner` is
/// the reference that moved into the part, spelled in the PART's own
/// names — unwrapped, because that is what the part's product answers
/// to. The wrapped form is what the remainder's mate now reads, and
/// re-wrapping is the split's own rebind, so storing it twice would be
/// storing a derivable fact.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct InterfaceCrossing(d::InterfaceCrossing);

#[pymethods]
impl InterfaceCrossing {
    /// The stable tag. One arm today, `mate`: a crossing is whatever
    /// KIND of edge crossed, and mates are the only kind that can.
    #[getter]
    fn variant(&self) -> &'static str {
        match self.0 {
            d::InterfaceCrossing::Mate { .. } => "mate",
        }
    }

    /// The crossing mate, in the remainder.
    #[getter]
    fn mate(&self) -> NodeId {
        match self.0 {
            d::InterfaceCrossing::Mate { mate, .. } => NodeId(mate),
        }
    }

    /// The class the crossing declares.
    #[getter]
    fn class_(&self, py: Python<'_>) -> PyResult<super::flush::ContactClass> {
        match self.0 {
            d::InterfaceCrossing::Mate { class, .. } => super::flush::contact_class(py, class),
        }
    }

    /// The remainder-side reference, as opaque name text.
    #[getter]
    fn outer(&self, py: Python<'_>) -> PyResult<String> {
        match &self.0 {
            d::InterfaceCrossing::Mate { outer, .. } => name_text(py, outer),
        }
    }

    /// The part-side reference, in the PART's own names.
    #[getter]
    fn inner(&self, py: Python<'_>) -> PyResult<String> {
        match &self.0 {
            d::InterfaceCrossing::Mate { inner, .. } => name_text(py, inner),
        }
    }

    fn __repr__(&self) -> String {
        format!("InterfaceCrossing({:?})", self.variant())
    }
}

/// The interface record of an instantiate seam: the declarations that
/// crossed the cut when the referenced document was split out.
///
/// Ordinary node data, recorded by the split that minted the instance.
/// **Empty for a directly-authored instance** — an instance you author
/// by hand crosses nothing — which is why `Node.instantiate_part` has
/// no way to supply one: only a split knows what crossed its cut.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct InterfaceRecord(pub(crate) d::InterfaceRecord);

#[pymethods]
impl InterfaceRecord {
    /// The crossings, in the deterministic order the split collected
    /// them (the pre-split document's mate order).
    #[getter]
    fn crossings(&self) -> Vec<InterfaceCrossing> {
        self.0
            .crossings
            .iter()
            .cloned()
            .map(InterfaceCrossing)
            .collect()
    }

    fn __len__(&self) -> usize {
        self.0.crossings.len()
    }

    fn __repr__(&self) -> String {
        format!("InterfaceRecord({} crossing(s))", self.0.crossings.len())
    }
}

// ---- split ----

/// Raise `SplitError` carrying the refusal's stable tag and payload.
fn split_err(py: Python<'_>, err: &d::SplitError) -> PyErr {
    use d::SplitError as E;
    let id = |n: &d::RecipeNodeId| -> Py<PyAny> {
        Py::new(py, NodeId(*n))
            .map(|v| v.into_any())
            .unwrap_or_else(|_| py.None())
    };
    let text = |s: &str| -> Py<PyAny> { PyString::new(py, s).unbind().into_any() };
    let named = |n: &pncad::prelude::StableName| -> Py<PyAny> {
        name_text(py, n)
            .map(|s| text(&s))
            .unwrap_or_else(|_| py.None())
    };
    let none = || py.None();
    let (node, consumer, input, gauge, instance, param, name, doc_id) = match err {
        E::EmptyCut => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        E::UnknownCutNode { id: n } => (
            id(n),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        E::PartIdCollides { id: which } => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            text(&which.hex()),
        ),
        E::SeveredEdge {
            consumer: c,
            input: i,
            ..
        } => (none(), id(c), id(i), none(), none(), none(), none(), none()),
        E::TornCluster {
            gauge: g,
            instance: i,
            ..
        } => (none(), none(), none(), id(g), id(i), none(), none(), none()),
        E::UncutParamReference {
            param: p,
            cut_node,
            kept_node,
        } => (
            id(cut_node),
            none(),
            id(kept_node),
            none(),
            none(),
            text(&p.0),
            none(),
            none(),
        ),
        E::PartNameReachesRemainder { node: n, name } => (
            id(n),
            none(),
            none(),
            none(),
            none(),
            none(),
            named(name),
            none(),
        ),
        E::NameStraddlesCut { name } | E::BodyNameCrossesCut { name } => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            named(name),
            none(),
        ),
        E::Pin { .. } | E::PartEdit { .. } | E::RemainderEdit { .. } => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
    };
    typed_err(
        py,
        ErrorClass::Split,
        err.to_string(),
        &[
            (
                "variant",
                PyString::new(py, split_error_tag(err)).unbind().into_any(),
            ),
            ("node", node),
            ("consumer", consumer),
            ("input", input),
            ("gauge", gauge),
            ("instance", instance),
            ("param", param),
            ("name", name),
            ("id", doc_id),
        ],
    )
}

/// What a split produced: the two document VALUES and the recorded
/// edits that make each.
///
/// Nothing is persisted and nothing is mutated. Writing the part
/// document into a store is `Workspace.create`; writing the remainder
/// back over the original is `Workspace.resave`.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct SplitOutcome {
    remainder: d::ProfileDoc,
    part: d::ProfileDoc,
    remainder_edits: Vec<d::DocEdit<d::ProfileProgram>>,
    part_edits: Vec<d::DocEdit<d::ProfileProgram>>,
    instance: NodeId,
    node_map: Vec<(NodeId, NodeId)>,
}

#[pymethods]
impl SplitOutcome {
    /// The original document with the cut nodes replaced by ONE
    /// instance of the new part.
    #[getter]
    fn remainder(&self) -> Doc {
        Doc {
            inner: self.remainder.clone(),
            maintenance: Vec::new(),
        }
    }

    /// The new part document, carrying the cut nodes.
    #[getter]
    fn part(&self) -> Doc {
        Doc {
            inner: self.part.clone(),
            maintenance: Vec::new(),
        }
    }

    /// The ordinary recorded edits that turn the original into the
    /// remainder.
    #[getter]
    fn remainder_edits(&self) -> Vec<DocEdit> {
        self.remainder_edits
            .iter()
            .cloned()
            .map(|inner| DocEdit { inner })
            .collect()
    }

    /// The ordinary recorded edits that build the part from empty.
    #[getter]
    fn part_edits(&self) -> Vec<DocEdit> {
        self.part_edits
            .iter()
            .cloned()
            .map(|inner| DocEdit { inner })
            .collect()
    }

    /// The instantiate node left behind in the remainder.
    #[getter]
    fn instance(&self) -> NodeId {
        self.instance
    }

    /// Cut node → its id in the part document, as pairs in the part's
    /// own order.
    #[getter]
    fn node_map(&self) -> Vec<(NodeId, NodeId)> {
        self.node_map.clone()
    }

    fn __repr__(&self) -> String {
        format!("SplitOutcome(instance={})", self.instance.0.0)
    }
}

/// Cut a closed node set out into a NEW document, leaving one instance
/// of it behind — the first-class recorded refactoring.
///
/// `part_id` is the new document's identity, supplied by the caller
/// (`random_document_id()` for interactive authoring): identity is
/// never defaulted, and a fresh one is what lets both documents live
/// in one store.
///
/// The cut must be **ancestor- and consumer-closed** and a union of
/// WHOLE placement clusters. Everything else refuses typed, naming the
/// offending edge, cluster, parameter or name.
///
/// Pure: `doc` is untouched, nothing is written, and the two documents
/// come back as values with the edits that produce them.
///
/// Raises `SplitError`, typed.
#[pyfunction]
pub(crate) fn split(
    py: Python<'_>,
    doc: &Doc,
    cut: Vec<NodeId>,
    part_id: &str,
) -> PyResult<SplitOutcome> {
    let tol = Tol::witness();
    let part_id = document_id(part_id)?;
    let set: BTreeSet<d::RecipeNodeId> = cut.iter().map(|n| n.0).collect();
    let out = d::split(&doc.inner, &set, part_id, tol).map_err(|err| split_err(py, &err))?;
    Ok(SplitOutcome {
        remainder: out.remainder,
        part: out.part,
        remainder_edits: out.remainder_edits,
        part_edits: out.part_edits,
        instance: NodeId(out.instance),
        node_map: out
            .node_map
            .into_iter()
            .map(|(a, b)| (NodeId(a), NodeId(b)))
            .collect(),
    })
}

// ---- inline ----

/// Raise `InlineError` carrying the refusal's stable tag and payload.
fn inline_err(py: Python<'_>, err: &d::InlineError) -> PyErr {
    use d::InlineError as E;
    let id = |n: &d::RecipeNodeId| -> Py<PyAny> {
        Py::new(py, NodeId(*n))
            .map(|v| v.into_any())
            .unwrap_or_else(|_| py.None())
    };
    let text = |s: &str| -> Py<PyAny> { PyString::new(py, s).unbind().into_any() };
    let named = |n: &pncad::prelude::StableName| -> Py<PyAny> {
        name_text(py, n)
            .map(|s| text(&s))
            .unwrap_or_else(|_| py.None())
    };
    let number = |v: f64| -> Py<PyAny> {
        v.into_pyobject(py)
            .map(|b| b.unbind().into_any())
            .unwrap_or_else(|_| py.None())
    };
    let none = || py.None();
    let (node, by, name, param, key, root, host_eps, part_eps) = match err {
        E::UnknownNode { id: n } => (
            id(n),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        E::NotAnInstance { node: n } => (
            id(n),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        E::InstanceConsumed { node: n, by: b } => {
            (id(n), id(b), none(), none(), none(), none(), none(), none())
        }
        E::Unresolved { failure } => (
            none(),
            none(),
            none(),
            none(),
            text(&failure.message),
            none(),
            none(),
            none(),
        ),
        E::EpsilonSeam {
            host_eps: h,
            part_eps: p,
        } => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            number(*h),
            number(*p),
        ),
        E::PartCarriesMetadata { key: k } => (
            none(),
            none(),
            none(),
            none(),
            text(k),
            none(),
            none(),
            none(),
        ),
        E::ParamConflict { param: p } => (
            none(),
            none(),
            none(),
            text(&p.0),
            none(),
            none(),
            none(),
            none(),
        ),
        E::UnplaceableFrame { root: r } => (
            none(),
            none(),
            none(),
            none(),
            none(),
            id(r),
            none(),
            none(),
        ),
        E::InstanceBodyNameReferenced { name }
        | E::ForeignInstanceName { name }
        | E::StrandedPartName { name } => (
            none(),
            none(),
            named(name),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
        E::Edit { .. } => (
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
            none(),
        ),
    };
    typed_err(
        py,
        ErrorClass::Inline,
        err.to_string(),
        &[
            (
                "variant",
                PyString::new(py, inline_error_tag(err)).unbind().into_any(),
            ),
            ("node", node),
            ("by", by),
            ("name", name),
            ("param", param),
            ("key", key),
            ("root", root),
            ("host_epsilon", host_eps),
            ("part_epsilon", part_eps),
        ],
    )
}

/// What an inline produced: the spliced document value and the
/// recorded edits that make it.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct InlineOutcome {
    doc: d::ProfileDoc,
    edits: Vec<d::DocEdit<d::ProfileProgram>>,
    node_map: Vec<(NodeId, NodeId)>,
}

#[pymethods]
impl InlineOutcome {
    /// The host document with the instance replaced by the referenced
    /// document's own nodes.
    #[getter]
    fn doc(&self) -> Doc {
        Doc {
            inner: self.doc.clone(),
            maintenance: Vec::new(),
        }
    }

    /// The ordinary recorded edits that produce it.
    #[getter]
    fn edits(&self) -> Vec<DocEdit> {
        self.edits
            .iter()
            .cloned()
            .map(|inner| DocEdit { inner })
            .collect()
    }

    /// Part node → its id in the spliced document.
    #[getter]
    fn node_map(&self) -> Vec<(NodeId, NodeId)> {
        self.node_map.clone()
    }

    fn __repr__(&self) -> String {
        format!("InlineOutcome({} edit(s))", self.edits.len())
    }
}

/// Splice a referenced document back in, replacing the instantiate
/// node with the part's own nodes — `split`'s inverse.
///
/// `resolver` crosses the document seam **at this call**, under the
/// full pin gate: a reference whose pinned version is not what the
/// store holds refuses (`part_pin_mismatch`), never silently splices
/// the version on disk. That is the same gate `evaluate(resolver=)`
/// runs, spelled in the same tags.
///
/// Pure: `doc` is untouched and the spliced document comes back as a
/// value with the edits that produce it.
///
/// Raises `InlineError`, typed.
#[pyfunction]
pub(crate) fn inline(
    py: Python<'_>,
    doc: &Doc,
    instance: &NodeId,
    resolver: &super::store::Workspace,
) -> PyResult<InlineOutcome> {
    let tol = Tol::witness();
    let store = resolver.resolver();
    let out = d::inline(&doc.inner, instance.0, store.as_ref(), tol)
        .map_err(|err| inline_err(py, &err))?;
    Ok(InlineOutcome {
        doc: out.doc,
        edits: out.edits,
        node_map: out
            .node_map
            .into_iter()
            .map(|(a, b)| (NodeId(a), NodeId(b)))
            .collect(),
    })
}

// ---- The pin-update door ----

/// Raise `UpdateError` carrying the refusal's stable tag and payload.
pub(crate) fn update_err(py: Python<'_>, err: &d::UpdateError) -> PyErr {
    use d::UpdateError as E;
    let (id, pin) = match err {
        E::NoSuchReference { id } => (id, None),
        E::AlreadyPinned { id, pin } => (id, Some(*pin)),
    };
    let pin = pin
        .and_then(|p| Py::new(py, ContentPin(p)).ok())
        .map(|v| v.into_any())
        .unwrap_or_else(|| py.None());
    typed_err(
        py,
        ErrorClass::Update,
        err.to_string(),
        &[
            (
                "variant",
                PyString::new(py, update_error_tag(err)).unbind().into_any(),
            ),
            ("id", PyString::new(py, &id.hex()).unbind().into_any()),
            ("pin", pin),
        ],
    )
}

/// One referenced document id carrying more than one pin, with the
/// sites holding each.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct PinMultiplicity(d::PinMultiplicity);

#[pymethods]
impl PinMultiplicity {
    /// The document referenced at two or more versions.
    #[getter]
    fn id(&self) -> String {
        self.0.id.hex()
    }

    /// Its pins, ascending, each with the nodes naming it. At least
    /// two by construction — a single-pin id is not a multiplicity and
    /// is not reported.
    #[getter]
    fn pins(&self) -> Vec<PinSites> {
        self.0.pins.iter().cloned().map(PinSites).collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "PinMultiplicity({:?}, {} pins)",
            self.0.id.hex(),
            self.0.pins.len()
        )
    }
}

/// One pin of one id, and the instantiate nodes that name it.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct PinSites(d::PinSites);

#[pymethods]
impl PinSites {
    /// The version.
    #[getter]
    fn pin(&self) -> ContentPin {
        ContentPin(self.0.pin)
    }

    /// The referencing nodes, in document order. Non-empty by
    /// construction — a pin is listed because a site holds it.
    #[getter]
    fn nodes(&self) -> Vec<NodeId> {
        self.0.nodes.iter().copied().map(NodeId).collect()
    }

    fn __repr__(&self) -> String {
        format!("PinSites({} node(s))", self.0.nodes.len())
    }
}

/// The **mixed-pin lint**: every referenced id whose pin multiplicity
/// exceeds one, listing each pin and the nodes naming it.
///
/// **Reports, never gates.** Nothing calls this from `apply`, from
/// `load`, or from evaluation — a document in mixed-pin state is valid
/// at every one of those doors and stays that way, because a staged
/// migration IS this state. A clean document returns an empty list,
/// which is the whole difference between "checked and fine" and "not
/// checked".
///
/// Reads the DOCUMENT and no store: it reports what the document says
/// about its own references, never what a store holds.
///
/// Deterministic: ids ascending, pins ascending within an id, nodes in
/// document order — a report that changes only when the document does.
#[pyfunction]
pub(crate) fn mixed_pins(doc: &Doc) -> Vec<PinMultiplicity> {
    d::mixed_pins(&doc.inner)
        .into_iter()
        .map(PinMultiplicity)
        .collect()
}

/// The edits that move EVERY reference to `id` onto `new_pin`, in
/// document order, one per site whose pin actually moves.
///
/// **Pure, and it reads no store.** `doc` is untouched, nothing is
/// applied, and `new_pin` is taken as given — whether it names content
/// anything holds is not asked here and cannot be, because this layer
/// has no store. Where that pin came from and how stale it is stays
/// the caller's fact; `Workspace.update_to_store` is the door that
/// computes one from disk, and it says exactly when it reads.
///
/// The caller applies the whole list or none of it, and that
/// all-or-nothing is what "atomic" means here: there is no partially
/// applied state to roll back from, because applying is the caller's
/// single step.
///
/// A site already pinning `new_pin` contributes NO edit — mixed-pin
/// state is authorable, so "update everywhere" stays usable from the
/// staged state where some sites already moved.
///
/// Raises `UpdateError`, typed: `no_such_reference` when no live node
/// instantiates the id (a typo or a stale id, never a silent success),
/// `already_pinned` when every site already names it (a completed
/// update). The two are separate because the recourses differ.
#[pyfunction]
pub(crate) fn update_references(
    py: Python<'_>,
    doc: &Doc,
    id: &str,
    new_pin: &ContentPin,
) -> PyResult<Vec<DocEdit>> {
    let id = document_id(id)?;
    d::update_references(&doc.inner, id, new_pin.0)
        .map(|edits| edits.into_iter().map(|inner| DocEdit { inner }).collect())
        .map_err(|err| update_err(py, &err))
}

/// Register the refactoring and pin-update surface on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<SplitOutcome>()?;
    m.add_class::<InlineOutcome>()?;
    m.add_class::<InterfaceRecord>()?;
    m.add_class::<InterfaceCrossing>()?;
    m.add_class::<PinMultiplicity>()?;
    m.add_class::<PinSites>()?;
    m.add_function(wrap_pyfunction!(split, m)?)?;
    m.add_function(wrap_pyfunction!(inline, m)?)?;
    m.add_function(wrap_pyfunction!(mixed_pins, m)?)?;
    m.add_function(wrap_pyfunction!(update_references, m)?)?;
    Ok(())
}
