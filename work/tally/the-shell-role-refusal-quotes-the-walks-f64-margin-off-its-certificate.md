---
id: the-shell-role-refusal-quotes-the-walks-f64-margin-off-its-certificate
kind: issue
title: A shell-role refusal quotes the f64 walk's V/A, 7% off the certified enclosure on a near-tangent sliver lump
status: open
opened: 2026-10-09
priority: P3
cost: M
refs: [lane-free-volume-sign-reads-decide-on-a-rounded-sum]
---


## What

Found by the door-typing unit (branch `join/door-types-in-band-results`).

When the f64 walk and the interval re-derivation both leave a shell's role
unread, `certify_role` reports the walk's reading. It does this on
purpose, so the refusal quotes a valued margin. On the near-tangent
census's witness lump (`notch307 nt e0 a3 d1e-8 pc I`, ε = 1e-9), that
value is 7% off:

- **walk:** V = 6.0650e-16, A = 1.98836e-7, so V/A = 3.0502e-9;
- **certified:** V ∈ [6.53916707e-16, 6.53916734e-16], so V/A ∈
  [3.288716e-9, 3.288716e-9];
- **the pose's own geometry**, clipped about `v` in f64
  (`offer_rows.rs` `SliverLump::thickness`): V = 6.53916713e-16 and
  V/A = 3.2887163e-9.

The decision is sound: it reads the certified enclosure. The number a
refusal quotes is not. Its error is about 2.4e-10 on a model a few
metres across, far beyond the few ulp Q1's conditioning premise
presumes. The tolerance the refusal offers (below |m|/K) inherits the
error. The door's own refusal quotes the certified enclosure
(`finding_arm`), but `validate`'s and `classify_shells`' refusals still
quote the walk's value.

The measurement row's oracle V/A of 3.87e-9 for the same lump
(`near-tangent-boolean-results-ship-with-an-escalated-tier-3-census`) has
the same cause: a volume summed in f64 about the world origin.

## The shape to give

Have the refusal quote the certified reading wherever one was taken. Or,
if a valued margin is wanted, take the walk's sums about the body's
corner, as the re-derivation does.
