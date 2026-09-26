---
id: the-arc-carrier-fence-fixture-is-written-twice-in-editor-core-tests
kind: issue
title: The m10-p arc-carrier fixture (rocker eye and vesica lens programs) is written twice in editor-core tests
status: open
opened: 2026-09-26
priority: P4
cost: E
refs: [cert3r1-dump-is-a-print-only-replica-of-the-m10-p-fence]
---


## Finding

Filed by S-DUP's Step-embed fold (2026-09-26, `dup/b6-c`). That unit
routed both files' lift onto `profile::Step::map_scalar` and left the
programs where they were.

`crates/editor-core/tests/m10_p_fence.rs`'s `fixture_digest` and
`crates/editor-core/tests/cert3r1_dump.rs`'s `fixture_walk` each build
the same `programs` array: the rocker eye and the vesica lens, built
through `Open.arc_fillet_arc(Center {..}, 0.5, Center {..}, ..)`.
Measured at `032999ff2`, `diff` of the two `let programs = [ … ];`
blocks is **empty**. `fixture_walk`'s doc calls itself *"replicated
verbatim from `m10_p_fence.rs`"*. The line that lifts each program
(`closed.program.iter().map(|s| s.map_scalar(T::from_f64)).collect()`)
is also the same in both files. It is one iterator adaptor over the
door, not a construction, and is not this row.

## Why it is filed and not folded

- **The tint row may delete one of the two files.**
  `work/tint/cert3r1-dump-is-a-print-only-replica-of-the-m10-p-fence.md`
  asks whether `cert3r1_dump` should run at all. If it goes, this row
  closes with it, and a shared home would have one consumer.
- **The shared home is a call to make.** Both files reach
  `tests/corpus/` and `tests/fixture/`. `corpus/mod.rs` is chartered
  for `DocEdit` documents (*"Every document here is a RECIPE authored
  through `DocEdit`s"*), and this fixture is a profile program that
  the fence's own doc describes as the one the corpus does not contain.
  Both directories are also symlinked into `crates/viewer/tests/`, so
  a program list homed there compiles into the viewer binary too.

**Take it** after the tint row decides. If both files stay, home the
list as one function in `m10_p_fence.rs`, the fence and the original,
reachable as `crate::m10_p_fence::…`, or in `tests/fixture/` with the
viewer symlink in mind.
