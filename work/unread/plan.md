# UNREAD — claims, references and surfaces that nothing reads (plan)

Opened 2026-10-26 by GUARD's split (`work/guard/log.md`, that date),
and joined the same day by UNSEEN, META's cut of the cross-tree
reference and surface checkers (`docs/doc-ledger/unseen-leaves-the-tracker.md`).
Live state is `work/unread/log.md`'s tail and the item files beside this
plan, never this file.

Branch prefix: **`unread/`** — unit branches `unread/<unit>-<slug>`.
Away-channel tag `(UNREAD orchestrator)`. A/B ordinal band
**UNREAD = 15700–15799**, claimed on this program for bookkeeping.

## Charter

GUARD's rows are gates that cannot see what they claim; BLIND's are
discipline gates whose matchers miss a spelling. **This program's rows
have no gate at all.** Each names something the tree carries with
nothing reading it — a claim (a measured number, a census, a pin, a
table's reasons) or a reference a human wrote (a citation, a tracker
path, a ledger recovery command, a public method, a merge commit's
authored lines) — and owes one of three things: a mechanical guard, a
scheduled register that re-measures it, or a written reason it can have
neither. A new gate lands under `scripts/gates/` on GUARD's harness
(`lib.sh`), with its selftest, announced on GUARD's log; a checker that
is an arm of `scripts/work.py lint` is built on META's ground by
announced seam, announced on META's log; a new CI step lands by
announced seam on CIW's workflows.

## The slate

**24 budget points** of dispatchable work against a ceiling of 30 (two
rows unpriced, charged 2.5 each; four legacy `D` rows at 2.5).

| item | pri | cost | what it is |
| --- | --- | --- | --- |
| `measurements-have-no-mechanical-guard` | P3 | E | the class row: measured numbers in comments with nothing red if they go false; `Cargo.toml`'s opt-level block first |
| `viewer-disabled-control-population-has-no-gate` | P3 | E | the 30 `add_enabled` sites' dispositions, an allowlist keyed by site |
| `raw-door-compile-proof-needs-a-gate` | P3 | D | BOOL-9's witness-crate compile-fail proof needs a gate with a toolchain |
| `comment-symbol-names-outside-rustdocs-reach-have-no-gate` | P3 | E | 26 `<mod>::<name>` spans in plain `//` comments under viewer, checked by nothing |
| `door-table-reason-strings-are-unguarded-prose` | P3 | E | `review_m1_pr5_internal`'s `ALLOWED` reasons are never read against their doors |
| `vocabulary-macro-bodies-are-outside-rustfmt` | P3 | E | `vocabulary!` bodies are formatted by hand; accept and say so, or gate |
| `doc-citations-no-gate-checks-rot-silently` | P3 | D | `file.rs:NNN` citations across `work/` and test names in doc comments, neither checked |
| `no-instrument-guards-a-public-method` | P3 | D | the façade censuses guard exported names and members, never a public method |
| `authored-content-folded-into-a-merge-commit` | P3 | D | authored lines inside merge commits, invisible to `git log -p` |
| `feature-gated-lib-unit-tests-are-compiled-and-never-run` | — | — | six `probe`-gated `#[test]`s in library `src/` that no hosted job executes |
| `ledger-notes-have-no-guard-that-their-recovery-command-resolves` | — | — | nothing runs a doc-ledger note's `git show <sha>:<path>` |
| `inserting-an-item-above-another-steals-its-docs` | P4 | E | a doc block silently rebinds to an item inserted above its own |
| `S115` | P4 | E | the prelude's import census was taken once and nothing re-takes it |
| `tracker-paths-in-crate-doc-comments-rot-on-a-program-re-cut` | P4 | E | a `work/<program>/<id>` path in a crate comment rots when the row moves |
| `perf-plan-citations-name-a-path-that-moved` | P4 | E | five citations still name `docs/PERF-PLAN.md` |

The two unpriced rows are charged 2.5 each; whoever picks one up prices
it, and re-prices the legacy `D` rows on touch.

## Order

The cheap E rows whose guard is already sketched in their bodies first:
`viewer-disabled-control-population-has-no-gate` (precedent:
`viewer-vocab-declared-once.sh`), `door-table-reason-strings-are-unguarded-prose`
(the cheap arm is written), `comment-symbol-names-…`. Then the citation
family, whose three rows are one instrument seen from three sites:
`doc-citations-no-gate-checks-rot-silently` (two arms) and
`tracker-paths-in-crate-doc-comments-rot-on-a-program-re-cut` (its third
arm, and the one every program re-cut feeds), with
`perf-plan-citations-name-a-path-that-moved` as the instance a working
checker should find and the fix that lands with it;
`comment-symbol-names-…` is a sibling of the same instrument and should
be read beside it. Then `feature-gated-lib-unit-tests-…`, priced on
pickup — six pins go unrun today, one of them a completeness pin — and
the ledger guard, which needs a band and a cost and whose body already
names the shallow-clone case it must not red on.
`measurements-have-no-mechanical-guard` is a sweep; its own sub-list
orders it. The two surface guards each open on a decision their bodies
state: how large an alphabet the public-method census pays for, and how
strict "resolution content" is for a merge commit.
`raw-door-compile-proof-needs-a-gate` needs a CI job with a toolchain,
so it is CIW-seamed. The P4s last.

## Review posture

Infra-only: one style review per unit against
`docs/prompts/reviewer-style-lane.md`, no A/B row. A new gate or lint
arm takes a second correctness reviewer when it can refuse code on
main, because a check that is wrong the day it lands is either silent
or red for the wrong reason, and a false positive there blocks every
program.
