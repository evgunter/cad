---
id: coset-translation-finiteness-reads-the-norms-length
kind: issue
title: the coset fold's translation finiteness asks is_finite_length of its norm, which waves an enclosure through: a candidate only, the norm may already be bounded where it is built
status: open
opened: 2026-10-13
priority: P3
cost: E
---


**A candidate, not shown.** The coset fold (`crates/editor-core/src/mate/coset.rs:1320`)
refuses `FoldStop::OutOfRange` when
`!is_finite_length(translation.norm())`. `is_finite_length` is the
length reading, and it waves an enclosure through by design, so at
`Interval` an unbounded translation could pass. Whether it can depends
on what `candidate_translation` builds. If it returns a point value, or
an enclosure already bounded, there is nothing to fix.

`Real::is_bounded` (`crates/geom-core/src/real.rs:385`, PR #4197) is the
question for "is this translation a finite number" at every scalar. SYM-17
made `RigidMotion`'s mints and its decided door ask it.

Found by SYM-17's final pass, in the sweep for `x − x` / `0·x`
finiteness spellings. First, establish whether the enclosure can be
unbounded here.
