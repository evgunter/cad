# M9-3 — the join lane (spec)

**STATUS: DRAFT — NOT DISPATCHED, NOT BINDING YET.** One
ruling is deliberately left open (the PR-boundary question in
"Process" below, which the germ-reachability spike was running to
answer when this thread paused); the block M9-16 draw has NOT
been made, so no implementer arm is assigned and no ordinal is
claimed. Everything else here is the orchestrator's considered
work order and can be taken as written by the resuming session.
Resume pointer: the issue filed against this document.

Orchestrator work order for M9-PLAN item 4 with item 6 (the M9-4
mark-wiring) folded in per the M9-D/U2 ratification. Substrate:
the dedicated join-lane exploration (2026-08-17, report in the
substrate lane; file:line refs below are from it, against main @
97fdd75e). Two stated dependencies, both binding:

- **The M9-2 census-arm dependency (satisfied — #564 merged)**:
  this unit's verification CONSUMES the shared doors M9-2 built —
  `contact_pair_verdict` (boolean/contact_verify.rs:92, two-body
  by signature) and the chart-overlap door (`T::chart_overlap`,
  chart_region.rs:252) — and never re-runs the census. The
  boolean's at-use verification calls the same doors per declared
  pair; no second verification vocabulary is minted.
- **The U2 emission-shape constraint (PCURVE-UNIFY-DESIGN,
  ratified)**: every edge description this unit mints stays within
  today's taxonomy AND maps 1:1 onto (surface, exact-lane pcurve)
  — `Intersection` on exact conic carriers,
  `TangentIntersection{s1,s2,witness}`, retained conventional
  `MappedCurve`. **No new `EdgeGeometry` variant** (U0 rejected).

ONE unit, TWO PRs, serialized (PR-B consumes PR-A's opened door
and its records): **PR-A — the wall and the door**; **PR-B — the
zip and the marks**. The carrier scope of the whole unit is
**plane/sphere/cylinder** (carrier_eq's certified set); cone,
torus and NURBS keep their typed refusals per-arm (C5 dispatch-
table discipline — lily wall 1's torus operand-gate pin SURVIVES
this unit untouched).

## PR-A — the wall and the door (binding)

1. **The front door opens by inventory, not wholesale.** The real
   C8 boundary is `validate_declarations` (boolean/mod.rs:
   1503-1530), which today refuses any declared NON-PLANAR face
   and any `class != Rest` — not vtxfac. Replace the planar check
   with the carrier inventory (plane/sphere/cylinder); admit
   `Tangent` ONLY where a witness locus derives from the DEV-1
   closed-form lane (`tangent_locus`, rest.rs:597-848:
   plane×cylinder, parallel cylinders); everything outside stays a
   typed refusal naming its class. mod.rs:1509's "declared face is
   not a plane" prose retires here; the door remains the single
   auditable C8 statement (undeclared touching refuses FOREVER —
   the door only widens what a VERIFIED declaration can unlock).
2. **The Rest (cosurface) descent is a type-level no-op — keep it
   one.** For declared-Rest same-carrier sectors at the coplanar-
   lump sites, swap `face_plane` + `oriented_plane_eq` for
   `face_carrier` (rest.rs:510) + `carrier_eq` (carrier_eq.rs:167)
   — `PlaneRelation` IS `CarrierRelation` (carrier_eq.rs:55), so
   `eq15_3_lump` consumes the verdict unchanged. This swap lands
   at BOTH wall sites: vtxfac.rs:118-132 (the typed
   `CurvedBooleanUnsupported` refusal) **and** the vertex-vertex
   lane's `recl_sectors` (recl.rs:103-135), whose curved lump
   today dies UNTYPED (`ClassificationInvariant "sector face lost
   its plane"`, recl.rs:40) — the two-peg path's rim vertices are
   v-v sites, so the recl swap is load-bearing, not symmetry
   polish. Structural/declared rungs only; nothing numeric is
   added to the Rest path (C12.5's never-numeric rule).
3. **The Tangent descent reuses the existing trilean — mint no
   predicate.** For declared-Tangent (distinct-carrier) pairs the
   lump verdict comes from the second-order sector trilean that
   already exists and is already metered:
   `enters_material_order2` (enters.rs:134, rows
   `tangent_sector_order2{,_arm}`) / the `tangent_jet` →
   `kappa_rel` chain (tangent.rs:51-76). Definite sign → SideCode
   (Enters→In, Exits→Out); exact-zero → bridged by the verified
   declaration per C4; in-band → escalate. **Binding default: no
   new metered predicate name.** If the implementer finds the
   boolean site's lever-arm story genuinely differs from the
   existing rows' derivation, STOP and report — a named row plus
   its predicate-dimension audit entry is an orchestrator ruling,
   never a silent mint (k-lint discipline).
4. **C8 invariant rows (acceptance-grade, in this PR):** an
   UNDECLARED touching curved pair still refuses (same types, same
   geometry — before/after pinned); a declared pair with DEFINITE
   counter-evidence still contradicts; an in-band/osculating pair
   ESCALATES (three-outcome honesty on every new row); the
   `UnsupportedDeclarationClass{Tangent}` door pin
   (m9_1_contact_vocabulary.rs:93-118) executes its own retirement
   text — the planar-flush-declared-Tangent fixture becomes a
   CONTRADICTION case, not a class refusal.

## PR-B — the zip and the marks (binding; after PR-A)

1. **The zip generalizes structurally, because it already is.**
   `try_rest_union` (rest.rs:137) is structural end-to-end —
   verification kind-general, patch discovery by verified declared
   surface key (rest.rs:210-220), `zip_seam` (zip.rs:50) pure
   Euler surgery. The carrier-kind work is exactly the residue:
   seam realization prefers the structural `fan_edge_between`
   (rest.rs:1010, carrier-agnostic — existing rim circle edges
   resolve here); where a seam segment must be MINTED, the chord
   mint (`mint_chord`, rest.rs:1046) gains a carrier-aware sibling
   emitting an arc/circle `EdgeCurveSpec` on the SHARED carrier —
   the one genuinely new emission site, U2-shaped per the header
   constraint.
2. **The marks are the folded M9-4, and they are emission-only.**
   Tier-3 enforcement already exists in full (validate.rs:
   1940-2043; wedge = π already legal, dihedral.rs:83-86;
   `SmoothUnderdetermined` already conservatively unenforced). The
   D6 smooth arm (`describe_minted_edges`, ops.rs:923-968) gains
   the `TangentIntersection` mint for determinate-jet G1 rims;
   isolated-κ_rel-zero rims keep their conventional `MappedCurve`
   and pass by today's ratified posture. The stale-description
   refusal `JoinDesync` (ops.rs:953-962) binds: curved smooth
   seams arrive non-stale or refuse loudly — a red-then-green row
   pins the non-stale arrival.
3. **Cosurface re-merge rides for free — pin it, don't rebuild
   it.** merge_faces' hard rungs are kind-agnostic since C12.5;
   full-engagement patch removal DELETES the bore-wall faces
   rather than merging, so the period-closure refusal
   (merge_faces.rs:123) is a test obligation (pinned green on the
   two-peg shape), not a blocker.
4. **Acceptance fixtures:** (i) the TWO-PEG KERNEL PATH — plate +
   two pegs + mating plane, three declared contacts (one planar
   Rest + two cylindrical Rest), union succeeds, volume EXACTLY
   additive against a closed-form oracle (the C7-lane statement;
   the demo CELL stays M9-5's); (ii) the lily-tube-chain rim shape
   — equal-minor-radius tangent walls, declared Tangent on the
   wall pair, rim minted as the wedge=π smooth seam carrying
   `TangentIntersection` where the jet is determinate; (iii)
   crosslap and the S1 suites unregressed (crosslap_rest.rs,
   m5_s1_rest_zip.rs, review_s1_probes/controls.rs, the crosslap
   tour cell, test_north_star.py — byte-identical expectations
   where they are mechanical).

## Acceptance (unit)

1. C8's invariant is auditable at the boundary: every
   currently-refusing configuration keeps its typed refusal unless
   a VERIFIED declaration on the pair unlocks the specific arm
   this spec names; undeclared touching refuses forever;
   osculating/in-band escalates.
2. The two-peg kernel fixture certifies three PatchContacts,
   unions, and answers exactly-additive volume; the boss_union and
   kiss pins stay green (M9-2's acceptance is not regressed).
3. ε-row three-outcome honesty on every new row; no new metered
   predicate name without an orchestrator ruling; hosted CI fully
   green on both PRs; PR-B's branch merges PR-A before opening
   (CONFLICTING = silent CI outage — merge main immediately before
   opening and on every main move).
4. Per-arm retirement notes land with the PR that retires each
   text (vtxfac.rs:118 comment and mod.rs door prose with PR-A;
   rest.rs module "planar" framing with PR-B); arms this unit does
   not open (torus/cone/NURBS gates, join.rs section-arm,
   ops.rs extent-scan) keep their texts VERBATIM.

## The one OPEN ruling — the PR boundary

The two-PR split above is the substrate's recommendation and the
orchestrator's default, but it rests on an unverified assumption:
that curved germ / vertex-vertex records REACH the rest lane's
segment discovery (rest.rs:149-266) on a two-peg-shaped body
today. If they do not — if reduction upstream refuses or drops
them first — PR-B inherits reduction work it is not scoped for,
and the split moves (the likely re-shape: a reduction-reachability
PR ahead of the zip, or the wall PR absorbing it).

**The spike that answers this was authored and never run** (the
substrate agent died at a model usage limit mid-authorship). Its
partial fixture is preserved on branch `m9/3-spike-wip`
(crates/sweep/tests/spike_peg.rs — plate + radius-0.5 peg sharing
the bore carrier, declared Rest, driving the union path so
`try_rest_union` is reached via ops.rs:478). **NEVER merge that
branch**; finish the fixture, run it, read where the pipeline
stops, then fix this section's ruling and drop the DRAFT status.
The verdict needed is one line: does PR-B inherit reduction work,
YES or NO — and if YES, roughly what.

## Substrate evidence (the anchors this spec was written against)

Kept here because the substrate report is lane-private and its
lane gets swept; refs are against main @ 97fdd75e.

- The wall's typed site: boolean/vtxfac.rs:118-132 (coplanar-
  classified curved sector, `face_plane` → None → refusal);
  sector data at sectors.rs:51-70, curved chart normals
  sectors.rs:217-283 (cylinder/sphere arms only).
- The wall's UNTYPED second site: recl.rs:103-135 lumping through
  `require_same` → `plane_of`, erroring `ClassificationInvariant`
  at recl.rs:40 — the v-v lane, which is the two-peg path.
- The real door: boolean/mod.rs:1503-1511 (non-planar declared
  face refused) and mod.rs:1528-1530 (`class != Rest` refused).
- Type-level identity making the Rest descent a no-op:
  `PlaneRelation` IS `CarrierRelation` (carrier_eq.rs:55-64);
  ladder `carrier_eq`/`carrier_eq_verdict` (carrier_eq.rs:167/186),
  `face_carrier` (rest.rs:510-542); `eq15_3_lump` (tables.rs:59-86).
- Second-order machinery, all extant and metered:
  `enters_material_order2` (enters.rs:134, rows
  `tangent_sector_order2{,_arm}`), `tangent_jet`/`kappa_rel`
  (tangent.rs:51-76), `tangent_second_order` (validate.rs:2005).
- The shared verifier (M9-2's, two-body by signature):
  `contact_pair_verdict` (contact_verify.rs:92-112); census
  consumption at census.rs:1560-1628; chart door chart_region.rs:
  252-263/345; Tangent witness lane `tangent_locus` rest.rs:597-848.
- The zip: `try_rest_union` rest.rs:137 (steps 149-266), entered
  at ops.rs:478; `zip_seam` zip.rs:50; seam realization
  `fan_edge_between` rest.rs:1010 else `mint_chord` rest.rs:1046
  (the straight-chord mint — the one new emission site).
- Marks: tier-3 check 4 validate.rs:1940-2043
  (`TransverseNotIntrinsic` 1965, `TangentNotIntrinsic` 2031-2042,
  `SmoothUnderdetermined` unenforced); wedge = π legal
  dihedral.rs:83-86; D6 smooth arm ops.rs:923-968; `JoinDesync`
  ops.rs:953-962; end-to-end precedent m5_pr9_sector2.rs:160-232.
- Pins: no test pins vtxfac:118 itself; retires-by-own-text =
  m9_1_contact_vocabulary.rs:93-118. Must-not-regress =
  crosslap_rest.rs, m5_s1_rest_zip.rs, review_s1_probes.rs,
  review_s1_controls.rs, demos/tour/src/crosslap.rs,
  test_north_star.py. SURVIVE untouched (different arms) =
  review_m3_pr4.rs:504/540, m3_pr2_reduce.rs:440,
  pncad/tests/all.rs:49-87, demos klein.rs:768, lily walls 1/2
  (lily.rs:1422-1460 — wall 1 pins the OPERAND-GATE torus refusal,
  reduce.rs:168-186, which this unit does NOT open).
- Two-peg: demos/README.md:569-585, CONTACT-DESIGN.md:523-533.

## Process

Unit protocol: ONE implementer arm for the unit = **block M9-16
slot 1** (v4 draw recorded at this spec's seam PR; blocked
randomization {opus×3, fable}, one urandom byte, reject ≥252,
byte mod 4 = fable's slot). Difficulty pre-logged at the draw:
**PR-A M, unit L**; task-class **STRUCTURAL** (the decided
predicates are reused as-is — carrier_eq's ladder and the
existing second-order rows; no new numeric decision is taken by
this unit; reasoning above). One blinded adversarial reviewer +
fix pass at the unit level (row AT MERGE with per-phase figures);
review ordinal claimed from the ledger ON MAIN at review
dispatch — if it lands on a multiple of 3, the dual is
CROSS-MODEL per #572 (R1 fable + R2 opus, concurrent same-head).
Standard brief lines: OUTPUT DISCIPLINE + the verbatim foreground
sentence; NO Co-Authored-By trailer in lane commits (blinding);
lane-private publish paths; comments state the invariant, never
the history; k-lint discipline; merge-main + BUILD THE UNION;
express build slots while the main mutex wedge stands.
