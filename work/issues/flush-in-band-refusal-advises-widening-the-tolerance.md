---
id: flush-in-band-refusal-advises-widening-the-tolerance
kind: issue
title: topo FlushRefusal::PairInBand advises 'widen the tolerance' beside a geometry lever, which D4 ¶1 (i) rules out
status: open
opened: 2026-09-28
priority: P3
cost: E
---


## What

`crates/topo/src/flush.rs`, `FlushRefusal`'s `Display`, `PairInBand`
arm, ends "separate the geometry or widen the tolerance". D4 ¶1 (i)
(Ev, `[ev]` PR 3352, 2026-09-28): no refusal advises loosening ε unless
it would otherwise name no recourse, and this one names a geometry
lever (separate the geometry). So the tolerance clause should go.

The clause may also point the wrong way. An in-band pair is decided by
a smaller ε, not a larger one. D4 allows "tighten the tolerance" only
conditionally, with a value, and only on a decision that passes on a
nonzero sign. So whoever takes this should derive the arm's recourse from
the flush decision's own shape, rather than just deleting the clause.
Not investigated beyond reading the arm.

The arm also opens with a stage prefix (`flush detection:`) and names
the pair by arena key (`{:?}/{:?}`), against the CHROME refusal
standard (`work/chrome/error-and-check-text-overflows-its-region.md`).

Found by the ENCL kernel-limit last-resort sweep, whose second pass
searched for loosening spelled without the word "loosen". `flush.rs` is
unowned: `work/topo/program.md` says SEAT held it and it has been
unowned since 2026-09-06.

The ENCL row that found it is `work/encl/kernel-limit-refusals-name-loosening-without-the-bug-note.md`.
