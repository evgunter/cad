//! **What a predicate decides, in words — the one lookup.**
//!
//! A predicate's static name is routing: it rides `Debug` and the K
//! stream, never a sentence. A sentence that reports a decision — an
//! escalation that could not call it, a flip between two runs — states
//! what was being decided instead, and reads the words here.
//!
//! The words live with each predicate's owner, beside the decision
//! they describe: the naming layer's (`names::decision_words`), this
//! crate's evaluation's (`eval::decision_words`), the boolean's
//! (`topo::decision_words`) and the sketch validator's
//! (`profile::decision_subject`). This function only consults them, so
//! a predicate is given words in one place, where it is decided.

/// The words for `predicate`, from whichever owner holds them; `None`
/// when no owner has words for it (the caller then states
/// `geom_core::UNNAMED_DECISION`).
pub(crate) fn words(predicate: &str) -> Option<&'static str> {
    crate::names::decision_words(predicate)
        .or_else(|| crate::eval::decision_words(predicate))
        .or_else(|| topo::decision_words(predicate))
        .or_else(|| profile::decision_subject(predicate))
}
