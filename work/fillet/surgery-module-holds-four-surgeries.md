---
id: surgery-module-holds-four-surgeries
kind: unit
title: sweep: blend/surgery.rs holds four surgeries (4.3k lines); splitting the open bands out needs one re-scoping of the compound-bound allowlist
status: closed
opened: 2026-09-05
branch: fillet/split-open-bands
pr: 1964
closed: 2026-09-05
---


## The finding (both FILLET-H7 reviewers, Q8, 2026-09-05)

`crates/sweep/src/blend/surgery.rs` is ~4 300 lines and now holds FOUR
surgeries as titled sections — the plane–plane open band with its
trihedral corners (the blank phase), the closed-rim LADDER, the closed-rim
ANNULUS (with H5's hostless struts), and H7's RULED open band with the
transverse cut-off — plus the refusal constructors, the plan structs, the
ring check and the shared description pass. Each section is reasonable;
the accumulation is the shape `docs/prompts/reviewer-style-lane.md` Q8
names, and "the open band" in the file's header now means two unrelated
walks that share only `seam_split_param` and `attach_contact`.

## Why it is here and not already split

FILLET-H7's spec asked for a new `blend/ruled.rs`; the lane put the carve
in `surgery.rs` instead (its deviation 1) because the ruled plan reads the
split parameter's window and so needs `T: Decide + Bounds`, and the
compound-bound allowlist (`scripts/gates/bounds-allowlist.sh:443`, the
"M5 PR 12 (orchestrator ruling 2026-08-03), the edge-blend battery" entry
in `crates/geom-core/src/real.rs`'s `bounds_allowlist` ledger) names
`blend/(battery|build|surgery).rs` and nothing else. A new file would red
the gate; both reviewers verified the fence and the lane's reading of it.

## The ask (Ev)

Whether the existing ratification may be RE-SCOPED — the same code, the
same necessity argument (the split parameter's bracket read), moved into
`blend/open/{planar,ruled}.rs` (or `blend/{open,ruled}.rs` and a thinner
`surgery.rs` holding the closed-rim walks) — with the ledger entry and the
gate's file list amended in one PR. The `bounds_allowlist` doc says an
extension owes a NECESSITY demonstration (the weakest bound that works and
the next tighter one failing); this is not an extension of scope but a
move of already-ratified code, so the question is only whether the entry's
file list is the ratified thing or the seam is. R2's taste, for the record:
`blend/open/{planar,ruled}.rs` behind one ratification is the honest shape.

Until ruled, no lane moves the code. A 👍 on this PR's `[ev]` thread
approves the re-scoping as a FILLET follow-up unit (S / STRUCTURAL: a
file move, bit-identical by the dump); a comment says otherwise.

## Ruled (Ev, comment on PR 1916, 2026-09-05)

"sure, you don't need to ask me about moving things around, unless it has
design implications". The re-scoping is approved, and the rule generalises:
a file move with no design implication is the orchestrator's call. This
item becomes the FILLET-SPLIT unit (S / STRUCTURAL): `blend/open/{planar,ruled}.rs`
(or the shape the lane finds honest) behind the existing ratification's
entry re-scoped in `scripts/gates/bounds-allowlist.sh` and the `real.rs`
ledger, bit-identical by the dump. Dispatches into block FILLET-B3 slot 1
once FILLET-T's fix pass has left `surgery.rs`.
