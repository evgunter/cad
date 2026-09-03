---
id: coplanar-cap-pair-f7-repair-half-b
kind: issue
title: Full-revolve axis-touching planar caps are born as the F7 defect, and merge_coplanar_faces refuses to repair them
status: open
opened: 2026-08-26
github: 1031
refs: [1131, 1059]
---

## From GitHub issue 1031

Opened 2026-08-26; 3 comments.

(VERBS orchestrator) The named repair question behind lily wall 7's TRUE mechanism, measured twice and now owned by an issue rather than two programs' prose.

**The gap**: a full revolve whose planar cap reaches the axis mints the cap as TWO half-faces on ONE plane key — precisely the F7 defect (`reduce.rs`'s `gate_maximal_faces`, whose own text rules same-key CURVED adjacency canonical and only the PLANAR same-key pair defective) — and `merge_coplanar_faces` refuses to repair it (`MergedFaceRoleAmbiguous`). So no such body is ever a legal boolean operand: the operand gate is never the binding constraint, and no germ arm is ever reached.

**Measurements**: (1) the M9-5 implementer's live probe in the lily rebuild (PR forthcoming; the reduce.rs citation in the probe text); (2) the corrected wall-7 chain — VERBS-GATE's fix pass measured the `NonMaximalFaces` refusal but mis-attributed its cause to the sphere zone's half-bands, which the gate explicitly `continue`s past; the M9-5 measurement located the actual same-plane-key pair in the caps.

**Planning consequence (recorded in VERBS-PLAN)**: Wave 2 items 6+9 (the pair-scoped gate — landed — plus a sphere×sphere germ arm) are NOT sufficient to flip wall 7. The flip additionally requires this repair, by one of two shapes: (a) `merge_coplanar_faces` learns the full-revolve cap pair (resolving the role ambiguity for the two-half-disc-plus-seam configuration), or (b) `revolve` mints axis-touching caps MAXIMAL (one face; whether a plane face may carry an interior seam edge is the chart question that decides between these). Unowned; scoping belongs to whichever program takes it, with the chart question likely needing a design note before code.

**Attribution honesty**: the wall-7 record has now been corrected three times, each by a better instrument (body-scoped kind → box-artifact pair → face-maximality precondition → the caps specifically). The current lily wall text's "zone half-bands" sentence is FALSE and the M9-5 PR carries its correction; VERBS-PLAN's row-6 note is corrected at this issue's filing.

## Comments

**2026-08-27** — comment:

(M9 orchestrator) **Ev's steer on the (a)/(b) choice, recorded here so whoever scopes this inherits it rather than re-deriving it.**

Verbatim intent, from the M9 closure review:

> i think i still lean (b) because other producers can do the same when they're created, but if the change would look more like getting the faces and then merging them then we should probably just do (a) to start off.

Two things in that, both load-bearing:

1. **The lean is (b), and the reason is the one I had wrong.** I framed the trade as "(b) is narrower — it only fixes revolve, while (a) repairs the defect wherever it arises." Ev's point inverts the weight: *other producers can do the same when they're created*, i.e. (b) is the discipline that stops each new producer from minting the defect in the first place, rather than a per-producer patch. Fixing the source generalises through the convention; fixing the repair op generalises through the repair.

2. **A concrete decision rule, and it is a scoping test rather than a preference.** If making `revolve` mint the cap maximal turns out to *be* "get the faces and then merge them" internally — i.e. the producer ends up performing the same role-ambiguous merge that `merge_coplanar_faces` currently refuses — then **do (a) first**. There is no point paying for the hard case twice, once inline and once in the op that exists for it.

So the substrate pass for this issue has a specific question to answer BEFORE the spec chooses a shape: **does emitting one maximal axis-touching cap fall out of the sweep's own construction, or does it require the merge?** `crates/sweep/src/revolve/full.rs`'s wire case and its half-turn construction (`half = θ/2`, `rot_pi`) are where that gets measured. My own read of the module docs was not confident enough to call it, and this milestone's record on stated-blocker-vs-binding-constraint (#1031 itself, then #1059) argues for measuring rather than asserting.

The chart question — may a plane face carry an interior seam edge — is answered permissively by Ev for the record ("i think i'm probably ok with a plane face carrying an interior seam if necessary"), so it is no longer a blocker on either shape; it is a preference to avoid needing, not a wall.

**2026-08-28** — comment:

**#1031 is two defects under one number** — measured, and the split is now landed for one half.

The measurement (VERBS-LILYWELD lane, 2026-08-28) dumped the actual structure at each F7 refusal site:

**Half A — the pole-split cap (the lantern's, and every full revolve's).** An axis-touching planar cap arrives as two half-faces on one plane key. Dumped:

```
lily_lantern  pole 6v1 valence 2 — Edge 5v1[F6v1=plane,F10v1=plane], Edge 14v1[same pair]
ball          pole 1v1 valence 2 — Edge 1v1[F2v1=sphere,F1v1=sphere], Edge 2v1[same pair]
```

The planar cap and the *curved* wall beside it are **the same structure** — two faces on one surface key, two meridian edges, a valence-2 pole — differing only in the carrier's kind, and `gate_maximal_faces` called one canonical and the other a defect.

There was never a one-face form to demand, and the reason is tier 2's own rule: `revolve`'s wire case sweeps in two π-bands *precisely* so each pole ends with valence 2 — "a one-band wire would leave the tips valence-1, which tier 2 rightly bans as strut scaffolding". Merge one meridian away → valence 1, banned. Merge both → an isolated vertex interior to the face, which is exactly the `MergedFaceRoleAmbiguous` that `merge_coplanar_faces` refuses.

**This half is FIXED** by narrowing the rule rather than by a producer or merge change: `boolean::reduce::pole_split_cap` exempts a planar same-key pair whose edge has a valence-2 endpoint both of whose edges separate the same face pair. Purely structural, reads no geometry.

**Half B — the ordinary coplanar pair (the teapot cup's). STILL OPEN, and this issue keeps it.** Dumped:

```
F7DUMP A edge EdgeKey(3v1) faces FaceKey(4v1)/FaceKey(10v1)
       vertex VertexKey(3v1) valence 4
       vertex VertexKey(4v1) valence 4
       carrier plane o=(0.046875, 0.09375, 0) n=(0,1,0)   [a MERIDIAN plane]
```

No pole anywhere near it — this is two genuinely coplanar faces meeting at an ordinary edge, exactly the case the F7 rule exists for. The exemption correctly does not touch it (measured: the teapot's walls hold unchanged).

**And the repair door is shut for this half too**, which was the open question:

```
merge_coplanar_faces(cup) -> Err(MergedFaceRoleAmbiguous { face: FaceKey(4v1) })
```

So half B is **not** an authoring/verb-composition fix — it stays a kernel question, and the original framing (teach the merge op this pair, or have the producing op mint it merged) applies to it alone.

**Consumers, re-attributed:** lily wall 2 and lily wall 7 demanded half A (wall 7 now moves to the curved pierce arm); the teapot demands half B. The earlier "triple-demanded" reading was 2 + 1, not 3 of a kind.

**2026-08-29** — comment:

**Correction to my earlier comment on this issue.** That comment said the pole half was *"FIXED by narrowing the rule rather than by a producer or merge change: `boolean::reduce::pole_split_cap`"*. **That is wrong and the approach it names was withdrawn.**

What happened after it was written:

1. The gate exemption `reduce::pole_split_cap` was **falsified twice** and removed. It admitted shapes it was claimed to exclude — R1's subdivided chord and inset-patch ring, R2's mid-vertex chord and two-vertex chain (all bent, all genuinely mergeable, all sailed past the gate), and then a second, narrower form was falsified by `merge_skip`'s brick flush caps, whose L-shaped seam has exactly the "two shared edges at a valence-2 vertex" shape. `reduce.rs` is byte-identical to main.

2. **The impossibility premise in that comment was also false.** It argued no one-face cap existed to build. Both review arms reached it from the revolve's own output with two public ops — `kef` then `kev` — tier 1–3 Ok; tier 2 binds bodies at rest, not intermediates.

3. **The actual fix is in `merge_coplanar_faces`**, not the gate: when a coplanar pair's shared seam is two edges meeting at a valence-2 vertex whose departures are collinear and *opposed*, the vertex is interior to one straight carrier, so `kev` removes the seam and the vertex without changing any locus. The licence is **collinearity, not poleness** — `merge_faces::redundant_subdivision_vertex` — which makes the repair general: *remove a redundant subdivision vertex on a shared collinear seam*.

Measured: a plain revolved cone, faces 4→3, vertices 4→3, planar same-key pairs 2→0, tier 2 and tier 3 green; the lily's lantern (two caps) faces 10→8, vertices 10→8, edges 18→14. Verified under `--features interval` at ε = 1e-9, 1e-6 and 1e-12.

**The half-A / half-B split in that comment stands unchanged**, and it is the part worth keeping: the pole-split cap and the teapot cup's ordinary coplanar pair (valence-4 endpoints, meridian plane) are two different defects. Half A is repaired; **half B is still open and this issue still owns it** — the merge door refuses it too, so it remains a kernel question rather than an authoring one.

PR: #1131.

## Home

S-BOOL's `keep_out` names issue 1031 half B — all that remains open — as VERBS' ground.
