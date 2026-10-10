//! **The part store an assembly suite instantiates through** — the
//! `PartResolver` an `InstantiatePart` node reaches its document by
//! (ASM-2A D-3), plus the name an instantiated part's cap faces are
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
    CapEnd, DocRef, DocumentId, EntityKind, EvalOptions, PartResolver, ProfileDoc, RecipeNodeId,
    ResolveFailure, ResolveFault, RoleSeg, StableName, content_pin,
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
        let pin = content_pin(&doc, tol).expect("the pin computes");
        let id = doc.id();
        self.docs.insert(id, doc);
        DocRef { id, pin }
    }

    /// [`Self::insert`] for a part builder's `(document, body)`, with
    /// `body` placed in the part's world first (A10: the part's product
    /// is its world), answering the reference with the body [`in_part`]
    /// names the part's caps through.
    pub fn insert_part(
        &mut self,
        (doc, body): (ProfileDoc, RecipeNodeId),
        tol: Tol,
    ) -> (DocRef, RecipeNodeId) {
        let (doc, placement) = super::place(doc, body);
        record_placement(body, placement);
        (self.insert(doc, tol), body)
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

/// The placement [`PartStore::insert_part`] placed each body with, by
/// the body. A node id is the digest of the mint chain it was minted
/// at, and the placement is inserted right after its body, so one body
/// id names one placement id in every document that holds it.
static PLACED: std::sync::Mutex<BTreeMap<RecipeNodeId, RecipeNodeId>> =
    std::sync::Mutex::new(BTreeMap::new());

fn record_placement(body: RecipeNodeId, placement: RecipeNodeId) {
    PLACED
        .lock()
        .expect("the placement record")
        .insert(body, placement);
}

/// **The world placement [`PartStore::insert_part`] placed `body`
/// with**: what the part's product names `body`'s entities under.
///
/// # Panics
///
/// If no part stored through [`PartStore::insert_part`] holds `body`.
pub fn placement_of(body: RecipeNodeId) -> RecipeNodeId {
    *PLACED
        .lock()
        .expect("the placement record")
        .get(&body)
        .unwrap_or_else(|| panic!("{body:?} is no body a part stored with `insert_part` placed"))
}

/// **A cap face of `instance`'s part product**, in the wrapper the
/// instantiate node mints: the part's own name for the face — a cap of
/// `body`, as its placement's copy carries it ([`RoleSeg::Placed`]) —
/// worn inside a [`RoleSeg::InPart`] under the
/// instance that placed it, the kernel's own wrapper
/// (`FaceName::in_part`).
pub fn in_part(instance: RecipeNodeId, body: RecipeNodeId, cap: CapEnd) -> StableName {
    editor_core::FaceName::new(in_world(body, cap))
        .expect("a cap is a face")
        .in_part(instance)
        .into_name()
}

/// **A cap face of a part's product**, as the part's own gather names
/// it: the body's cap under its placement's copy ([`placement_of`]).
pub fn in_world(body: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: placement_of(body),
        path: vec![RoleSeg::Placed {
            of: editor_core::NameRef::new(StableName {
                kind: EntityKind::Face,
                node: body,
                path: vec![RoleSeg::Cap(cap)],
            }),
        }],
    }
}
