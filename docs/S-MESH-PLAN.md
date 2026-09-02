# S-MESH — mesh honesty and budget (plan)

**STATUS: DRAFT (design conversation for the Rulings sought section;
the ruling-independent units below are dispatchable pre-ratification).**
Opened on Evan's direction (in-chat, 2026-08-31: "pick up S-MESH as the
orchestrator", with S-BOOL taken by the same instruction —
`docs/S-BOOL-PLAN.md` opens in the same PR) from the ratified stream
cut in `docs/WORK-STREAMS-2026-08.md` (§S-MESH). The cut is the
charter and is cited, not re-litigated.

Branch prefix (the #396 convention): **`mesh/`** — unit branches
`mesh/<unit>-<slug>`, orchestrator branch `mesh/orchestrator`. (The
remote's dormant `mesh/*` branches are the pre-program #284-era work,
not this program's; no legacy registry, per #396.) Away-channel tag
`(S-MESH orchestrator)`. A/B ordinal band **S-MESH = 1200–1299**,
claimed in `docs/MODEL-AB-LOG.md`'s banding entry in this same PR, per
that entry's rule; implementer blocks are named `MESH-B1, MESH-B2, …`
(unit names occupy `MESH-<n>`). Live state is `docs/S-MESH-LOG.md`'s
tail, never this file.

## Charter (from the cut, verbatim in substance)

`crates/mesh` has no live program and a coherent defect list:

- **Watertightness/guards**: #897 (S65's two uncovered cases — S65
  itself is Evan's), #896 (undeclared-pole misclassification), #868
  (typed warning channel).
- **Sizing intent vs budget**: #685 (`nv` ignored), #320 (NURBS wall
  budget, per `memories/tessellation-budget.md`), #950, #555.
- **Structure**: #881 (ε as a bare f64), #726/#727 (iso-rectangle
  ownership — design input), #782 (red tessellation pin).
- **SMELL track R's mesh rows**; R's geom-brep remainder is deferred
  (see Substrate: the live constraint is S-CERT's slate, not PCURVE).

Inherited at opening (both from S-CERT, by name): **#1362** (the
`walk.rs:1067` world-origin loop-area anchor) and **the
`closing_column` debug_assert note** (CERT-1's out-of-scope report on
#723 — folded into #868/#897's ground below).

## Ratified ground (cited, not re-litigated)

- The stream cut and its keep-outs.
- **D9**: mesh structure is a function of (body, δ) alone;
  bit-identical replay. The crate's own qualifier
  (`mesh/src/sizing.rs:100–134`): the ε-independence claim is "a
  statement about the tree, not a theorem" — #896 and #881 are the two
  halves of making it enforceable, and no unit here may weaken it.
- **D2 addendum** (`docs/DESIGN.md`): every guard this program mints is
  classified by row (row 0 unrepresentable; rows 1/3 input-quality;
  row 2 capability boundary; row 5 boundary debug_asserts).
- **S65 is Evan's decision, not a row** (§D of the SMELL scan): the
  three-way table and its measured prices live at S65; #897 supplies
  the two missing costings; when it is ruled, the implementation is
  this program's.
- `memories/tessellation-budget.md`: budget questions are measured with
  the committed instrument, never estimated; the gate compares
  differences, and a re-baseline states why.
- #881's port constraint is the issue's own binding non-scope (no mesh
  byte moves), taken as a gate, not as an output-stability argument for
  keeping code (`memories/output-stability-as-justification.md`).

## Substrate facts the slate is shaped by (surveyed 2026-08-31)

- **#320 and #782 are resolved on main and want closing.** #320's both
  halves landed (TESS-SPAN #594: leaf_a 261,780 → 84,524; TESS-SPLIT
  #951: → 43,798, with #950 the scheduled residual — the A/B log rows
  say "#320 IS CLOSED ON BOTH HALVES"). #782's table was re-pinned
  green by TESS-SPLIT (468/414 with the snap exonerated by experiment)
  and the arming it blocked landed (`ci.yml`'s "demos tour suite" row
  runs the whole suite); `docs/VERBS-PLAN.md` already records "the
  issue wants closing". Closing both is orchestrator-direct
  bookkeeping, after verifying the pins at HEAD.
