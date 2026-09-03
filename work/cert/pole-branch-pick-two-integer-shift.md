---
id: pole-branch-pick-two-integer-shift
kind: issue
title: chord_join sphere-pole branch pick hands shift_branch a two-integer shift at Interval when an entry azimuth lands on the previous exit
status: open
opened: 2026-08-30
github: 1305
refs: [1191]
---

## From GitHub issue 1305

opened 2026-08-30, 0 comments.

(S-CERT orchestrator) Found and documented in place by CERT-4 (issue 1191's unit — the disposition comment now lives at the site, `chord_join.rs`'s pole arm); filed so the open half has a durable home.

The pole arm's strictly-next/strictly-previous branch selection is a half-open-boundary jump, which is a different thing from a period fold written around its own live value — CERT-4 documented it and deliberately did not respell it. But at `Interval`, when a pole junction's entry azimuth encloses the previous exit exactly, the strict inequality cannot decide and the pick hands `shift_branch` a shift enclosing TWO integers. A strict-inequality selection has no better local answer; the open question is downstream: whether `shift_branch`'s consumer deserves a **typed refusal** on a non-singleton shift rather than the silent non-singleton it gets today.

D2-addendum classification is owed by whichever unit takes it (a refusal minted where today's behaviour is silent is the addendum's core case). Fence note: `chord_join.rs` is ground CERT-4 just landed on and CERT-8 (chart-stretch honesty) will visit; sequencing with S-CERT's slate is the natural home unless another program claims it first.

## Home

`work/cert/` — S-CERT's charter names interval-mode honesty and period folds, the issue names S-CERT's slate as the natural home, and CERT-4 just landed on `chord_join.rs` with CERT-8 due to visit it.
