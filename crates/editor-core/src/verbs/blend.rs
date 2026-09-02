//! **The blend pair's correspondence** — one module for two verbs.
//!
//! # Why one module and not two
//!
//! Their sharing is total. `Node::Fillet` and `Node::Chamfer` have the
//! same shape (a target input, a frozen selection, one size slot), they
//! resolve their selections through the same ladder, they run through
//! the same battery and the same surgery, and they come back carrying
//! the same birth record. Everything that distinguishes them is a
//! literal: which slot holds the size, which label a selection refusal
//! carries, which verb the kernel is asked for, and what a missing
//! birth record is called in the refusal. Two modules here would be two
//! copies of one declaration differing in four constants — the
//! near-parallel twin the blend doors themselves already refused to be
//! (`sweep::blend::BlendRefusal`'s type docs).
//!
//! # The emitter field, stated honestly
//!
//! [`BlendVerb::emitter`] selects between `names::name_fillet` and
//! `names::name_chamfer`, and those two functions have IDENTICAL
//! bodies: each is a thin door that forwards every argument to
//! `names::emit_blend::name_blend`, which is the one implementation.
//! So the choice made here changes nothing about the names that come
//! out. What discriminates a fillet's names from a chamfer's is the
//! minting node id the lowering passes in — the same role vocabulary,
//! stamped under a different node (RECIPE-DOORS D3).
//!
//! The field is carried anyway, and not as decoration: it is the slot
//! where a verb whose emitter is NOT a blend door plugs in, and
//! collapsing it now would mean re-deriving it at the first verb whose
//! naming differs. Read it as "this verb's emitter", not as "the reason
//! these two differ".

use std::sync::Arc;

use sweep::blend::BlendKind;
use sweep::blend::naming::BlendNaming;
use topo::{Body, EdgeKey};
use verbs::{Verb, VerbKind};

use crate::names::{self, NameTable, NamingError};
use crate::node::{RecipeNodeId, SlotId};

/// A verb's naming emitter: this node's id, its operand's id and table,
/// the result body and its birth record, in.
pub(crate) type Emitter<T> = fn(
    RecipeNodeId,
    RecipeNodeId,
    &NameTable,
    &Body<T>,
    &BlendNaming,
) -> Result<Arc<NameTable>, NamingError>;

/// **One blend verb's correspondence**, as data.
///
/// Everything the generic lowering needs to turn a `Node` into a
/// [`Verb`] and its result into a name table. Adding a field here is
/// how a verb declares something the lowering must know; adding an
/// arm to a match inside the lowering is not.
pub(crate) struct BlendVerb {
    /// Which kernel verb the node lowers to. This is the canonical
    /// name, and the content tag is a function of it
    /// ([`crate::eval::verb_content_tag`]).
    pub(crate) kind: VerbKind,
    /// The label a SELECTION refusal carries. It is the kernel's blend
    /// door label rather than [`Self::kind`] because the refusal it
    /// lands in is shared with the kernel's own
    /// (`NodeErrorKind::Blend`), and one vocabulary there is what keeps
    /// a refusal's verb from being rendered twice or differently.
    pub(crate) selection_label: BlendKind,
    /// The slot whose evaluated scalar is the verb's size parameter:
    /// the fillet's radius, the chamfer's setback.
    pub(crate) size_slot: SlotId,
    /// What a missing birth record is called when this verb's result
    /// arrives without one. A kernel bug either way; the sentence names
    /// the door that produced it.
    pub(crate) no_records: &'static str,
}

/// The fillet's correspondence.
pub(crate) const FILLET: BlendVerb = BlendVerb {
    kind: VerbKind::Fillet,
    selection_label: BlendKind::Fillet,
    size_slot: SlotId::Radius,
    no_records: "the fillet returned a body with no birth records",
};

/// The chamfer's correspondence.
pub(crate) const CHAMFER: BlendVerb = BlendVerb {
    kind: VerbKind::Chamfer,
    selection_label: BlendKind::Chamfer,
    size_slot: SlotId::ChamferDistance,
    no_records: "the chamfer returned a body with no birth records",
};

impl BlendVerb {
    /// **Slot value + resolved selection → the kernel verb.** The one
    /// place a document's evaluated size and canonical edge keys become
    /// a [`Verb`] payload.
    pub(crate) fn verb<T>(&self, edges: Vec<EdgeKey>, size: T) -> Verb<T> {
        match self.kind {
            VerbKind::Fillet => Verb::Fillet {
                edges,
                radius: size,
            },
            VerbKind::Chamfer => Verb::Chamfer {
                edges,
                distance: size,
            },
        }
    }

    /// This verb's naming emitter — see the module docs on what this
    /// choice does and does not decide.
    pub(crate) fn emitter<T: geom_core::Real>(&self) -> Emitter<T> {
        match self.kind {
            VerbKind::Fillet => names::name_fillet,
            VerbKind::Chamfer => names::name_chamfer,
        }
    }
}