- **#881 is half-landed.** #894 shipped the witness/confinement half;
  the reopen comment scopes what remains — the named-operations half
  over four terminal reads (now `walk.rs:601/872/1053`,
  `trimmed.rs:593`) plus `Tol::eps()` / `SizingTols::eps` handing out
  bare f64s.
- **`walk.rs` is contended** by #1362, #896, #881 and #868. These
  units are sequenced, never fanned out.
- **#555's pin went live**: the Klein wall-7 `Triangulation` pin is CI-
  gated since #782's arming, and `mitigate_underflow` appears nowhere
  in `crates/` — the fix is genuinely unapplied while an ordinary
  annular revolve cap refuses at every δ.
- **Track R table corrections ride this PR** (per §D rule 2, the table
  is edited in the PR that moves it): the item count re-derived (D304
  arrived from Track T's `T-c` via the S-BLEND exit handoff with no
  count bump); C3/D30's "NOT TAKEABLE until #723 is fixed" gate is
  discharged (CERT-1 merged 2026-08-29, closing #723/#893, with C-m's
  recorded questions answered in CERT-1/CERT-5's PR bodies); D302's
  mesh member landed (`Display for TessellateError` at
  `mesh/src/types.rs:271`, commit `03368cfd`) so the row leaves, its
  consumer-side remainder being #1111's (LIB's charter, per
  `viewer/src/scene.rs`'s own citation) and Track U's `D47` rule now
  unblocked for this type; S112's member (e) pointer to the landed
  `D282` deleted (the S-BOOL side of the same sweep).
- **S-CERT is live and stays ahead on shared ground.** Its remaining
  slate (CERT-6 in flight, then CERT-8, CERT-10, CERT-M/CERT-N) edits
  `props/quad.rs`, `patch_bound.rs` and the area lanes, and CERT-10
  owns a render/tess-budget re-baseline. Consequences taken here:
  **C3 + D30 and C23 are sequenced after CERT-10 lands** (same files);
  **S26 is expected to shrink or land at CERT-6's merge** (#870's
  gauge) and is not touched until then; the tess-budget baseline
  (cut `ecdd9ec7`, 2026-08-29) is not re-cut ahead of CERT-10. The
  CERT-9 "mesh-fence ground with no live claimant" precedent ends at
  this PR — mesh ground now has a claimant and seams go by
  coordination.
- The `demos/tour` lily pins are Track X/G ground: a mesh unit that
  moves counts re-pins in its own PR under the render-lane and
  budget-gate conventions (what moved and why, stated).

## The slate

Ordered; each unit gets its own binding spec at dispatch; difficulty
logged pre-draw per the protocol.

- **MESH-1 — #1362, the walk.rs anchor-class fold (S; dispatchable
  pre-ratification — an inherited defect with the fix shape #303's
  merged unit already established).** `walk.rs:1067`'s `band_u`
  loop-area vector re-anchored off the world origin (the consumer is
  direction-only — `atan2` picks the azimuth branch — so the fix is a
  local anchor, #303's move); a red-first row at a large placement
  pinning the direction honestly. The issue's secondary sweep (the
  copy-source template sites: `sweep/tests/revolve_common`'s
  `signed_volume`/`signed_volume_lifted`, `pncad-py`'s
  `mesh_signed_volume`, the `docs/guide/meshing.md` snippet) rides
  along as the issue routes it, with a coordination note to LIB on the
  guide page (#1198 is live there).
- **MESH-2 — #555, sub-floor engineered zeros (S/M).** The projection
  site is the issue's own preferred home ("this coordinate is an
  engineered zero" is known there), spelled either via spade's
  `mitigate_underflow` or a targeted snap — the unit decides by
  measurement and defends the choice against `planar.rs`'s
  no-value-snapping doctrine in that module's own prose (the issue's
  argument: a structural zero snapped to zero is not value snapping).
  Red-first on the Klein inner-tube rim repro including the parameter
  lottery (the flare-angle × rim-radius sweep). The wall-7 pin's
  retire instruction is honored: the entry's narrative updates to
  "banked case closed", never a silent delete.
