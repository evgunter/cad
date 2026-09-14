# SYM-2 — the tier's file split (spec)

**Program:** SYM (`work/sym/plan.md`, the hygiene lane). **Item:**
`work/sym/sym-rs-is-one-file-with-a-347-line-header.md`. **Track:** a
HYGIENE unit — pure moves, no behaviour change. Outside the A/B
experiment: no ordinal, one style review, no row.

**Read first, in full:** `docs/prompts/implementer-discipline.md` (§4
comment style binds every sentence you move); the item; the whole of
`crates/geom-core/src/sym.rs` (Q8 of the reviewer brief will be applied
to you: read it end to end before you cut it); `memories/cad-working-style.md`
§Doc prose discipline and §Code comments.

## The finding

`sym.rs` was 3,898 lines behind a 347-line header when the item was
filed; it is **4,478** today (M10-10 added the trig section and the
zero normalization). Two reviews of M10-9 each found a contract
sentence in the header that the code hundreds of lines below no longer
honoured. The item's own diagnosis: a header that long is read as
history, not as a contract; the goal is **contracts next to the code
they bind**, not a smaller number.

## Phase 1 — the moves

Take the three the item names, in this order, each its own commit so
the review can read each as one move:

1. **The coefficient tower** → `sym/rational.rs`: `Int`, `Rat`,
   `gcd_u128`, `isqrt_u128`, `COEFF_BITS`, their impls and their tests.
   No dependency on the session; a self-contained argument (the
   `i128` inline, the bound as a freeze discipline).
2. **The polynomial** → `sym/form.rs`: `Poly`, `Mono`, `mono_mul`,
   `Form`, `within`, `powi_form`, and whatever else is a pure operation
   on forms with no session in its signature. `combine` and `form_in`
   are the WALK and stay with the session unless the lane can show the
   cut is cleaner one line lower — say which and why in the PR body.
3. **The header goes with them.** Each moved block takes the part of
   the `//!` argument that is about it — the coefficient section under
   `# Freezing` goes to `rational.rs`, the quotient-form paragraphs
   ("What remains outside the PLAIN form", the normal form as a field of
   fractions) to `form.rs` — and what stays in `sym.rs` is about the
   tier as a whole: what a symbolic `Zero` claims, the DAG, the rules
   by name with a pointer to each home, the door, the census, the
   session-and-no-session paragraph. The item's ask is that a sentence
   binding a piece of code sits in that piece's file.

A move is a move: `git diff -M --color-moved=dimmed-zebra` over each
commit shows relocated blocks plus `use` lines and visibility changes
(`pub(super)` where a crate-private item crosses the module line;
nothing becomes `pub` that was not). No logic edits, no renames of
anything with a caller outside the file, no reordering inside a moved
block. If a move forces a change that is not one of those, stop and
say so in the PR body rather than fold it in.

## Phase 2 — history out of the header

While the prose is in hand, apply implementer-discipline §4 to the
header sentences you move or leave: a sentence whose only content is
archaeology — "They were an in-tree `i128` through M10-7", "Through
M10-9 this paragraph listed all three as undecided; R1 of M10-10 ran
them", "M10-8 built them per node at 138 s … and M10-10 made them
affordable", "An earlier draft of this paragraph said …" — is cut or
rewritten to the present-tense invariant it defends. **Measurements
stay**: every number in the header is a measured claim with a pin
named beside it, and those are the contract. The test for a cut is
`memories/cad-working-style.md`'s: keep it only if something would go
wrong without it. **Every cut is one line in the PR body**, quoted, so
the reviewer adjudicates each rather than trusting a sweep — and a
sentence you are unsure about stays and is listed as left.

Do NOT edit any sentence's technical content. If a header sentence
looks false against the code — the M10-9 class — leave it, list it in
the PR body under "looks wrong, not touched", and file it as an issue
on `work/sym/` per implementer-discipline §6.

## What this unit does not do

- No split of `Session`, the walk, `Discharge`, the registry or the
  `Decide` impl. If the lane believes a third module is the right next
  cut, propose it in the item body (`## After SYM-2`) with the line
  count it would take out; the orchestrator decides.
- Nothing in `sym/algebra.rs`, `report.rs`, `signed.rs`, `trig.rs`
  beyond a `use` path that a move forces.
- No change to any test's assertion. Tests move with the code they
  test and keep their names.

## Acceptance

- `sym.rs` is the tier's whole minus the two blocks; `sym/rational.rs`
  and `sym/form.rs` exist with their own `//!` carrying the argument
  about them; the remaining header is about the tier as a whole and is
  shorter by at least the two blocks' share (state the line counts
  before and after, header and file).
- Every existing row green on the full hosted matrix (the twelve
  `test (…)` jobs and the five `k-lint` unifications); the M10-10 pins
  and the `SymbolicDials::off()` bit-identity rows do not move. The
  rustdoc gate (`scripts/doc-gate.sh`, `rustdoc::broken_intra_doc_links`)
  green — every `[`link`]` in the moved prose resolves from its new
  home.
- `cargo clippy --all-targets -- -D warnings` on `geom-core` and the
  workspace, and the two demo roots (`demos/tour`, `demos/wild`) per
  implementer-discipline §2.
- The PR body: the three commits named; the cut list; the "looks
  wrong, not touched" list; `git diff -M --stat` per commit.

## Review

One style reviewer, `docs/prompts/reviewer-style-lane.md` in full, with
these claims to falsify: (1) every hunk in each move commit is a
relocation, a `use` line or a visibility change — nothing else (the
reviewer reads the `--color-moved` diff commit by commit); (2) no
sentence's technical content changed; (3) each cut in Phase 2 is
archaeology and not an invariant; (4) the header that remains is about
the tier as a whole, and a reader looking for the coefficient bound or
the quotient form's limits finds it beside the code. Fix pass on the
implementer's lane; the unit's log entry rides the PR last.

## Coordination

SYM-1 (the profile) runs concurrently and adds feature-gated counters
inside `form_in` / `combine` / the coefficient ring — the code this
unit moves. **This unit lands first**; SYM-1 merges `origin/main` after
it and re-sites its counters. Keep the moves pure so that merge is a
relocation, not a conflict of logic.

## Landing

Status `review` on `work/sym/SYM-2.md` when the PR opens; the
orchestrator closes the unit and deletes this spec at merge. The item
`sym-rs-is-one-file-with-a-347-line-header` closes with it unless the
lane's `## After SYM-2` proposal is taken, in which case it stays open
on that.
