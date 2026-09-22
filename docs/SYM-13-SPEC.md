# SYM-13 — the leaf receipt's `frozen` column is the leaf's NEED, not its work: schedule-independent by construction (spec)

**Program:** SYM (`work/sym/plan.md`). **Item:**
`work/sym/leaf-frozen-column-is-schedule-dependent-under-the-drive-memo`
(P0, filed by SYM-7 as a residue of the drive-scoped plain memo Ev
ratified on `[ev]` #2581 — not a defect in it). **Track:** protocol v7
**IN** — a receipt-contract decision: what a column on a leaf receipt
MEANS, read by every consumer of `CertifiedLeaf`/`RefusedLeaf` and by
every row that compares leaf lists across schedules. The full v6 dual.
Block SYM-B3 slot 2, arm OPUS per the block's draw (byte 178).
**Pre-draw fields, logged on `sym/b3-block` before the draw:**
difficulty **D**, task-class **STRUCTURAL**.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item whole; `crates/geom-core/src/sym/memo.rs` (the header — THE
soundness argument, the write order "frozen, then atoms, then forms",
`DriveMemo::frozen` as the DISTINCT nodes frozen over the drive, and
the "unrecorded node" paragraph: a receipt is schedule-independent
only while no leaf reaches that branch); `crates/geom-core/src/sym.rs`
(`SymCounts`, its `frozen` doc — "a work measure and not a receipt
column" — and `# Freezing`; `Session::forms`, the hash-consing table,
`FreezeCause`); `crates/editor-core/src/drive.rs` (`CertifiedLeaf`,
`RefusedLeaf`, their `decisions`, the `PartialEq` derives,
`ParamBoxVerdict::serialize` — which writes the DRIVE's `frozen`, and
`DriveConfig::plain_memo`); the rows:
`crates/editor-core/tests/m10_sym_drive_memo_interval.rs` (the
differential row that prints the leaves' own `frozen` on/off beside
the drive's; `no_leaf_of_a_drive_freezes_a_node_its_session_never_recorded`),
`m10_3_r2_probes_interval::my_own_drive_is_bit_identical_across_repeats_and_schedules`
(compares whole leaf lists across schedules — and so the column),
`m10_3_r1_probes_interval`'s receipt-identity row (the serialized
receipt across one sequential and two parallel drives);
`geom-core`'s `sym_drive_memo` rows (both directions of the unrecorded
asymmetry); `[ev]` #2581's body (the memo's ratified design);
`memories/review-and-dependency-policy.md`.

## The claim

Every column on a leaf receipt is a claim about that leaf's own
predicates and is the same under every schedule — except `frozen`,
which under the drive memo records which leaf got to a node FIRST: a
leaf that finds a node's plain form in the memo never freezes it
itself. The rows that compare leaf lists across schedules are green
today only because a drive's level-0 root publishes the whole DAG
before anything splits, so no later leaf freezes at all — a fact
about today's documents, not a guarantee. **The unit's thesis: the
leaf's column becomes its NEED — the distinct nodes of the drive's
frozen set that this leaf's own walks reached — which is a function
of the leaf's node set and of the drive's frozen set, both
schedule-independent, so the column is schedule-independent by
construction; and the rows that were accidentally satisfied become
well-founded, with an adversary document that races two leaves for a
node as the pin.** The drive's column (`DriveMemo::frozen`, a set) and
the serialized receipt do not move.

