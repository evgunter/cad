---
id: bound-list-readers-have-three-homes
kind: issue
title: Three bracket-depth bound-list readers: bounds-allowlist.sh awk, bounds_census.rs, test_utils::source
status: open
opened: 2026-09-06
---


Found by the style review of PR 2056 (GATES `D102`). Three readers now
walk a Rust bound list at bracket depth, splitting at top-level commas
and keying per parameter, in two languages, none citing the others:

- `scripts/gates/bounds-allowlist.sh` — the awk reader D102 added
  (generic list + `where` clause, keyed by the bounded type's text,
  grouped across clauses); the compound-bound gate.
- `crates/geom-core/tests/bounds_census.rs:20-60, 443-466` — the
  sole-bound twin: generic list + `where` clause, split at top-level
  commas, keyed per parameter, with "a private copy of
  `test_utils::source::top_level_split`, kept deliberately".
- `crates/test-utils` `source::top_level_split` — the third walker.

The census's documented blind-spot list (its #1: `impl Bounds` in
argument position) is the mirror of what the gate's reader now handles;
neither says the other exists. `bounds_census.rs` is outside every
program's fence (no `paths` entry names `crates/geom-core/tests/*`), so
this is filed here rather than on GATES or PROPS. Least change: each
reader cites the other two and says which shapes it reads that they do
not; the fuller fix, one reader that both the gate and the census
consume, is a design question (shell vs Rust) for whoever picks this up.

## Re-homed to GUARD (2026-09-11, the cut in `docs/WORK-TRACKS-2026-09.md` addendum 3)

GUARD is the successor on `scripts/gates/*`, the half of code-quality
Track K that came back to that directory when GATES closed on
2026-09-08. This row is here because the gates — and the guards the
gates themselves lack — are one subject and one file territory.

Its class at the cut was **M** — cross-citation is cheap, but the real
fix is a shell-vs-Rust shared-reader design call. The class is a
dispatch estimate made by reading the row against the tree on
2026-09-11, not a verdict on the finding, and a lane that finds it wrong
says so in its PR. The id, the `track:` letter where the row carries
one, and the body above are unchanged by the move.
