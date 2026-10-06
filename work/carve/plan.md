# CARVE — the plan

what a sweep verb builds and how it describes it

Re-scoped 2026-10-06 by CARVE's second priority-seam cut
(`work/README.md`, Track size): the P0 rows stay, the rest went to
CARVETAIL and STRUT (`work/carve/log.md`).

## The slate

**30 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `intersection-pair-order-is-unpinned-and-extrude-disagrees-with-itself` | M +design | EdgeDescription::Intersection's (s1, s2) order is unpinned, and extrude writes it one way on cap rims and another on struts |
| P0 | `loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body` | H +design | loft_geometry takes the whole surface's v from the first strip, so a section rolled about its own normal builds a different body |
| P0 | `self-closed-link-sharing-its-vertex-records-two-junctions` | H | walk_chains records two junctions at one vertex and closes the chain when a self-closed link shares its vertex with one other requested link |
| P0 | `self-overlapping-spines-build-and-validate` | H +design | A loft or sweep whose spine revisits itself builds a self-overlapping body and every validation tier says Ok |
| P0 | `skin-coincident-section-check-is-an-unbanded-f64-compare` | H | skin.rs refuses coincident loft sections by a bare f64 strict comparison |
| P0 | `sweep-cap-plane-winds-against-a-convex-arc-region` | M | extrude and loft mint a cap plane inside out when a big convex arc makes the inscribed polygon wind against the region |
| P0 | `two-section-loft-with-an-inverted-top-normal-builds` | H | A two-section loft whose top section's plane normal points down builds and tier 3 says Ok |

## Order

Two streams at once.

**Built now**, because each row's fix shape is written and no choice
on it is Ev's: `sweep-cap-plane-winds-against-a-convex-arc-region`
(orient the cap by the profile's arc-exact winding),
`skin-coincident-section-check-is-an-unbanded-f64-compare` (one
banded decide, stated once) and
`self-closed-link-sharing-its-vertex-records-two-junctions` (count a
self-closed link's vertex twice).

**Weighed first** by an Opus and a Fable designer
(`memories/orchestration-model.md`), each pair given the problem and
not the options:

- `self-overlapping-spines-build-and-validate` together with
  `two-section-loft-with-an-inverted-top-normal-builds`. Both are
  bodies the loft builds and validates when it should refuse, and both
  are a placement of the sections that the loft never checks.
- `loft-v-parameterization-is-the-first-strips-so-a-rolled-section-changes-the-body`.
- `intersection-pair-order-is-unpinned-and-extrude-disagrees-with-itself`.

A pair that agrees on something that is not Ev's fork gets built. One
that is Ev's fork goes to an `[ev]` PR.

## The D10 hold

None of the seven rows reads declared pairs, declared contact,
placement or the node vocabulary. The coincident-section row decides a
coincidence inside one operation by a margined verdict, which is what
D10 says a coincidence is. The three rows the hold does cover went to
CARVETAIL parked.

## Review posture

Protocol v7 (`docs/DUAL-REVIEW-PROTOCOL.md`): the dual review only on
units triaged in. Answered at this sitting's first dispatch: none of
the three built-now units is triaged in. Each gets a single FULL review,
because each is a confident wrong answer if it is wrong (a cap's sense,
a section's verdict, a chain's closure). The weighed rows are triaged
when their build is dispatched.
