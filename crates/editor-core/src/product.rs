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
//! Before any root is read, the recipe is checked for one body placed
//! under two roots — two roots reaching one node through transforms
//! and part selections alone, which would carry its names twice — and
//! the gather refuses that shape as [`ProductError::PlacedUnderTwoRoots`].
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
//! (the gather writes no table of the evaluation's — it builds one
//! aggregate table of its own, and the per-node tables it reads are
//! untouched).
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
//! gathering on every edit cannot afford: the heat sink at 160 fins
//! (161 solids / 991 faces) costs ~11.4 s there — refusing, with 125
//! findings — where THIS gather costs ~250 ms and the whole check
//! registry over a subject already gathered costs ~8 ms. The split is
//! the reason a caller gathers ONCE: the gather, not the resident
//! above it, is what a landing pays for. (That the doubled gather was
//! therefore most of the doubled cost is an INFERENCE from those three
//! numbers, not a fourth measurement: nothing here has timed a landing
//! before and after.) The figures are a dev-profile wall clock and
//! machine-dependent; the ones of record are hosted, re-taken by the
//! `registry split` row of
//! `crates/editor-core/tests/m4_pr8_latency.rs` on a nightly cron
//! gated on `main` having moved, and appended to
//! `docs/perf-data/rebuild-latency/`. What the gather
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
use crate::names::{CarriedRows, EntityKey, NameTable, SplitHalf, StableName};
use crate::node::RecipeNodeId;
use geom_core::Tol;

