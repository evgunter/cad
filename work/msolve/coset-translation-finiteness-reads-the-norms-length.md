---
id: coset-translation-finiteness-reads-the-norms-length
kind: issue
title: the coset fold's translation finiteness asks is_finite_length of its norm, which waves an enclosure through: a candidate only, the norm may already be bounded where it is built
status: closed
opened: 2026-10-13
closed: 2026-10-18
priority: P3
cost: E
---


**A candidate, not shown.** The bundle fold (`fold_bundle`,
`crates/editor-core/src/mate/solve.rs:790`; before stage 3 F it was
`coset.rs`'s `FoldStop::OutOfRange`) refuses `MateFault::PoseOutOfRange`
when `!is_finite_length(held.representative.translation.norm())`. `is_finite_length` is the
length reading, and it waves an enclosure through by design, so at
`Interval` an unbounded translation could pass. Whether it can depends
on what `mate_coset` and `ball_meets` build. If it returns a point value, or
an enclosure already bounded, there is nothing to fix.

`Real::is_bounded` (`crates/geom-core/src/real.rs:385`, PR #4197) is the
question for "is this translation a finite number" at every scalar. SYM-17
made `RigidMotion`'s mints and its decided door ask it.

Found by SYM-17's final pass, in the sweep for `x − x` / `0·x`
finiteness spellings. First, establish whether the enclosure can be
unbounded here.

## Closed (2026-10-18): the branch is retired

PR 5314 retires the fold's range check and `MateFault::PoseOutOfRange` with
it (`the-folds-out-of-range-refusal-has-no-execution-row`). At a point scalar
the bundle's arm bounds the meeting finitely. At an enclosure, a translation
that is no bounded number refuses where the placed copy's motion is minted:
`RigidMotion`'s mints ask `Real::is_bounded` (SYM-17). `fold_bundle`'s doc
states both.
