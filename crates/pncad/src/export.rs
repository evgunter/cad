//! **The document layer's export door**.
//!
//! `step_export::step_string` takes a finished kernel body
//! ([`topo::AtRestBody`]); before
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
//!
//! # Read-outs
//!
//! The bindings' bodies leave through here too: [`body_in_space`],
//! [`assembly_body_in_space`] and [`pick_mesh_in_space`] compose a body
//! (or its pick mesh) into its space as export does, for a
//! host-language read-out that never re-enters a document.

use editor_core::{
    BooleanValue, Evaluation, NodeStanding, ProductError, ProfileDoc, RecipeNodeId, Said, Say,
    Speaker, ValuePayload,
};
use geom_core::Tol;
use step_export::{StepExportError, StepOptions, step_string};
use topo::{AtRestBody, Body};

/// Why [`step_for_node`] refused. Fail-loud and typed, D2-style; the
/// standing carries the ids to ask the caller's own [`Evaluation`] for
/// the payload (a failure's root cause is one
/// [`Evaluation::node_error`] call away — the error type does not
/// clone the kernel's non-`Clone` refusals to repeat them here).
#[derive(Debug)]
pub enum ExportError {
    /// The node has no value in this evaluation.
    Standing(NodeStanding),
    /// The node's value is not a single body (`profile`, `pose`,
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
    /// The node's body fails the at-rest gate
    /// ([`topo::AtRestBody::validate`]), so it is not a finished body
    /// and the writer is not handed it.
    BodyInvalid {
        /// The node whose body was refused.
        node: RecipeNodeId,
        /// The gate's findings, verbatim.
        errors: Vec<topo::ValidationError>,
    },
    /// The body could not be composed into its space's coordinates: the
    /// kernel refused the rigid map of its frame
    /// ([`editor_core::in_space`]).
    Compose(topo::TransformError),
    /// The body was denoted but the STEP writer refused it.
    Step(StepExportError),
    /// The whole-document door's gather refused: an empty world
    /// (naming the unplaced bodies), a stranded placement, a failed
    /// placement, or a kernel refusal while gathering.
    Product(ProductError),
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
            ExportError::BodyInvalid { node, errors } => write!(
                f,
                "export: {}'s body is not at rest ({} finding(s), first: {})",
                by.node(*node),
                errors.len(),
                errors
                    .first()
                    .map_or_else(|| "none".to_owned(), ToString::to_string)
            ),
            ExportError::Compose(e) => write!(
                f,
                "export: the body could not be composed into its space's coordinates: {e}"
            ),
            ExportError::Step(e) => write!(f, "export: the STEP writer refused: {e}"),
            ExportError::Product(e) => write!(f, "export: {}", Said(e, by)),
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
/// Accepts a `Body` value or a non-empty `Boolean`, an instance's copy
/// and a `⊔` among them; every other shape of result is a typed
/// [`ExportError`], never a guess.
///
/// # Errors
///
/// Every arm of [`ExportError`]: the evaluation-side refusals above, or
/// [`ExportError::BodyInvalid`] for a body the at-rest gate refuses, or
/// [`ExportError::Step`] carrying the writer's own refusal. A body is
/// written in its space's coordinates, as it stands there.
pub fn step_for_node(
    evaluation: &Evaluation<f64>,
    node: RecipeNodeId,
    options: &StepOptions,
    tol: Tol,
) -> Result<String, ExportError> {
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
    let body = in_its_space(body.body(), &editor_core::in_space(body), tol)?;
    let body = AtRestBody::validate(body, tol)
        .map_err(|errors| ExportError::BodyInvalid { node, errors })?;
    step_string(&body, options, tol).map_err(ExportError::Step)
}

/// **`body` composed into its space** by `map`, the frame
/// [`editor_core::in_space`] answers: itself where the frame is the
/// identity, so a body computed in its space's coordinates is written
/// as it was built.
fn in_its_space(
    body: &Body<f64>,
    map: &geom_core::RigidMotion<f64>,
    tol: Tol,
) -> Result<Body<f64>, ExportError> {
    if is_identity(map) {
        return Ok(body.clone());
    }
    topo::transform_rigid(body, map, tol).map_err(ExportError::Compose)
}

/// **A body value as it stands in its space**, the façade's read-out
/// door: the body composed through [`editor_core::in_space`], shared
/// as it is where its frame is the identity.
///
/// A read-out is an export in D10 `[placement.world]`'s sense: what it
/// answers (a host-language body, its points and measures) has no way
/// back into a document. The bindings read a value's bodies through
/// this door and compose nothing themselves.
///
/// # Errors
///
/// [`topo::TransformError`] when the kernel refuses the frame's rigid
/// map.
pub fn body_in_space(
    body: &editor_core::Framed<f64>,
    tol: Tol,
) -> Result<std::sync::Arc<Body<f64>>, topo::TransformError> {
    let map = editor_core::in_space(body);
    if is_identity(&map) {
        return Ok(std::sync::Arc::clone(body.shared()));
    }
    topo::transform_rigid(body.body(), &map, tol).map(std::sync::Arc::new)
}

/// **An assembly's body as it stands in the world**, [`body_in_space`]
/// for the body [`editor_core::assemble`] gated: borrowed as it is
/// where the gather computed in the world's coordinates. The rigid map
/// keeps every arena key, so the assembly's contact records cite the
/// composed body as they cited the gathered one.
///
/// # Errors
///
/// As [`body_in_space`].
pub fn assembly_body_in_space(
    assembly: &editor_core::Assembly<f64>,
    tol: Tol,
) -> Result<std::borrow::Cow<'_, Body<f64>>, topo::TransformError> {
    let map = editor_core::in_space(assembly);
    if is_identity(&map) {
        return Ok(std::borrow::Cow::Borrowed(&assembly.body));
    }
    topo::transform_rigid(&assembly.body, &map, tol).map(std::borrow::Cow::Owned)
}

