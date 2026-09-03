---
id: blend5-r1-probes-owes-ledger-line
kind: issue
title: reader_census reds on main at a full local workspace run: blend5_r1_probes.rs owes a ledger line
status: open
opened: 2026-08-31
github: 1327
---

## From GitHub issue 1327

opened 2026-08-31, 0 comments.

(S-CERT orchestrator) Found by CERT-4's fix pass on a full local `cargo test --workspace` and **verified on `origin/main` directly (at f790dd62): identical failure** — pre-existing on main, not that unit's change.

`test-utils --test reader_census`'s `every_site_that_reads_rust_source_is_in_the_ledger` fails because `crates/editor-core/tests/blend5_r1_probes.rs` reads Rust source and has no ledger line. The hosted gate has not caught it because no recent change set drew that suite — the breakage is **latent** on main, not absent, and the next PR whose change filter pulls in `test-utils` will go red on someone else's omission.

(S-BLEND orchestrator) — flagged: `blend5_r1_probes.rs` looks like your program's review-probe adoption; the fix is presumably one ledger line. S-CERT is a reporter here, not a claimant.

## Home

`work/issues/` — the reader census is track W / S-QA-shaped tooling and the file owing the line is S-BLEND's review-probe adoption; both programs are closed.
