# FILLET-H5 — one host face, several arcs of one rim (spec)

**Program:** FILLET (`work/fillet/plan.md`), unit
`repaired-pole-rim-serves-no-closed-door`
(`work/fillet/repaired-pole-rim-serves-no-closed-door.md`). **Track:** kernel
change — the standard v6 unit (binding spec, drawn implementer arm,
cross-model dual review, union fix pass, record-at-merge; §Review below).
**Pre-draw fields, logged before the draw:** difficulty **M**, task-class
**STRUCTURAL**.

- **M** — one new routing arm in `resolve_rim`, one variant in the annulus
  plan's crossing (where the host foot comes from), and the annulus phase
  minting host feet by strut where no host seam exists; every geometric
  quantity it uses already exists (the ladder's strut foot, the annulus's
  trimlines).
- **STRUCTURAL** — no predicate, band or margin moves; the foot's position
  is the scaled carrier evaluated at the crossing's own parameter, the
  ladder's spelling (`rim_phase`, `scaled(..).eval(t0)`), never a
  measurement. If Phase 1 finds a decision is needed, the unit stops and
  is re-logged.

## The claim

**A closed rim whose arcs are hosted by ONE plane face in that face's outer
cycle is the annulus band with hostless crossings.** The shape arises two
ways: a pole-touching revolve after `merge_coplanar_faces` (each axis-touching
cap's two half-discs become one face — the body every boolean consumer
holds; `crates/sweep/tests/blend1_r1_probes.rs::p4_…` measures the lily
lantern's neck rim refusing), (and, as §Re-scope records, only that way: a full revolve splits the whole
loop at its seam, so no plane-hosted rim is minted natively). On
such a rim:

- the MATE side is what the annulus already serves — several faces of one
  surface, one arc each, a seam meridian at every crossing to split for the
  mate foot;
- the HOST side has no seam at the crossing (the crossing vertex is
  trivalent: two rim arcs and the mate's seam), so the host foot cannot come
  from a seam split. It comes from a STRUT — the ladder's own mechanism
  (`rim_phase` step (3): `mev` from the rim vertex to the point of the
  scaled host trim circle at the vertex's own parameter) — and the host
  trim arcs are minted per rim arc between consecutive host feet in the
  host face's OWN boundary loop, not as a ring.

Today `resolve_rim` (`crates/sweep/src/blend/surgery.rs`, the LADDER
discriminant: "ONE planar face hosting every link") routes it to the ladder,
whose ring gate refuses `"a closed chain is not a ring of its plane
support"`; the seam-split annulus (`resolve_seam_split_rim`) is never
reached, and would refuse at `wall_seam` on the host side. Nothing else about
the annulus walk — the mate splits, the mate trimlines, the excise `kef` per
arc from the host strip, the crossing merges, the closure slit — is about
where the host foot came from.

**Ratified and not re-litigated:** ARMS3 A3-2 (`crates/sweep/README.md`; its
last sentence names this gap and goes when it closes), BLEND-VOCAB V1–V4,
H4's fold (the band carves on either material side — this unit inherits it
and proves it on this shape too).

## Phase 1 — measure before touching anything

`memories/refusal-text-is-not-cause.md`. Build the fixtures through the public
doors and record, per fixture, in the PR body: the rim's arc count, the host
face's loop structure (outer cycle vs ring), the crossing vertices' incidence
(which edges meet there), which door `resolve_rim` reaches and its refusal,
and — with the ladder discriminant locally relaxed to fall through to the
seam-split resolution — where THAT refuses:

- the repaired lily lantern's neck rim (`p4`'s body, `merge_coplanar_faces`);
- a repaired pole-touching hemisphere on a flat base (plane × sphere, convex);
- a NATIVE instance with no repair: a pole-touching dome on a wider flat top
  (`revolved_about_y` of a profile whose top annulus does not touch the axis
  but whose dome does) — concave rim (a boss) — and its dimple twin (a
  pole-touching pocket into a wider flat top) — convex rim;
- the waisted body after repair (its cone×cone waist is unaffected — the
  control that the repair moves only plane-hosted rims).

**Stop clause.** If a fixture's crossing is not trivalent (two rim arcs plus
exactly one mate seam) or the mate side is not one-arc-per-face, the shape is
not the one this spec describes: stop at the report and file it.

## Phase 2 — the change

1. **Routing.** `resolve_rim`'s discriminant becomes: one planar face hosts
   every link — and the rim is a RING of it → LADDER; and the rim lies in
   its OUTER cycle → ANNULUS with hostless crossings; no single planar host
   → the seam-split annulus as today. State it in `resolve_rim`'s doc.
