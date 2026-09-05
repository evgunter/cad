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
