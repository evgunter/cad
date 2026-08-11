//! **The whole-document product** (ASSEMBLY-DESIGN A10; ASM-ROOTS
//! D-4): the deterministic gather, in root-list order, of every
//! body-denoting product root into ONE aggregate [`Body`].
//!
//! This is what A2's "an assembly's evaluation is a body" means for a
//! part document, and it is C1's resolution: [`ValuePayload::Instances`]
//! keeps its semantics ("patterns do not implicitly union"), and the
//! ROOT GATHER is what materializes an instances-valued root into
//! placed solids of the one product body — disjoint solids, no boolean
//! implied. The single-node export door's multi-body refusal therefore
//! stays correct; THIS is the door that accepts them.
//!
//! # What a root contributes
//!
//! - [`ValuePayload::Body`] and a non-empty [`ValuePayload::Boolean`] —
//!   their solids;
//! - [`ValuePayload::Instances`] — each placed body, in instance order;
//! - [`ValuePayload::Split`] — both pieces, `above` then `below` (a
//!   mold document wants both halves).
//!
//! A root that denotes no body at all (a datum, a WIP profile tip, a
//! declaration) contributes NOTHING and is not an error — A10 states
//! that outright. An EMPTY boolean or split side is body-denoting and
//! contributes no solid: the empty result is a typed success upstream
//! (F8) and stays one here.
//!
//! # The posture the gather copies
//!
//! Bodies arrive through [`topo::graft_disjoint`] — the disjoint half
//! of the boolean pipeline's combine door — one call per source solid,
//! exactly as `step-import` materializes a STEP assembly's instances.
//! Nothing is fused and no seam is implied; provenance rides through
//! verbatim, so a pattern instance's `GeomSource::placed(node, i)`
//! survives into the product, and the `Instance(i)` names the pattern
//! minted keep addressing it through the evaluation's own name tables
//! (the gather touches no table).
//!
//! Validation is the same F8/D7 shape as the import loop: each source
//! solid is gated on its own when the product has more than one (with
//! one, the per-solid and aggregate subjects are the same body and the
//! call is skipped as an identity, never as an exemption), then the
//! aggregate is gated. Disjoint multi-solid bodies are tier-3 legal;
//! solids that overlap are a false body and tier 3 says so.

use std::sync::Arc;

use geom_core::Decide;
use topo::{Body, PropsQuadLane, ValidationError};

use crate::doc::Doc;
use crate::eval::{BooleanValue, Evaluation, NodeResult, SplitSide, ValuePayload};
use crate::names::{EntityKey, EntityRef, Entry, NameTable, StableName};
use crate::node::RecipeNodeId;

/// Why [`product`] refused. Fail-loud and typed: a product is all of
/// the roots or none of them — there are no partial products.
#[derive(Debug)]
pub enum ProductError {
    /// A root has no entry in this evaluation (never scheduled, or
    /// past a cancelation's completed prefix).
    UnknownNode {
        /// The root that was asked for.
        node: RecipeNodeId,
    },
    /// Two roots' name rows would alias in the product table — the
    /// same name twice, or two names on one aggregate entity. An
    /// emission-level bug surfaced, never resolved by picking one.
    Naming {
        /// The root whose rows collided.
        node: RecipeNodeId,
        /// The colliding name.
        name: Box<StableName>,
    },
    /// A root's node failed to evaluate (ask
    /// [`Evaluation::node_error`] for the typed cause).
    RootFailed {
        /// The failed root.
        node: RecipeNodeId,
    },
    /// A root never ran: an ancestor failed.
    RootPoisoned {
        /// The poisoned root.
        node: RecipeNodeId,
        /// Its nearest failed ancestor.
        through: RecipeNodeId,
    },
    /// No root denotes a body — a profile-only or datum-only document
    /// has no product for a door that needs one.
    NoBodyRoots,
    /// A body-denoting root's value is a MULTI-SOLID body, which the
    /// kernel's graft door cannot take apart today
    /// ([`topo::graft_disjoint`] admits single-solid sources only).
    /// The flip condition is **ASM-2b**, which owns the multi-solid
    /// graft: when that door lands, this arm becomes unreachable and
    /// the loop grafts solid by solid.
    MultiSolidRoot {
        /// The root whose value carries several solids.
        node: RecipeNodeId,
        /// How many.
        solids: usize,
    },
    /// The kernel's disjoint-graft door refused a source body.
    Graft {
        /// The root whose body was being grafted.
        node: RecipeNodeId,
        /// The kernel's own refusal.
        source: Box<topo::BooleanError>,
    },
    /// A source solid failed the at-rest validity gate on its own
    /// (only asked when the product has more than one solid).
    SolidInvalid {
        /// The root that contributed it.
        node: RecipeNodeId,
        /// Every failure the validator found.
        errors: Vec<ValidationError>,
    },
    /// The gathered product failed the at-rest validity gate — solids
    /// that overlap are a false body, and tier 3 says so.
    ProductInvalid {
        /// Every failure the validator found.
        errors: Vec<ValidationError>,
    },
}

