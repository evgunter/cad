//! **The contradiction instrument** (ERROR-DESIGN E12, "the two channels
//! are checked where an answer is returned"): whether the tier's algebra
//! or a registration would have claimed `Zero` on a margin the numbers
//! decided definite.
//!
//! A definite numeric sign is the answer and the decision asks the tier
//! nothing about it, so a wrong rule, a wrong enclosure or a registrant
//! stating a false identity is invisible to a build wherever the
//! enclosure straddles nothing. This asks the question after the build,
//! over the decisions the re-valuation recorder kept
//! ([`revalue::record`](super::revalue::record)), and reports every one
//! the tier would have answered otherwise ([`Event`]). No receipt column
//! counts it and no shipped build contains it.
//!
//! **Installed by the test that wants it**: [`check`] walks only the
//! decisions it is handed, inside the session that took them (the nodes
//! live in that session's table), so a test build that compiles this
//! feature pays nothing in a row that does not call it. The walks it
//! asks leave the session's receipt as they found it.
//!
//! A [`Decide::free_identity`](crate::Decide::free_identity) margin is
//! left out: its number is a measured bound, not its expression's value,
//! so a definite sign beside a zero form is the door working.

use super::revalue::Decision;
use super::{Path, Rung, SESSION, SymId, ladder, reaches_free};
use crate::predicate::Sign;

/// **The most forms a retry attempt's memos may hold while the
/// instrument asks**, below the session's own cap: a definite margin is
/// rarely an identity and its forms are the large ones, so the retry
/// memos of a document's whole definite population outgrow memory at
/// the session's cap (R2's pad past 12 GB). An attempt whose memos are
/// at it is not offered (`ladder`'s growth guard), which costs the
/// instrument cancellations it would have found on the attempts' later
/// margins, never a report it should not make.
const RETRY_FORMS: usize = 20_000;

/// **What contradicted what.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
    /// The tier's algebra proves the margin zero where a CERTIFIED
    /// enclosure excludes zero (`Sym<Interval>`): one of the two channels
    /// is wrong.
    Enclosure,
    /// The tier's algebra proves the margin zero where a ROUNDED number
    /// read a definite sign (`Sym<f64>`): the theorem holds wherever the
    /// function is defined, and the reading is the arithmetic losing the
    /// margin at this scale, or a defect.
    Rounded,
    /// A zero only a registration reaches, under a definite sign: the
    /// axiom a constructor stated is false over this box.
    Registration,
}

/// One decision the tier would have answered `Zero`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Contradiction {
    /// Its place in the decisions handed to [`check`].
    pub index: usize,
    /// The funnel predicate it was taken under.
    pub predicate: &'static str,
    /// The margin's node.
    pub node: SymId,
    /// The sign the numbers decided.
    pub sign: Sign,
    /// What contradicted it.
    pub event: Event,
    /// The rung that proved the zero.
    pub rung: Rung,
    /// The attempt that proved it: `0` the first, `k` the `k`th retry.
    pub attempt: u8,
}

/// What [`check`] read.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// Definite decisions asked about.
    pub checked: usize,
    /// Of them, decisions the tier proves zero that are left out as
    /// `free_identity` margins.
    pub free: usize,
    /// Definite decisions whose node this session's table does not hold
    /// (taken in another session): nothing can be asked of them here.
    pub absent: usize,
    /// Every decision the tier would have answered `Zero`.
    pub contradictions: Vec<Contradiction>,
}

/// **The decisions in `decisions` the numbers decided definite, asked of
/// the tier**: every attempt of the algebra (the retry ladder included,
/// no door), then, where that declines, the registry door. Run inside
/// the session that took them; `None` outside a session or at a
/// zero-term budget, where the tier proves nothing.
#[must_use]
pub fn check(decisions: &[Decision]) -> Option<Report> {
    #[cfg(feature = "sym-profile-testing")]
    let origin = super::profile::set_origin(super::profile::Origin::Instrument);
    let report = SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let sess = slot.as_mut()?;
        if sess.budget.max_terms == 0 {
            return None;
        }
        let counts = sess.counts;
        let cap = sess.retry.max_forms;
        sess.retry.max_forms = cap.min(RETRY_FORMS);
        let exact = matches!(sess.witness, Some(crate::Witness::Exact));
        let door = sess.rules.registered && sess.rules.early;
        let mut report = Report::default();
        // One node is one question: a margin decided many times is asked
        // once.
        let mut asked = std::collections::HashMap::new();
        for (index, d) in decisions.iter().enumerate() {
            let Some(sign @ (Sign::Positive | Sign::Negative)) = d.sign else {
                continue;
            };
            if !sess.nodes.contains_key(&d.node) {
                report.absent += 1;
                continue;
            }
            report.checked += 1;
            let found =
                *asked
                    .entry(d.node)
                    .or_insert_with(|| match ladder(sess, d.node, Path::Algebra) {
                        Some(at) => Some((
                            if exact {
                                Event::Enclosure
                            } else {
                                Event::Rounded
                            },
                            at,
                        )),
                        None if door && !sess.registry.is_empty() => {
                            ladder(sess, d.node, Path::Decision).map(|at| (Event::Registration, at))
                        }
                        None => None,
                    });
            if let Some((event, (rung, attempt))) = found {
                report.contradictions.push(Contradiction {
                    index,
                    predicate: d.predicate,
                    node: d.node,
                    sign,
                    event,
                    rung,
                    attempt,
                });
            }
        }
        sess.counts = counts;
        sess.retry.max_forms = cap;
        Some(report)
    });
    #[cfg(feature = "sym-profile-testing")]
    super::profile::set_origin(origin);
    let mut report = report?;
    // Read after the session borrow is released: `reaches_free` takes
    // its own.
    let before = report.contradictions.len();
    report.contradictions.retain(|c| !reaches_free(c.node));
    report.free = before - report.contradictions.len();
    Some(report)
}
