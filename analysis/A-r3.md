# Arrival-constructed arcs — designer report (round 3)

## For Ev

**Recommendation (likely).** One new endpoint-full arc mode, working name `Arrive`, whose END direction is part of the spec: the spec's end is a two-variant value — an authored target (`p` or `Start`) with a director, or `Start.arrives_tangent()` alone — and the director is the ONE direction value type the lattice has (an angle, or exact components), carried by the mode exactly as the `.angle` / `.toward` steps carry it. The arc is `tangent_arc_to` mirrored: chord and end tangent fix the sweep, the departure is derived (the end tangent reflected across the chord) and junction-checked at the tip. It is admitted wherever `Bulge`, `Via`, `Center` are — the sharp `arc_to` from a Point tip and the fused incomings of `arc_fillet` / `arc_fillet_arc` — and nowhere else. `Start.arrives_tangent()` gets one rule on every closer: the seam joint is tangent; the kernel CHECKS it where the leg's data already fix the end direction (`line_to`, `continue_to`, `tangent_arc_to`, `Bulge`/`Via`/`Center` legs, the `Center` fillet arrival) and CONSTRUCTS it where they leave it free (`Arrive`, the `Via` fillet arrival, which then closes at the verb with no director). `Via` and `Center` take the token on every row they close from (issue 1579). Both designers now recommend this; what follows is why the pending-director alternative, which each of us held for one round, is not a coherent final state.

**Definitions.** *End direction*: the arc's tangent at its endpoint, in travel sense. *Director*: an authored direction, angle or components. *Fused incoming*: the arc `arc_fillet(spec, r)` authors and fillets off. *Check / construct*: verify the seam against `Start`'s direction, or build the leg from it.

### 1. The fused rows are owed, and "no concrete need" was use-case gating

Two ratified sentences decide it, both from the §2c conversation (#419): round 7's "Plain `arc_to(spec)` remains for SHARP-cornered arcs; converting sharp→filleted is an edit at the same call site", and round 9's ruling, recorded from your own exhaustiveness argument, that "the ENTIRE family ships — every admissible (site, mode) pair, tested … Use-case-gating applies to no variant; inadmissible pairs remain unrepresentable." Admissibility there is DOF-derived and state-keyed, not spelled per mode: an endpoint-full mode from a Point tip has the DOF `Bulge` has, so `Arrive` is admissible at `FUSED_POINT` / `FUSED_LEG_END` by the same arithmetic that admits `Bulge` there. Round 9 then says it ships. My round-2 "no case I can name" is exactly the gating round 9 forbids, and round 8's own history shows why: `Sweep`/`ArcLen` were "use-case-gated" one round and un-gated the next on this argument. A spelling that cannot reach an admissible site creates an inadmissibility the DOF does not — the pending director cannot sit between a fused verb's spec and its `r` — so it is not a coherent final state under round 9, and it breaks round 7's property besides: a sharp `Arrive` arc could not be rounded at its call site, it would have to be re-authored as a `Bulge` with a hand-computed number, the friction this mode exists to remove. Sure on the reading; the provenance is your argument, agent-recorded.

### 2. Is "the end direction is the leg's construction datum" answered by the `ViaArrival` precedent? No.

Round 2 took the precedent as exact because `fillet_arc(r, Via { q, p }).angle(θ)` leaves a directed point with its outgoing slot empty, so a director already authors an arc's end tangent. That is true and beside the point. In the fillet family every SIDE is "a real on-path point plus a direction" (§2), bound by binders on the side's own builder, and the spec carries only the carrier's residual datum; the builder exists because an arrival side is opened from `Open` and its two bits arrive in either order. A sharp leg has no side builder: round 8 ruled that once the family is DOF-complete "the endpoint lives INSIDE the endpoint-full variants", and every `arc_to` completes at the verb. The end direction is the same kind of datum as the endpoint it belongs to — the data that determine this leg's carrier — and goes where round 8 put the endpoint. The one binder that does author a LEG-like end direction, `ViaArrivalStart.angle(θ)` at the seam, is the odd row out in its own family, not a precedent to extend. The remaining objection to a field, "a second encoding of directions", is answered by §3.

### 3. The third shape: one director value type, two carriers

The objection was never to a direction in a spec (modes already carry angular data: `Sweep`'s angle, `Center`'s winding, `side`) but to a SECOND spelling of "an authored direction" beside the `Angle`/`Toward` steps. The answer is a single value type — `Director { Angle(θ) | Toward { dx, dy } }` — that the two director steps project and the `Arrive` end carries. One type, one lifting rule in editor-core's expression layout (the rule the two steps already need, factored), one exactness contract (`.toward`'s ray verbatim). Nothing is encoded twice. The other third shape, a pending state inside the fused verbs, makes `arc_fillet` the first fused verb that does not complete at the verb and puts two pending directors in a row (the arc's end, then the arrival side's); rejected. The two-variant end, rather than an optional field or two constructors over one struct, keeps "interior `p` with no direction" and "the token with a direction" unrepresentable on the wire as well as in the typed surface.

