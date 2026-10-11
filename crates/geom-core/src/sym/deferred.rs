//! **The deferring session** ([`with_deferred_session`]): every decision
//! is taken by the numeric channel, as the base scalar takes it, and a
//! decision it calls `Zero` keeps its margin's node on a list instead
//! of being discharged on the spot. A reader that wants to know whether
//! one of those Zeros is a theorem asks for it by its margin
//! ([`discharge`]), inside the same session, and pays the identity tier
//! for that margin alone.
//!
//! A replay that discharges every Zero as it is taken pays the tier on
//! every decision of the build, the curve-on-surface residuals of each
//! edge certificate among them; a reader that asks about a handful of
//! decisions pays for those. The decisions are the same either way: a
//! symbolic `Zero` is only ever offered where the numeric channel did
//! not call the sign definite, which is where it calls `Zero` or
//! escalates, so deferring changes which sign an escalation reads and
//! nothing else.
//!
//! A margin names its entry by a number no other entry in the process
//! carries ([`NEXT`]), so a margin carried out of its session reads as
//! no Zero in any other one, never as that one's entry of the same
//! place. Past the last number a Zero is kept as no entry at all, which
//! discharges as nothing: the door then reports the row unproven.

use super::{SESSION, SymBudget, SymCounts, SymId, SymRetry, SymRules};
use crate::predicate::{Decided, Indeterminate, MarginDiag, Sign, Theorem};

/// One entry of a deferring session's list: a Zero's margin, a theorem
/// already in hand, or two entries a decision stands on together
/// ([`MarginDiag::jointly`]).
#[derive(Clone, Copy, Debug)]
pub(crate) enum Pending {
    /// A margin the numeric channel called `Zero`.
    Zero(SymId),
    /// A theorem a decision already carried.
    Proven(Theorem),
    /// Both entries.
    Both(u32, u32),
}

/// [`super::with_session_retry`] as a **deferring** session: no decision
/// inside it is discharged as it is taken.
pub fn with_deferred_session<R>(
    budget: SymBudget,
    rules: SymRules,
    retry: SymRetry,
    f: impl FnOnce() -> R,
) -> (R, SymCounts) {
    super::with_session_retry(budget, rules, retry, || {
        SESSION.with(|s| {
            if let Some(sess) = s.borrow_mut().as_mut() {
                sess.deferred = Some(std::collections::HashMap::new());
            }
        });
        f()
    })
}

/// The hook `Sym::sign_within` calls with the numeric channel's answer:
/// `None` outside a deferring session, where the decision goes on to
/// the tier; inside one, that answer, a `Zero`'s margin stamped with
/// its place on the list.
pub(super) fn defer(
    node: SymId,
    numeric: Result<Decided, Indeterminate>,
) -> Option<Result<Decided, Indeterminate>> {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let list = slot.as_mut()?.deferred.as_mut()?;
        Some(match numeric {
            Ok(Decided {
                sign: Sign::Zero,
                margin,
            }) => Ok(Decided {
                sign: Sign::Zero,
                margin: match push(list, Pending::Zero(node)) {
                    Some(index) => margin.deferred_at(index),
                    None => margin,
                },
            }),
            other => other,
        })
    })
}

/// Entries `a` and `b` together, as one entry of the installed
/// deferring session's list; `None` outside one.
pub(crate) fn join(a: Pending, b: Pending) -> Option<u32> {
    SESSION.with(|s| {
        let mut slot = s.borrow_mut();
        let list = slot.as_mut()?.deferred.as_mut()?;
        let (a, b) = (push(list, a)?, push(list, b)?);
        push(list, Pending::Both(a, b))
    })
}

/// The next entry's number, process-wide: an entry's number names it
/// in no other session.
static NEXT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

/// The last number an entry can carry: the margin's word spends four
/// values on "none" and the theorems.
const LAST: u32 = u32::MAX - 4;

fn push(list: &mut std::collections::HashMap<u32, Pending>, entry: Pending) -> Option<u32> {
    use std::sync::atomic::Ordering;
    let index = NEXT
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| {
            (n < LAST).then_some(n + 1)
        })
        .ok()?;
    list.insert(index, entry);
    Some(index)
}

/// **Whether `margin`'s Zero is a theorem**, and of which kind: the
/// weakest of the theorems every Zero it stands on discharges as, run
/// now through the installed session's tier for a margin it deferred.
/// `None` where one of them is not a theorem, where the margin was not
/// a Zero, or outside the session that decided it.
#[must_use]
pub fn discharge(margin: MarginDiag) -> Option<Theorem> {
    match margin.pending()? {
        Pending::Proven(how) => Some(how),
        entry => discharge_entry(entry),
    }
}

fn discharge_entry(first: Pending) -> Option<Theorem> {
    let mut stack = vec![first];
    let mut weakest = Theorem::SymbolicZero;
    while let Some(next) = stack.pop() {
        match next {
            Pending::Proven(how) => weakest = weakest.max(how),
            Pending::Zero(node) => {
                let (how, _) = super::discharge_retried(node)?;
                weakest = weakest.max(how.theorem());
            }
            Pending::Both(a, b) => {
                stack.push(entry(a)?);
                stack.push(entry(b)?);
            }
        }
    }
    Some(weakest)
}

/// The entry at `index` of the installed session's list.
pub(crate) fn entry(index: u32) -> Option<Pending> {
    SESSION.with(|s| s.borrow().as_ref()?.deferred.as_ref()?.get(&index).copied())
}

/// **How large the expressions behind `margin`'s Zeros are**: the
/// distinct nodes of their DAGs and, among them, the parameter symbols,
/// for the measurement rows that price the door. `None` where the
/// margin stands for no deferred Zero, or outside its session.
#[must_use]
pub fn dag_size(margin: MarginDiag) -> Option<DagSize> {
    let mut roots = Vec::new();
    let mut stack = vec![margin.pending()?];
    while let Some(next) = stack.pop() {
        match next {
            Pending::Zero(node) => roots.push(node),
            Pending::Proven(_) => {}
            Pending::Both(a, b) => {
                stack.push(entry(a)?);
                stack.push(entry(b)?);
            }
        }
    }
    SESSION.with(|s| {
        let s = s.borrow();
        let sess = s.as_ref()?;
        let mut seen = std::collections::HashSet::new();
        let mut size = DagSize::default();
        while let Some(id) = roots.pop() {
            if !seen.insert(id) {
                continue;
            }
            let Some(n) = sess.nodes.get(&id) else {
                continue;
            };
            size.nodes += 1;
            if n.op == super::SymOp::Param {
                size.params += 1;
            }
            roots.extend(n.kids().iter().copied());
        }
        Some(size)
    })
}

/// The size [`dag_size`] reads.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DagSize {
    /// Distinct nodes.
    pub nodes: usize,
    /// Of them, parameter symbols.
    pub params: usize,
}

/// **The forms behind `margin`'s Zeros, rendered** with the session's
/// rules applied per node (`report::render_early_of`), for a row that
/// reads why a Zero did not discharge. Empty where the margin stands
/// for no deferred Zero.
#[must_use]
pub fn render(margin: MarginDiag) -> Vec<String> {
    let mut out = Vec::new();
    let Some(first) = margin.pending() else {
        return out;
    };
    let mut stack = vec![first];
    while let Some(next) = stack.pop() {
        match next {
            Pending::Zero(node) => {
                out.push(super::report::render_early_of(node).unwrap_or_default());
            }
            Pending::Proven(_) => {}
            Pending::Both(a, b) => stack.extend(entry(a).into_iter().chain(entry(b))),
        }
    }
    out
}
