//! **The cost profile** (`sym-profile-testing`, a test-only cargo
//! feature — see this crate's manifest): structural counters over one
//! thread's symbolic sessions, so the question "where inside the normal
//! form does the time go" is answered by a count rather than by a
//! reading of the code.
//!
//! What it records, per node the walk builds: the op, the sizes of the
//! kids' forms and of the form built, or — where the node FROZE — the
//! CAUSE the ring or the budget refused for, noted at the refusal site
//! itself ([`FreezeCause`]) rather than re-derived afterwards. Per walk
//! ([`Walk`]): how many forms each built and the wall time it took.
//! For the coefficient ring: every `Rat` operation, every one that
//! left the `i128` inline path, and the widest coefficient any form
//! carried. Inside the early walk: the per-node rule A/B reduction and
//! rule D's trig fold, each with its call count and wall time.
//!
//! **With the feature off none of this compiles** — no counter, no
//! branch, no clock: the hooks below are the only entry points, every
//! call site in `sym.rs` is a `#[cfg]`-gated statement, and the shipped
//! tier is bit for bit what it was. With the feature ON but the profile
//! not installed, every hook is one flag read. Thread-local like the
//! shape report (`report`), so a harness installs it, replays or drives
//! on the same thread, and takes it.

use core::cell::{Cell, RefCell};
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use super::report::FormSize;
use super::{Form, SymBudget, SymOp};

/// Why a node froze — the refusal the ring or the budget made, noted
/// where it was made.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FreezeCause {
    /// The TERM budget (`SymBudget::max_terms`): a product refused on
    /// its pre-bound (`Poly::mul`), or a form past the bound once built
    /// (`within`).
    Terms,
    /// The DEGREE budget (`SymBudget::max_degree`), at the same two
    /// doors.
    Degree,
    /// The COEFFICIENT bound: an integer past `COEFF_BITS` after
    /// reduction (`Rat::from_parts`), or an exponent alignment wider
    /// than it (`Rat::add`).
    Coefficient,
    /// An arithmetic OVERFLOW the ring refuses: a monomial exponent
    /// (`mono_mul`), a dyadic exponent (`Rat::mul`), a non-finite
    /// literal (`Rat::of_f64`).
    Overflow,
    /// A node not in the session's table — minted before the session
    /// was installed, or never recorded.
    Unrecorded,
    /// `combine` declined and no refusal site noted a cause — a shape
    /// this instrument does not name. Zero on every measured document;
    /// a non-zero count here is a hook missing, not a fifth cause.
    Unnoted,
}

/// The three normal-form walks (`plain_form`, `early_form`,
/// `door_form`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Walk {
    /// Every atom opaque, rule A0 only.
    Plain,
    /// Rules A/B per node, rule C's and rule D's folds.
    Early,
    /// The early walk with the registered-identity door applied.
    Door,
}

impl Walk {
    pub(super) fn of(early: bool, registry: bool) -> Self {
        match (early, registry) {
            (false, _) => Self::Plain,
            (true, false) => Self::Early,
            (true, true) => Self::Door,
        }
    }
}

/// What one op kind did across every node of that kind.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct OpProfile {
    /// Forms built (not frozen).
    pub built: u64,
    /// Nodes frozen.
    pub frozen: u64,
    /// Sum over the kids of every node (built or frozen) of the kid's
    /// numerator terms plus denominator terms.
    pub terms_in: u64,
    /// Sum over the forms built of numerator terms plus denominator
    /// terms.
    pub terms_out: u64,
    /// Sum over the kids of every node of the kid's total degree (the
    /// larger of numerator and denominator).
    pub degree_in: u64,
    /// Sum over the forms built of the form's total degree.
    pub degree_out: u64,
    /// The largest form built (numerator plus denominator terms).
    pub max_terms_out: usize,
    /// The highest total degree any form built reached.
    pub max_degree_out: u32,
}

