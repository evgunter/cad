# BRIEF — the plan

The standing discipline: `docs/prompts/` and the memories it leans on,
every change an `[ev]` PR.

Opened 2026-10-26 by META's cut (`work/README.md`, Track size). Nothing
dispatched; branch prefix `brief/`, away-channel tag
`(BRIEF orchestrator)`, A/B band 15100–15199 claimed for bookkeeping.

## The slate

**7 budget points** of dispatchable work against a ceiling of 30 (one
row unpriced, charged 2.5). The load is small on purpose: every row
lands as an `[ev]` PR and waits on Ev's answer, so the sitting's cost is
the round trip, not the edit.

| pri | item | cost | title |
|---|---|---|---|
| — | `four-lane-practices-have-no-home-outside-the-ledger` | — | four standing lane practices survive only in ledger sweep notes (`needs_ev`) |
| P3 | `an-items-stated-sweep-pattern-may-not-match-its-own-instance` | D | an item's stated sweep pattern can fail to match the instance it was derived from |
| P3 | `unit-numbered-test-suite-prefixes-collide-across-program-generations` | E | unit-numbered suite prefixes collide across program generations |
| P4 | `orchestration-model-prescribes-a-usage-instrument-with-no-file` | E | `memories/orchestration-model.md` names a usage file that is not there |

## Order

`four-lane-practices-have-no-home-outside-the-ledger` first: its
question is already in front of Ev, and its body carries the diff for
§2, §5 and §6 of `docs/prompts/implementer-discipline.md`. The two
`implementer-discipline.md` sentences that follow —
`an-items-stated-sweep-pattern-may-not-match-its-own-instance` (§5, or
the reviewer brief's Q6) and
`unit-numbered-test-suite-prefixes-collide-across-program-generations`
(the suite-naming rule) — can ride the same `[ev]` PR if it is still
open when they are written, since Ev reads one diff of one file more
cheaply than three. The memory row is a separate `[ev]` PR on
`memories/orchestration-model.md`.

## Review posture

META's, which it inherits: prose units, no A/B row, one style review per
unit. Ev's sign-off is the merge gate for every row.
