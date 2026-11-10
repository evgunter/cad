---
id: the-census-sort-of-a-boolean-refusal-has-no-legal-home-under-topo-homes
kind: issue
title: The census's sort of a boolean FaceUnread has no legal home under [topo.homes]: pub(crate) in boolean is refused by engine-leaves, pub by unreachable_pub
status: open
opened: 2026-11-03
priority: P2
cost: H
refs: [6451]
---

Found in PR 6451, which raised `census.rs`'s `names-boolean` pin from 506 to 510 in `scripts/gates/engine-leaves.counts`, in plain sight.

`census.rs` `line_unread` sorts a `crate::boolean::FaceUnread` (why one face left a tangent-line station or its boundary unread, `boolean/mod.rs`) into the census's `ValidationError`s. PR 6451 added two arms to it (`FaceUnread::Boundary`, `FaceUnread::Uncertified`), and those raised the pin.

`[topo.homes]` (`crates/topo/README.md`) says code naming an engine's item belongs to that engine, so the sort's home is `boolean/`. It cannot be written there:
- an engine may declare nothing `pub(crate)` (`engine-leaves.sh`'s crate-visible kind), so `pub(crate) fn census` on `FaceUnread` is refused;
- a `pub fn` on `FaceUnread`, a `pub(crate)` enum, is refused by clippy's `unreachable_pub` (`-D warnings`);
- a `pub` door re-exported at the crate root would need `FaceUnread` and `contain::PlacementError` (both `pub(crate)` today) in topo's public API, or `private_interfaces` refuses it. That would publish crate-internal plumbing of the census confirm as product surface.

The sort also uses the census's private `pair_lane_refused` and `read_containment`.

So the census–boolean seam has no door shape for a crate-internal sort of an engine type. The question is which way it should go:
- `[topo.homes]` admits a crate-internal door, a `pub(crate)` item the gate accepts where only the root's non-engine modules consume it;
- the census confirm's tangent-line reading moves wholly into `boolean/` and returns crate-root types;
- the pin stands as the record.

This is a design question on a ratified page (PR 6426), so it goes to Ev's channel before any code moves.


## Weighed (2026-11-06): two designers, agreed on first reports

The draw (127, 42) is on `analysis/design-fork/onecopy-census-sort-home`.

**A** (likely; rejected the framing; no ratified text changes) and **B** (sure; rejected the framing; no ratified text changes) agree:
- **The premise was wrong.** The census's sort is census code, and it is in its home. What is misplaced is what it reads. The at-rest tangent-line reading (`tangent_pair_at_rest`, `RestTangent`, `rest_line_in_both_faces`, `line_through_both_faces`, `boundary_cuts`, `tangent_table_at_rest`, `FaceUnread`) takes a body, faces and a band, and names no boolean state. So it is arm-2 code, a reading of a body, sitting in `boolean/` only because the migration has not reached it.
- **Nothing changes in `[topo.homes]`.** Its sentence "a red there means that what you need belongs at the root or below" is the ruling. This is one site of `work/onedoor/plan.md` unit 5's class: about 1,590 root sites name `boolean/`, 505 of them in `census.rs`.
- **Both ways forward the row offered fail.** A crate-internal door negates the `pub(crate)` ban that fork row 249 agreed. Moving the reading into `boolean/` gives an engine a door for one caller.
- **Each caller keeps its own sort** of the shared refusal: the census sorts into `ValidationError`, the op into `BooleanError` (`locus_through_both_faces`).

No `[ev]` PR is needed. Nothing ratified changes, and both recommend against adding a clarifying line.

## Decided (2026-11-06)

1. **This row becomes a move row.** The at-rest tangent-line reading leaves `boolean/` for a crate-root module, in dependency order:
   - first, `contain`'s placement door (`face_placement`, `CurvedPlacement`, `PlacementError`, `contfp`, `ContainError`, already on unit 5's list);
   - then the face-boundary walk `carrier_cross::face_boundary`, which must refuse in its own type, not `BooleanError`;
   - last, the reading itself.

   `contact_verify::tangent_locus_relation`, which takes no body, goes to `geom-brep`. When the reading moves, `FaceUnread` is reshaped by verdict kind (B): `Escalated(Indeterminate)`, `Containment(ContainError)`, `Unread { what }`, `Fault { what }`. Each caller's sort is then arm to arm, with no judgement.
2. **What makes the next instance land right:**
   - `engine-leaves.sh`'s failure text states the README's remedy (move the named item to the root or below), not "belongs to that engine" (A);
   - the gate fails a raised pin against the merge base (A);
   - the `.counts` header states the migration practice: a new site is paid in the reader's pin, citing its move row, and never by a new `pub(crate)` in the engine (B).

   All three are agent text (PR 6458).

## Also cited by (2026-11-09)

`contain_loop.rs`'s `names-boolean` pin (`onecopy/small-sweeps`): the
containment door's `From<PointInLoopError> for ContainError`, moved out
of `boolean/contain.rs` so the boolean no longer names the splitting
engine there. Its two sites are `ContainError`'s, and fall when step 1
above moves `ContainError` to the crate root.
