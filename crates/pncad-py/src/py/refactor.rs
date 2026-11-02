//! **The recorded refactorings** (`split`, `inline`, `flatten`) and **the
//! pin-update door** (`update_references`, `mixed_pins`).
//!
//! Both families are PURE. `split` and `inline` hand back the new
//! document VALUES plus the ordinary recorded edits that produce them,
//! and mutate nothing; `update_references` hands back an edit list and
//! applies none of it, each edit one site's and independent of the
//! rest.
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
use pyo3::types::{PyDict, PyString};

use crate::errors::ErrorClass;
use crate::py::typed_err;
use crate::tags::{
    flatten_error_tag, inline_error_tag, interface_crossing_tag, split_error_tag, update_error_tag,
};
use pncad::document as d;
use pncad::tolerance::Tol;

use super::doc::{Doc, DocEdit, NodeId, name_text};
use super::step::StepId;
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
///
/// Both are FACE names — a crossing is written out of the two heads of
/// a mate, and each head names a face — so the kind is fixed by the
/// record's type and neither getter can answer anything else.
///
/// Those two are the whole of it: a crossing carries no
/// provenance, and the kernel's `InterfaceCrossing::Mate` says why.
#[pyclass(frozen, module = "pncad", skip_from_py_object)]
#[derive(Clone)]
pub(crate) struct InterfaceCrossing(d::InterfaceCrossing);

#[pymethods]
impl InterfaceCrossing {
    /// The stable tag: `mate`, the one word today — a crossing is
    /// whatever KIND of edge crossed the cut, and a mate is the only
    /// kind that can.
    // The map is `crate::tags::interface_crossing_tag`, whose words
    // `TAG_INVENTORY` pins.
    #[getter]
    fn variant(&self) -> &'static str {
        interface_crossing_tag(&self.0)
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
    // The machine channel keeps the full id and the stored name; the
    // spoken forms are the message's.
    let id = |n: &d::SpokenNode| -> Py<PyAny> {
        Py::new(py, NodeId(n.id()))
            .map(|v| v.into_any())
            .unwrap_or_else(|_| py.None())
    };
    let text = |s: &str| -> Py<PyAny> { PyString::new(py, s).unbind().into_any() };
    let named = |n: &d::SpokenName| -> Py<PyAny> {
        name_text(py, n.name())
            .map(|s| text(&s))
            .unwrap_or_else(|_| py.None())
    };
    let none = || py.None();
    let (node, consumer, input, param, name, doc_id) = match err {
        E::EmptyCut | E::CutsTheWorld | E::Stopped(_) => {
            (none(), none(), none(), none(), none(), none())
        }
        E::UnknownCutNode { id: n } | E::NoMaterial { node: n } => {
            (id(n), none(), none(), none(), none(), none())
        }
        E::PartIdCollides { id: which } => {
            (none(), none(), none(), none(), none(), text(&which.hex()))
        }
        E::SeveredEdge {
            consumer: c,
            input: i,
            ..
        } => (none(), id(c), id(i), none(), none(), none()),
        // The reader is the consumer, and why no one body carries the
        // read is in the message.
        E::RemainderReadUncarried { reader, why: _ } => {
            (none(), id(reader), none(), none(), none(), none())
        }
        E::UncutVarReference {
            var,
            cut_node,
            kept_node,
            ..
        } => (
            id(cut_node),
            none(),
            id(kept_node),
            text(&var.to_string()),
            none(),
            none(),
        ),
        // The space is the shared thing, as `UncutVarReference`'s
        // variable is: its node in the `param` field's place.
        E::SpaceStraddlesCut {
            space,
            cut_node,
            kept_node,
        } => (
            id(cut_node),
            none(),
            id(kept_node),
            text(&space.to_string()),
            none(),
            none(),
        ),
        E::DefinitionStraddlesCut { var, .. } => (
            none(),
            none(),
            none(),
            text(&var.to_string()),
            none(),
            none(),
        ),
        E::UnresolvedVarCrossesCut { var, node: n } => (
            id(n),
            none(),
            none(),
            text(&var.to_string()),
            none(),
            none(),
        ),
        E::PartNameReachesRemainder { node: n, name, .. } => {
            (id(n), none(), none(), none(), named(name), none())
        }
        E::NameStraddlesCut { name, .. }
        | E::BodyNameCrossesCut { name }
        | E::NameOutsidePartWorld { name }
        | E::NameOnDroppedStep { name, .. } => {
            (none(), none(), none(), none(), named(name), none())
        }
        E::Pin { .. } | E::PartEdit { .. } | E::RemainderEdit { .. } => {
            (none(), none(), none(), none(), none(), none())
        }
    };
    // A definition tied across the cut: the variable it reads that
    // moves, and the one that stays (or that the document no longer
    // holds), as `UncutVarReference` carries both of its nodes.
    let (moving, staying) = match err {
        E::DefinitionStraddlesCut {
            moving, staying, ..
        } => (text(&moving.to_string()), text(&staying.to_string())),
        _ => (none(), none()),
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
            ("param", param),
            ("name", name),
            ("id", doc_id),
            ("moving", moving),
            ("staying", staying),
        ],
    )
}

