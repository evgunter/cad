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
//! ([`Walk`]) and per ORIGIN ([`Origin`] — the decision's own discharge,
//! the contradiction assertion on a definite sign, or the shape
//! report's rendering): how many forms each built and the wall time it
//! took. For the coefficient ring: every `Rat` operation, every one
//! that left the `i128` inline path, and the widest coefficient any
//! form carried. Inside the early walk: the per-node rule A/B reduction
//! and rule D's trig fold, each with its call count and wall time.
//!
//! **With the feature off none of this compiles** — no counter, no
//! branch, no clock: the hooks below are the only entry points, every
//! call site in `sym.rs` is a `#[cfg]`-gated statement, and the shipped
//! tier is bit for bit what it was. With the feature ON but the profile
//! not installed, every hook is one flag read. Thread-local like the
//! shape report (`report`), so a harness installs it, replays or drives
//! on the same thread, and takes it.

use core::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::time::{Duration, Instant};

use super::form::Form;
use super::report::{FormSize, size_of};
use super::{Hash128, Rung, SymBudget, SymOp};

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
    /// (`mono_mul`), a dyadic exponent (`Rat::mul`, `Rat::from_parts`),
    /// a non-finite literal (`Rat::of_f64`).
    Overflow,
    /// A rational with a zero denominator (`Rat::from_parts`) — the
    /// reciprocal of a zero coefficient.
    ZeroDivisor,
    /// A node not in the session's table — minted before the session
    /// was installed, or never recorded.
    Unrecorded,
    /// `combine` declined and no refusal site noted a cause — a shape
    /// this instrument does not name. A non-zero count here is a hook
    /// missing, not a further cause, and the evidence rows assert it
    /// zero.
    Unnoted,
}

/// The normal-form walks (`plain_form`, `early_form`, `door_form`), and
/// the two a RETRY attempt runs in its own memos (`sym::SymRetry`).
///
/// The retry buckets are SEPARATE and not summed into the first
/// attempt's: `Early` and `Door` are the tier's first attempt on every
/// profile, with a ladder installed or without one, so a row that pins
/// them is measuring the same thing it measured before the ladder
/// existed. They carry no attempt number — an attempt's own forms are
/// in `DecisionRecord`'s attribution, and a bucket per attempt would
/// make the ledger's shape depend on how long the ladder is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Walk {
    /// Every atom opaque, rule A0 only.
    Plain,
    /// Rules A/B per node, rule C's and rule D's folds.
    Early,
    /// The early walk with the registered-identity door applied.
    Door,
    /// A RETRY attempt's early walk, in that attempt's own memo.
    RetryEarly,
    /// A RETRY attempt's door walk, in that attempt's own memo.
    RetryDoor,
}

impl Walk {
    pub(super) fn of(early: bool, registry: bool) -> Self {
        match (early, registry) {
            (false, _) => Self::Plain,
            (true, false) => Self::Early,
            (true, true) => Self::Door,
        }
    }

    /// The bucket this walk is CHARGED to — itself on the first
    /// attempt, its retry twin inside one ([`set_attempt`]).
    fn charged(self) -> Self {
        if ATTEMPT.get() == 0 {
            return self;
        }
        match self {
            Self::Early => Self::RetryEarly,
            Self::Door => Self::RetryDoor,
            other => other,
        }
    }
}

/// WHY a walk was asked — the `Decide` site has three callers of the
/// normal form, and only one of them is the tier deciding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Origin {
    /// The decision's own discharge: the numeric channel could not
    /// answer, and the form is what decides. The shipped release
    /// check of a registered zero against a definite sign
    /// (`door_zero`) counts here too — it is the decision path.
    Decision,
    /// The contradiction ASSERTION on a definite sign: `Decide for
    /// Sym<T>` runs `discharge` inside a `debug_assert!` on every
    /// margin the numeric channel proved non-zero, so under debug
    /// assertions (dev, test, and this workspace's release profile)
    /// those forms are built by the assertion and not by the tier.
    Assertion,
    /// The shape report's rendering of a blocked residual
    /// (`report::render_node`), when the report is installed.
    Report,
}

/// What one op kind did across every node of that kind.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OpProfile {
    /// The op, as `SymOp`'s `Debug` renders it.
    pub op: String,
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
    /// Wall time building every node of this kind, frozen ones
    /// included: `combine`, the per-node reduction, rule E and the
    /// budget check, by `Instant` ([`NodeCost::time`] per node).
    pub time: Duration,
}

/// **One node's cost in one walk**, summed over every session that
/// computed it — the per-node row [`SymProfile::node_delta`] joins two
/// profiles on. A node id is a content hash of the DAG, and rule G
/// changes no node of it, so two replays of one leaf under two rule
/// sets visit the same ids and the join compares the SAME node's form
/// under each.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NodeCost {
    /// The op's tag (`SymOp::tag`).
    pub op: u64,
    /// Computations: one per session whose walk built it.
    pub visits: u64,
    /// Of those, frozen.
    pub frozen: u64,
    /// Numerator plus denominator terms of the forms built, summed.
    pub terms: u64,
    /// Total degree of the forms built, summed.
    pub degree: u64,
    /// Wall time building it, as [`OpProfile::time`] counts.
    pub time: Duration,
}

/// One freeze: the node's op, the cause, the walk and its origin, and
/// the kids' sizes at the moment of the refusal.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FreezeSite {
    /// The op's tag (`SymOp::tag`), the key of [`SymProfile::ops`];
    /// `None` for an unrecorded node. [`SymProfile::op_name`] renders
    /// it.
    pub op: Option<u64>,
    /// The refusal.
    pub cause: FreezeCause,
    /// The walk that was building it.
    pub walk: Walk,
    /// Why the walk was asked.
    pub origin: Origin,
    /// The kids' forms, in slot order; `None` for an absent slot. Three
    /// slots, the third read by the arity-3 ops alone.
    pub kids: [Option<FormSize>; 3],
}

/// One walk's totals under one origin.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WalkProfile {
    /// Calls to the walk — one per discharge asked of it, memo hits
    /// included, whether the discharge was the decision's, the
    /// assertion's or the report's ([`Origin`]).
    pub calls: u64,
    /// Forms the walk put in its memo, frozen ones included.
    pub forms: u64,
    /// Of those, frozen.
    pub frozen: u64,
    /// Wall time inside the walk, by `Instant`. The session's teardown
    /// — dropping the memos and the table — is outside every walk and
    /// no clock here sees it.
    pub time: Duration,
    /// **The forms themselves, folded**: every form the walk put in its
    /// memo under this origin, in build order, each by its canonical
    /// digest ([`Form::digest`] — the key an atom over it is minted
    /// under), chained through one hash. Two profiles agree here iff
    /// the walk built the same forms in the same order, to the
    /// coefficient and the term order, so a change to what a form IS
    /// in memory that moved what it SAYS reads as a different number.
    pub digest: u128,
}

impl WalkProfile {
    fn absorb(&mut self, o: Self) {
        self.calls += o.calls;
        self.forms += o.forms;
        self.frozen += o.frozen;
        self.time += o.time;
        self.digest = chain(self.digest, o.digest);
    }
}

