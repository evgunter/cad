---
id: a-chart-spans-solids-after-move-shells-to-new-solid
kind: issue
title: move_shells_to_new_solid re-homes a shell without re-minting its surfaces, so a chart can span two solids
status: open
opened: 2026-09-08
needs_ev: true
priority: P0
cost: D
---



Measured by SHELL-8's R2 reviewer (PR #2207, 2026-09-08; probe
`r2_chart_spans_solids_through_subtract_then_move_shells` on
`shell/8-r2-probes` @ eaf2976b4) and placed here by the SHELL
orchestrator: `topo::subtract(brick 6×1×1, brick 1×3×3 across its
middle)` is a disconnecting subtract that files both fragments under
ONE solid with TWO shells, and the fragments of each cut operand face
keep their operand's surface key — so a chart (one `SurfaceKey`)
holds faces on both shells. That is legitimate inside one solid.
`Body::move_shells_to_new_solid` (`crates/topo/src/movefac.rs:228`)
then re-homes one shell into a solid of its own and leaves every
surface key as it was: after the move the chart spans two solids, a
state no other producer builds (grafts and instancing mint fresh keys
per transplanted entity, `instance.rs:123`, `graft_solids_with`).
SHELL-8 assumed "a chart lives in one solid" (its per-solid doors
group faces by chart and refuse `ShellError::ChartSpansSolids`
typed when the premise fails, `crates/topo/src/shell.rs`), and its
own PR text claimed every producer mints per solid — false, this
path. Either the mover re-mints the moved shell's surfaces (one
surface per solid, the invariant SHELL-8 wants stated at the
`Body` level), or the invariant is not one and the shell doors'
grouping is per (solid, surface) — the choice is TOPO's, with S-BOOL
on the disconnecting-subtract half (see
`work/bool/subtract-of-a-hollow-operand-files-the-island-under-one-solid`,
the same one-solid filing on a different shape). Signed (SHELL
orchestrator).

## Ruling proposed (TOPO, 2026-09-13) — for Ev

**The state today is a typed refusal, not a silent wrong.** SHELL-8's
doors group faces by chart and refuse `ShellError::ChartSpansSolids`
when a chart's faces sit on two solids (`crates/topo/src/shell.rs`,
the partition check in the group door). `Body::move_shells_to_new_solid`
(`crates/topo/src/movefac.rs`) moves shells whole with every key kept
and re-partitions ownership only — its doc says so — so after a
disconnecting subtract (both fragments under one solid, the cut
operand's face fragments sharing their operand's `SurfaceKey`) a move
leaves one chart on two solids. Every other producer that puts faces
under a new solid mints fresh keys per transplanted entity
(`combine`'s transplant, `graft_solids_with`, `instance.rs`).

**The question is whether "a chart lives in one solid" is a `Body`
invariant.** Three answers:

- **(A) It is, and the mover keeps it.** `move_shells_to_new_solid`
  re-mints, for each moved shell, every surface key its faces wear
  that is also worn by a face staying behind — the moved faces get
  the copy, the stayers keep the original, and the same for curve keys
  if an edge could straddle (it cannot: shells move whole and every
  edge's two faces are in one shell, so only surfaces need it). The
  invariant is stated at `Body` and checked by tier 1 as one more
  reference-coherence pass (group faces by surface key, refuse a key
  whose faces name two solids — O(F), the same walk pass 9 makes). The
  N6 identity channel already has the vocabulary for a re-minted key
  being the same recipe source seen again (`instance.rs`). SHELL-8's
  `ChartSpansSolids` arm becomes unreachable and retires with a row
  that pins the mover's re-mint. The disconnecting subtract's
  one-solid filing stays S-BOOL's separate defect
  (`work/bool/subtract-of-a-hollow-operand-files-the-island-under-one-solid`
  is the same filing on a different shape); when it files two solids
  the same invariant tells it to mint per solid.
- **(B) It is not, and the shell doors group per (solid, surface).**
  The mover stays as it is; SHELL-8's partition key becomes the pair
  and `ChartSpansSolids` retires as a refusal of nothing. A chart is
  then a body-wide description that any number of solids may wear,
  which is geometrically honest (two solids can be coplanar) but makes
  "a chart moves as one" a per-solid statement every chart-walking
  door has to remember to make — the shell verb today, the offset
  doors, whatever thickens or transforms a chart next.
- **(C) Neither: keep the typed refusal and document the premise.**
  Costs nothing now; leaves a state one public door can reach and
  another refuses, with the invariant stated by the refusal's message
  rather than by the store.

**Recommendation: (A).** It is what every other producer already
does, it turns SHELL-8's assumption into a fact the store enforces
rather than one a door discovers, and it is the shape the boolean's
fix will need anyway. Counter-arguments, honestly: re-minting copies
descriptions (a plane is three vectors; the cost is real only for
NURBS charts, and a moved shell that shares a NURBS chart with a
stayer is the disconnecting-subtract shape, rare); a new tier-1 pass
is a per-operator cost in the debug profile PERF has just priced
(`work/perf/d1-per-op-tier1-sweep-price`) — it rides the once-per-door
sweep, and its walk is linear in faces. (B) is the smaller diff and
the larger obligation.

**Scope, if ratified:** the re-mint in `move_shells_to_new_solid` with
its provenance stated (a `Provenance` variant or the existing
`MoveShells` record extended — phase 1 decides), the tier-1 pass and
its `ValidationError` variant, a red-first row on SHELL-8's probe shape
(subtract, then move, then the door thickens), SHELL-8's arm retired by
announced seam to SHELL, and the mover's doc losing "every key kept".
Kernel answer: draws a block slot (TOPO-B3 or later).

## The attach postcondition, in the same PR (2026-09-13) — for Ev

`attach-postconditions-validate-the-whole-body-and-panic` (SHELL-10's
placement) has two halves. The COST half is answered: Ev's ruling on
`work/perf/d1-per-op-tier1-sweep-price` (PERF-4, PR 2305) made the
setters' tier-1 sweep once per public door through the surgery scope,
and `attach.rs`'s module doc now says so. The PANIC half — a setter's
tier-1 postcondition panicking on a body whose OTHER solid was already
torn — is D1's contract firing where the tear surfaced rather than
where it was made: every arena writer is `pub(crate)` and every public
operator preserves tier 1, so a public caller cannot present a torn
body to a setter; SHELL-10's `#[should_panic]` row tears the body
in-crate first. That is not the D9 case ("a refusal is owed on invalid
input") — no public input is invalid there. Proposed disposition:
close the row with this reading, unless Ev wants D1's postcondition to
refuse typed rather than assert (a change to a ratified clause, which
this PR does not make). The row is not edited until the ruling lands.

## Ruled (2026-09-14, PR 2527)

Ev: "yep, (A)". A chart lives in one solid is a `Body` invariant and
the mover keeps it: `move_shells_to_new_solid` re-mints, per moved
shell, every surface key its faces wear that a stayer also wears
(curves need none — shells move whole), the invariant is stated at
`Body` and checked by tier 1 as one reference-coherence pass,
SHELL-8's `ChartSpansSolids` arm retires by announced seam with a row
pinning the re-mint, the mover's doc loses "every key kept". The
disconnecting subtract's one-solid filing stays S-BOOL's. Kernel
answer: draws a block slot (TOPO-B6 or the next cut); the row is now
a unit with the scope paragraph above as its brief's spine.