/// One freeze: the node's op, the cause, the walk, and the kids'
/// sizes at the moment of the refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FreezeSite {
    /// The op of the node that froze (`"?"` for an unrecorded node).
    pub op: &'static str,
    /// The refusal.
    pub cause: FreezeCause,
    /// The walk that was building it.
    pub walk: Walk,
    /// The kids' forms, in slot order; `None` for an absent slot.
    pub kids: [Option<FormSize>; 2],
}

/// One walk's totals.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WalkProfile {
    /// Calls to the walk (one per decision asked of it, memo hits
    /// included).
    pub calls: u64,
    /// Forms the walk put in its memo, frozen ones included.
    pub forms: u64,
    /// Of those, frozen.
    pub frozen: u64,
    /// Wall time inside the walk, by `Instant`.
    pub time: Duration,
}

/// A timed counter: calls and wall time.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Timed {
    /// Calls.
    pub calls: u64,
    /// Wall time, by `Instant`.
    pub time: Duration,
}

/// Everything recorded since [`start_profile`].
#[derive(Clone, Debug, Default)]
pub struct SymProfile {
    /// Sessions that ended while the profile was installed.
    pub sessions: u64,
    /// DAG nodes interned, summed over those sessions.
    pub nodes: u64,
    /// Opaque atoms minted, summed over those sessions.
    pub atoms: u64,
    /// Per op kind.
    pub ops: BTreeMap<&'static str, OpProfile>,
    /// Per walk.
    pub walks: BTreeMap<Walk, WalkProfile>,
    /// Every freeze, in order.
    pub freezes: Vec<FreezeSite>,
    /// `Rat` additions and multiplications.
    pub rat_ops: u64,
    /// `Int` operations that ran on the heap integer — an operand was
    /// already `Big`, or the `i128` path overflowed.
    pub big_ops: u64,
    /// Of those, PROMOTIONS: both operands `Small` and the checked
    /// `i128` operation overflowed.
    pub promotions: u64,
    /// The widest integer (bits) any coefficient KEPT carried — at most
    /// `COEFF_BITS`.
    pub widest_bits: u64,
    /// The widest integer (bits) the ring REFUSED at the coefficient
    /// bound — zero when nothing was refused.
    pub widest_refused_bits: u64,
    /// The per-node rule A/B reduction in the early walk
    /// (`algebra::reduce_steps` under `SymRules::early_ab`).
    pub reduce: Timed,
    /// Rules A/B over the top residual (`algebra::reduce` in
    /// `discharge`).
    pub reduce_top: Timed,
    /// Rule D's fold (`trig::fold`), the memoized closed forms
    /// included.
    pub trig: Timed,
}

thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static NOTE: Cell<Option<FreezeCause>> = const { Cell::new(None) };
    static PROFILE: RefCell<SymProfile> = RefCell::new(SymProfile::default());
}

/// Installs the profile on this thread, dropping anything recorded.
pub fn start_profile() {
    PROFILE.with(|p| *p.borrow_mut() = SymProfile::default());
    ACTIVE.set(true);
}

/// Removes the profile and answers everything recorded since
/// [`start_profile`].
pub fn take_profile() -> SymProfile {
    ACTIVE.set(false);
    PROFILE.with(|p| core::mem::take(&mut *p.borrow_mut()))
}

#[inline]
fn active() -> bool {
    ACTIVE.get()
}

fn with(f: impl FnOnce(&mut SymProfile)) {
    if active() {
        PROFILE.with(|p| f(&mut p.borrow_mut()));
    }
}

/// A clock reading when the profile is installed, `None` otherwise —
/// so an uninstalled profile costs a flag read and no `Instant`.
#[inline]
pub(super) fn clock() -> Option<Instant> {
    active().then(Instant::now)
}

fn elapsed(t0: Option<Instant>) -> Duration {
    t0.map_or(Duration::ZERO, |t| t.elapsed())
}

/// Notes a refusal, at the site that made it. The next
/// [`record_node`] that finds its node frozen reads it.
#[inline]
pub(super) fn note(cause: FreezeCause) {
    if active() {
        NOTE.set(Some(cause));
    }
}