/// A step map as a `dict[StepId, StepId]`.
fn step_dict<'py>(
    py: Python<'py>,
    map: &std::collections::BTreeMap<d::StepId, d::StepId>,
) -> PyResult<Bound<'py, PyDict>> {
    let dict = PyDict::new(py);
    for (from, to) in map {
        dict.set_item(StepId(*from), StepId(*to))?;
    }
    Ok(dict)
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
    remainder_maintenance: Vec<d::Maintenance>,
    part_edits: Vec<d::DocEdit<d::ProfileProgram>>,
    part_maintenance: Vec<d::Maintenance>,
    instance: NodeId,
    node_map: Vec<(NodeId, NodeId)>,
    step_map: std::collections::BTreeMap<d::StepId, d::StepId>,
}

#[pymethods]
impl SplitOutcome {
    /// The original document with the cut nodes replaced by ONE
    /// instance of the new part.
    ///
    /// The document and the maintenance its edits performed travel
    /// TOGETHER, so `last_maintenance` on the `Doc` handed back reads
    /// the record `remainder_edits` produced, net of what a later edit
    /// in the same split took back, rather than an empty list that
    /// would read as "nothing moved".
    #[getter]
    fn remainder(&self) -> Doc {
        Doc {
            inner: self.remainder.clone(),
            maintenance: self.remainder_maintenance.clone(),
        }
    }