/// One more link in a digest chain.
fn chain(acc: u128, next: u128) -> u128 {
    Hash128::new().wide(acc).wide(next).finish()
}

/// A timed counter: calls and wall time.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Timed {
    /// Calls.
    pub calls: u64,
    /// Wall time, by `Instant`.
    pub time: Duration,
}

/// **How ONE decision was answered, and what froze on its way** — the
/// per-decision attribution the ladder's measurement needs.
///
/// **What `causes` is, exactly, and what it is not.** It is the freezes
/// THIS decision's walks COMPUTED — the sites recorded between
/// `sym::discharge_in`'s two hooks. A decision whose forms were already
/// in a memo (another decision built them, or a drive memo served them)
/// records none, so the column attributes a freeze to the FIRST
/// decision that paid for it and not to every decision that stood on
/// it. That is the honest reading of a memoized walk and it is the
/// reason the table beside it counts REFUSALS rather than dividing
/// freezes among them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecisionRecord {
    /// Why the walk was asked — the decision's own discharge, the
    /// contradiction assertion, or the shape report.
    pub origin: Origin,
    /// The rung that answered and the ATTEMPT it answered on (0 the
    /// first, `k` the `k`th rung of the retry ladder); `None` where
    /// every rung of every attempt declined and the decision is
    /// numeric.
    pub answered: Option<(Rung, u8)>,
    /// The freezes this decision's own walks made, by cause.
    pub causes: BTreeMap<FreezeCause, u64>,
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
    /// Per op kind, keyed by `SymOp::tag`.
    pub ops: BTreeMap<u64, OpProfile>,
    /// Per walk and origin.
    pub walks: BTreeMap<(Walk, Origin), WalkProfile>,
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
    /// **The DISTINCT node ids the plain walk computed a form for**,
    /// over every session recorded — against `walk(Walk::Plain).forms`,
    /// which counts each computation, this is how many of them a memo
    /// keyed by the id could have answered from another leaf. The
    /// ratio is the ceiling of a drive-scoped plain memo's win.
    pub plain_ids: BTreeSet<u128>,
    /// **The retry memos' size, per attempt** — for the `k`th retry
    /// (index `k − 1`), the most forms its early and door memos held
    /// together at the end of any session recorded. The measurement
    /// `sym::RETRY_FORMS` is set against: what a ladder actually costs a
    /// leaf in held forms, beside [`Self::nodes`] (the DAG that bounds
    /// one attempt's walk).
    pub retry_forms: Vec<usize>,
    /// **Every decision, in the order it was answered** — the rung and
    /// attempt that answered it and the freezes it computed
    /// ([`DecisionRecord`]).
    pub decisions: Vec<DecisionRecord>,
    /// **Each session's set of `Opaque` indeterminate ids**, in the
    /// order the sessions ended.
    ///
    /// An `Opaque` id is the SEQUENCE NUMBER the leaf minted it at
    /// (`OPAQUE_SEQ`) — the one part of a node id that is not a hash of
    /// what the expression says. Two leaves that mint in different
    /// orders therefore build different ids for the same subexpression,
    /// and a drive-scoped plain memo MISSES on them. It does not answer
    /// them wrongly: `sym::memo`'s header carries that argument once
    /// (a plain form is a syntactic normal form of a syntactic id), and
    /// this set is the instrument for the HIT RATE, not for soundness.
    ///
    /// On every document in the tree today every set is EMPTY, because
    /// no drive mints an opaque at all: `Sym::opaque`'s one caller is
    /// the unnamed `AxisScalar::axis`, and a drive binds its axes
    /// through `axis_named`.
    pub opaque_ids: Vec<BTreeSet<u128>>,
    /// **The decision read's cost** ([`ReadProfile`]).
    pub read: ReadProfile,
    /// **Rule G's own cost** ([`RootProfile`]).
    pub root: RootProfile,
    /// **Every node each walk built, by `(walk, node id)`** — the
    /// per-node sizes and times [`Self::node_delta`] aligns.
    pub node_costs: HashMap<(Walk, u128), NodeCost>,
}

/// **Rule G's own cost** (`sym::root`): the calls and wall time of the
/// canonical root by the branch that answered or declined, the parts
/// inside it, the `abs` node's atom door, and what the canonical form
/// hands the walk in place of the one-term atom.
///
/// **`own` is the mint site's whole time and nothing else**: every
/// OUTERMOST entry into rule G — `root::canonical` from `combine` or
/// from `root::mint`, and `root::magnitude_atom` from `combine`'s `abs`
/// arm — clocked once, so a nested entry is not counted twice. The
/// rest of a walk's time is the rest of the walk: what it does with
/// the forms rule G handed it.
///
/// A branch is the one [`root_note`] named last inside the call: the
/// exact quotient, the polynomial argument (`den = 1`), the split by
/// the side condition's source, or a decline by where it declined.
/// A part is timed inside the call it belongs to, so the parts of one
/// branch sum to at most that branch's time.
#[derive(Clone, Debug, Default)]
pub struct RootProfile {
    /// `root::canonical`'s calls, whole, by branch.
    pub branches: BTreeMap<&'static str, Timed>,
    /// The parts inside it, with how many of each SUCCEEDED (a division
    /// that was exact, a perfect-square search that found a root, a
    /// side condition that proved a sign).
    pub parts: BTreeMap<&'static str, (Timed, u64)>,
    /// `root::magnitude_atom` asked by an `abs` NODE.
    pub abs_door: Timed,
    /// Rule G's own time: the outermost entries, summed.
    pub own: Timed,
    /// Calls whose argument form this session had already asked
    /// `canonical` about, by digest — what a per-session memo keyed by
    /// the argument would answer — and their time.
    pub repeats: Timed,
    /// Where `canonical` answered: the answers' terms (numerator plus
    /// denominator) and total degree, summed, and how many carry a
    /// non-constant denominator. The atom it stands in for is one term
    /// of degree one over the constant one.
    pub answered: u64,
    /// See [`Self::answered`].
    pub out_terms: u64,
    /// See [`Self::answered`].
    pub out_degree: u64,
    /// See [`Self::answered`].
    pub out_quotients: u64,
}

impl RootProfile {
    /// Rule G's table, as text.
    #[must_use]
    pub fn render(&self) -> String {
        use core::fmt::Write as _;
        let mut f = String::new();
        let _ = writeln!(
            f,
            "rule G own: {} outermost entries in {:?}; abs door {} in {:?}; \
             repeated arguments {} in {:?}",
            self.own.calls,
            self.own.time,
            self.abs_door.calls,
            self.abs_door.time,
            self.repeats.calls,
            self.repeats.time
        );
        let calls: u64 = self.branches.values().map(|t| t.calls).sum();
        let time: Duration = self.branches.values().map(|t| t.time).sum();
        let _ = writeln!(f, "canonical: {calls} calls in {time:?}");
        for (b, t) in &self.branches {
            let _ = writeln!(f, "  branch {b:<34} {:6} in {:?}", t.calls, t.time);
        }
        for (p, (t, hit)) in &self.parts {
            let _ = writeln!(
                f,
                "  part   {p:<34} {:6} ({hit} succeeded) in {:?}",
                t.calls, t.time
            );
        }
        let _ = writeln!(
            f,
            "answers: {} (mean terms {:.2}, mean degree {:.2}, {} with a non-constant denominator)",
            self.answered,
            mean(self.out_terms, self.answered),
            mean(self.out_degree, self.answered),
            self.out_quotients
        );
        f
    }
}

