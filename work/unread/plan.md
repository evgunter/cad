# UNREAD — measurements, populations and pins that nothing reads (plan)

Opened 2026-10-26 by GUARD's split (`work/guard/log.md`, that date).
Live state is `work/unread/log.md`'s tail and the item files beside this
plan, never this file.

Branch prefix: **`unread/`** — unit branches `unread/<unit>-<slug>`.
Away-channel tag `(UNREAD orchestrator)`. A/B ordinal band
**UNREAD = 15700–15799**, claimed on this program for bookkeeping.

## Charter

GUARD's rows are gates that cannot see what they claim; BLIND's are
discipline gates whose matchers miss a spelling. **This program's rows
have no gate at all.** Each names a claim the tree carries with nothing
reading it, and owes one of three things: a mechanical guard, a
scheduled register that re-measures it, or a written reason it can have
neither. A new gate lands under `scripts/gates/` on GUARD's harness
(`lib.sh`), with its selftest; a new CI step lands by announced seam on
CIW's workflows.

## The slate

| item | pri | cost | what it is |
| --- | --- | --- | --- |
| `measurements-have-no-mechanical-guard` | P3 | E | the class row: measured numbers in comments with nothing red if they go false; `Cargo.toml`'s opt-level block first |
| `viewer-disabled-control-population-has-no-gate` | P3 | E | the 30 `add_enabled` sites' dispositions, an allowlist keyed by site |
| `raw-door-compile-proof-needs-a-gate` | P3 | D | BOOL-9's witness-crate compile-fail proof needs a gate with a toolchain |
| `comment-symbol-names-outside-rustdocs-reach-have-no-gate` | P3 | E | 26 `<mod>::<name>` spans in plain `//` comments under viewer, checked by nothing |
| `door-table-reason-strings-are-unguarded-prose` | P3 | E | `review_m1_pr5_internal`'s `ALLOWED` reasons are never read against their doors |
| `vocabulary-macro-bodies-are-outside-rustfmt` | P3 | E | `vocabulary!` bodies are formatted by hand; accept and say so, or gate |
| `feature-gated-lib-unit-tests-are-compiled-and-never-run` | — | — | six `probe`-gated `#[test]`s in library `src/` that no hosted job executes |
| `inserting-an-item-above-another-steals-its-docs` | P4 | E | a doc block silently rebinds to an item inserted above its own |
| `S115` | P4 | E | the prelude's import census was taken once and nothing re-takes it |

`feature-gated-lib-unit-tests-are-compiled-and-never-run` carries no
band or cost yet and is charged 2.5; whoever picks it up prices it.

## Order

The cheap E rows whose guard is already sketched in their bodies first:
`viewer-disabled-control-population-has-no-gate` (precedent:
`viewer-vocab-declared-once.sh`), `door-table-reason-strings-are-unguarded-prose`
(the cheap arm is written), `comment-symbol-names-…`. Then
`feature-gated-lib-unit-tests-…`, priced on pickup — six pins go unrun
today, one of them a completeness pin. `measurements-have-no-mechanical-guard`
is a sweep; its own sub-list orders it. `raw-door-compile-proof-needs-a-gate`
needs a CI job with a toolchain, so it is CIW-seamed. The P4s last.

## Review posture

Infra-only: one style review per unit against
`docs/prompts/reviewer-style-lane.md`, no A/B row. A new gate takes a
second correctness reviewer when it can refuse code on main, because a
gate that is wrong the day it lands is either silent or red for the
wrong reason.
