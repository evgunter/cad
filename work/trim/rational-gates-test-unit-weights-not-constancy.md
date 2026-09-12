---
id: rational-gates-test-unit-weights-not-constancy
kind: issue
title: The cap class's 'is this rational' gate tests weights == 1.0 where any constant weight vector is polynomial - over-strict by TRIM-1's own insight
status: open
opened: 2026-09-06
refs: [interior-iso-curve-de-boor-extractor, 2095]
---


## What

TRIM-1 (PR #2095) established that a constant weight vector cancels out
of the rational basis, so a row wrapped with any constant is the same
curve as its polynomial wrap. The cap class's gate in `pcurve_cache.rs`
still tests `any(|w| *w != 1.0)` and refuses a constant-weight carrier
as "rational". Found by the dual (R2 S1); the same shape sits in
`props/loop_area.rs`, `props/quad.rs`, `patch_bound.rs`, `mesh/chords.rs`
and `mesh/nurbs_cert.rs` (other programs' fences — reported on the away
channel, not filed here); `step-export/writer.rs`'s `== 1.0` is a
file-format arm and correct as is.

## Fix

The gate asks "constant", with the constant folded out; one row on a
constant-`2.0` carrier that certifies. E.

## Home

TRIM — `pcurve_cache.rs`.
