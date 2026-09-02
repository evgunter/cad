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
use verbs::Verb;
#[cfg(test)]
use verbs::VerbKind;

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
///
/// # Every field is DIRECT per-instance data, deliberately
///
/// The first shape of this struct carried a `kind: VerbKind` and read
/// the verb constructor and the emitter off it through two internal
/// matches. That was a wart with a measurable cost: `kind` was typed at
/// the WHOLE vocabulary while only two of its variants were ever legal
/// here, so adding an unrelated `VerbKind::Shell` broke both matches
/// inside this blend-only module and made a shell's author edit the
/// blends' correspondence to say nothing about shells. A reviewer found
/// it by adding that variant.
///
/// The constructor and the emitter are therefore FUNCTION POINTERS held
/// per instance. Nothing in this module matches on a verb vocabulary
/// any more, which is what makes it true that a future verb never has
/// to open this file.
pub(crate) struct BlendVerb<T: geom_core::Real> {
    /// **Slot value + resolved selection → the kernel verb.** The one
    /// place a document's evaluated size and canonical edge keys become
    /// a [`Verb`] payload, per instance.
    pub(crate) build: fn(Vec<EdgeKey>, T) -> Verb<T>,
    /// This verb's naming emitter — see the module docs on what this
    /// choice does and does not decide.
    pub(crate) emitter: Emitter<T>,
    /// The label a SELECTION refusal carries. It is the kernel's blend
    /// door label rather than a `verbs::VerbKind` because the refusal it
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

/// The fillet's kernel payload. A named function rather than a closure
/// so it can be a plain `fn` pointer in the struct above.
fn build_fillet<T>(edges: Vec<EdgeKey>, radius: T) -> Verb<T> {
    Verb::Fillet { edges, radius }
}

/// The chamfer's kernel payload.
fn build_chamfer<T>(edges: Vec<EdgeKey>, distance: T) -> Verb<T> {
    Verb::Chamfer { edges, distance }
}

/// The fillet's correspondence.
///
/// A function rather than a `const` because the struct is generic in
/// the lane scalar and a module-level const cannot be: the call is
/// monomorphized and inlined, so this costs nothing at run time.
pub(crate) fn fillet<T: geom_core::Real>() -> BlendVerb<T> {
    BlendVerb {
        build: build_fillet,
        emitter: names::name_fillet,
        selection_label: BlendKind::Fillet,
        size_slot: SlotId::Radius,
        no_records: "the fillet returned a body with no birth records",
    }
}

/// The chamfer's correspondence.
pub(crate) fn chamfer<T: geom_core::Real>() -> BlendVerb<T> {
    BlendVerb {
        build: build_chamfer,
        emitter: names::name_chamfer,
        selection_label: BlendKind::Chamfer,
        size_slot: SlotId::ChamferDistance,
        no_records: "the chamfer returned a body with no birth records",
    }
}

/// **The canonical verb name of each blend node**, which is NOT a field
/// above.
///
/// The vocabulary entry a blend node lowers to is read by exactly one
/// consumer — the content tag beside the memo machinery
/// (`verb_content_tag` in [`mod@crate::eval`]) — and it is read there
/// from the `Node` variant directly. Carrying a copy here would be a
/// second spelling of the same correspondence with nothing forcing the
/// two to agree, so this module deliberately does not hold one; the
/// tests below pin that the pair this module builds is the pair those
/// tags name.
#[cfg(test)]
pub(crate) const BLEND_VERB_KINDS: [VerbKind; 2] = [VerbKind::Fillet, VerbKind::Chamfer];

#[cfg(test)]
mod tests {
    use super::*;

    /// The two correspondences build the two verbs their names claim —
    /// the check the deleted `kind` match used to make structurally.
    #[test]
    fn each_correspondence_builds_its_own_verb() {
        let f: Verb<f64> = (fillet::<f64>().build)(Vec::new(), 1.0);
        let c: Verb<f64> = (chamfer::<f64>().build)(Vec::new(), 1.0);
        assert_eq!(f.kind(), VerbKind::Fillet, "the fillet built a {f:?}");
        assert_eq!(c.kind(), VerbKind::Chamfer, "the chamfer built a {c:?}");
        assert_eq!(
            [f.kind(), c.kind()],
            BLEND_VERB_KINDS,
            "the blend pair is no longer the pair this module claims"
        );
    }

    /// The two differ in every literal a reader would expect them to,
    /// so a copy-paste that left one behind fails here.
    #[test]
    fn the_two_correspondences_share_no_literal() {
        let f = fillet::<f64>();
        let c = chamfer::<f64>();
        assert_ne!(f.size_slot, c.size_slot, "both verbs read one slot");
        assert_ne!(
            f.selection_label, c.selection_label,
            "both verbs label refusals identically"
        );
        assert_ne!(f.no_records, c.no_records, "both verbs share one sentence");
    }
}
