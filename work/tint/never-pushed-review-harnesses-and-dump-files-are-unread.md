---
id: never-pushed-review-harnesses-and-dump-files-are-unread
kind: issue
title: Five review harnesses that call themselves never pushed, and eleven dump files, are in the tree unread; whether each asserts, runs by default or replicates a live suite is open
status: open
opened: 2026-09-28
priority: P4
cost: M
refs: [cert3r1-dump-is-a-print-only-replica-of-the-m10-p-fence]
---


## Finding

The wider half of
`cert3r1-dump-is-a-print-only-replica-of-the-m10-p-fence`, moved here
when that row closed on Ev's ruling for its one file (2026-09-28): keep
`cert3r1_dump` as an on-demand instrument over the fence's own walk.
That row found the class and did not read it. **Nobody has read the
files below.** For each one, whether it asserts anything, whether it
runs in the default suite, and whether it replicates a live suite's
walk is open.

Census at merge base `2adacbc0e` (`git grep -n -i 'never pushed\|local-only'
2adacbc0e -- '*.rs'` and `git ls-tree -r --name-only 2adacbc0e | grep
'dump.*\.rs$'`):

**Harnesses that call themselves "Local-only; never pushed"** (five):

- `crates/geom-brep/tests/cert3r1_e2e.rs`
- `crates/geom-core/tests/cert3r1_probes.rs`
- `crates/geom-core/tests/cert4r2_probes.rs`
- `crates/profile/tests/cert4r2_e2e.rs`
- a `local-only` CERT-4 R2 probe in `crates/topo/src/chord_join.rs`
  (the doc line "CERT-4 R2 probe (local-only)", ~:2929)

**`*dump*` files not yet read** (eleven): `geom/tests/curves/{n1r1_c24_dump,n1r2_dump}.rs`;
`sweep/tests/{bitdump,shell10_r2_dump,shell5_r1_dump,shell7_dump,shell8_dump,shell9_r2_dump,shellfix1_bitdump}.rs`;
`viewer/tests/debug_dumps.rs`; `demos/tour/src/uvdump.rs`.

**Read, in `crates/editor-core/tests`** (by `dup/b9-a`, which closed
the parent row):

- `cert3r1_dump.rs` and `r2_cert3_coord_dump.rs` were both replicas of
  `m10_p_fence`'s corpus walk. Both now print over
  `m10_p_fence::walk` and are `#[ignore]`d. Done.
- `switch_dump.rs` is already `#[ignore]`d with a run note. It writes
  each document's payload `Debug` and name tables to files, which is
  not the fence's digest stream. No action seen.
- `lib_tube_r1_dump.rs` runs by default. It asserts one thing, that
  `tube_ring`'s save names `"Tube"`, and writes the save only when
  `R1_DUMP` is set. Open: whether that assertion is already held by
  the persistence round trip over the corpus. If it is, the file is a
  helper for a probe that has been run and can be `#[ignore]`d or
  dropped.

## Why it sits here

Ownership splits. `python3 scripts/work.py territory --files -` gives
every `crates/*/tests` path above to `tcost` and `tint`, the
`profile` harness to `paths` as well, `viewer/tests/debug_dumps.rs` to
`chrome` and `vdoc` as well, `crates/topo/src/chord_join.rs` to
`reach` and `tang`, and no program for `demos/tour/src/uvdump.rs`. The
question on every file is whether a row that cannot go red on an answer
should run, which is S-TINT's charter (`tcost`'s keep_out: "a row
justified by a claim that cannot fail is S-TINT's"). A file that turns
out to replicate a live suite's walk is S-DUP's half, as
`cert3r1_dump` was. `chord_join.rs` is `src`, so a change there is
announced to `reach`/`tang` before it lands.

## Cost

M: sixteen files, each read and dispositioned (keep and say what it
guards, `#[ignore]` with its run command, or delete), and any replica
among them folded onto the suite it copies. Neither census can see a
harness named without `dump` that does not call itself local-only.