    /// The new part document, carrying the cut nodes.
    ///
    /// Its `last_maintenance` is what building the part from empty
    /// reported, net of what a later edit in the same split took back.
    #[getter]
    fn part(&self) -> Doc {
        Doc {
            inner: self.part.clone(),
            maintenance: self.part_maintenance.clone(),
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

    /// Each cut profile step's id → the id the part document minted
    /// for it: a name the caller
    /// holds on a cut profile's piece is spelled in the part by this
    /// map, as its node is by `node_map`.
    #[getter]
    fn step_map<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        step_dict(py, &self.step_map)
    }

    fn __repr__(&self) -> String {
        format!("SplitOutcome(instance={})", self.instance.0.full())
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
/// The cut must be **ancestor- and consumer-closed**, and no
/// constraint's frame may cross it. Everything else refuses typed,
/// naming the offending edge, constraint, parameter or name. The
/// remainder holds ONE instance of the part, pinned to the world at
/// the identity, so the cut material stands where it stood.
///
/// Pure: `doc` is untouched, nothing is written, and the two documents
/// come back as values with the edits that produce them.
///
/// Raises `SplitError`, typed.
#[pyfunction]
#[pyo3(signature = (doc, cut, part_id, *, resolver=None))]
pub(crate) fn split(
    py: Python<'_>,
    doc: &Doc,
    cut: Vec<NodeId>,
    part_id: &str,
    resolver: Option<&super::store::Workspace>,
) -> PyResult<SplitOutcome> {
    let tol = Tol::witness();
    let part_id = document_id(part_id)?;
    let set: BTreeSet<d::RecipeNodeId> = cut.iter().map(|n| n.0).collect();
    // The cut's edits can need a solved frame of another part, so the
    // split levers through the same seam `evaluate(doc, resolver=)` crosses — plus the part it
    // is minting, which it holds itself; absent, an edit that needs a
    // solved frame of another part refuses typed.
    let store = resolver.map(super::store::Workspace::resolver);
    let out = d::split(
        &doc.inner,
        &set,
        part_id,
        tol,
        store.as_ref(),
        &d::Stop::default(),
    )
    .map_err(|err| split_err(py, &err))?;
    let node_map = pairs_in_order(&out.node_map, &out.part);
    Ok(SplitOutcome {
        remainder: out.remainder,
        part: out.part,
        remainder_edits: out.remainder_edits,
        remainder_maintenance: out.remainder_maintenance,
        part_edits: out.part_edits,
        part_maintenance: out.part_maintenance,
        instance: NodeId(out.instance),
        node_map,
        step_map: out.step_map,
    })
}

// ---- inline ----

/// Raise `InlineError` carrying the refusal's stable tag and payload.
fn inline_err(py: Python<'_>, err: &d::InlineError) -> PyErr {
    use d::InlineError as E;
    // The machine channel keeps the full id and the stored name; the
    // spoken forms are the message's.
    let id = |n: &d::SpokenNode| -> Py<PyAny> {
        Py::new(py, NodeId(n.id()))
            .map(|v| v.into_any())
            .unwrap_or_else(|_| py.None())
    };
    let text = |s: &str| -> Py<PyAny> { PyString::new(py, s).unbind().into_any() };
    let named = |n: &d::SpokenName| -> Py<PyAny> {
        name_text(py, n.name())
            .map(|s| text(&s))
            .unwrap_or_else(|_| py.None())
    };
    let number = |v: f64| -> Py<PyAny> {
        v.into_pyobject(py)
            .map(|b| b.unbind().into_any())
            .unwrap_or_else(|_| py.None())
    };
    let none = || py.None();
    let (node, name, param, key, host_eps, part_eps) = match err {
        // The instance (or the node a construction reads it at) is the
        // subject: why it cannot be spliced is the variant.
        E::UnknownNode { id: n }
        | E::NotAnInstance { node: n }
        | E::UnplaceablePart { instance: n } => (id(n), none(), none(), none(), none(), none()),
        E::Unresolved { failure } => (
            none(),
            none(),
            none(),
            text(&failure.message),
            none(),
            none(),
        ),
        E::EpsilonSeam {
            host_eps: h,
            part_eps: p,
        } => (none(), none(), none(), none(), number(*h), number(*p)),
        E::PartCarriesMetadata { key: k } => (none(), none(), none(), text(k), none(), none()),
        E::VarNameConflict { name: p } => {
            (none(), none(), text(p.as_str()), none(), none(), none())
        }
        E::UnreadPose { var } => (
            none(),
            none(),
            text(&var.to_string()),
            none(),
            none(),
            none(),
        ),
        E::UnresolvedVarCrossesCut { var, node: n } => (
            id(n),
            none(),
            text(&var.to_string()),
            none(),
            none(),
            none(),
        ),
        E::InstanceBodyNameReferenced { name }
        | E::StrandedPartName { name, .. }
        | E::StrandedPartRead { name, .. }
        | E::NameOnDroppedStep { name, .. } => {
            (none(), named(name), none(), none(), none(), none())
        }
        E::Edit { .. } => (none(), none(), none(), none(), none(), none()),
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
            ("name", name),
            ("param", param),
            ("key", key),
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
    maintenance: Vec<d::Maintenance>,
    node_map: Vec<(NodeId, NodeId)>,
    step_map: std::collections::BTreeMap<d::StepId, d::StepId>,
    placement: d::RecipeNodeId,
    space: d::VarId,
}

#[pymethods]
impl InlineOutcome {
    /// The placement standing where the instance stood: the part's
    /// world copied into `space` and placed under the instance's
    /// bundle, its output the instance's.
    #[getter]
    fn placement(&self) -> NodeId {
        NodeId(self.placement)
    }

    /// The new space the part's recipe was copied into
    /// (`Doc.new_space()`), which `placement` places out of.
    #[getter]
    fn space(&self) -> super::doc::Var {
        super::doc::Var(self.space, Some(d::VarKind::Space))
    }

    /// The host document with the instance replaced by the referenced
    /// document's own nodes.
    ///
    /// The document and the maintenance its edits performed travel
    /// TOGETHER, so `last_maintenance` on the `Doc` handed back reads
    /// what the splice's edits reported, net of what a later edit in
    /// the same inline took back.
    #[getter]
    fn doc(&self) -> Doc {
        Doc {
            inner: self.doc.clone(),
            maintenance: self.maintenance.clone(),
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

    /// Part node → its id in the spliced document, as pairs in the
    /// spliced document's order.
    #[getter]
    fn node_map(&self) -> Vec<(NodeId, NodeId)> {
        self.node_map.clone()
    }

    /// Each part profile step's id → the id the host minted for it on
    /// the splice, the step half of `node_map`.
    #[getter]
    fn step_map<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        step_dict(py, &self.step_map)
    }

    fn __repr__(&self) -> String {
        format!("InlineOutcome({} edit(s))", self.edits.len())
    }
}

/// A node map as the pairs Python reads ([`crate::node_map`]).
fn pairs_in_order(map: &d::NodeMap, doc: &d::ProfileDoc) -> Vec<(NodeId, NodeId)> {
    crate::node_map::in_target_id_order(map, doc)
        .into_iter()
        .map(|(a, b)| (NodeId(a), NodeId(b)))
        .collect()
}

/// Splice a referenced document back in, replacing the instantiate
/// node with the part's own nodes — `split`'s inverse.
///
/// One rule: the part's recipe is copied verbatim into a new space
/// (`Doc.new_space()`, the outcome's `space`), and a placement of what
/// the part's world placed is put where the instance stood, under the
/// instance's own bundle (the outcome's `placement`), so nothing moves
/// and a variable the bundle reads keeps driving it. `flatten` folds
/// that placement away where its bundle is the identity.
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
    let out = d::inline(&doc.inner, instance.0, &store, tol).map_err(|err| inline_err(py, &err))?;
    let node_map = pairs_in_order(&out.node_map, &out.doc);
    Ok(InlineOutcome {
        doc: out.doc,
        edits: out.edits,
        maintenance: out.maintenance,
        node_map,
        step_map: out.step_map,
        placement: out.placement,
        space: out.space,
    })
}

// ---- flatten ----

/// Raise `FlattenError` carrying the refusal's stable tag and payload.
fn flatten_err(py: Python<'_>, err: &d::FlattenError) -> PyErr {
    use d::FlattenError as E;
    let id = |n: &d::SpokenNode| -> Py<PyAny> {
        Py::new(py, NodeId(n.id()))
            .map(|v| v.into_any())
            .unwrap_or_else(|_| py.None())
    };
    let text = |s: &str| -> Py<PyAny> { PyString::new(py, s).unbind().into_any() };
    let none = || py.None();
    let (node, by, name, param) = match err {
        E::UnknownNode { id: n }
        | E::NotAPlacement { node: n }
        | E::NotTheIdentity { placement: n }
        | E::NotANewSpace { placement: n } => (id(n), none(), none(), none()),
        // The other placement reading out of the space rides `by`.
        E::SpaceShared { placement, other } => (id(placement), id(other), none(), none()),
        E::ReadUncarried { reader } => (none(), id(reader), none(), none()),
        // The copy's name, and the name the shape it copies holds.
        E::BothNamed { name, held } => (none(), none(), text(name.as_str()), text(held.as_str())),
        E::Edit { .. } => (none(), none(), none(), none()),
    };
    typed_err(
        py,
        ErrorClass::Flatten,
        err.to_string(),
        &[
            (
                "variant",
                PyString::new(py, flatten_error_tag(err))
                    .unbind()
                    .into_any(),
            ),
            ("node", node),
            ("by", by),
            ("name", name),
            ("param", param),
        ],
    )
}

/// What a flatten produced: the document value and the recorded edits
/// that make it.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct FlattenOutcome {
    doc: d::ProfileDoc,
    edits: Vec<d::DocEdit<d::ProfileProgram>>,
    maintenance: Vec<d::Maintenance>,
}

#[pymethods]
impl FlattenOutcome {
    /// The document with the placement and its space gone, what the
    /// space held now in the placement's target. The maintenance its
    /// edits performed travels with it (`last_maintenance`).
    #[getter]
    fn doc(&self) -> Doc {
        Doc {
            inner: self.doc.clone(),
            maintenance: self.maintenance.clone(),
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

    fn __repr__(&self) -> String {
        format!("FlattenOutcome({} edit(s))", self.edits.len())
    }
}

/// **Flatten an identity placement out of a `new_space()`** into its
/// target — what `inline` leaves, folded away: each placement taking a
/// source from the space is re-targeted into the placement's target,
/// each read of the copy reads the shape it copies, the copy's name
/// moves to that shape, and the placement and the space are deleted.
///
/// Pure: `doc` is untouched. Raises `FlattenError`, typed: a node that
/// is no placement (`not_a_placement`), a bundle that is not the
/// identity as written (`not_the_identity`), a source not in one new
/// space (`not_a_new_space`), another placement reading out of the
/// space (`space_shared`, `by` the other), a read of the copy naming no
/// one shape it copies (`read_uncarried`), the copy and its shape both
/// named (`both_named`, `name` the copy's and `param` the shape's).
#[pyfunction]
pub(crate) fn flatten(py: Python<'_>, doc: &Doc, placement: &NodeId) -> PyResult<FlattenOutcome> {
    let out =
        d::flatten(&doc.inner, placement.0, Tol::witness()).map_err(|err| flatten_err(py, &err))?;
    Ok(FlattenOutcome {
        doc: out.doc,
        edits: out.edits,
        maintenance: out.maintenance,
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
/// Each edit moves one site and reads no other edit's result, so the
/// edits apply in any order, and any subset leaves an authorable
/// mixed-pin state (`mixed_pins` reads it). "Update everywhere" is the
/// whole list.
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
    m.add_class::<FlattenOutcome>()?;
    m.add_class::<InterfaceRecord>()?;
    m.add_class::<InterfaceCrossing>()?;
    m.add_class::<PinMultiplicity>()?;
    m.add_class::<PinSites>()?;
    m.add_function(wrap_pyfunction!(split, m)?)?;
    m.add_function(wrap_pyfunction!(inline, m)?)?;
    m.add_function(wrap_pyfunction!(flatten, m)?)?;
    m.add_function(wrap_pyfunction!(mixed_pins, m)?)?;
    m.add_function(wrap_pyfunction!(update_references, m)?)?;
    Ok(())
}
