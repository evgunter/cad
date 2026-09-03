# S-MATE — contacts, rest, and assembly composition (plan)

**STATUS: DRAFT (design conversation; Q1, Q2(a) and Q4 RULED
in-chat 2026-08-31 and recorded in §Rulings; Q2(b) directed; Q3's
scheduling half open. Ruling-independent units are dispatchable
pre-ratification).** Opened on Evan's direction (in-chat, 2026-08-31:
"can you pick up S-MATE as the orchestrator?") from the ratified
stream cut in `docs/WORK-STREAMS-2026-08.md` (§S-MATE) — verified at
opening as the cut's last unclaimed stream (S-CERT/S-QA graduated in
the doc itself; S-BLEND graduated and CLOSED at its ratified exit
walk; S-MESH/S-BOOL opened 2026-08-31, PR #1373). The cut is the
charter and is cited, not re-litigated.

Branch prefix (the #396 convention): **`mate/`** — unit branches
`mate/<unit>-<slug>`, orchestrator branch `mate/orchestrator` (this
opening rides the harness-assigned session branch; program branches
follow the prefix from the first unit). Away-channel tag
`(S-MATE orchestrator)`. A/B ordinal band **S-MATE = 1300–1399**
(opened claiming 1200; renumbered per the main-is-authority
tiebreak when S-MESH's 1200–1299 reached main first),
claimed in `docs/MODEL-AB-LOG.md`'s banding entry in this same PR;
implementer blocks are named `MATE-B1, …` (unit names occupy
`MATE-<n>`). Live state is `work/mate/log.md`'s tail, never this
file.

## Charter (from the cut, verbatim in substance)

ASM is closed at v1; its exit-walk residue plus the declared-contact
gaps form one topic with no live claimant. Ruling-heavy: the stream
opens as design conversations, then implements.

- Assembly composition: #945 (mates × patterns), #946 (sub-assembly
  mate loss at the instantiation seam), #944 (mint alignment frames —
  LIB holds the plan; taken only with LIB's hand-off).
- Rest reach: #943's residue (face-level Rest closure — see Ratified
  ground), #1032 (cylindrical-only Rest), #973 (the two remaining
  boolean-lane-premise sites in the at-rest census), #968 (torus
  declared-Rest — ceded by VERBS with the #966/#968 record and
  LILYWELD's measured cone×torus adjacency; pickup governed by its
  ruling's conditions).
- Kernel side: #941 (declared cusps, ruled at #131) — items 1–2;
  #750 is S-BOOL's to land (its BOOL-4) and this stream's to consume.

