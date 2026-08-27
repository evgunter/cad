# VERBS-ARMS — the curved-support fillet arms, in three units

Wave 1 unit 3 of `docs/VERBS-PLAN.md`, cut into three sub-units per
the 2026-08-22 substrate survey (anchors verified on post-#920
main). Consumer: #319 (the calochortus bud's sphere–cone seam).
This document is ARMS-1's binding spec plus the ratified cut for
ARMS-2/-3; each later unit gets its dispatch addendum here.

**A C8 prose correction, called out for Evan's visibility (the math
is checkable and the change is scope-honest, so it rides this spec's
state-sync rather than a conversation):** CURVED-DESIGN C8 says
"cone cases → cone/torus per configuration". The derivation does not
support the cone half for THIS unit's family: a constant-radius
rolling ball is the envelope of EQUAL spheres, and every such
envelope over a line/circle spine is a cylinder/torus — a cone is
the envelope of spheres of linearly varying radius, i.e.
variable-radius territory (the canal/frontier-(f) family) or the
chamfer's ruled strip. Every arm below mints a torus or a cylinder;
C8's cone wording stays correct for the variable-radius family it
also covers. The correction lands as a scoped parenthetical in
CURVED-DESIGN at ARMS-2's merge.

**The unifying derivation** (six of eight pairs): when both supports
are surfaces of revolution about a COMMON axis, the offset surfaces
are too, their intersection (the spine) is a circle about that axis,
and the blend is the torus (major = spine radius s, minor = r) —
exactly `plane_sphere_blend`'s existing shape (h/s formulas, poison
flowing to `spine_curvature` so predicate 3 escalates degenerate
spines — copy that posture, no new gate). The per-pair content is
only the two offset parameters and the s formula. Non-coaxial
configurations refuse `SpineUnsupported` honestly (they are the
canal family, not this unit).

## ARMS-1 — the closed-rim surgery + the torus validate net (this dispatch)

Branch `verbs/arms1`, PR to main. Difficulty logged pre-draw: **L**.
Uses ONLY the existing plane×sphere arm; delivers the #554
verb-level unlock (`fillet_edges` on a full solid of revolution).

1. **The one-edge closed-chain band.** `resolve_rim`'s
   `link_count() < 2` gate (surgery.rs:616-632; its comment already
   says #910 made the door live) retires for the supported case: a
   one-link closed rim's band face is an ANNULUS (two closed
   boundary circles), not the strut-and-`kef` quad ladder
   `rim_phase` walks — mint it with ring-class Euler moves
   (`kemr`/`mfkrh` family). The `unreachable!` at surgery.rs:675-680
   is justified BY the retired gate — revisit it explicitly. The
   other closed-chain gates (PlaneSphereTorus arm, convex, one
   shared plane support, plane ring, ring-free sphere) STAY — they
   fence exactly what ARMS-2 will widen.
2. **The spelled wrap-around G1.** A self-closed link never reaches
   `chain_g1` (walk_chains registers its vertex once — pinned
   deliberately by the r1 probe row). Add the explicit wrap-around
   tangency check on the single link's own carrier endpoints —
   vacuously true for `Curve3::Circle` by construction (assert it
   cheaply), non-vacuous the day a closed NURBS carrier arrives.
   Reuse existing predicate names (`fillet3_chain_g1` at the
   wrap-around site) — no new metered name.
3. **#889's `R > r` net.** Tier-3 check 1's shape
   (validate.rs:1803-1828, the surface-implementedness loop): a
   ring-torus violation (`minor >= major`) raises a typed
   validation error on ANY door's torus — the second net behind the
   poisoned-spine refusal, load-bearing before ARMS-2 mints tori
   from curved supports. Update both error-order tables
   (validate.rs:4146, :4318). Correct the two stale prose sites the
   issue names (step-import reads verbatim; surfaces.rs:216's
   "rejected upstream"). #889 closes here.
