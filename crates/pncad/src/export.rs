//! **The document layer's export door**.
//!
//! `step_export::step_string` takes a kernel [`topo::Body`]; before
//! this module, nothing curated accepted an EVALUATED body, so the
//! one-shot journey ("build a bracket, export STEP") stopped at
//! "measure". This door completes it in the document layer's own
//! vocabulary: an [`Evaluation`] plus the [`RecipeNodeId`] whose value
//! is to be exported.
//!
//! # Shape (a measured fork)
//!
//! A `pncad` FUNCTION taking `Evaluation` + node was chosen over a
//! method on the bindings' body handle: the "which body does this
//! node denote" unwrap then has ONE construction site, in Rust, and
//! Rust document-layer consumers get the same door Python binds —
//! one semantics, two host languages.
//!
//! # Scope
//!
//! STEP only. The STL writers consume a [`mesh::Mesh`], not a body, so
//! an STL door needs the tessellation layer threaded through the
//! document surface — a further curation step, recorded rather than
//! rushed. Single-body values only (`Body`, or a non-empty `Boolean`):
//! a `Split` or `Instances` value denotes SEVERAL bodies and gets a
//! typed refusal naming its kind, not a silent first-element pick.

use editor_core::{
    BooleanValue, Evaluation, NodeResult, ProductError, ProfileDoc, RecipeNodeId, ValuePayload,
};
use geom_core::Tol;
use step_export::{StepExportError, StepOptions, step_string};

/// Why [`step_for_node`] refused. Fail-loud and typed, D2-style; the
/// evaluation-side arms carry the ids to ask the caller's own
/// [`Evaluation`] for the payload (`NodeFailed`/`Poisoned` root causes
/// are one [`Evaluation::node_error`] call away — the error type does
/// not clone the kernel's non-`Clone` refusals to repeat them here).
#[derive(Debug)]
pub enum ExportError {
    /// The id has no entry in this evaluation (never scheduled, or
    /// past a cancelation's completed prefix).
    UnknownNode {
        /// The id that was asked for.
        node: RecipeNodeId,
    },
    /// The node itself failed to evaluate.
    NodeFailed {
        /// The failed node.
        node: RecipeNodeId,
    },
    /// The node never ran: an ancestor failed.
    Poisoned {
        /// The poisoned node.
        node: RecipeNodeId,
        /// Its nearest failed ancestor.
        through: RecipeNodeId,
    },
    /// The node's value is not a single body (`profile`, `datum`,
    /// `split`, `instances`, ...).
    NotABody {
        /// The node whose value was refused.
        node: RecipeNodeId,
        /// The value's kind, from the kernel's own
        /// [`ValuePayload::kind_name`].
        kind: &'static str,
    },
    /// The node's Boolean produced an empty result — nothing to
    /// export.
    EmptyBoolean {
        /// The node whose Boolean came up empty.
        node: RecipeNodeId,
    },
    /// The body was denoted but the STEP writer refused it.
    Step(StepExportError),
    /// The whole-document door's gather refused: no
    /// body-denoting root, a failed root, or a kernel refusal while
    /// gathering.
    Product(ProductError),
}

// A node reaches prose as its bare id. The message is for a human,
// and the wrapper's `Debug` spelling puts a Rust type name in front of
// the one part of it they can act on.
impl core::fmt::Display for ExportError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownNode { node } => {
                write!(f, "export: node {} has no entry in this evaluation", node.0)
            }
            Self::NodeFailed { node } => write!(
                f,
                "export: node {} failed to evaluate (ask \
                 `Evaluation::node_error` for the typed cause)",
                node.0
            ),
            Self::Poisoned { node, through } => write!(
                f,
                "export: node {} never ran — poisoned through failed \
                 ancestor {}",
                node.0, through.0
            ),
            Self::NotABody { node, kind } => {
                write!(
                    f,
                    "export: node {} evaluates to a `{kind}`, not a body",
                    node.0
                )
            }
            Self::EmptyBoolean { node } => {
                write!(
                    f,
                    "export: node {}'s Boolean is empty — nothing to export",
                    node.0
                )
            }
            Self::Step(e) => write!(f, "export: the STEP writer refused: {e}"),
            Self::Product(e) => write!(f, "export: {e}"),
        }
    }
}

impl core::error::Error for ExportError {}

/// Serializes the single body `node` denotes in `evaluation` as a STEP
/// (AP214 Part 21) exchange file and returns it as a string.
///
/// Accepts a `Body` value or a non-empty `Boolean`; every other shape
/// of result is a typed [`ExportError`], never a guess.
///
/// # Errors
///
/// Every arm of [`ExportError`]: the evaluation-side refusals above,
/// or [`ExportError::Step`] carrying the writer's own refusal.
pub fn step_for_node(
    evaluation: &Evaluation<f64>,
    node: RecipeNodeId,
    options: &StepOptions,
    tol: Tol,
) -> Result<String, ExportError> {
    let result = evaluation
        .result(node)
        .ok_or(ExportError::UnknownNode { node })?;
    let value = match result {
        NodeResult::Ok(value) => value,
        NodeResult::Failed(_) => return Err(ExportError::NodeFailed { node }),
        NodeResult::Poisoned { through } => {
            return Err(ExportError::Poisoned {
                node,
                through: *through,
            });
        }
    };
    let body = match &value.payload {
        ValuePayload::Body(body) => body,
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body,
        ValuePayload::Boolean(BooleanValue::Empty) => {
            return Err(ExportError::EmptyBoolean { node });
        }
        other => {
            return Err(ExportError::NotABody {
                node,
                kind: other.kind_name(),
            });
        }
    };
    step_string(body, options, tol).map_err(ExportError::Step)
}

/// Serializes the WHOLE DOCUMENT's product — the gather of every
/// body-denoting product root, in root-list order — as a STEP (AP214
/// Part 21) exchange file.
///
/// This is the door that accepts what [`step_for_node`] refuses: a
/// pattern's instances, a split's two halves, several disjoint tips.
/// One writer path serves both cases — `step_export` already emits one
/// `MANIFOLD_SOLID_BREP` per shell of every solid, which is exactly
/// the shape an imported multi-solid assembly round-trips through — so
/// a one-solid product is not a special case here, it is the same call
/// with one solid.
///
/// # Errors
///
/// [`ExportError::Product`] carrying the gather's own typed refusal
/// (no body-denoting root, a failed root, a kernel graft or validity
/// refusal), or [`ExportError::Step`] carrying the writer's.
pub fn export_document_step(
    evaluation: &Evaluation<f64>,
    doc: &ProfileDoc,
    options: &StepOptions,
    tol: Tol,
) -> Result<String, ExportError> {
    let body = editor_core::product(doc, evaluation, tol).map_err(ExportError::Product)?;
    step_string(&body, options, tol).map_err(ExportError::Step)
}
