---
id: the-intersection-ring-facing-is-measured-not-derived
kind: issue
title: An intersection's ring struts face their start germ by a measured rule: the walk reads the end germ in half the two-run poses, and nothing derives why the start builds
status: closed
opened: 2026-10-04
priority: P1
cost: H
refs: [a-pierce-with-two-out-runs-at-one-vertex-refuses-every-op]
closed: 2026-10-05
---


## What

`crates/topo/src/boolean/vtxfac.rs` `classify_vertex_on_face`, step 3.
With two Out runs, the ring struts face their germs by the walk about
the pierced face's outward normal (`insert::strut_order`), except in
an intersection. There, the half leaving the ring vertex always faces
the run's start germ. The claim site says that is measured.

Evidence: PR 4026's review r1, claim 2 (`review-r1/mode-*.txt` on
`join/pierce-two-out-runs-review-r1`; 340 two-run poses per operand
order over six prism corners).

- **The walk splits START / END.** The walk faces the start germ in 171
  poses and the end germ in 169. Where it says START, the rule is a
  no-op. Where it says END, the walk's facing refuses all 169
  `SelfLoopEdge`, and the rule's start facing builds 69 `SOUND` and
  refuses 100 `JoinDesync`, per order.
- **Always facing END** (r1's `inter_end`) refuses all 169 + 171.
- **The keying is pinned to ∩.**
  - Extending the rule to "the piercing operand keeps In" drops
    cube ∖ prism from 169 SOUND to 0.
  - Extending it to "the pierced operand keeps In" drops prism ∖ cube
    from 340 SOUND to 171.
- Review r2 ran ∩ → end and ∩ → walk over 16 966 lines. Neither turned
  a head refusal into SOUND, and the rule never built a BAD body.

So the rule is a fit that holds on every reachable two-run pose. The
recorded purpose is that the ring's In face passes a copy per run, as
the piercing vertex's In face passes that vertex once per run (PR 4026,
the zip's second fusion). That is a topological reason, not a reading
of the germs' geometry. In the poses where the walk reads END, the
start facing contradicts the walk, and the result still validates. The
Out side, where the walk's reading would show, is discarded in an
intersection.

## The shape to give

Derive which face the pinch's crossed corner belongs to in an
intersection, from the germs' geometry or the sense algebra. Then check
the derivation against the START/END split above. If it confirms the
rule, the claim site cites it; if not, the rule changes with the row.

## Built

PR 4038 (2026-10-05). The measured ∩ start rule is gone, and the ring struts face by the walk in every op. That is right once `zip::cross_pinches` crosses the pinch the rule had been dodging. Both dual reviewers built every two-run face ∩ SOUND: r2 350/350 over 8 corners, r1 340 per order on the original five corners plus its new ones, notch307 and shallow200 included. The rule's mutant turns 2 rows red.
