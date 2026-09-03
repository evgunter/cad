---
id: geom-brep-test-unused-edgedescription-import
kind: issue
title: geom-brep test binary carries an unused EdgeDescription import visible only under --all-features
status: open
opened: 2026-09-01
github: 1525
refs: [1517, 1523]
---

## From GitHub issue 1525

Opened 2026-09-01; 0 comments.

Recorded from MESH-4's review cycle (PR [#1517](https://github.com/evgunter/cad/pull/1517); both reviewers flagged the disclosed-but-unscheduled deferral): `cargo clippy --workspace --all-targets --all-features` reports `unused import: EdgeDescription` in a `geom-brep` test binary. It is invisible at default features, so no hosted gate reds on it — the same "a feature combination nobody builds" family as the editor-core probe+interval break fixed in PR [#1523](https://github.com/evgunter/cad/pull/1523) (whose gated build DID exist; this one's doesn't). The fix is a one-line import trim wherever `--all-features` compiles the use away; worth pairing with a look at whether any hosted row should build `--all-features` clippy so the class stays visible.

VERBS is live in `geom-brep` — theirs to take or to wave through a drive-by.

## Home

`work/verbs/` — the issue names VERBS as the live claimant in `geom-brep`, whose `intersect.rs`/`ssi*`/`offset*` files are VERBS' territory.
