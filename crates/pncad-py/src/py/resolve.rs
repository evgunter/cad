//! **Name resolution across re-evaluation** (LIB-B-RESOLVE).
//!
//! Python's whole selection story is store-then-reuse. `select` and
//! the four materializers MATERIALIZE — they answer as of ONE
//! evaluation and hand back opaque name texts a caller keeps —
//! `Node.fillet` freezes a name set into the document, and `PickHit`
//! hands a pick straight into the same slot. Every one of those
//! stored names is a claim about a run that has already finished, and
//! the next run may not honour it: the node it was minted by can
//! fail, the entity it denoted can be merged away by an upstream
//! edit, the node itself can be deleted.
//!
//! [`Resolution`] is the answer to that question, and
//! `Evaluation.resolve` is the door. Total: every name asked gets
//! exactly one verdict, never a panic and never a raise for a name
//! that simply does not denote — "this name is gone" is an ANSWER,
//! not an error, and flattening it into an exception would make the
//! ordinary case of re-evaluating a stored selection a control-flow
//! exercise.
//!
//! # Three states, and the recourse is why
//!
//! `resolved` — the name denotes exactly one entity; `node` and
//! `body` say where, and `kind` says what.
//!
//! `failed` — the name does not denote in this evaluation, and it
//! will not come back on its own. The repair is an explicit rebind;
//! nothing here auto-repairs, and `offers` is what the kernel is
//! willing to SUGGEST (a merged name for a retired constituent, a
//! collapsed group's survivor), never a substitution it has made.
//!
//! `indeterminate` — the NAME is fine and the RUN is not. The
//! minting node failed, was poisoned by an upstream failure, or was
//! never evaluated, so the reference cannot be answered right now and
//! resolves again when that node does. This is the state a consumer
//! most needs kept apart from `failed`: they look identical from a
//! selection panel and their repairs are opposite ends of the
//! document. Telling a user to rebind a name that never broke is
//! exactly the confusion the kernel split this arm out to prevent.
//!
//! # Evaluation-wide, where `denotation` is node-scoped
//!
//! `Evaluation.denotation(node, name)` asks of ONE node's table:
//! does this node's output carry the name, uniquely or as a tie.
//! `Evaluation.resolve(name)` asks the whole evaluation and answers
//! WHICH node carries it — the first in evaluation order, since
//! pass-through tables carry the same rows downstream. So the two
//! doors are not rungs of one ladder: a name can resolve here while
//! `denotation` refuses `no_such_name` for the node you happened to
//! ask, because the name lives one node upstream.
//!
//! # What crosses, and what does not
//!
//! A resolved verdict projects `(node, body)` — deliberately the
//! same pair `NodePick.build` takes, so a stored name that still
//! resolves feeds a pick index directly — plus the entity `kind`.
//! What it does NOT project is the arena key beside them: keys are
//! body-lineage-scoped and do not leave `editor-core` (G1). The kind
//! is worth carrying even though the name records it, because a
//! Python caller holds the name as OPAQUE TEXT and is told never to
//! parse it; through this surface the verdict is the only door that
//! answers "what kind of thing is this stored name".
//!
//! **A failure crosses as one word plus prose, and that is a
//! measured limit, not an oversight.** `ResolveError`,
//! `ResolutionFailure` and `ResolveIndeterminate` are DECIDED absent
//! from the façade — `crates/pncad/tests/all.rs`'s `NOT_CARRIED`,
//! "Naming interior", which records that the resolution VERDICT left
//! that family at GUI-2 as exactly three names and that
//! "`Resolution`'s arms answer it ... through pattern matching and
//! `Display`, without naming a payload type". So this module matches
//! the three arms it can name and reads `detail` off the kernel's own
//! `Display`; there is no `vanished` / `ambiguous` / `node_gone`
//! discriminant to forward, and inventing one — by parsing prose, or
//! by re-deriving the ladder here — would be a second implementation
//! of a kernel decision. The gap is banked as
//! `work/lib/resolution-failure-arms-are-unmatchable-under-resolution.md`;
//! it is a façade question, not one a binding unit closes.

