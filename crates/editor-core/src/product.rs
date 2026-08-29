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
//! Bodies arrive through [`topo::graft_disjoint_all_keyed`] — the
//! disjoint half of the boolean pipeline's combine door — one call per
//! source BODY, exactly as `step-import` materializes a STEP
//! assembly's instances. A source body carrying several solids (an
//! instantiated sub-assembly) arrives as several solids of the product
//! in its own solid order: that door is the N-solid one. Nothing is
//! fused and no seam is implied; provenance rides through
//! verbatim, so a pattern instance's `GeomSource::placed(node, i)`
//! survives into the product, and the `Instance(i)` names the pattern
//! minted keep addressing it through the evaluation's own name tables
//! (the gather touches no table).
//!
//! Validation is the same F8/D7 shape as the import loop: each source
//! body is gated on its own when the product holds more than one solid
//! (with one, the per-solid and aggregate subjects are the same body
//! and the call is skipped as an identity, never as an exemption), then the
//! aggregate is gated. Both gates go through the SCALAR'S at-rest
//! policy ([`topo::AtRestPolicy`], `docs/DUAL-DESIGN.md` DL3):
//! certifying scalars run [`topo::validate_geometric`] verbatim; at a
//! dual the gates are structurally absent, and their success arm SAYS
//! so ([`topo::AtRestOutcome::NotRunAtThisScalar`]). The PAIRING
//! OBLIGATION rides with that: at a non-certifying scalar a gathered
//! product is NOT a validated product — the base-scalar evaluation
//! beside it, whose value channel is bit-identical, is the validation
//! of record. Disjoint multi-solid bodies are tier-3 legal.
//! Know what the aggregate gate proves: tier 3 is a LOCAL battery
//! (per-face, per-edge, per-edge–face-pair, plus one whole-body signed
//! volume that SUMS), so solids that OVERLAP pass THIS call undetected
//! — inter-solid interference is not among its checks. Undeclared
//! cross-instance contact is A5's hard error and interference fits are
//! C6's recorded-gate-skips territory; both are decided by the tier-3′
//! door ([`topo::validate_pseudomanifold`]), which the ASSEMBLY gate
//! runs over this gather's output ([`crate::assemble`]) and which this
//! function does not.
//!
//! **That is a division of labour, not a gap** (#382 closed at M9-2 —
//! the census reaches the touching/overlap space and nothing in it
//! validates silently). The gather stays on the local battery because
//! tier 3′ is quadratic in the aggregate's entities, which a caller
//! gathering on every edit cannot afford: the heatsink at 160 fins
//! (966 faces) costs ~1.1 s there, against ~28 ms for the whole check
//! registry over the same product (gather included). The per-call
//! split between this gather and the resident above it was never
//! measured separately and no number for it is stated here. What the
//! gather
//! DOES owe — [`topo::graft_disjoint_all_keyed`] asserts nothing about
//! its operands, so every caller of it must establish disjointness —
//! is discharged by [`crate::checks`]'s separation resident, which
//! reads this gather's `solid_roots` and holds every cross-root solid
//! pair to the box-level certificate. It reports rather than refuses,
//! because a viewer must keep drawing a document it can diagnose.

use std::sync::Arc;

use geom_core::Decide;
use topo::{AtRestPolicy, Body, ContactRecords, ValidationError};

