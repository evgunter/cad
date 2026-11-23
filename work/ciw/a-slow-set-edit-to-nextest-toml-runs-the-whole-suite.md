---
id: a-slow-set-edit-to-nextest-toml-runs-the-whole-suite
kind: issue
title: an edit to .config/nextest.toml that only adds or drops slow-set names forces TIER=all, about a quarter of all PRs' full-suite runs
status: closed
closed: 2026-10-16
opened: 2026-10-16
priority: P3
cost: M
---


## Measured (2026-10-16, CIW orchestrator)

I replayed 257 of the 263 first-parent merges to main from 2026-10-09 to 2026-10-16 through main's `scripts/ci-filter.py`; 6 were cut off by a time limit.
- 108 were docs, 82 closure, 3 aux, and 64 all.
- Of the 64 all-tier merges, **36 forced TIER=all because of `.config/nextest.toml`**. `docs/predicate-dimension-audit.md` is a separate row; 2 came from `ci.yml`, and one each from a script, `interval-transcendentals/` and `benches/`.

`docs/prompts/implementer-discipline.md` §2 tells a lane to add any test costing ≥ 1 s to the `ci` profile's slow-set filter in the PR that adds the test. So the commonest way for a PR to land in the slowest tier is to follow that instruction. TIER=all is a cold workspace build plus the whole suite at three ε rows: about 30 min of the `test` job before any slow set runs.

## Wanted

When a `.config/nextest.toml` diff changes only exact-name entries of the `ci` profile's `default-filter`, classify it by the crates those names belong to, through their `binary_id(<pkg>::…)`. Any other nextest.toml change still forces TIER=all. ci-filter.py is S-TCOST's ground, so announce the seam.

## Closed

`scripts/ci-filter.py` (`_slow_set_seeds`) now reads `.config/nextest.toml`
at the merge base and at HEAD. It seeds the crates of the `(binary_id, name)`
pairs that were added or dropped. It does this only when three things hold:
both revisions parse, the text outside the one multi-line `default-filter`
string is byte-identical, and both filters are exactly `not ( <entry> | ... )`
over `(binary_id(<pkg>[::<bin>]) & test(=…))` entries. Any other change still
forces TIER=all: a setting, a new profile, a comment, a non-exact predicate,
a reordering that moves no name, a non-member package, a parse failure, or a
`--files` run with no base. A moved name seeds both crates. An added name runs
in its crate's slow-set step. A dropped name runs in its crate's default
step. `check-slow-set-names.py` checks both, because the seeded packages are
in the listing.

One parser reads the filter: `scripts/slow_set.py`. ci-filter,
`check-slow-set-names.py` and ci.yml's slow-set step all go through it. It
follows nextest's semantics: whitespace is allowed between tokens but not
inside a matcher's text. A name or binary id with whitespace inside it is
refused, which makes the edit TIER=all and the name check red. It is never
trimmed into a different, unchanged entry.

Replayed through the new filter. The first set is the 268 merges that
/home/user/cad's shallow clone reaches, 2026-10-13 to 2026-10-15. The
"7-day" figures above came from that set, since the clone does not reach
2026-10-09. TIER=all went from 68 to 10. Every all→closure transition is
one of the two rules. Of the 39 merges forced by nextest.toml, 36 now scope.
Two of the other three edited the header comment (#5206, #5212), and the
third also touched `scripts/payload-rung-sweep.py`. The 10 left: `ci.yml` 3,
`scripts/payload-rung-sweep.py` 3, the nextest.toml header 2, `benches/` 1
and `interval-transcendentals/` 1. Over the true
7 days, 805 merges since 2026-10-09: TIER=all went from 168 to 37, and
closure from 242 to 373. Docs (391) and aux (4) did not change. The 37 left,
by first unscopable path: `scripts/payload-rung-sweep.py` 13, a member
`Cargo.toml` 8, `ci.yml` 4, `interval-transcendentals/src/arith.rs` 3,
`scripts/oracles/*` 5, the nextest.toml header 2, `benches/` 1, and
`scripts/spiric-quadrature-references.py` 1. 7 of the 37 seed no crate.