/// Notes which half of the budget a form is outside — terms first, as
/// `within` tests them.
pub(super) fn note_within(budget: SymBudget, f: &Form) {
    if !active() {
        return;
    }
    let terms = f.num.terms.len() > budget.max_terms || f.den.terms.len() > budget.max_terms;
    NOTE.set(Some(if terms {
        FreezeCause::Terms
    } else {
        FreezeCause::Degree
    }));
}

/// Clears the note before a node is combined, so a refusal a recovered
/// path made earlier (a fold that fell back to an atom) is never read
/// as this node's cause.
#[inline]
pub(super) fn clear_note() {
    if active() {
        NOTE.set(None);
    }
}

fn size_of(f: &Form) -> FormSize {
    FormSize {
        num: (f.num.terms.len(), f.num.degree()),
        den: (f.den.terms.len(), f.den.degree()),
    }
}

fn terms(s: FormSize) -> u64 {
    (s.num.0 + s.den.0) as u64
}

fn degree(s: FormSize) -> u32 {
    s.num.1.max(s.den.1)
}

/// Records one node the walk visited: its kids' forms and what came of
/// it — a form, or a freeze whose cause is the last note.
pub(super) fn record_node(op: SymOp, walk: Walk, kids: [&Form; 2], made: Option<&Form>) {
    if !active() {
        return;
    }
    let arity = op.arity();
    let sizes = [
        (arity >= 1).then(|| size_of(kids[0])),
        (arity >= 2).then(|| size_of(kids[1])),
    ];
    let name = op.name();
    let cause = NOTE.take();
    with(|p| {
        let o = p.ops.entry(name).or_default();
        for s in sizes.into_iter().flatten() {
            o.terms_in += terms(s);
            o.degree_in += u64::from(degree(s));
        }
        let w = p.walks.entry(walk).or_default();
        w.forms += 1;
        match made {
            Some(f) => {
                let s = size_of(f);
                o.built += 1;
                o.terms_out += terms(s);
                o.degree_out += u64::from(degree(s));
                o.max_terms_out = o.max_terms_out.max(s.num.0 + s.den.0);
                o.max_degree_out = o.max_degree_out.max(degree(s));
            }
            None => {
                o.frozen += 1;
                w.frozen += 1;
                p.freezes.push(FreezeSite {
                    op: name,
                    cause: cause.unwrap_or(FreezeCause::Unnoted),
                    walk,
                    kids: sizes,
                });
            }
        }
    });
}

/// Records a freeze of a node the session never recorded.
pub(super) fn record_unrecorded(walk: Walk) {
    with(|p| {
        let w = p.walks.entry(walk).or_default();
        w.forms += 1;
        w.frozen += 1;
        p.freezes.push(FreezeSite {
            op: "?",
            cause: FreezeCause::Unrecorded,
            walk,
            kids: [None, None],
        });
    });
}

/// Records one call to a walk: how many forms it added to its memo and
/// how long it took.
pub(super) fn walk_done(walk: Walk, t0: Option<Instant>) {
    let dt = elapsed(t0);
    with(|p| {
        let w = p.walks.entry(walk).or_default();
        w.calls += 1;
        w.time += dt;
    });
}

/// One `Rat` addition or multiplication.
#[inline]
pub(super) fn rat_op() {
    with(|p| p.rat_ops += 1);
}

/// One `Int` operation on the heap path; `promoted` when both operands
/// were `Small` and the `i128` operation overflowed.
#[inline]
pub(super) fn big_path(promoted: bool) {
    with(|p| {
        p.big_ops += 1;
        if promoted {
            p.promotions += 1;
        }
    });
}

/// The width of one reduced coefficient, kept or refused at the bound.
#[inline]
pub(super) fn coefficient_bits(bits: u64, kept: bool) {
    with(|p| {
        if kept {
            p.widest_bits = p.widest_bits.max(bits);
        } else {
            p.widest_refused_bits = p.widest_refused_bits.max(bits);
        }
    });
}

