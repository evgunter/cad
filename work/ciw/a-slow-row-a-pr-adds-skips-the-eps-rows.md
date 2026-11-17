---
id: a-slow-row-a-pr-adds-skips-the-eps-rows
kind: issue
title: A slow row a PR adds runs at the default eps only, so its 1e-6 and 1e-12 rows wait for the nightly
status: open
opened: 2026-11-17
priority: P3
cost: M
refs: [7078]
---

Found by a review of PR 7078 (lane `band-reach`). `docs/prompts/implementer-discipline.md` §2 says the per-PR gate runs "the slow tests your diff adds or edits" and "the 1e-6 and 1e-12 rows of the eps-sensitive crates your diff touches and of every test file it adds or edits". The two meet only at the default ε. The ε rows run the `ci` profile, which drops the slow set, so a slow row the diff adds runs once, at 1e-9. Its 1e-6 and 1e-12 behaviour is first read by the nightly.

PR 7078 added nine slow rows in `crates/sweep/tests/band_mixed_corner_reach.rs`, and one of them pins a branch that exists only at 1e-12 (the inset fillet's `shared_row_touch` escalation at `Interval`). To get that branch gated per PR, the lane had to add a fast row (`the_inset_witness_fillets_or_escalates_at_every_eps_row`).

What a fix needs: the ε rows run the slow tests the diff adds, as the default row does. Alternatively, §2 says plainly that they do not.
