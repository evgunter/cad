# EMBEDGATE — plan

**In flight, handed over with the cut:**

- `the-at-rest-gate-certifies-embedding` (P0, H): the gate lane is a
  cloud session, `session_018u3gVH8SNTMN3JcG6kETSc` ("EMBED lane:
  gate"), idle since 06:17Z on 22 Nov with nothing running. Its branch
  `embed/gate` (7951bfdfb6) holds the gate; `embed/gate-measure`
  (6eb881a631) the census harness, the `CAD_GATE_MINDIST` hook and the
  measured tables. It keeps its `embed/` branch names.

**What the gate's landing waits on** (none of it this track's to build):

- EMBED's readings: the third wave in `injective.rs` (PRs 7113, 7115,
  7315, 7318, 7320, 7322, 7327, 7328, 7329, 7334, and the corner-pieces
  and clip-vertex units), each measured against the per-solid census;
- EMBEDWALK's `share-nothing-face-pairs-are-halved-over-their-carriers-crossing-off-the-faces`
  (P0);
- EMBED's PR 7319 (an ask stops at its first finding; Ev on PR 7309 and
  PR 7319), whose edits the gate lane takes when it lands;
- CHARTTAIL's `a-station-far-from-the-origin-reads-indeterminate-on-its-own-carrier-and-panics`
  (filed on `embed/first-finding`): an Interval ask at ε 1e-12 panics 30
  units off the origin;
- the hand-over Ev approved on PR 6893 (the strict question takes tier
  3's verdict; `AtRestBody` keeps tier 3's per-entity facts), unbuilt.
  What the verdict keeps is being weighed by EMBED's design fork
  `embed-wedge` (`analysis/design-fork/embed-wedge`); the unit it
  yields is cut when Ev has ruled;
- the gate landing must pin EMBED's
  `the-verifier-reads-a-separation-inside-the-band-as-a-coincidence-where-the-tiers-clause-says-ambiguous`
  (contact9's sector ∩).

**Dispatchable now:**

1. `a-finish-carries-the-embedding-facts-of-unchanged-face-pairs` (P0,
   H). It builds on the records type of PR 7124 (`embed/records`, in a
   fix pass under dual review with EMBED's orchestrator); stack on that
   branch, as `heldrecords/split-keys` does, rather than wait.
2. `an-op-over-an-assembly-answers-only-for-what-it-made` (P1, H),
   after the carry or beside it.
3. `the-f64-cavity-gate-refuses-thirty-two-valid-shells` (P0, E): the
   measuring row; it closes into lane C.
