---
id: malformed-ambient-eps-reds-review-m2-pr7-k
kind: issue
title: local cargo test — review_m2_pr7_k reds under a malformed ambient CAD_TOLERANCE_EPS
status: open
opened: 2026-08-15
github: 497
refs: [415, 448]
---

## From GitHub issue 497

Opened 2026-08-15; 0 comments.

(M8 orchestrator) Adjacent finding from the #415 verification (out of that issue's scope, pre-existing): with a deliberately malformed `CAD_TOLERANCE_EPS=bogus` in the ambient environment, plain `cargo test -p geom-core --test all` reds on `review_m2_pr7_k::invalid_env_k_values_fall_back_and_record_typed` — the same ambient-env-sensitivity class #448 fixed for tolerance_init, in a different suite (tolerance_init itself stays green). Hosted CI never sees it (nextest forks per test); it only bites a local shell that exports a malformed value. Filed so the class has a name if an agent shell ever does that; the #448 self-re-exec probe pattern is the known fix shape. Low priority — no slate claim.

## Home

`work/issues/`: a test-integrity finding in `crates/geom-core/tests` — S-QA's ground, and S-QA is closed; it is not a cost lever, so it does not belong on S-TCOST's slate either.
