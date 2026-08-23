# At-rest census structural identity (#943 + #591 Door-2) — design conversation

STATUS: **OPEN — awaiting Evan's ruling.** Two proposed rulings
below (U-R1 dominant-argument with pushback window; U-R2 a genuine
C3/C4 revision that WAITS for sign-off), four questions. This is
the census-owned design pass both steers asked for: #943 (the
at-rest face-pair-backed closure) and the F1/Door-2 item on #591's
thread (cross-instance chart identity), inherited by M9 because
the at-rest door is M9-2's census machinery. Substrate: dedicated
exploration 2026-08-23 (file:line evidence below is from it,
against main @ 76856a8d). Evan's constraint from #943, held
throughout: **do not re-implement contact machinery as mates** —
the mate already said the right thing (one face pair, declared
once); what is missing is the census consulting it.

Both gaps fire on the same natural document (a post seated flush
under a shelf's end; #943's repro): gap 1 makes the seat a hard
`AtRest` refusal (Unattributed findings), gap 2 keeps even the
inset seat at the `Uncertified` frontier for INSTANCED parts. They
compose but do not depend — either lands alone with a stated
residue.

## What the substrate settled (facts, not options)

- **The shared entry chain**: `assemble` (editor-core/src/
  assembly.rs:342) mints each live Rest mate as a `PatchContact`
  (assembly.rs:435) → `validate_pseudomanifold` → the census
  (census.rs:171-188) → attribution (assembly.rs:520-577):
  all-Declined ⇒ `Uncertified` (typed frontier), any Unattributed
  ⇒ hard `AtRest` (assembly.rs:377-388).
- **Gap 1's mechanism is two visible holes in an established
  pattern, not a missing feature.** The face-pair-backed rungs
  #943 cites (`vv_face_backed` census.rs:235, `vf_face_backed`
  :245) are consulted at :434, :551, :622, :926, :1600-1607 —
  everywhere EXCEPT: (i) `sweep_vertex_edge` (census.rs:446-484),
  the ONLY sweep called without `declared`, so every interior
  vertex-on-edge is an unconditional `UndeclaredContact`
  (:473-481) per D4 (:105-114); and (ii) `ee_bound_backed`
  (:909-927), which returns false when a bound lands on the other
  edge's INTERIOR (:919/:924) before any rung is reached. D4's
  "therefore" (a vertex-on-edge is always refined to v-v before
  records are emitted) is a boolean-lane fact — reduction's
  `split_other_at_point` — and at rest nothing refines: #943's
  diagnosis confirmed verbatim in the code.
- **Gap 2's mechanism**: `confirm_curve_and_patch_records`
  (census.rs:1851) — Door 1 (`contact_pair_verdict`, :1926)
  PASSES for the mated planes; Door 2 (`T::chart_overlap`, :1963
  → `same_chart`, chart_region.rs:414-473) falls through the
  fused body's fresh graft keys to the source match: two
  instances' sources are `Placed{node, instance, inner}`
  (compose_placed, wire.rs:295-325) — distinct → the :466 arm →
  `ChartDivergence` → `CensusUnsupported{Face}`
  (census.rs:1974-1978) → Declined → `Uncertified`. (The #591
  comment's census.rs:1631 has drifted; :1974-1978 is current.)
- **Provenance is NOT destroyed at the graft** — sources are
  copied verbatim under fresh keys (combine.rs:179-180); the
  `Placed{instance}` wrapper differs BY DESIGN (per-evaluation
  recipe identity).
- **Both fixes floated on #591's thread are structurally
  insufficient — fact, not preference.** A mated pair is two
  DIFFERENT part faces under DIFFERENT placements: (a) carried
  provenance can never equalize their sources (different
  `Minted{index}` inners under any wrapper convention); (b) "same
  description, different key" never holds — `transform_rigid`
  bakes each placement into the world-space description, and a
  Rest pair's charts MIRROR anyway (opposed senses, the :463
  arm). The one-body shared-key rung works precisely because
  sense lives on the face, not the surface; no cross-instance
  analogue exists to restore.
- **Gap 2 sits on a latent tension in the ratified docs**: C4's
  Rest must-verify list REQUIRES "chart-space overlap definitely
  positive" (CONTACT-DESIGN.md:284-288) while C3's invariant
  ratifies that chart-space exactness is UNACHIEVABLE for a
  rung-3 pair and escalates it typed (:243-246; implemented at
  chart_region.rs:19-24). As written, a declared cross-instance
  Rest can never certify: today's refusal is the docs' own
  consequence, so closing it is necessarily a C3/C4 revision —
  which is why this is a design conversation and not a patch.
- **File-disjoint from M9-3, already ruled**: M9-3-SPEC (BINDING)
  fences both gaps out of the join lane; the join lane writes
  boolean/{mod,reduce,vtxfac,recl,rest,zip,ops}.rs and only READS
  the census doors. Parallel dispatch is safe.

## Proposed ruling U-R1 (gap 1) — the forced closure, dominant argument

