//! **The re-valuation instrument** (FORK-S4F's guard against a computed
//! value re-entering the symbolic lane as a constant; ERROR-DESIGN E12).
//!
//! A symbolic `Zero` is a theorem about the DAG the decision saw. That
//! DAG is the computation only if every value the kernel computed
//! stayed in the scalar: a value read out to `f64` and re-entered
//! through [`crate::Real::from_f64`] is a `Lit`, a constant, where the
//! computation had a function of the parameters. This module makes that
//! visible. [`record`](crate::sym::revalue::record) keeps every decision a `Sym` lane takes — its
//! predicate, its margin's node and, at `Sym<f64>`, its value — and
//! [`revalue`](crate::sym::revalue::revalue) evaluates a recorded node's DAG at another parameter
//! point, op by op as `f64` performs it. Built at `p0` and re-valued at
//! `p1`, a margin equals the `p1` build's own margin bit for bit when
//! the DAG is the computation; a `Lit` that should have moved stays
//! where `p0` put it, and the two differ.
//!
//! An [`Opaque`](crate::sym::SymOp::Opaque) node has no value away from its
//! own build, and neither has a [`Hull`](crate::sym::SymOp::Hull) (a
//! function of enclosures, not of the reals they stand for), so a DAG
//! reaching either re-values to `None`: no claim is made about it, and
//! no identity is proved over it either.
//!
//! Test-only (`sym-revalue-testing`): with the feature off nothing here
//! compiles, and the hook in `Sym::sign_within` is absent.

use std::cell::RefCell;

use super::{ParamSymbol, SESSION, SymId};
use crate::Real;
use crate::predicate::{Decided, Indeterminate, Sign};

/// One decision a `Sym` lane took, in the order it took it (D9).
#[derive(Clone, Debug)]
pub struct Decision {
    /// The funnel predicate in flight (`k_stats::decide`'s name).
    pub predicate: &'static str,
    /// The margin's DAG node.
    pub node: SymId,
    /// The value channel, where it is an `f64` (`Sym<f64>`): read back
    /// from its `Debug` rendering, which at `f64` is round-trip exact
    /// (the scalar's bits are reached through no other door here).
    pub value: Option<f64>,
    /// The sign the decision answered; `None` where it answered none
    /// (indeterminate, or refused). Two builds that decided one
    /// predicate with different signs may have branched after it.
    pub sign: Option<Sign>,
    /// Whether the symbolic tier proved the decision's `Zero`
    /// ([`crate::MarginDiag::proven`]): `false` for a sign read off the
    /// value channel, or no sign at all.
    pub proven: bool,
    /// What the decision did where it was taken
    /// ([`crate::k_stats::Role`]): a check can only make the build
    /// refuse, so a reader of this record may align the checks apart
    /// from the choices that steered the build.
    pub role: crate::k_stats::Role,
}

thread_local! {
    static RECORDER: RefCell<Option<Vec<Decision>>> = const { RefCell::new(None) };
}

/// Runs `f`, keeping every decision a `Sym` lane takes inside it.
///
/// # Panics
///
/// Nested inside another `record` on this thread: one recorder at a
/// time, so a decision is never charged to two runs.
pub fn record<R>(f: impl FnOnce() -> R) -> (R, Vec<Decision>) {
    RECORDER.with(|r| {
        let mut r = r.borrow_mut();
        assert!(r.is_none(), "revalue::record does not nest");
        *r = Some(Vec::new());
    });
    let out = f();
    let decisions = RECORDER.with(|r| r.borrow_mut().take()).unwrap_or_default();
    (out, decisions)
}

/// How many decisions the installed recorder holds, `None` without one
/// ([`super::aside`]'s mark).
pub(super) fn mark() -> Option<usize> {
    RECORDER.with(|r| r.borrow().as_ref().map(Vec::len))
}

/// Drops every decision recorded after `mark` ([`super::Aside::discard`]);
/// nothing where no recorder was installed at the mark.
pub(super) fn truncate(mark: Option<usize>) {
    if let Some(mark) = mark {
        RECORDER.with(|r| {
            if let Some(v) = r.borrow_mut().as_mut() {
                v.truncate(mark);
            }
        });
    }
}

/// The hook `Sym::sign_within` calls before it decides: one
/// thread-local read when no recorder is installed. Returns the
/// decision's index for [`settle`].
pub(super) fn note<T: Real>(node: SymId, value: T) -> Option<usize> {
    debug_assert!(
        !SESSION.with(|s| s.borrow().as_ref().is_some_and(|s| s.instrumented)),
        "a decision after `contradiction::check` in its session: the instrument is asked last, \
         because its walks are in the session's memos"
    );
    RECORDER.with(|r| {
        r.borrow_mut().as_mut().map(|v| {
            v.push(Decision {
                predicate: crate::k_stats::current_predicate(),
                node,
                value: format!("{value:?}").parse::<f64>().ok(),
                sign: None,
                proven: false,
                role: crate::k_stats::current_role(),
            });
            v.len() - 1
        })
    })
}

/// The hook `Sym::sign_within` calls with what it decided: the sign of
/// the decision [`note`] numbered.
pub(super) fn settle(noted: Option<usize>, decided: &Result<Decided, Indeterminate>) {
    let (Some(i), Ok(d)) = (noted, decided) else {
        return;
    };
    RECORDER.with(|r| {
        if let Some(x) = r.borrow_mut().as_mut().and_then(|v| v.get_mut(i)) {
            x.sign = Some(d.sign);
            x.proven = d.margin.proven();
        }
    });
}

/// `node`'s DAG evaluated at `f64` with every parameter read from
/// `env`, op by op as `Sym<f64>` performs it (a quotient as `a / b`, a
/// sine as `sin_cos().0`), so a DAG that IS the computation re-values
/// to the `f64` build's own bits.
///
/// `None` when the DAG reaches an opaque value or a hull, when `env`
/// has no value for a parameter, or outside a session (the nodes live
/// in the installed session's table, so call this inside the session
/// that built `node`).
#[must_use]
pub fn revalue(node: SymId, env: &dyn Fn(ParamSymbol) -> Option<f64>) -> Option<f64> {
    SESSION.with(|s| {
        let s = s.borrow();
        let sess = s.as_ref()?;
        super::point::evaluate(sess, &mut Default::default(), node, &|symbol| {
            env(ParamSymbol(symbol))
        })
    })
}
