---
id: a-slow-row-reaching-its-data-file-by-a-built-name-is-not-gated
kind: issue
title: A slow-set row that reaches its golden or fixture by a name it builds (format!, a directory walk), or from another crate, is not selected when the diff edits only that data file
status: open
opened: 2026-11-21
priority: P3
cost: M
---



Filed by GREEN's `green/slow-row-selection` lane, as the residue of
`a-slow-row-whose-assertion-moves-without-its-name-line-is-not-gated`.

## What the selector now sees

`scripts/check-fast-set-times.py`'s `edited_tokens` treats a Rust file as
edited when one of its string literals spells the file name of a non-Rust
file the diff edits (`_git_referrers`: `git grep -F` on the base name,
kept only where a `"` opens the run before it). So
`include_str!("golden/perf12_census_1e-6.txt")` selects the census that
reads it, and `slow_set.is_edited` picks that file's slow rows.

## What it cannot see

- **A name the test builds.** Replaying PR 6956's data-only edits (merge
  `796eac568e`, its `.txt` and `.step` paths alone): the three
  `perf12_census_*.txt` goldens and the thread-count digests select their
  readers, while the five `crates/step-export/tests/fixtures/*.step`
  re-cuts (`ball.step`, `composed_die.step`, ...) select nothing. Their
  reader spells the fixture's stem and adds the extension (or walks the
  directory), so no literal spells `ball.step`.
- **A reader in another crate.** ci.yml's slow-set step runs
  `($pkgs) & <selection>` with `$pkgs` the change filter's seeds, so a
  reader in a crate the diff seeds nothing in is dropped after
  selection.
- **A shared base name** over-selects (`perf12_census_1e-6.txt` exists in
  both editor-core and step-import, and an edit to one selects both
  readers). That costs a few seconds, and it is noted here only so a
  narrowing does not take it for a defect.

## Where to look

A cheap widening: also match a literal that spells the edited file's
parent directory (`"tests/fixtures"`), so a directory walk or a built name
under it is selected. Measure it the way that row's PR did, by replaying
the selection over recent merges against the nightly's per-test times.
