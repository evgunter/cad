---
id: display-budget-include-str-names-v15
kind: issue
title: main does not compile: viewer's display_budget.rs include_str! names gallery_ring.v15.pncad, renamed to v16
status: open
opened: 2026-08-29
github: 1272
refs: [1247]
---

## From GitHub issue 1272

Opened 2026-08-29; 0 comments.

Found by the VERBS-GERMARMS lane's full local battery (hosted CI's sampled matrix does not necessarily draw the row): `f0214ea8` (#1247) added `crates/viewer/tests/display_budget.rs` with `include_str!("gallery_ring.v15.pncad")`; `4d63a013` renamed that golden to v16. Both are ancestors of main — a full `cargo test --workspace` fails to compile at HEAD. Deliberately NOT fixed by the finding lane: the include names a golden whose contents the test's assertions may depend on, so repointing is a claim about the test's subject — the owning program should make it. @ gui / whoever owns crates/viewer.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_016pYMaeU4woYZN8YGdTLfSK

## Home

`work/issues/` — `crates/viewer/tests/display_budget.rs` is GUI-era ground and that program is closed.