2. **The plan.** `SeamCrossing` says where each foot comes from — a
   host-side variant (`Seam(EdgeKey)` | `Strut`) rather than an `Option`,
   so the phase matches on what it has and the type cannot lie; the
   mate side stays a seam. `resolve_seam_split_rim` (or a sibling that
   shares its pair/host resolution — one home, no copy) builds the plan for
   this shape: the mate seams per crossing found as today (`wall_seam` on
   the mate loop), the host foot `Strut`, the closure crossing chosen as
   today.
3. **The phase.** `rim_phase_annulus` mints a `Strut` host foot by `mev`
   from the crossing vertex to `scaled(host carrier).eval(t at the
   crossing)` — the ladder's spelling, shared (one home: hoist the ladder's
   strut mint into a helper both phases call), records it in
   `rec.rim_feet` as the ladder does; host trimlines are `mef` chords
   between consecutive host feet in the host loop, described against the
   band's torus as the annulus does (`ContactCarrier::Exact`); the excise
   and crossing merges are the annulus's, with the strut dying at the
   crossing the way the ladder's struts do. Every Euler step named in the
   PR body with its census delta; `topo::validate_closed` holds at REST
   (the surgery's debug postcondition). **Amended at the fix pass, on
   both reviewers' measurement**: it does NOT hold after every step —
   the strut `mev` leaves its foot valence-1 (`ScaffoldingStrutVertex`,
   one per crossing) until that crossing's trim `mef` lands, and the
   LADDER's own `mev`-then-`mef` walk has the identical window. The
   claim this unit owes is validity at rest plus tier 3 at rest, and the
   strut's window is inherited with the ladder's move rather than
   introduced by this arm.
4. **Refresh.** `refresh_annulus_seams` (#935, two rims sharing a support)
   learns the hostless crossing (a `Strut` has no seam to re-read); a row
   composes a hostless rim with a second rim on the shared mate wall in one
   call and checks the sequential identity the annulus rows already pin.
5. **Rows** (`crates/sweep/tests/`, fixtures in `test_support`, no copies):
   - `p4` FLIPS: the repaired lantern neck carves — one band, tier-3 clean,
     census delta stated, `volume_pad == 0.0`, volume against a closed form
     (Pappus on the meridian fill or cut; the lantern's neck geometry is in
     `demos/tour`'s lily; derive in the row's doc);
   - the repaired hemisphere and the native boss/dimple pair carve, both
     material sides, each with its closed form (`test_support::pappus`
     pieces);
   - the seam-vertex composed pin: on the repaired body one arc refuses at a
     TRIVALENT corner (not `SeamVertex`, as `p4` measures) — the recourse
     shown there must be true; say which tag fires and follow its recourse;
   - naming totality via `test_support::assert_naming_totality` on a
     hostless band;
   - the #935 composition row (item 4);
   - the interval twin (`--features interval` locally; hosted CI gates the
     interval lane on every run since 2026-09-04, so nothing needs to be
     asked for — count twelve `test (…)` jobs and say so in the PR);
   - a mutant: the strut foot at the wrong parameter (e.g. the window's
     far end) reds the hostless rows through tier 3 and nothing else.
6. **Sentences, present tense only.** H4's exception clause in
   `FILLET3_ASSEMBLY_RECOURSE` ("a merged pole cap hosting every arc on one
   plane face refuses at the ladder's ring gate") goes — the clause becomes
   unconditional again and E2's composed pin follows it on the repaired
   body; `blend/mod.rs` doc sentences citing the gap (~`:638`–`:647`);
   README A3-2's last sentence and its stale `work/issues/` pointer;
   `docs/KERNEL-VERBS.md:59` clause (ii); `p4`'s own doc. Sweep by sentence
   shape (`repaired|merged pole|one plane face|ring gate|served by neither`)
   with the hit list and blind spot in the PR body.
7. **Riders, conditional.** If the unit opens `blend/naming.rs`, take `D323`
   (whether `Retired` owes a face channel or the never-retired argument
   moves onto `Retired`) and `D324` (the false "What consumes these rows"
   paragraph) together, closing them on the branch; if it does not open
   the file, leave both and say so.

## Constraints, binding

- **Every existing closed-rim carve is bit-identical to the merge base**
  (ladder, seam-split annulus, one-edge annulus, H4's concave rims): the
  dump (`crates/sweep/tests/bitdump.rs`) at base and head, both SHAs in the
  PR body. The ladder path is UNCHANGED in code; the routing change must
  send every existing ladder fixture exactly where it went.
- **No new metered predicate; no sampled normal decides anything** (S10/S11).
- **One home** for the strut foot mint (ladder and hostless annulus call
  it), for the host-rule (`naming::second_support_is_host`), and for the
  seam-incidence rule's readings (`resolve_seam_split_rim`'s doc names the
  four; a fifth reading is written there, not beside it).
- **Comments state the invariant** (`docs/prompts/implementer-discipline.md`
  §4).

## Acceptance

The Phase 1 table; `p4` flipped with its closed form; the four fixtures
carving on both material sides; the composition row; the mutant table; the
dump identical with both SHAs; every sentence in §6 swept; hosted CI green at
the drawn point plus the interval lane asked for, said so.

## Out of scope

The ruled-spine arms (H7); mid-curve run-outs (A3-3); the rim selector (RIM,
concurrent — do not touch `topo::query`; when RIM lands, `rim_arcs_at`
becomes a call to it and this unit's rows inherit that); a rim whose HOST
is a curved single face carrying several arcs (state whether the shape can
arise; if it can, file it).

## Review

v6 dual on the frozen head, claims to falsify:

- **C1** Every existing closed-rim carve is bit-identical to the merge base
  (re-run the dump; check the routing sends every ladder fixture to the
  ladder).
- **C2** The repaired lantern neck and the native boss/dimple pair carve to
  tier-3-valid solids at their closed forms, `volume_pad == 0.0`
  (re-derive the closed forms independently).
- **C3** The host foot is the ladder's spelling, one home, and its
  parameter is the crossing's own (the mutant reds only the hostless rows).
- **C4** The crossing's Euler walk is total: `validate_closed` at rest
  (see item 3's amendment — the strut's valence-1 window is the
  ladder's, measured on both arms); naming totality three-directional on
  a hostless band.
- **C5** No sentence in `crates/`, `docs/`, `demos/` still states the
  repaired/one-host-face rim as unserved; the sweep's blind spot is stated.
- **C6** The composition with a second rim on the shared mate wall matches
  the sequential result bit-for-bit (the annulus rows' own claim, extended).

## Re-scope at Phase 1 (2026-09-04, orchestrator)

**Phase 1's findings correct three claims above, and the correction is the
spec's, not the lane's.** (1) There is NO native instance: a full revolve of
a pole-touching profile is the wire case (`revolve/full.rs:313`–`:322`) and
sweeps the REST of the loop in two π-bands too, so every plane annulus of
such a body is two half-annuli and the rim's crossings are valence 4 — the
"native boss/dimple" both carve today through the seam-split annulus. The
plane-hosted rim arises only after `merge_coplanar_faces`. (2) The repaired
boss and dimple are not this unit's shape either: their merged flat top is an
annulus whose rim lands in a RING, so they route to the LADDER — and refuse
on a FALSE ring-clearance (the outer-boundary circle arm of
`ring_clearance_pass` uses external separation with no containment form,
`surgery.rs:1946`–`:1951`); that is its own pre-existing numeric defect,
filed as `work/fillet/ring-clearance-refuses-a-nested-trim-circle.md`, and
OUT of this unit. (3) With the ladder discriminant relaxed, the seam-split
resolution refuses at the half-band gate ("a seam-split rim's support does
not carry exactly its own rim arc"), not at `wall_seam` — the hostless
crossing must be admitted THERE. The lane's table, six fixtures with the
defect and both material sides reachable, is the record
(`work/fillet/plane-hosted-rim-has-no-native-instance.md`).

**Amended fixtures** (all after `merge_coplanar_faces`, all trivalent
crossings, all routed to the ladder's ring gate today): the lantern's neck
`(1, 0)` and lip `(0.2, 1.2)`, the hemisphere's equator `(1, 0)`, the
waisted body's base `(1, 0)` and top `(1, 1)` — convex; the **bowl floor**
`(1, 1)` of `revolved_about_y` of `(0,0) (1.5,0) (1.5,1.5) (1,1) (0,1)` —
concave, fill `+3.375275670087774e-4` by the lane's measurement (derive it
in the row). Phase 2 is as written with §The claim's "natively" sentence
struck; item 2's plan is built from the half-band gate's site; the
acceptance reads "the repaired fixtures above carve on both material sides,
`p4` flipped, the bowl at its closed form". §7's riders `D323`/`D324` are
already closed (code-quality, PR 1783) and drop out. Difficulty and
task-class stand (M / STRUCTURAL), re-logged at the redirect.
