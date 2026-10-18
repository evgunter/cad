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
    BooleanValue, Evaluation, NodeStanding, ProductError, ProfileDoc, RecipeNodeId, Said, Say,
    Speaker, ValuePayload,
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
    /// The whole-document door's gather refused: an empty world
    /// (naming the unplaced bodies and the loose copies), a stranded
    /// placement, a failed placement, or a kernel refusal while
    /// gathering.
    Product(ProductError),
    /// **Loose copies** (D10, Spaces): STEP writes one world, and these
    /// copies live in loose spaces of their own, which their bundles do
    /// not pin to it — each by the loose copy whose space it is.
    Loose {
        /// Every loose space the export would have to write, by its
        /// loose copy, in document order.
        copies: Vec<RecipeNodeId>,
    },
}

impl Say for ExportError {
    fn say(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        match self {
            ExportError::Standing(standing) => write!(f, "export: {}", Said(standing, by)),
            ExportError::NotABody { node, kind } => {
                write!(
                    f,
                    "export: {} evaluates to a `{kind}`, not a body",
                    by.node(*node)
                )
            }
            ExportError::EmptyBoolean { node } => {
                write!(
                    f,
                    "export: {}'s Boolean is empty — nothing to export",
                    by.node(*node)
                )
            }
            ExportError::Step(e) => write!(f, "export: the STEP writer refused: {e}"),
            ExportError::Product(e) => write!(f, "export: {}", Said(e, by)),
            ExportError::Loose { copies } => {
                write!(
                    f,
                    "export: STEP writes one world, and these copies are loose, in spaces their \
                     bundles do not pin to it:"
                )?;
                for copy in copies {
                    write!(f, " {};", by.node(*copy))?;
                }
                write!(
                    f,
                    " {}",
                    editor_core::Recourse(
                        "add the constraints that pin each to a copy in the world, or to the world"
                    )
                )
            }
        }
    }
}

/// The sentence where no document is at hand: this document's nodes
/// by their tags.
impl core::fmt::Display for ExportError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.say(f, Speaker::TAG)
    }
}

impl ExportError {
    /// **The refusal as the frame holding the evaluated document says
    /// it**: each of its nodes as `doc` holds it now. The door reads an
    /// evaluation alone, so the refusal holds this document's ids,
    /// never their labels.
    #[must_use]
    pub fn spoken(&self, doc: &ProfileDoc) -> String {
        editor_core::spoken_by(self, doc)
    }
}

impl core::error::Error for ExportError {}

/// Serializes the single body `node` denotes in `evaluation` as a STEP
/// (AP214 Part 21) exchange file and returns it as a string.
///
/// Accepts a `Body` value, a non-empty `Boolean`, or an instance's
/// copies read as its part product's one body; every other shape of
/// result is a typed [`ExportError`], never a guess.
///
/// # Errors
///
/// Every arm of [`ExportError`]: the evaluation-side refusals above,
/// [`ExportError::Loose`] for a node that lives in a loose copy's
/// space (a construction's is its own coordinates, written as they
/// are), or [`ExportError::Step`] carrying the writer's own refusal.
pub fn step_for_node(
    evaluation: &Evaluation<f64>,
    node: RecipeNodeId,
    options: &StepOptions,
    tol: Tol,
) -> Result<String, ExportError> {
    // A construction's space is its own coordinates, which its body is
    // written in; a loose copy's has no place to write it at.
    if let Some(root) = evaluation.space(node).loose()
        && evaluation.loose_copies.contains(&root)
    {
        return Err(ExportError::Loose { copies: vec![root] });
    }
    let value = evaluation.usable(node).map_err(ExportError::Standing)?;
    let body = match &value.payload {
        ValuePayload::Body(body) => body,
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => body,
        ValuePayload::Boolean(BooleanValue::Empty) => {
            return Err(ExportError::EmptyBoolean { node });
        }
        // An instance's copies, read as the one body its part product is.
        ValuePayload::Copies(copies) if copies.bodies.len() == 1 => &copies.bodies[0],
        other => {
            return Err(ExportError::NotABody {
                node,
                kind: other.kind_name(),
            });
        }
    };
    step_string(body, options, tol).map_err(ExportError::Step)
}

/// Serializes the WHOLE DOCUMENT's product — the world: every copy a
/// placement defines, in the placements' document order — as a STEP
/// (AP214 Part 21) exchange file.
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
/// [`ExportError::Loose`] naming every loose copy, which the product
/// leaves out and STEP's one world cannot hold;
/// [`ExportError::Product`] carrying the gather's own typed refusal
/// (an empty world naming the unplaced bodies, a placement whose body
/// is gone, a failed placement, a kernel graft or validity refusal), or
/// [`ExportError::Step`] carrying the writer's.
pub fn export_document_step(
    evaluation: &Evaluation<f64>,
    doc: &ProfileDoc,
    options: &StepOptions,
    tol: Tol,
) -> Result<String, ExportError> {
    // A world beside loose copies is refused rather than written
    // without them; a world of loose copies alone is the gather's
    // `EmptyProduct`, which names them.
    let copies = evaluation.loose_roots();
    if !copies.is_empty()
        && doc
            .placements()
            .iter()
            .any(|&p| evaluation.space(p).loose().is_none())
    {
        return Err(ExportError::Loose { copies });
    }
    let body = editor_core::product(doc, evaluation, tol).map_err(ExportError::Product)?;
    step_string(&body, options, tol).map_err(ExportError::Step)
}
