# S-BOOL — boolean reach and containment (plan)

**STATUS: DRAFT (design conversation for the Rulings sought section;
the ruling-independent units below are dispatchable pre-ratification).**
Opened on Evan's direction (in-chat, 2026-08-31: "you can also take
S-BOOL if that's not claimed yet" — it was not; verified against docs,
branches and open PRs at opening) from the ratified stream cut in
`docs/WORK-STREAMS-2026-08.md` (§S-BOOL), by the S-MESH orchestrator
(`docs/S-MESH-PLAN.md` opens in the same PR). The cut is the charter
and is cited, not re-litigated.

Branch prefix (the #396 convention): **`bool/`** — unit branches
`bool/<unit>-<slug>`, orchestrator branch `bool/orchestrator`.
Away-channel tag `(S-BOOL orchestrator)` (one orchestrator, two tags —
the tag names the program a comment speaks for). A/B ordinal band
**S-BOOL = 1100–1199**, claimed in `docs/MODEL-AB-LOG.md`'s banding
entry in this same PR; implementer blocks are named `BOOL-B1, …`
(unit names occupy `BOOL-<n>`). Live state is `docs/S-BOOL-LOG.md`'s
tail, never this file.

## Charter (from the cut, verbatim in substance)

Operand gates and containment doors that refuse (or mis-admit) legal
inputs — `topo/boolean`, `splitting`, containment; not pcurves, and
not the germ-arm lanes VERBS holds (its Wave-4 claims: #347's
remaining half, #1031 half B, #1076, #1077; #1059 resolved into
#1031's chain).

- **Gates**: #1011 (`point_in_solid` missing ray arms — the named
  cost of VERBS-GATE's pair-scoping), #1152.
- **Containment/props**: #750 (extent-box coarse), #542, #368, #433
  (needs a disposition), #134.
- **SMELL track Q's topo rows**; Q's `geom-brep/ssi*` files stay
  untouched until PCURVE's P-2 (#1177) resumes and lands.

The fence is confirmed from the other side:
`docs/VERBS-PLAN.md`'s work-stream note names
"#1011/#750/#542/#368/#433/#1152/#134" as "S-BOOL's honest remainder
(… never VERBS')".

## Ratified ground (cited, not re-litigated)

- The stream cut and its keep-outs; VERBS' Wave-4 claims as listed.
- **D2 addendum**: a refusal of valid input the lane could serve is
  row 2 — #1011's `KindUnsupported` arms are exactly that class, and
  every refusal this program retires or mints is classified there.
- **#750's two recorded falsifications bind**: no C6 gate-skip
  (re-creates the class #737 removed), and no separating plane derived
  from the container's own face planes (falsified on non-convex
  containers, with the L-bracket counterexample in the issue). A
  BOOL-4 spec carries both verbatim.
- **The grazing posture**: tangential/grazing ray crossings escalate
  rather than answer (the sphere closed-group discipline #1011's issue
  cites); margined trilean predicates per D4/Q1.
- **S-MATE consumes, not decides**: #750's fix has a named downstream
  consumer in the unopened S-MATE stream; the handoff is recorded at
  landing, nothing is co-designed ahead of it.

## Substrate facts the slate is shaped by (surveyed 2026-08-31)

- All seven charter issues are open with zero comments — no claim or
  disposition traffic anywhere; the substrate lives in the docs and
  pins.
- **#1152's reproduction is already committed**: the P-1b probe landed
  `#[ignore]`d on main (`sweep/tests/p1b_r1_probes.rs`, with "Un-ignore
  it when #1152 lands" at the site), the defect reproduces
  byte-identically on main, and the suspected site is
  `splitting/finish.rs`'s `describe_section_boundary` — an empty
  smooth arm, the same defect genus P-1b just fixed in `extrude`. The
  battery missed it because the committed coplanar row asserts tier 2
  only.
- **#1011 splits naturally in two**: the cone arm (quadratic + nappe
  test + axial trim window) and the torus arm (quartic with a
  certified root-count posture; two chart-trim windows). The two
  pinned red-on-landing probes in `sweep/tests/verbs_gate_r1_probes.rs`
  are both torus-shaped, so they flip in the torus unit's PR — that
  file is VERBS-authored, so the flip is coordinated on the
  away channel.
- **VERBS is hot on adjacent ground** (GERMARMS PR-2 dual in flight,
  SPHSPH staged): its ground is `geom-brep/src/intersect.rs` and the
  germ substrate; #1011's arms live in
  `topo/src/boolean/solid_contain.rs`, which VERBS is not editing.
  Units here re-merge main frequently anyway.
- **#542 edits `geom-brep/src/props/curved.rs`** — SMELL track R fence
  ground (S-MESH's) and a file CERT-1 just rewrote. Taken by this
  program per the cut, with the seam recorded in both plans (same
  orchestrator); the unit merges main frequently.
- **Track Q corrections riding this PR**: S112's member (e) pointer to
  the landed `D282` deleted (the doc fix is verified at
  `ssi/exhaust.rs:92` — "in the lane's own units"); the Q table itself
  is current (16, re-derived 2026-08-31; D285/D286 already left with
  CERT-2).
- **Q-claim carve-outs**: `D283` is Evan's question, not a row to
  work; `S83` and `D36` sit on the `ssi*`/`pcurve_cache` ground that
  waits for P-2 (#1177, in blinded review); `D36`'s `certify.rs` half
  is R's ground — a real seam, worked by coordination when reached.

## The slate

Ordered; each unit gets its own binding spec at dispatch; difficulty
logged pre-draw per the protocol.

- **BOOL-1 — #1152, the coplanar-split citation defect (S/M;
  dispatchable pre-ratification — a charter-named defect whose
  reproduction is already pinned).** Establish whether the
  `Intersection` citation is stale rather than merely absent; fill or
  correct `describe_section_boundary`'s smooth arm for the
  face-coplanar case; un-ignore the P-1b probe (its own site says to);
  upgrade the `notched_block_end_to_end` coplanar row to assert tier 3
  so the class cannot re-enter silently.
- **BOOL-2 — #1011's cone arm (M/L).** Ray×cone: quadratic + nappe
  test (`(p − apex)·axis` sign) + axial trim window, grazing
  escalates; the containment doors' `KindUnsupported` refusal for
  `Cone` retires as a D2-row-2 capability landing. Its own new rows
  (interior/exterior/near-apex/grazing) rather than borrowed pins.
- **BOOL-3 — #1011's torus arm (L; its own unit, after BOOL-2 sets
  the arm pattern).** Ray×torus quartic with a certified root-count
  posture at run tolerance; two chart-trim windows; tangential and
  grazing escalate. Flips both `verbs_gate_r1_probes` pins in this PR
  (coordinated with VERBS — their authored file, our landing).
- **BOOL-4 — #750, material containment (L; after BOOL-2/3 — check
  the coupling first).** A containment test separating "inside the
  extent box, outside the material" from "inside the material". The
  spec carries the two falsifications verbatim, and its first
  question is whether the test consumes the #1011 ray arms (if yes,
  the sequencing above is load-bearing; deriving ray casting twice
  would be waste). The L-bracket repro from the issue becomes the
  red-first row; the honest `CensusUndecidable` refusal retires for
  pocket/cavity assemblies. Handoff to S-MATE recorded at landing.
  The adjacent observation in the issue (no cross-solid check on
  `declared.faces`) is filed as its own issue at spec time, not
  absorbed silently.
- **BOOL-5 — #542, the rim-free wedge props arm (M).** `sphere()`'s
  spherical-wedge arm integrates over the meridian pair directly (the
  issue's flip condition); the two `m9_d1_r2_probes` sites promote to
  tier 3 and their pointer comments come out. R-fence seam as noted
  above.
- **BOOL-6 — #368, the per-slab stacking fold (M; ruled
  2026-09-01, scheduled on Helix demand).** Replace loft.rs's
  ends-only stacking statement with a per-slab fold, margin = min
  over slabs (the per-slab margins already exist); a consumer audit
  for anything reading the end-to-end value as a feature;
  straddle-driving rows for spines past π; VERBS coordination on
  Helix timing (sweep-ground seam).
- **BOOL-7 — #134, the vdiff shadow-exec rung (M; assigned by
  Evan 2026-09-01, M10 dormant).** When the vdiff engine hits an
  empty pair population on a verdict vanish, shadow-execute exactly
  the vanished pair's predicates from the prior evaluation's
  context and diff those — bounded, diagnosis-time-only, recovering
  the full `PredicateFlip` (option (a), ruled 2026-07-29; recording
  pruned-pair pseudo-verdicts stays ruled out). Ground:
  `editor-core/resolve/vdiff.rs` + immediate callers; M10 keep-outs
  per the Q3 ruling.
