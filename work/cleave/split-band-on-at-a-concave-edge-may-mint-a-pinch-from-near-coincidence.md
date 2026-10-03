---
id: split-band-on-at-a-concave-edge-may-mint-a-pinch-from-near-coincidence
kind: issue
title: Split's ON verdicts at margins within ε can mint a pinch from a value coincidence, which D1 tier 3′ (i) forbids
status: open
opened: 2026-10-02
priority: P1
cost: M
refs: [3856]
design: true
---


## What

DESIGN's tier 3' (i) says near-coincidence never silently becomes contact. A plane within tolerance of a concave edge, but not on it, reads the edge's vertices ON and can mint a pinch half whose touching is an artefact of the band. Unverified: whether an in-band ON verdict escalates typed (Q1) or decides. A row: a plane offset by tol/2 from a notch's tip line.

Found by the TQUERY designer pair weighing `split-halves-have-no-contact-records-so-no-pseudomanifold-self-check` (2026-10-02); read from the tree, NOT run — measure first.

## Measured (2026-10-02, review of PR 3856)

Reproduced on main (9768086e8) and on `tquery/split-pinch-shared-point`.
The fixture is the notched block (`m3_pr3_split.rs` `NOTCHED`, height
1), cut by `plane_y(1 + δ)` (origin `(0, 1 + δ, 0)`, normal `+y`) at
`Tol::witness()` (ε = 1e-9). The notch tip line is at `y = 1` exactly.

- **δ = 1e-13, 1e-12, 1e-11, 1e-10:** split reads the tip ON and
  returns the same pinch half as δ = 0. Above has 3 shells and two tip
  copies at `(4, 1, 0)`.
  - On main the pseudomanifold door with no records refuses that half:
    `UndeclaredContact` ×2 `VertexVertex` plus `EdgeEdgeOverlap`. It
    refuses the δ = 0 half the same way, so it never told the two
    apart.
  - After PR 3856 the copies share the tip's point, and the door
    passes the half at every δ in that range. A plane that misses the
    tip by up to 1e-10 now yields a touching body that nothing
    reports.
- **δ = 5e-10:** split refuses, but in a kernel-defect voice: "geometry
  attachment gate: the unified conventional residual at sample 0
  escalated: margin 4.0e-9 lies inside the ambiguity band (1e-9, 1e-8).
  There is no way through: this is a kernel defect; report it". That is
  an in-band input, not a defect, so the refusal's words are wrong.
- **δ = 1e-9, 2e-9, 1e-8:** split refuses typed: "a vertex lies within
  tolerance of the split plane … move the split plane or the
  geometry". This is Q1's escalation, as it should read.

So after PR 3856 the door cannot catch this: on one shared point, a
band-made pinch and a designed one are the same body. The fix has to
be at split's ON verdict. Today a margin below ε decides ON, and only
a margin inside the band (ε, 10ε) escalates typed. One fix is for a
vertex the plane misses, even by less than ε, to escalate typed (Q1)
rather than decide ON; which ON test is right is CLEAVE's to design.
**P1:** a ratified "never" (D1 tier 3′ (i),
near-coincidence never silently becomes contact) is violated on a
reachable input.


## Designed (2026-10-03): the pinch below ε is correct; the residue is the δ = 5e-10 arm

A designer pair (Opus and Fable; labels on `analysis/design-fork/split-band-on`,
byte 40) weighed this row independently and converged on every point, so it
does not go to Ev:

- **No clash between ratified clauses.** Q1's vocabulary makes |m| ≤ ε
  *coincidence* (Zero) and ε < |m| < Kε *near-coincidence* (the band). D1
  tier 3′ (i)'s "near-coincidence NEVER silently becomes contact (escalated
  typed error instead)" means the band, and split honours it: δ ≥ 1e-9
  refuses typed. Both clauses were written in one commit (`e16309aa7`).
- **δ = 0 and 0 < δ ≤ ε cannot be told apart without an exact-zero test**, which
  the "coincidence is structural or declared" commitment forbids (an
  equal-vs-one-ulp cliff with no band). A rule that refused the δ = 1e-13 pinch
  would also refuse the δ = 0 pinch, and with it `notched_block_end_to_end` and
  the D7 pinch behaviour.
- **"Not ON" is not buildable below ε.** The true cut has prong tips 2δ apart
  and a sliver δ thick, which the census reads as `UndeclaredContact` on
  distinct points. The real choice is the pinch or a refusal, and refusing is
  the ulp cliff.
- **The pinch is tier 3′ (ii) by name**: one ON operand vertex cut into copies
  that share its point. After PR 3856 the door is right to pass it.

So the P1 premise ("a ratified never is violated") does not hold, and the
priority drops to P2. What is owed:

1. **Measure, then fix, the δ = 5e-10 arm.** The tip reads ON (5e-10 ≤ ε), then
   an edge split builds from that verdict fails the attachment gate's chart
   residual at 4.0e-9 (about 8δ; 8 is `NOTCHED`'s x-extent), in kernel-defect
   voice. The designers differ only on the form of the fix, both pending a
   trace:
   - (a) split's ON decision should promise what its build certifies, by
     measuring the margin the build will carry (distance × worst lever);
   - (b) the section pcurve mint is lossy, or the residual is extent-scaled.

   Find which edge and which residual, per `memories/refusal-text-is-not-cause.md`.
2. **Pin δ ∈ {1e-13 … 1e-10} as success**: the same body as δ = 0, passing the
   pseudomanifold door.
3. **`SliverVertex`'s text** says "lies within tolerance of the split plane".
   The arm fires only in the band, beyond ε, so the text says the opposite of
   what decided it. Reword it in `SliverSector`'s manner ("too close to the
   split plane to call").

Brief correction: the fixture is `crates/topo/tests/m3_pr3_split.rs`.

## Overturned by Ev (2026-10-03): the section above is withdrawn

The designers' reading above is wrong. Ev, in chat: "(i) specifically DOES
refer to coincidence < eps, and says we can't infer intent from that." A value
coincidence is not intent, however small the margin. So a pinch that split
mints from a ≤ ε ON verdict, at δ = 0 or at 0 < δ ≤ ε alike, breaks tier
3′ (i). The P1 stands.

The designers are re-weighing in round 2, with this as a given. Open
questions:
- where the line runs between an ON verdict that only adds topology and one
  that creates touching;
- what a user who means the pinch does;
- how far the same inference reaches past split;
- what ratified text then conflicts.

Items 2 and 3 of the withdrawn section (the δ = 5e-10 voice, and
`SliverVertex`'s wording) still need measuring. The "pin the sub-ε pinch as
success" item is withdrawn.
