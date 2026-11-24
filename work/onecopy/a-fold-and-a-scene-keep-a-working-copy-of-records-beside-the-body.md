---
id: a-fold-and-a-scene-keep-a-working-copy-of-records-beside-the-body
kind: issue
title: The union fold's accumulator and the tour's scenes each keep a working copy of contact records beside the body that holds them
status: open
opened: 2026-11-22
priority: P2
cost: E
---


Found in review of PR 7124 (`embed/records`, 2026-11-22). The finished
body holds its contact records, and two places still keep a list of
their own beside a body:

- **The union fold's accumulator**, `acc_records` in
  `crates/editor-core/src/eval/wire.rs` (`fold_members`): the records of
  the fold so far, kept in step with the accumulated body, with an
  `unreachable!` where `Carry::contacts` finds the citation list and
  the body's records of different lengths
  (`crates/editor-core/src/eval/carry.rs`, `Citing::Each`).
- **The tour's scenes**, `SceneBody { body, contacts }` in `demos/tour`:
  a bare `Body` with its records as a public field beside it, read by
  the scenes' own tier-3′ pass.

Neither is read after its body changes, by inspection. The gather's
`product::Source.contacts` was a third and is gone (the gather reads
each source's records off its body). What each needs: the fold to read
its accumulated body's records and keep only how each cites
(`Citing`), as a value's carry does; the scenes to hold the finished
body.

**A third copy, with a check on it** (PR 7124, 2026-11-24). The cut's
tool material (`topo::boolean::material::assembled`) returns its
records twice: on the finished body, and as a list beside it, which is
what the cut reads. Nothing read the body's own, so a mutant that
finished the material with no records survived. Until the list beside
the body goes, the site reads it back onto the finished body after the
gate and refuses `JoinDesync` where the two name different pairs (as
sets, so two of the tool's rows on one pair are no fault). That check
goes with the copy.