/// **The decision read's cost** (`signed::decision`, and
/// `signed::order` in front of it at `min`/`max`): how often it is
/// asked, what it answers, WHY it declines, and where its time goes —
/// the measurement that says which of the cheap answers before the
/// enclosure would pay.
///
/// **A decline's cause is the enclosure's own refusal**, noted at the
/// arm of `signed::enclose_indet` / `enclose_deep` that refused it
/// ([`read_note`]), so there is one spelling of what the enclosure
/// reaches. The instrument re-encloses each half after the read has
/// answered and reads the first note: an atom past the depth cap
/// (`depth exhausted`, the cap's test coming first, as the enclosure
/// makes it); an indeterminate with no bracket at the top of a half
/// (`unbracketed@0`) or `k` atom levels down (`unbracketed@k`), with
/// what it is (an atom's op, or `opaque` for an id no atom was minted
/// for); a poisoned argument; or a poisoned bracket. Past those, the
/// read's own order decides: the denominator's half straddles, or the
/// numerator's.
///
/// `prepass` counts the declines where either half's enclosure is
/// refused STRUCTURALLY — an unbracketed id, the cap, a poisoned
/// argument — which an id-walk over both halves could prove before an
/// interval is built. A half the enclosure refuses on a poisoned
/// bracket first is not counted, so the column is a floor.
#[derive(Clone, Debug, Default)]
pub struct ReadProfile {
    /// Reads asked at the `Select` door.
    pub select: u64,
    /// Reads asked at `min`/`max` (`signed::order`), before the
    /// difference is built.
    pub order: u64,
    /// Of those, the ones whose difference `a − b` the ring or the
    /// budget refused, so `signed::decision` was never asked. A
    /// POISONED difference is not a second arm: `combine` answers
    /// poison before `order` when either kid is tainted, and
    /// `Form::add` poisons only a tainted pair.
    pub order_refused: u64,
    /// Time building `a − b` at `min`/`max`.
    pub order_diff: Duration,
    /// Reads that returned an arm.
    pub settled: u64,
    /// Declines, by cause (see the type's doc).
    pub declines: BTreeMap<String, u64>,
    /// Wall time of the read itself, every call: from entering
    /// `signed::decision` to its answer, the instrument's own
    /// re-enclosure and the form's digest excluded.
    pub time: Duration,
    /// Of it, stripping the manifestly positive content.
    pub strip: Duration,
    /// Of it, the enclosure (`enclose_deep`, both halves).
    pub enclose: Duration,
    /// Declines an id-walk over both halves could prove (see the
    /// type's doc).
    pub prepass: u64,
    /// The read time of the calls in `prepass`.
    pub prepass_time: Duration,
    /// Of it, the enclosure alone — the most a pre-pass could save,
    /// since it must strip the halves before it walks them, and before
    /// its own cost.
    pub prepass_enclose: Duration,
    /// Reads whose decision form's digest this session had already
    /// read — what a per-session memo keyed by the digest would hit.
    /// The digest carries the `gated` bit, which does not move the
    /// read, so a memo keyed without it could hit more: this is a
    /// floor.
    pub repeats: u64,
    /// The read time of those repeats.
    pub repeat_time: Duration,
    /// Reads whose two halves both enclose, by the deepest atom level
    /// the enclosure enters (0: parameters and π alone), with the ones
    /// that settled beside: `(asked, settled)`.
    pub depth: BTreeMap<usize, (u64, u64)>,
}

/// What the instrument learned about one call of the read, beside its
/// answer.
pub(super) struct ReadClass {
    /// The decline's cause, `None` where the read settled.
    pub cause: Option<String>,
    /// Whether an id-walk could prove the decline before any enclosure.
    pub prepass: bool,
    /// The deepest atom level the enclosure enters, where both halves
    /// enclose.
    pub depth: Option<usize>,
}

impl ReadProfile {
    /// The read's table, as text.
    #[must_use]
    pub fn render(&self) -> String {
        use core::fmt::Write as _;
        let mut f = String::new();
        let asked = (self.select + self.order).checked_sub(self.order_refused);
        let declined = asked.and_then(|a| a.checked_sub(self.settled));
        let _ = writeln!(
            f,
            "read: select {} order {} (a-b refused {}, built in {:?})  asked {asked:?}  \
             settled {}  declined {declined:?}",
            self.select, self.order, self.order_refused, self.order_diff, self.settled,
        );
        let _ = writeln!(
            f,
            "read time {:?}: strip {:?}, enclose {:?}",
            self.time, self.strip, self.enclose
        );
        for (cause, n) in &self.declines {
            let _ = writeln!(f, "  decline {cause:<34} {n}");
        }
        let _ = writeln!(
            f,
            "pre-pass-provable declines {} ({:?} of read time, {:?} of it enclosing)",
            self.prepass, self.prepass_time, self.prepass_enclose
        );
        let _ = writeln!(
            f,
            "digest memo: {} repeats ({:?} of read time)",
            self.repeats, self.repeat_time
        );
        for (d, (asked, settled)) in &self.depth {
            let _ = writeln!(
                f,
                "  enclosable at depth {d}: asked {asked} settled {settled}"
            );
        }
        f
    }
}

// The install / take scaffold is `report`'s, spelled again: two
// `Cell`s and a `RefCell` of a different payload, which is less than a
// generic would cost to name. `report.rs` says the same at its copy.
thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static NOTE: Cell<Option<FreezeCause>> = const { Cell::new(None) };
    static ORIGIN: Cell<Origin> = const { Cell::new(Origin::Decision) };
    static PROFILE: RefCell<SymProfile> = RefCell::new(SymProfile::default());

    /// The `Opaque` ids the session now installed has minted, flushed
    /// into [`SymProfile::opaque_ids`] when it ends.
    static OPAQUE_LEAF: RefCell<BTreeSet<u128>> = const { RefCell::new(BTreeSet::new()) };

    /// **Which RETRY attempt is running** (`sym::SymRetry`) — 0 for the
    /// first, `k` inside the `k`th rung of the ladder. Read by
    /// [`Walk::charged`], so a retry's forms land in their own buckets
    /// and the first attempt's rows are what they were.
    static ATTEMPT: Cell<u8> = const { Cell::new(0) };

    /// The decision forms this session's reads have already asked, by
    /// digest ([`ReadProfile::repeats`]).
    static READ_SEEN: RefCell<BTreeSet<u128>> = const { RefCell::new(BTreeSet::new()) };

    /// Whether the read running was asked by `signed::order`.
    static IN_ORDER: Cell<bool> = const { Cell::new(false) };

    /// Whether the instrument is re-enclosing a half ([`read_note`]
    /// records only then), the first refusal noted since it began, and
    /// the deepest atom level entered.
    static CLASSIFYING: Cell<bool> = const { Cell::new(false) };
    static READ_NOTE: RefCell<Option<String>> = const { RefCell::new(None) };
    static READ_DEEPEST: Cell<usize> = const { Cell::new(0) };

    /// How deep inside rule G the thread is ([`RootProfile::own`]
    /// clocks the outermost entry), the branch the call running last
    /// named, and the argument digests this session's `canonical` has
    /// seen ([`RootProfile::repeats`]).
    static ROOT_DEPTH: Cell<u32> = const { Cell::new(0) };
    static ROOT_BRANCH: Cell<Option<&'static str>> = const { Cell::new(None) };
    static ROOT_SEEN: RefCell<BTreeSet<u128>> = const { RefCell::new(BTreeSet::new()) };
}

