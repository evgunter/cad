---
id: nightly-rustdoc-gate-red-on-main
kind: issue
title: the nightly's rustdoc (gate, every root) job is red on main, and no row names its cause
status: closed
opened: 2026-10-28
priority: P1
cost: E
closed: 2026-10-28
---


## What

Nightly 74 (run 40637814390, main `a1cfd1cfdb`, 2026-10-28) fails
`rustdoc (gate, every root)` (job 122649031004) with `Process completed
with exit code 1` after the `demos/tour` root documents; the log's last
60 lines carry no diagnostic, so the failing root and warning are
further up the log. No row on any slate names this job (grep over
`work/` for `rustdoc (gate`, 2026-10-28).

## Owed

Read the job's log for the first `warning:`/`error:`, find the merge
that introduced it, and fix it at its site. If the root cause is a gap
in the per-PR doc gate (a root or a cfg the PR row does not document),
say so in the PR and file it on CIW.

## Closed (2026-10-28)

**Nightly 74's cause.** The full job log's first diagnostic is in the
workspace pass, at `geom-brep`: three broken links, after which cargo
stops documenting every crate downstream, so the `##[error]doc-gate:
rustdoc rejected the workspace pass` annotation sits ~200 lines above
the tail (the later roots run on and pass). PR 5917 (merge `2a2518c4`,
triage) had already fixed those three and the six more downstream ones
the next pass would have hit. Introducing merges (first-parent `-S`):
5810 (`[at_rest]` ×2 in `editor-core/src/checks.rs`, `[crate::assemble]`
in `mate.rs`), 5872 (`[crate::pair_section]` and
`[geom_brep::pair_section]` ×2 ambiguous once `pair_section` became both
a module and a function), 5885 (`[without]`, a `cfg(test)` item), 5868
(`[crate::dihedral_side]`, ambiguous), 5741
(`[topo::SignCertificate::detach]`), 5856 (`[Sector::cleared]`).

**Still red on current main.** `scripts/doc-gate.sh` on main `4beeeb05`
exits 1: `crates/topo/src/chord_join.rs`'s `IslandReading::closed_by`
links `[Self::new]`, which is `#[cfg(test)]`. Introduced by PR 5894
(merge `34051ed3`, 2026-10-28 11:44 UTC, after nightly 74 ran), so
tonight's nightly would have been red on it. Fixed on PR 5943 (branch
`green/small-reds`): the substantive doc moves onto `closed_by` and the
test-only `new` points at it. `scripts/doc-gate.sh` then exits 0 over
every root.

**Why no per-PR gate caught it.** There is no per-PR rustdoc pass by
design: the latency cut (`work/ciw/latency-cut.md`) moved rustdoc to the
nightly. Not a gap in a gate that should have fired, so no CIW row.
