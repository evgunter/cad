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
//! **A failure crosses as TWO words plus prose**, and the two are
//! kept apart on purpose. `status` is the state — one of three,
//! exhaustible, and the thing whose repairs are opposite ends of the
//! document. `variant` is the arm underneath it: `vanished` /
//! `ambiguous` / `node_gone` under a failure, `target_failed` /
//! `target_poisoned` / `target_not_evaluated` under an
//! indeterminate. A caller that only branches on the state never has
//! to learn the second vocabulary; one offering a repair does, because
//! a tie is refined among candidates and a stranded name is rebound.
//!
//! `detail` stays the kernel's own prose beside them, and nothing here
//! parses it — the discriminant comes from matching the payload types,
//! which the façade carries.

use pyo3::prelude::*;

use crate::py::doc::{NodeId, name_text};
use crate::py::select::EntityKind;
use crate::py::select::entity_kind;
use crate::tags::{resolution_status_tag, resolve_error_tag, resolve_indeterminate_tag};
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
    /// WHICH failure, under the two states that have arms: the
    /// kernel's own `vanished` / `ambiguous` / `node_gone` and
    /// `target_failed` / `target_poisoned` / `target_not_evaluated`.
    /// `None` when resolved, which is the state with nothing to say
    /// here.
    ///
    /// Kept apart from `status` rather than folded into it: the three
    /// states are what a caller must handle, and the six arms are what
    /// a caller offering a REPAIR reads — a tie is refined among
    /// `offers`, a stranded name is rebound, an indeterminate one is
    /// left alone until its node evaluates.
    #[pyo3(get)]
    variant: Option<&'static str>,
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
/// unprojected state. So are the two the tag maps run over, one rung
/// down, which is what makes `variant` a closed vocabulary rather
/// than a best effort.
pub(crate) fn resolution(py: Python<'_>, verdict: &s::Resolution) -> PyResult<Resolution> {
    let status = resolution_status_tag(verdict);
    match verdict {
        s::Resolution::Resolved(found) => Ok(Resolution {
            status,
            variant: None,
            node: Some(NodeId(found.node)),
            body: Some(found.entity.body),
            kind: Some(entity_kind(found.entity.key.kind())),
            detail: None,
            offers: None,
        }),
        s::Resolution::Failed(failure) => Ok(Resolution {
            status,
            variant: Some(resolve_error_tag(&failure.error)),
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
            variant: Some(resolve_indeterminate_tag(cause)),
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
