# Arrival-constructed arcs — designer report (round 2)

## For Ev

**Recommendation (likely).** Add one endpoint-full mode to `arc_to` from a Point tip — working name `Arriving { p }` — that names only the endpoint; the arc's END direction is bound by the ordinary director afterwards (`.angle(θ)` / `.toward(dx, dy)`), the binder-after-spec shape the fillet family already uses for `fillet_arc(r, Via { q, p })`. The arc is `tangent_arc_to` mirrored: chord and end tangent fix the sweep, the departure tangent is derived (the end tangent reflected across the chord) and junction-checked at the tip like every other Point-state arc's. At the seam, `Arriving { p: Start }` followed by a director is a sharp seam, classified as every sharp seam is; `Arriving { p: Start.arrives_tangent() }` is complete at the verb, the end direction being `Start`'s by reference. In the same change, `Start.arrives_tangent()` gets one rule on every closer — it says the arriving carrier is tangent to `Start`'s direction; where the leg's authored data already fix its end direction the kernel CHECKS, where they leave it free the token SUPPLIES it — which admits `Via` and `Center` legs and the `Center`/`Via` fillet-arc arrivals at `Start` (issue 1579) under the same sentence. This is both designers' recommendation now; round 1's fork (direction as a field inside the mode) is withdrawn below.

**Definitions.** *End direction*: the arc's tangent at its endpoint, in travel sense. *Director*: an authored direction in one of the lattice's two exact spellings, an angle or components. *Check*: a closing leg built from its own data with the seam verified against `Start`'s direction. *Supply*: a closing leg with one free angular DOF that `Start`'s direction fills, so the seam is tangent by construction and nothing is verified.

### What moved, and why

Round 1 put the direction inside the mode (`Arrive { p, dir }`), on the ground that the end direction is the LEG's construction datum, like a bulge, and that a director binds the tip's outgoing slot, which §2 says the incoming direction never is. Two things answer that:

