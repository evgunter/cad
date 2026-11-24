---
id: the-session-box-reach-misses-a-curved-faces-bulge
kind: issue
title: The session box reads a body's vertices and edges, so a curved face's far side no edge rides is missed
status: review
opened: 2026-11-02
priority: P1
cost: H
pr: 7225
branch: oneround/box-region
---


`crates/geom-core/README.md` ("Float rounding") defines a body's reach for the session-box predicate at the finished-body gate as read over its vertices, its edges' spans and its faces' boundaries. PR 6380's `topo::session_box` (rework: through `geom_brep::extent::edge_reach_bounds`) reads exactly that. A curved face's interior can reach farther than any edge on it, and the definition misses that.

**Example (from PR 6380's FULL review; likely, not run).**
- The body is the `bulge_loop` torus (latitude seams at radius R), with r = 400 and R = 450.
- It is turned so its meridian faces the origin, and centred at 250 m.
- The gate reads about 806 m; the true reach is about 1100 m.

So a body that leaves the 1 km box is admitted, and the floor premise of the admission rule is false for it.

**Shape of the fix.** A face's reach comes from its surface over the face's own region, not from its boundary:
- a sphere or torus: the farthest point of the surface patch;
- a NURBS face: its control net clipped to the face.

That reach should come from the one extent home (`work/onehome/reach-names-eight-unrelated-types.md`, the `onehome/one-extent` lane), not from a new walk in topo. The README's sentence then names faces' regions, not their boundaries. That is a change to ratified text, so it goes to Ev.

## Slate triage (2026-11-08, ONEHOME split)

- Moved to ONEROUND (`work/oneround/`) by `git mv`, id and body unchanged. ONEHOME measured 78 budget points against 35 after PRs 6371 and 6380 landed, and split along its seams (`work/README.md` Track size): the rows on f64 rounding, levers and the consumed extent that PR 6380 and PR 6391 filed went to ONEROUND.

## A second example, measured (2026-11-09, `onecopy/main-reds-stations`)

A sphere of `r = 50` centred at `(0, 0, 975)` has a true largest `|coordinate|` of 1025, at its top.
- **Poled on `y`** (`sweep::test_support::ball_poled_y`): the finished-body gate admits it (`validate_geometric` is `Ok`). Its top is interior to a face, on no vertex or edge.
- **Poled on `z`** (`ball_poled_z`): the gate refuses it, `OutsideSessionBox` at margin −25, because its pole is a vertex at the top.

The box reads the cube since PR 6608 (`[tolerance.units]` bounds each coordinate). It still reads only vertices and edges, so whether a curved face's extreme is caught depends on where the body's seams fall.

## A split or a concave fillet of a placed copy mints edges on the bulge (delta-4 review of PR 7088)

Since PR 7088, a placed copy's box is asked once, at the door that places it (`Framed::asked_where_it_stands`), of the copy's vertices and edges. An operation that then mints edges on a curved face's interior leaves those edges unasked. A split of the copy through its face, or a concave fillet whose rail crosses the face's bulge, are both one-input operations whose result lies within the operand's hull, and no door re-asks them. Where the bulge reaches past the box, the new edges stand past it with no refusal. The reach that closes this row, a face's interior extreme read from below, closes this too, if it is asked at the placing door.

## Ev's answer (2026-11-20, PR 7140)

"i feel uncertain about this / like i am not familiar enough with the relevant considerations to have a good sense of what's best, but i'm good with proceeding with this unless there's something you specifically want to discuss". So the README's session-box bullet is as merged: the box is a predicate of a region read from outside as its cover and from inside off points on it, faces included; a region is in the box only where its cover is; a point past it refuses as past the box and a cover alone past it as a limit; an edge's certification asks it before its residual, the finished-body gate asks it of the faces, and the placing door of the body where it stands.

**Ev approved without a view of their own, so the build reports back what the designers could not check**, on its own PR and before it merges: whether any construction certifies helper or cutter edges past 1 km on purpose (the certification ask would newly refuse them); whether every finished face has a cover; what a face-cover read at every finish costs on the box-scalar lanes; and every verdict that moves. The certification ask is the part that reverses Ev's PR 6329 choice, and it can be taken out again leaving the gate.

The design question is closed. What is left on this row is the build, cut into units at spec time: the two-sided read per kind of face (ruled and sphere faces from the existing cover; a torus's far points; a spline's net clipped to its face), the certification ask, the gate's face ask, and the placing door's read. `a-body-past-the-session-box-refuses-at-certifications-residual-first-by-its-spelling` closes with the certification ask.


## Cut into units (2026-11-20, ONEROUND orchestrator)

Read against main first (a map of `topo::session_box`, the gate, certification, the placing doors and the covers; nothing on this ground is touched by `oneround/lever-doors` but `certify.rs`, by 30 lines):

- Every ask but certification's goes through one fold, `session_box::reach_of`: the gate (`validate::structural_via`), tier 3′ (`pseudomanifold_certificate_via`), the move (`finish::moved_and_asked`) and the placing door (`finish::session_box_at`, `Framed::asked_where_it_stands`). So the gate's face ask and the placing door's read are one change, not two.
- The outside read of a face exists for every kind: `topo::cover::face_cover`. It is loose for a spline (the whole control net) and a spindle (the tube), and a ring torus's window is orientation-blind (BOXES holds rows on each).
- The inside read of a curved face does not exist: no function produces points known to lie on a sphere's or torus's face. Candidates (a sphere's poles and coordinate extremes, a torus's far points) can be placed by `contain::curved_face_placement`.
- Certification never asks the box.

So, two units:

1. **`oneround/box-region`** (this row; H): the predicate over a region, its cover from outside and points on it from inside, with both refusals (past the box; a limit), and the one fold asking it of every face. The two examples above become rows. It reports back to Ev, on its PR and before it merges: whether every finished face has a cover, what the face read costs at every finish on the box-scalar lanes, and every verdict that moves.
2. **`oneround/box-certify-ask`** (`a-body-past-the-session-box-refuses-at-certifications-residual-first-by-its-spelling`; M): certification asks the predicate of an edge with its ends before its residual. Stacked on the first once its predicate is pushed. It reports whether any construction certifies helper or cutter edges past 1 km on purpose.

Tighter covers (a spline's net clipped to its face, the spindle, the clockwise torus) are precision, not soundness: a loose cover refuses as a limit where a tight one would admit. The first unit measures how often; a row follows if it is ever.

### `PLATE_LEDGER` re-pinned by `oneround/box-region` (PR 7225, 2026-11-22)

`editor-core`'s `m10_sym_profile_interval::the_forms_the_plates_walks_build_are_pinned` moves, at all three ε. From main's pin to this branch merged with main on 2026-11-22, in three walks only:
- `Plain/Assertion` 12224 → 12236;
- `Early/Assertion` 12674 → 12686;
- `Door/Decision` 12366 → 12378.

Each walk gains 12 forms, with its digest; no call, frozen count or decision moves.

The cause is the box's region read. Each of the plate's two placements asks the session box where its hole stands (`topo::session_box_at`). The box now reads each placed face's region by its carrier and boundary (`cover::face_cover_unproven`), as well as each vertex and edge. That builds the forms of each placed face's cover and asks no decision. Stage 1 admits both holes, so no stage-2 cover or stage-3 point is read.

**Verdict: owed.** The region read is the unit's design (the README's predicate over the region a body occupies). Re-pinned at this branch's reading.

### `PLATE_LEDGER` re-pinned again, at main's move (PR 7225's fix pass, 2026-11-23)

Merging main `2130aa45` brought PR 7434's move of the same row: +4 calls and +146 forms on `Plain/Assertion`, `Early/Assertion` and `Door/Decision`, and a new `Plain/Decision` digest. Bisected and filed on DOCTAIL's slate: `work/doctail/main-red-the-plates-ledger-reads-four-more-calls-since-the-least-turn-reads-no-exactness.md`. Main is red on the row without this branch.

The row is re-pinned at this branch's reading of the merged tree, the same at ε = 1e-9, 1e-6 and 1e-12:
- `Plain/Decision` calls 979, forms 33088, digest `31fb2516…`;
- `Plain/Assertion` 1051 / 12382;
- `Early/Decision` unmoved;
- `Early/Assertion` 1051 / 12832;
- `Door/Decision` 554 / 12524.

That is main's move plus this unit's +12 forms on the three lines named above. This fix pass's own changes move nothing: the row reads the earlier pin on the pre-merge commit `56a16cc2`.