fn timed(t: &mut Timed, t0: Option<Instant>) {
    t.calls += 1;
    t.time += elapsed(t0);
}

/// One per-node rule A/B reduction finished.
pub(super) fn reduce_done(t0: Option<Instant>) {
    with(|p| timed(&mut p.reduce, t0));
}

/// One top-residual rule A/B reduction finished.
pub(super) fn reduce_top_done(t0: Option<Instant>) {
    with(|p| timed(&mut p.reduce_top, t0));
}

/// One rule D fold finished.
pub(super) fn trig_done(t0: Option<Instant>) {
    with(|p| timed(&mut p.trig, t0));
}

/// One session ended, with this many nodes and atoms in its table.
pub(super) fn session_done(nodes: usize, atoms: usize) {
    with(|p| {
        p.sessions += 1;
        p.nodes += nodes as u64;
        p.atoms += atoms as u64;
    });
}

impl SymProfile {
    /// Freezes by `(cause, op)`, with the kids' total degrees and term
    /// counts summarised: `(count, min degree, max degree, min terms,
    /// max terms)` over every kid slot at those freezes.
    #[must_use]
    pub fn freezes_by_cause(&self) -> BTreeMap<(FreezeCause, &'static str), FreezeSummary> {
        let mut out: BTreeMap<(FreezeCause, &'static str), FreezeSummary> = BTreeMap::new();
        for f in &self.freezes {
            let s = out.entry((f.cause, f.op)).or_default();
            s.count += 1;
            for k in f.kids.into_iter().flatten() {
                let d = degree(k);
                let t = terms(k);
                s.min_degree = s.min_degree.min(d);
                s.max_degree = s.max_degree.max(d);
                s.min_terms = s.min_terms.min(t);
                s.max_terms = s.max_terms.max(t);
                s.degree_sum += u64::from(d);
                s.terms_sum += t;
                s.kids += 1;
            }
        }
        out
    }

    /// Frozen nodes in total, every walk.
    #[must_use]
    pub fn frozen(&self) -> u64 {
        self.freezes.len() as u64
    }
}

/// The kids' sizes over one `(cause, op)` class of freezes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FreezeSummary {
    /// Freezes in the class.
    pub count: u64,
    /// Kid slots summarised (up to two per freeze).
    pub kids: u64,
    /// The smallest kid total degree.
    pub min_degree: u32,
    /// The largest kid total degree.
    pub max_degree: u32,
    /// The sum of kid total degrees (for the mean).
    pub degree_sum: u64,
    /// The fewest kid terms (numerator plus denominator).
    pub min_terms: u64,
    /// The most kid terms.
    pub max_terms: u64,
    /// The sum of kid terms (for the mean).
    pub terms_sum: u64,
}

impl Default for FreezeSummary {
    fn default() -> Self {
        Self {
            count: 0,
            kids: 0,
            min_degree: u32::MAX,
            max_degree: 0,
            degree_sum: 0,
            min_terms: u64::MAX,
            max_terms: 0,
            terms_sum: 0,
        }
    }
}

fn mean(sum: u64, n: u64) -> f64 {
    if n == 0 { 0.0 } else { sum as f64 / n as f64 }
}

