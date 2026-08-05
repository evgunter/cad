# M5 S1 — the REST-contact join lane (binding spec)

Executes M5-PLAN S1 (#102's crosslap frontier; M3 envelope class
(iii)): the join stage learns to zip **declared boundary-on-
boundary REST contacts** — mates whose interiors are disjoint and
whose shared geometry lies entirely on both operands' boundaries,
so no chord has a facing partner and today's join refuses typed
(`JoinDesync` family). Branch `ev/m5-s1-rest-join` from main.
Planar unit (R4); exit-listed.

## 1. Scope and trigger

- **Union only.** The recorded frontier is the declared union
  (crosslap mate, corner-flush REST). Subtract/intersect on pure
  REST contacts keep their typed refusals; update their door text
  to name the narrowed frontier (union now joins; these ops still
  refuse) composing the shared recourse carrier.
- **Declared-only, verified-never-trusted.** The lane is reached
  exclusively through the declared-coincidence rung (M4 PR 5's
  ladder): the undeclared mate MUST keep refusing
  `UndeclaredCoincidence` at the coincidence door —
  `crosslap_rest.rs`'s narrowing pin stays green verbatim. False
  declarations keep contradicting where the lie meets geometry
  (`DeclarationContradicted`), including through the new lane.
- Planar faces only; a REST contact involving curved carriers
  refuses typed naming the M5 curved-boolean chain (those arrive
  with PR 9's zip, not here).

## 2. Semantics

Union of two bodies with disjoint interiors + declared REST
contact region R:
- The contact patches (the face regions of A and B that coincide
  on R with opposite orientation) become interior after gluing
  and are REMOVED; the boundary of R is seamed: mate edges minted
  once, shared by the surviving faces of both operands.
- Non-contact geometry passes through unchanged; faces of A and B
  that partially overlap R are split (existing splitting
  machinery) so removal is exact patch removal, never numeric
  trimming.
- Partial-face REST contacts (contact region strictly inside a
  face) are in scope if the splitting machinery yields them
  naturally; if a sub-case requires new face-region machinery,
  refuse typed with a named sub-frontier and report it — do NOT
  build speculative region algebra for this side unit.
- Result must pass the full gate ladder (Euler inventory, tier 2,
  tier 3, tier 3′ where declared) with **exact dyadic volume
  additivity** on the fixtures (vol(A∪B) = vol(A)+vol(B) exactly
  in f64 for the dyadic fixtures — interiors are disjoint).

## 3. Naming and honesty

- Minted seam edges/vertices get the standard N-machinery
  treatment (mint-order identity; naming keys per #95's
  disposition); parallel-vs-sequential mint identity stays
  bitwise.
- Any group the lane declines (sub-frontier refusals above) rides
  an honest skip/refusal record like the `SkippedMerge`
  precedent — never a laundered catch-all.
- New/updated messages follow the ratified two-tolerance shape
  and compose `COINCIDENCE_RECOURSE` where the situation is a
  coincidence one.

## 4. The tripwire flip (the unit's acceptance headline)

`crosslap_rest.rs::tripwire_declared_crosslap_rest_union` FIRES
by design when this lands. Follow its embedded instructions:
- The declared mated cross-lap union BUILDS: exact volume
  2·(BEAM_VOL − NOTCH_VOL), watertight, gate-ladder green.
- Upgrade the demos/tour `crosslap` stop to ship the glued union
  (exact volume + watertight STL/STEP at the stop), then retire
  the wire — replace it with certified-pass pins at both doors
  (undeclared still refuses; declared builds with the exact
  volume) and keep the history note.
- Revisit `m3_pr6_tier3prime`'s declared corner-flush REST pin —
  same frontier; flip it to a passing pin if the lane covers it,
  or document precisely why it remains refused (sub-frontier).

## 5. Acceptance

- Crosslap fixture: declared union builds (exact volume,
  watertight STL row, STEP row); undeclared refuses unchanged.
- Corner-flush REST pair (the tier-3′ fixture's shape): declared
  outcome per §4.
- A stacked-plates fixture (full-face REST contact): declared
  union = single body, contact faces gone, exact volume; the
  three-plate chain (two declared contacts) also builds.
- Contradiction row: a false REST declaration refuses
  `DeclarationContradicted` at the lane, not a silent no-op,
  when it meets geometry.
- Trilean/escalation rows for any new named predicate (K-funnel
  registered, k-lint clean); if the lane needs NO new numeric
  predicate (pure structural/declared routing), state that
  explicitly in the report.
- Persistence round-trip of a glued body; naming-key stability
  row (re-run bit-identical).
- Local checks per the narrowed-battery process: -p topo (both
  lanes), the demos/tour crosslap stop, fmt, clippy -p topo.
  Hosted CI gates the matrix.

## 6. Out of scope

Curved REST contacts (PR 9); subtract/intersect lanes; any
value-inferred coincidence (the ladder is law); face-region
algebra beyond what splitting already yields; census changes.

## 7. Process

Standard: foreground rows, one per Bash call (poll with
`pgrep -x cargo` if ever needed — never a self-matching
`pgrep -f`); push per unit; adversarial e2e review + fix pass;
PR by orchestrator. OUTPUT DISCIPLINE per standing header.
