---
id: the-parity-reader-does-not-compare-test-name-filters
kind: issue
title: The parity reader compares flags and env but not a cargo test's positional name filters, so a mirrored row can run fewer tests on one half
status: open
opened: 2026-09-26
priority: P2
cost: D
---


The release-profile row: hosted `.github/workflows/ci.yml` (~3399) runs `cargo test --release -p topo --lib -- review_m1_pr2::release_corruption review_m1_pr4::kill_ops_survive_torn_bodies_without_panicking review_d18`. The local half, `local-scripts/ci-local.sh` `topo_release` (~625), ran the same command without `review_d18`. `review_d18` holds two release-only hammer rows, and the hosted comment calls this job "the ONLY lane that runs them". So a local run of that row never ran them, and `scripts/check-ci-mirror-parity.py` passed.

The reader's claim 10 compares flags (tokens that start with `-`) and a SEMANTIC_ENV allowlist. It does not compare the positional arguments after `--`, which are cargo test's name filters. A filter dropped from one half, or added to it, shrinks what that half runs while keeping the row's name, flags and citation.

Found by S-DUP's sixth-batch local gate, 2026-09-26. The same row was also missing the hosted step's `CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS: "false"`, which reds the row outright, because the workspace's release profile keeps debug assertions on. S-DUP's batch PR fixed both halves of that row. It also added the variable to `SEMANTIC_ENV`, following the reader's own rule that a semantics-bearing variable is added "in the diff that sets it". That covers the env half. This row is the filter half, which is still open.

Fix direction: treat the name filters after `--` as a token class alongside flags, with the same asymmetry table and expiry arm. Then re-read every mirrored `cargo test`/`nextest` pair for filters that already disagree.
