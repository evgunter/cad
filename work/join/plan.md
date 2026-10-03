# JOIN — the plan

The boolean's join: chord matching, loose-end pairing, and which loop
of a section is the IN copy.

Cut from ZIP on 2026-10-02 along its layer seam (`work/README.md`, Track
size; Ev, in chat, chose three tracks by layer over two by priority).

## The slate

**22.5 budget points** of dispatchable work against a ceiling of 30.

| pri | item | cost | title |
|---|---|---|---|
| P0 | `an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired` | H +design | an operand edge lying in a cutter face, the cutter ending inside the operand: `UnpairedLooseEnds` |
| P0 | `dumbbell-joint-union-leaves-four-loose-ends` | H | the dumbbell's declared joint: `UnpairedLooseEnds { count: 4 }` |
| P0 | `join-desync-on-the-star-fixture` | H | the star fixture's last fold: `JoinDesync` |
| P1 | `blind-d-pocket-subtract-refuses-with-join-internal-words` | H | a blind D pocket from the top face: ring-run winding `Zero` |
| P1 | `ring-run-winding-is-a-second-spelling-of-the-loop-winding-sum` | M | `ring_run_ccw` spells the winding sum a second time |

## Order

Three lanes, independent of each other, so they can run at once:

1. **The ring lane's winding.** `ring-run-winding-is-a-second-spelling-of-the-loop-winding-sum`
   first, then `blind-d-pocket-subtract-refuses-with-join-internal-words`,
   whose top-entry refusal is raised by the function the first row
   rewrites. One lane, one PR if the second closes cheaply once the
   first has landed.
2. **An operand edge lying in the partner's face.**
   `an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired`
   and `dumbbell-joint-union-leaves-four-loose-ends` meet the same
   refusal at the same site (`connect`'s leftovers), on an operand edge
   lying in the other operand's face. Whether they are one defect is
   the first thing to measure. What the join should do with such an edge
   is open: reuse it as the section segment, the way the REST zip
   reuses one inside a declared union, or refuse at a typed door. That
   question goes to the two designers before a lane builds anything.
3. **The star fixture.** `join-desync-on-the-star-fixture`: diagnose
   first. Its refusal comes at the last fold, against a merged cap face.

## Review posture

Per unit, at dispatch, by the review tiers of
`memories/orchestration-model.md`; the log names each tier and its
reason. The A/B experiment is suspended, so the track carries no band.

## The intent-refactor hold (Ev, `[ev]` PR #3990, 2026-10-03)

Started units finish: JOIN-2 (PR 3880, with the REST-zip segments row
it closes) and the reflex-wedge membership (PR 3962). Rows that
meaningfully use declared contact, declared flush pairs, continuations
or the REST zip are parked with `blocked_on: [3990]` (the ruling's
item is not on main yet, so the PR number stands for it, as MSOLVE
parks):

- `four-germ-vertex-pairs-run-b-in-a-order` (P0): its fix waits on the
  REST zip refusing a union that is not a pure REST contact;
- `peg-in-socket-union-refuses-join-desync-at-a-coarse-eps`;
- `a-declared-flush-wedge-sunk-in-a-block-refuses-its-intersect-join-desync`;
- `declared-flush-intersect-refuses-in-one-operand-order`;
- `reflex-corner-edge-in-face-poses-zip-a-ring-parallel-to-its-section-loop`
  (its poses are declared-flush).

Still startable, because they are undeclared booleans or join topology
alone: the ring re-homing pocket, the reflex vertex's B senses, the
tube on a ball, the closed in-face loop (its join half), the
cylinder-sphere frame, conic ranking, parallel cylinders, and the
fan-end consolidation (its "mate" is the half-edge mate). So JOIN is
not blocked.
