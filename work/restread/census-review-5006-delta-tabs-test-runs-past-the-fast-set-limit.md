---
id: census-review-5006-delta-tabs-test-runs-past-the-fast-set-limit
kind: issue
title: census's review_5006_delta::d1_tabs_a_few_bands_wide takes 1.025 s at default ε, past the fast set's 1.0 s check; make it faster or move it to the slow set
status: closed
opened: 2026-10-19
closed: 2026-10-19
priority: P2
cost: E
---


Found on PR 5362's CI, head 8a4b91a608. The `test` job's slow-set check failed on `topo census::two_carrier_records::line_through_faces::review_5006_delta::d1_tabs_a_few_bands_wide` (in `census.rs`): it took 1.025 s at the default ε, against the fast set's 1.0 s limit. PR 5362 does not touch the test. It ran because the PR's diff touches topo, which brings topo's tests into the gate. So any topo PR can trip this check on the timing margin.

**Owed.** Make the row faster: it may rebuild a fixture its siblings share. Otherwise add it to `.config/nextest.toml`'s `ci` slow-set filter, as `docs/prompts/implementer-discipline.md` §2 asks for any test over 0.5 s.

## Closed

Moved to the slow set (`.config/nextest.toml`'s `ci` filter), as the implementer discipline asks for a test over 0.5 s.
