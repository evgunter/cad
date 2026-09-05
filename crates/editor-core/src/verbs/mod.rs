//! **The per-verb correspondence**: what connects a document node to
//! the kernel verb it invokes.
//!
//! The recipe layer keeps the authoring vocabulary — an [`Expr`] per
//! slot, a frozen canonical selection of [`StableName`]s, node ids for
//! inputs — because that IS the document's semantics: what an
//! expression is, what a frozen reference is, how resolution refuses.
//! None of that is a restatement of the kernel.
//!
//! What was scattered is the CORRESPONDENCE between the two
//! vocabularies, and this module is where it now lives, one declaration
//! per verb: which [`SlotId`] feeds which verb parameter, which payload
//! selection feeds the key list, which emitter mints the names, which
//! arm of the record channel a family's result arrives in. The
//! lowering in [`mod@crate::eval`] is generic over it — one body of
//! code per declared door (`wire_blend` for the one-body verbs,
//! `wire_boolean` for the pair family, `wire_swept` for the profile
//! family, `wire_split` for the two-sided split), each driven by the
//! declarations here rather than matching a verb vocabulary of its
//! own.
//!
//! [`Expr`]: crate::expr::Expr
//! [`StableName`]: crate::names::StableName
//! [`SlotId`]: crate::node::SlotId

pub(crate) mod blend;
pub(crate) mod boolean;
pub(crate) mod split;
pub(crate) mod sweep;

use geom_core::Real;
use verbs::VerbRecord;

use crate::eval::NodeErrorKind;
use crate::names::NamingError;

/// **The one rule for taking a family's record out of the closed
/// channel**: apply the correspondence's own projection — an
/// exhaustive match over [`VerbRecord`] that answers `Some` for its
/// family's arm and `None` for every other, so a record family added
/// to the channel breaks every projection at compile time (D3) — and
/// refuse a foreign family typed, in the correspondence's own words.
///
/// A wrong-family record is a kernel bug: which variant a verb's run
/// produces is fixed by its family, so this refusal is unreachable
/// while the doors and the correspondences agree. It is refused, not
/// panicked, and the sentence names the door so the refusal does
/// too. Every lowering reads its record through here and nowhere
/// else; the rule was once written inline at each consumer and the
/// copies had begun to drift in their comments before their code.
pub(crate) fn read_record<T: Real, R>(
    record: VerbRecord<T>,
    family: fn(VerbRecord<T>) -> Option<R>,
    foreign_record: &'static str,
) -> Result<R, NodeErrorKind> {
    family(record).ok_or(NodeErrorKind::Naming(NamingError::Emission {
        what: foreign_record,
    }))
}
