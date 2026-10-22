---
id: a-certified-leaf-that-diverges-on-replay-refuses-typed-as-a-kernel-defect
kind: issue
title: stackup's LeafDiverged refuses typed a replay-identity break D9 calls a kernel bug; the D9 test has not been asked of it
status: open
opened: 2026-10-20
priority: P3
cost: E
---


Filed by `wire-evalarms` (WIRE), from the sweep for typed kernel-defect
refusals that D9's rewrite (PR 5411) left in `crates/editor-core`.

## Finding

`StackupRefusal::LeafDiverged` (`crates/editor-core/src/stackup.rs`,
built in the certified-leaf replay where `.map_err(|(node, cause)|
StackupRefusal::LeafDiverged { .. })` runs) documents itself as "a D9
replay-identity break, surfaced typed rather than swallowed", and says
a foreign or edited document is caught before it by the tie
(`SensitivityRefusal::VerdictNotOfThisBuild`).

D9 `[failure]` as ratified on PR 5411: a state only a kernel bug reaches
panics (row 4), and a site a correct kernel can reach takes its true
row's ending instead. A replay-identity break is the first, unless some
input reaches it after the tie (a lane refusal at the replay scalar
that the drive's own pass did not meet, say), which makes it the
second.

## What to do

Ask the D9 test of the arm: can a correct kernel reach it from any
input? If not, the replay's refusal becomes an `unreachable!` whose
message carries the leaf and the cause, and the arm goes (with its
pncad-py tag and census rows). If so, name the input, and the arm keeps
a row-1/2 ending that says what to do.