use pyo3::prelude::*;

use crate::py::doc::{NodeId, name_text};
use crate::py::select::EntityKind;
use crate::py::select::entity_kind;
use crate::tags::resolution_status_tag;
use pncad::select as s;

/// **A stored name's standing in one evaluation** — the question
/// every consumer that stores names must ask on the next run.
///
/// Total and report-only: reading a verdict changes nothing, and no
/// arm repairs anything. `status` is the fact to branch on, and the
/// other attributes are always present, `None` where the state does
/// not carry them — so `getattr` never raises and a caller never has
/// to test `status` before reading.
#[pyclass(frozen, module = "pncad")]
pub(crate) struct Resolution {
    /// `"resolved"`, `"failed"` or `"indeterminate"`.
    #[pyo3(get)]
    status: &'static str,
    /// The node whose table carries the name — the FIRST in
    /// evaluation order, which need not be the node that minted it.
    /// `None` unless resolved.
    #[pyo3(get)]
    node: Option<NodeId>,
    /// The output-body index within that node's value; pairs with
    /// `node` as `NodePick.build`'s arguments. `None` unless
    /// resolved.
    #[pyo3(get)]
    body: Option<u32>,
    /// What the name denotes there. `None` unless resolved.
    #[pyo3(get)]
    kind: Option<EntityKind>,
    /// The kernel's own account of why — prose, not an interface, and
    /// the only thing a non-resolved verdict says about its arm.
    /// `None` when resolved.
    #[pyo3(get)]
    detail: Option<String>,
    /// Structural rebind SUGGESTIONS, in the same opaque alphabet the
    /// materializers speak: a retired constituent's merged name, a
    /// collapsed over-tie group's survivor. Never a repair the kernel
    /// has made — the policy menu is empty and a rebind is always the
    /// caller's explicit edit.
    ///
    /// A list on `failed`, EMPTY where nothing structural offers
    /// itself, and `None` on the other two states: "no suggestions"
    /// and "suggestions do not apply" are different facts and this
    /// attribute keeps them apart.
    #[pyo3(get)]
    offers: Option<Vec<String>>,
}

#[pymethods]
impl Resolution {
    fn __repr__(&self) -> String {
        match (self.node, self.body) {
            (Some(node), Some(body)) => {
                format!("Resolution(resolved at node={}, body={body})", node.0.0)
            }
            _ => format!("Resolution({})", self.status),
        }
    }
}

/// Project one kernel verdict into the Python value.
///
/// The match is EXHAUSTIVE with no wildcard: an arm added kernel-side
/// arrives here as a compile error rather than as a silently
/// unprojected state. The three arms' payload TYPES are not nameable
/// through the façade (module docs), which is why the bindings below
/// read fields and `Display` off values whose types this file never
/// spells.
pub(crate) fn resolution(py: Python<'_>, verdict: &s::Resolution) -> PyResult<Resolution> {
    let status = resolution_status_tag(verdict);
    match verdict {
        s::Resolution::Resolved(found) => Ok(Resolution {
            status,
            node: Some(NodeId(found.node)),
            body: Some(found.entity.body),
            kind: Some(entity_kind(found.entity.key.kind())),
            detail: None,
            offers: None,
        }),
        s::Resolution::Failed(failure) => Ok(Resolution {
            status,
            node: None,
            body: None,
            kind: None,
            detail: Some(failure.error.to_string()),
            offers: Some(
                failure
                    .offers
                    .iter()
                    .map(|name| name_text(py, name))
                    .collect::<PyResult<Vec<_>>>()?,
            ),
        }),
        s::Resolution::Indeterminate(cause) => Ok(Resolution {
            status,
            node: None,
            body: None,
            kind: None,
            detail: Some(cause.to_string()),
            offers: None,
        }),
    }
}

/// Register the resolution vocabulary on the module.
pub(crate) fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Resolution>()?;
    Ok(())
}
