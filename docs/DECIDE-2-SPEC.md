# DECIDE-2 — one pin per seam: the discharge vocabulary's five spellings held together (spec)

**Program:** DECIDE (`work/decide/plan.md`). **Item:**
`work/decide/registered-is-spelled-five-times-and-pinned-once.md`
(M10-9's class item: a discharge outcome is spelled five times in two
crates — `SymCounts::registered`, `Discharge::Registered`,
`ShapeOutcome::Registered`, `SampleOutcome::Registered`,
`Scan::registered` — and only the `SampleOutcome` ↔ `ACCEPTED_OUTCOMES`
pair is pinned, in `tools/k-lint/tests/outcome_vocabulary.rs`).
**Track:** OUTSIDE protocol v7 — the MECHANICAL tier (item 2: "the
mechanical change merges on the orchestrator's own read, where neither
correctness nor style is meaningfully at risk"): an OPUS implementer,
green CI and the orchestrator's read, no review lane, no draw, no row.
**Difficulty E, class STRUCTURAL** (bookkeeping only).

**The ruling the item asks for (the orchestrator's, 2026-09-21):
shape 2, a pin per seam.** Not shape 1 (one enum projected: `k_stats`
would come to depend on `sym`, a dependency the tier's design keeps
absent, and a `From` impl is a pin only where the compiler's
exhaustiveness reaches, which it does not across `tools/k-lint`'s
workspace boundary); not shape 3 (leave it: the item was filed because
the door's five edits were made by hand and the order they were made
in is why one pair is pinned and three are not — the next discharge
kind, whoever mints it, should red somewhere). Five spellings stay
five types, by design; what changes is that a sixth kind reaching
four of them cannot compile-and-pass.

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item whole; `tools/k-lint/tests/outcome_vocabulary.rs` (the shape
every new pin copies: enumerate one side's ALL, assert each maps into
the other, and that the other accepts nothing else; the module doc says
why a pin lives where it lives); `crates/geom-core/src/sym.rs` —
`enum Discharge` and `pub struct SymCounts` (the receipt's columns) and
where a `Discharge` becomes a count; `crates/geom-core/src/sym/report.rs`
— `ShapeOutcome` and the `Discharge → ShapeOutcome` arm (`:153`);
`crates/geom-core/src/k_stats.rs` — `SampleOutcome` and `ALL`, and
where a `ShapeOutcome`/`Discharge` becomes a `SampleOutcome` (find the
site); `tools/k-lint/src/lib.rs` — `Scan`, `ACCEPTED_OUTCOMES`, the
`lint_sample` arms.

## What is owed

Three pins, one per unpinned seam, each in the shape of
`outcome_vocabulary.rs` and each placed in the crate that can see
BOTH sides of its seam:

1. **`Discharge` ↔ `SymCounts`**: every `Discharge` variant increments
   exactly one receipt column, and every discharge-kind column of
   `SymCounts` is reachable from a `Discharge` (the columns that are
   NOT discharge kinds — `numeric`, `frozen`, the call counts — are
   named as such in the row so the pin says what it excludes). If
   `Discharge` is private, expose what the pin needs at
   `pub(crate)`/test-support level rather than widening the public
   surface; say which.
2. **`Discharge` ↔ `ShapeOutcome`**: every `Discharge` variant has a
   `ShapeOutcome` row and the report's discharge rows come from nothing
   but a `Discharge` (the non-discharge rows — `Invalid`,
   `Indeterminate`, the definite signs — named as excluded).
3. **`ShapeOutcome`/`Discharge` ↔ `SampleOutcome`**: every discharge
   row maps to one `SampleOutcome` token and every discharge-kind
   `SampleOutcome` variant is reachable from a discharge.

Each pin is a row that REDS under a planted sixth kind: the lane
plants a `Discharge::Planted` variant (a local, reverted commit),
wires it into ONE side of each seam, runs the three rows plus the
existing `outcome_vocabulary.rs`, and records in the PR body which row
reds at which seam — the demonstration that the pins are what the item
asked for. The plant is reverted; the PR carries no sixth kind.

Also: the item's table gains a sixth column, "pinned by", naming the
row for each seam; and one sentence at `enum Discharge`'s doc
(`sym.rs`) says "a new variant reds these three pins and
`outcome_vocabulary.rs`; do not add one without them" — the pointer a
person adding a sixth will read. No behaviour changes; no count moves;
no golden is touched.

## Scope

- Files: the three new rows (under `crates/geom-core/tests/` or
  `crates/geom-core/src/...`'s test modules where the seam is private —
  the lane says which and why), the one doc sentence, the item. Nothing
  in `tools/k-lint/src/lib.rs` or `k_stats.rs` beyond a
  `pub(crate)`/`ALL` accessor if a pin needs one (say so).
- No change to any discharge kind, count, token or report row.

## Acceptance

- The three rows green; each demonstrated to red under the plant, with
  the PR body's table naming the row and the seam.
- `cargo fmt --all -- --check`; clippy `-D warnings` on `geom-core`
  (default and `interval`, all targets) and `tools/k-lint` (its own
  workspace root); `scripts/doc-gate.sh`; `python3 scripts/work.py lint`;
  the hosted matrix green.
- The item CLOSED (`status: closed`, `closed:` dated) with the ruling
  and the three rows named; the spec deleted at merge with its
  `docs/DOC-LEDGER.md` entry (the lane does this in its last commit,
  in the ledger's shape).

## Review and landing

Mechanical tier: no review lane; the orchestrator reads the diff and
the plant table and merges on green. PR against `main`; branch
`decide/2-discharge-pins`. Territory: `python3 scripts/work.py territory`
on the branch — `tools/k-lint` is INSTR's ground and `k_stats.rs` is
PROPS'; announce any touch in the PR body.
