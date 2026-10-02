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
    BooleanValue, CarriedUnplaced, Evaluation, NodeStanding, ProductError, ProfileDoc,
    RecipeNodeId, SpokenNode, Unplaced, ValuePayload,
};
use geom_core::Tol;
use step_export::{StepExportError, StepOptions, step_string};

/// Why [`step_for_node`] refused. Fail-loud and typed, D2-style; the
/// standing carries the ids to ask the caller's own [`Evaluation`] for
/// the payload (a failure's root cause is one
/// [`Evaluation::node_error`] call away — the error type does not
/// clone the kernel's non-`Clone` refusals to repeat them here).
#[derive(Debug)]
pub enum ExportError {
    /// The node has no value in this evaluation.
    Standing(NodeStanding),
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
    /// **Unplaced parts** (A11 (2)): STEP writes one world, and these
    /// live in an unplaced group's own space — each with its group,
    /// by its root, and why nothing places it.
    Unplaced {
        /// Every unplaced part the export would have to write, in node
        /// order: the node, its group's root, and the cause.
        parts: Vec<(RecipeNodeId, RecipeNodeId, Unplaced)>,
    },
    /// **Unplaced groups in a part below** (A9, A11 (2)): a part
    /// crosses the document seam as its world product, which leaves its
    /// unplaced groups out, so writing it would write the part without
    /// them. Each group with the route it arrived by and its cause.
    UnplacedBelow {
        /// Every such group, once each, in node order.
        groups: Vec<CarriedUnplaced>,
    },
}

/// [`ExportError`]'s sentence with each of this document's nodes said
/// by `doc` when the frame holds it, and by its tag when not.
struct Said<'a>(&'a ExportError, Option<&'a ProfileDoc>);

impl Said<'_> {
    fn node(&self, id: RecipeNodeId) -> SpokenNode {
        match self.1 {
            Some(doc) => doc.spoken(id),
            None => SpokenNode::absent(id),
        }
    }
}

impl core::fmt::Display for Said<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            ExportError::Standing(standing) => match self.1 {
                Some(doc) => write!(f, "export: {}", standing.spoken(doc)),
                None => write!(f, "export: {standing}"),
            },
            ExportError::NotABody { node, kind } => {
                write!(
                    f,
                    "export: {} evaluates to a `{kind}`, not a body",
                    self.node(*node)
                )
            }
            ExportError::EmptyBoolean { node } => {
                write!(
                    f,
                    "export: {}'s Boolean is empty — nothing to export",
                    self.node(*node)
                )
            }
            ExportError::Step(e) => write!(f, "export: the STEP writer refused: {e}"),
            ExportError::Product(e) => write!(f, "export: {e}"),
            ExportError::Unplaced { parts } => {
                write!(
                    f,
                    "export: STEP writes one world, and these parts are unplaced:"
                )?;
                for (node, group, cause) in parts {
                    write!(
                        f,
                        " {} (its group, rooted at {}, is unplaced because {cause});",
                        self.node(*node),
                        self.node(*group)
                    )?;
                }
                write!(
                    f,
                    " {}",
                    editor_core::Recourse(editor_core::UNPLACED_RECOURSE)
                )
            }
            // A group below is spelled in its part's ids, so its row
            // keeps its own words.
            ExportError::UnplacedBelow { groups } => {
                write!(
                    f,
                    "export: STEP writes one world, and a part below holds unplaced groups its \
                     world leaves out:"
                )?;
                for group in groups {
                    write!(f, " {group};")?;
                }
                write!(
                    f,
                    " {}",
                    editor_core::Recourse(format_args!(
                        "open that document and {}",
                        editor_core::UNPLACED_RECOURSE
                    ))
                )
            }
        }
    }
}

/// The sentence where no document is at hand: each node by its tag.
impl core::fmt::Display for ExportError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Said(self, None).fmt(f)
    }
}

impl ExportError {
    /// **The refusal as the frame holding the evaluated document says
    /// it**: each of its nodes as `doc` holds it now. The door reads an
    /// evaluation alone, so the refusal holds ids, never a label.
    #[must_use]
    pub fn spoken(&self, doc: &ProfileDoc) -> String {
        Said(self, Some(doc)).to_string()
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
/// [`ExportError::Unplaced`] for a node that lives in an unplaced
/// group's own space, [`ExportError::UnplacedBelow`] for one holding a
/// part that leaves an unplaced group out, or [`ExportError::Step`]
/// carrying the writer's own refusal.
pub fn step_for_node(
    evaluation: &Evaluation<f64>,
    node: RecipeNodeId,
    options: &StepOptions,
    tol: Tol,
) -> Result<String, ExportError> {
    if let Some(&(group, cause)) = evaluation.unplaced.get(&node) {
        return Err(ExportError::Unplaced {
            parts: vec![(node, group, cause)],
        });
    }
    if let Some(groups) = evaluation.unplaced_below.get(&node) {
        return Err(ExportError::UnplacedBelow {
            groups: groups.clone(),
        });
    }
    let value = evaluation.usable(node).map_err(ExportError::Standing)?;
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
/// [`ExportError::Unplaced`] naming every unplaced part, which the
/// product leaves out and STEP's one world cannot hold;
/// [`ExportError::UnplacedBelow`] naming every unplaced group a part
/// below holds, with its route;
/// [`ExportError::Product`] carrying the gather's own typed refusal
/// (no body-denoting root, a failed root, a kernel graft or validity
/// refusal), or [`ExportError::Step`] carrying the writer's.
pub fn export_document_step(
    evaluation: &Evaluation<f64>,
    doc: &ProfileDoc,
    options: &StepOptions,
    tol: Tol,
) -> Result<String, ExportError> {
    let parts: Vec<(RecipeNodeId, RecipeNodeId, Unplaced)> = evaluation
        .unplaced
        .iter()
        .filter(|(node, _)| {
            matches!(
                doc.node(**node),
                Some(editor_core::Node::InstantiatePart { .. })
            )
        })
        .map(|(&node, &(group, cause))| (node, group, cause))
        .collect();
    if !parts.is_empty() {
        return Err(ExportError::Unplaced { parts });
    }
    let groups = evaluation.all_unplaced_below();
    if !groups.is_empty() {
        return Err(ExportError::UnplacedBelow { groups });
    }
    let body = editor_core::product(doc, evaluation, tol).map_err(ExportError::Product)?;
    step_string(&body, options, tol).map_err(ExportError::Step)
}