/// Why [`product`] refused. Fail-loud and typed: a product is all of
/// the roots or none of them — there are no partial products.
#[derive(Debug)]
pub enum ProductError {
    /// The evaluation is an evaluation of ANOTHER document (DI3).
    /// Raised before the first root is read: node ids are minted per
    /// document, so a foreign evaluation can carry entries for these
    /// very ids and gather a product out of another document's
    /// geometry without a single lookup missing.
    EvaluationOfAnotherDocument {
        /// The document whose product was asked for.
        expected: crate::ident::DocumentId,
        /// The document the handed evaluation is of.
        found: crate::ident::DocumentId,
    },
    /// A root has no entry in this evaluation (never scheduled, or
    /// past a cancelation's completed prefix).
    UnknownNode {
        /// The root that was asked for.
        node: RecipeNodeId,
    },
    /// One node's body is placed under two product roots: each root
    /// reaches `placed` through transforms and part selections alone,
    /// and the two select the same body of it (the whole value, or the
    /// same half or instance).
    ///
    /// Raised from the recipe before any root's value is read, because
    /// the shape alone decides it; why the shape cannot gather, and
    /// which edges it follows, is `placed_under_two_roots`'s doc.
    PlacedUnderTwoRoots {
        /// The node whose body both roots place.
        placed: RecipeNodeId,
        /// Which body of `placed` both roots read: `None` when either
        /// takes it whole, else the one selection they share.
        select: Option<crate::node::PartSelect>,
        /// The earlier of the two roots, in root-list order.
        first: RecipeNodeId,
        /// The later root.
        second: RecipeNodeId,
    },
    /// Name rows the gather carried would alias in the product table
    /// — the same STRICT name twice, or two names on one aggregate
    /// entity. Raised by the per-root carry (`carry_names`, `node` a
    /// root) or by the tie flush after the last source (`node` the
    /// name's minter, since the colliding rows belong to no one root).
    /// Never resolved by picking one.
    ///
    /// Two roots that place one body through transforms and part
    /// selections refuse earlier, as
    /// [`ProductError::PlacedUnderTwoRoots`]. The routes a document
    /// still has to this arm are two:
    ///
    /// - **A split's intact pass-through.** A split root beside another
    ///   root over the split's target shares whichever of the target's
    ///   entities the plane leaves uncut — geometry the recipe cannot
    ///   see. The per-root carry refuses.
    /// - **One instance index spelled two ways.** `Part` selections
    ///   are compared as written, so `Instance(1)` beside
    ///   `Instance(0 + 1)` passes the recipe check and the per-root
    ///   carry refuses.
    ///
    /// Rows arriving under one tied name MERGE into one `Entry::Tied`
    /// rather than colliding — which is what a split ROOT hands the
    /// gather for a tie its plane separates, since the split's own
    /// table keeps the tie across both output bodies. The two halves
    /// taken as two `Part` roots merge the same way: a `Part` that
    /// narrows the tie to the one candidate in its half publishes it
    /// `Unique` but marks the row as one piece of a separated tie
    /// ([`NameTable::project`]), and the carry defers a marked row as
    /// it defers a tied one, so the product table is the split root's,
    /// row for row. What that costs is stated where it lands: the
    /// product genuinely holds two entities under the one name, and a
    /// selection that matches both refuses
    /// (`SelectRefusal::TiedDisagrees`) instead of the gather refusing
    /// for it.
    Naming {
        /// The root whose rows collided — or, for a collision the
        /// tie merge below the roots surfaced, the node that minted
        /// the name (`product_recorded`'s flush).
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
    /// A root never ran: ancestor failed.
    RootPoisoned {
        /// The poisoned root.
        node: RecipeNodeId,
        /// Its nearest failed ancestor.
        through: RecipeNodeId,
    },
    /// No root denotes a body: there is no product for a door that
    /// needs one, and nothing has gone wrong. The reading, and the
    /// documents that are in this state, are
    /// [`ProductErrorKind::means_no_body`]'s.
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

/// **The pairing predicate's finding, in this door's vocabulary.**
///
/// A2a's rule is one predicate (`ident::mispaired`) and one arm per
/// error type over it. The projection lives HERE, at the type that
/// owns the arm, so a door that runs the predicate writes `?` or
/// `m.into()` and no site re-spells which field goes where.
impl From<crate::ident::Mispaired> for ProductError {
    fn from(m: crate::ident::Mispaired) -> Self {
        Self::EvaluationOfAnotherDocument {
            expected: m.expected,
            found: m.found,
        }
    }
}

// The human-readable rendering (LIB-DOORS F6 shape): each arm states
// the PROBLEM and FORWARDS its payload's own `Display` — the kernel's
// refusals and validity findings both carry one, so no arm re-states
// them (and none Debug-dumps them). A validity-finding list renders
// one kernel finding per indented line through the finding sink's own
// `render_lines`, which is where that shape lives for the whole layer;
// node ids render plain, names as kind + minting node.
impl core::fmt::Display for ProductError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let list = crate::finding::render_lines::<&ValidationError, _>;
        match self {
            Self::EvaluationOfAnotherDocument { expected, found } => write!(
                f,
                "product: the evaluation is of document {found}, not of \
                 document {expected}",
            ),
            Self::UnknownNode { node } => {
                write!(
                    f,
                    "product: root {} has no entry in this evaluation",
                    node.0
                )
            }
            Self::PlacedUnderTwoRoots {
                placed,
                select,
                first,
                second,
            } => {
                let what = match select {
                    None => format!("node {}'s body", placed.0),
                    Some(crate::node::PartSelect::SplitHalf(SplitHalf::Above)) => {
                        format!("the above half of node {}", placed.0)
                    }
                    Some(crate::node::PartSelect::SplitHalf(SplitHalf::Below)) => {
                        format!("the below half of node {}", placed.0)
                    }
                    Some(crate::node::PartSelect::Instance(i)) => format!(
                        "instance `{}` of node {}",
                        crate::expr::unparse(i),
                        placed.0
                    ),
                };
                write!(
                    f,
                    "product: {what} is placed under two roots, {} and {} — \
                     a transform or part selection mints no name, so both \
                     would carry its names; place it under one root, or \
                     union the two",
                    first.0, second.0
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
            // TWO SENTENCES BECAUSE `node` CARRIES TWO MEANINGS (the
            // arm's own doc): the root whose rows were being carried,
            // or — for the tie merge's collision, which happens after
            // the last root and belongs to no one of them — the node
            // that minted the name. The guard is what keeps the second
            // from being announced as a root: on that path `node` IS
            // `name.node`, and the sentence below says only what is
            // then true. A carried row could reach it too, by naming
            // its own root's mint, and would be described correctly.
            Self::Naming { node, name } if *node != name.node => write!(
                f,
                "product: root {}'s {} name (minted by node {}) collides in the \
                 product's name table",
                node.0,
                name.kind.noun(),
                name.node.0
            ),
            Self::Naming { name, .. } => write!(
                f,
                "product: the {} name minted by node {} collides in the \
                 product's name table",
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

/// Which arm of [`ProductError`] refused, without the payload.
///
/// A [`ProductError`] is neither `Clone` nor `PartialEq` — it carries
/// validity-finding lists and the kernel's own boolean refusal — so a
/// consumer that must record, compare or hash the refusal has had only
/// the rendered prose to substring-match. This projection drops exactly
/// the part that cannot be cloned or compared, so the class rides where
/// the error itself cannot: into a `Clone + PartialEq` refusal record,
/// a hash key, a test assertion.
///
/// One variant per [`ProductError`] arm, and [`ProductError::kind`]
/// matches exhaustively — an arm added to the error reds `kind` itself,
/// here in this crate.
///
/// A variant HERE with no arm behind it is a phantom: nothing
/// constructs it, so no test can reach it. This module's tests
/// therefore carry the visit that reds one — an exhaustive match over
/// this enum, which names the phantom at compile time. The fix at that
/// red is to delete the phantom, never to give it a label: a name
/// minted for a phantom publishes a class no refusal can ever carry.
///
/// Deliberately NOT `Ord`. The declaration order mirrors
/// [`ProductError`]'s for reading, and nothing depends on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProductErrorKind {
    /// [`ProductError::EvaluationOfAnotherDocument`].
    EvaluationOfAnotherDocument,
    /// [`ProductError::UnknownNode`].
    UnknownNode,
    /// [`ProductError::PlacedUnderTwoRoots`].
    PlacedUnderTwoRoots,
    /// [`ProductError::Naming`].
    Naming,
    /// [`ProductError::RootFailed`].
    RootFailed,
    /// [`ProductError::RootPoisoned`].
    RootPoisoned,
    /// [`ProductError::NoBodyRoots`].
    NoBodyRoots,
    /// [`ProductError::Graft`].
    Graft,
    /// [`ProductError::SolidInvalid`].
    SolidInvalid,
    /// [`ProductError::ProductInvalid`].
    ProductInvalid,
    /// [`ProductError::ContactLineage`].
    ContactLineage,
}

impl ProductErrorKind {
    /// Whether this class means the DOCUMENT denotes no body at all —
    /// nothing to gather, rather than something gone wrong.
    ///
    /// **The one home of that reading.** Every consumer of a gather
    /// refusal draws this line before it can act on one, and it is the
    /// one line all of them draw the same way, so it is drawn here and
    /// cited rather than re-argued at each site. Consumers that badge
    /// or report call it the empty-document reading; the class itself
    /// is about the roots, which is why this is not named for them.
    ///
    /// [`ProductErrorKind::NoBodyRoots`] is the only class where there
    /// is nothing to gather rather than something wrong: a document
    /// with no body-denoting root — a fresh one, one holding only
    /// sketches and datums, one whose last feature was just deleted.
    /// Every other class is a refusal, with a cause the error carries.
    ///
    /// **What a consumer does with either answer stays the consumer's,
    /// in both directions.** `true` says there is nothing to gather; it
    /// does not say nothing is wrong where the CALLER stands, and a
    /// caller that needed a body may be entitled to fail over its
    /// absence. `eval::parts`'s `product_fault` is the live case:
    /// instantiating a body-less part document is a fault of the
    /// instantiate node, so it renders this class as a part fault like
    /// any other. `false` says the class IS a refusal, not that this
    /// consumer is the one to report it — a consumer whose other
    /// channels already carry some of those classes still decides that
    /// for itself.
    ///
    /// Exhaustive over [`ProductErrorKind`], so a class cannot be added
    /// without being classified here. **That is the whole of what the
    /// compiler buys**: a [`ProductError`] arm added under an EXISTING
    /// class inherits that class's answer silently, and nothing reds.
    #[must_use]
    pub fn means_no_body(self) -> bool {
        match self {
            Self::NoBodyRoots => true,
            Self::EvaluationOfAnotherDocument
            | Self::UnknownNode
            | Self::PlacedUnderTwoRoots
            | Self::Naming
            | Self::RootFailed
            | Self::RootPoisoned
            | Self::Graft
            | Self::SolidInvalid
            | Self::ProductInvalid
            | Self::ContactLineage => false,
        }
    }
}

impl ProductError {
    /// Which arm refused, without the payload.
    ///
    /// Exhaustive over [`ProductError`]: adding an arm there is a
    /// compile error here and in every consumer that maps this enum.
    #[must_use]
    pub fn kind(&self) -> ProductErrorKind {
        match self {
            Self::EvaluationOfAnotherDocument { .. } => {
                ProductErrorKind::EvaluationOfAnotherDocument
            }
            Self::UnknownNode { .. } => ProductErrorKind::UnknownNode,
            Self::PlacedUnderTwoRoots { .. } => ProductErrorKind::PlacedUnderTwoRoots,
            Self::Naming { .. } => ProductErrorKind::Naming,
            Self::RootFailed { .. } => ProductErrorKind::RootFailed,
            Self::RootPoisoned { .. } => ProductErrorKind::RootPoisoned,
            Self::NoBodyRoots => ProductErrorKind::NoBodyRoots,
            Self::Graft { .. } => ProductErrorKind::Graft,
            Self::SolidInvalid { .. } => ProductErrorKind::SolidInvalid,
            Self::ProductInvalid { .. } => ProductErrorKind::ProductInvalid,
            Self::ContactLineage { .. } => ProductErrorKind::ContactLineage,
        }
    }
}

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
///
/// A [`crate::Node::Part`] is a `Body` value and contributes exactly
/// the body it selected. The half or instance it did NOT select is in
/// no product through it: a split or a pattern consumed by a Part is
/// no longer a sink, so it is no longer a root, and the product of a
/// document whose only root is a `Part(Above)` is that one half.
pub(crate) fn sources_of<T: Decide>(value: &NodeValue<T>) -> Option<Vec<Source0<T>>> {
    let carried = || Arc::clone(&value.contacts);
    let none = || Arc::new(ContactRecords::default());
    // The DECLARATION rows ride the value's own channel, filled at the
    // instantiate op alone. They are keyed in the same arena as
    // `value.contacts`, so they travel with exactly the arm that
    // travels those records — and nowhere else, because a body 0 that
    // is not the instantiated one has no declaration to carry.
    let rows = || Arc::clone(&value.carried);
    let norows = || Arc::new(crate::assembly::CarriedDeclarations::default());
    match &value.payload {
        ValuePayload::Body(body) => Some(vec![(0, Arc::clone(body), carried(), rows())]),
        ValuePayload::Boolean(BooleanValue::Body { body, contacts, .. }) => {
            Some(vec![(0, Arc::clone(body), Arc::clone(contacts), norows())])
        }
        ValuePayload::Boolean(BooleanValue::Empty) => Some(Vec::new()),
        // Multi-output ops carry no records (the `OpOut` invariant):
        // "output body 0" names nothing here, so there is no home to
        // read from and none is invented.
        //
        // Cannot refuse: an instance list is minted by the pattern op,
        // which refuses any index `names::output_body` does not fit, or
        // by `Placeable::map`, which rebuilds an existing list one body
        // for one.
        ValuePayload::Instances(bodies) => Some(
            bodies
                .iter()
                .enumerate()
                .map(|(i, body)| {
                    let ix = crate::names::output_body(i).unwrap_or_else(|e| {
                        unreachable!("instance {i} outlived its minting op's index refusal: {e}")
                    });
                    (ix, Arc::clone(body), none(), norows())
                })
                .collect(),
        ),
        // Split's halves are output bodies by `SplitHalf::output_body`
        // (the one definition of that mapping); an EMPTY half
        // contributes nothing but does not shift the other half's
        // index — the index is the value's layout, not a position in
        // this list.
        ValuePayload::Split { above, below } => Some(
            [(SplitHalf::Above, above), (SplitHalf::Below, below)]
                .into_iter()
                .filter_map(|(half, side)| match side {
                    SplitSide::Body(body) => {
                        Some((half.output_body(), Arc::clone(body), none(), norows()))
                    }
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
        | ValuePayload::MeasureUnavailable { .. }
        | ValuePayload::Assertion(_) => None,
    }
}

// Gathers this thread has performed, the debug-only witness of the
// one-gather-per-landing invariant. Thread-local rather than global: a
// witness two tests running in one process can both read is a witness
// neither can trust.
#[cfg(debug_assertions)]
thread_local! {
    static GATHERS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// How many times [`product_recorded`] has run on THIS thread — every
/// call, refusals included, since a refused gather is still a gather
/// paid for.
///
/// `cfg(debug_assertions)`-gated, the shape `topo::source`'s bit
/// witnesses use. **That is not the same as "absent from a release
/// build" here**: this workspace's `[profile.release]` sets
/// `debug-assertions = true` deliberately (and says so, and says it
/// comes out before publish), so every build this repo produces today
/// carries the counter and the increment. What the gate buys is that
/// cargo's OWN release defaults strip both, which is what a consumer
/// building this crate normally gets — and that the day the stanza
/// comes out, nothing here has to change.
///
/// It counts, and a caller reads a DIFFERENCE across the operation it
/// is asking about; the absolute value means nothing.
#[cfg(debug_assertions)]
#[must_use]
pub fn gathers_on_this_thread() -> u64 {
    GATHERS.with(std::cell::Cell::get)
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
/// Every arm of [`ProductError`]: an evaluation of another document
/// ([`ProductError::EvaluationOfAnotherDocument`]); a root that
/// failed, was poisoned, or is absent from this evaluation; a document
/// whose roots denote no body ([`ProductError::NoBodyRoots`]); the
/// kernel's graft and at-rest validity refusals.
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
    /// The document this product is OF (DI3).
    ///
    /// A gather is a statement about one document, and the doors that
    /// take a gathered product rather than gathering for themselves
    /// have no other way to check that it is the one they were asked
    /// about — `crate::run_checks_on` refuses a product from another
    /// document exactly as this gather refuses a foreign evaluation.
    /// Written from `doc.id()` after the pairing door below, so it is
    /// the identity BOTH arguments agreed on.
    pub document: crate::ident::DocumentId,
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
    /// One row per mate of THIS document whose declaration the gather
    /// minted into `contacts`, in document order.
    ///
    /// The rows are the attribution channel and nothing else: the
    /// records themselves carry arena keys, so a finding against one
    /// needs this list to say which mate authored it. A declaration
    /// that arrived from a sub-assembly has no row HERE — its mate
    /// belongs to another document, so it rides `carried` below, which
    /// keeps that document and the route with it.
    pub minted: Vec<crate::assembly::MintedDeclaration>,
    /// One row per live mate the gather could NOT mint, in document
    /// order ([`crate::MintRefusal`]).
    ///
    /// Recorded rather than refused: neither refusal changes what
    /// material the document denotes, so the product stands and
    /// [`crate::assemble`] is the door that turns the first row into
    /// its typed refusal.
    pub unminted: Vec<crate::assembly::MintRefusal>,
    /// One row per declaration a document BELOW this one minted,
    /// re-keyed onto the aggregate through the graft's descendant map
    /// and tagged with the route it arrived by
    /// ([`crate::CarriedDeclaration`]).
    ///
    /// The records themselves already crossed the seam on `contacts`;
    /// these are the rows that say WHOSE mate authored each of them,
    /// so a finding against a carried record names a mate and a file
    /// instead of nobody. A sub-assembly's own carried rows carry up
    /// again, their route extended.
    pub carried: Vec<crate::assembly::CarriedDeclaration>,
    /// One row per mate a document below this one could NOT mint
    /// ([`crate::CarriedRefusal`]), carried verbatim: inner mint
    /// health is what the outermost gate refuses over, and the gather
    /// is where it arrives.
    pub carried_unminted: Vec<crate::assembly::CarriedRefusal>,
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
/// Every arm of [`ProductError`], including
/// [`ProductError::EvaluationOfAnotherDocument`] when `evaluation` is
/// not an evaluation of `doc`; [`ProductError::PlacedUnderTwoRoots`],
/// from the recipe before any root is read, when two roots place one
/// body through transforms and part selections; and
/// [`ProductError::Naming`] when two roots' rows would still name one
/// aggregate entity or collide on one name — never resolved silently.
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
/// [`ProductError::EvaluationOfAnotherDocument`] — `evaluation` must
/// be an evaluation OF `doc`, and this is the door all three read it
/// through — and [`ProductError::ContactLineage`] when the graft's
/// bridge has no image for a record's entity.
pub fn product_recorded<P, T: Decide + AtRestPolicy>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    tol: Tol,
) -> Result<Product<T>, ProductError> {
    #[cfg(debug_assertions)]
    GATHERS.with(|gathers| gathers.set(gathers.get().saturating_add(1)));
    // The pairing door (DI3), before the first root is read: this
    // gather is a statement about `doc`, and an evaluation of another
    // document answers about other geometry — silently, whenever the
    // two documents' node ids overlap, which two documents built from
    // one recipe always do.
    if let Some(m) = crate::ident::mispaired(doc.id(), evaluation.document) {
        return Err(m.into());
    }
    // The recipe's own refusal, before any value is read: a body two
    // roots both place is a fact of the DAG, and no evaluation makes
    // it representable.
    if let Some(err) = placed_under_two_roots(doc) {
        return Err(err);
    }
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
        sources.extend(bodies.into_iter().map(|(ix, body, contacts, rows)| {
            (
                node,
                ix,
                body,
                Arc::clone(&value.name_table),
                contacts,
                rows,
            )
        }));
    }
    if !any_body_denoting {
        return Err(ProductError::NoBodyRoots);
    }

    // Pass 2: the per-source gate, asked only when the product holds
    // more than one solid (this module's F8/D7 shape). The count is
    // over SOLIDS, not sources: one source may itself carry several
    // (an instantiated sub-assembly), and it is the product's solid
    // count the rule speaks about.
    let total_solids: usize = sources
        .iter()
        .map(|(_, _, b, _, _, _)| b.solids().count())
        .sum();
    if total_solids > 1 {
        for (node, _, body, _, _, _) in &sources {
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
    let mut carried: Vec<crate::assembly::CarriedDeclaration> = Vec::new();
    let mut carried_unminted: Vec<crate::assembly::CarriedRefusal> = Vec::new();
    let mut tie_rows = CarriedRows::default();
    for (node, ix, body, table, records, rows) in &sources {
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
        carry_names(&mut names, &mut tie_rows, table, *node, *ix, &keys)?;
        carry_contacts(&mut contacts, records, &keys)
            .map_err(|what| ProductError::ContactLineage { node: *node, what })?;
        carry_declarations(&mut carried, &rows.minted, &keys)
            .map_err(|what| ProductError::ContactLineage { node: *node, what })?;
        // A refusal names no entity — it is a mate that produced NO
        // record — so it carries with nothing to re-key.
        carried_unminted.extend(rows.unminted.iter().cloned());
    }
    // The name carry's second half, after the last source: every
    // tie-descended row, narrowed ONCE over all of them (`carry_names`).
    // A tie whose candidates the document separated into different
    // SOURCES is one tie of the product — the product holds both faces
    // — and this is where that is decided, because no single source
    // can see it.
    //
    // A refusal here names the node that MINTED the colliding name
    // rather than a root: the collision is between rows that arrived
    // from different sources, so no one root is its author.
    tie_rows
        .finish(&mut names)
        .map_err(|e| ProductError::Naming {
            node: e.name.node,
            name: e.name,
        })?;
    T::gate_at_rest(&aggregate, tol).map_err(|errors| ProductError::ProductInvalid { errors })?;
    // Pass 4: MINTING (A3's "Declaration minting"). Every evaluated
    // product carries its own mates' declarations, so what a document
    // MEANS includes what its mates say about the material — which is
    // what lets the instantiation seam compose: a sub-assembly's
    // declarations ride up on the contacts channel above, keyed by the
    // graft, rather than being lost because the consuming document
    // never ran the mate loop.
    //
    // Last, and after the gate: minting resolves references against
    // the FINISHED name table, and the aggregate's own at-rest verdict
    // is about geometry, so a product that is not a body at all
    // refuses before any mate is read.
    let (minted, unminted) = crate::assembly::mint(doc, evaluation, &names, &mut contacts);
    Ok(Product {
        document: doc.id(),
        body: aggregate,
        names,
        contacts,
        solid_roots,
        minted,
        unminted,
        carried,
        carried_unminted,
    })
}

/// **The first body two roots both place**, in root-list order, as
/// [`ProductError::PlacedUnderTwoRoots`]; `None` when every root places
/// bodies no other root places.
///
/// A root's chain is the run of nodes it reaches through the two
/// verbatim-naming edges a recipe can decide on: a
/// [`crate::Node::Transform`]'s input, which it places whole, and a
/// [`crate::Node::Part`]'s `of`, which it narrows to one selection.
/// The selection in effect rides down through the transforms below a
/// part, since a transform of an `Instances` value keeps its instance
/// order. Any other node ends the chain: every other op re-mints what
/// it carries (N1) or, for a split's intact entities, carries a subset
/// only its geometry decides.
///
/// Neither edge mints a name (N1), so two chains that meet at one node
/// with overlapping selections — either whole, or the same selection —
/// both carry that node's names verbatim, and the product would hold
/// two entities under each of them. The node reported is the one nearest the later root, which
/// is the one nearest both: below a meeting point the two chains are
/// one chain.
///
/// Selections are compared as written. Two `Instance` selections whose
/// expressions differ but evaluate to one index are not seen here and
/// refuse later, as [`ProductError::Naming`].
fn placed_under_two_roots<P>(doc: &Doc<P>) -> Option<ProductError> {
    use crate::node::{Node, PartSelect};
    let overlaps = |a: Option<&PartSelect>, b: Option<&PartSelect>| match (a, b) {
        (Some(a), Some(b)) => a == b,
        _ => true,
    };
    let mut seen: std::collections::HashMap<
        RecipeNodeId,
        Vec<(RecipeNodeId, Option<&PartSelect>)>,
    > = std::collections::HashMap::new();
    for &root in doc.roots() {
        let mut at = root;
        let mut select: Option<&PartSelect> = None;
        loop {
            let (next, narrowed) = match doc.node(at) {
                Some(Node::Transform { input, .. }) => (*input, select),
                Some(Node::Part { of, select: s }) => (*of, Some(s)),
                _ => break,
            };
            if let Some(&(first, earlier)) = seen
                .get(&next)
                .and_then(|rows| rows.iter().find(|(_, s)| overlaps(*s, narrowed)))
            {
                return Some(ProductError::PlacedUnderTwoRoots {
                    placed: next,
                    select: earlier.and(narrowed).cloned(),
                    first,
                    second: root,
                });
            }
            seen.entry(next).or_default().push((root, narrowed));
            at = next;
            select = narrowed;
        }
    }
    None
}

/// One body the gather will graft: which root contributed it, which
/// OUTPUT-BODY index it occupies in that root's value (the index its
/// name rows are keyed by), the body, the root's name table, the
/// body's declared contact records, and the declaration rows that say
/// whose mate authored each carried record.
type Source<T> = (
    RecipeNodeId,
    u32,
    Arc<Body<T>>,
    Arc<NameTable>,
    Arc<ContactRecords>,
    Arc<crate::assembly::CarriedDeclarations>,
);

/// One body-denoting source as [`sources_of`] hands it back: output
/// index, body, the records keyed in that body's arena, and the
/// declaration rows keyed in the same arena.
pub(crate) type Source0<T> = (
    u32,
    Arc<Body<T>>,
    Arc<ContactRecords>,
    Arc<crate::assembly::CarriedDeclarations>,
);

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

/// Re-keys one grafted body's carried DECLARATION rows onto the
/// aggregate — [`carry_contacts`]'s twin, under its lineage rule and
/// its refusal, for the bookkeeping that says whose mate authored each
/// record. Dropping a row here would leave its RECORD in the set with
/// nothing to attribute it to, which is the anonymity this channel
/// exists to end.
fn carry_declarations(
    into: &mut Vec<crate::assembly::CarriedDeclaration>,
    from: &[crate::assembly::CarriedDeclaration],
    keys: &topo::GraftKeys,
) -> Result<(), &'static str> {
    for row in from {
        let (a, b) = row.declaration.faces;
        into.push(crate::assembly::CarriedDeclaration {
            declaration: crate::assembly::MintedDeclaration {
                faces: (keys.face(a).ok_or("face")?, keys.face(b).ok_or("face")?),
                ..row.declaration.clone()
            },
            ..row.clone()
        });
    }
    Ok(())
}

/// Re-keys one grafted body's name rows onto the aggregate: the same
/// stable names, pointing at the entities the graft minted. Body index
/// on the product side is 0 — the product is ONE body.
///
/// The KEY MAP is this function's own — it is the graft's descendant
/// map, plus the product's rule that a root body-row does not carry
/// (see `product_named`: the product's own body is nobody's root
/// body). WHICH ROWS go in strict and which are deferred is not: that
/// is [`crate::names::CarriedRows`], the accumulate-then-narrow
/// mechanism the emitters share, so the aggregate table narrows a tie
/// by the one rule every other table narrows by.
fn carry_names(
    into: &mut NameTable,
    rows: &mut CarriedRows,
    from: &NameTable,
    node: RecipeNodeId,
    ix: u32,
    keys: &topo::GraftKeys,
) -> Result<(), ProductError> {
    rows.carry(into, from, ix, |key| match key {
        EntityKey::Body => None,
        EntityKey::Face(f) => keys.face(f).map(EntityKey::Face),
        EntityKey::Edge(e) => keys.edge(e).map(EntityKey::Edge),
        EntityKey::Vertex(v) => keys.vertex(v).map(EntityKey::Vertex),
    })
    .map_err(|e| ProductError::Naming { node, name: e.name })
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::{ProductError, ProductErrorKind};
    use crate::names::{EntityKind, StableName};
    use crate::node::RecipeNodeId;

    /// One error per [`ProductError`] arm — a CENSUS, not a sample:
    /// every payload this error carries is constructible from here
    /// (keys, ids, `&'static str`, empty finding lists, and one unit
    /// arm of the kernel's own refusal), so no arm is left unbuilt.
    fn every_arm() -> Vec<ProductError> {
        let node = RecipeNodeId(3);
        vec![
            ProductError::EvaluationOfAnotherDocument {
                expected: crate::ident::DocumentId::derive("expected"),
                found: crate::ident::DocumentId::derive("found"),
            },
            ProductError::UnknownNode { node },
            ProductError::PlacedUnderTwoRoots {
                placed: RecipeNodeId(1),
                select: None,
                first: node,
                second: RecipeNodeId(4),
            },
            ProductError::Naming {
                node,
                name: Box::new(StableName {
                    kind: EntityKind::Face,
                    node: RecipeNodeId(1),
                    path: Vec::new(),
                }),
            },
            ProductError::RootFailed { node },
            ProductError::RootPoisoned {
                node,
                through: RecipeNodeId(1),
            },
            ProductError::NoBodyRoots,
            ProductError::Graft {
                node,
                source: Box::new(topo::BooleanError::UnrepresentableResult),
            },
            ProductError::SolidInvalid {
                node,
                errors: Vec::new(),
            },
            ProductError::ProductInvalid { errors: Vec::new() },
            ProductError::ContactLineage { node, what: "face" },
        ]
    }

    /// **The phantom direction, closed by the compiler; the pairing
    /// direction, closed by construction.**
    ///
    /// [`ProductError::kind`] is exhaustive over the ERROR, so an arm
    /// added there reds this crate. `label` below is exhaustive over
    /// the KIND, so a variant added to [`ProductErrorKind`] alone reds
    /// HERE, by name, in the crate that owns both — rather than in
    /// whatever downstream crate next maps the enum.
    ///
    /// Neither exhaustiveness objects to an arm PROJECTED to the wrong
    /// kind, which type-checks. That is what the errors below are for:
    /// each is built, projected, and its kind's name compared with the
    /// variant name `Debug` prints for the error itself, so a
    /// mis-projected arm and a mis-labelled arm both fail here with no
    /// expected value written down twice.
    ///
    /// The census is complete as measured: every arm of
    /// [`ProductError`] is built here, so no arm's projection is
    /// unchecked today. What no guard closes is an arm added to the
    /// error LATER, projected onto an existing kind and left out of
    /// [`every_arm`] — neither exhaustiveness reds on that, and this
    /// row accuses no author of anything it has not measured. The
    /// distinctness assertion below is what makes the collision half
    /// of it visible whenever the new arm IS built here.
    #[test]
    fn each_kind_has_an_arm_and_each_built_arm_projects_to_its_own_kind() {
        fn label(kind: ProductErrorKind) -> &'static str {
            match kind {
                ProductErrorKind::EvaluationOfAnotherDocument => "EvaluationOfAnotherDocument",
                ProductErrorKind::UnknownNode => "UnknownNode",
                ProductErrorKind::PlacedUnderTwoRoots => "PlacedUnderTwoRoots",
                ProductErrorKind::Naming => "Naming",
                ProductErrorKind::RootFailed => "RootFailed",
                ProductErrorKind::RootPoisoned => "RootPoisoned",
                ProductErrorKind::NoBodyRoots => "NoBodyRoots",
                ProductErrorKind::Graft => "Graft",
                ProductErrorKind::SolidInvalid => "SolidInvalid",
                ProductErrorKind::ProductInvalid => "ProductInvalid",
                ProductErrorKind::ContactLineage => "ContactLineage",
            }
        }
        /// The variant name `Debug` opens with.
        fn variant_of(err: &ProductError) -> String {
            format!("{err:?}")
                .chars()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect()
        }
        let built = every_arm();
        let mut seen: Vec<ProductErrorKind> = Vec::new();
        for err in &built {
            assert_eq!(
                label(err.kind()),
                variant_of(err),
                "kind() projects each arm to its own kind, and label names it"
            );
            assert!(
                !seen.contains(&err.kind()),
                "two arms project to {:?}",
                err.kind()
            );
            seen.push(err.kind());
        }
    }

    /// **What this reads is the SET, and the set is what a bug moves.**
    ///
    /// [`ProductErrorKind::means_no_body`] is exhaustive over the KIND,
    /// so its answer for any one class is fixed the moment it compiles
    /// and asserting that answer alone names nothing. What is not fixed
    /// is which of the arms [`every_arm`] builds answer `true`. That
    /// set moves when an existing class is re-classified, and when
    /// [`ProductError::kind`] re-projects a built arm onto a class
    /// carrying the other answer — both red here, naming the arms that
    /// came back.
    ///
    /// **It is a floor, not a bijection, and the gap is the census's
    /// own.** A [`ProductError`] arm added under an EXISTING kind and
    /// left out of [`every_arm`] would read as no-body with nothing
    /// red anywhere: not here, because the roster never sees it; not at
    /// the predicate, which is exhaustive over the kind and not over
    /// the error; and not at [`ProductError::kind`], where projecting a
    /// new arm onto an existing kind compiles. That is exactly the hole
    /// `each_kind_has_an_arm_and_each_built_arm_projects_to_its_own_kind`
    /// states above, and no guard closes it: the roster is hand-written
    /// because stable Rust cannot enumerate an enum's variants, so a
    /// bijection is not available to be asserted. This row states what
    /// it has rather than what would be better.
    #[test]
    fn exactly_one_arm_reads_as_no_body() {
        let reads_no_body: Vec<String> = every_arm()
            .iter()
            .filter(|err| err.kind().means_no_body())
            .map(|err| format!("{err:?}"))
            .collect();
        assert_eq!(
            reads_no_body,
            vec![format!("{:?}", ProductError::NoBodyRoots)],
            "no root denoting a body is the only gather refusal that is \
             an absence rather than a fault"
        );
    }

    /// **The refusal calls a node a ROOT only when it is one.**
    /// [`ProductError::Naming`]'s `node` is the carried root on the
    /// per-source path and the MINTING node on the tie merge's, where
    /// no one root authored the collision — so the rendering is
    /// guarded, and this is the guard's other side. The two renderings
    /// are asserted apart by the word the second must not use and by
    /// the id the first must not print twice.
    #[test]
    fn the_naming_refusal_claims_rootedness_only_on_the_per_root_path() {
        let named = |node: u64, minted: u64| {
            ProductError::Naming {
                node: RecipeNodeId(node),
                name: Box::new(StableName {
                    kind: EntityKind::Face,
                    node: RecipeNodeId(minted),
                    path: Vec::new(),
                }),
            }
            .to_string()
        };
        let carried = named(8, 6);
        assert!(
            carried.contains("root 8") && carried.contains("node 6"),
            "the per-root path names the root that carried and the node that minted: {carried}"
        );
        let merged = named(6, 6);
        assert!(
            !merged.contains("root"),
            "the tie merge's collision has no one root to name, and must not invent one: {merged}"
        );
        assert!(
            merged.contains("node 6"),
            "it still names the node that minted the colliding name: {merged}"
        );
    }
}
