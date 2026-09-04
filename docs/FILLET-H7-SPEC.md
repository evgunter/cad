# FILLET-H7 — the ruled band and its transverse cut-off (spec)

**Program:** FILLET (`work/fillet/plan.md`), unit
`fillet-ruled-spine-arms-no-surgery`
(`work/fillet/fillet-ruled-spine-arms-no-surgery.md`; Ev's ruling on PR
1736: option 1, the transverse cut-off at caps perpendicular to the ruling).
**Track:** kernel change — the standard v6 unit (binding spec, drawn
implementer arm, cross-model dual review, union fix pass, record-at-merge;
§Review below). **Pre-draw fields, logged before the draw:** difficulty **L**,
task-class **NUMERIC**.

- **L** — a new termination in the open-chain surgery (admission, plan,
  Euler walk, descriptions), the first open band on curved supports, and a
  new corner/run-out tag; three code regions and one vocabulary.
- **NUMERIC** — the cut-off admits a cap only when it is perpendicular to
  the ruling, and that is a new metered decision (`fillet3_cap_transverse`,
  band-metered and trio-pinned like its siblings).

## The vocabulary this spec proposes for ratification (Ev, per PR 1736)

- `CornerConfig::TransverseCap` — *a straight-spine band's edge ends at a
  vertex whose other incident edges lie in one face perpendicular to the
  spine.* Trivalent: the requested edge and the two rim edges the cap
  shares with the band's two supports; the cap plane's normal is parallel
  to the ruling. Not a corner in the trihedral sense — the ball does not
  turn — and not a seam vertex: the surface is not smooth through it.
- `RunOutPolicy::CutOffAtTransverseCap` — *the band ends in the cap's own
  section of it*: for a cylinder band about the ruling, the cap plane's
  section is a circle of the band's radius about the spine, and the band's
  end is the arc of it between the two feet. Exact and stored
  (`Curve3::Circle`), no new surface kind. `CornerConfig::policy` maps
  `TransverseCap` to it, so tag and policy cannot disagree.
- The oblique cap, the curved end face and the chart-seam end refuse
  typed with their own detail: they are the mid-curve / general run-out
  A3-3 reserves, unchanged by this unit.

A 👍 on PR 1736's `[ev]` thread for this spec ratifies the two names; a
comment renames them. The unit dispatches after that.

## The claim

**A ruled band is the open band with straight trimlines on curved supports,
and it terminates where its supports do.** The arms exist and are exact
(`BlendArm::CylinderCylinderCylinder`, `BlendArm::CylinderPlaneCylinder`,
`arms.rs`: `Ruling` reduces both supports to the cross-section normal to the
ruling, the ball centre is the crossing of the two offset traces, the band a
`Surface::Cylinder` about a straight spine, both trimlines lines along the
ruling; `verbs_arms2_arms.rs` pins both closed forms on both material
sides). The open-chain door (`admit.rs`, `AdmittedOpen::admit`) refuses them
because the surgery's open carve assumes planar supports and the trivalent
corner patch as its only termination. What this unit builds:

1. **Curved-support trimlines.** The strip on a cylinder support between
   the rim line and its trimline is bounded by a LINE along the ruling; its
   description is `TangentIntersection { band, support, witness }` (the
   band is tangent to the support along it), through the line arm
   `geom_brep::tangent_certificate_lane` already carries. Struts and chords
   on a curved support are on the surface exactly when they run along a
   ruling — they do here — so the scaffolding-door escalation the planar
   strut carries (`surgery.rs` `blank_phase`'s "NOT re-described at rest"
   note) does not arise: describe them.