Keep out: #795-adjacent demo questions; #947 and the refusal-display
prose (LIB's charter); the germ-arm/SPHSPH lanes and #941's boolean
routing + M9-3 emission halves (VERBS' ground — handoffs recorded,
below); M10's editor-core parameters/analysis/eval files, `dual.rs`,
the `AtRestPolicy` seam in `topo/src/props.rs` and the Dual arms of
`product.rs`; PCURVE's `geom-brep` certify/edge_nurbs/adopt/nurbs_iso
files; assembly GUI chrome (GAUTH's).

## Ratified ground (cited, not re-litigated)

- **ASSEMBLY-DESIGN A1–A13 with the A11 member-vocabulary rider**
  (ratified 2026-08-23 at ASM's exit-walk sign-off; the ruling of
  record is the orchestrator comment on #945). #945 is therefore a
  **banked implementation unit, not a design question**: `head_of`'s
  member vocabulary (live `InstantiatePart` OR pattern-placed
  `Instance(i)`) plus composing the derived offset into the member
  frame in the solve; coset tables, cluster machinery, gauge
  convention and refusal vocabulary untouched; mates never solve
  pattern parameters; `Instance(i)` heads are the canonical spelling.
- **CENSUS-REST-CLOSURE-DESIGN (RATIFIED; both gaps BUILT — #969,
  #1063).** What remains of #943 is its NAMED RESIDUE: cross-instance
  CURVED declared Rest, with the sanctioned closing shape recorded —
  a certified everywhere-within-ε overlap enclosure on the shared
  curved carrier (Evan's latitude note, folded into that design's
  ratification). Evan's #943 constraint binds this whole stream: do
  not re-implement contact machinery as mates — the census consults
  the mate's own declaration.
- **The #131 cusp ruling** (DESIGN.md, D1 tier 3): #941 is its
  implementation door; the verdict table is pre-ruled.
- **The #966 ruling**: the torus declared-Rest lane is deferred but
  recorded (#968); VERBS ceded it in the #1200 work-stream survey
  (VERBS-PLAN's plan note) with LILYWELD's killed-rung context.
- **D2's addendum row 2**: a refusal of valid input the lane could
  serve — #1032's `CurvedPierceUnsupported` on a legal shaft-in-bore
  mate and #946's seam loss are classified there.
- **#750's two recorded falsifications** bind its S-BOOL unit
  (BOOL-4); this stream consumes the landed record, co-designs
  nothing ahead of it (both plans carry the handoff).

## Substrate facts the slate is shaped by (surveyed 2026-08-31)

- **The cut's "Needs Evan first" list is partly stale**, and the
  staleness runs in the good direction: #945 was RULED at the ASM
  exit-walk ratification (before the cut was written; the cut's own
  §S-MATE bullet 5 already carries the correction), and #943's two
  planar gaps are BUILT and MERGED (#969 gap 1, #1063 gap 2 — the
  tour's inset seat retired at gap 2). What is left of #943 is the
  curved residue, whose closing shape is already sanctioned in
  principle. The genuinely open rulings are the four below.
- **A stranded branch rides #943**: `m9/census-xid` @ 890d3fb6
  (recorded on the issue by the M9 orchestrator, 2026-08-26 —
  unreviewed, no PR, never gated) predates #1063's merge and covers
  the same gap-2 ground. Whether #1063 fully supersedes it is
  UNVERIFIED; dispositioning it (diff against the landed #1063,
  salvage anything unique, record the verdict on #943) is an
  orchestrator-owned chore, not a unit.
- **#1032's measurement stands on its own** (four spellings, the
  3-arc face structure held constant) and its mechanism hypothesis
  names `curved_face_arm`'s declared-Rest coverage test
  (`topo/src/boolean/reduce.rs`) — ground no live program is editing
  (VERBS' germ arms are `geom-brep/intersect.rs`; S-BOOL's is
  `solid_contain.rs`/`splitting`). The lily wall-probe-12 pin
  (`demos/tour/src/lily.rs`) flips when it lands — a VERBS-measured
  pin, so the flip is coordinated on the away channel.
- **#973's two sites are both executed live**, not argued: (a)'s
  residue is pinned red-when-closed by `the_lemma_probe_declared`
  (`crates/topo/tests/m9_c1_r1_probes.rs`); (b) reproduces with
  plain overhang geometry. Both turn on one question — which
  lower-dimensional configurations a declared face pair may answer
  for at rest — which is Q2 below.
- **#941 items 1–2 are independent of M9-3's resumption** (the
  issue's own sequencing). Items 3 (boolean routing) and 4 (M9-3
  emission) are VERBS-lane ground; item 5 (consumer sweep) crosses
  every program. The unit here lands 1–2 and records the 3–5
  handoffs.
- **#944 waits on LIB drafting its plan, not on Evan** (LIB-LOG's
  register correction). Per the cut it is taken only with LIB's
  hand-off; until then it is not on this slate.
- **Territory seams, named**: the mate solve and gate live in
  `editor-core/src/{mate,mate/solve,assembly}.rs` — the same crate
  M10 is working, different files (M10's slate is
  parameters/analysis/eval, `product.rs`'s Dual arms, schema
  v15/v16). Units here touch mate/assembly/census files only and
  merge main frequently. `census.rs` is single-file territory:
  MATE-4 and MATE-5 serialize with each other (the CENSUS-REST
  precedent — "serialized because both write census.rs").

## The slate

Ordered; each unit gets its own binding spec at dispatch; difficulty
pre-logged here per the protocol (pre-draw).

- **MATE-1 — #945, mates × patterns (M/L; dispatchable
  pre-ratification — the ruling is on the issue and in
  ASSEMBLY-DESIGN).** `head_of` gains the member vocabulary; the
  solve composes the pattern-derived static offset into the member
  frame (conjugation through the derived offset, per the rider); a
  sibling-instance tree mate closes a loop → non-tree → declaring →
  verified. Red-first rows from the issue's own repro (the
  four-legs-one-top shape the demo could not draw); the two ratified
  pins (parameters never solved — CONTRADICTORY with measured clash;
  `Instance(i)` canonical with the consumed master's faces honestly
  `Vanished`) each get their own row.
- **MATE-2 — #1032, cylindrical-only Rest (M).** First a
  measurement to the line (the issue's hypothesis is explicit but
  undiagnosed); then either widen `curved_face_arm`'s coverage test
  so an incidence on a declared shared carrier is covered regardless
  of which incident face the declaration names, or state the
  planar-contact requirement as a typed refusal at
  `validate_declarations` — the C8 posture decides which per the
  measurement, and the spec carries both candidate shapes. The four
  measured spellings become rows; the lily wall-probe-12 flip is
  coordinated with VERBS.
- **MATE-3 — #941 items 1–2, declared cusps (L).** The material-side
  wedge verdict table (transverse legal at θ = ε/r margin; π legal;
  0/2π legal iff declared + κ_rel margin; osculation escalates;
  undeclared refuses; lamina refuses) in the tier-3 pass, and the
  PATHS authoring door (the cusp analogue of `.tangent()`, retiring
  `PathError::JunctionCusp`'s no-door text and its pin). Revert
  symmetry is a test obligation. Items 3–5 recorded as handoffs at
  landing, not absorbed.
- **MATE-4 — #973 (Q2: (a) RULED, (b) DIRECTED). (a) impl M,
  dispatchable**: extend the face rung to `ef_bound_backed`'s
  interior arm at the existing rung strength; the pinned lemma
  probe goes red and re-blesses, by design. **(b) design S**: the
  side/region-aware crossing machinery is needed eventually
  (Evan's direction), so the design pass proposes its SHAPE and
  STAGING, not whether. Forward constraint recorded with it:
  interpenetration may eventually be ALLOWED when explicitly
  declared (Evan, same conversation; A5/C6's interference-fit
  gate-skips are the ratified anchor) — (b)'s vocabulary must not
  foreclose a declared-interpenetration class.
- **MATE-5 — #943's curved residue, the certified-ε overlap
  enclosure rung (L; RULED — Q3: build now;
  serialized with MATE-4 on `census.rs`; sequenced after MATE-2,
  whose measurement says which door a shaft-in-bore pair meets
  first).** Door 1 stays exact/certified; Door 2 gains the
  sanctioned certified everywhere-within-ε overlap enclosure on
  the shared curved carrier. New metered rows, NUMERIC class.
  Q3's dependency half is answered: **no M10 dependency** — the
  enclosure builds on the existing interval-scalar /
  certified-predicate substrate (the `carrier_eq`/`tangent_locus`
  class), not M10's parameter door or its bvh lift. Soft
  touchpoint: S-CERT's #1191 (period-fold widening) on the
  angular coordinate when one trim maps across the two
  descriptions' `u_ref` offset — consumed when it lands, never
  co-designed; until then conservative widening is honest (it
  widens toward escalation, never toward certification).
  Cylinder-first; other curved kinds stay refused with the
  residue restated per kind.
- **MATE-6 — #946, minting moves to evaluation (M; RULED — Q1,
  dispatchable).** A drift-closure, not a semantics change: A3's
  ratified "Declaration minting" paragraph already states that
  EVALUATION carries each mate's declaration into the evaluated
  body's contact record set — the code implemented minting in
  `assemble` and the seam inherited the gap. Minting moves into
  the product gather universally; `assemble` = product + tier-3′;
  construction composes, verification runs once at the outermost
  gate. Rows: the three-stands-in-a-row shape (inner mates, outer
  gate); a carried declaration the outer geometry refutes lands
  `StaleContactDeclaration → Refuted` naming its mate. The
  persistence check (by-eye at ruling, re-verified at spec):
  minted records are evaluation-side, the persisted recipe carries
  the Mate node only — no schema bump; if the unit finds
  otherwise it STOPS and reports. Seam: `product.rs`'s Dual arms
  are M10-4's — touch the gather/mint path only, merge main
  frequently.
- **MATE-7 — #968, the torus declared-Rest lane (L/XL; last;
  RULED — Q4: scheduled).** The three measured needs from the issue:
  torus through the operand gate under covered declarations (the
  VERBS-GATE posture consulted, klein walls 3/4 re-pinned), a torus
  rung in `carrier_eq` for the declared-Rest descent, and the
  torus×torus shared-rim tangency disposition — which returns to
  Evan as its own conversation before implementation (the #966
  thread's two candidate shapes). Lily wall 1 is the standing demand
  signal and its retirement path.
- **Not scheduled**: #944 (LIB hand-off gate), #750 consumption
  (recorded when BOOL-4 lands).

## Rulings (Evan, in-chat, 2026-08-31)

1. **Q1 — #946: RULED, minting moves to evaluation.** Of the
   alternatives surveyed in the conversation (carry the minted
   records across the seam; mint at the seam only; gate at the
   seam; gate at pin time memoized by the content pin; a typed
   interim refusal), Evan ruled for minting in the product gather
   UNIVERSALLY: `assemble` = product + tier-3′ — construction
   composes, verification runs once at the outermost gate. The
   ruling turned out to be a drift-closure: A3's ratified
   "Declaration minting" paragraph already assigns minting to
   EVALUATION, so ASSEMBLY-DESIGN needs no edit and the drift was
   the code's (the #945 shape). Soundness: the outer census
   re-verifies everything it consumes — crossings re-verify
   (#591), and a refuted carried declaration lands
   `StaleContactDeclaration → Refuted` naming its mate. Executed
   by MATE-6.
2. **Q2 — #973: (a) RULED, (b) DIRECTED.** (a) The face rung
   extends to `ef_bound_backed`'s interior arm at the existing
   rung strength — dispatchable. (b) The side/region-aware
   crossing machinery is needed eventually; MATE-4's design pass
   proposes shape and staging. Recorded constraint from the same
   conversation: interpenetration may eventually be legal WHEN
   EXPLICITLY DECLARED (A5/C6's interference-fit gate-skips are
   the ratified anchor) — (b)'s vocabulary must not foreclose it.
3. **Q3 — #943's curved residue: RULED (both halves).** No M10
   dependency (the S-CERT #1191 touchpoint is recorded at MATE-5),
   and the scheduling half confirmed in-chat: build in this
   program, cylinder-first, #1032's shaft-in-bore class as the
   named demand. MATE-5 proceeds as scoped.
4. **Q4 — #968: RULED, scheduled.** This program's opening
   satisfies the #966 ruling's recorded-pickup condition; MATE-7
   is last on the slate and its torus×torus tangency disposition
   returns to Evan as its own conversation before implementation.

## Process

As S-MESH/S-BOOL's, verbatim in substance (`docs/S-MESH-PLAN.md`
§Process): standard v6 — substrate → binding spec → one implementer +
the cross-model dual review + union fix pass; arms drawn per the
current block rule in `docs/MODEL-AB-LOG.md` (read on main at each
dispatch); ordinals claimed on main at review dispatch from band
1300–1399; record-at-merge with per-phase tokens/wall; blinding
discipline verbatim. Hosted CI is the only gate; every new row
ε-three-outcome honest; the #1356 ε-trailer practice from the first
dispatch. Implementer dispatches point at
`docs/prompts/implementer-discipline.md` by path; reviewers get
explicit claims to falsify plus `docs/prompts/reviewer-style-lane.md`.

**This orchestrator runs in a remote container** (the
S-CERT/S-QA/M10/GUI/S-MESH precedent): no persistent
`~/.local/share/cad-work`, no script monitors — PR watching via MCP
subscriptions plus scheduled self check-ins; away-channel etiquette
by hand under the `(S-MATE orchestrator)` tag; GitHub through MCP.
Lanes are worktrees sharing one object store, each with its own
`CARGO_TARGET_DIR`; review targets reclaimed the moment the report is
in hand. The build-slot mutex, CONFLICTING-means-silent-CI, and
push-early rules bind unchanged.

## Exit shape (proposed)

Mates compose with patterns per the ratified rider (#945 closed); a
shaft seats in a bore with no plate beside it, or the requirement
refuses typed at the door (#1032); declared cusps have their verdict
table and their authoring door, with the boolean-routing and emission
halves handed off by name (#941 items 1–2); the at-rest census
answers for exactly the configurations Q2 ratifies, with the
remainder's justification restated on an at-rest premise (#973); the
cross-instance curved seat certifies through the sanctioned ε-rung,
or its residue is re-recorded with Q3's ruling (#943 closed or
re-scoped); sub-assembly instantiation carries its mates per Q1's
ruling (#946); #968 is landed or re-banked per Q4; the
`m9/census-xid` strand is dispositioned on #943; #750's landed
record is consumed when BOOL-4 lands; and #944 is taken iff LIB
hands it off. Every unit merged on its own green hosted head; the
walk convention applies at exit.
