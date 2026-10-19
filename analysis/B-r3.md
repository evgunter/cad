## For Ev

**Recommendation.** Two parts are settled. The question of where the arc's end direction is written is a genuine choice for you, and I lean (b) below.

**Settled (sure on the second half, likely on the rest).**
- **The new mode.** `arc_to` from a Point tip gains one endpoint-full mode, working name `Arriving`. It is `tangent_arc_to` mirrored: the endpoint and the arc's END direction fix the sweep. The departure is derived and junction-checked at the tip; a zero turn refuses `JunctionTangent`, whose recourse names `.tangent().tangent_arc_to(..)`.
- **Its three seam cells.**
  - Interior `p` with an authored direction.
  - `Start` with an authored direction: a sharp seam, classified like any seam.
  - `Start.arrives_tangent()` with no direction: the direction is `Start`'s own, so it closes as a tangent seam (issue 1578).
  - A bare `Start` stays a position.
- **The token rule.** `Start.arrives_tangent()` means one thing on every closer: the seam joint is tangent. Where the leg's other data already fix its end direction, the kernel checks it (`line_to`, `continue_to`, `tangent_arc_to`, the `Bulge`/`Via`/`Center` legs, the `Center` fillet arrival). Where the end is free, the token supplies it (`Arriving`, and the `Via` fillet arrival at `Start`). `Via` and `Center` taking the token closes issue 1579; their refusal today is a leftover with no rule behind it.

**The fork: where the end direction is written.**

**(a) As a director step after the spec:** `arc_to(Arriving { p }).toward(-1, 0)`.
- For: every direction in the vocabulary stays a director step.
- For: the precedent is exact. The director on a `Via` fillet arrival (`fillet_arc(r, Via { q, p }).angle(θ)`) already authors an arc's end tangent and leaves the tip with no outgoing direction bound.
- Against: it cannot be spelled on the fused verbs. `arc_fillet(Arriving { p }, r).toward(…)` would put a director exactly where, after every other `arc_fillet`, `.toward` binds the arrival side. The same word in the same position would then mean two things, decided only by the typestate.
- Against: so `arc_fillet(Arriving…)` would have to be refused. That breaks two ratified statements that hold for every `arc_to` mode today:
  - "converting sharp→filleted is an edit at the same call site" (§2c round 7);
  - "every admissible (site, mode) pair ships … use-case-gating applies to no variant" (your round-9 ruling).
- The fused pair is admissible on degrees of freedom: one datum past the chord, exactly like `Bulge`.

**(b) As a director value inside the spec:** `arc_to(Arriving::toward(p, -1, 0))`, and `arc_fillet(Arriving::toward(p, -1, 0), r)` with no further edit.
- For: both ratified statements hold, and the fused rows ship as the rule says.
- Against: it is the first direction authored anywhere other than a director step.
- That cost is mostly removed by giving the `angle`/`toward` steps and the mode ONE shared `Director` type (an angle, or exact components stored verbatim). Then it is one encoding carried in two places, not two encodings, and editor-core lifts it with one rule.
- The two-variant end (a target plus a director, or the token alone) makes "a point with no direction" and "the token with a direction" unwritable.

**I lean (b) (likely).** The thing (a) protects ("directions are steps") is an observed pattern; I find no ruling behind it. What (b) protects was written in your rounds, round 9 in your own ruling.

**What changes in the fused case.** The fused incoming's `p` is a real on-path anchor; the fillet trims the arc's carrier beyond `p`, towards the virtual corner. So `arc_fillet(Arriving…, r)` means "arrive at `p` along `dir`, then round into the next side". It does not mean "arrive along `dir` at the corner".
- That is no different from `Bulge`'s fused row today, and I know of no case that only this row serves.
- That is why I rest the row on the rule, not on a need. If the rule is not meant to bind new modes, (a) is the cleaner spelling and the fused rows can come later.

**Rejected third shape.** A fused pending state (`arc_fillet(Arriving { p }, r)`, then a director that binds the incoming arc's end) lets (a) reach the fused rows. But it is exactly the collision above: after `arc_fillet`, `.toward` would mean the incoming arc's end in one mode and the arrival side in every other.

**Worked example: your `B`, entered at the spine's foot.**
```text
Open.at(foot).toward(0, 1).line(h)                 // spine
    .toward(1, 0).tangent_arc_to(mid)              // upper bump: departure known
    .arc_to(Arriving::toward(Start, -1, 0))        // (b)
    .arc_to(Arriving { p: Start }).toward(-1, 0)   // (a), the same leg
```
Either way the stadium stays `.tangent().tangent_arc_to(Start.arrives_tangent())`, and when both ends are tangent the seam fillet does it.

## For the orchestrator

- **(1) Checked.** `ViaArrival::angle_kernel` returns `PartialPath<T, HasPos<WithIncoming>, NoAng>` (`crates/profile/src/path/family.rs`). A is right: the director binds the arrival arc's end tangent, not the tip's outgoing direction. My round-2 point (3) is withdrawn.
- **Other direction-like fields in specs are not end directions:** `Sweep`'s angle is a sweep, `Center`'s winding and `Radius`'s side are bits, and `CircleSplit`'s phase is a position on the circle. So under (b), `Arriving` would hold the only direction authored inside a spec. That is A's point and it stands; (b) answers it with the shared type, not by denying it.
- **(2) Provenance.**
  - "edit at the same call site" comes from `86df7f4e8e` (§2c round 7, the fusion reframe), written during the rounds with Ev and ratified with #419. It is not a stand-alone ruling.
  - Round 9 comes from `e502dd8a49` and is marked "RULED (Ev, round 9)". Its forcing argument was wire exhaustiveness (no stub arms), and it is about modes shipping rather than about new modes' fused sites. Reading it as covering every degree-of-freedom-admissible site of a new mode is my extension. I hold it as likely, not sure.
- **Does the property hold today?** Within `arc_to(spec)`, yes:
  - `Bulge`, `Via` and `Center` from a Point tip, and `Sweep` and `ArcLen` from a Directed tip, all have fused rows.
  - The exclusions are `Center@Directed` (degrees of freedom) and every `Start` target on the fused rows (structural).
  - It fails for `tangent_arc_to`, a verb outside `ArcData` that §2 kept on purpose. So the property holds for the family `Arriving` joins, and A's defect (b) (no fused tangent-departing arc) is the one existing gap.
- **Correction to my round 2.** The rounded-foot `B` example was wrong. The fused arc's `p` is retained, and the corner lies beyond it on the arc's own carrier. So the direction is at `p`, not at the rounded corner, and there is no concrete need. A is right on that point.
- **Residual disagreement.** A and I disagree only on whether round 9's rule binds a new mode's fused sites, and that is Ev's to read. Everything else is shared.
- Read-only; nothing built or committed. The worktree `/home/user/design-arrival-b` is removed.
