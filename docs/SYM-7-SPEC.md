# SYM-7 — the plain form outlives the leaf: a drive-scoped memo (spec)

**Program:** SYM (`work/sym/plan.md`, the cost lane). **Item:**
`work/sym/symbolic-tier-costs-95-percent-of-the-m10-3-drive.md` (the
volume ask; D3 = (1) on `[ev]` #2581, Ev's answer 2026-09-14).
**Track:** kernel change — the standard v6 unit (binding spec, drawn
implementer arm, cross-model dual review, union fix pass,
record-at-merge; §Review). Block SYM-B2 slot 0. **Pre-draw fields,
logged before the draw:** difficulty **H**, task-class
**STRUCTURAL**.

- **H** — a second scope for the tier's state, shared across rayon
  workers, with a receipt whose one moved column has to stay
  schedule-independent; the soundness argument is the one the tier
  already makes, applied across leaves, and it has to be pinned.
- **STRUCTURAL** — no certification decision moves: the plain form of
  a node is a function of the node's content hash and the budget
  alone, so every verdict and every decision count is bit-identical;
  only `frozen`'s aggregation changes and it is re-blessed as this
  unit's own move.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item whole (its `## Decision for Ev` and `## Ev's answer` are what this
spec implements — the three side effects are named there); `sym.rs`'s
header sections `# Node ids are CONTENT HASHES (D9)`, `# Freezing`,
`# Cost` and the D9 argument on `OPAQUE_SEQ` (`sym.rs` ~1300); `struct
Session` and `with_session_rules` (~1191, ~1345); the plain walk
(`form_in` and its `frozen` closure, ~1708); `combine` (where atoms are
minted, ~1511/~1649); `discharge`'s top-residual reduce (~1964);
`sym/profile.rs` (`session_done`, the walk ledger);
`crates/editor-core/src/drive.rs` — `DriveConfig`, the level loop
(~1185: `frontier.par_iter().map(leaf)` under `config.parallel`),
`classify` (~1381) and its `with_session_rules` call (~1396), the
K-probe replay (~1783), `serialize()`'s `decisions … frozen=` line
(~783) and `render()`'s (~899); `crates/editor-core/src/eval/mod.rs`
`replay_leaf` (~2338); `crates/editor-core/tests/m10_3_r1_probes_interval.rs`
(`the_driven_chamber_replays_bit_identically_…`, ~452: the receipt
identity across one sequential and two parallel drives — the row this
unit must keep green with `frozen` in the receipt);
`m10_sym_profile_interval.rs` (`sym_profile_slab_drive`,
`sym_profile_callgrind_replay`, the ledger row);
`memories/review-and-dependency-policy.md`.

## The claim

Today the tier's whole state is one `Session` per leaf replay —
the hash-consing table, the plain memo `forms`, the early and door
memos, `atoms`, the registry, `counts` — installed by
`with_session_rules` and dropped with the leaf ("holds nothing across
leaves", D9). On the M10-3 slab a drive is 2,559 sessions, each
interning the same 12,208-node DAG and computing ~10,000 plain forms
for 1,490 decisions; the plain walk is half of every leaf's replay
and `intern` a quarter (SYM-4's callgrind: 99.0 M instructions per
nominal replay; 21.7 M `intern`s over the drive).

**A node's plain form is a function of its id and the budget alone.**
The id is a content hash of `(op, payload, kids)`: a `Param` carries
its symbol, a `Lit` its bits, an `Opaque` the per-leaf sequence
number (the same on every leaf of a drive by D9's fixed single-
threaded walk), an atom the digests of its arguments' forms. The plain
walk reads no value — every atom opaque, rule A0 only — so two leaves
that build a node with one id build the same plain form for it. That
is the argument the tier already makes for two occurrences of a node
inside one leaf; this unit applies it across the leaves of one drive.

**Ratified and not re-litigated:** E12; D9 (content-hashed ids, the
per-leaf opaque sequence, receipts identical under every schedule);
the early and door walks stay per leaf (they consult the leaf's
registry, which a value-dependent refusal can make differ between
leaves, and rule C reads the leaf's `params`); the drive's receipt
identity across schedules (the M10-3 row).

## Phase 1 — before touching anything (the measurement)

1. **The hit rate the memo could have.** Over the slab drive
   (sequential, `CHAMBER_LEAVES`), count per leaf the plain forms
   computed and, over the drive, the DISTINCT ids they were computed
   for; the same for the plate drive at its default. The ratio is the
   ceiling of the win. Instrument through `sym::profile` (the walk
   ledger already counts `calls`/`forms` per walk and origin — add a
   distinct-id counter under `sym-profile-testing` or read it from
   `session_done`); one table in the PR body: document, leaves, plain
   forms per leaf, distinct over the drive, ratio.
2. **The opaque sequence across leaves, by execution.** Over the same
   drive record every leaf's set of `Opaque` ids and assert they are
   identical across leaves. If they are not, STOP and report: the
   premise D9 states does not hold on that document and the memo is
   unsound there until it does. (Read `Sym::opaque`'s callers in
   `crates/editor-core/src/eval/*` for any value-dependent mint before
   running.)
3. **The `frozen` column's readers.** Every reader of `SymCounts::frozen`
   and of the receipt's `frozen=` line (`serialize`, `render`, tests,
   goldens): the list, with which ones compare across schedules.

## Phase 2 — the change

1. **`DriveMemo`** in `geom_core::sym` (a new file `sym/memo.rs`, in
   the split's shape): the plain forms and the atoms they mint, keyed
   by `SymId` / indeterminate id, **shared across the drive's rayon
   workers** so the receipt stays schedule-independent — a
   read-mostly map behind a `RwLock` (or a sharded one of the lane's
   own, no new dependency), with the forms `Arc<Form>` where the memo
   holds them (`Rc` stays inside the leaf's own memos if the cost of
   `Arc` on the leaf's clones is measurable — the lane measures and
   states it). The leaf consults its own `forms` first and the drive
   memo on a miss; a leaf publishes its new plain forms to the drive
   memo at its end (one write lock per leaf, not one per node — say
   so in the header with the reason). **A form the memo hands back is
   the form the leaf would have built**: same budget, same freeze
   points, same atom ids (the memo carries its `AtomInfo`s and the
   leaf's `atoms` is seeded from it on a hit path, so
   `reduce_steps`/`reduce` find them).
2. **The door.** `with_session_rules` gains a memo-carrying spelling
   (`with_session_memo(budget, rules, &DriveMemo, f)` or an `Option`
   on the existing one — the lane picks; the no-memo spelling stays
   bit-for-bit for every other caller: `replay_leaf`, `assertion_at`,
   the K-probe, every test). `DriveMemo::new(budget)` refuses a leaf
   whose budget differs (a `debug_assert!` and a stated invariant:
   the memo is valid for one budget).
3. **The drive installs it** (`drive.rs`, PROPS' file, by announced
   seam — the parameter and nothing else): one `DriveMemo` per
   `drive(...)`, created before the level loop and dropped after it,
   passed into `classify`'s `with_session_rules` call; `DriveConfig`
   gains `plain_memo: bool` (default `true`; the differential dial),
   documented beside `parallel`.
4. **`frozen`'s new meaning.** On the drive's receipt `frozen` is
   **the number of DISTINCT nodes frozen over the drive** — the size
   of the memo's frozen set at the drive's end (a set, so identical
   under every schedule). A leaf's own receipt (`CertifiedLeaf` /
   `RefusedLeaf.decisions`) keeps `symbolic_zero`, `numeric`,
   `registered`, … — the DECISION counts, which are that leaf's — and
   its `frozen` is the frozen nodes THIS leaf computed (zero on a hit
   path); the drive-level `frozen` is not their sum and `absorb`
   does not sum it (say so on `SymCounts::frozen`'s doc, present
   tense, and on `absorb`). The per-leaf and per-drive `frozen=`
   lines in goldens that move are re-blessed with this reason named
   in the commit that moves them (Phase 1.3's list says which).
5. **The header**: D9's "holds nothing across leaves, dropped with the
   leaf" becomes the truth — the hash-consing table and the early/door
   memos are per leaf; the plain memo is per DRIVE when the drive
   installs one, with the soundness argument above and the
   drive-wide hash-collision assumption stated (it was per-leaf).
   `# Cost` carries the new numbers.

## Phase 3 — optional, measured: the hash-consing table too

If Phase 2's measurement leaves `intern` as the largest remaining
share (SYM-4 read it at 24 % of the slab replay), measure sharing the
`nodes` table across the drive on the same argument (content-hashed,
value-free) and the same map, and TAKE it only if the win on the slab
drive is ≥ 10 % of wall time with the receipt identity row green;
otherwise record the number and leave it. Either way the PR body
says which.

## Scope

- Files: `crates/geom-core/src/sym.rs`, `sym/memo.rs` (new),
  `sym/profile.rs` (the instrument); `crates/editor-core/src/drive.rs`
  by announced seam (PROPS'): the memo's creation, the flag, the
  receipt's `frozen` line and its docs — nothing else in that file;
  tests under `crates/editor-core/tests/m10_3_*`, `m10_sym_profile_*`,
  and one new file for the unit's rows (the tests-family overlap with
  S-TCOST/S-TINT is announced).
- No change to what any walk computes, to the early/door memos, to
  the registry, to `SymBudget`/`SymRules`, or to `OPAQUE_SEQ`'s
  per-leaf reset.
- No new dependency for the map (a `RwLock<IdMap<…>>` is enough until
  measured otherwise; a sharded map is the lane's own code if taken).

## Acceptance

- **Every certification decision bit-identical**: the M10-3 receipt
  identity row green (sequential and two parallel drives, `frozen`
  included, now distinct-over-the-drive); the M10-7/8/9/10 pins; the
  goldens except the `frozen=` lines Phase 1.3 named; the walk ledger
  row unchanged (it is one leaf, no memo).
- **The differential pin**: the slab and plate drives with
  `plain_memo` on and off — every leaf verdict and every per-leaf
  decision count identical, the drive's `frozen` equal to the off
  run's distinct frozen ids (compute the set off-line in the row).
- **The opaque-sequence pin** (Phase 1.2 as a row): every leaf of a
  drive mints the same `Opaque` id set; the row explains what breaks
  if it fails.
- **The number**: the slab drive's wall time and the callgrind per-leaf
  replay (SYM-4's instrument), memo on vs off, sequential and
  parallel; the plate the same. The memo's size at the drive's end
  (forms, atoms, bytes) on both documents, pinned as a growth guard in
  the shape of `SLAB_MAX_TERMS`.
- The header and `# Cost` in the present tense; `DriveConfig.plain_memo`
  documented; D9's per-leaf sentences corrected.

## Review

The full v6 dual (`docs/MODEL-AB-LOG.md` protocol v6; ordinal from
SYM's band at dispatch). Claims to falsify: (1) no decision moves —
the pins, and the reviewers' own document driven parallel and
sequential with the memo on and off; (2) the memo is
schedule-independent — the receipt byte-identical across thread
counts, including `frozen`; (3) a memo hit hands back the form the
leaf would have built — plant a leaf-varying opaque (a value-dependent
mint) and show the opaque-sequence row reds before anything else
does; a budget mismatch is refused; (4) the atoms a hit needs are
present (a hit on a node whose atom the leaf never minted — the
top-residual reduce finds its `AtomInfo`); (5) the numbers; (6) the
memo is dropped with the drive and holds nothing of the next one.
Style: the map's locking is one write per leaf and the header says
why; `Rc`/`Arc` measured not assumed; `frozen`'s two meanings said at
both sites. Union fix pass on the implementer's lane; delta by R1;
the row lands at merge.

## Landing

Status `review` on `work/sym/SYM-7.md` when the PR opens; the
orchestrator closes the unit and deletes this spec at merge; the cost
row's volume ask closes with it (its other asks stay).