4. **Acceptance**: the dome fixture's closed rim FILLETS end to end
   (flip the r1 probe row per its own instruction — "flip this row
   when a one-edge torus band is actually built"); tier-3 valid,
   census pinned, mass properties vs closed form; the partial
   revolve of the same profile still refuses per its own gates
   (differential pair); planted `minor >= major` torus red at
   tier 3; existing fillet + chamfer suites bit-identical
   (the surgery changes must not move open-chain or N-link-rim
   outputs). Note the drawn CI point in the PR body.

Fences: no new arms (ARMS-2), no corner-door changes (ARMS-3, and
closed rims register no corners at all — surgery.rs:300-308), #883
still parked, no MappedCurve reach.

## ARMS-2 — the coaxial revolution arms (DISPATCHED 2026-08-23; addendum below)

**Dispatch addendum (binding, post-ARMS-1/RING facts):**

- **The surgery half is real scope, not a rider.** ARMS-1's annulus
  door is plane×sphere-specific by its retained gates (one shared
  PLANE support; the plane trim circle; the sphere-side seam
  handling). ARMS-2 widens `resolve_rim`/`rim_phase_annulus` to
  per-kind supports (a sphere×cone rim's band replaces material on
  a sphere wall AND a cone wall — trimlines are latitude circles on
  each, minted per kind). Every gate ARMS-1 kept is a deliberate
  widening decision here — widen each explicitly or keep it with
  the refusal naming what ARMS-2 does not cover. The
  `shared_support_gate` applies as-is (it is kind-agnostic).
- **Ask the material-side question structurally for EVERY arm** —
  the ARMS-1 lesson: `plane_sphere_blend` silently assumed the
  pocket configuration and only the dome exposed it. Each new arm
  derives its offset signs from the support faces' STORED sense
  bits (R ∓ r-style folds), with both configurations unit-rowed at
  closed forms. Every full-revolve wall is a 4-half-edge cycle with
  EMPTY `face.rings` (banked at ARMS-1) — the consumer shape all
  these rims share.
- **Acceptance consumer**: the calochortus bud's sphere×cone seam
  (#319), requested as the MOUTH RIM ALONE (lily wall 6 requests
  every lantern edge and refuses at the co-surface seam meridian
  first — it cannot distinguish; the register's own note). Closed
  rims register no corners, so ARMS-3's valence-4 door is not in
  the path.
- Re-mint `BlendArm::name` from 60941420 as the enum grows;
  `SpineUnsupported`'s hand-formatted supports string is the
  consumer. The C8 prose correction (constant-radius rolls mint
  only torus/cylinder; cone belongs to the variable-radius family)
  lands in CURVED-DESIGN as the scoped parenthetical flagged on
  #930.
- Difficulty logged pre-dispatch: **L**. Branch `verbs/arms2`.

**The ratified cut (from the survey):**

The one shared coaxial-spine derivation with per-pair s: sphere×cone,
cone×plane(⊥ axis), cone×cone, cylinder×cone, cylinder×sphere,
cylinder×plane(⊥) → torus; the straight-spine pair family
(cylinder∥cylinder, cylinder×plane(∥)) → cylinder. Extends the
`blend.rs:6-10` module table; re-mint `BlendArm::name` from
60941420 when the enum grows (the `SpineUnsupported` payload's
hand-formatted list is the consumer). Rides AFTER ARMS-1: every
coaxial-arm consumer is a one-link closed rim, unreachable without
it. The acceptance names the calochortus MOUTH RIM alone — lily
wall 6 requests every lantern edge and refuses at the co-surface
seam meridian first, so it cannot distinguish (the register's own
note). C8 prose correction lands here.

## ARMS-3 — general sphere×sphere + the valence-4 corner run-out (last)

The only general-position arm (two spheres always meet in a circle),
and the natural carrier for the corner door: open curved chains
terminating at seam vertices. The run-out taxonomy at a
non-trihedral vertex is OQ6's reserved design question — **ARMS-3 is
a design-conversation PR for Evan**, not a self-merge. Reproduce
#319's valence-4 witness first (the survey could not build to verify
it).

**Dispatch addendum (post-implementation facts, binding on anything
that touches this ground next):**

- **The witness, reproduced.** `FilletCornerUnsupported { NEdgeVertex
  { valence: 4 } }` needs a full revolve of a POLE-TOUCHING
  (non-annular) profile: that splits every wall into two half-bands,
  so each latitude rim is TWO arcs and each seam vertex has four
  incident edges — the two arcs plus the two supports' seam
  meridians. On an ANNULAR profile (the bud, the dome, the snowman)
  each rim is one self-closed edge, hence a closed chain, hence no
  corner is registered and the witness is unreachable. Requesting ONE
  arc is what reaches it.
- **Both extra edges are co-surface** (one surface on both sides), so
  the dihedral there is zero by construction; each refuses
  `TangentialEdge` at margin exactly zero in its own right. That is
  the structural recognition `SeamVertex` is decided on — never a
  sampled normal.
- **A3-2's recourse premise was false and is corrected in the design
  doc**: the whole-rim request on a seam-split rim is a multi-link
  closed chain and the annulus band is a one-edge rim's, so that door
  refuses one step later (#1022). The tag's claim is unaffected.
- **The snowman waist does NOT build.** It is a valley, so its band
  would add material; with the arm in place it now refuses at the
  concave-chain door instead of the spine door. The convex
  sphere×sphere consumer is a LENTIL (the solid between two spheres),
  and that is the fixture the acceptance uses.

## Lane obligations (every unit)

`docs/prompts/implementer-discipline.md` binds; no Co-Authored-By
trailer (blinding); lane-private PR drafts; merge origin/main before
opening; confirm CI runs STARTED; note the drawn point; watch to
completion; do not merge.