use crate::doc::Doc;
use crate::eval::{BooleanValue, Evaluation, NodeResult, NodeValue, SplitSide, ValuePayload};
use crate::names::{EntityKey, EntityRef, Entry, NameTable, StableName};
use crate::node::RecipeNodeId;
use geom_core::Tol;

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
    /// The kernel's disjoint-graft door refused a source body.
    Graft {
        /// The root whose body was being grafted.
        node: RecipeNodeId,
        /// The kernel's own refusal.
        source: Box<topo::BooleanError>,
    },
    /// A source body failed the at-rest validity gate on its own — a
    /// multi-solid source is gated whole, as one body (only asked when
    /// the product holds more than one solid).
    SolidInvalid {
        /// The root that contributed it.
        node: RecipeNodeId,
        /// Every failure the validator found.
        errors: Vec<ValidationError>,
    },
    /// The gathered product failed the at-rest validity gate — a
    /// per-entity local verdict; inter-solid overlap is not among its
    /// checks (module docs, issue #382).
    ProductInvalid {
        /// Every failure the validator found.
        errors: Vec<ValidationError>,
    },
    /// A source's declared contact record names an entity the graft's
    /// descendant map has no image for (ASM-R2b D-1). The bridge is
    /// total over what it grafted, so this is a bridge bug surfaced —
    /// never a quietly dropped declaration.
    ContactLineage {
        /// The root whose records were being carried.
        node: RecipeNodeId,
        /// Which entity kind had no descendant.
        what: &'static str,
    },
}

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM and FORWARDS its payload's own `Display` — the kernel's
// refusals and validity findings both carry one, so no arm re-states
// them (and none Debug-dumps them). A validity-finding list renders
// one kernel finding per indented line, the finding sink's list
// shape; node ids render plain, names as kind + minting node.
impl core::fmt::Display for ProductError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let list = |f: &mut core::fmt::Formatter<'_>, errors: &[ValidationError]| {
            for error in errors {
                write!(f, "\n  {error}")?;
            }
            Ok(())
        };
        match self {
            Self::UnknownNode { node } => {
                write!(
                    f,
                    "product: root {} has no entry in this evaluation",
                    node.0
                )
            }
            Self::RootFailed { node } => write!(
                f,
                "product: root {} failed to evaluate (ask \
                 `Evaluation::node_error` for the typed cause)",
                node.0
            ),
            Self::RootPoisoned { node, through } => write!(
                f,
                "product: root {} never ran — poisoned through \
                 failed ancestor {}",
                node.0, through.0
            ),
            Self::NoBodyRoots => f.write_str(
                "product: no product root denotes a body — this document \
                 has no body product",
            ),
            Self::Naming { node, name } => write!(
                f,
                "product: root {}'s {} name (minted by node {}) collides in the \
                 product's name table",
                node.0,
                name.kind.noun(),
                name.node.0
            ),
            Self::Graft { node, source } => {
                write!(f, "product: grafting root {} refused: {source}", node.0)
            }
            Self::SolidInvalid { node, errors } => {
                write!(
                    f,
                    "product: root {}'s solid is not valid at rest ({} finding(s)):",
                    node.0,
                    errors.len()
                )?;
                list(f, errors)
            }
            Self::ProductInvalid { errors } => {
                write!(
                    f,
                    "product: the gathered product is not valid at rest ({} finding(s)):",
                    errors.len()
                )?;
                list(f, errors)
            }
            Self::ContactLineage { node, what } => write!(
                f,
                "product: root {}'s declared contact names a {what} the \
                 graft's descendant map has no image for — the key bridge is \
                 incomplete; declarations are never dropped to make a gather \
                 succeed",
                node.0
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
///
/// Each source also carries the DECLARED CONTACT RECORDS keyed in that
/// body's arena (ASM-R2b D-1). This function is the ONE place the
/// channel's two homes reconcile: a boolean's records ride its payload
/// (the `BooleanBody` contract, which predates the channel), every
/// other op's ride [`crate::eval::NodeValue::contacts`]. Downstream
/// therefore never asks which op put records where.
pub(crate) fn sources_of<T: Decide>(value: &NodeValue<T>) -> Option<Vec<Source0<T>>> {
    let carried = || Arc::clone(&value.contacts);
    let none = || Arc::new(ContactRecords::default());
    match &value.payload {
        ValuePayload::Body(body) => Some(vec![(0, Arc::clone(body), carried())]),
        ValuePayload::Boolean(BooleanValue::Body { body, contacts, .. }) => {
            Some(vec![(0, Arc::clone(body), Arc::clone(contacts))])
        }
        ValuePayload::Boolean(BooleanValue::Empty) => Some(Vec::new()),
        // Multi-output ops carry no records (the `OpOut` invariant):
        // "output body 0" names nothing here, so there is no home to
        // read from and none is invented.
        ValuePayload::Instances(bodies) => Some(
            bodies
                .iter()
                .enumerate()
                .map(|(i, body)| {
                    (
                        u32::try_from(i).unwrap_or(u32::MAX),
                        Arc::clone(body),
                        none(),
                    )
                })
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
                    SplitSide::Body(body) => Some((ix, Arc::clone(body), none())),
                    SplitSide::Empty => None,
                })
                .collect(),
        ),
        // A12: the gather IGNORES a mate — it is a non-body root, and
        // "ignored by the gather" is exactly this arm. E3/E10's two
        // sinks join it for the same reason: a measured quantity and an
        // an assertion's verdict denote no material, so a document
        // whose only addition is an assertion has the same product it
        // had without one.
        ValuePayload::Datum(_)
        | ValuePayload::Profile(_)
        | ValuePayload::Declarations(_)
        | ValuePayload::Mate(_)
        | ValuePayload::Measure { .. }
        | ValuePayload::Assertion(_) => None,
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
pub fn product<P, T: Decide + AtRestPolicy>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    tol: Tol,
) -> Result<Body<T>, ProductError> {
    product_recorded(doc, evaluation, tol).map(|p| p.body)
}

/// The whole-document product, with everything the gather knows about
/// it: the aggregate body, its name table, and its DECLARED CONTACT
/// RECORDS (ASM-R2b D-1).
///
/// Three doors, one implementation — [`product`] and [`product_named`]
/// are this function with fields dropped — because a gather that named
/// or recorded its entities differently from the gather that shipped
/// them would be a second truth about what a document's product is.
#[derive(Debug)]
pub struct Product<T: Decide> {
    /// The gathered aggregate.
    pub body: Body<T>,
    /// Its stable names, re-keyed onto the aggregate ([`product_named`]).
    pub names: NameTable,
    /// Its declared contacts, re-keyed onto the aggregate through the
    /// graft's own descendant map.
    pub contacts: ContactRecords,
    /// Which product ROOT contributed each of the aggregate's solids,
    /// in gather order (`crate::checks`'s separation resident is the
    /// consumer: it turns a kernel finding about two solid keys into a
    /// sentence about two roots).
    ///
    /// Read off the GRAFT's own minted-key list, exactly as the name
    /// and contact carries are — never re-derived by looking at the
    /// gathered geometry.
    pub solid_roots: Vec<SolidOrigin>,
}

/// One gathered solid's origin: the root that contributed it, that
/// root's output-body index, and the solid's key in the aggregate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SolidOrigin {
    /// The product root the solid came from.
    pub node: RecipeNodeId,
    /// The output-body index within that root's value.
    pub output: u32,
    /// The solid's key in the gathered aggregate.
    pub solid: topo::SolidKey,
}

