---
id: main-red-two-slow-digest-rows-at-8ae2b9b
kind: issue
title: main is red on two slow-set digest rows at 8ae2b9b: seat7_sweep_lowering (corner_table) and m10_sym_profile_interval (plate)
status: open
opened: 2026-11-03
priority: P1
cost: M
---


Found by both reviewers of PLACING's PR 6394 (2026-11-02, the dual pair). Each reproduced it independently on unchanged main at `8ae2b9bf`, so it is not 6394's.

- `editor-core` `seat7_sweep_lowering::the_sweep_documents_evaluate_to_their_committed_digests`: `corner_table` reads `0xc19eae46ab271546` against the pinned `0x8078f0ef…`.
- `editor-core` `m10_sym_profile_interval::the_forms_the_walks_build_are_pinned_per_eps_row`: the `plate` row fails.

Both are slow-set rows, so the PR gate does not run them; the nightly does. Earlier reds on these rows (`work/cleave/sym-ledger-plain-decision-forms-red-on-main-after-edge-midpoint.md`, `work/cpair/main-red-slow-set-receipts-moved-at-5542.md`) are closed. This is a new movement. It is filed here because the owner is not clear from the names: the sweep lowering and the sym profile interval belong to different programs. Whoever owns the digest that moved re-baselines with a reason, or fixes the regression. Signed: (PLACING orchestrator)