impl core::fmt::Display for ProductError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnknownNode { node } => {
                write!(f, "product: root {node:?} has no entry in this evaluation")
            }
            Self::RootFailed { node } => write!(
                f,
                "product: root {node:?} failed to evaluate (ask \
                 `Evaluation::node_error` for the typed cause)"
            ),
            Self::RootPoisoned { node, through } => write!(
                f,
                "product: root {node:?} never ran — poisoned through \
                 failed ancestor {through:?}"
            ),
            Self::NoBodyRoots => f.write_str(
                "product: no product root denotes a body — this document \
                 has no body product",
            ),
            Self::Naming { node, name } => write!(
                f,
                "product: root {}'s name {name:?} collides in the product's name table",
                node.0
            ),
            Self::MultiSolidRoot { node, solids } => write!(
                f,
                "product: root {node:?} denotes a {solids}-solid body; \
                 grafting a multi-solid source is ASM-2b's door, not \
                 this one"
            ),
            Self::Graft { node, source } => {
                write!(f, "product: grafting root {node:?} refused: {source:?}")
            }
            Self::SolidInvalid { node, errors } => write!(
                f,
                "product: root {node:?}'s solid is not valid at rest: {errors:?}"
            ),
            Self::ProductInvalid { errors } => write!(
                f,
                "product: the gathered product is not valid at rest: {errors:?}"
            ),
        }
    }
}

impl core::error::Error for ProductError {}

/// The body-denoting sources one root contributes, in gather order,
/// each tagged with the OUTPUT-BODY INDEX it occupies in the root's own
/// value (module docs). That index is what a root's name table keys its
/// rows by, so carrying it here is what lets [`product_named`] find the
/// rows belonging to each grafted body. `None` for a root that denotes
/// no body at all.
fn sources_of<T: Decide>(payload: &ValuePayload<T>) -> Option<Vec<(u32, Arc<Body<T>>)>> {
    match payload {
        ValuePayload::Body(body) => Some(vec![(0, Arc::clone(body))]),
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => Some(vec![(0, Arc::clone(body))]),
        ValuePayload::Boolean(BooleanValue::Empty) => Some(Vec::new()),
        ValuePayload::Instances(bodies) => Some(
            bodies
                .iter()
                .enumerate()
                .map(|(i, body)| (u32::try_from(i).unwrap_or(u32::MAX), Arc::clone(body)))
                .collect(),
        ),
        // Split's halves are output bodies 0 (above) and 1 (below);
        // an EMPTY half contributes nothing but does not shift the
        // other half's index — the index is the value's layout, not a
        // position in this list.
        ValuePayload::Split { above, below } => Some(
            [(0u32, above), (1u32, below)]
                .into_iter()
                .filter_map(|(ix, side)| match side {
                    SplitSide::Body(body) => Some((ix, Arc::clone(body))),
                    SplitSide::Empty => None,
                })
                .collect(),
        ),
        ValuePayload::Datum(_) | ValuePayload::Profile(_) | ValuePayload::Declarations(_) => None,
    }
}

/// The document's product: every body-denoting root's solids gathered,
/// in root-list order, into one [`Body`] (module docs).
///
/// The result is a pure function of (`doc.roots()`, `evaluation`) — no
/// ambient state, so two evaluations of a root-neutral edit yield the
/// same solid order (D9).
///
/// # Errors
///
/// Every arm of [`ProductError`]: a root that failed, was poisoned, or
/// is absent from this evaluation; a document whose roots denote no
/// body ([`ProductError::NoBodyRoots`]); the kernel's graft and
/// at-rest validity refusals.
pub fn product<P, T: Decide + PropsQuadLane>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
) -> Result<Body<T>, ProductError> {
    product_named(doc, evaluation).map(|(body, _)| body)
}