/// Installs the profile on this thread, dropping anything recorded.
pub fn start_profile() {
    PROFILE.with(|p| *p.borrow_mut() = SymProfile::default());
    ACTIVE.set(true);
}

/// Removes the profile and answers everything recorded since
/// [`start_profile`] — or, if it was not installed, whatever a hook
/// recorded regardless, which is nothing when every hook is gated.
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

/// Sets the origin the next walks are charged to, answering the one
/// it replaces so the caller restores it.
#[inline]
pub(super) fn set_origin(origin: Origin) -> Origin {
    ORIGIN.replace(origin)
}

/// Sets the retry attempt the next walks are charged to, answering the
/// one it replaces so the caller restores it.
#[inline]
pub(super) fn set_attempt(attempt: u8) -> u8 {
    ATTEMPT.replace(attempt)
}

/// **Opens a DECISION's record**, answering the mark the freezes it
/// makes start at — `sym::discharge_in` calls this before the ladder
/// and hands the mark back to [`decision_end`].
///
/// The mark is an index into [`SymProfile::freezes`] and not a counter
/// of its own, so the causes a decision is charged with are exactly the
/// freeze sites recorded between the two calls: nothing is attributed
/// twice and nothing needs a second hook at the refusal sites.
#[must_use]
pub(super) fn decision_begin() -> usize {
    let mut mark = 0;
    with(|p| mark = p.freezes.len());
    mark
}

/// **Closes a decision's record**: the rung and attempt that answered
/// (`None` for a refusal), and the freezes this decision's own walks
/// made, by cause.
pub(super) fn decision_end(mark: usize, answered: Option<(Rung, u8)>) {
    let origin = ORIGIN.get();
    with(|p| {
        let mut causes: BTreeMap<FreezeCause, u64> = BTreeMap::new();
        for f in &p.freezes[mark.min(p.freezes.len())..] {
            *causes.entry(f.cause).or_default() += 1;
        }
        p.decisions.push(DecisionRecord {
            origin,
            answered,
            causes,
        });
    });
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
    let terms = f.num.terms().len() > budget.max_terms || f.den.terms().len() > budget.max_terms;
    NOTE.set(Some(if terms {
        FreezeCause::Terms
    } else {
        FreezeCause::Degree
    }));
}

/// Clears the note before a node is combined, so a refusal a recovered
/// path made earlier (a fold that fell back to an atom) is never read
/// as this node's cause. The note is a thread-local side channel
/// because the refusal sites answer `None` and nothing else; the
/// convention it holds by is that `form_in` clears it before each
/// `combine` and reads it once, at the freeze, and no site reads it
/// otherwise.
#[inline]
pub(super) fn clear_note() {
    if active() {
        NOTE.set(None);
    }
}

fn terms(s: FormSize) -> u64 {
    (s.num.0 + s.den.0) as u64
}

fn degree(s: FormSize) -> u32 {
    s.num.1.max(s.den.1)
}

/// Records one node the walk visited: its kids' forms and what came of
/// it — a form, or a freeze whose cause is the last note — and the time
/// since `t0`, taken before the node was combined.
pub(super) fn record_node(
    op: SymOp,
    walk: Walk,
    id: u128,
    kids: [&Form; 3],
    made: Option<&Form>,
    t0: Option<Instant>,
) {
    if !active() {
        return;
    }
    let dt = elapsed(t0);
    let walk = walk.charged();
    let arity = op.arity();
    let sizes = [
        (arity >= 1).then(|| size_of(kids[0])),
        (arity >= 2).then(|| size_of(kids[1])),
        (arity >= 3).then(|| size_of(kids[2])),
    ];
    let tag = op.tag();
    let cause = NOTE.take();
    let origin = ORIGIN.get();
    with(|p| {
        let o = p.ops.entry(tag).or_insert_with(|| OpProfile {
            op: format!("{op:?}"),
            ..OpProfile::default()
        });
        for s in sizes.into_iter().flatten() {
            o.terms_in += terms(s);
            o.degree_in += u64::from(degree(s));
        }
        o.time += dt;
        let n = p.node_costs.entry((walk, id)).or_default();
        n.op = tag;
        n.visits += 1;
        n.time += dt;
        match made {
            Some(f) => {
                let s = size_of(f);
                n.terms += terms(s);
                n.degree += u64::from(degree(s));
            }
            None => n.frozen += 1,
        }
        let w = p.walks.entry((walk, origin)).or_default();
        w.forms += 1;
        match made {
            Some(f) => {
                let s = size_of(f);
                w.digest = chain(w.digest, f.digest());
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
                    op: Some(tag),
                    cause: cause.unwrap_or(FreezeCause::Unnoted),
                    walk,
                    origin,
                    kids: sizes,
                });
            }
        }
    });
}

/// Records a freeze of a node the session never recorded.
pub(super) fn record_unrecorded(walk: Walk) {
    let walk = walk.charged();
    let origin = ORIGIN.get();
    with(|p| {
        let w = p.walks.entry((walk, origin)).or_default();
        w.forms += 1;
        w.frozen += 1;
        p.freezes.push(FreezeSite {
            op: None,
            cause: FreezeCause::Unrecorded,
            walk,
            origin,
            kids: [None, None, None],
        });
    });
}

/// Records one call to a walk and how long it took.
pub(super) fn walk_done(walk: Walk, t0: Option<Instant>) {
    let walk = walk.charged();
    let dt = elapsed(t0);
    let origin = ORIGIN.get();
    with(|p| {
        let w = p.walks.entry((walk, origin)).or_default();
        w.calls += 1;
        w.time += dt;
    });
}

