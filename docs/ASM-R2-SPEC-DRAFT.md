# ASM-R2 — mates, constructively (SPEC DRAFT — not yet binding)

Status: DRAFT by the outgoing orchestrator (2026-08-12) so the R2
dispatch starts from structure, not a blank page. Finalize AFTER
ASM-4 merges (the interface-record hook shape feeds D-5) and after
a substrate mini-recon of the census entry points. Binds ratified
A3 (the Mate node), A5 (at-rest validation), A11 (the
constructive-solve boundary — all five rules), CONTACT-DESIGN
C1–C4 (classes, verification tables — PLANAR subset only in this
unit; curved verification is R3/M9's C7-era sibling door).

## Unit cut (proposed: two PRs)

**R2-a — the solve (structural only, no geometry verification):**
- `Node::Mate { a, b, class, alignment }` per A3 — a/b are
  instance-qualified stable references (`InPart`-composed), class
  = CONTACT-DESIGN declaration vocabulary, alignment = which
  frames coincide + axis senses + clocking.
- Placement clusters become mate-connected components (A11 rule 2
  generalizing ASM-2A's singleton keying — the cluster-record
  keying migration is THE delicate step; the registry frame moves
  from per-instance-node key to per-cluster-representative key
  with recorded maintenance edits on mate insert/delete: join
  consumes the absorbed frame into the edit, split re-mints from
  the solved pose, gauge = document-order-first instance,
  gauge-deletion rewrite).
- Per-pair coset combination (A11 rule 1): the closed-form
  intersection table over {frame-coincidence, coaxial, planar-
  rest(+offset), clocking} — DETERMINED / UNDER / CONTRADICTORY,
  the latter two typed naming pair + residual/clash.
- Deterministic spanning tree from the gauge (document-order tie
  breaks); tree edges must be DETERMINED (UNDER refuses per A11
  rule 4 with recourse text); non-tree mates recorded as
  DECLARING (solved nothing) — their verification is R2-b.
  ReferenceCycle-style diagnosis discipline throughout.
- Evaluation: compose outward from gauge; Δc ≡ 0 by construction.
  D9 determinism rows; pin covers mates/clusters automatically.

**R2-b — declaration minting + planar verification (the A5 door,
planar subset):**
- Evaluation carries each mate's declaration into the product
  body's contact record set (the boolean 3′ currency — same type,
  no adapter; this is C4's second home landing).
- DECLARING mates verify against solved geometry via the C2
  planar tables (the census inventory that exists — census is
  planar-corpus-only today, which exactly matches); definite
  mismatch refuses naming the mate AND its loop; in-band
  escalates per C4; trilean discipline.
- The at-rest tier-3′ evidence: a touching two-instance assembly
  with a declared planar Rest validates; the same touching pair
  UNDECLARED is the F1 hard error (scan-to-bless ban across the
  seam — its first executable row).
- ASM-4's interface record extends: crossing declarations ARE the
  seam (A4) — split now populates it, and the split acceptance
  gains the re-verification row (pin-move gate = A4's "does it
  actually fit").

## Known open items for the finalizer

- The exact coset table's entries (write it out class × class
  before dispatch; refuse-typed any pair the table lacks).
- Whether Mate refs use reading edges (A10 coverage must NOT
  count mates as consumers — instances stay roots; verify the
  #414 walk semantics compose).
- AQ6 (cross-document Rest trilean detail) is deliberately NOT
  discharged here — planar value-equality rests get the C4
  bridged treatment; the peg/bore CURVED case waits for C7-era
  tables with AQ6's own conversation.
- Improper frames still refuse (A6/R4 unchanged).
- Difficulty pre-log: R2-a = L / structural; R2-b = M /
  **numeric-predicate** (the program's first — the verification
  margins are numeric decisions; stratified allocation applies
  per #409 P3 from block ASM-3 onward).

## Recon addendum (2026-08-15; full report cad-work/asm-r2-recon-report.md)

The census mini-recon ran against post-M8-close main. Four
corrections the finalizer folds in (file:line grounding in the
report):

- **(a) R2-b BUILDS the cross-instance census door, it does not
  call one.** `census_and_certify` is single-body; touching
  solids after a disjoint graft sit in the tier-3-not-3′ gap by
  documented design (instance.rs:41-56, = #382). The F1 row's
  cost estimate rises accordingly; the planar-corpus-only claim
  is otherwise CONFIRMED (structurally enforced refusals).
- **(b) The product gather DROPS ContactRecords** — product_named
  gates with validate_geometric only and sources_of discards the
  boolean wrapper's contacts; PartValue is {body, names}. The
  "same type, no adapter" landing needs a contacts channel
  through product/instantiate, a real (if mechanical) plumbing
  addition to R2-b's cut. resolve_declarations (wire.rs:819) is
  the no-adapter name→key mechanism a Mate reuses.
- **(c) A no-input `Node::Mate` is an A10 SINK → root** (sink-set
  theorem): uncovered violates coverage, root hits the
  no-body-root refusal. The draft's "mates must not make
  instances consumers" half composes (Declare precedent:
  name refs are not DAG edges); the Mate node's OWN root status
  needs a ruling at finalization — carve-out vs input edge vs
  off-DAG registry (placements precedent). Likely an Evan
  touchpoint: it grazes ratified A10's invariant statement.
- **(d) ContactRecords is vertex-granularity** in per-body arena
  keys; a face-pair planar Rest mate maps onto it via vv/vf rows
  plus a face-pair rung that exists today only as
  BooleanDeclarations::coincident_faces (no class payload yet;
  editor-side ContactClass is Rest-only non_exhaustive and can
  be shared rather than minting a third enum).

Also confirmed for R2-a: NO coset/SE(3) machinery exists — the
class×class table is greenfield (imitate oriented_plane_eq /
merge_faces ladders, don't reuse); the trilean rail is fully in
place (Margin/Band/decide funnel with predicate-name recording);
a solved pose could ride WitnessDatum's schema-tagged bytes.

## R2-a coset table (drafted 2026-08-15; finalizer folds into the spec)

Discipline: each mate primitive constrains the pair's relative pose
to a coset (representative pose × residual subgroup). The solve
folds a pair's mates by set intersection (order-independent), so
the table must cover the CLOSURE of the primitive set under
intersection. Closure set (proof obligation: the enumeration below
— the spec unit-tests it): **SE(3)** (unconstrained), **planar(P)**
(2 trans in P + rot about n̂, dim 3), **cylindrical(L)** (rot about
+ trans along L, dim 2), **prismatic(d)** (trans along d, dim 1),
**revolute(L)** (rot about L, dim 1), **trivial** (DETERMINED),
**empty** (CONTRADICTORY). No screw subgroup arises from these
primitives (parallel-cylinder intersection yields pure translation).

Primitives from A3 alignment data: frame-coincidence → trivial;
coaxial → cylindrical; planar-rest(+offset) → planar; clocking is
a RIDER (coaxial+clocking → prismatic along the axis; on
frame-coincidence it is redundant-or-contradictory, decided).
Standalone clocking without a carrying mate: refuse typed (the
table lacks it, per the refuse-any-missing-pair rule).

Every case split below is a DECIDED predicate (Margin over an
angle or a distance through the k_stats funnel; Indeterminate =
typed escalation, never silent). Axis senses and one-sided rest
normals kill all π-flip ambiguities; a sense mismatch is empty.

Unordered entries (X ∩ trivial = consistency check of X's
predicate at the pinned pose, redundant-or-empty; X ∩ SE(3) = X;
X ∩ empty = empty):

1. **planar(P1) ∩ planar(P2)** — normals parallel (decided): gap
   compatibility (A-side inter-plane offset + declared offsets vs
   B-side, decided equal) → planar (redundant rep merge), else
   empty naming the gap mismatch. Normals at dihedral θ: A-side
   dihedral must equal B-side (decided) → **prismatic** along the
   planes' intersection direction (the V-block), else empty.
2. **planar(P) ∩ cylindrical(L)** — L ∥ n̂(P) (decided) →
   **revolute(L)** (the pin-in-hole; the planar contact kills the
   axial translation). L ⊥ n̂(P) → axis-to-plane height must match
   A vs B (decided) → **prismatic(L)**, else empty. Generic angle
   → **trivial** (the τ-component along n̂ and the normal-moving
   rotation both must vanish); representative solved closed-form,
   incompatibility → empty.
3. **cylindrical(L1) ∩ cylindrical(L2)** — same line (decided
   coincident, senses aligned) → **cylindrical**. Parallel
   distinct: inter-axis displacement invariants match A vs B
   (decided) → **prismatic** (common direction), else empty.
   Concurrent at angle θ: angle match (decided) → **trivial**,
   else empty. Skew: angle + common-perpendicular distance +
   handedness match (decided) → **trivial**, else empty.
4. **prismatic(d) ∩ planar(P)** — d ⊥ n̂(P) (in-plane, decided) →
   **prismatic**, else **trivial**. **∩ cylindrical(L)** — d ∥ L →
   **prismatic**, else **trivial**. **∩ prismatic(d′)** — d ∥ d′ →
   **prismatic**, else **trivial**. **∩ revolute** — **trivial**.
5. **revolute(L) ∩ planar(P)** — L ∥ n̂(P) → **revolute**, else
   **trivial**. **∩ cylindrical(L′)** — L = L′ → **revolute**,
   else **trivial**. **∩ revolute(L′)** — L = L′ → **revolute**,
   else **trivial**.
   (Every "else trivial" entry still constructs and checks the
   unique candidate pose; an unsatisfiable candidate is empty,
   named.)

Verdicts after the fold: residual trivial → **DETERMINED** (pose =
representative); positive-dim → **UNDER**, refusal names the
residual subgroup and its parameters (A11 rule 4 recourse text);
empty at any step → **CONTRADICTORY**, refusal names the two mates
whose intersection died, the failed predicate, and the measured
clash.

**Pre-log flag for the finalizer**: the table's case splits are
decided NUMERIC predicates, so R2-a is not purely structural as
pre-logged — under #409's mixed rule it records as numeric. Amend
the pre-log AT SPEC TIME, before any dispatch draw reads it.

**AQ7 dependency**: the cluster partition and root behavior on
mate insert/delete wait on the AQ7 ruling (PR #522); the table
above is partition-independent.
