---
id: a-cut-in-faces-merge-escalation-leaves-as-the-surviving-cut-refusal
kind: issue
title: A cut-in face's output-merge escalation is recorded as a skip, and the op then answers surviving_cut's refusal instead of the escalation
status: closed
opened: 2026-11-25
priority: P2
cost: M
refs: [a-section-loop-inside-two-one-band-curved-faces-refuses-at-the-section-certificate]
closed: 2026-11-26
pr: 7414
---



Found by PR 7575's review (LATHE, 2026-11-25, `lathe/two-curved-interior-loop`).

## What

The class is an escalation reported as a refusal. On a boolean that cuts a
sphere face in and re-enters (`crates/topo/src/boolean/ops.rs`,
`reenter_recut`), the output merge runs under
`GroupRegime::RecordsASkip` (`crates/topo/src/merge_faces.rs`). There,
any group refusal that is no arena fault is recorded as a
`SkippedMerge`, escalations included. One example is a
`MergeCoplanarError::Pcurve { source }` from `curved_outline_ring`'s mint
of a re-cut face, whose `source` holds an in-band certify reading. The
cut that group would have glued away then survives the merge, so
`reenter_recut`'s `surviving_cut` finds it and the op answers
`FallbackExtentUnsupported` ("the meridian the sphere face was cut in
along survives the output merge") or the interior-loop guard's
refusal. Either is a refusal: the escalation the merge recorded is never
surfaced, and a tighter tolerance that would decide it is not offered.

## Witness

`crates/topo/src/boolean/ops.rs`, `reenter_recut`: the
`surviving_cut(..)` branch reads the result body and never reads
`out.naming.merge_skipped`. It is not demonstrated on an input: no
row today drives the role pass's mint into band. The path is read off
the code.

## Route

In `reenter_recut`, before raising `survives(plane)`, look for a
`SkippedMerge` in `out.naming.merge_skipped` whose faces include the
surviving cut's pieces (`piece_of` reads them) and whose `reason` is an
escalation: `MergeCoplanarError::Escalated`, or a `Pcurve { source }`
whose source escalates. Return that escalation (through the boolean's
merge-escalation spelling) instead of the refusal. Only a skip whose
reason is structural should answer as the surviving-cut refusal.

## Re-grounded (2026-11-26, LATHE, the main merge's port)

`reenter_recut` and `surviving_cut` are gone. On main's cut-in framework,
the output stage closes every cut the kernel made
(`topo::boolean::slit::close_cuts`) before the output merge runs
(`ops.rs`, `finish_output`), so no cut reaches the merge, and no
`SkippedMerge` stands in for one. An in-band reading in the closing
(`slit::pole_beyond`, `bool_cut_close_pole_side`) raises
`BooleanError::Escalated` as itself. The class this row records, an
escalation answered as the surviving-cut refusal, has no path left. The
crossings path decides that reading's premise before it cuts in
(`interior_cuts::closing_holds`, which escalates the same question in
band). What remains for the orchestrator is to close the row.

- 2026-11-26 — Closed as moot (LATHE orchestrator), landed with PR 7414. The port onto ORBIT's closed cut deleted `reenter_recut` and `surviving_cut`. A cut-in now closes through `slit::close_cuts`, and a cut its chase misses refuses typed `JoinDesync` in every profile. On both paths, the merge's minting escalation propagates through `cut_in_made` as an escalation. There is no surviving-cut refusal left to mask it.