/// **A pick index's mesh as it stands in its space**, the read-out a
/// host-language display draws: [`editor_core::NodePick::mesh`] is in
/// its body's frame, and [`editor_core::pick_face`] reads a ray in the
/// space's coordinates into that frame, so the mesh drawn is the one
/// composed here. Borrowed as it is where the frame is the identity; a
/// rigid map keeps every winding and every patch's and polyline's key.
#[must_use]
pub fn pick_mesh_in_space(pick: &editor_core::NodePick) -> std::borrow::Cow<'_, mesh::Mesh> {
    let map = editor_core::in_space(pick);
    if is_identity(&map) {
        return std::borrow::Cow::Borrowed(pick.mesh());
    }
    let mut mesh = pick.mesh().clone();
    for p in &mut mesh.positions {
        *p = map.transform_point(*p);
    }
    std::borrow::Cow::Owned(mesh)
}

/// Whether `map` is the identity, bit for bit.
fn is_identity(map: &geom_core::RigidMotion<f64>) -> bool {
    let (u, v, w, o) = (map.u().get(), map.v().get(), map.w().get(), map.origin());
    [u.x, u.y, u.z, v.x, v.y, v.z, w.x, w.y, w.z, o.x, o.y, o.z]
        == [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]
}

/// Serializes the WHOLE DOCUMENT's product — the body its product slot
/// reads, a `⊔`'s pieces each its own solids — as a STEP (AP214 Part 21)
/// exchange file.
///
/// This is the door that accepts what [`step_for_node`] refuses: a
/// pattern's instances, a split's two halves, several disjoint tips.
/// One writer path serves both cases — `step_export` already emits one
/// `MANIFOLD_SOLID_BREP` per solid, which is exactly
/// the shape an imported multi-solid assembly round-trips through — so
/// a one-solid product is not a special case here, it is the same call
/// with one solid.
///
/// # Errors
///
/// [`ExportError::Product`] carrying the gather's own typed refusal
/// (an empty slot, a product whose body is gone, a failed operand, a
/// kernel graft or validity refusal), or
/// [`ExportError::Step`] carrying the writer's.
pub fn export_document_step(
    evaluation: &Evaluation<f64>,
    doc: &ProfileDoc,
    options: &StepOptions,
    tol: Tol,
) -> Result<String, ExportError> {
    let product =
        editor_core::product_recorded(doc, evaluation, tol).map_err(ExportError::Product)?;
    // The product is in the frame its gather computed in; it is written
    // in the world's coordinates.
    let map = editor_core::in_space(&product);
    if is_identity(&map) {
        return step_string(&product.body, options, tol).map_err(ExportError::Step);
    }
    let body = in_its_space(&product.body, &map, tol)?;
    let body = AtRestBody::validate(body, tol).map_err(|errors| {
        ExportError::Product(editor_core::ProductError::ProductInvalid { errors })
    })?;
    step_string(&body, options, tol).map_err(ExportError::Step)
}