/// The document's product, with the product's own NAME TABLE: every
/// gathered root's stable names, re-keyed onto the aggregate's entities
/// (ASM-2A D-4).
///
/// One implementation serves all three doors — this is
/// [`product_recorded`] with the contacts dropped.
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
pub fn product_named<P, T: Decide + AtRestPolicy>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    tol: Tol,
) -> Result<(Body<T>, NameTable), ProductError> {
    product_recorded(doc, evaluation, tol).map(|p| (p.body, p.names))
}

/// The document's product with its name table AND its declared contact
/// records — the widest of the three doors, and the one the other two
/// are defined by ([`Product`]).
///
/// # The contacts carry (ASM-R2b D-1)
///
/// A source body's records move onto the aggregate through the GRAFT's
/// own descendant map, exactly as its name rows do — the lineage rule
/// the boolean pipeline's `remap_contacts` states: a record's new key
/// is the key the graft says its old entity BECAME, never a key
/// re-derived by looking at the gathered geometry. Re-derivation is
/// the scan-to-bless move F1 bans; there is no second opinion here
/// about which faces touch.
///
/// # Errors
///
/// Every arm of [`ProductError`], including
/// [`ProductError::ContactLineage`] when the graft's bridge has no
/// image for a record's entity.
pub fn product_recorded<P, T: Decide + AtRestPolicy>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    tol: Tol,
) -> Result<Product<T>, ProductError> {
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
        let Some(bodies) = sources_of(value) else {
            continue;
        };
        any_body_denoting = true;
        sources.extend(
            bodies.into_iter().map(|(ix, body, contacts)| {
                (node, ix, body, Arc::clone(&value.name_table), contacts)
            }),
        );
    }
    if !any_body_denoting {
        return Err(ProductError::NoBodyRoots);
    }

    // Pass 2: the per-source gate, asked only when the product holds
    // more than one solid (the import loop's rule, verbatim). The count
    // is over SOLIDS, not sources: one source may itself carry several
    // (an instantiated sub-assembly), and it is the product's solid
    // count the rule speaks about.
    let total_solids: usize = sources
        .iter()
        .map(|(_, _, b, _, _)| b.solids().count())
        .sum();
    if total_solids > 1 {
        for (node, _, body, _, _) in &sources {
            T::gate_at_rest(body.as_ref(), tol).map_err(|errors| ProductError::SolidInvalid {
                node: *node,
                errors,
            })?;
        }
    }

    // Pass 3: the graft, one call per SOURCE BODY, in list order, each
    // carrying its source's name rows across on the key bridge. A
    // source holding N solids goes through as one call: the keyed graft
    // is the N-solid door (#381), and its per-entity bridge is total
    // over the source however many solids it spans, so the name carry
    // is the same code for N as for 1.
    let mut aggregate = Body::new();
    let mut names = NameTable::new();
    let mut contacts = ContactRecords::default();
    let mut solid_roots: Vec<SolidOrigin> = Vec::new();
    for (node, ix, body, table, records) in &sources {
        // An empty source contributes nothing; the graft door refuses a
        // solidless body, so the skip is here rather than there.
        if body.solids().next().is_none() {
            continue;
        }
        let keys = topo::graft_disjoint_all_keyed(&mut aggregate, body.as_ref(), tol).map_err(
            |source| ProductError::Graft {
                node: *node,
                source: Box::new(source),
            },
        )?;
        solid_roots.extend(keys.solids().iter().map(|&solid| SolidOrigin {
            node: *node,
            output: *ix,
            solid,
        }));
        carry_names(&mut names, table, *ix, &keys)
            .map_err(|name| ProductError::Naming { node: *node, name })?;
        carry_contacts(&mut contacts, records, &keys)
            .map_err(|what| ProductError::ContactLineage { node: *node, what })?;
    }
    T::gate_at_rest(&aggregate, tol).map_err(|errors| ProductError::ProductInvalid { errors })?;
    Ok(Product {
        body: aggregate,
        names,
        contacts,
        solid_roots,
    })
}