2. **The transverse cut-off.** At each end of the requested edge, admission
   classifies the vertex `TransverseCap` (trivalent; the two unrequested
   edges share one plane face; the plane's normal parallel to the ruling —
   metered, `fillet3_cap_transverse`, escalate-never-guess; both supports
   meet it transversally, which follows). The carve: split the cap's two
   rim edges at the trimlines' feet (the ladder's `seam_split_param`
   spelling, one home), `mef` the arc of the cap's section of the band
   between the two feet inside the cap face — the corner region between
   the arc and the old vertex becomes a sliver face that dies with the
   support strips (the open band's own excise), the old vertex dies with
   it; the arc is described `Intersection { band, cap, witness }`. On a
   concave chain the same walk, the arc folded by `Convexity::signed`
   (H4's home): the band adds material and the cap gains the region.
3. **Admission widens exactly this far**: `AdmittedOpen::admit` takes a
   one-link chain whose arm is ruled OR plane–plane; a ruled link's ends
   must each be `TransverseCap` or (for plane–plane, as today) a
   fully-requested uniform trihedron. Everything else refuses as today.

**Ratified and not re-litigated:** ARMS3 A3-3 (the mid-curve run-out stays
reserved; this termination is a different situation — the supports end),
A3-2, BLEND-VOCAB V1–V4, H4's fold, the arms' closed forms.

## Phase 1 — measure before touching anything

Which bodies with ruled crease edges and perpendicular caps do the public
doors build today? Record, per attempt, the door reached and its verdict:

- a rod with a flat milled along it (`cylinder ∖ box`, the
  `CylinderPlaneCylinder` shape — two ruling edges, two perpendicular caps);
- two parallel cylinders of one height, overlapping, unioned
  (`CylinderCylinderCylinder`: the crease is a common ruling line; the
  boolean's parallel-cylinder pair — measure whether the union builds or
  refuses `CurvedPierceUnsupported`);
- a box (the plane–plane straight edge ending at perpendicular caps: the
  cut-off's non-ruled instance; record whether the arm-agnostic termination
  reaches it — see §Out of scope);
- for each body that builds: `fillet_edges` on one crease edge today, the
  refusal and its site; the arm's `EdgeBlend` for that link (spine, trim
  lines) checked by hand against the cross-section construction.

**Stop clause.** If NO fixture with a ruled crease and perpendicular caps
can be built through the public doors, the unit has no consumer and no
acceptance row: stop at the report and file what blocks each (the boolean
pair, most likely — its lane's ground), and the orchestrator parks the
item on that.

## Phase 2 — the change

1. Vocabulary above, in `blend/mod.rs`, `CornerConfig::policy` extended,
   Display sentences in the V1–V4 shape, `FILLET3_CORNER_RECOURSE` naming
   the new configuration as one that carves (E2's composed pin follows it).
2. `battery::corner_at` classifies `TransverseCap`; the new predicate
   `fillet3_cap_transverse` (the cap normal's departure from the ruling,
   levered by the link's own extent — the `Ruling::lever` the arm already
   names) joins the `fillet3_*` roster with its two-tolerance trio pin.
3. `AdmittedOpen::admit` admits ruled one-link chains; the corner plan
   grows a `TransverseCap` termination beside the trihedral patch; the
   blank phase's strut/trim mints describe curved-support edges.
4. Rows (`crates/sweep/tests/`, fixtures in `test_support`): the rod with a
   flat fillets (both ruling edges in one call; convex) — census delta,
   `validate_geometric` clean, `volume_pad == 0.0`, and the volume against
   the closed form **`ΔV = A_section · L`** (the band is a prism: the
   cross-section's fillet cut area — kite minus sector for the plane–plane
   twin, the circle-arc/line construction for cylinder–plane — times the
   rod's length; derive in the row's doc); the parallel cylinders if the
   boolean builds them (concave crease at the union's waist — the material-
   adding band on a ruled spine); the oblique-cap refusal typed with its
   detail; the trio pin for `fillet3_cap_transverse`; naming totality via
   `test_support::assert_naming_totality`; the interval twin; a mutant
   (the cut-off arc at the wrong radius or centre) red only on the new rows
   through tier 3.
5. Prose: `admit.rs`'s comment naming #987, README A3-3's "no consumer" and
   the corner recourse's "general run-outs are not implemented" clause
   (still true of the mid-curve run-out; say what IS built), KERNEL-VERBS
   row 59 (b); the item file's stale `work/issues/` pointer.

## Constraints, binding

- Every existing carve bit-identical to the merge base (the dump; the
  plane–plane open band, corners, ladder and annulus untouched in code
  where possible, and measured either way).
- No sampled normal decides anything; the one new decision is metered and
  trio-pinned; the cap plane is read from the stored surface.
- One home: the fold (`Convexity::signed`), the seam-split parameter,
  the strut mint; no copies.

## Acceptance

The Phase 1 table; the vocabulary ratified on the `[ev]` thread before
dispatch; the rod row at its prism closed form; the oblique refusal; the
trio pin; the dump identical; hosted CI green at the drawn point plus the
interval lane asked for.

## Out of scope

Mid-curve run-outs (ball-cap stop, feather-out — A3-3); oblique or curved
end faces (refused typed); junction carry-through (multi-link open chains);
widening the cut-off to plane–plane straight edges as a deliverable — if
the termination is arm-agnostic and a box edge reaches it in Phase 1, ONE
row may record that it carves, and the widening is otherwise its own unit.

## Review

v6 dual on the frozen head, claims to falsify:

- **C1** Every existing carve is bit-identical to the merge base.
- **C2** The rod's band is the arm's exact cylinder and the volume is the
  prism closed form (re-derive `A_section` independently).
- **C3** `fillet3_cap_transverse` is metered through the link's own lever
  and its trio pin is red-capable at every arm; an oblique cap refuses
  typed and names the reserved run-out.
- **C4** The curved-support trimlines and the cut-off arc are described
  exactly (tier-3 clean; no scaffolding-door escalation; the descriptions
  cite the band and the support/cap that carry them).
- **C5** The vocabulary is the ratified one and `CornerConfig::policy` maps
  it; no sentence still says the ruled arms have no surgery.
