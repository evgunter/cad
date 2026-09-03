---
id: shell-of-hollow-body-thicken-every-boundary
kind: issue
title: shell of an already-hollow body - refused today; the ratified semantics is thicken every boundary
status: open
opened: 2026-08-27
github: 1056
refs: [1048]
---

## From GitHub issue 1056

opened 2026-08-27, 0 comments.

**Shelling an already-hollow body.** Raised by the OFF-D PR-2 review (#1048, ordinal 82, MAJ-2) and adjudicated with Ev.

**Measured behaviour before the gate.** `shell(shell(box, 0.25), 0.05)` returned `Ok` with FOUR shells, tier-3 valid, volume 4.362. The verb offset the operand's VOID shell along with its outer one and inserted both cavity-clone shells as new voids beside the existing one — the new voids overlap and contain the old void, and the `Carried { Positive }` containment evidence is false for the void-derived shell, which was never in material at all. `NotOneSolid` did not gate it because it counts solids, and a hollow body is one solid with two shells.

**What ships now.** `topo::shell` refuses `ShellError::OperandAlreadyHollow { shells }` when the operand's solid carries more than one shell, with a planted red on exactly the composition above.

**Ev's ruling on the eventual semantics, recorded verbatim as the requirement for whoever closes this:**

> the eventual resolution must be "thicken every boundary" — offsetting only the outer shell is explicitly rejected.

So shelling a hollow body must erode the outer shell INWARD and dilate each void shell OUTWARD, both by `t`, leaving a thin wall at every boundary the operand has. An implementation that offsets only the outer shell and leaves the existing voids untouched does not close this issue, and neither does one that refuses a hollow operand permanently.

The gate site in `crates/topo/src/shell.rs` cites this issue.

## Home

`crates/topo/src/shell.rs` is in VERBS' `paths:` territory and the shell verb is its Wave 3 ground.
