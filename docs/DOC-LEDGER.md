
## Per-merge deletion — SYM-13's spec (2026-09-22)

Recoverable at `git show dc88037b3:docs/SYM-13-SPEC.md` (the polished
head of #3054). Its sentences that did not survive: the thesis's set —
"the distinct nodes of the drive's frozen set that this leaf's walks
reached" — the set is the closure of the leaf's plain-walk ROOTS over
its hash-consing table (`Session::forms` is truncated by every memo
hit), intersected with the drive's frozen set, UNIONED with the leaf's
own side read from its TABLE and not its walk: the ids its DAG named
before its table held them (`Session::foreign`, a candidate set
reconciled at leaf end) and the freezes it could not publish — the
first reviewer's delta showed by execution that a walk-side count moved
under a memo hit with every decision standing still, and the fix pass
that argued the branch away was wrong and said so; "schedule-independent
by construction" — qualified: one reading is still the schedule's, a
taint-induced freeze under a hit, pinned by name and filed at P2
(`a-taint-induced-freeze-under-a-hit-still-reads-by-order`); "the same
under every schedule — except `frozen`" — now every column, with that
one reading named; Phase 1.1's two shapes — a first level wider than
one box is unreachable (`drive`'s frontier is one box), the reachable
shape a DAG that grows between levels (the slab at eight leaves with the
budget cut); "per leaf and per drive at the whole-certifying ceiling" —
that drive is one leaf; the STOP on NEED's cost — not reached (2.3 ms a
leaf against the 1.6 s line, 0.6 % of a 48-leaf plate drive; the
table-side collection +7 % on the slab's build, +1.3 % on the plate's,
disclosed); the drive's column "not the sum of theirs" — nor a bound on
them: a leaf's column can EXCEED the drive's. Recorded in the PR body
and the unit's `## Closed` section.

- `SYM-13-SPEC.md` — SYM-13, the leaf receipt's `frozen` column is the leaf's NEED (#3054)
