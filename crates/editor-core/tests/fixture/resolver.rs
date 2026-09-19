//! **The part store an assembly suite instantiates through** — the
//! `PartResolver` an `InstantiatePart` node reaches its document by
//! (ASM-2A D-3), plus the two names an instantiated part's faces are
//! spelled with.
//!
//! Suites that instantiate parts each need a resolver, and a resolver
//! over an in-memory map is the same twenty lines every time. One
//! home for it means one place to fix if the trait's shape or the pin
//! rule moves, and one place a reader learns what an assembly suite's
//! substrate is.
//!
//! It is a REAL resolver, not a stub that answers anything: it
//! computes the pin of what it holds and refuses a mismatch with
//! [`ResolveFault::PinMismatch`], exactly as a store on disk does, so
//! a suite that hands out a stale `DocRef` gets the refusal the
//! shipped door would give it.

use std::collections::BTreeMap;

use editor_core::{
    CapEnd, DocRef, DocumentId, EntityKind, EvalOptions, Node, PartResolver, ProfileDoc,
    RecipeNodeId, ResolveFailure, ResolveFault, RoleSeg, StableName, content_pin,
};
use geom_core::Tol;

/// An in-memory part store.
#[derive(Debug, Default, Clone)]
pub struct PartStore {
    docs: BTreeMap<DocumentId, ProfileDoc>,
}

impl PartStore {
    /// An empty store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Takes `doc` in, answering the reference an `InstantiatePart`
    /// node names it by: its id, and the pin of the bytes stored.
    pub fn insert(&mut self, doc: ProfileDoc, tol: Tol) -> DocRef {
        assert_part_body(&doc);
        let pin = content_pin(&doc, tol).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }

    /// The document stored under `id`, as the store holds it.
    pub fn doc(&self, id: DocumentId) -> ProfileDoc {
        self.docs
            .get(&id)
            .expect("the store holds that document")
            .clone()
    }

    /// Writes `doc` over whatever `id` held, WITHOUT minting a new
    /// [`DocRef`] — so every reference taken from the earlier insert
    /// now names bytes whose pin has moved, and [`Self::resolve`]
    /// refuses it. This is how a suite stages a stale reference; the
    /// name says so, because the refusal is the point of the call and
    /// a plain map write does not read that way.
    pub fn replace_without_repinning(&mut self, id: DocumentId, doc: ProfileDoc) {
        self.docs.insert(id, doc);
    }
}

impl PartResolver for PartStore {
    fn resolve(&self, doc_ref: &DocRef, _tol: Tol) -> Result<ProfileDoc, ResolveFailure> {
        let fail = |fault, message: &str| ResolveFailure {
            fault,
            message: message.to_string(),
        };
        let doc = self
            .docs
            .get(&doc_ref.id)
            .ok_or_else(|| fail(ResolveFault::Unresolved, "no such document"))?;
        let found = content_pin(doc, Tol::witness()).expect("the pin computes");
        if found != doc_ref.pin {
            return Err(fail(ResolveFault::PinMismatch, "the pin does not hold"));
        }
        Ok(doc.clone())
    }
}

/// **The body node of a one-block part document** — frame, profile,
/// extrude, so the extrude is node 2. Each suite authors that shape
/// in its own builder, and [`in_part`] names a face of the minted
/// instance through it.
///
/// The premise is not restated in sixteen builders: [`PartStore::insert`]
/// checks it, through [`assert_part_body`], on every document that has
/// a body at all.
pub const PART_BODY: RecipeNodeId = RecipeNodeId(2);

/// **The coupling [`PART_BODY`] rests on, checked rather than
/// written down**: in a document that HAS an extrude, the extrude is
/// the node [`in_part`] names.
///
/// A suite that slips a datum in ahead of its sketch frame goes red
/// here, at the insert that authored it — not silently, five suites
/// later, as a member name that resolves to nothing. That is the
/// failure `mate1_r1_probes` and `mate6r1_shared` carried unseen:
/// their builders were right and their constant was not, and no
/// assertion in either suite could tell.
///
/// Documents with no extrude — the stands and rows an assembly suite
/// instantiates, whose nodes are `InstantiatePart` and mates — are
/// exempt, because `in_part` is not how their faces are named.
///
/// **Two edges the exemption leaves open.** Neither is reachable on
/// this tree, and both are here so a reader does not take this check
/// for more coverage than it has:
///
/// - It keys on "has an `Extrude`" as the proxy for "is a part
///   document". A part whose body is a `Revolve`, `Loft` or `Sweep`
///   takes the exempt branch, and `in_part` naming [`PART_BODY`] in it
///   is unchecked. `seat6_param_source`'s `filleted_lantern` already
///   builds a revolved body; it is not handed to a [`PartStore`], so
///   the hole is one suite away rather than open.
/// - It lives at [`PartStore::insert`], so it cannot see a suite that
///   uses [`in_part`] with NO store. That is
///   `rev_fix_xsplit_unreachable`, whose six call sites go zero rows
///   red under a planted [`PART_BODY`] —
///   `work/tint/mate6r1-shared-has-eleven-tests-and-no-assertions.md`
///   owns it.
///
/// # Panics
///
/// If `doc` has an extrude and `PART_BODY` is not one.
fn assert_part_body(doc: &ProfileDoc) {
    let is_extrude = |id: RecipeNodeId| matches!(doc.node(id), Some(Node::Extrude { .. }));
    if !doc.order().iter().copied().any(is_extrude) {
        return;
    }
    assert!(
        is_extrude(PART_BODY),
        "a part document's body is {PART_BODY:?} — this one's node order is {:?}, \
         so `in_part` would name a face of {:?}",
        doc.order(),
        doc.node(PART_BODY),
    );
}

/// **Evaluation options that reach parts through `store`** — the one
/// thing a suite changes about `EvalOptions` to instantiate at all.
///
/// **This function is now how the population is reached, and a sweep
/// keyed on `resolver: Some(` no longer finds it.** That grep does not
/// come back short, it comes back CONFIDENTLY WRONG: every hit left in
/// the tree is a deliberate non-member — `asm2a_instantiate`'s
/// seam-refusing store and its `CyclicStore`, `asm_upd_pin_update`'s
/// version shelf, `msolve3_placer_refused`'s shared-`Arc` harness, and
/// `pncad`'s real `Workspace` — so a lane sweeping that shape reads a
/// list of exceptions as the population. Grep for this name instead,
/// beside `impl .*PartResolver for`, and across `crates/*/tests` and
/// not one crate: the copy this unit closed had a seventeenth instance
/// in `viewer`, found only when the sweep was widened.
pub fn with_resolver(store: PartStore) -> EvalOptions {
    EvalOptions {
        resolver: Some(std::sync::Arc::new(store)),
        ..EvalOptions::default()
    }
}

/// **A cap face of `instance`'s part product**, in the wrapper the
/// instantiate node mints: the part's own name for the face, worn
/// inside a [`RoleSeg::InPart`] under the instance that placed it.
pub fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: StableName {
                kind: EntityKind::Face,
                node: PART_BODY,
                path: vec![RoleSeg::Cap(cap)],
            }
            .into(),
        }],
    }
}
