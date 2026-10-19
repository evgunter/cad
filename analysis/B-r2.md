## For Ev

**Recommendation (likely): a new endpoint-full arc mode, `Arrive`, whose authored END direction is part of the spec.** The spec carries the endpoint and the direction the arc arrives along. It is `tangent_arc_to` mirrored: the chord and the end tangent fix the sweep (θ = 2·∠(chord, dir)), and the departure is derived and junction-checked at the tip like any arc leg's start.
- **Where it is admitted:** wherever `Bulge`, `Via` and `Center` are, meaning the sharp `arc_to` from a Point tip and the fused incomings of `arc_fillet` / `arc_fillet_arc`.
- **Where it is not:** a Directed tip (both tangents bound, so it is over-determined) and fillet arrivals (`Radius` with `.at` and a director already says "anchor plus direction" there).
- **The direction is one director value carried as data.** It is an angle or exact components (`.toward`'s verbatim ray, so the W1 exactness contract rides along), and the step stores it as authored; the bulge is derived at replay.

**Its cells, with the bad combinations unspellable rather than refused:**

| target | direction | meaning |
|---|---|---|
| interior `p` | authored | arc to `p`, arriving along `dir`; the tip becomes a directed point at `p` |
| `Start` | authored | sharp seam, classified like any seam (a zero turn refuses `SeamTangent`) |
| `Start.arrives_tangent()` | none | the direction is `Start`'s own; tangent seam constructed (issue 1578's closer) |

There are two further combinations, and the spec's shape makes both unrepresentable rather than refused. One is an interior `p` with no direction. The other is the token *with* a direction, which would state the direction twice. The way to do that is a two-variant end inside the mode, not an optional field. A bare `Start` stays a position, as it is for every other mode; only the token reads `Start`'s direction.

**The `Start.arrives_tangent()` rule, which changes ratified text.** The token keeps one meaning: the seam joint is tangent. Whether the kernel *checks* it or *constructs* it depends on whether the leg's other authored data already fix its end direction.
- It is a check for `line_to`, `continue_to`, `tangent_arc_to`, and the `Bulge`, `Via` and `Center` legs, as well as the `Center` fillet arrival.
- It is a construction for `Arrive` and for the `Via` fillet arrival at Start. That arrival closes at the verb, with its carrier being the circle through `q` tangent to `Start`'s direction. Today it can only refuse `SeamTangent`, because the only way to give it that direction is to re-type `Start`'s angle.
- `Via` and `Center` take the token on every row they close from. Their current refusal is a disclosed BOOL-12 scope leftover with no rule behind it (sure); this settles issue 1579.
- PATHS §2 Closure and §6 PQ4 change from "the kernel CHECKS" to "checks or constructs". "Every closing verb takes it" is your 2026-09-02 ruling and is extended, not changed; "CHECKS" is BOOL-12's mechanism text.

**Why the direction goes in the spec, not in a director bound afterwards.** The other answer considered was `arc_to(Arrive { p }).toward(-1, 0)`, with a pending-director state like `fillet_arc(r, Via { q, p })`'s. It loses on three counts:
1. **It breaks a ruled property.** §2c says "converting sharp→filleted is an edit at the same call site", and round 9 ruled that every admissible (site, mode) pair ships. A pending director cannot sit between `arc_fillet`'s spec and its `r`, so the fused rows could not exist. A rounded-corner version of a sharp `Arrive` arc would then have to be re-authored as a `Bulge` with a hand-computed number, which is the friction this mode removes.
2. **The fused need is real.** Take a `B` whose foot is rounded rather than sharp: the lower bump still "arrives perpendicular to the spine" at the virtual corner the fillet trims away. That end point has the same status `Bulge`'s and `Via`'s `p` already have in those rows.
3. **The direction is the leg's data, not the tip's.** A director on a fillet arrival binds that side's own anchor. For a sharp leg, the end tangent is how the carrier is constructed, exactly as `Bulge`'s `b` or `Center`'s `c` are, and every other `arc_to` completes at the verb. Carrying it as a field is the same director value in a different place, not a second concept: modes already carry angular data (`Sweep`'s angle, `Center`'s winding).

The remaining cost is real but small: editor-core's expression layout needs one lifting rule for a director-shaped field (an angle, or a pair of components). Reversible: yes; the mode is additive.

**Worked example, your `B`** (entered at the spine's foot; bumps on the right):

```text
Open.at((0, 0)).toward(0, 1).line(2)            // spine
    .toward(1, 0).tangent_arc_to((0, 1))        // upper bump: departure known
    .arc_to(Arrive::toward(Start, -1, 0))       // lower bump: arrival known; sharp seam
```

With a rounded foot, the same spec moves into the fused verb: `.arc_fillet(Arrive::toward((0, 0), -1, 0), r)`, and the spine follows as the fillet's arrival side.

**Stays as it is:** the stadium remains `.tangent().tangent_arc_to(Start.arrives_tangent())`. Constructing from the arrival would derive a departure tangent to the straight, which is an undeclared tangency, so it refuses `JunctionTangent`, and that refusal's text should name the stadium spelling. The both-ends-tangent case remains the seam fillet's job.

**Confidence:**
- The 1579 half is sure.
- The mode, its cells, and the direction being in the spec are likely.
- The check-or-construct reading of the token is likely.

## For the orchestrator

- **What moved me.** A's argument answers mine, but by way of ratified text A did not cite. §2c's family list says "Plain `arc_to(spec)` remains for SHARP-cornered arcs; converting sharp→filleted is an edit at the same call site", and round 9 (Ev) rules that every admissible (site, mode) pair ships with no use-case gating. My round-1 pending director cannot be spelled on the fused rows, so under those rulings it is not a coherent final state. I had treated the fused rows as optional scope; they are not.
- **Is the in-mode director a "second encoding"?** No. It is the same Angle/Toward value carried as a field. I concede the point.
- **The precedent I leaned on was weaker than I claimed.** Fillet-arrival directors bind the arrival side's own anchor, which is a tip binding. A also notes that `ViaArrivalStart`'s director is the odd one out: it binds the arrival tangent at `Start`.
- **Where A and I still differ.** Only on detail:
  - The mode's end should be a two-variant field (authored target plus director, or the token alone), not constructors over an optional field. That makes A's two "ill-typed" cells unrepresentable on the wire as well as in the typed surface.
  - `Via`'s fillet arrival at `Start.arrives_tangent()` completing at the verb with no director is a construction row. A says so too; it needs its own census cell.
- **A's off-question defect (b) is worth filing as a sibling `[ev]` question:** `tangent_arc_to` is a verb, not a mode, so it has no fused form. It is the same sharp→filleted property, unmet for departure-tangent arcs. I did not file it (read-only).
- Nothing was built or committed. The worktree `/home/user/design-arrival-b` was re-created for this round and is removed.
