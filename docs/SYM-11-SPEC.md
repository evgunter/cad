# SYM-11 — the point channel is not a proof: the theorem-vs-numeric contradiction charged per witness kind (spec)

**Program:** SYM (`work/sym/plan.md`, first in its order). **Item:**
`work/sym/sym-f64-far-placement-trips-the-theorem-vs-numeric-assert.md`
(both mechanisms: the far placement's rounding, and rule F's sign
amplification). **Track:** protocol v7 **IN** — an architectural
decision on the tier's `Decide` door (which channel a contradiction
between the form and the value is charged to, per lane scalar), the
same decision the registered-identity door made for its witness
(SYM-6: `Contradicted` from an exact witness, `Disputed` from an
inexact one) now made for the theorem channels. The full v6 dual.
Block SYM-B3 slot 0. **Pre-draw fields, logged before the draw:**
difficulty **H**, task-class **STRUCTURAL**.

- **H** — the change sits on the door every decision goes through,
  its soundness argument is re-stated per witness kind, and the
  receipt grows a column every consumer of `SymCounts` must know the
  side of.
- **STRUCTURAL** — no decision at `Sym<Interval>` moves (every pin is
  bit-identical); what moves is the door's contract at the inexact
  scalars, where today it panics.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item whole (the far placement, R2's adversary, R1's pole);
`crates/geom-core/src/sym.rs` — the `impl<T: Decide> Decide for Sym<T>`
and the doc block above it ("the numeric channel runs FIRST, always";
"a definite non-zero numeric answer short-circuits the form"; the
`debug_assert!` and its comment "the enclosure proves it"), then
`Sym::register_equal` and its doc (the witness first; `Contradicted`
vs `Disputed` forwarded from the lane scalar), `SymRegistration`,
`SymCounts` (every column's doc, and `registrations_refused` /
`registrations_contradicted` — the refusal columns that are not
discharge kinds), `count_registration_contradicted`;
`crates/geom-core/src/real.rs` — `Real::register_equal`'s contract and
the per-scalar impls (`:95–140` the two refusal arms; `:305` the exact
witness; `:1601` the inexact one) — this is the partition the unit
makes a declared property; `crates/geom-core/src/predicate.rs` —
`Decide` (and `enclosure_probe`'s clause "an instrument, not a
decision channel: nothing in the funnel may branch on it" — the marker
this unit adds is NOT that), `Decide for f64`;
`crates/geom-core/src/k_stats.rs` — `Probe`, `Decide for Probe`,
`retag_at`; `crates/geom-core/src/dual.rs`'s `Decide`;
`crates/geom-core/src/sym/discharge_pins.rs` (DECIDE-2: a receipt
column added without a side declared in `NOT_A_DISCHARGE_KIND` reds
there — that is the pin doing its job; the row says which side and
why); the rows: `crates/geom-core/tests/sym_rule_f_rows.rs`'s
`the_adversary_a_positive_form_whose_f64_channel_reads_negative`
(`#[ignore]`d because it fires the assertion),
`the_adversary_at_the_interval_lift_is_a_plain_theorem`,
`a_manifestly_positive_form_undefined_inside_the_box`; the
`m10_3_r1_probes_interval` receipt-identity row (the serialized
receipt must not move); `m10_8`/`m10_9`/`m10_10`/`m10_bulge` pins;
`crates/geom-core/README.md` if it carries the door's clauses.

## The claim

