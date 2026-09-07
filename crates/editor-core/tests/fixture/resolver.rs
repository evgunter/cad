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
    CapEnd, DocRef, DocumentId, EntityKind, PartResolver, ProfileDoc, RecipeNodeId, ResolveFailure,
    ResolveFault, RoleSeg, StableName, content_pin,
};
use geom_core::Tol;

/// An in-memory part store.
#[derive(Debug, Default)]
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
/// extrude, so the extrude is node 2. The part builders below all
/// author that shape, and a face of the minted instance is named
/// through it.
pub const PART_BODY: RecipeNodeId = RecipeNodeId(2);

/// **A cap face of `instance`'s part product**, in the wrapper the
/// instantiate node mints: the part's own name for the face, worn
/// inside a [`RoleSeg::InPart`] under the instance that placed it.
pub fn in_part(instance: RecipeNodeId, cap: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: instance,
        path: vec![RoleSeg::InPart {
            of: Box::new(StableName {
                kind: EntityKind::Face,
                node: PART_BODY,
                path: vec![RoleSeg::Cap(cap)],
            }),
        }],
    }
}
