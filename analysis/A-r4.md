# Arrival-constructed arcs — designer report (round 4: the fillet-family framing)

## For Ev

**Recommendation (likely).** Yes — and it is better than both earlier shapes. Build it as a fillet-family verb with no arguments, working name `arrive()`, taken on a point tip (plain point, directed point, or the entry point), which opens the family's ordinary ARRIVAL BUILDER: `.at(p)` and a director (`.angle(θ)` / `.toward(dx, dy)`) in either order. Its arc runs from the tip, untrimmed and with no tangency asked at the tip, to the anchor `p`, arriving tangent to the bound direction; the tip then lands as a directed point at `p` (incoming = that direction, departure free), exactly as the family's arc arrivals land today. The seam is the family's: `.to(Start)` is the tangent closer (`Start`'s direction by reference, joint 0 constructed, no token, as the seam fillet does it), and `.at(Start)` + a director is the sharp seam with its arrival direction authored. `ArcData` is untouched — no new mode, no direction inside a spec, no fused cell to admit or refuse — and the `Via`/`Center` token half of the PR is unaffected.

**Definitions.** *Arrival builder*: the half-bound state a fillet verb opens, bound by `.at` and a director in either order (§2 "every side is anchored by a real on-path point plus a direction"). *Arc arrival*: a fillet arrival whose side is an arc; its tip lands as a directed point at the anchor with the arc's tangent as incoming and the departure free (§2c dissolution). *Tangent closer*: the arc from the tip to `Start` tangent to `Start`'s direction.

### 1. What the framing is, and why it is right

What the two rows record is "the arc whose end I know". The family already authors every side as (anchor, direction) through binders, and already has the exact landing semantics (an arc arrival lands at its anchor as a directed point, departure free). What it lacks is an arc arrival whose START is pinned rather than trimmed. `Radius { r, side }` is that arrival with the start free, so `r` supplies the DOF; `arrive()` is the same builder with the start pinned at the tip, so no `r` exists and the derived departure replaces the trim. That is the whole design: one existing builder, one new way to close its DOF count.

The reading I take of "not necessarily arriving tangent" is the orchestrator's: tangent where it ARRIVES (at the anchor, to the arrival side's direction), free where it LEAVES (the tip). The opposite reading — tangent at the tip, free at the anchor, i.e. a fillet of radius `r` whose arrival side is only a point — is coherent (circle tangent to the incoming ray through `p`: `r` + `p` determine it) but does not serve the case: the datum nobody has is the departure, and what the author knows is the arrival direction; and its untrimmed form is `tangent_arc_to(p)` already. Say which you meant on the PR; the design below is the first.

Why it beats the `Arrive` mode (round 3) and the pending-director mode: the mode put a direction inside an arc spec, or a director step after `arc_to`, and either way dragged in round 7/9's obligation on fused sites, where an "arrival direction" would be met mid-arc — admissible by DOF, wrong by meaning, an exception by prose in a DOF-derived matrix. A verb outside `arc_to` never raises the question: `arc_fillet` takes modes, and this is not one. And the direction is authored where the family authors every direction, by a binder on a side.

### 2. Cells

| from | result |
|---|---|
| plain point (incl. the entry `Open.at(p)`) | no junction; at the entry the derived departure becomes the entry direction, as `arc_to(Bulge)` does |
| directed point (a leg end) | the derived departure (the arrival direction reflected across the chord) is junction-checked: definitely sharp proceeds; in-band tangent refuses `JunctionTangent`, recourse `.tangent().tangent_arc_to(p)`; reverse refuses `JunctionCusp` (a declared cusp into a point-state arc has no spelling today either: `.cusp()` leaves a Directed tip) |
| Directed tip / `Angle` / `Open` | ill-typed: the bound direction would have to value-match the derived one (the `Center@Directed` rule); a half-bound tip has no position to leave from |
| builder: `.at(p)` then director, or director then `.at(p)` | the arc; tip a directed point at `p` |
| builder: `.to(Point2)` (far end), `.tangent()`, `.turn()`, `.line()` | ill-typed (the anchor IS the end; no incoming tangent to inherit) |
| builder: `.to(Start)` | the tangent closer: arrival at the entry along `Start`'s direction, joint 0 constructed, nothing verified |
| builder: `.at(Start)` + director | the sharp seam, classified as every seam is; a zero turn refuses `SeamTangent`, recourse `.to(Start)` |

