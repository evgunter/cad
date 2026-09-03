# MATE-5 — issue 943's curved residue: the certified-ε overlap enclosure, cylinder-first

**Binding at dispatch** (S-MATE program, `docs/S-MATE-PLAN.md`;
difficulty pre-logged in the plan's opening commit: **L**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
The primary specification is the RULING CHAIN: the Q3 ruling
(`docs/S-MATE-PLAN.md` §Rulings item 3 — build now, cylinder-first)
executing the sanctioned closing shape recorded at
`docs/CENSUS-REST-CLOSURE-DESIGN.md` (Q2's answer + latitude note 2:
for conformal contact a CERTIFIED everywhere-within-ε approximation
is acceptable; Door 1 — the carrier ladder — stays exact/certified).

## Situation

Cross-instance CURVED declared `Rest` — a cylinder seated in a bore
across two instances, the ordinary mated shaft-in-bore assembly —
dead-ends at the census's Door 2: two instances' descriptions carry
genuinely divergent chart parameters (`u_ref`, seams), `same_chart`
falls through to the source match, and the pair refuses
`ChartDivergence` → `CensusUnsupported{Face}` → `Declined` → the
`Uncertified` frontier. #1063 closed this for PLANAR pairs with the
world-carrier arm; the curved arm was its NAMED RESIDUE with this
unit's shape recorded as sanctioned.

## Deliverables

1. **The cylinder arm of Door 2**: for a declared cylinder×cylinder
   pair whose Door-1 verdict certifies one carrier, decide trim
   overlap on that shared carrier by a CERTIFIED
   everywhere-within-ε enclosure — enclose both trims' images
   (folding the angular coordinate across the two descriptions'
   `u_ref` offset and seams), and answer three-outcome: overlap
   DEFINITELY POSITIVE at ε (certify), definitely empty (the
   refuted/stale arm — see 4), or an honest DECLINE stating its
   cause. Door 1 is untouched and stays exact.
2. **The frame-invariance obligation** (#1063's lemma pattern): the
   answer must not depend on which description is the
   representative — the lemma WRITTEN and PINNED by a row that runs
   the pair both ways.
3. **The two inherited caveats, consumed not repaired**:
   - **issue 1191** (period-fold widening, S-CERT's): the angular
     fold is exactly its territory — cite it where the fold enters;
     conservative widening is HONEST here (it widens toward
     decline/escalation, never toward certifying a false overlap);
     do not repair the fold in this unit.
   - **issue 1435** (the fixed-schedule incompleteness in
     `interior_witness`): do NOT inherit a fixed candidate schedule
     blindly. The cylinder arm's decline posture is its OWN and
     must say when it can decline on decidable geometry; any
     schedule or subdivision budget it uses is disclosed with its
     incompleteness stated and a 1435 cross-reference.
4. **The consequence wiring** (the #1063 precedent): when Door 2
   starts answering for these pairs, a cylinder declaration the
   geometry refutes (definitely-empty overlap) lands the
   `StaleContactDeclaration → Refuted` arm naming its mate — its
   own acceptance row.
5. **Red-first from the class's own shape**: a two-instance mated
   cylinder-in-bore assembly — quote today's refusal chain from
   main; after, the seat CERTIFIES (or, if the honest enclosure
   cannot decide it, the measured decline is stated and the row
   pins the decline's cause — do not tune the fixture to force
   certification).
6. **Kind honesty**: sphere/cone/torus cross-instance pairs stay
   refused exactly as today, with the residue restated PER KIND at
   the refusal site (the closing shape stays recorded for each).
7. **Metering**: new certified-numeric decisions are metered NUMERIC
   rows per the house convention; every new row ε-three-outcome
   honest.
8. **The C3/C4 sentence**: CONTACT-DESIGN's rung-3 invariant gained
   the planar world-carrier arm at #1063; extend the same passage
   with the certified-ε cylinder arm (one addition in the ratified
   pattern — state the arm, not history).
9. **Class sweep** (discipline §5): the genus is "a Door-2 consumer
   assuming chart-space exactness or planar-only overlap answers" —
   sweep census.rs/chart_region.rs consumers; hit list with
   dispositions, blind spots stated.

## Acceptance

- Red-first demonstrated; the frame-invariance row green both ways;
  the Refuted-arm row green; the per-kind refusals pinned; existing
  census/chart suites green (the #969/#1063/MATE-4a probe suites
  are the oracles — they must stay green untouched).
- Refusal-reach moves classified against the D2 addendum in the PR
  body (row 2 expected: the cross-instance cylinder class's
  over-refusal retires or narrows honestly).
- ε posture (issue-1356): this unit is genuinely band-sensitive —
  pin the interval lane with a `CI-Config:` trailer and argue the
  band story explicitly (what each band does to the enclosure's
  three outcomes).

## Hard rules

- NO `Co-Authored-By` trailer and no model names in lane commits.
- Keyword hygiene: write "issue 943" spelled out; never a closing
  keyword before a `#`-reference. The orchestrator decides issue
  943's closure at merge (the planar gaps landed long ago; this is
  its last named residue for the cylinder kind).
- Scope fence: `crates/topo/src/chart_region.rs`,
  `crates/topo/src/census.rs` (the Door-2 consumption only),
  `crates/topo/tests/`, and the ONE `docs/CONTACT-DESIGN.md`
  passage (deliverable 8). Nothing else — no `boolean/`, no
  `solid_contain.rs`, no `geom-brep`, no `editor-core`, no interval
  scalar internals (consume, never modify — issue 1191's ground is
  S-CERT's), no `docs/MODEL-AB-LOG.md`, no `docs/S-MATE-*.md`.
- Sibling lanes run concurrently; work only inside your worktree
  (the shared session scratchpad is off-limits); merge main before
  opening the PR and whenever it moves.
- Commit and push after every coherent unit of work (branch
  `mate/5-curved-eps-rung`).