- **MESH-3 — #896, the undeclared-pole guard (M).** `pole_v`'s
  classification gets the guard the issue names — no non-pole junction
  within ε of a chart pole it is not being identified with — as a D2
  row-5 debug_assert beside #895's declared-vertex guard. The hard
  half is the fixture: no in-tree body reaches it and the issue names
  STEP import as the plausible route (`mesh` has no dev-dependency on
  `step-import`, so the row likely lives in `step-import/tests` or a
  cross-crate suite — the unit decides and records where, and why).
  Sequenced before MESH-4 because it mints the fifth ε consumer.
- **MESH-4 — #881's remaining half, named ε operations (M).** The
  reopen comment's scope verbatim: `Eps` operations named
  (`separates`, `coincident`, `dominates`, `pad`) over the terminal
  reads, `Tol::eps()`/`SizingTols::eps` no longer handing out bare
  f64s, the inventory becoming the methods. Binding gate from the
  issue's non-scope: **no mesh byte moves** — the unit ships a
  byte-identity pin over the tour corpus as its own gate. The
  comment's open question (ops on `Tol` itself vs a mesh-local
  newtype) is decided in the spec with #741's configuration surface
  read first (LIB holds #741's plan; coordinate, do not implement
  their half).
- **MESH-5 — #685, the `nu == 1` sizing-intent decision (S/M).**
  Either the v-schedule is honoured when `nu == 1` (rows emitted) or
  the code says why one triangle is right for a ruled patch and stops
  computing a schedule it discards — the issue holds both defensible;
  the unit decides by measurement (the π/6 cone wedge δ-sweep, budget
  instrument in hand) and writes the decision at the site. Explicitly
  not conflated with #678's `nu == 2` pole floor. Retires that S29
  instance; if the unit finds the answer belongs to the sizing-policy
  conversation instead, it stops and reports rather than patching.
- **MESH-6 — #897, the two uncovered S65 cases (S; discretionary
  under the Q1 ruling).** Measure the cost of covering the full-2π
  seam and cross-face identification; verify or refute
  `pole_columns`' own MAX_ANGULAR_STEP argument as a floor for the
  seam case; add the cases as debug-profile-only guards if cheap,
  else record the verdict at the site and close #897. No shipped
  guard either way (S65 ruled: stays compiled out).
- **MESH-7 — #727 then #726, iso-rectangle door ownership (M/L;
  design first — Q3 below, then the C11 fold-in).** After the ruling:
  fold `mesh::curved`'s SHAPE question onto
  `props::require_rims_at_extremes`, keeping the walk-consistency
  spatial check as mesh's own question, deciding what `mesh` does
  with a face whose rims it never sees. Do-not-lose (the issue's own
  bold): the `walk::iso_side_starts` qualification either survives
  the fold or is closed by it. #723's extent gap, which #726 warned
  the fold would import, is closed (CERT-1).
- **MESH-8 — #868, the coherence-detector relocation (M/L; under
  the Q2 ruling).** Delete the three mesh-side detectors; land the
  body-side coherence examination with a non-gating findings report
  per the ruling; the door decision (tier-adjacent examine vs
  step-import diagnostics) is the unit's first recorded question,
  answered by the dependency graph and the finding's audience. The
  issue's non-scope still binds: no condition becomes a refusal as
  a drive-by. Sequenced after MESH-3 (which may add a fourth
  detector-adjacent guard to the same file).