/// The document's product, with the product's own NAME TABLE: every
/// gathered root's stable names, re-keyed onto the aggregate's entities
/// (ASM-2A D-4).
///
/// One implementation serves both doors — [`product`] is this function
/// with the table dropped — because a gather that named its entities
/// differently from the gather that shipped them would be a second
/// truth about what a document's product is.
///
/// The table carries FACE, EDGE and VERTEX rows. Body-kind rows are
/// deliberately absent: a root's body name denotes THAT ROOT's body,
/// and the product is not any root's body — it is the document's, a
/// distinct entity whose naming belongs to whoever mints it (an
/// instantiate node names its own placed body at itself).
///
/// # Errors
///
/// Every arm of [`ProductError`], including [`ProductError::Naming`]
/// when two roots' rows would name one aggregate entity or collide on
/// one name — an aliasing bug surfaced, never resolved silently.
pub fn product_named<P, T: Decide + PropsQuadLane>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
) -> Result<(Body<T>, NameTable), ProductError> {
    // Pass 1: every root's value, refused whole. "No partial products"
    // means a FAILED root refuses even when a later root would have
    // supplied a body, so the whole list is read before anything is
    // grafted.
    let mut sources: Vec<Source<T>> = Vec::new();
    let mut any_body_denoting = false;
    for &node in doc.roots() {
        let result = evaluation
            .result(node)
            .ok_or(ProductError::UnknownNode { node })?;
        let value = match result {
            NodeResult::Ok(value) => value,
            NodeResult::Failed(_) => return Err(ProductError::RootFailed { node }),
            NodeResult::Poisoned { through } => {
                return Err(ProductError::RootPoisoned {
                    node,
                    through: *through,
                });
            }
        };
        let Some(bodies) = sources_of(&value.payload) else {
            continue;
        };
        any_body_denoting = true;
        sources.extend(
            bodies
                .into_iter()
                .map(|(ix, body)| (node, ix, body, Arc::clone(&value.name_table))),
        );
    }
    if !any_body_denoting {
        return Err(ProductError::NoBodyRoots);
    }

    // Pass 2: the per-solid gate, asked only when the product holds
    // more than one solid (the import loop's rule, verbatim).
    if sources.len() > 1 {
        for (node, _, body, _) in &sources {
            topo::validate_geometric(body.as_ref()).map_err(|errors| {
                ProductError::SolidInvalid {
                    node: *node,
                    errors,
                }
            })?;
        }
    }

    // Pass 3: the graft, one call per source solid, in list order, each
    // carrying its source's name rows across on the key bridge.
    let mut aggregate = Body::new();
    let mut names = NameTable::new();
    for (node, ix, body, table) in &sources {
        let solids = body.solids().count();
        if solids > 1 {
            return Err(ProductError::MultiSolidRoot {
                node: *node,
                solids,
            });
        }
        if solids == 0 {
            continue;
        }
        let keys =
            topo::graft_disjoint_all_keyed(&mut aggregate, body.as_ref()).map_err(|source| {
                ProductError::Graft {
                    node: *node,
                    source: Box::new(source),
                }
            })?;
        carry_names(&mut names, table, *ix, &keys)
            .map_err(|name| ProductError::Naming { node: *node, name })?;
    }
    topo::validate_geometric(&aggregate)
        .map_err(|errors| ProductError::ProductInvalid { errors })?;
    Ok((aggregate, names))
}

/// One body the gather will graft: which root contributed it, which
/// OUTPUT-BODY index it occupies in that root's value (the index its
/// name rows are keyed by), the body, and the root's name table.
type Source<T> = (RecipeNodeId, u32, Arc<Body<T>>, Arc<NameTable>);

/// Re-keys one grafted body's name rows onto the aggregate: the same
/// stable names, pointing at the entities the graft minted. Body index
/// on the product side is 0 — the product is ONE body.
fn carry_names(
    into: &mut NameTable,
    from: &NameTable,
    ix: u32,
    keys: &topo::GraftKeys,
) -> Result<(), Box<StableName>> {
    let mapped = |key: EntityKey| -> Option<EntityKey> {
        match key {
            // See `product_named`: the product's own body is nobody's
            // root body, so root body-rows do not carry.
            EntityKey::Body => None,
            EntityKey::Face(f) => keys.face(f).map(EntityKey::Face),
            EntityKey::Edge(e) => keys.edge(e).map(EntityKey::Edge),
            EntityKey::Vertex(v) => keys.vertex(v).map(EntityKey::Vertex),
        }
    };
    for (name, entry) in from.iter() {
        let rows: Vec<EntityRef> = match entry {
            Entry::Unique(e) => vec![*e],
            Entry::Tied(es) => es.clone(),
        };
        let moved: Vec<EntityRef> = rows
            .into_iter()
            .filter(|e| e.body == ix)
            .filter_map(|e| mapped(e.key).map(|key| EntityRef { body: 0, key }))
            .collect();
        match moved.len() {
            0 => {}
            1 => into.insert(name.clone(), moved[0]).map_err(|e| e.name)?,
            _ => into.insert_tied(name.clone(), moved).map_err(|e| e.name)?,
        }
    }
    Ok(())
}