/// Records the forms each retry attempt's memos hold as a session
/// ends, keeping the largest per attempt ([`SymProfile::retry_forms`]).
pub(super) fn retry_memos(sizes: &[usize]) {
    with(|p| {
        if p.retry_forms.len() < sizes.len() {
            p.retry_forms.resize(sizes.len(), 0);
        }
        for (acc, n) in p.retry_forms.iter_mut().zip(sizes) {
            *acc = (*acc).max(*n);
        }
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

/// One session is about to run: the per-session accumulators start
/// empty, so a mint outside any session cannot land in a leaf's set.
pub(super) fn session_start() {
    if active() {
        OPAQUE_LEAF.with(|s| s.borrow_mut().clear());
        READ_SEEN.with(|s| s.borrow_mut().clear());
        ROOT_SEEN.with(|s| s.borrow_mut().clear());
    }
}

/// One session ended, with this many nodes and atoms in its table.
pub(super) fn session_done(nodes: usize, atoms: usize) {
    let opaque = OPAQUE_LEAF.with(|s| core::mem::take(&mut *s.borrow_mut()));
    with(|p| {
        p.sessions += 1;
        p.nodes += nodes as u64;
        p.atoms += atoms as u64;
        p.opaque_ids.push(opaque);
    });
}

/// `signed::order` opened (`true`) or closed (`false`) a read: the
/// calls inside are charged to `min`/`max`.
pub(super) fn read_in_order(on: bool) {
    IN_ORDER.set(on);
}

/// `signed::order` built (or refused) its difference.
pub(super) fn read_order(t0: Option<Instant>, refused: bool) {
    with(|p| {
        p.read.order += 1;
        p.read.order_diff += elapsed(t0);
        p.read.order_refused += u64::from(refused);
    });
}

/// One half's positive content stripped.
pub(super) fn read_strip(t0: Option<Instant>) {
    with(|p| p.read.strip += elapsed(t0));
}

/// One half enclosed.
pub(super) fn read_enclose(t0: Option<Instant>) {
    with(|p| p.read.enclose += elapsed(t0));
}

/// The enclosure time so far — the mark [`read_done`] reads a call's
/// own enclosure time against.
pub(super) fn read_enclose_mark() -> Duration {
    let mut d = Duration::ZERO;
    with(|p| d = p.read.enclose);
    d
}

/// One read finished: the read's own time, its form's digest, and
/// what the instrument learned about it.
pub(super) fn read_done(spent: Duration, mark: Duration, digest: u128, class: &ReadClass) {
    let repeat = active() && !READ_SEEN.with(|s| s.borrow_mut().insert(digest));
    let in_order = IN_ORDER.get();
    with(|p| {
        let r = &mut p.read;
        if !in_order {
            r.select += 1;
        }
        r.time += spent;
        match &class.cause {
            None => r.settled += 1,
            Some(c) => *r.declines.entry(c.clone()).or_default() += 1,
        }
        if class.prepass {
            r.prepass += 1;
            r.prepass_time += spent;
            r.prepass_enclose += r.enclose.saturating_sub(mark);
        }
        if repeat {
            r.repeats += 1;
            r.repeat_time += spent;
        }
        if let Some(d) = class.depth {
            let e = r.depth.entry(d).or_default();
            e.0 += 1;
            e.1 += u64::from(class.cause.is_none());
        }
    });
}

/// The instrument begins (`true`) or ends (`false`) re-enclosing one
/// half: the note and the depth start empty.
pub(super) fn read_classifying(on: bool) {
    CLASSIFYING.set(on);
    READ_NOTE.with(|n| *n.borrow_mut() = None);
    READ_DEEPEST.set(0);
}

/// **The enclosure refused, and why** — called at each refusal arm of
/// `signed::enclose_indet` / `enclose_deep`. Records only while the
/// instrument is re-enclosing, and keeps the FIRST note: the innermost
/// refusal, which every enclosing level then passes up.
pub(super) fn read_note(cause: impl FnOnce() -> String) {
    if CLASSIFYING.get() {
        READ_NOTE.with(|n| {
            let mut n = n.borrow_mut();
            if n.is_none() {
                *n = Some(cause());
            }
        });
    }
}

/// The enclosure entered an atom `level` levels down (1 for an atom at
/// the top of a half).
pub(super) fn read_entered(level: usize) {
    if CLASSIFYING.get() {
        READ_DEEPEST.set(READ_DEEPEST.get().max(level));
    }
}

/// The note and the deepest level since [`read_classifying`] began.
pub(super) fn read_noted() -> (Option<String>, usize) {
    (READ_NOTE.with(|n| n.borrow().clone()), READ_DEEPEST.get())
}

/// One id the PLAIN walk put a form in its memo for — the distinct
/// count a drive-scoped memo would key by.
pub(super) fn record_plain_id(id: u128) {
    with(|p| {
        p.plain_ids.insert(id);
    });
}

/// One `Opaque` indeterminate this session minted ([`super::Sym::opaque`]).
pub(super) fn record_opaque(id: u128) {
    if active() {
        OPAQUE_LEAF.with(|s| {
            s.borrow_mut().insert(id);
        });
    }
}

/// **Rule G is entered** — `root::canonical` or the `abs` node's door:
/// the clock the matching `root_*_done` reads, one level deeper.
pub(super) fn root_begin() -> Option<Instant> {
    if !active() {
        return None;
    }
    if ROOT_DEPTH.get() == 0 {
        ROOT_BRANCH.set(None);
    }
    ROOT_DEPTH.set(ROOT_DEPTH.get() + 1);
    Some(Instant::now())
}

/// One level out of rule G; `true` where the entry closing was the
/// outermost.
fn root_end() -> bool {
    let d = ROOT_DEPTH.get().saturating_sub(1);
    ROOT_DEPTH.set(d);
    d == 0
}

/// **The branch the running `canonical` call took** — named at the arm
/// that answered or declined; the last name before the call returns
/// is its branch.
#[inline]
pub(super) fn root_note(branch: &'static str) {
    if active() {
        ROOT_BRANCH.set(Some(branch));
    }
}

/// The branch noted so far — the split reads it before it asks the
/// halves' roots, and [`root_renote`] puts it back once they answered.
pub(super) fn root_noted() -> Option<&'static str> {
    ROOT_BRANCH.get()
}

/// Puts back a branch [`root_noted`] read.
pub(super) fn root_renote(branch: Option<&'static str>) {
    if active() {
        ROOT_BRANCH.set(branch);
    }
}

/// One `root::canonical` call finished: its whole time under the branch
/// it noted, whether its argument repeated one this session already
/// asked, and the size of what it answered.
pub(super) fn root_canonical_done(t0: Option<Instant>, arg: u128, out: Option<&Form>) {
    let Some(start) = t0 else { return };
    let dt = start.elapsed();
    let outermost = root_end();
    let branch = ROOT_BRANCH.take().unwrap_or("unnoted");
    let repeat = !ROOT_SEEN.with(|s| s.borrow_mut().insert(arg));
    let size = out.map(|f| (size_of(f), f.den.as_constant().is_none()));
    with(|p| {
        let r = &mut p.root;
        let b = r.branches.entry(branch).or_default();
        b.calls += 1;
        b.time += dt;
        if outermost {
            r.own.calls += 1;
            r.own.time += dt;
        }
        if repeat {
            r.repeats.calls += 1;
            r.repeats.time += dt;
        }
        if let Some((s, quotient)) = size {
            r.answered += 1;
            r.out_terms += terms(s);
            r.out_degree += u64::from(degree(s));
            r.out_quotients += u64::from(quotient);
        }
    });
}

/// One `abs` node's atom door (`root::magnitude_atom` from `combine`)
/// finished.
pub(super) fn root_abs_done(t0: Option<Instant>) {
    let Some(start) = t0 else { return };
    let dt = start.elapsed();
    let outermost = root_end();
    with(|p| {
        timed(&mut p.root.abs_door, Some(start));
        if outermost {
            p.root.own.calls += 1;
            p.root.own.time += dt;
        }
    });
}

/// One part inside rule G finished, and whether it succeeded.
pub(super) fn root_part(part: &'static str, t0: Option<Instant>, hit: bool) {
    with(|p| {
        let e = p.root.parts.entry(part).or_default();
        timed(&mut e.0, t0);
        e.1 += u64::from(hit);
    });
}

/// The kids' sizes over one `(cause, op)` class of freezes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FreezeSummary {
    /// Freezes in the class.
    pub count: u64,
    /// Kid slots summarised (up to two per freeze).
    pub kids: u64,
    /// The smallest and largest kid total degree, `None` with no kid.
    pub degree_range: Option<(u32, u32)>,
    /// The sum of kid total degrees (for the mean).
    pub degree_sum: u64,
    /// The fewest and most kid terms (numerator plus denominator),
    /// `None` with no kid.
    pub terms_range: Option<(u64, u64)>,
    /// The sum of kid terms (for the mean).
    pub terms_sum: u64,
}

fn widen<T: Ord + Copy>(range: Option<(T, T)>, v: T) -> Option<(T, T)> {
    Some(range.map_or((v, v), |(lo, hi)| (lo.min(v), hi.max(v))))
}

impl SymProfile {
    /// The name of an op the profile saw, by its tag; `"?"` for an
    /// unrecorded node or a tag it never saw.
    #[must_use]
    pub fn op_name(&self, op: Option<u64>) -> &str {
        op.and_then(|t| self.ops.get(&t))
            .map_or("?", |o| o.op.as_str())
    }

    /// Freezes by `(cause, op name)`, with the kids' total degrees and
    /// term counts summarised over every kid slot at those freezes.
    #[must_use]
    pub fn freezes_by_cause(&self) -> BTreeMap<(FreezeCause, String), FreezeSummary> {
        let mut out: BTreeMap<(FreezeCause, String), FreezeSummary> = BTreeMap::new();
        for f in &self.freezes {
            let s = out
                .entry((f.cause, self.op_name(f.op).to_owned()))
                .or_default();
            s.count += 1;
            for k in f.kids.into_iter().flatten() {
                let d = degree(k);
                let t = terms(k);
                s.degree_range = widen(s.degree_range, d);
                s.terms_range = widen(s.terms_range, t);
                s.degree_sum += u64::from(d);
                s.terms_sum += t;
                s.kids += 1;
            }
        }
        out
    }

    /// Frozen nodes in total, every walk and origin.
    #[must_use]
    pub fn frozen(&self) -> u64 {
        self.freezes.len() as u64
    }

    /// Freezes whose cause no refusal site noted — zero unless a hook
    /// is missing.
    #[must_use]
    pub fn unnoted(&self) -> u64 {
        self.freezes
            .iter()
            .filter(|f| f.cause == FreezeCause::Unnoted)
            .count() as u64
    }

    /// One walk's totals summed over every origin.
    #[must_use]
    pub fn walk(&self, walk: Walk) -> WalkProfile {
        let mut out = WalkProfile::default();
        for ((w, _), p) in &self.walks {
            if *w == walk {
                out.absorb(*p);
            }
        }
        out
    }

    /// **WHERE THE REFUSALS ARE** — one line per `(origin, rung,
    /// attempt)` with the decisions it answered, then one line for the
    /// decisions EVERY rung of every attempt refused, with the freeze
    /// causes those decisions' own walks made
    /// ([`DecisionRecord::causes`] carries what that attribution does
    /// and does not say).
    ///
    /// No clock on it, so it is the same text on every box and a row
    /// can pin it.
    #[must_use]
    pub fn rung_table(&self) -> String {
        use core::fmt::Write as _;
        let mut answered: BTreeMap<(Origin, Rung, u8), u64> = BTreeMap::new();
        let mut refused: BTreeMap<Origin, u64> = BTreeMap::new();
        let mut causes: BTreeMap<(Origin, FreezeCause), u64> = BTreeMap::new();
        for d in &self.decisions {
            match d.answered {
                Some((rung, attempt)) => {
                    *answered.entry((d.origin, rung, attempt)).or_default() += 1;
                }
                None => {
                    *refused.entry(d.origin).or_default() += 1;
                    for (cause, n) in &d.causes {
                        *causes.entry((d.origin, *cause)).or_default() += n;
                    }
                }
            }
        }
        let mut f = String::new();
        for ((origin, rung, attempt), n) in &answered {
            let _ = writeln!(f, "{origin:?}/{rung:?}/attempt {attempt} answered {n}");
        }
        for (origin, n) in &refused {
            let _ = writeln!(f, "{origin:?}/refused {n}");
        }
        for ((origin, cause), n) in &causes {
            let _ = writeln!(f, "{origin:?}/refused froze {cause:?} {n}");
        }
        f
    }

    /// **The walk ledger**: one line per walk and origin — calls, forms,
    /// frozen, and the digest chain of the forms built — with no clock
    /// on it, so it is the same text on every box and a row can pin
    /// it. `frozen` and the digest are both of the plain walk's forms
    /// whoever asked; the pin reads the whole table.
    #[must_use]
    pub fn walk_ledger(&self) -> String {
        use core::fmt::Write as _;
        let mut f = String::new();
        for ((w, o), p) in &self.walks {
            let _ = writeln!(
                f,
                "{w:?}/{o:?} calls {} forms {} frozen {} digest {:032x}",
                p.calls, p.forms, p.frozen, p.digest
            );
        }
        f
    }

    /// **The same nodes under two profiles** — `self` against `other`
    /// (rule G on against rule G off, say), joined on `(walk, node id)`.
    /// Per walk: the nodes both built and their summed sizes and times
    /// under each, how many grew and shrank, and the nodes only one of
    /// them built with their time; then, over the aligned nodes, the
    /// time and size per op under each, largest time difference first.
    #[must_use]
    pub fn node_delta(&self, other: &Self) -> String {
        use core::fmt::Write as _;
        #[derive(Default)]
        struct Side {
            nodes: u64,
            visits: u64,
            terms: u64,
            degree: u64,
            time: Duration,
        }
        impl Side {
            fn add(&mut self, n: &NodeCost) {
                self.nodes += 1;
                self.visits += n.visits;
                self.terms += n.terms;
                self.degree += n.degree;
                self.time += n.time;
            }
        }
        let mut f = String::new();
        let walks: BTreeSet<Walk> = self
            .node_costs
            .keys()
            .chain(other.node_costs.keys())
            .map(|(w, _)| *w)
            .collect();
        for walk in walks {
            let (mut a, mut b) = (Side::default(), Side::default());
            let (mut only_a, mut only_b) = (Side::default(), Side::default());
            let (mut grew, mut shrank) = (0u64, 0u64);
            let mut per_op: BTreeMap<u64, (Side, Side)> = BTreeMap::new();
            for ((w, id), n) in &self.node_costs {
                if *w != walk {
                    continue;
                }
                match other.node_costs.get(&(walk, *id)) {
                    Some(m) => {
                        a.add(n);
                        b.add(m);
                        let (x, y) = (n.terms * m.visits, m.terms * n.visits);
                        grew += u64::from(x > y);
                        shrank += u64::from(x < y);
                        let e = per_op.entry(n.op).or_default();
                        e.0.add(n);
                        e.1.add(m);
                    }
                    None => only_a.add(n),
                }
            }
            for ((w, id), m) in &other.node_costs {
                if *w == walk && !self.node_costs.contains_key(&(walk, *id)) {
                    only_b.add(m);
                }
            }
            let _ = writeln!(
                f,
                "{walk:?}: aligned {} nodes (visits {} / {}): terms {} / {} (mean {:.2} / {:.2}), \
                 degree {} / {}, time {:?} / {:?}; {grew} grew, {shrank} shrank; \
                 only here {} nodes in {:?}, only there {} nodes in {:?}",
                a.nodes,
                a.visits,
                b.visits,
                a.terms,
                b.terms,
                mean(a.terms, a.visits),
                mean(b.terms, b.visits),
                a.degree,
                b.degree,
                a.time,
                b.time,
                only_a.nodes,
                only_a.time,
                only_b.nodes,
                only_b.time,
            );
            let mut ops: Vec<(u64, (Side, Side))> = per_op.into_iter().collect();
            ops.sort_by(|(_, (x, y)), (_, (u, v))| {
                let d1 = x.time.as_secs_f64() - y.time.as_secs_f64();
                let d2 = u.time.as_secs_f64() - v.time.as_secs_f64();
                d2.total_cmp(&d1)
            });
            for (tag, (x, y)) in ops {
                let _ = writeln!(
                    f,
                    "  {:8} {:7} nodes: time {:?} / {:?}, mean terms {:.2} / {:.2}, \
                     mean degree {:.2} / {:.2}",
                    self.op_name(Some(tag)),
                    x.nodes,
                    x.time,
                    y.time,
                    mean(x.terms, x.visits),
                    mean(y.terms, y.visits),
                    mean(x.degree, x.visits),
                    mean(y.degree, y.visits),
                );
            }
        }
        f
    }

    /// The tables, as text: totals; each walk by origin; the early
    /// walk's inner clocks; the ring; per op; freezes by cause and op.
    #[must_use]
    pub fn render(&self) -> String {
        use core::fmt::Write as _;
        let mut f = String::new();
        let _ = writeln!(
            f,
            "sessions {}  nodes {}  atoms {}  frozen {}  unnoted {}",
            self.sessions,
            self.nodes,
            self.atoms,
            self.frozen(),
            self.unnoted()
        );
        let _ = writeln!(
            f,
            "walk   | origin     | calls | forms | frozen | time | digest"
        );
        for ((w, o), p) in &self.walks {
            let _ = writeln!(
                f,
                "{:6} | {:10} | {:5} | {:5} | {:6} | {:?} | {:032x}",
                format!("{w:?}"),
                format!("{o:?}"),
                p.calls,
                p.forms,
                p.frozen,
                p.time,
                p.digest
            );
        }
        let _ = writeln!(
            f,
            "early: reduce_steps {} calls {:?}; trig::fold {} calls {:?}; top reduce {} calls {:?}",
            self.reduce.calls,
            self.reduce.time,
            self.trig.calls,
            self.trig.time,
            self.reduce_top.calls,
            self.reduce_top.time
        );
        let _ = writeln!(
            f,
            "ring: rat ops {}  big-path int ops {}  promotions {}  widest coefficient kept {} bits, refused {} bits",
            self.rat_ops, self.big_ops, self.promotions, self.widest_bits, self.widest_refused_bits
        );
        let _ = writeln!(
            f,
            "op       | built | frozen | terms in->out (mean) | degree in->out (mean) | max terms | max degree | time"
        );
        let mut ops: Vec<&OpProfile> = self.ops.values().collect();
        ops.sort_by(|a, b| a.op.cmp(&b.op));
        for o in ops {
            let n = o.built + o.frozen;
            let _ = writeln!(
                f,
                "{:8} | {:5} | {:6} | {:8.1} -> {:8.1} | {:6.1} -> {:6.1} | {:9} | {:10} | {:?}",
                o.op,
                o.built,
                o.frozen,
                mean(o.terms_in, n),
                mean(o.terms_out, o.built),
                mean(o.degree_in, n),
                mean(o.degree_out, o.built),
                o.max_terms_out,
                o.max_degree_out,
                o.time
            );
        }
        if !self.freezes.is_empty() {
            let mut by_origin: BTreeMap<(Origin, Walk, FreezeCause), u64> = BTreeMap::new();
            for site in &self.freezes {
                *by_origin
                    .entry((site.origin, site.walk, site.cause))
                    .or_default() += 1;
            }
            let _ = writeln!(f, "freezes by origin | walk | cause | count");
            for ((o, w, c), n) in by_origin {
                let _ = writeln!(
                    f,
                    "{:10} | {:6} | {:12} | {:5}",
                    format!("{o:?}"),
                    format!("{w:?}"),
                    format!("{c:?}"),
                    n
                );
            }
            let _ = writeln!(
                f,
                "freeze cause | op       | count | kid degree min/mean/max | kid terms min/mean/max"
            );
            for ((cause, op), s) in self.freezes_by_cause() {
                let (dlo, dhi) = s.degree_range.unwrap_or((0, 0));
                let (tlo, thi) = s.terms_range.unwrap_or((0, 0));
                let _ = writeln!(
                    f,
                    "{:12} | {:8} | {:5} | {:3} / {:6.1} / {:3} | {:4} / {:7.1} / {:4}",
                    format!("{cause:?}"),
                    op,
                    s.count,
                    dlo,
                    mean(s.degree_sum, s.kids),
                    dhi,
                    tlo,
                    mean(s.terms_sum, s.kids),
                    thi
                );
            }
        }
        f
    }
}

fn mean(sum: u64, n: u64) -> f64 {
    if n == 0 { 0.0 } else { sum as f64 / n as f64 }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod tests {
    use super::*;
    use crate::k_stats::decide;
    use crate::predicate::Margin;
    use crate::real::Real;
    use crate::sym::rational::Rat;
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

    /// Every freeze as `(cause, op, walk, origin)`. A node the plain
    /// walk freezes is frozen AGAIN by the early walk (a second memo,
    /// the same refusal), so under the shipped rules a freeze appears
    /// once per walk — which is why this profile's `frozen()` is not
    /// `SymCounts::frozen`, the plain walk's count alone.
    fn causes(p: &SymProfile) -> Vec<(FreezeCause, String, Walk, Origin)> {
        p.freezes
            .iter()
            .map(|f| (f.cause, p.op_name(f.op).to_owned(), f.walk, f.origin))
            .collect()
    }

    fn per_walk(cause: FreezeCause, op: &str) -> Vec<(FreezeCause, String, Walk, Origin)> {
        vec![
            (cause, op.to_owned(), Walk::Plain, Origin::Decision),
            (cause, op.to_owned(), Walk::Early, Origin::Decision),
        ]
    }

    /// **Uninstalled, the hooks record nothing**: with the profile taken
    /// (not installed), a session that freezes, promotes and walks
    /// leaves the next `take_profile` empty — a hook that wrote while
    /// inactive would show here, because nothing resets the store
    /// between the two takes.
    #[test]
    fn nothing_is_recorded_while_uninstalled() {
        let _ = take_profile();
        let _ = with_session_rules(budget(3, 128), SymRules::shipped(), || {
            let (x, y) = (p("x", 1.0), p("y", 2.0));
            ask((x + y) * (x - y));
        });
        let out = take_profile();
        assert_eq!(out.sessions, 0, "{}", out.render());
        assert!(out.ops.is_empty(), "{}", out.render());
        assert!(
            out.walks.is_empty() && out.freezes.is_empty(),
            "{}",
            out.render()
        );
        assert_eq!((out.rat_ops, out.big_ops, out.widest_bits), (0, 0, 0));
    }

    /// **A product past the term budget freezes for `Terms`**, on the
    /// `Mul` that asked for it, with the kids' sizes at the freeze —
    /// `(x + y)·(x − y)` has four candidate terms and a budget of three
    /// refuses it before it is built. At `x = y` the margin is
    /// numerically zero, so it is the DECISION that asks the form.
    #[test]
    fn a_term_budget_refusal_is_a_terms_freeze_on_the_product() {
        let out = profiled(budget(3, 128), || {
            let (x, y) = (p("x", 1.0), p("y", 1.0));
            ask((x + y) * (x - y));
        });
        assert_eq!(out.sessions, 1);
        assert_eq!(
            causes(&out),
            per_walk(FreezeCause::Terms, "Mul"),
            "{}",
            out.render()
        );
        let site = out.freezes[0];
        assert_eq!(site.kids[0].unwrap().num, (2, 1), "x + y");
        assert_eq!(site.kids[1].unwrap().num, (2, 1), "x − y");
        assert_eq!(out.ops[&SymOp::Mul.tag()].frozen, 2, "once per walk");
        assert_eq!(out.walk(Walk::Plain).frozen, 1);
        assert_eq!(out.walk(Walk::Early).frozen, 1);
        assert_eq!(out.frozen(), 2);
        assert_eq!(out.unnoted(), 0);
    }

    /// **A product past the degree budget freezes for `Degree`** —
    /// `x·x·x` at a degree budget of two — and the kid degrees at the
    /// freeze are the two operands', `x²` and `x`.
    #[test]
    fn a_degree_budget_refusal_is_a_degree_freeze_with_the_kids_degrees() {
        let out = profiled(budget(4096, 2), || {
            // Numerically zero, so the decision asks the form.
            let x = p("x", 0.0);
            ask(x * x * x);
        });
        assert_eq!(
            causes(&out),
            per_walk(FreezeCause::Degree, "Mul"),
            "{}",
            out.render()
        );
        let site = out.freezes[0];
        let degrees: Vec<u32> = site.kids.iter().flatten().map(|k| k.num.1).collect();
        assert_eq!(degrees, vec![2, 1]);
        let s = out.freezes_by_cause()[&(FreezeCause::Degree, "Mul".to_owned())];
        assert_eq!(s.degree_range, Some((1, 2)));
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
            // Numerically zero, so the decision asks the form.
            let x = p("x", 0.0);
            ask(x * m * m * m * m * m);
        });
        assert_eq!(
            causes(&out),
            per_walk(FreezeCause::Coefficient, "Mul"),
            "{}",
            out.render()
        );
        assert_eq!(out.widest_refused_bits, 265, "{}", out.render());
        assert_eq!(out.widest_bits, 212, "four mantissas: {}", out.render());
        assert!(
            out.promotions >= 1,
            "the third product leaves i128: {}",
            out.render()
        );
    }

    /// **The walks and the ring are counted, and a walk knows its
    /// origin**: a theorem asked of the plain form is one plain-walk
    /// call under `Decision` with its forms and no early walk; a
    /// definite margin's forms are the ASSERTION's, under debug
    /// assertions, and are charged to it.
    #[test]
    fn the_walks_and_the_ring_are_counted_by_origin() {
        let out = profiled(budget(4096, 128), || {
            let (x, y) = (p("x", 1.0), p("y", 2.0));
            ask(x * y - y * x);
        });
        let plain = out.walks[&(Walk::Plain, Origin::Decision)];
        assert_eq!(plain.calls, 1);
        // x, y, x·y, y·x and the difference: five nodes, five forms.
        assert_eq!(plain.forms, 5, "{}", out.render());
        assert!(
            out.walks
                .keys()
                .all(|(w, o)| *w == Walk::Plain && *o == Origin::Decision),
            "{}",
            out.render()
        );
        assert_eq!(out.ops[&SymOp::Mul.tag()].built, 2);
        assert_eq!(out.ops[&SymOp::Sub.tag()].op, "Sub");
        assert_eq!(out.nodes, 5);
        assert!(out.rat_ops > 0 && out.promotions == 0, "{}", out.render());
        assert_eq!(out.frozen(), 0);

        // A definite sign: the decision path builds no form; the
        // contradiction assertion does (dev and test profiles), and the
        // profile charges every walk it asks to `Assertion`.
        let out = profiled(budget(4096, 128), || {
            let (x, y) = (p("x", 1.0), p("y", 2.0));
            ask(x * y + y * x);
        });
        let decision = out.walks.keys().any(|(_, o)| *o == Origin::Decision);
        assert!(!decision, "{}", out.render());
        if cfg!(debug_assertions) {
            assert_eq!(
                out.walks[&(Walk::Plain, Origin::Assertion)].forms,
                5,
                "{}",
                out.render()
            );
            assert_eq!(
                out.walks[&(Walk::Early, Origin::Assertion)].calls,
                1,
                "{}",
                out.render()
            );
        } else {
            assert!(out.walks.is_empty(), "{}", out.render());
        }
    }

    /// **A zero divisor is a noted cause**: the reciprocal of the zero
    /// coefficient reaches `Rat::from_parts` with a zero denominator,
    /// which notes `ZeroDivisor`, so no freeze it causes reads as
    /// `Unnoted`.
    #[test]
    fn a_zero_denominator_is_noted_as_a_zero_divisor() {
        start_profile();
        clear_note();
        assert!(Rat::zero().recip().is_none());
        assert_eq!(NOTE.get(), Some(FreezeCause::ZeroDivisor));
        let _ = take_profile();
    }
}
