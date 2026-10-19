# Arrival-constructed arcs — designer report (round 1)

## For Ev

**Recommendation (likely).** One new endpoint-full arc mode on the Point-state rows — working name `Arrive` — carrying the endpoint and the arc's authored END direction, so the arc is `tangent_arc_to` mirrored: chord plus end tangent fix the sweep, the departure tangent is derived (the end tangent reflected across the chord) and is checked at the tip like any other leg's start tangent. At the seam the same mode has two shapes: `Arrive` at `Start` with an authored end direction (a sharp seam, classified like any seam), and `Arrive` at `Start.arrives_tangent()` with no direction, where the kernel takes `Start`'s own direction and CONSTRUCTS the tangent seam. Independently, `Via` and `Center` take `Start.arrives_tangent()` on every row they close from (sharp leg and fillet arrival); their refusal is a scope leftover with no semantic content. Both halves change ratified text (§2 Legs/Closure, §2c's family list, §3's table, §6's PQ4 entry); the clauses and the provenance are below.

**Definitions.** *End direction*: the arc's tangent at its endpoint, in travel sense. *Director*: an authored direction in one of the two exact spellings the lattice already has — an angle (`.angle(θ)`, sin/cos-quantized) or components (`.toward(dx, dy)`, ray stored verbatim). *Check form*: a closing leg built from its own data, with the seam declared tangent by the target token and verified (`seam_arrival_check`). *Construct form*: a closing leg with one free angular DOF, which `Start`'s direction fills, so the seam joint is tangent by construction and nothing is verified.

### 1. Premise check — what the two rows are evidence of

The brief's framing holds, with one correction and one widening.

- *Correct on main*: `Via @ StartArriving` and `Center @ StartArriving` are still refused as replay `Transition` violations on both the sharp-leg dispatcher (`ARC_TO_POINT`) and the fillet-arrival dispatcher (`ARRIVAL`) in `crates/profile/src/path/program.rs`, and the typed surface has no `PointLeg` impl for `Via<T, ArrivesTangent>` / `Center<T, ArrivesTangent>` (`family.rs`). The program.rs comment and the `Bulge<_, ArrivesTangent>` impl's rustdoc both say this is "a scope statement rather than a rule". The kernel they would call is the one `Bulge` uses, `arc_to_start(leg, declared: true, …)`, which takes any built arc. There is nothing to design there — the special case earns nothing (sure).
- *What the gap is evidence of*: the lattice can author the carrier of a Point-state arc by a bulge, a through-point or a centre, but not by the one datum an author most often has, the end tangent — while the DEPARTURE tangent has its own verb (`tangent_arc_to`) and the fillet family already takes "anchor + direction" through binders (`fillet_arc(r, Radius { r, side }).at(p).angle(θ)`). So the missing concept is an arc leg determined by its endpoint and its end direction. The seam closer of issue 1578 is the one cell of that mode where the direction is not authored.
- *Widening the brief missed*: Ev's `B` as described (loop entered at the bottom of the spine, second bump arriving PERPENDICULAR to the spine) is a **sharp** seam with an authored arrival direction, not a tangent one. So the mode's `Start` row must take an authored director too, exactly as its interior row does; the tangent closer is a third cell, not the `Start` cell.

### 2. The mode — final state

`Arrive` is an endpoint-full mode of `ArcData`, admissible exactly where `Bulge`/`Via`/`Center` are: the sharp leg from a Point tip (plain or directed) and the fused incomings from a Point tip (`FUSED_POINT`, `FUSED_LEG_END`). Excluded from a Directed tip (both tangents bound → over-determined; the `Center @ Directed` argument verbatim) and from fillet ARRIVALS (endpoint + end direction leaves the trimmed start free, a one-parameter family — which is precisely the data `Radius { r, side }` + binders already carries; no second spelling).

Cells, with what the kernel does:

| target | direction | result |
|---|---|---|
| interior `p` | authored director | the arc through tip and `p` with end tangent `dir`; sweep θ = 2·∠(chord, dir); tip becomes a directed point at `p` with incoming `dir`; the derived departure is junction-checked at the tip (a definitely-sharp departure proceeds; in-band tangent refuses `JunctionTangent`; reverse refuses `JunctionCusp`) |
| `Start` | authored director | same leg to the entry; the seam is classified as every sharp seam is (zero-turn refuses `SeamTangent`, undeclared) |
| `Start.arrives_tangent()` | none | the leg built from `Start`'s own direction; joint 0 constructed, no check — the construct form |
| interior `p` | none | ill-typed (nothing supplies the direction) |
| `Start.arrives_tangent()` | authored director | ill-typed (the direction said twice) |

Refusals are `tangent_arc_to`'s mirrored and run through its own kernel: `dir` along the chord and ahead is a straight segment, behind is `DegenerateArcChord` (the full turn); `tangent_arc` already returns no arc at exact collinearity. Nothing new is decided by value; the departure is derived, as `tangent_arc_to`'s end tangent is today, and checked where that one is checked. The step stores the endpoint and the director as authored (angle or components — the `.toward` exactness contract rides inside the mode), and derives the bulge at replay, as `Via`/`Center` do (V1).

What it makes true: an author who knows how an arc arrives writes that and nothing else; the departure angle, the bulge and the via point stop being numbers to solve for. The fused rows come free and are real (an incoming arc known by the direction it would have reached its anchor at, filleted off). The viewer's picker, `arc_specs_at` and the census walk it with no further edit.

What stays: `tangent_arc_to(Start.arrives_tangent())` is still the stadium's spelling — the construct form derives a departure tangent to the incoming straight there and refuses `JunctionTangent`, and the both-ends-tangent case stays the seam fillet's (ratified; unchanged). The two closers do not overlap: one constructs the departure and checks the arrival, the other constructs the arrival and checks the departure. `JunctionTangent`'s recourse text should name the first when raised by the second.

Reversible: yes — a mode is additive and deletes cleanly; the family's exhaustive matches make the deletion loud.

### 3. `Start.arrives_tangent()` — one token, construct or check by DOF

Today the token means "the seam joint is a tangent joint" and the ratified text says every closing verb takes it and the kernel CHECKS. Two facts strain that sentence: `.to(Start)` (the seam fillet) already closes tangent with no token, and the `Via` fillet-arrival close (`fillet_arc(r, Via { q, p: Start })` → builder → `.angle(θ)`) authors the ARRIVAL direction at `Start` through a director — so its zero-turn case refuses `SeamTangent` with no way to declare it, and its tangent form would have no director to bind at all.

Proposed reading, which the mode above and both 1579 rows then follow: **the token classifies the joint; whether the kernel checks or constructs is decided by whether the arriving leg's authored data already fixes its end direction.** Fixed (line_to, continue_to, tangent_arc_to, Bulge/Via/Center legs, the Center arrival): check, as today. Free (the `Arrive` leg, the `Via` arrival — its carrier is the circle through `q` tangent to `Start`'s direction, so the close completes at the verb with no builder): construct, like `.tangent()` on the departure side, where every leg's data leaves its start direction free and the declaration is always a construction. `Start` in a target slot stays a POSITION for every mode (it is one for `Bulge`/`Via`/`Center` today), so no leg silently reads a direction off a bare `Start` — "`Start` alone reads as it always did" is kept in its sense. The alternative — `Arrive { p: Start }` bare reading `Start` as a directed-point VALUE and constructing without the token — is defensible (`.to(Start)` is its precedent) but makes the mode the one place where a bare `Start` means tangent; I lean against it (likely).

### 4. Ratified text this changes, with provenance