### 4. Cells

| target | direction | result |
|---|---|---|
| interior `p` | director | arc to `p` arriving along it; tip a directed point at `p`; derived departure junction-checked (in-band tangent refuses `JunctionTangent`, recourse `.tangent().tangent_arc_to(p)`; reverse `JunctionCusp`) |
| `Start` | director | sharp seam, classified as every seam is; zero turn refuses `SeamTangent`, recourse the token |
| `Start.arrives_tangent()` | — | end direction is `Start`'s; joint 0 constructed, nothing verified (issue 1578's closer) |
| fused incomings | director only (`Start` targets refuse there as for every mode) | the arc known by the direction it reaches its anchor at, filleted off |
| Directed tip; fillet-arc arrival | — | ill-typed (over-determined; `Radius`/`Via` + binders already author an arrival by its end) |

Refusals mirror `tangent_arc_to` through its own kernel with the reflected direction: along the chord ahead is a straight segment, behind is `DegenerateArcChord`. The stadium stays `.tangent().tangent_arc_to(Start.arrives_tangent())` — the new closer derives a departure tangent to the straight and refuses `JunctionTangent`, whose text should name that spelling — and both ends tangent stays the seam fillet's. Additive and reversible.

### 5. Ratified text this changes

§2 Legs' mode list and "`tangent_arc_to(p)` stays the tangent-departing endpoint-full form", §2c's "the family is closed and DOF-complete" (it enumerated the carrier data it knew; end tangent was missing), §3's `arc_to(spec)` row, a new §2a item: a seventh endpoint-full mode, shipping at every admissible site per round 9. §2 Closure, §3's token row, §6 PQ4: "the kernel CHECKS" → checks or constructs by the leg's DOF ("every closing verb takes it", your 2026-09-02 ruling, is extended); the "`Via`/`Center` are the one gap" and "construct-from-arrival closer unbuilt" sentences, program.rs's "`Bulge` legs alone" comment and the `Bulge<_, ArrivesTangent>` rustdoc retire — BOOL-12's disclosed deviations, never a design choice.

### 6. Worked example — the `B`, entry at the spine's foot

```text
Open.at((0, 0)).toward(0, 1).line(2)            // spine
    .toward(1, 0).tangent_arc_to((0, 1))        // upper bump: departure known
    .arc_to(Arrive::toward(Start, -1, 0))       // lower bump: arrives west at the foot; sharp seam
```

Rounded foot: `.arc_fillet(Arrive::toward((0, 0), -1, 0), r)` with the spine as the fillet's arrival side — the same spec, the §2c edit at the call site. A D with a corner at its tip and a cap tangent at the entry: `.line_to(tip).arc_to(Arrive::at(Start.arrives_tangent()))`.

**Confidence.** Fused rows owed under round 9: sure. Direction in the spec over a pending director: likely. One shared director type: likely. Check-or-construct token rule: likely. `Via`/`Center` admission: sure.

## For the orchestrator

- Provenance checked this round: round 7 (`86df7f4e8e`) and round 9 (`e502dd8a49`) commits on PR #419; the PR's round-9 fold says "per your exhaustiveness argument" — Ev's argument, agent-recorded, then signed off. Round 8's fold records `Sweep`/`ArcLen` as "use-case-gated", un-gated by round 9: the direct precedent for rejecting my round-2 argument.
- Convergence: B's round-2 and this report recommend the same design; the one refinement here is the shared `Director` value type (§3), which answers B's own round-1 objection rather than conceding it. If the `[ev]` PR respells `Step::Angle`/`Step::Toward` over that type, that is a wire change with no design content and should be its own line in the PR body, not folded in silently.
- Still owed by the `[ev]` PR: `JunctionTangent`'s recourse naming `tangent_arc_to(Start.arrives_tangent())` when raised by the new closer; `ViaArrivalStart`'s rustdoc saying which direction its director binds; the sibling gap (`tangent_arc_to` is a verb, so no fused verb takes a departure-tangent arc toward an authored point — the same round-7 property unmet) as its own `work/paths` row and `[ev]` question.
- Read-only; nothing built or committed; no worktree this round (two `git log -S` and one `gh api` read against the main checkout).
