---
id: pickindex-merges-parts-on-a-rounded-t-it-never-converts
kind: issue
title: pickindex merges parts on a rounded t it never converts, and slacks it by a tuned 1e-6
status: open
opened: 2026-09-16
priority: P1
cost: H
---


Filed by EDIT-PICK3 (`pick-door-answers-a-t-interval`, PR #2786) and
its two review lanes. The kernel door now answers a `t` INTERVAL
(`TSpan`: `t`, `t_lo`, `t_hi`), and three sites in
`crates/viewer/src/pickindex.rs` read the parameter in ways that the
interval either breaks or makes cheap to repair.

## 1. A moved instance's hit carries `t_lo`/`t_hi` unconverted

`PickIndex::pick_for` (`pickindex.rs:~1174–1179`) picks a free-moved
instance by carrying the ray into the instance's display-local space
and then lifting the hit back out:

```rust
let world = PickHit { point: map.transform_point(hit.point), ..hit };
```

`..hit` carries `t`, `t_lo` and `t_hi` VERBATIM. They are parameters of
the LOCAL ray, `inverse.transform_vec(ray.dir)`, not of the caller's.
The doc paragraph argues this is safe because the display layer admits
only RIGID probe frames, so `|dir|` is preserved and `t` means the same
world distance. That argument is about `t` and about rigidity; with an
interval there are three numbers, and the day a `Frame` may scale (or
the day a non-rigid probe is admitted anywhere else) all three are
wrong together while `point` stays right. The kernel's own statement is
now explicit at `PickHit::t`: *"a hit carried across a transform
converts all three or none."*

Repair shape: convert the span through the same map — for a rigid map
the three are unchanged and the conversion is a no-op the compiler can
see; for any other map it is the correction — or refuse a non-rigid
probe frame at the door that admits it, by name.

## 2. The merge across groups compares the rounded `t`

Same function, immediately below:

```rust
let better = match &best { None => true, Some(b) => world.t < b.t };
```

That is `main`'s order: the rounded `t`, ties to the first group. The
interval order now exists — `TSpan::precedes` and `TSpan::best_of`,
`crates/editor-core/src/resolve/pick.rs` — and says that two candidates
whose intervals OVERLAP are not ordered by their rounded `t` at all.
Inside one `pick_face` call the kernel breaks that tie by the narrower
interval and then by position; across groups the viewer breaks it by
`<` on a rounded float and a comment about group order. The two paths
disagree in exactly the class the ruling was about — a face the ray
meets edge-on in front of a transversal one — and the disagreement is
now between the kernel and its own caller rather than inside either.

Repair shape: build the candidate spans and call `TSpan::best_of`, with
the group order as the slice order so the documented last key is kept.

## 3. `OCCLUSION_SLACK_REL = 1e-6` is a tuned tolerance on `t`

`pickindex.rs:~1609`:

```rust
.is_some_and(|front| front.t < t - OCCLUSION_SLACK_REL * t.abs())
```

A relative `1e-6` on the hit parameter, chosen rather than derived —
the exact shape `docs/EDIT-PICK3-SPEC.md` and the ruling row
`what-t-the-pick-door-answers-and-with-what-width` forbid in the
kernel. The hit now carries the width the arithmetic certifies for it,
so "is this front hit really in front" has a derived answer:
`front.span.precedes(&span)`. Whether the viewer wants the certified
answer or a deliberately looser one is VIEW's call, but the number
should stop being a guess either way.

## Why this is VIEW's and not EDIT's

`python3 scripts/work.py territory --files -` reports
`crates/viewer/src/pickindex.rs: owned by chrome, view`. EDIT-PICK3
left the file untouched and says so in its PR body; the interval is the
ray's own parameter, and converting a moved instance's hit is the
viewer's job, which is what makes this a row rather than a patch.


## §2 closed by announcement (2026-09-17, EDIT `edit/pick-tie-refuses`)

Ev's ruling on `[ev]` PR #2795 gave the kernel door a set-valued
answer: the survivors of `TSpan::precedes` are the answer when they
name one face and `HitTestError::Ambiguous { hits }` when they name
several, with no width and no position key. The ruling names the
viewer's cross-group merge as taking the same rule, so the unit built
it: `PickIndex::pick_for` collects one candidate per group, runs
`TSpan::survivors` over their spans, answers the single survivor and
refuses with the rest. `<` on a rounded `t`, and the comment about
group order, are gone.

**§1 and §3 stand.** §1 — a moved instance's `t_lo`/`t_hi` carried
across `map` unconverted by `..hit`, while `point` is converted — is
untouched: the unit changed the merge, not the crossing, and the
`..hit` struct update is still there. The merge now runs over the
UNION of every group's answer rather than propagating the first
group's refusal, so the number of spans crossing a probe frame
unconverted went up and the argument holding them comparable is the
same one — the display layer admits rigid probe frames only. §3 —
`OCCLUSION_SLACK_REL`'s tuned `1e-6` — is untouched too, and its site
moved: the occlusion probe now reads `PickIndex::front_of`, which
answers the nearest parameter across a certified tie rather than
refusing it, and still compares it with the same relative slack.
Whether the viewer wants `front.span.precedes(&span)` instead remains
VIEW's call and this row's §3.

**`front_of` has TWO readers now** (2026-09-17, the same unit's fix
pass): the occlusion probe above, and `PickIndex::seed` — the
un-projection plus the ray path's answer that `edge_at_for`,
`hovered_for` and `faces_under_cursor` all open with. The seed reads a
DEPTH rather than a pick, for the reason §3's site does: a depth
across a certified tie is not a pick of a face. It is the nearest of
the faces the door names, which is a point of the surface under the
cursor whichever tied face owns it, and every tied answer overlaps
every other, so the smallest of their parameters is a function of the
set and not of the list's order. The face is what the arithmetic
refuses to name, so only the caller whose own answer IS a face raises
the refusal — `hovered_for`, after the edge-priority rule has had the
cursor. Seeding on the FACE answer instead refused the edge pick at
every cursor on a shared edge, which is the pixel a user aims an edge
with: 18 of the 66 segment-midpoint cursors on the shipped plate
(`crates/viewer/tests/edge_pick.rs`,
`a_cursor_the_face_pick_ties_on_still_picks_the_edge`). This does not
touch §3's number, and reading the occlusion question with
`precedes` instead would now be a decision about both readers.
