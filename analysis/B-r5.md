## For Ev

**Recommendation (likely): a verb `arrive()` with no arguments, on a point tip.** It opens the fillet family's arc-arrival builder, bound by `.at(p)` and a director (`.angle(θ)` / `.toward(dx, dy)`) in either order. Its arc runs from the tip to `p`, untrimmed and with no tangency asked at the tip, and arrives at `p` tangent to the bound direction. The tip then lands as a directed point at `p`, its incoming tangent being that direction and its departure free, which is how every arc arrival lands today. `ArcData` is untouched, and the `Via`/`Center` token half of this PR stands as written.

**The cells.**
- **Where it starts.**
  - From a plain point (including the entry `Open.at(p)`): there is no junction to check. At the entry, the derived departure becomes `Start`'s direction, as with `arc_to(Bulge)`.
  - From a leg end: the derived departure (the arrival direction reflected across the chord) is junction-checked. A definitely sharp departure proceeds. A tangent one refuses `JunctionTangent`, naming `.tangent().tangent_arc_to(p)`. A reversed one refuses `JunctionCusp`.
  - From a Directed, `Angle` or `Open` tip: ill-typed. A bound departure would have to value-match the derived one, and a half-bound tip has nowhere to leave from.
- **What the builder takes.**
  - `.at(p)` with a director, in either order, builds the arc and lands a directed point at `p`.
  - The far end `.to(p)`, `.tangent()`, `.turn()` and `.line()` are ill-typed: the anchor is already the end, and there is no incoming tangent to inherit.
- **The seam.**
  - `.to(Start)` is the tangent closer. It arrives along `Start`'s own direction, taken by reference, so joint 0 is constructed tangent with nothing to verify, as with the seam fillet. No token is needed (issue 1578).
  - `.at(Start)` with a director, in either order, is the sharp seam with an authored arrival. It is classified as every seam is: a zero turn refuses `SeamTangent`, whose recourse is `.to(Start)`.
  - `Start.arrives_tangent()` stays the token of a target slot. Its rule is unchanged: checked where the leg's data fix the end direction, supplied where they leave it free. The `Via`/`Center` legs and arrivals take it (issue 1579). A builder has no target slot, so `.at(Start.arrives_tangent())` is ill-typed.
- **Degenerate cases**, mirroring `tangent_arc_to` with the reflected direction:
  an anchor at the tip is a degenerate chord; a direction along the chord ahead is a straight segment, behind it the full turn (`DegenerateArcChord`).

  Both ends tangent refuses at the departure, so the stadium stays `.tangent().tangent_arc_to(Start.arrives_tangent())`.
- **Composition.** It is not a `spec₂` of a fillet verb: a fillet's arrival arc starts at a trim point, never at a pinned tip. It is not a fused incoming: it is not a mode, and an arrival direction would be met mid-arc. An `arrive()` arc filleted at its own end is the same filed gap that `tangent_arc_to` has.
- **What it replaces:** the proposed `Arrive` mode, outright. Nothing that has shipped.

**Your `B`, entered at the spine's foot:**
```text
Open.at((0, 0)).toward(0, 1).line(2)          // spine
    .toward(1, 0).tangent_arc_to((0, 1))      // upper bump: departure known
    .arrive().at(Start).toward(-1, 0)         // lower bump: arrives west; sharp seam
```
Entered elsewhere, the lower bump is `.arrive().at((0, 0)).toward(-1, 0)`. That leaves a directed point at the foot with its departure free, so `.toward(0, 1)` turns a sharp corner. A `D` whose cap meets the entry tangentially is `.line_to(tip).arrive().to(Start)`.

**Ratified text to change:**
- §2 Fillet and §2c's family list: the arrival builder standing alone, with its start pinned at the tip.
- §3's table.
- A new §2a item.
- §2 Closure and §6 PQ4: the check-or-supply wording for the token, and `arrive().to(Start)` named beside the seam fillet.
- §2a item 4's open far-end `Start` form stays open.

Confidence: the cells likely; `.at` landing a directed point sure.

## For the orchestrator

- **(2) Settled for A, against my round 4.**
  - A line arrival's `.at(p)` on `Angle` lands `DirectedPlain`, a Directed tip with the side running on along θ (the `At` rows in `program.rs`'s `transition_table!`).
  - An arc arrival's `.at` lands a directed point with no direction bound: `RadiusArrival` → `RadiusArrivalAt` → `radius_complete`, which returns `PartialPath<…, WithIncoming>, NoAng>`; `ViaArrival` does the same.
  - This verb's arrival is an arc ending at the anchor, so it is an arc arrival, not a line side. My round-4 split between `.at` (carry on tangentially) and `.to(p)` (stop) applied line-arrival grammar to an arc arrival.
  - Under A's spelling, `.at(foot).toward(-1, 0)` gives the `B` its sharp corner at the foot with the next leg free. That is exactly what it needs.
- **(1) Settled for A.**
  - `FarEndTo` exists only on the line arrival's `Angle` state: a side that would otherwise run past its anchor stops there. An arc arrival already ends at its anchor, so the far-end form has no meaning on it.
  - An arc arrival anchored at the entry with its director pending has a precedent: `Via { q, p: Start }` → `ViaArrivalStart` → a director closes.
  - `.at(Start)` on this builder is that precedent spelled as a binder, and it is admitted only on arc-arrival builders, where it means something. For a line arrival, `.at(Start)` would make a side pass through the entry, which is a close in the middle of a side.
  - My `.toward(d).to(Start)` instead rules §2a item 4's open line-arrival form for every fillet verb. No case needs that: `at-cannot-bind-start-in-the-path-form` was closed with it left open on purpose, and Ev's report there turned out to be the seam fillet.
  - So A's spelling adds less, and it is the family's grammar.
- **What A's `.at(Start)` costs in code:**
  - `At` takes a `Point2` today, so it needs either a Start-accepting `At` row on the new builder states or a target-typed `at` argument.
  - The viewer's `at` row gains a target picker on those states only.
  - It should not be extended to `RadiusArrival` here; that is the separate "radius-arrival close" this issue named.
- **Name: `arrive()`, conceding to A.** `.arrive().at(p).toward(-1, 0)` reads as what it builds. `arc_onto` implies a side to land on, and that was my line-side misreading. The only cost is some proximity to `arrives_tangent()`, a different word class, which I accept.
- **A's argument answers mine on both points.** My round 4 is withdrawn where it differs. Read-only; nothing built or committed. The worktree `/home/user/design-arrival-b` is removed.