impl core::fmt::Display for SymProfile {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "sessions {}  nodes {}  atoms {}  frozen {}",
            self.sessions,
            self.nodes,
            self.atoms,
            self.frozen()
        )?;
        writeln!(f, "walk   | calls | forms | frozen | time")?;
        for (w, p) in &self.walks {
            writeln!(
                f,
                "{:6} | {:5} | {:5} | {:6} | {:?}",
                format!("{w:?}"),
                p.calls,
                p.forms,
                p.frozen,
                p.time
            )?;
        }
        writeln!(
            f,
            "early: reduce_steps {} calls {:?}; trig::fold {} calls {:?}; top reduce {} calls {:?}",
            self.reduce.calls,
            self.reduce.time,
            self.trig.calls,
            self.trig.time,
            self.reduce_top.calls,
            self.reduce_top.time
        )?;
        writeln!(
            f,
            "ring: rat ops {}  big-path int ops {}  promotions {}  widest coefficient kept {} bits, refused {} bits",
            self.rat_ops, self.big_ops, self.promotions, self.widest_bits, self.widest_refused_bits
        )?;
        writeln!(
            f,
            "op       | built | frozen | terms in->out (mean) | degree in->out (mean) | max terms | max degree"
        )?;
        for (op, o) in &self.ops {
            let n = o.built + o.frozen;
            writeln!(
                f,
                "{:8} | {:5} | {:6} | {:8.1} -> {:8.1} | {:6.1} -> {:6.1} | {:9} | {:10}",
                op,
                o.built,
                o.frozen,
                mean(o.terms_in, n),
                mean(o.terms_out, o.built),
                mean(o.degree_in, n),
                mean(o.degree_out, o.built),
                o.max_terms_out,
                o.max_degree_out
            )?;
        }
        if !self.freezes.is_empty() {
            writeln!(
                f,
                "freeze cause | op       | count | kid degree min/mean/max | kid terms min/mean/max"
            )?;
            for ((cause, op), s) in self.freezes_by_cause() {
                writeln!(
                    f,
                    "{:12} | {:8} | {:5} | {:3} / {:6.1} / {:3} | {:4} / {:7.1} / {:4}",
                    format!("{cause:?}"),
                    op,
                    s.count,
                    if s.kids == 0 { 0 } else { s.min_degree },
                    mean(s.degree_sum, s.kids),
                    s.max_degree,
                    if s.kids == 0 { 0 } else { s.min_terms },
                    mean(s.terms_sum, s.kids),
                    s.max_terms
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::k_stats::decide;
    use crate::predicate::Margin;
    use crate::real::Real;
    use crate::sym::{ParamSymbol, Sym, SymRules, with_session_rules};
    use crate::tolerance::Tol;

    fn p(name: &str, v: f64) -> Sym<f64> {
        Sym::param(ParamSymbol::of(name), v)
    }

    fn ask(m: Sym<f64>) {
        let band = crate::predicate::Band::linear(Tol::witness()).unwrap();
        let _ = decide("sym_profile_test", Margin::of(m), band);
    }

    fn budget(max_terms: usize, max_degree: u32) -> SymBudget {
        SymBudget {
            max_terms,
            max_degree,
        }
    }

    /// One session under the shipped rules with the profile installed,
    /// answering what it recorded.
    fn profiled(b: SymBudget, f: impl FnOnce()) -> SymProfile {
        start_profile();
        let _ = with_session_rules(b, SymRules::shipped(), f);
        take_profile()
    }

    /// Every freeze as `(cause, op, walk)`. A node the plain walk
    /// freezes is frozen AGAIN by the early walk (a second memo, the
    /// same refusal), so under the shipped rules a freeze appears
    /// once per walk — which is why this profile's `frozen()` is not
    /// `SymCounts::frozen`, the plain walk's count alone.
    fn causes(p: &SymProfile) -> Vec<(FreezeCause, &'static str, Walk)> {
        p.freezes.iter().map(|f| (f.cause, f.op, f.walk)).collect()
    }

    /// **Uninstalled, the profile records nothing** — the hooks are one
    /// flag read, and a session run before `start_profile` leaves no
    /// trace in what a later `take_profile` answers.
    #[test]
    fn nothing_is_recorded_while_uninstalled() {
        let _ = with_session_rules(budget(4096, 128), SymRules::shipped(), || {
            ask(p("x", 1.0) * p("y", 2.0) - p("y", 2.0) * p("x", 1.0));
        });
        start_profile();
        let out = take_profile();
        assert_eq!(out.sessions, 0, "{out:?}");
        assert!(out.ops.is_empty() && out.freezes.is_empty(), "{out:?}");
    }

    /// **A product past the term budget freezes for `Terms`**, on the
    /// `Mul` that asked for it, with the kids' sizes at the freeze —
    /// `(x + y)·(x − y)` has four candidate terms and a budget of three
    /// refuses it before it is built.
    #[test]
    fn a_term_budget_refusal_is_a_terms_freeze_on_the_product() {
        let out = profiled(budget(3, 128), || {
            let (x, y) = (p("x", 1.0), p("y", 2.0));
            ask((x + y) * (x - y));
        });
        assert_eq!(out.sessions, 1);
        assert_eq!(
            causes(&out),
            vec![
                (FreezeCause::Terms, "Mul", Walk::Plain),
                (FreezeCause::Terms, "Mul", Walk::Early),
            ],
            "{out}"
        );
        let site = out.freezes[0];
        assert_eq!(site.kids[0].unwrap().num, (2, 1), "x + y");
        assert_eq!(site.kids[1].unwrap().num, (2, 1), "x − y");
        assert_eq!(out.ops["Mul"].frozen, 2, "once per walk");
        assert_eq!(out.walks[&Walk::Plain].frozen, 1);
        assert_eq!(out.walks[&Walk::Early].frozen, 1);
        assert_eq!(out.frozen(), 2);
    }

    /// **A product past the degree budget freezes for `Degree`** —
    /// `x·x·x` at a degree budget of two — and the kid degrees at the
    /// freeze are the two operands', `x²` and `x`.
    #[test]
    fn a_degree_budget_refusal_is_a_degree_freeze_with_the_kids_degrees() {
        let out = profiled(budget(4096, 2), || {
            let x = p("x", 1.0);
            ask(x * x * x);
        });
        assert_eq!(
            causes(&out),
            vec![
                (FreezeCause::Degree, "Mul", Walk::Plain),
                (FreezeCause::Degree, "Mul", Walk::Early),
            ],
            "{out}"
        );
        let site = out.freezes[0];
        let degrees: Vec<u32> = site.kids.iter().flatten().map(|k| k.num.1).collect();
        assert_eq!(degrees, vec![2, 1]);
    }

    /// **A coefficient past the ring's bound freezes for
    /// `Coefficient`**: the product of five 53-bit mantissas is 265
    /// bits, past `COEFF_BITS`, and the profile names the refused
    /// width beside the widest coefficient the form kept.
    #[test]
    fn a_coefficient_past_the_bound_is_a_coefficient_freeze() {
        let out = profiled(budget(4096, 128), || {
            // Minted INSIDE the session: a literal built before it is
            // not in the session's table and freezes as `Unrecorded`.
            let m = Sym::<f64>::from_f64(9_007_199_254_740_991.0);
            let x = p("x", 1.0);
            ask(x * m * m * m * m * m);
        });
        assert_eq!(
            causes(&out),
            vec![
                (FreezeCause::Coefficient, "Mul", Walk::Plain),
                (FreezeCause::Coefficient, "Mul", Walk::Early),
            ],
            "{out}"
        );
        assert_eq!(out.widest_refused_bits, 265, "{out}");
        assert_eq!(out.widest_bits, 212, "four mantissas: {out}");
        assert!(out.promotions >= 1, "the third product leaves i128: {out}");
    }

    /// **The walks and the ring are counted**: a theorem asked of the
    /// plain form is one plain-walk call with its forms, no early walk,
    /// and every `Rat` operation the forms took.
    #[test]
    fn the_walks_and_the_ring_are_counted() {
        let out = profiled(budget(4096, 128), || {
            let (x, y) = (p("x", 1.0), p("y", 2.0));
            ask(x * y - y * x);
        });
        let plain = out.walks[&Walk::Plain];
        assert_eq!(plain.calls, 1);
        // x, y, x·y, y·x and the difference: five nodes, five forms.
        assert_eq!(plain.forms, 5, "{out}");
        assert!(!out.walks.contains_key(&Walk::Early), "{out}");
        assert_eq!(out.ops["Mul"].built, 2);
        assert_eq!(out.ops["Sub"].built, 1);
        assert_eq!(out.nodes, 5);
        assert!(out.rat_ops > 0 && out.promotions == 0, "{out}");
        assert_eq!(out.frozen(), 0);
    }
}
