---
id: shell-of-hollow-body-thicken-every-boundary
kind: issue
title: shell of an already-hollow body - refused today; the ratified semantics is thicken every boundary
status: closed
opened: 2026-08-27
github: 1056
refs: [1048, SHELL-5]
closed: 2026-09-08
---

## From GitHub issue 1056

Opened 2026-08-27; 0 comments.

**Shelling an already-hollow body.** Raised by the OFF-D PR-2 review (#1048, ordinal 82, MAJ-2) and adjudicated with Ev.

**Measured behaviour before the gate.** `shell(shell(box, 0.25), 0.05)` returned `Ok` with FOUR shells, tier-3 valid, volume 4.362. The verb offset the operand's VOID shell along with its outer one and inserted both cavity-clone shells as new voids beside the existing one — the new voids overlap and contain the old void, and the `Carried { Positive }` containment evidence is false for the void-derived shell, which was never in material at all. `NotOneSolid` did not gate it because it counts solids, and a hollow body is one solid with two shells.

**What ships now.** `topo::shell` refuses `ShellError::OperandAlreadyHollow { shells }` when the operand's solid carries more than one shell, with a planted red on exactly the composition above.

**Ev's ruling on the eventual semantics, recorded verbatim as the requirement for whoever closes this:**

> the eventual resolution must be "thicken every boundary" — offsetting only the outer shell is explicitly rejected.

So shelling a hollow body must erode the outer shell INWARD and dilate each void shell OUTWARD, both by `t`, leaving a thin wall at every boundary the operand has. An implementation that offsets only the outer shell and leaves the existing voids untouched does not close this issue, and neither does one that refuses a hollow operand permanently.

The gate site in `crates/topo/src/shell.rs` cites this issue.

## Home

`crates/topo/src/shell.rs` is in VERBS' `paths:` territory and the shell verb is its Wave 3 ground.

## Closed (SHELL-5, PR #2159, 2026-09-08)

To the ruling verbatim: `shell` / `shell_open` on a hollow operand
erode the outer shell inward and dilate every void outward by `t`
through the one signed rule (`inward` reads the face's sense), and
return one thin solid per operand shell — `k + 1` solids for `k`
voids — re-partitioned out of the void door's graft by
`Body::move_shells_to_new_solid`, paired structurally off the graft
map. `OperandAlreadyHollow` is gone; the record carries `thickened`
and `RimNaming::side`. Rows: the hollow box, the two-void box, the
hollow vessel, both opened arms, the pillar-through-a-void ceiling
(`crates/sweep/tests/verbs_shell.rs`, `shell5_r1_probes.rs`,
`shell5_r2_probes.rs`). The planar clearance gate now grows footprints
by `t` before its separation decide (a pre-existing under-refusal both
reviewers reproduced on a notched single-shell operand, made common by
voids); the curved window (#1055) stays SHELL-4's and is pinned by a
self-retiring row.
