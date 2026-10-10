//! **The re-valuation instrument** (FORK-S4F's guard against a computed
//! value re-entering the symbolic lane as a constant; ERROR-DESIGN E12).
//!
//! A symbolic `Zero` is a theorem about the DAG the decision saw. That
//! DAG is the computation only if every value the kernel computed
//! stayed in the scalar: a value read out to `f64` and re-entered
//! through [`crate::Real::from_f64`] is a `Lit`, a constant, where the
//! computation had a function of the parameters. This module makes that
//! visible. [`record`] keeps every decision a `Sym` lane takes — its
//! predicate, its margin's node and, at `Sym<f64>`, its value — and
//! [`revalue`] evaluates a recorded node's DAG at another parameter
//! point, op by op as `f64` performs it. Built at `p0` and re-valued at
//! `p1`, a margin equals the `p1` build's own margin bit for bit when
//! the DAG is the computation; a `Lit` that should have moved stays
//! where `p0` put it, and the two differ.
//!
//! An [`Opaque`](super::SymOp::Opaque) node has no value away from its
//! own build, and neither has a [`Hull`](super::SymOp::Hull) (a
//! function of enclosures, not of the reals they stand for), so a DAG
//! reaching either re-values to `None`: no claim is made about it, and
//! no identity is proved over it either.
//!
//! Test-only (`sym-revalue-testing`): with the feature off nothing here
//! compiles, and the hook in `Sym::sign_within` is absent.

use std::cell::RefCell;
use std::collections::HashMap;

use super::{ParamSymbol, SESSION, SymId, SymOp};
use crate::Real;

/// One decision a `Sym` lane took, in the order it took it (D9).
#[derive(Clone, Debug)]
pub struct Decision {
    /// The funnel predicate in flight (`k_stats::decide`'s name).
    pub predicate: &'static str,
    /// The margin's DAG node.
    pub node: SymId,
    /// The value channel, where it is an `f64` (`Sym<f64>`).
    pub value: Option<f64>,
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

/// The hook `Sym::sign_within` calls: one thread-local read when no
/// recorder is installed.
pub(super) fn note<T: 'static>(node: SymId, value: &T) {
    RECORDER.with(|r| {
        if let Some(v) = r.borrow_mut().as_mut() {
            v.push(Decision {
                predicate: crate::k_stats::current_predicate(),
                node,
                value: (value as &dyn std::any::Any).downcast_ref::<f64>().copied(),
            });
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
        let mut memo: HashMap<SymId, Option<f64>> = HashMap::new();
        // Post-order without recursion: a margin's DAG can be deep.
        let mut stack = vec![(node, false)];
        while let Some((id, expanded)) = stack.pop() {
            if memo.contains_key(&id) {
                continue;
            }
            let Some(n) = sess.nodes.get(&id) else {
                memo.insert(id, None);
                continue;
            };
            if !expanded {
                stack.push((id, true));
                stack.extend(n.kids().iter().map(|k| (*k, false)));
                continue;
            }
            let kid = |i: usize| memo.get(&n.kids[i]).copied().flatten();
            let v = match n.op {
                SymOp::Param => env(ParamSymbol(n.payload)),
                SymOp::Opaque | SymOp::Hull => None,
                SymOp::Lit => Some(f64::from_bits(n.payload)),
                SymOp::Pi => Some(f64::pi()),
                SymOp::Add => Some(kid(0)? + kid(1)?),
                SymOp::Sub => Some(kid(0)? - kid(1)?),
                // `Div` mints `Mul(a, Inv(b))` with the quotient as its
                // value; re-value it as the quotient it was.
                SymOp::Mul => match sess.nodes.get(&n.kids[1]) {
                    Some(inv) if inv.op == SymOp::Inv => {
                        Some(kid(0)? / memo.get(&inv.kids[0]).copied().flatten()?)
                    }
                    _ => Some(kid(0)? * kid(1)?),
                },
                SymOp::Neg => Some(-kid(0)?),
                SymOp::Powi => Some(kid(0)?.powi(n.payload as u32 as i32)),
                SymOp::Inv => Some(1.0 / kid(0)?),
                SymOp::Sqrt => Some(Real::sqrt(kid(0)?)),
                SymOp::Abs => Some(Real::abs(kid(0)?)),
                SymOp::Sin => Some(Real::sin_cos(kid(0)?).0),
                SymOp::Cos => Some(Real::sin_cos(kid(0)?).1),
                SymOp::Tan => Some(Real::tan(kid(0)?)),
                SymOp::Asin => Some(Real::asin(kid(0)?)),
                SymOp::Acos => Some(Real::acos(kid(0)?)),
                SymOp::Atan => Some(Real::atan(kid(0)?)),
                SymOp::Floor => Some(Real::floor(kid(0)?)),
                SymOp::Atan2 => Some(Real::atan2(kid(0)?, kid(1)?)),
                SymOp::Min => Some(Real::min(kid(0)?, kid(1)?)),
                SymOp::Max => Some(Real::max(kid(0)?, kid(1)?)),
                SymOp::Copysign => Some(Real::copysign(kid(0)?, kid(1)?)),
                SymOp::Select => Some(Real::select_le_zero(kid(0)?, kid(1)?, kid(2)?)),
            };
            memo.insert(id, v);
        }
        memo.get(&node).copied().flatten()
    })
}
