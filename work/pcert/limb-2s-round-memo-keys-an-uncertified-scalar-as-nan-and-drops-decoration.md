---
id: limb-2s-round-memo-keys-an-uncertified-scalar-as-nan-and-drops-decoration
kind: issue
title: Limb 2's round memo keys an uncertified scalar as NaN and drops a certified one's decoration, and keys by a rule of its own
status: open
opened: 2026-10-31
priority: P3
cost: E
---


Found in the review of PR 6062 (SECT `sect-seamslow`, 2026-10-31), whose
span-hull memo (`pcurve_cache::span_memo`) had the same shape and was
fixed there.

## Measured

`pcurve_cache::limb2_memo::Key::of` renders the carrier and the surface
through `map_scalar` at each scalar's certified bracket ends, NaN where
the scalar does not certify, beside `T::WITNESS`. Two things a round can
read are not in it:

- **An uncertified scalar's value.** Every uncertified scalar keys as
  NaN, while the round reads the carrier's knots and weights on their
  `f64` value channel (`carrier_net::framed_carrier_net` builds the net
  over them), so two carriers differing only in an uncertified knot or
  weight share a key.
- **A certified scalar's decoration.** `Com`, `Dac` and `Def` key alike,
  while certification arithmetic can take a different path on a
  decoration (`spline::blossom`'s `step`, through `same_enclosure`), so a
  hit need not be bit-identical to a miss, which `memo.rs`'s module docs
  promise.

`span_memo` closes both by keying what its body reads: it crosses its
net through `Interval::from_certified` before the memo (every scalar
then `Def` or `Trv`, so the bracket's presence is the decoration), and
keys the knots' and weights' value channel as `f64` bits. The two memos
now key by different rules: a `String` render with the witness kind, and
a `Vec<u64>` without it, which `span_memo`'s docs argue it does not need.

## What closing it takes

Key limb 2's rounds by what the round reads: cross the carrier's
scalars, or key the decoration and the value channel, and then one
keying rule for every `crate::memo` user, with the witness kind where
the memoized body computes in the scalar.
