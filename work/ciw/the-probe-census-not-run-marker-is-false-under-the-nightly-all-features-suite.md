---
id: the-probe-census-not-run-marker-is-false-under-the-nightly-all-features-suite
kind: issue
title: probe-suite-census makes a whole-file probe suite say NO TEST IN THIS FILE IS EXECUTED BY CI, which the nightly's --all-features suite falsifies
status: open
opened: 2026-11-20
priority: P3
cost: E
refs: [7148]
---


`scripts/gates/probe-suite-census.sh` (`FILE_NOT_RUN_MARKER`, the disposition
half) requires every probe-gated suite it does not roster in `RUN_FLOOR` to
carry the `//!` sentence `NO TEST IN THIS FILE IS EXECUTED BY CI`. Its
predicate is narrower than the sentence: "nothing in
`scripts/k_probe_sweep.sh` runs it". `.github/workflows/nightly.yml`'s
`all-features` job ("suite under --all-features (default eps)") runs every
non-`#[ignore]`d probe-gated test, so for 4 headers (grep the sentence under
`crates/`) the sentence is false as written. A reader who believes it
skips the suite when a door moves, and nightly 97 went red because of one
(`crates/editor-core/tests/m10_7_r1_census_probe.rs`, row
`work/green/main-red-the-collinear-walls-probe-expects-the-edit-door-to-refuse-a-tangent-junction.md`).
That file's header first said CI did not run it, and the gate then refused
the header once the sentence came out (PR 7148, lint job, gates step).

PR 7148 kept the sentence, qualified in prose ("in the sense
`probe-suite-census.sh` asks … the nightly's `--all-features` suite does
run …"). The fix is the gate's: reword the marker to what it checks (for
example `NOT IN THE K-PROBE SWEEP`), or roster the all-features job as a
second executed set. Then re-word the headers that carry it.

(DOCTAIL lane, found while re-pinning the R1 census probe)

Moved from GUARD to CIW on 2026-11-20, by `git mv`, when GUARD merged into CIW: GUARD held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CIW carries.
