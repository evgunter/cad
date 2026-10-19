## For Ev

**Recommendation (likely).** Add one endpoint-full mode to `arc_to` from a Point tip, provisionally `Arriving { p }`. It names only the endpoint, and the arc's **end direction is bound afterwards by the ordinary director** (`.angle(θ)` / `.toward(dx, dy)`). That is the binder-after-spec shape `fillet_arc(r, Via { q, p })` already uses. At the seam, `Arriving { p: Start.arrives_tangent() }` is complete at the verb: the end direction is Start's, taken by reference. In the same change, give `Start.arrives_tangent()` one rule across every closer: **it says the closing carrier arrives along Start's direction. Where the authored data already fix the end direction it is CHECKED; where they leave it free it SUPPLIES it.** That rule settles the `Via`/`Center` gap (issue 1579) as well, so both rows close together.

Worked example, the `B` (spine up from the entry, bump 1 departs east, bump 2 must arrive west at the spine's foot):

```text
Open.at(foot).toward(0, 1).line(h)                         // spine
    .toward(1, 0).tangent_arc_to(mid)                      // bump 1: departure known
    .arc_to(Arriving { p: Start }).toward(-1, 0)           // bump 2: arrival known; sharp seam, checked
```

If the entry is somewhere else, bump 2 is `.arc_to(Arriving { p: foot }).toward(-1, 0)`. The tip is then an ordinary directed point at `foot` with incoming tangent west, and the next leg binds its own departure. Nobody computes a bulge, a via point or a middle-corner angle.

### Premise check
- **Issue 1579's claim still holds on main (sure).** `do_arc_to_point` refuses `Via @ StartArriving` and `Center @ StartArriving`. The fillet-arrival dispatcher (`do_arrival`) refuses the same two cells, which the row never mentioned. PATHS §2's "every closing verb takes it" lists only `Bulge` among the arc modes, so the text and the code agree on a gap that the 2026-09-02 ruling's *spirit* ("every closing verb") does not allow. Today a `Center` arc that closes tangent at the seam has no spelling: it refuses `SeamTangent` and has no declaration to reach for.
- **The issue's sketch of the Start row is too narrow (sure).** It says "with `p: Start`, the end direction is `Start.dir` and is not authored". The `B` shows why that is wrong: if its entry sits at the spine's foot, bump 2 closes at Start arriving *perpendicular* to `Start.dir`, which is a sharp seam with an authored arrival direction. The Start case therefore has two rows, not one: an authored direction (sharp seam, checked like any seam) and Start's own direction (the tangent seam, declared).
- **The "construct vs check" tension BOOL-12 recorded is not a conflict (likely).** BOOL-12 built the CHECK form because a *constructed* closer cannot close the stadium: its derived departure is an undeclared tangency. That stays true and stays right. The stadium remains `.tangent().tangent_arc_to(Start.arrives_tangent())` (both ends tangent, so one end constructs and one is checked). The new mode is for a **sharp departure**: from a directed point, its derived start tangent is junction-checked like `Bulge`/`Via`/`Center`'s, and a zero turn refuses `JunctionTangent`.
- **What the issue points at.** An endpoint-full arc is a chord plus one more degree of freedom. The vocabulary can author that freedom as a bulge, a through-point, a centre, or a start tangent (`tangent_arc_to`), but not as an **end tangent**. The fillet family already authors arcs by their end (an anchor plus a director), and the sharp leg is the one place that cannot. This is a hole in a family that is otherwise complete, not a seam-specific feature.

### The options as final states
**(1) Recommended: `Arriving { p }` plus a pending director.** The mode's only datum is the target, and the director is the existing step.
- Makes true: one way to spell any direction (the director steps, including `.toward`'s exact ray, which matters for the W1 ulp class). No direction-valued field ever enters `ArcData`, so no second direction encoding has to be kept in step with the first. The derived bulge `tan(θ/4)` with `θ = 2·∠(chord, t₁)` is computed at lowering, so the steps store only authored data (PROFILES-V2 §V1).
- The cells, in the replay grammar:
  - `Arriving @ Point(p)` gives a pending state, and a director makes it a directed point at `p`.
  - `Arriving @ Start` gives a pending close, and a director closes a sharp seam that is checked against `Start.dir`. A zero turn refuses `SeamTangent`, whose recourse is the declaration.
  - `Arriving @ StartArriving` is complete at the verb.
  - Refusals mirror `tangent_arc_to`: an end direction along the chord is a line, and one against the chord is a full turn.
  - Only `.angle` and `.toward` are admitted on the pending states. `.tangent()` and `.turn()` are ill-typed there, as they are on the `Via` arrival, because there is no incoming tangent.
- Leaves possible: the reading hazard that `arc_to(Arriving { p }).angle(a).angle(b)` has two directors in a row, the first binding the arc's end and the second the next departure. The same is already true of `fillet_arc(r, Via { q, p }).angle(θ)`, so this is ratified precedent, not a new cost. The viewer already renders "a via arrival awaiting its director".
- Reversible: yes. It is additive, and a mode can be dropped while nothing is released.

**(2) Direction as a spec field: `Arriving { p, dir }`** (the issue's sketch).
- Pro: one call; the direction reads inside the arc it shapes.
- Con: `dir` needs its own angle-or-ray type in `ArcData` and in both of editor-core's layouts. That is the director vocabulary spelled a second time, and the first time a direction would be authored somewhere other than a director.
- Con: the tangent seam needs `dir` to be *absent*, or to refer to Start's direction, so the spec needs either a second variant or a field that is meaningful only for some targets. Either is a state you can represent but should not be able to.
- I would take this only if you judge the double-director reading in (1) to be worse than a second direction encoding. **This is the one real fork; I lean (1) (likely).**

**(3) A directed-point TARGET form** (for example `Target::Directed(p, dir)`, which every endpoint verb accepts as a check). Rejected (sure). At an interior point no joint is classified by the arrival (the next leg's departure classifies it), so on every verb except the new mode the form would only author a fact twice. A target form with one consumer is that consumer's data. `Start` is different because its next side is already authored.

**(4) Build nothing; author with `Bulge`, `Via` or an `.angle`.** Rejected: the number is one nobody has, which is the friction both rows record.

### The arrival-token rule (changes ratified text)
PATHS §2 "Closure", the declared-seam bullet, currently lists `line_to`, `continue_to`, `tangent_arc_to`, `arc_to(Bulge)` and says the kernel CHECKS the arriving direction. I propose re-wording it to the rule above:
- `Start.arrives_tangent()` is admitted on **every** closer: `Bulge`, `Via`, `Center` and `Arriving` legs, and the `Center`/`Via` fillet-arc arrivals at Start.
- It is CHECKED where the end tangent is fixed: `line_to`, `continue_to`, `tangent_arc_to`, `Bulge`, `Via` leg, `Center` leg, `Center` arrival.
- It SUPPLIES the end tangent where the end is free: `Arriving`, and the `Via` arrival at Start. That second cell replaces "re-type Start's angle into the pending director", which today could only refuse `SeamTangent`.
- Fused incoming rows keep refusing every Start target, which is structural and unchanged.

What this changes: "the kernel CHECKS … and NOTHING else" becomes "checks or supplies, and nothing else". The token still says one thing, that the seam joint is tangent; whether that is enforced by a check or by construction depends on what else the verb fixes. That is the same relationship `.tangent()` has at an interior joint: it declares and constructs in one act. That clause was written as BOOL-12 addendum 3 implementing your 2026-09-02 in-chat ruling. The ruling was "every zero-turn joint is a declared tangent joint", and the "checks" wording is agent text that describes BOOL-12's implementation, not your words. Confidence: likely.

### Scope I would leave out
- `Arriving` as a **fused incoming** (`arc_fillet(Arriving { p }, r)`): its director would have to sit between the spec and `r`, which the fused verb's one-call shape cannot spell, and no case needs it. The issue guesses `FUSED_POINT`/`FUSED_LEG_END`; I would refuse those cells (likely).
- `Arriving` as a **fillet-arc arrival**: its third datum is the leg's start, and an arrival side has no start. `Via` and `Radius` already author arrival arcs by their end (sure).
- Both ends tangent stays the seam fillet's job, or `tangent_arc_to` plus the check where an arc can carry both (unchanged).

Spelling of the mode name is yours. Candidates: `Arriving`, `EndTangent`, `Into`. `Arriving` reads best at the call site.

## For the orchestrator

- **Evidence read:**
  - `crates/profile/src/path/program.rs`: `arc_modes!`, `target_forms!`, and the `spec_dispatch!` blocks `ARC_TO_POINT`, `ARRIVAL`, `FUSED_POINT`, `FUSED_LEG_END`.
  - `path/family.rs`: `ViaArrival`/`ViaArrivalStart`, the binder-after-spec precedent; `via_close` re-binds a direction at Start from a director.
  - PATHS §2 (lattice, Fillet, Closure), §2a items 2 to 5.
  - The PR #1573 body (BOOL-12 §3 "Both-ends tangent, and the DEVIATION").
  - `docs/BOOL-12-SPEC.md` is deleted on main (DOC-LEDGER sweep 15, `623cbe38d0`); I used the PR body instead.
- **Brief corrections:**
  - The brief and row 1579 frame the refusal as `ARC_TO_POINT` only; `ARRIVAL` refuses `Center @ StartArriving` and `Via @ StartArriving` too. The program.rs comment above the dispatchers ("Via and Center could take the same check and do not yet (issue 1579)") is the in-code marker.
  - The B worked example assumes an entry at the spine's foot for the Start row; the issue does not say where Ev's entry was. The design does not depend on it.
- **Not checked (no build was run):**
  - Whether `resolve_arc_close`'s `false` argument in `via_close` is the seam-declared flag. I believe it is, from the BOOL-12 body, but did not trace it. If so, the `Via`-arrival-at-Start supply row means passing `true` with `dir = Start.dir`.
  - Exact refusal names for the new degeneracies: they should mirror `tangent_arc_to`'s.
- **What lands if (1) is ratified:**
  - One `arc_modes!` entry; `ARC_TO_POINT` gains `Arriving @ {Point, Start, StartArriving}` plus `Via`/`Center @ StartArriving`; `ARRIVAL` gains `Center`/`Via @ StartArriving`.
  - Two new pending tip states with `Angle`/`Toward` rows in `transition_table!` (mirrors of `ViaArrival`/`ViaArrivalStart`).
  - editor-core's two layouts and census rows; `arc_spec_census.rs` walks the new cells automatically.
  - The `[ev]` PR edits PATHS §2 (the Legs list of endpoint-full modes, and the Closure declared-seam bullet) and §2a with a new item, and sets `needs_ev: true` on both rows. The two rows should be merged or cross-linked, since one ruling closes both.
- No off-question defects found. Worktree `/home/user/design-arrival-b` removed; no target dir was created.