**Why NEED and not "drop it":** the drive's column says how many nodes
froze; a leaf's says how much of THIS leaf's reasoning rested on
indeterminates — what a consumer reading a refused leaf wants to know
first. Dropping it loses that; leaving it (SYM-7's choice) keeps a
receipt column that is not a receipt. If Phase 1 shows NEED's cost
above the affordability line, the unit STOPS and reports with the
measurement rather than take "drop"; that choice is put to Ev.

**Ratified and not re-litigated:** E12; the drive-scoped plain memo's
design (#2581: soundness by content id, the write order, the unrecorded
guard on the taint); `DriveMemo::frozen` as the drive's column; the
receipt's serialized shape (`ParamBoxVerdict::serialize`); the freezing
budget.

## Phase 1 — before touching anything

1. **The race, built.** A document (or a drive over an existing one)
   where two leaves DO race for a node under the parallel schedule —
   a DAG that grows between levels, or a first level wider than one
   box (the item names both shapes; the unit builds whichever the
   public doors reach first and says which). Show, by execution,
   that `my_own_drive_is_bit_identical_across_repeats_and_schedules`'s
   comparison REDS on it today (the leaves' own `frozen` differs
   between the sequential and a parallel schedule while every
   decision column and the drive's `frozen` agree) — this is the
   demonstration the item lacks. If no such document can be built
   through the public doors, say why: that is a finding about the
   memo's guarantee, not a reason to stop.
2. **The column today, both documents, both dials, three schedules**
   (sequential, two parallel widths): the leaves' own `frozen` summed
   and per leaf, the drive's `frozen`, every decision column — one
   table (the plate's 50,112 / 0 / 1,044 is the item's reading).
3. **The consumers.** Every reader of a leaf's `decisions.frozen` and
   every `PartialEq` comparison over `CertifiedLeaf`/`RefusedLeaf` in
   the tree (a grep with a disposition per hit): what changes for each
   when the column becomes NEED.
4. **NEED's cost, measured** on a plant behind a local dial: at leaf
   end, the leaf's recorded node ids intersected with the drive's
   frozen set (a hash lookup per node the leaf recorded — say whether
   `Session::forms` or the hash-consing table is the set to walk, and
   whether a second traversal is needed at all). Per leaf and per
   drive on both documents at the nominal and at the whole-certifying
   ceiling; against the leaf instrument's line
   (`m10_10_leaf_cost_with_and_without_the_algebra`, release) and the
   drive-memo row's own timings. One table.

## Phase 2 — the column as NEED

- `SymCounts::frozen` on a LEAF receipt = the distinct nodes in the
  drive's frozen set that this leaf's walks reached; computed at leaf
  end from the leaf's own record and `DriveMemo::frozen`, so it is the
  same whichever leaf paid for the freeze. With the memo OFF the leaf
  froze every such node itself, so NEED equals the old work count
  there — say so, and pin it (the off-dial column unmoved on both
  documents). The doc on `SymCounts::frozen` says what the column
  means on a leaf and on a drive; `memo.rs`'s header carries the one
  argument for why NEED is schedule-independent (the frozen set is a
  property of the node's plain form against the budget, not of the
  schedule; the leaf's node set is a function of its box).
- Rows: the Phase 1.1 adversary as a GATING row — the leaf lists
  bit-identical across the three schedules under NEED (and the row
  reds with the old column: plant it); the drive-memo differential
  row re-cut with the leaves' NEED on/off beside the drive's column;
  `my_own_drive_is_bit_identical…` left as it is (now well-founded)
  with its doc saying why; the receipt-identity row byte-identical;
  a `geom-core` row at the scalar door for NEED on one session (a
  frozen node reached twice counts once; a node reached but not
  frozen counts zero).
- The `PartialEq` derives stay (they are what the rows compare);
  nothing else on the leaf receipt moves.

## Scope

- Files: `crates/geom-core/src/sym.rs` (`SymCounts`, the `frozen` doc,
  the per-leaf accounting), `crates/geom-core/src/sym/memo.rs` (the
  intersection's door, the header), `crates/editor-core/src/drive.rs`
  (the leaf receipts — PROPS' file by territory: announce the seam,
  change no shape, only what the column holds), tests under
  `crates/geom-core/tests/sym_drive_memo*` and
  `crates/editor-core/tests/m10_sym_drive_memo_interval.rs`,
  `m10_3_r{1,2}_probes_interval.rs` (the announced tests-family
  overlap).
- No change to what is serialized; no change to the drive's column; no
  new dial in `SymRules` (the memo's own `DriveConfig::plain_memo` is
  the differential); no new tolerance.

## Acceptance

1. Phase 1's four tables in the PR body; the race demonstrated (or
   its impossibility explained).
2. Under NEED the adversary's leaf lists are bit-identical across the
   three schedules and the row reds under the old column; both
   documents' decision columns, the drive's `frozen` and the
   serialized receipts bit-identical to `main`; the memo-off column
   unmoved.
3. NEED's cost disclosed against the leaf instrument's line, per leaf
   and per drive.
4. Local checks: `cargo fmt --all -- --check`; clippy `-D warnings` on
   `geom-core` at default, `interval`, `interval,sym-profile-testing`
   (all targets) and `editor-core --features interval` (all targets);
   `scripts/doc-gate.sh`; `python3 scripts/work.py lint`. The hosted
   matrix is the verification of record.

## Review

Protocol v7 IN: the full v6 dual — the arm per block SYM-B3's draw
(OPUS at slot 2), two blinded reviewers on a frozen green head
(ordinal claimed on `main` at dispatch, SYM's band 4700–4799), the
union fix pass on the implementer's lane, a delta by R1, the row at
merge.

## Landing

PR against `main`; the spec deleted at merge with its
`docs/DOC-LEDGER.md` entry; the item closed with the result; the SYM
log carries the verdict; block SYM-B3's record reaches `main` when
this slot concludes. Branch `sym/13-leaf-need`.
