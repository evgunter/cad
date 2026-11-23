---
id: the-one-foot-desync-is-blind-to-a-closed-components-seam
kind: issue
title: The foot lane's one-foot desync reads two feet one period apart on a closed component as one foot
status: open
opened: 2026-11-22
priority: P3
cost: E
---

Filed by PR 7354's review (SECTCONE U1). The check predates the PR.

## What

`crates/topo/src/boolean/join.rs`, `along_facing` (~4560–4568): `bool_join_foot_ahead` reads the travel `f2.t − f1.t` from the germ's foot to the partner's, the way the germ leaves. On a closed component a Negative reading wraps by the span (`forward + (hi − lo)`), and a Zero is a desync ("two distinct sites have one foot on their section").

Two feet one period apart, at `lo` and `hi`, are one point of a closed carrier. Their raw difference is `±(hi − lo)`: it reads definite, wraps, and gives a travel of about zero or a whole period. Two feet just either side of the seam read a travel of nearly a whole period. Neither is measured round the seam. The check, and the travel it hands `along_sign`, are blind to the seam.

## Done when

The foot travel on a closed component is read modulo its period, the shorter way being the seam-crossing one where the feet straddle it. A row pins two partners straddling a carried circle's seam, and two feet at `lo` and `hi`.