Extend the existing face rung to the two places it visibly stops
short: pass `declared` into `sweep_vertex_edge` with the rung
"either adjacent face of the edge is vf-face-backed onto the
vertex" (`e.{f_plus,f_minus}` are already on `EdgeGeo`,
census.rs:137-138 — the existing rung verbatim plus one incidence
step), and the same rung closes `ee_bound_backed`'s interior arm.
Rewrite the D3 bullets and D4's at-rest sentence, roughly: *in
the boolean lane the configuration never survives to records
(reduction refines); at rest it is certifiable only through the
face rung — a declared face pair holding the vertex on one
boundary and the edge on the other — and otherwise remains an
undeclarable defect.* Nothing outside census.rs + its module
docs; no CONTACT-DESIGN invariant moves. Evan's #943 constraint
is satisfied by construction: the rung consults `Declared::faces`
— the mate's own minted PatchContact (census.rs:220-229) — no new
machinery, no mate-side vocabulary. The tour's `SEAT_A`/`SEAT_B`
inset workaround (demos/tour/src/assembly.rs:104-113) retires.

Proposed as **dominant argument with the standard pushback
window**: it is the unique candidate the code offers and an
instance of the already-derived pattern. Its implementation unit
(S) can dispatch without waiting on U-R2's ratification; on gap 1
alone, #943's flush seat moves from hard `AtRest` to the same
`Uncertified` frontier the inset seat reaches today.

## Proposed ruling U-R2 (gap 2) — a world-carrier Door 2 for declared PLANAR pairs; C3/C4 revision. WAITS for sign-off.

Three options examined:

- **A (proposed): the world carrier as the shared chart, planar
  pairs only.** Door 1 plus the verified declaration already glue
  the carriers — C4:290-293's own words: "the declaration is what
  makes them one carrier." For PLANES the carrier's world
  embedding is a chart both descriptions agree on as a locus with
  no u_ref/seam ambiguity — C2's chart-divergence caveat
  (:151-155) is specifically about chart PARAMETERS, which a
  plane's world embedding does not have. The overlap test becomes
  the planar parity/containment machinery the census already owns
  (`contfp`), exact on the F5 subset — named by C3:230-232 itself
  as the intended machinery. Cross-instance CURVED declared pairs
  stay refused (u_ref/seam divergence is real there; matches the
  join lane's carrier posture and the backstop). Cost: revise
  C3's rung-3-escalates invariant (add the planar-world-carrier
  arm) and C4's "chart-space" wording to name the carrier.
- **B (rejected by the docs' own text): numeric chart transfer** —
  map trims across the affine relation between the two plane
  charts. C3 rejects this shape by name ("a margined pseudo-exact
  test in whichever chart we happened to pick",
  chart_region.rs:21-24); C2:200-205's rejected alternative is
  the same instinct. Dead unless C2 re-opens.
- **C (examined and CLOSED): provenance/predicate identity through
  the graft** — structurally insufficient per the settled fact
  above. Recorded here so it is not re-proposed.

Consequence wiring already reserved in the code and taken as part
of this ruling: when Door 2 starts answering for these pairs, the
`StaleContactDeclaration → Refuted` arm goes live and lands its
own acceptance row in the same change (assembly.rs:548-556's own
comment), and asm_r2b's `row3_b` goes red and is re-blessed
deliberately (#591's pin, working as designed).

## Sequencing consequence (part of the ruling)

One design doc (this one), then TWO implementation units, either
order safe: **gap 1 = S** (census.rs + module docs + rows + the
flush-seat acceptance row + the demos inset retirement), **gap 2
= M** (the Door 2 planar-carrier arm + the C3/C4 revision text +
the Refuted-arm acceptance row + row3_b re-bless). Both are
census-side and file-disjoint from M9-3; they schedule as M9
adjacents without touching the join lane's critical path. Gap 1
dispatches on U-R1's pushback window opening; gap 2 dispatches
only on U-R2's ratification.

## Questions for Evan

1. **U-R2's load-bearing claim**: for a declared+Door-1-verified
   PLANAR pair, the shared world carrier is a chart free of
   parameter ambiguity, so region overlap on it is the same
   exactness class as the one-body planar lane (F5 `contfp`
   parity — no margined transfer). Accept or refute — this is the
   C3/C4 revision's whole content.
2. **The curved boundary**: cross-instance CURVED declared Rest
   stays refused under U-R2. Permanent posture, or named residue
   for a future rung (the seam-normalization / inf-bounds era)?
   Proposed: named residue, stated in C3's revision.
3. **Gap 1's rung strength**: the new rung is structural-incidence
   (like the existing two), NOT confined to the declared
   interface region. Consistent with today's rungs — but it
   should be a stated sentence in the census docs, not an
   accident. Confirm.
4. **Where the revised D4 derivation lives**: today it is
   census-module-doc-pinned only. Keep it there (module docs stay
   the derivation of record), or lift the at-rest sentence into
   CONTACT-DESIGN alongside C3? Proposed: keep module-doc, add
   one C3 cross-reference line.
