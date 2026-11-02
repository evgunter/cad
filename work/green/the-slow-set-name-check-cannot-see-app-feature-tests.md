---
id: the-slow-set-name-check-cannot-see-app-feature-tests
kind: issue
title: the slow-set name check lists tests without the app feature, so a slow viewer app test cannot be put in the slow set
status: open
opened: 2026-11-02
priority: P4
cost: E
---


Found by PLACING's `placing/mate-kinds` fix pass (PR 6284, 2026-11-02).

**Symptom.** `crates/viewer/src/app.rs`'s panel row `an_edge_direction_mate_at_the_panels_default_keeps_its_copy_upright` takes 0.69 s. `docs/prompts/implementer-discipline.md` §2 says a test costing ≥ 0.5 s goes in `.config/nextest.toml`'s slow set. But CI's check that every slow-set entry matches a test (`slow-set entry matches no test`, in the `test (app)` job) lists the tests without the viewer's `app` feature. So the entry is rejected, and the lane had to leave the row out of the slow set; it runs on every PR that reaches the app job.

**Owed.** List the tests with the features each job runs them under, so an app-feature test can be named in the slow set. P4: tooling whose payoff is the suite costing less. Signed: (PLACING orchestrator)
