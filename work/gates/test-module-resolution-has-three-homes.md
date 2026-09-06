---
id: test-module-resolution-has-three-homes
kind: issue
title: Test-module resolution and the excluded-path filter have three homes under scripts/gates/ — one rustc-correct, two textual — and lib.sh owns neither
status: review
opened: 2026-09-06
refs: [gate-mod-path-resolved-textually, D211]
branch: gates/reader-homes
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

## Landed (2026-09-06)

On `gates/reader-homes`, with
`window-view-emits-a-record-for-a-comment-only-line`.

**What moved into `lib.sh`.** `gate_test_only_mounts FILE...` yields the
set of test-only files and directories a source list mounts, rustc's
way — PR 2033's resolver, its `gate_declaration_shape` /
`gate_path_payload` / `gate_norm_path` / `gate_refuse_declaration`
verbatim. `gate_filter_test_only_paths PATH...` is the anchored filter:
whole path for a file entry, prefix for a directory entry.
`gate_production_sources` composes the two with the refusal boundary
(the marker, read before any guard reads the list) and the
every-source-is-test-only guard, and sets `GATE_SCAN_FILES` to what the
gate will actually read. `interval-square-allowlist.sh`,
`witness-not-ambient.sh` and `panic-free-macro-bodies.sh` call it; the
two textual copies (`cfg_test_modules`, `cfg_test_module_files`) and
both `grep -vF` record filters are gone.

**The `cfg` regex has one spelling.** `GATE_CFG_TEST_RE` and its
subtraction `GATE_CFG_TEST_NOT_RE`, defined once above the reader, read
by the reader's `--skip-cfg-test` predicate (through `ENVIRON`, never
`-v`) and by the resolver's two greps. The raw narrowing reads it too
rather than a fixed `#[cfg(test)]` string, so a declaration under
`all(test, …)` can no longer slip past the first stage; the four live
files that match the regex and not the literal all carry inline
modules, so the exclusion set is unchanged by that.

**Scanned-set diff on the live tree** (433 sources): the rustc resolver
excludes 37 files, the textual copies 36. The one file both textual
gates read as production and rustc does not is
`crates/topo/src/boolean/solid_contain/r1_generic_poses.rs` — declared
`#[cfg(test)] mod r1_generic_poses;` in `boolean/solid_contain.rs`, a
non-root declarer, so the sibling rule named
`crates/topo/src/boolean/r1_generic_poses.rs`, which does not exist.
Nothing moved in the other direction, and
`interval-square-allowlist.sh`'s set is unchanged. The two gates' OK
lines now say 396 rather than 433, because they read a file set instead
of filtering records after reading everything.

**The fixtures moved with the resolver, into
`gate_selftest_test_module_homes`** — not into `gate_selftest_clean`
itself, which all nineteen gates run and sixteen of which resolve no
modules and would fail the cases. The helper is called by every caller
of `gate_production_sources` and takes the one thing that differs, the
gate's own breach (a square, a minted witness, a macro body spelling
`.unwrap`), appending it to a path the shared planter has made. Eighteen
cases, both directions, including the one-line `#[cfg(test)] mod x;`
spelling. This is the half one gate's fixtures cannot prove: the
resolution has one home, so `interval-square-allowlist.sh`'s cases did
prove the RESOLVER, and deleting either textual copy outright left its
own gate's self-test green.

**The three delimiter walkers: a stated cost, not a fourth walker.**
`lib.sh`'s `--skip-cfg-test` counts `{`/`}` by `gsub` per record and
holds `depth`/`skipping` as a state; `bit-identity-debug-only.sh` walks
`{`, `}` and `;` per character to end an item at depth zero;
`panic-free-macro-bodies.sh` walks `{}()[]` per character to cut the
SPAN of a `macro_rules!` body out of a line. A shared record shape
would be a depth column — `FILE:LINE:DEPTH:TEXT` — and it serves the
first two: both ask *what is the nesting depth here*. It does not serve
the third, which asks *where inside this line does the body start and
end*, and a per-record depth cannot answer that; serving all three
means emitting spans, which is a second traversal of the lexer's states
rather than a column on this one. Two of three is not enough to make a
shape the reader must then carry for every caller, so the walkers stay
as they are. **What would change the answer**: a third caller wanting
depth-at-record (then the column has two consumers that are not each
other's only one), or a caller wanting spans (then `--spans` is the
mode, and the panic walker is its first consumer).

**The sweep, and what it could not match.** The pattern for the class
was `grep -n 'cfg(test)' scripts/gates/*.sh` plus `grep -n '\-vF'` over
the same, at this branch's merge base. Hits and disposition:
`interval-square-allowlist.sh` (fixed — now the caller),
`witness-not-ambient.sh` (fixed), `panic-free-macro-bodies.sh` (fixed),
`lib.sh` (the reader's own predicate, now one spelling with the
resolver's), `probe-suite-census.sh` (`-vF` over a SUITE list, not a
path exclusion — not this class), `test-aggregation.sh` and
`gated-suite-paths.sh` (`cfg(test)` named in prose only).
`bit-identity-debug-only.sh` spells the attribute a third way —
`/#\[cfg\(test\)\]/` in its own awk, for its debug-only item skip — and
is held by another lane, so it is not in this diff; it wants
`GATE_CFG_TEST_RE` as a rider. **What the pattern cannot match**: a
gate excluding test code through a spelling that names neither `cfg`
nor `-vF` — a hard-coded path list, or a `find` that prunes a
directory by name. Neither exists under `scripts/gates/` today, and
neither would be found by this sweep if it did.