Degeneracies mirror `tangent_arc_to` through its kernel with the reflected direction: the direction along the chord and ahead is the straight segment (emitted as `tangent_arc_to` emits it); behind it is the full turn, `DegenerateArcChord`; an anchor at the tip is the degenerate chord. Both ends tangent (a tip whose incoming the derived departure matches) refuses `JunctionTangent` and names `.tangent().tangent_arc_to(Start.arrives_tangent())` or the seam fillet, so the stadium is unchanged.

**Seam relation to `Start.arrives_tangent()`.** Unchanged in meaning (the seam joint is tangent; checked where the leg's data fix its end, constructed where they do not) and unchanged in reach: it is the token of a TARGET slot — `line_to`, `continue_to`, `tangent_arc_to`, the `Bulge`/`Via`/`Center` legs and arrivals. A builder has no target slot; it closes with `.to(Start)`, as the seam fillet does, and `.at(Start.arrives_tangent())` is ill-typed (a second spelling). The `Via`/`Center` half of the PR stands as written.

**Composition.** Not a `spec₂` of `arc_fillet_arc`: an arrival arc of a fillet has its start trimmed, so "start at the tip" means nothing there, and that cell is `Radius` + the same binders. Not a fused incoming: it is not a mode, and an arrival direction cannot be a fused incoming's datum (met mid-arc). Honest residual: an `arrive()` arc filleted at its own end needs `arc_fillet(Radius { r: R, side }, r₂)` with its radius `R` re-typed — the limitation `tangent_arc_to` has today, already filed as the sibling gap.

**Replaces** nothing shipped; it replaces the proposed `Arrive` mode outright (same geometry, same landing — two spellings would be wrong). It is `tangent_arc_to`'s mirror (tangent at the end that has a direction), and the pair reads as one rule: an arc tangent at one end is spelled by the verb for that end.

**Name.** `arrive()`: `.arrive().at(p).toward(-1, 0)` reads "arrive at p heading west". Not "fillet": a fillet authors no vertex and takes `r`; this authors `p` and takes none, so a fillet name would misdescribe it. It is the family's arrival builder standing alone.

### 3. Ratified text this changes

§2 Fillet (the family gains a fifth verb: the bare arrival builder, start pinned at the tip), §2c's family list, §3's table (one verb row, the builder's `At`/`Angle`/`Toward`/`CloseTo`/`At(Start)` rows), §2a a new item. `ArcData` and its "closed and DOF-complete" sentence stand. §2 Closure / §6 PQ4 gain the check-or-construct wording for the token (the `Via`/`Center` half) and name `.to(Start)` on `arrive()` beside the seam fillet as the constructed closers.

### 4. Worked example — your `B`, entered at the spine's foot

```text
Open.at((0, 0)).toward(0, 1).line(2)            // spine
    .toward(1, 0).tangent_arc_to((0, 1))        // upper bump: departure known
    .arrive().at(Start).toward(-1, 0)           // lower bump: arrives west at the foot; sharp seam
```

Entered elsewhere: `.arrive().at((0, 0)).toward(-1, 0)` then `.toward(0, 1).continue_to(Start.arrives_tangent())`. A D with a corner at its tip and a cap tangent at the entry: `.line_to(tip).arrive().to(Start)`.

**Confidence.** The framing beats both earlier shapes: likely. The reading (tangent at arrival, free at departure): likely — confirm. Cells and exclusions: likely. `.to(Start)` rather than the token as its closer: likely. `Via`/`Center` half unaffected: sure.

## For the orchestrator

- Read this round: PR #5374's body and thread, `work/paths/at-cannot-bind-start-in-the-path-form.md` (closed: `.at(Start)` was never ruled out, only unplaced; the `.at(Start)` row here is the Start-anchored spelling that row said would need an `[ev]` PR — this is it), `close_at_seam`, and the `RadiusArrival*` rows. Read-only; nothing built or committed.
- Implementation shape, for the spec: three builder states mirroring `RadiusArrival` / `RadiusArrivalAt` / `RadiusArrivalDir` (holding the tip point rather than a spec), plus `CloseTo` and an `At(Start)` row on them — the `RadiusArrival` states have no `CloseTo` today, so both seam rows are new rows. The arc kernel is `tangent_arc_geom` with the reflected direction; landing is `leg_end_tip`. No `ArcData`, wire-mode, census-cell or viewer-picker change; the persisted-spelling pin gains one verb token.
- The ambiguity in Ev's sentence (§1) should be put as the one question at the top of the PR before the doc is rewritten; the opposite reading is a different verb.
- The two-designer convergence of round 3 is withdrawn by both the orchestrator's mid-arc finding and this framing; the `Arrive` mode should not survive in the PR as an alternative.
