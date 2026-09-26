---
id: cert3r1-dump-is-a-print-only-replica-of-the-m10-p-fence
kind: issue
title: cert3r1_dump replicates m10_p_fence's corpus walk and arc-carrier fixture verbatim, prints instead of asserting, and calls itself never pushed
status: open
opened: 2026-09-26
priority: P4
cost: E
---


## Finding

Found by S-DUP's Step-embed fold (2026-09-26, `dup/b6-c`) while it
routed this file's `embed` closure onto `profile::Step::map_scalar`.
The rest of the file was outside that unit's fence.

`crates/editor-core/tests/cert3r1_dump.rs` (mounted from `tests/all.rs`,
so it runs on every test row) says of itself: *"Replicates the m10-p
fence's exact walk (`corpus_digest`), but PRINTS every observable
instead of digesting it … Local-only; never pushed."* It was pushed,
with the CERT-3 R1 review probes (`fbf0bb6a5`), and it is live:

- `walk` is `m10_p_fence.rs`'s `corpus_digest` loop with `println!` in
  place of `Digest` feeds.
- `fixture_walk` is `m10_p_fence.rs`'s `fixture_digest`, whose doc says
  *"replicated verbatim from `m10_p_fence.rs`"*. That includes the
  hand-built `arc_fillet_arc` program list, which is now spelled in two
  files.
- `r1_dump_f64` and `r1_dump_interval` assert nothing. They can red
  only on a panic, so every observable they print is unasserted by
  design: the file was a diff harness for one review, run on two trees
  and read by eye.

It is both a copy and a row that cannot go red on an answer, so it is
filed here first, on S-TINT's slate. Under that charter the question is
whether it should run at all: keep it (and say what it guards), gate it
out of the default suite as the harness it declares itself to be, or
delete it and leave `m10_p_fence` as the fence. If it stays, the
program list should have one home shared with `m10_p_fence.rs`, which
is S-DUP's half.

**Instrument**: reading the file. `git log -S'Local-only; never pushed'
--all` names `fbf0bb6a5` as the commit that wrote it.

**The class is wider than this file, and it was not read.** A second
pass ran over every tracked file with no path argument. Its numbers
are taken at the merge base `032999ff2` (`git grep … 032999ff2` and
`git ls-tree -r 032999ff2`) and are the same at the branch head:

- `git grep -n -i 'never pushed\|local-only' -- '*.rs'` finds five more
  review harnesses that call themselves never pushed and are in the
  tree: `geom-brep/tests/cert3r1_e2e.rs`,
  `geom-core/tests/{cert3r1_probes,cert4r2_probes}.rs`,
  `profile/tests/cert4r2_e2e.rs`, and a `local-only` probe in
  `topo/src/chord_join.rs` (~:2870).
- `git ls-files '*dump*.rs'` finds fourteen more `*dump*` files: three in
  `editor-core/tests`, two in `geom/tests/curves`, seven in
  `sweep/tests`, `viewer/tests/debug_dumps.rs` and
  `demos/tour/src/uvdump.rs`.

This row read none of them. Whether each one asserts, and whether it
replicates a live suite, is open. Neither pass can see a harness that
is named without `dump` and does not call itself local-only.
