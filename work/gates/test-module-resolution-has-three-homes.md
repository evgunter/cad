---
id: test-module-resolution-has-three-homes
kind: issue
title: Test-module resolution and the excluded-path filter have three homes under scripts/gates/ — one rustc-correct, two textual — and lib.sh owns neither
status: open
opened: 2026-09-06
refs: [gate-mod-path-resolved-textually, D211]
---


## Finding

Filed by the GATES orchestrator from three lanes' reports on one day
(PRs 2029, 2032, 2033). Two mechanisms, three gates, no shared home:

**Resolving `#[cfg(test)] mod x;` to the files it excludes from a
production scan.** `interval-square-allowlist.sh` does it rustc's way
since PR 2033 (`#[path]` wins; a root or `mod.rs` declarer resolves to
the sibling; any other declarer `dir/foo.rs` to `dir/foo/x.rs` and
`dir/foo/x/`; an inline `mod x { … }` to no file; the raw and code-only
views cross-checked and a disagreement refused). `witness-not-ambient.sh`
(its header §"a test IS an entry point", the exclusion built near
`:118`) and the new `panic-free-macro-bodies.sh` (PR 2032, its
`#[cfg(test)] mod x;` file allow) resolve the same declaration
textually — the sibling `dir/x.rs`, which is wrong for every declarer
that is not a crate root or `mod.rs`. Both have one live resident today
and it sits in a `lib.rs`, where the two rules agree; the day a
non-root file declares a test module, the two textual gates under- or
over-scan silently, which is exactly `gate-mod-path-resolved-textually`
again, twice.

**Filtering the scan set by the excluded paths.**
`interval-square-allowlist.sh:203` filters with
`gate_grep -vF` over a newline-joined path list — a substring match per
path, so a scanned path that merely CONTAINS an excluded file's path
would leave the scan (no live pair today; reported by PRs 2029 and
2033 independently). `witness-not-ambient.sh:118` does the same with
a `:`/`/` suffix appended, which anchors the tail but not the head.

`lib.sh` builds the views every gate reads; it builds neither of these.
The fix shape is the one PR 2029 used inside `bounds-allowlist.sh`
(one reader, two modes): move PR 2033's resolver and one anchored path
filter into `lib.sh`, have the three gates call them, and keep PR
2033's fixtures as the shared reader's. Sequenced after PRs 2032 and
2033 land, since it edits both.

**What was not measured**: whether any other gate excludes by a
`#[cfg(test)]` declaration through a pattern the two greps above did
not match (the sweep was `cfg\(test\)` and `-vF` over
`scripts/gates/*.sh`).

**A third mechanism, same shape** (PR 2032's review, Q1): three
hand-rolled balanced-delimiter walkers over the code-only view —
`lib.sh`'s `--skip-cfg-test` counting `{}` by `gsub`,
`bit-identity-debug-only.sh`'s per-character `{};` walk, and
`panic-free-macro-bodies.sh`'s per-character `{}()[]` walk. `lib.sh`'s
own header argues that two hand-rolled readers is how the `//` strip
got copied everywhere; a record shape beside `--statements` and
`--window` is where a shared walker would live. Same row, because the
same helper file owns the answer.
