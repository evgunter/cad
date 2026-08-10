//! **The document layer's export door** (LIB-DOORS F2).
//!
//! `step_export::step_string` takes a kernel [`topo::Body`]; before
//! this module, nothing curated accepted an EVALUATED body, so §L3's
//! one-shot journey ("build a bracket, export STEP") stopped at
//! "measure". This door completes it in the document layer's own
//! vocabulary: an [`Evaluation`] plus the [`RecipeNodeId`] whose value
//! is to be exported.
//!
//! # Shape (a measured fork, reported in the LIB-DOORS PR)
//!
//! A `pncad` FUNCTION taking `Evaluation` + node was chosen over a
//! method on the bindings' body handle: the "which body does this
//! node denote" unwrap then has ONE construction site, in Rust, and
//! Rust document-layer consumers get the same door Python binds —
//! §L3's "one semantics, two host languages".
//!
//! # Scope
//!
//! STEP only. The STL writers consume a [`mesh::Mesh`], not a body, so
//! an STL door needs the tessellation layer threaded through the
//! document surface — a further curation step, recorded rather than
//! rushed. Single-body values only (`Body`, or a non-empty `Boolean`):
//! a `Split` or `Instances` value denotes SEVERAL bodies and gets a
//! typed refusal naming its kind, not a silent first-element pick.

use editor_core::{BooleanValue, Evaluation, NodeResult, RecipeNodeId, ValuePayload};
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
}

impl core::fmt::Display for ExportError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownNode { node } => {
                write!(f, "export: node {node:?} has no entry in this evaluation")
            }
            Self::NodeFailed { node } => write!(
                f,
                "export: node {node:?} failed to evaluate (ask \
                 `Evaluation::node_error` for the typed cause)"
            ),
            Self::Poisoned { node, through } => write!(
                f,
                "export: node {node:?} never ran — poisoned through failed \
                 ancestor {through:?}"
            ),
            Self::NotABody { node, kind } => {
                write!(
                    f,
                    "export: node {node:?} evaluates to a `{kind}`, not a body"
                )
            }
            Self::EmptyBoolean { node } => {
                write!(
                    f,
                    "export: node {node:?}'s Boolean is empty — nothing to export"
                )
            }
            Self::Step(e) => write!(f, "export: the STEP writer refused: {e}"),
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
    step_string(body, options).map_err(ExportError::Step)
}