At `Sym<Interval>` a definite non-zero sign is a certified proof that
the margin is not zero, so a form that is the zero polynomial
contradicts it, and the assertion is right: one of the two channels is
unsound and a panic is the honest answer. At `Sym<f64>`, `Sym<Probe>`
and any other lane scalar whose witness is INEXACT, a definite sign is
a point comparison: at a far placement the rounding exceeds the band
(the item's first mechanism); under rule F a one-ulp error in a sign
argument becomes a whole `2.0` at the margin (the second); and at a
pole the point channel has no clause 1 to refuse with. None of these
is a proof, the form's theorem is correct wherever the function is
defined, and the contradiction is a DISPUTE — the same thing
`Real::register_equal` already answers for a registration at those
scalars. **The unit's thesis: the assertion's premise holds at exactly
the scalars whose witness is exact, and the tier should charge the
contradiction by that partition — PROVED at an exact witness (assert,
as today), DISPUTED at an inexact one (counted on the receipt, never a
panic, the numeric answer kept).** The ratified order stands: the
numeric channel runs first and a definite non-zero answer
short-circuits the form; no shipped lane replays at an inexact `Sym`,
so this is the unit-test lane's contract made honest, not a change to
what ships.

**Ratified and not re-litigated:** E12; numeric-first and the
definite-non-zero shortcut; clause 1; the registered-identity door's
witness contract (`Contradicted` / `Disputed` / `Unwitnessed`, SYM-6);
rules A–F; D9 (a decision is a function of the value channel's
answer, never of an instrument read); DECIDE-2's pins.

## Phase 1 — before touching anything (the measurement)

1. **The two mechanisms, reproduced and counted.** At `Sym<f64>` and
   `Sym<Probe>`: the far placement (a stadium extruded and the M10-9
   washer revolved on a sketch plane at `(d, d, d)`, `d ∈ {1e6, 1e9}`,
   at ε = default, 1e-6, 1e-12 — the item's rows, re-homed if their
   file moved) and rule F's adversary (`E` at the six sampled `x`).
   One table: document, ε, scalar, the decisions that would trip the
   assertion (count, predicate, margin, the form's discharge kind),
   and whether the bare-`f64` lift refuses that document typed
   downstream anyway (the item says `ResidualExceeded` does). Each
   point on its own thread while the assertion still fires.
2. **The pole.** `copysign(1, 1/(t − 1)²) − 1` at `t = 1`: what each
   scalar answers (`f64`: theorem where the function is undefined;
   `Interval`: `refused Invalid`) — recorded as the reason the
   inexact channel cannot be given the form's answer without a clause
   1 of its own, and why the numeric answer is kept at those scalars.
3. **The exact channel never trips it.** On the six measured
   documents at `Sym<Interval>` (the `m10_8`/`m10_9`/`m10_10` corpus,
   the pad on the ceiling instrument only), at the nominal and at
   ceiling + δ, three ε: the count of definite-non-zero decisions
   whose form discharges is ZERO. A row pins it (the `Interval` twin of
   the dispute count, asserted `== 0`). **Stop clause:** if it is not
   zero anywhere, that is a soundness defect in a channel and a
   different unit; stop, file it with the render, and report.
4. **The partition, written down.** Which lane scalars have an exact
   witness and which an inexact one, read off their
   `Real::register_equal` impls (`Interval`; `f64`, `Probe`,
   `Dual<T>`, anything else that implements `Decide`), and where the
   declared marker should live — on `Decide` (the door's trait) or on
   `Real` beside `register_equal` — with the reason; one home.

## Phase 2 — the contract, per witness kind

- **The marker.** An associated const on the trait Phase 1.4 picks
  (`Witness::{Exact, Inexact}` or the name the code earns), declared
  by every lane scalar, read by `Sym<T>::sign_within` and by nothing
  that decides geometry (it is a property of the scalar, not a value
  read — D9 is untouched; `enclosure_probe` stays the instrument it
  is). `Real::register_equal`'s per-scalar arms cite it, so the two
  contracts cannot drift apart (a pin: an exact witness never answers
  `Disputed`, an inexact one never `Contradicted` — the rows at
  `sym.rs:4043`/`:4186` already say half of this).
- **The charge.** At an exact witness: the assertion as today. At an
  inexact one: the contradiction is COUNTED —
  `SymCounts::theorems_disputed` (or the name the lane earns), doc'd
  as a refusal column beside `registrations_contradicted`, declared
  in `discharge_pins`' `NOT_A_DISCHARGE_KIND` with its reason (not a
  discharge kind, not a K token) — and the numeric answer is returned;
  never a panic. The K sample at `Probe` stays what the base scalar
  recorded (a `Definite` at a point), with the dispute visible on the
  receipt, not in the K token vocabulary.
- **The receipt.** Whether the new column is serialized: the leaf and
  drive receipts ship from `Sym<Interval>` where it is zero by Phase
  1.3, so `m10_3_r1_probes_interval`'s receipt identity must stay
  byte-identical; the lane says whether the column is in the wire
  format (a schema bump is NOT this unit's — if the column cannot be
  added without one, it stays a session receipt column and the row
  says so).
- **The rows.** The far-placement rows and the adversary become
  GATING (no `#[ignore]`, no panic): at `Sym<f64>` the dispute count
  asserted `> 0` with the predicate named; at `Sym<Interval>` on the
  same document `== 0`; the pole row keeps both arms. The Phase 1.3
  `Interval` pin. `m10_8`/`m10_9`/`m10_10`/`m10_bulge` pins and the
  walk ledger bit-identical (a pin that moves is a finding, not a
  re-baseline).
- **The prose.** `sym.rs`'s header: "the enclosure proves it" re-worded
  per witness kind; `SymCounts`'s doc; the item's "what is owed"
  answered; `crates/geom-core/README.md` if it carries the clause.

## Scope

- Files: `crates/geom-core/src/sym.rs` (the `Decide` impl, `SymCounts`,
  the header), `sym/discharge_pins.rs` (the side declaration),
  `predicate.rs` or `real.rs` (the marker — announce: `real.rs` and
  `predicate.rs` are geom-core's core, LINALG's `paths` cover
  `linalg/*` and `interval.rs` only; say what `territory` prints),
  `k_stats.rs` (`Probe`'s marker — PROPS' file, announce),
  `dual.rs`, tests under `crates/geom-core/tests/` and the
  far-placement rows' home.
- No change to the numeric-first order, to any rule, to any decision
  at `Sym<Interval>`, to the K token vocabulary, or to the wire format
  of a receipt. No new tolerance.

## Acceptance

- Phase 1's tables and the partition in the PR body; the stop clause's
  verdict in one sentence.
- No panic at `Sym<f64>`/`Sym<Probe>` on the rows; disputes counted and
  pinned; the `Interval` twin at zero; every existing pin
  bit-identical; the two-contract pin (marker vs `register_equal`).
- Local checks green: `cargo fmt --all -- --check`; clippy
  `-D warnings` on `geom-core` at default, `interval`,
  `interval,sym-profile-testing` and `probe` (all targets), on
  `editor-core --features interval` and on `sweep` if its tests are
  touched; `scripts/doc-gate.sh`; `python3 scripts/work.py lint`. The
  hosted matrix is the verification of record.

## Review

Protocol v7 IN: the full v6 dual — pre-draw fields above, the arm per
block SYM-B3's draw, two blinded reviewers on a frozen green head
(ordinal claimed on `main` at dispatch, SYM's band), the union fix
pass on the implementer's lane, a delta by R1, the row at merge.

## Landing

PR against `main`; the spec deleted at merge with its
`docs/DOC-LEDGER.md` entry; the item's "what is owed" answered and
the item closed or re-cut; the SYM log carries the verdict; the row in
`docs/MODEL-AB-LOG.md` with the v7 triage line. Branch
`sym/11-witness-kind`.