/// One body the gather will graft: which root contributed it, which
/// OUTPUT-BODY index it occupies in that root's value (the index its
/// name rows are keyed by), the body, the root's name table, and the
/// body's declared contact records.
type Source<T> = (
    RecipeNodeId,
    u32,
    Arc<Body<T>>,
    Arc<NameTable>,
    Arc<ContactRecords>,
);

/// One body-denoting source as [`sources_of`] hands it back: output
/// index, body, and the records keyed in that body's arena.
pub(crate) type Source0<T> = (u32, Arc<Body<T>>, Arc<ContactRecords>);

/// Re-keys one grafted body's contact records onto the aggregate,
/// through the graft's DESCENDANT MAP (`product_recorded`'s contacts
/// carry). INVARIANT: every key here is `keys.<kind>(old)` — the
/// lineage the graft recorded. Nothing is looked up by position, by
/// name, or by re-measuring the aggregate.
///
/// The bridge is total over a source it grafted, so a missing image is
/// a bridge bug, not a dropped declaration: it refuses typed rather
/// than silently weakening the at-rest gate the records feed.
///
/// [`ProductError::ContactLineage`] is therefore DEFENSIVE and has no
/// test: no public API can reach it without first breaking the graft's
/// own key-bridge contract. Stated rather than left to be discovered —
/// an arm that cannot be exercised is worth knowing about, and the
/// alternative (dropping the record) is the failure this guards.
fn carry_contacts(
    into: &mut ContactRecords,
    from: &ContactRecords,
    keys: &topo::GraftKeys,
) -> Result<(), &'static str> {
    let vertex = |v| keys.vertex(v).ok_or("vertex");
    let face = |f| keys.face(f).ok_or("face");
    let edge = |e| keys.edge(e).ok_or("edge");
    for c in &from.vv {
        into.vv.push(topo::VvContact {
            a: vertex(c.a)?,
            b: vertex(c.b)?,
        });
    }
    for (src, dst) in [(&from.a_on_b, 0u8), (&from.b_on_a, 1)] {
        for c in src {
            let moved = topo::VfContact {
                vertex: vertex(c.vertex)?,
                face: face(c.face)?,
            };
            if dst == 0 {
                into.a_on_b.push(moved);
            } else {
                into.b_on_a.push(moved);
            }
        }
    }
    for c in &from.curves {
        into.curves.push(topo::CurveContact {
            face_a: face(c.face_a)?,
            face_b: face(c.face_b)?,
            witness: edge(c.witness)?,
        });
    }
    for c in &from.patches {
        into.patches.push(topo::PatchContact {
            face_a: face(c.face_a)?,
            face_b: face(c.face_b)?,
        });
    }
    Ok(())
}

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
