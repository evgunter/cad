---
id: doctail
kind: program
title: DOCTAIL — the document model's tail: the edit door's reports and value checks, the save format, the flip lane's reading, refusal prose, name hygiene and the certified pick
status: ready
opened: 2026-10-02
group: api/eval
prefix: doctail/
tag: (DOCTAIL orchestrator)
ab_band: 10500-10599
paths: [crates/editor-core/src/resolve/hit.rs, crates/editor-core/src/resolve/pick.rs, crates/editor-core/src/persist/*, crates/editor-core/src/program.rs, crates/editor-core/src/doc.rs, crates/editor-core/src/edit.rs, crates/editor-core/src/node.rs, crates/editor-core/src/names/role.rs, crates/editor-core/src/names/emit_blend.rs, crates/editor-core/src/eval/parts.rs, crates/editor-core/src/eval/memo.rs, crates/editor-core/src/resolve/mod.rs, crates/editor-core/src/range.rs, crates/editor-core/REFERENCES.md, crates/editor-core/IDENTITY.md, crates/editor-core/src/placement.rs]
keep_out: [opened 2026-10-02 at EDIT's exit (Ev, in chat) as the lower half of EDIT's priority-seam cut, and took RECIPE's ground and live residue when RECIPE closed on 2026-10-11 (docs/doc-ledger/recipe-leaves-the-tracker.md) — INTENT builds D10 on the same ground (doc.rs, edit.rs, node.rs, persist/*) and every crossing either way is announced on the other's log, the names rows land on names/* where WIRE and EMIT also hold files, mate.rs and mate/* are MSOLVE's, the viewer's pick index is VGEOM's, docs/RECIPE-DOORS-DESIGN.md is LIB's despite the name — the README pages above are ratified design (DM1–DM8, DI1–DI5): a change to what a clause decides waits for Ev — the profile self-reference build (U1 U3 U4 U7 of Ev's 6289 ruling, filed here by VSEAM) left for SELFREF on 2026-11-17 and crosses doc.rs edit.rs node.rs persist/* from there with each crossing announced on this log — the default budget of 30 applies again since that cut, WIREREFUSE merged in on 2026-11-20 (docs/doc-ledger/wirerefuse-wirecost-and-wirehold-merge-into-doctail.md) - it held no P0 or P1 row, from WIREREFUSE: opened 2026-10-28 by splitting WIREREST (37 of 30) and WIRETAIL (34 of 30) after main re-weighted the cost tiers - the rows moved by git mv with ids and bodies unchanged, from WIREREFUSE: crates/editor-core/src/names/* and eval/* are shared ground with WIRE WIREREST WIRETAIL and NAMECHECK, from WIREREFUSE: node-error-kind-has-no-fieldless-projection's three consumer doors (drive.rs mc.rs stackup.rs) are PROPS's ground and its declaration is here, from WIREREFUSE: crates/verbs/src/run.rs is the verb door's own, from WIREREFUSE: run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled, WIRECOST merged in on 2026-11-20 (docs/doc-ledger/wirerefuse-wirecost-and-wirehold-merge-into-doctail.md) - it held no P0 or P1 row, from WIRECOST: opened 2026-11-02 by the WIRE orchestrator when WIREHOLD went over budget after the ceiling fork (fork-log row 235, from WIRECOST: PR 6287) - the member budget itself stays on WIREHOLD's `a-family-has-no-member-ceiling` because the fork log cites its path, from WIRECOST: these rows are its cost-side siblings, from WIRECOST: eval/members.rs is tally ground and eval/mod.rs and eval/wire.rs are shared with WIRE, from WIRECOST: run scripts/work.py territory on your branch and announce the seam in the PR, WIREHOLD merged in on 2026-11-20 (docs/doc-ledger/wirerefuse-wirecost-and-wirehold-merge-into-doctail.md) - it held no P0 or P1 row, from WIREHOLD: opened 2026-10-17 by WIRE's priority-seam cut - the rows moved by git mv with ids and bodies unchanged, from WIREHOLD: WIRE stays open with its P0-P2 slate, from WIREHOLD: crates/editor-core/src/eval/wire.rs and eval/mod.rs are shared ground with WIRE WIRETAIL and INTENT, from WIREHOLD: whose stage 3 units rebuild the placement and pattern doors these rows stand on - when a trigger fires re-read the row against the tree first since most of them are expected to be moot, from WIREHOLD: and run scripts/work.py territory on your branch and announce the seam in the PR where the changes are coupled]
---

What EDIT left behind its doors, what RECIPE and INTENT left on the
same ground, and the findings later programs filed on it:
- the edit door: a reshaping's strand report that misses a run it joins or breaks (P0), a value edit that never re-asks the checks that read values, a profile family the door replays outside any member, and the face frame of the on-face build;
- the refactor and resolver: split widening a product that reads one cut copy, and the flip lane's inability to tell a validation reading from a building decision;
- the save format, which drops its edit log (Ev, PR 6123);
- the certified pick's two rounding rows (P2 H);
- refusal prose, the names layer's hygiene, the nesting bounds, and the load-door id-spelling question (P3–P4).

Charter and order: `work/doctail/plan.md`; narrative in `work/doctail/log.md`.

## From WIREREFUSE (merged 2026-11-20)

WIREREFUSE — WIRE's refusal tail: endings typed by the wrong D9 row, recourse labels spelled by hand or missing, and the eval doors' record and index plumbing

**WIRE's refusal rows**: how an ending is typed (D9 rows 1, 2 and 4), the recourse it carries, and the evaluation doors' record and index plumbing that decide what a refusal can say. The rows were cut out of WIREREST and WIRETAIL along that seam, once the re-weighted tiers put both tails over budget.

Slate, order and posture: `work/doctail/plan.md`; narrative in `docs/doc-ledger/wirerefuse-wirecost-and-wirehold-merge-into-doctail.md`.

## From WIRECOST (merged 2026-11-20)

WIRECOST — what a family costs to evaluate and hold: the cheaper per-member payload, the union fold over a family, and the stop condition the kernel's own evaluations take from their caller

**What a family costs.** After PR 6287 an evaluation holds a member
budget its options carry (one recorded default). These rows make each
member cheaper to hold, bring the union fold over a family down from
superlinear, and pass the caller's cancel token and budget into the
evaluations the kernel runs inside a door. The WIRE orchestrator holds
this track beside WIRE and WIREHOLD.

Slate and order: `work/doctail/plan.md`; narrative in
`docs/doc-ledger/wirerefuse-wirecost-and-wirehold-merge-into-doctail.md`.

## From WIREHOLD (merged 2026-11-20)

WIREHOLD — WIRE's post-D10 residue: member families and their cancelation, the placement and pose readers, forwarders the census does not weld, and the profile's structure record

**WIRE's rows that waited on D10, re-read once it was built.** INTENT
closed on 2026-10-30 (`docs/doc-ledger/intent-leaves-the-tracker.md`),
and every trigger these rows were parked on fired with it. A read-only
triage on 2026-11-01 checked each row against main: two were moot, and
the rest are re-scoped in a dated section of their own. Three carry a
design question (the profile's structure record, which reaches PP1/PP2;
the pivot turn; the cluster-refused standing). The WIRE orchestrator
holds this track beside WIRE. The rows stay here, with their ids,
because tests cite their paths
(`tests/refusal_concision_chains.rs`, `tests/edit_step_segments.rs`).

Slate, order and posture: `work/doctail/plan.md`; narrative in
`docs/doc-ledger/wirerefuse-wirecost-and-wirehold-merge-into-doctail.md`.
