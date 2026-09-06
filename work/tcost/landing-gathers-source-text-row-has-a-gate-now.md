---
id: landing-gathers-source-text-row-has-a-gate-now
kind: issue
title: landing_gathers.rs's source-text row over product.rs's gather counter now duplicates a CI gate on the gatedness axis
status: open
opened: 2026-09-06
---


## Finding

Filed by the GATES orchestrator from PR 2030's style review (Q1).
`crates/viewer/tests/landing_gathers.rs::every_site_of_the_gather_counter_carries_the_debug_gate`
reads `crates/editor-core/src/product.rs` as text and pins that the
gather counter's sites are under `#[cfg(debug_assertions)]`. Since PR
2030 `scripts/gates/bit-identity-debug-only.sh` scans the same file for
the same question with a real enclosure analysis, so on the gatedness
axis the row is a second reader. What the row pins that the gate does
not: presence (the gate is green on a subject with zero hits), an exact
count of three (today the only mechanical check that `GATHERS` occurs
nowhere else in the file, the gate's KNOWN GAP 3 premise), and exact
attribute-to-statement adjacency (an `#[allow]` between them breaks the
row and not the gate). The decision — retire the row, or narrow it to
the presence-and-count half and cite the gate for gatedness — is a
test-mechanism call on S-TCOST's glob (VIEW's file by charter); the
row's `crates/test-utils/tests/reader_census.rs` ledger line moves
with it. Sequenced after PR 2030 lands.
