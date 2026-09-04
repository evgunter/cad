---
id: geom-brep-test-unused-edgedescription-import
kind: issue
title: geom-brep test binary carries an unused EdgeDescription import visible only under --all-features
status: review
opened: 2026-09-01
github: 1525
refs: [1517, 1523]
pr: 1795
branch: ciw/all-features-clippy-row
---

## From GitHub issue 1525

Opened 2026-09-01; 0 comments.

Recorded from MESH-4's review cycle (PR [#1517](https://github.com/evgunter/cad/pull/1517); both reviewers flagged the disclosed-but-unscheduled deferral): `cargo clippy --workspace --all-targets --all-features` reports `unused import: EdgeDescription` in a `geom-brep` test binary. It is invisible at default features, so no hosted gate reds on it — the same "a feature combination nobody builds" family as the editor-core probe+interval break fixed in PR [#1523](https://github.com/evgunter/cad/pull/1523) (whose gated build DID exist; this one's doesn't). The fix is a one-line import trim wherever `--all-features` compiles the use away; worth pairing with a look at whether any hosted row should build `--all-features` clippy so the class stays visible.

VERBS is live in `geom-brep` — theirs to take or to wave through a drive-by.

## Home

`work/verbs/` — the issue names VERBS as the live claimant in `geom-brep`, whose `intersect.rs`/`ssi*`/`offset*` files are VERBS' territory.

## Re-measured (2026-09-04, CIW): four imports, and not in the file named above

`cargo clippy -p geom-brep --all-targets --all-features` on this tree,
exit 0 with four warnings — `geom-brep` aggregates its suites into one
test binary (`all`), which is why the issue could not name the file:

| file:line | unused import |
|---|---|
| `crates/geom-brep/tests/span_meter_dim_twins.rs:47` | `EdgeDescription` |
| `crates/geom-brep/tests/span_meter_dim_twins.rs:51` | `geom_core::Band` |
| `crates/geom-brep/tests/rim_dim_review_probes.rs:36` | `geom_core::Tol` |
| `crates/geom-brep/tests/rim_dim_scale_twins.rs:61` | `geom_core::Band` |

All four are invisible at default features, so no hosted row reds on
them — the class, not the instance, is the point, and it has grown from
one to four since filing.

**The split, restated now that both halves have owners.** The one-line
trims are `geom-brep` test files and VERBS is live there (PRs 1674,
1671) — theirs to take, or to wave through as a drive-by. The CI half —
whether a hosted row should build `--all-features` clippy so this class
stops accumulating unseen — is CIW's, and is what CIW schedules. Note
the two are independent: the row can land red-then-green in either
order, but landing the row BEFORE the trims means the row's first run
is red on someone else's files, so the trims go first or the two go
together.

Cost of the row, now measurable rather than guessed: the repository
went public 2026-09-03, so standard-runner minutes are free and the
question is wall clock only. `-p geom-brep --all-targets
--all-features` was 24.6 s warm on a 4-vCPU box here; a workspace-wide
`--all-features` clippy row is the figure to take before proposing one,
and `viewer`'s `app` feature (~140 eframe/wgpu crates) is the term that
decides it — `ci.yml` already treats that graph as a seed-keyed axis
(`clippy (viewer app feature)`), so the row this item wants may want
the same treatment rather than a flat `--all-features`.
