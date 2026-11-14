# Design fork: a door states what it knows by construction (BOXFINISH, 2026-11-14)

Row: `work/boxfinish/the-extrude-mints-its-plane-normals-and-a-cylinders-u-ref-through-arithmetic-over-the-box.md`.
Protocol: `docs/DESIGN-FORK-PROTOCOL.md` as on main 2026-11-14.

| draw | byte | result |
|---|---|---|
| arm (below 86 → three designers) | 93 | two designers |
| labels (even → Opus is A) | 179 | Fable is A, Opus is B |

Dispatched 2026-11-14, concurrently, same problem statement, as subagents of the orchestrator's session.
Models as dispatched: A = Fable 5.2, B = Opus 5.5 (to be confirmed from each agent's result).

## First reports

- **B** delivered 2026-11-14 (192,128 tokens, 404 s, harness figures). Verbatim: `a-door-states-what-it-knows-by-construction.B-first.md`.
- **A** delivered 2026-11-14 (token and wall figures: see the harness notification, added below when it arrives). Verbatim: `a-door-states-what-it-knows-by-construction.A-first.md`.

Neither designer saw the other's report before both were delivered.

A's figures: 356,437 tokens, 1064 s (harness).

## First positions (judged on the recommended final state)

- **A:** (1) the sweep doors keep the sketch frame they are handed, never re-deriving a direction from placed points: "not a fork", sure. (2) A carrier stores the witness and check 1 stops asking a frame for its length: the fork, likely; named alternative 2′, check 1 asks through the decision funnel. Proposes changing ratified text (geom's at-rest paragraph, the "What is NOT this type" paragraphs of `UnitVec3` and `RigidMotion`, "never up"). Rejected the framing in part ("one act, not three data"; "two different widths are tangled").
- **B:** one rule for every door, carrier data built from the construction's inputs and rigid motions, never re-fitted from vertices (its option A), with the profile recording each segment's unit direction, and a tight unit map as the scalar's normalize (unsure); likely. Would not open changing what a carrier stores or what check 1 asks (its option C). Proposes adding a clause to `crates/sweep/README.md`; no ratified text changed. Confirmed the framing, adding sites.
- **Agreement:** on the first change. **Difference:** whether a second change (what a carrier stores / what check 1 asks) is recommended now.

## Round 1 (2026-11-14)

Each designer resumed and handed the other's first report whole, asked what moved it, whether its own arguments are answered, and the one question beneath the remaining difference.

- **B, round 1** (harness figure after the round: 213,556 tokens; the round took 142 s). **Moved.** Withdrew "off by about δ²/2, cured by bisection" (the bracket norm's deviation is first order off the axes), withdrew "the revolve and tube already follow the rule", dropped the tight unit map. Now recommends two changes in order: the first buildable now with the profile carrying directions; the second (check 1 stops reading a frame's length back) as its own `[ev]` question, leaning to the stored witness. Remaining difference as B states it: whether the tour's stop needs the second change once the first is built. Verbatim: `….B-round1.md`.
- **A, round 1** (harness figure after the round: 395,781 tokens; the round took 285 s). **Moved.** The tour's stop does not need its second change: the circle verb's start direction is exactly ±x and the profile drops it, so the fix is one level above the door. The profile recording directions moves from "candidate, unmeasured" to part of the recommendation (and, A says, needs sign-off: it changes the ratified "Profile format" clause). The carrier-witness change is demoted to a separate row with its own measurement; A still leans to it. Withdrew "the `Vector` arm needs the strut row settled". Adopted a written home for the rule, leaning to `docs/DESIGN.md` beside `[edges]`. Verbatim: `….A-round1.md`.

## Reconciliation ends after round 1: agreed

Both designers moved and met. Agreed final state: (1) every sweep door builds its carrier directions and frames from the construction's inputs and rigid motions, never from placed vertices (buildable now, no ruling); (2) the profile records each segment's unit directions; (3) what check 1 may ask of a frame that truly turns over the box is a separate question, both leaning to a carrier storing the witness; (4) the rule gets a written home. Residue, none of it a position: whether (2) touches ratified text (it does: `crates/profile/README.md`, "Profile format"; B's grep missed it), and which document holds the rule's clause (A: DESIGN.md beside `[edges]`, with the sweep README stating its instance; B: the sweep README), which is Ev's to pick.

Models, as the harness reported each agent's dispatch: A = Fable 5.2 (`model: fable`), B = Opus 5.5 (`model: opus`); the orchestrator is Fable 5.2 (`get_session`: `last_served_model` claude-fable-5-2).

## Ev decided (2026-11-14, PR 6936)

Both changes approved; the rule's text cut down to one sentence in `[intent.stored]` (what a cache holds never flows back toward the description), neither designer's proposed home. The row (268) carries the decision and the mapping on the `[ev]` PR's branch.