1. **The precedent is exact, and I had mis-weighed it.** Since the OnArc dissolution, the director on a `Via` fillet arrival (`fillet_arc(r, Via { q, p }).angle(θ)`) authors the ARRIVAL arc's tangent at its anchor and leaves the tip an ordinary directed point with its outgoing slot EMPTY (`ViaArrival::angle` returns the `NoAng` directed-point state). So the ratified family already lets a director author an arc's end direction, and already has the two-directors-in-a-row reading (`…).angle(θ₁).angle(θ₂)`, the first the arc's end, the second the next departure). `Arriving { p }.angle(θ)` adds no new kind of binding; a direction field would.
2. **A direction field is the director vocabulary spelled twice.** The program would hold an authored direction in two encodings — the `Angle`/`Toward` steps and an angle-or-ray value inside `ArcData`, mirrored in both of editor-core's layouts and their lifting — and it would be the first direction authored anywhere but a director step. Nothing is gained for it: the steps store only authored data either way (the bulge is derived at lowering, as `Via`/`Center` do), and `.toward`'s exact ray is kept either way.

What did **not** move me, for the record: the "representable-but-wrong" argument against the field. The wire already represents inadmissible cells (`Via @ StartArriving` today) and refuses them by the dispatcher's `refuse` list, and the typed surface keys admissibility on types; a field would have been handled the same way. The case against the field is (1) and (2), not that one.

### The fused rows — a consequence of the spelling, not a need

Round 1 admitted `Arriving` as a fused incoming (`arc_fillet(Arriving { p }, r)`) because the matrix is DOF-derived and the mode has `Bulge`'s DOF from a Point tip. Asked for a concrete case, I have none that `Bulge`'s fused row does not equally serve: the one shape it would spell — an arc known by its arrival direction, filleted on its OWN carrier onto the next side without re-typing its radius — is a friction every endpoint-full mode shares, and `Arriving { p }.angle(θ).arc_fillet(Radius { r, side }, r₂)` (arc extension off the directed point) reaches it today at the cost of the radius. Under the binder shape the director would have to sit between the spec and `r` inside one call, which the fused verbs cannot spell without a pending state of their own. So: refuse `Arriving` on `FUSED_POINT` / `FUSED_LEG_END` (ill-typed, a missing impl), as the other designer says. If a case appears, the row is additive. Sure that no need is identified; likely on refusing.

### Cells (the final state)

| cell | result |
|---|---|
| `Arriving @ Point(p)` | pending state; a director makes it a directed point at `p` with incoming = the director; the derived departure is junction-checked at the tip (definitely sharp proceeds; in-band tangent refuses `JunctionTangent`, whose recourse names `.tangent().tangent_arc_to(p)`; reverse refuses `JunctionCusp`) |
| `Arriving @ Start` | pending close; a director closes a sharp seam checked against `Start`'s direction; zero turn refuses `SeamTangent`, recourse the token |
| `Arriving @ StartArriving` | complete at the verb: the end direction is `Start`'s, joint 0 constructed, nothing verified (the closer of issue 1578) |
| directors on the pending states | `.angle`, `.toward` only; `.tangent`/`.turn` ill-typed (no incoming tangent), as on the `Via` arrival |
| from a Directed tip; as a fillet-arc arrival; as a fused incoming | ill-typed (over-determined; one DOF short and `Radius`/`Via` already author it; no need, above) |
| `Via`/`Center` leg `@ StartArriving`, `Center`/`Via` arrival `@ StartArriving` | admitted: `Center` and the legs CHECK (same kernel as `Bulge`, flag true); the `Via` arrival SUPPLIES (its carrier is the circle through `q` tangent to `Start`'s direction, so the close completes at the verb with no builder) |

Refusals mirror `tangent_arc_to`'s through its kernel with the reflected direction: end direction along the chord ahead is a straight segment, behind is `DegenerateArcChord`. What stays: the stadium is still `.tangent().tangent_arc_to(Start.arrives_tangent())` (the new closer derives a departure tangent to the incoming straight there and refuses `JunctionTangent`), and both ends tangent stays the seam fillet's. Reversible: additive; the family's exhaustive matches make a removal loud.

### Ratified text this changes

- §2 Legs ("`tangent_arc_to(p)` stays the tangent-departing endpoint-full form" and the mode list), §2c's "the family is closed and DOF-complete", §3's `arc_to(spec)` row, §2a as a new item: a seventh endpoint-full mode. Agent re-spell text under the #419 ratification; round 9's rule (every admissible pair ships, tested) applies as written.
- §2 Closure, §3's `Start.arrives_tangent()` row, §6 PQ4: "every closing verb takes it … the kernel CHECKS" → checks or supplies by the leg's DOF. "Every closing verb" is your 2026-09-02 ruling, extended; "CHECKS" is BOOL-12's mechanism text. The "`Via` and `Center` are the one gap" and "construct-from-arrival closer unbuilt" sentences, the program.rs "`Bulge` legs alone" comment and the `Bulge<_, ArrivesTangent>` rustdoc retire — all BOOL-12's own disclosed deviations.

### Worked example — the `B`, entry at the spine's foot

```text
Open.at(foot).toward(0, 1).line(h)                 // spine, foot → top
    .toward(1, 0).tangent_arc_to(mid)              // upper bump: departure known
    .arc_to(Arriving { p: Start }).toward(-1, 0)   // lower bump: arrives west at the foot; sharp seam
```

Entry elsewhere: `.arc_to(Arriving { p: foot }).toward(-1, 0)` and the spine closes with `.toward(0, 1).continue_to(Start.arrives_tangent())`. A D with a corner at its tip and a cap tangent at the entry: `.line_to(tip).arc_to(Arriving { p: Start.arrives_tangent() })`. Nobody computes the middle corner's departure, a bulge or a via point.

**Confidence.** Binder-after-spec over a direction field: likely (the precedent is exact; the second encoding buys nothing). Check-or-supply token rule: likely. `Via`/`Center` admission a leftover, not a question: sure. Fused rows unneeded: sure that no case is identified.

## For the orchestrator

- B's report answers mine on the spelling; the one claim I checked it against is `ViaArrival::angle_kernel`'s return type in `crates/profile/src/path/family.rs` (`PartialPath<T, HasPos<WithIncoming>, NoAng>`), read in round 1, which is what makes the precedent exact. No further reads this round; read-only, nothing built or committed.
- Residual disagreement: none on the design. One emphasis: B's "representable-but-wrong" argument should not be forwarded as the reason — the wire already carries refused cells by the `refuse` idiom; (1) and (2) above are the reasons.
- Still owed by the `[ev]` PR, from round 1: `JunctionTangent`'s recourse text when raised by the new closer should name `tangent_arc_to(Start.arrives_tangent())`; the `ViaArrivalStart` rows' rustdoc should say the director binds the arrival tangent at `Start` (it reads "an angle at the entry anchor"); the sibling gap that no fused verb takes a departure-tangent arc toward an authored point (`tangent_arc_to` is a verb, not a mode) belongs in `work/paths` as its own question.
- Blast radius unchanged from round 1 except smaller: no new field shape in `ProgramArcData` (a target-only variant), two new pending `DynTip` states with `Angle`/`Toward` rows, the persisted-spelling pin re-pinned (wire break: one new mode token), census and viewer rows walk themselves.