- §2 Legs ("`tangent_arc_to(p)` stays the tangent-departing endpoint-full form"), §2c's family enumeration and "the family is closed and DOF-complete", §3's `arc_to(spec)` row: the family gains a seventh endpoint-full mode. Agent-written re-spell text (LIB-RESPELL) under the #419 ratification; the enumeration listed the carrier data it knew, and end tangent was not among them. Round 9's rule — every admissible pair ships, tested — applies as written.
- §2 Closure, §3's `Start.arrives_tangent()` row, §6 PQ4: "every closing verb takes it … the kernel CHECKS" → the token classifies the joint; check or construct per the leg's DOF (above). The "every closing verb takes it" half is Ev's in-chat ruling of 2026-09-02 and is extended, not reversed; the "CHECKS" half is BOOL-12's mechanism text. The "`Via` and `Center` modes are the one gap" and "construct-from-arrival closer … unbuilt" sentences retire; both are BOOL-12's own disclosed deviations (PR #1573 §3 and §9), never a design choice.
- `program.rs`'s "admitted on `Bulge` legs alone" comment and the `Bulge<_, ArrivesTangent>` rustdoc retire with the impls that replace them.

### 5. Worked example — Ev's `B`

Spine up from the entry, bumps on the right, seam at the bottom; every direction is a fact the author has, and the middle corner's departure is never written:

```text
Open.at((0, 0)).toward(0, 1).line(2)              // spine, bottom → top
    .toward(1, 0).tangent_arc_to((0, 1))          // upper bump: departs perpendicular, lands mid-spine
    .arc_to(Arrive::toward(Start, -1, 0))         // lower bump: arrives at the bottom perpendicular; sharp seam
```

With the loop entered at the middle of the spine instead, the lower bump is `.arc_to(Arrive::toward((0, 0), -1, 0))` and the spine closes with `.toward(0, 1).continue_to(Start.arrives_tangent())`. A D whose tip is a corner and whose cap must meet the straight tangentially at the entry: `.line_to(tip).arc_to(Arrive { p: Start.arrives_tangent() })` — the closer of issue 1578.

**Confidence.** The two 1579 rows are a leftover and not a design question: sure. The mode, its cells and its exclusions: likely. The construct-or-check reading of the token: likely. The `B` needing a sharp-seam `Arrive` row: sure (Ev's own description of the arrival as perpendicular).

## For the orchestrator

- **Assumed**: the exact spelling of the director inside the mode (two constructors `Arrive::angle(tgt, θ)` / `Arrive::toward(tgt, dx, dy)`, plus the token-only form; a `Director` value type with the two authored shapes) is the `[ev]` PR's to fix; the report states cells, not Rust. The name `Arrive` is a working label.
- **Could not check**: `git log --all -S` on the "stays the tangent-departing" sentence returned an unrelated commit (shallow graft); provenance above is from §2c's own header and PR #1573's body. `docs/BOOL-12-SPEC.md` is deleted on main (DOC-LEDGER sweep 15, `623cbe38d0`); read at `623cbe38d0^`.
- **Blast radius (not a ranking input)**: `ArcData` is matched exhaustively in `carries_radius`, every `spec_dispatch!` refuse list, `editor-core`'s `ProgramArcData` layout, lifting and `spec_roles`, the persisted-spelling pin in `switch_program_vocabulary.rs` (a wire break: new tokens), `arc_spec_census.rs`'s `spec()`, the viewer's `arc_mode_label` and field rendering, and `pncad`'s Python surface. A director payload inside a mode is a new field SHAPE for the Formula-bearing layout (one angle or a pair), so `ProgramArcData` needs a lifting rule, not just a variant.
- **Defects off the question, not filed (read-only checkout)**: (a) `ViaArrivalStart`'s director authors the arrival tangent at `Start`, where every other arrival-builder director binds the anchor's OUTGOING direction — the row's rustdoc ("completes the close with an angle at the entry anchor") hides that; it is the 1579 arrival row and is covered above, but the rustdoc should say which direction it binds either way. (b) `tangent_arc_to` being a verb rather than a mode means no fused verb can take a departure-tangent arc toward an authored point (`arc_fillet(<tangent to p>, r)` has no spelling; `.tangent_arc_to(p).fillet(r)` is ray extension, not a fillet on the arc). Sibling gap, `work/paths`, separate `[ev]` question; not folded in here.
- Nothing was built; no target directory was created. The worktree `/home/user/design-arrival-a` is removed.