- **MESH-9 — #950, parked with a trigger.** The issue's own words:
  neither fix is needed until a body presents the configuration, and
  the failure is a typed `CertificateExceeded` refusal that names it
  loudly. The two candidate fixes (rim chords snap up to a multiple
  of `patch_nuc`; rim-adjacent band raised to the rim's chord count)
  stay recorded in the issue; built on first demand.
- **MESH-10 — issue 1562, the torus extent from a split seam (S; filed
  from MESH-7's D9 finding).** `props::curved::torus_ends` reads the
  face's v-extent from the FIRST meridian's stored span, so a meridian
  carried by two edges (a split seam) refuses `NotIsoRectangle
  props_rim_level` on a genuine chart rectangle — at the door and at
  `mass_properties` alike. Fold consecutive meridian arcs on one
  carrier before reading the extent (or take it from the rims, the
  linear kinds' move now that CERT-1 fixed the sphere's); MESH-7's
  reverse red-first row flips and the issue-653 sweep returns to
  (254, 4). `props/curved.rs` is Track R ground; closed-form extent,
  so it rides its own unit rather than a fix pass.
- **MESH-11 — issue 1571, the walk's arc premise (M; filed from
  MESH-7's review).** The iso-curve premise is still inherited: props
  certifies each edge's CARRIER, not that the traversed ARC stays on
  one chart meridian, so a pole-crossing great-circle arc passes the
  shape door and the walk meshes it non-watertight with debug
  assertions off. Either the walk verifies the arc premise (monotone,
  pole-free parameter span, decided at the band, refusing typed) or
  the door does (props certifying arc span — which also serves
  `mass_properties`, whose L-shaped complement returns volume 0.0 on
  a closed sphere, the same probe's props-side finding). A row per
  direction; `walk.rs:803`'s ledger already names the two π-rad
  witnesses. After MESH-8; the door/walk split is the unit's first
  recorded question.
- **MESH-R — the remaining Track R rows as track lanes** after the
  defect cluster clears, sequenced by the track's own table: S28,
  S236 (its `tools/` half is Track K's row, per the cell), S237,
  D300, D303, D304; C23 with its premise check first (the two
  schedules may not be one) and its one-line `geom/src` exception as
  written; C3 + D30 and S26 on the S-CERT sequencing stated above.
  Rows land per §D's conventions (delete the row in the landing PR).

Cross-program interfaces, named: S65 and D283-class questions are
Evan's; `props/quad.rs`, `patch_bound.rs`, the area lanes and the
tess-budget re-baseline are S-CERT's until its slate closes;
`demos/tour` pins are Track X/G conventions; the guide page is LIB's;
S-BOOL's #542 unit edits `props/curved.rs` (R fence ground) by this
program's leave — same orchestrator, seam recorded in both plans.

## Rulings sought (Evan)

1. **Q1 — RULED (Evan, in-chat, 2026-09-01)**: S65 stays compiled
   out — no unconditional shipped guard; there is no record of the
   backstop itself catching anything outside dev. Coverage for the
   two uncovered cases (full-2π seam, cross-face identification) is
   discretionary: MESH-6 reshapes from a ruling-feeder into a small
   unit that measures their cost, adds them as debug-profile-only
   guards if cheap, and otherwise records the verdict and closes
   #897.
2. **Q2 — RULED (Evan, in-chat, 2026-09-01): option (d),
   relocation.** The three detectors measure body-data coherence
   (carrier-vs-vertex gaps computable from the body alone — no mesh
   state, no δ dependence), so the conditions move to the body's own
   examination lane and the mesh-side debug_asserts are DELETED —
   the tessellator stops being a lint for other people's data and
   `tessellate`'s signature never changes. MESH-8 reshapes to the
   relocation unit: one body-side coherence examination with a
   NON-GATING findings report (bodies that mesh today keep meshing;
   nothing panics), the unit's first job being to confirm which door
   carries it (tier-adjacent examine vs step-import diagnostics) and
   its second to show each relocated condition firing on the same
   witness the mesh assert would have caught.
   **mesh-local**, a typed `MeshWarning` (all three detectors measure
   the same thing — a gap against a lever arm, in meters, against ε,
   so the payload is a struct, not a string), returned beside the
   mesh in `tessellate`'s Ok value. Warnings are deterministic in
   (body, δ, ε) but are **not mesh bytes** — D9's byte-identity
   contract is stated to cover the mesh and not the diagnostics.
   Honest counterarguments: a kernel-wide channel would dominate
   other lanes' debug_asserts too (the file's own observation), and
   changing `tessellate`'s Ok type touches every caller — the
   alternative is a sidecar field on `Mesh`, which keeps signatures
   but muddies "the mesh is the output". Mesh-local now does not
   preclude hoisting the type later if a second crate demands it.
3. **Q3 — RULED (Evan, in-chat, 2026-09-01): explicit doors.**
   No consumer keeps a transitive floor — each door that needs the
   iso-rectangle premise cites `props_rim_level` itself. MESH-7 is
   unblocked and implements the mesh side on this ruling. The
   original recommendation, kept as the ruling's record: **no consumer
   keeps a transitive floor** — each door that needs the
   iso-rectangle premise cites `props_rim_level` itself (the S58
   single-home predicate), so when the certified-quadrature lane
   learns notched domains, each door's line changes visibly instead
   of a floor silently vanishing. `mass_properties` stays a mass-
   properties door, not a de-facto gate. Honest counterargument: more
   citation sites to keep honest; but they cite one predicate rather
   than re-deriving it, which is exactly the fragmentation S58
   closed. MESH-7 implements the mesh side on this ruling.

## Process

Standard, v6: substrate → binding spec → one implementer + the
cross-model dual review + union fix pass; implementer arms drawn per
the current block rule in `docs/MODEL-AB-LOG.md` (read on main at each
dispatch — that document owns every live number); ordinals claimed on
main at review dispatch from band 1200–1299; record-at-merge with
per-phase tokens/wall-clock; blinding discipline verbatim (no
`Co-Authored-By` in lane commits; no arm-naming surface reviewers can
read). Hosted CI is the only gate; every new row ε-three-outcome
honest; **the #1356 ε-trailer practice is adopted from the first
dispatch** — a band-sensitive unit's spec says so and pins ε for its
gate runs. Implementer dispatches point at
`docs/prompts/implementer-discipline.md` by path; reviewers get
explicit claims to falsify plus `docs/prompts/reviewer-style-lane.md`.

**This orchestrator runs in a remote container** (the
S-CERT/S-QA/M10/GUI precedent): no persistent
`~/.local/share/cad-work`, no script monitors (PR watching via MCP
subscriptions + scheduled self check-ins; away-channel etiquette by
hand under the `(S-MESH orchestrator)` tag), GitHub through MCP. Disk
(~28 G free) is the binding constraint: lanes are worktrees sharing
one object store, each with its own `CARGO_TARGET_DIR`, ≤ ~2
concurrent lane targets, review targets reclaimed the moment the
report is in hand. The build-slot mutex, CONFLICTING-means-silent-CI,
and push-early rules bind unchanged. The clone was unshallowed with a
blob filter at opening. This orchestrator also runs S-BOOL
(`docs/S-BOOL-PLAN.md`); the two programs share the container's lane
budget, so their dispatch cadences are interleaved, never doubled.

## Exit shape (proposed)

No mesh guard fires where nobody looks and none lies about its ε: the
walk anchors are placement-honest (#1362), the sub-floor engineered
zeros mesh (#555 — the Klein wall-7 entry narrates a closed case),
pole classification is guarded against undeclared poles (#896), ε's
consumers are the methods on a type (#881 closed on both halves), the
`nu == 1` schedule says what it does (#685), S65 is ruled with all
three prices measured and the ruling implemented (#897 → Q1), the
iso-rectangle premise has one home and every door cites it (#727/#726
under Q3), input-quality detectors speak through the ratified warning
channel (#868 under Q2), #950 stands parked with its typed trigger,
and Track R is empty in §D. Every unit merged on its own green hosted
head; the walk convention applies at exit.
