---
id: gating-nextest-jobs-discard-every-passing-tests-stdout
kind: issue
title: Record on the gate that a passing test's stdout reaches no reader, and keep it that way
status: open
opened: 2026-09-15
priority: P4
cost: E
---

`.config/nextest.toml`'s `[profile.ci]` sets `failure-output = "immediate-final"`
and no `success-output` (nextest's default is `never`), and no `cargo nextest
run` in `.github/workflows/ci.yml` or `nightly.yml` passes `--success-output`.
So `test_utils::vacuity::stood_down` (`crates/test-utils/src/vacuity.rs`) and
the `*_lane_skipped_*` marker bodies print to a local run only. That is the
right posture: the profile's own comment says a session reads a job log only
through its last 5000 lines and one run prints ~14 000 PASS lines, so printing
every passing row's stdout would bury the failure output the gate exists to
show. `vacuity.rs` already says a stand-down has no reader on the gate.

Owed: one comment line beside `failure-output` in `.config/nextest.toml`
stating that `success-output` is left at `never` on purpose and why (log
volume against the 5000-line read), pointing at `vacuity.rs`'s section, so
the next reader does not re-open the question. No workflow change.

What would close it: that comment in `.config/nextest.toml`.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): Options 1/2 (`--success-output`) cost no test time but flood the log: nextest.toml's
`[profile.ci]` comment says sessions read only the last 5000 lines of ~14 000 PASS lines, so
failure output would be buried — a latency cost in diagnosis. Grain: option 3, record the
decision in one comment. Reform: reform/gating-nextest-jobs-discard-every-passing-tests-stdout.md