- **BOOL-Q — track Q's topo rows as track lanes** after the defect
  cluster clears, sequenced by the track's own table: G9, S173, H11
  (its third door in `geom/src/curves/boxes.rs` is outside the fence
  — filed on N's owner, not edited), S234, D95, D280, D66 (the unit
  decides sentence-vs-row; if the row, `sweep/tests` is Track T's to
  file), D284, D287, D288; the ledger rows D57/D46/D281 (D281 is a
  per-row read, the row's own bold). Carve-outs per Substrate: D283
  (Evan's), S83/D36 (wait on P-2). Rows land per §D's conventions.

Cross-program interfaces, named: germ arms, SPHSPH/CYLSPH, #1031 half
B, #1076/#1077 are VERBS'; `ssi*`/`pcurve_cache` is Q-ground behind
P-2; `props/curved.rs` is R's fence (seam recorded both sides); #134's
ground (`editor-core/resolve/vdiff.rs`) is M10-adjacent and outside
this program's files — see Q3 below.

## Rulings sought (Evan)

1. **Q1 — #433's disposition (OPEN; geometry clarified in-chat
   2026-09-01).** The concrete disagreement: a loop whose two
   consecutive straight segments lie on one carrier line (an
   intermediate vertex splitting a straight run) is refused by the
   constructive lattice as `SameCarrierJunction` ("carrier identity
   is not tangency — extend the leg") and accepted by `validate` as
   raw data. Stance (c) — intentional, stated at both sites — has
   real texture: the constructive door refusing to MINT what the
   data door must ACCEPT (imports exist) is coherent. Three stances are in
   the issue (loosen the junction check / tighten validate / rule the
   disagreement intentional and state it at both sites). Evan's
   ProfileLoop-seals ruling already shrank it to a kernel-internal
   consistency question, and a proposal is recorded riding PR #576's
   body — the conversation opens by putting that proposal (or its
   correction) up for the ruling rather than re-deriving one. Not
   implemented until ruled.
2. **Q2 — RULED (Evan, in-chat, 2026-09-01): decide now — Helix
   is coming.** The shape: a per-slab stacking fold with margin =
   min over slabs, replacing the ends-only statement whose wall is
   exactly π. Scheduled as **BOOL-6** (added to the slate): the
   fold, a consumer audit for anything reading the end-to-end value
   as a feature, straddle-driving rows, and VERBS coordination on
   Helix timing (`loft.rs` is sweep ground — seam recorded).
3. **Q3 — RULED (Evan, in-chat, 2026-09-01): S-BOOL takes it — M10
   is dormant.** Scheduled as **BOOL-7** (slate): the vdiff
   shadow-exec rung under Evan's standing option-(a) ruling
   (2026-07-29), pure implementation. Keep-outs stay sharp:
   `editor-core/resolve/vdiff.rs` and its immediate callers only;
   M10's Dual arms in `product.rs` and the `AtRestPolicy` seam are
   untouched, and the unit stops and reports if the work reaches
   either. The original routing recommendation, kept as the
   record: it
   is pre-ruled, unpressured, and lives in
   `editor-core/resolve/vdiff.rs`, which is nobody's ground here
   (M10's slate is the nearest live claimant and V's fence is the
   nearest track); parking it back on the banked list with a pointer
   costs nothing. Honest counterargument: it is implementation-ready
   with the approach already ruled (option (a), shadow-exec), so it
   could serve as a filler unit — but a stream editing a crate its
   charter never names is how fences erode.

## Process

As S-MESH's, verbatim in substance (`docs/S-MESH-PLAN.md` §Process):
v6 duals, blocks `BOOL-B<n>`, ordinals claimed on main at review
dispatch from band 1100–1199, blinding, record-at-merge, hosted CI as
the only gate, the #1356 ε-trailer practice, discipline/reviewer
prompts by path. Same remote container, same shared lane budget — the
two programs' dispatches interleave, never double.

## Exit shape (proposed)

The gate/door asymmetry the charter names is gone: `point_in_solid`
answers cone and torus with the grazing posture escalating (#1011,
both VERBS pins flipped), coplanar splits carry adjacent citations
(#1152, probe un-ignored), material containment separates box from
material and the pocket/cavity refusal retires with S-MATE consuming
the record (#750), the rim-free wedge computes its volume (#542),
#433 and #368 are ruled and their rulings implemented or recorded at
both sites, #134 is routed per its ruling, and track Q's topo rows
are empty in §D with the carve-outs discharged or explicitly ceded
(S83/D36 to whoever holds `ssi*` when P-2 lands; D283 ruled). Every
unit merged on its own green hosted head; the walk convention applies
at exit.
