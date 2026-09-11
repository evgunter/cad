---
id: cert10-strict-gap-floor-gates-on-a-varying-seed
kind: issue
title: nurbs_cert_fuzz's cert10 strict-gap floor gates hosted CI over a per-run varying seed and fails at some seeds
status: open
opened: 2026-09-05
---


## What

`crates/mesh/src/nurbs_cert_fuzz.rs:247-253` asserts `strict > trials`
over a randomly seeded sweep whose seed varies per run (`:87`, `:106`).
On PR 1919's head `38dd4692` — a diff that does not touch `crates/mesh`
— the `k-lint (gate, dev-budget)` row drew seed `0x3f9f92d266ab512c`
and got **59 strict of 300 comparisons**, one short of the floor
(`trials = 60`). The TOPO lane reproduced it locally at that seed with
its own changes reverted, so the draw is the whole cause: the floor is
a distribution claim gated on one sample, and some seeds fall under it.

The shape question `memories/test-suite-cost.md` asks — which shape is
this row, and should a seed vary under a gate — is this program's to
answer: pin the seed and keep the floor, keep the varying seed and
report rather than gate, or derive the floor from the draw. What
should not stand is a hosted gate that reds unrelated PRs at a rate
nobody has measured.

Reported by the `topo/d261-reader-collapse` lane in its PR body
(run 33945662993); filed by the TOPO orchestrator, 2026-09-05.
