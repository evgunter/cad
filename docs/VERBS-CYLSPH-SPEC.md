# VERBS-CYLSPH — the exact coaxial cylinder×sphere arm

**Ratified 2026-09-02** against main `69640aaba`, from the CYLSPH
premise survey (run the same day; premise table P1–P13 recorded in
the survey report, key rows restated below) and two rulings by Evan
(in-chat, 2026-09-02):

1. **SHAPE.** The unit is the exact coaxial classification arm plus
   the germ-frame arm, NOW. The fitted-chord join window — the only
   part of the historical "CYLSPH germ lane" that genuinely touches
   the fitted rung — is **DEFERRED, deliberately**: SPHSPH's
   measured precedent is that a new germ class flips no union
   (crossings die at `CurvedPierceUnsupported` a layer above the
   join, and #1291's ring-join arm is missing even for Line
   pierces), so building the window now would ship machinery with
   no reachable consumer. The deferral's door is named in the
   differential (below); this is a scope cut with a record, not a
   STOP.
2. **COAXIALITY IS DECLARED-ONLY** — never inferred from a measured
   `d ≈ 0`. The declaration-ladder precedent binds: VERBS-PIERCE
   put coaxial contacts in the ladder; CYLCYL's `RadiusEvidence` is
   never inferred; SPHSPH's polar gate is declared alignment +
   typed refusal. The eventual honest carrier for the declaration
   is #1372's parameter-identity channel (cite it at the type).

Historical scope note: the deleted `docs/VERBS-CYLCYL-SPEC.md`
(recover via `git show 5b7a092f9^:docs/VERBS-CYLCYL-SPEC.md`,
lines 95–104) said "CYLSPH runs LAST and alone: it is the only
fitted-rung lane … and must not drag that machinery into the exact
lanes' dispatches." That fence still binds in spirit — but its
"f64-only marcher" clause is STALE (M6-2 lifted it: `Pcurve::Fitted`
certifies at rest), and the `(Cylinder, Sphere)` route arm has been
`Rung::General, implemented: true` since M5 PR 7
(`geom_brep::ssi::cylinder_sphere_ssi`, marched + fitted with the
full three-limb certificate — survey P1/P2). This unit does NOT
re-do any of that; it adds the exact coaxial classification the
route note explicitly declines ("the coaxial circle special case is
not classified here — it is marched like any other configuration")
and the frame arm that consumes it.

## Opening measurement — BEFORE ANY CODE, in the PR body

Build the coaxial union fixture (a sphere threaded on a cylinder:
sphere centre on the cylinder axis, R > r so the walls cross) and
run `topo::union` at the unit's head. **Name the door that actually
refuses TODAY**, payload and raising site quoted — the candidates,
from the survey:

- the extent scan (P6): `FallbackExtentUnsupported`, "the
  cyl×sphere seam lane is not wired (its fitted-chord window has no
  azimuth analog)" — `boolean/ops.rs`, inside `sphere_extent_scan`;
- the crossing layer (P8): `CurvedPierceUnsupported` — the pierce
  ring lane covers Line×cylinder-wall only; a circle carrier
  against a wall and a sphere face both still refuse
  (`boolean/reduce.rs`, `curved_face_arm`);
- the germ frame (P3): `FrameError::NoArm` → typed
  `GermFrameUnsupported` (`boolean/join.rs`, `pair_section_frame`'s
  catch-all).

This measurement is the guard against repeating the falsified
CYLCYL PR-B premise (an arm spec that presumed crossings exist to
route). **The spec does not presume the union is reachable**; the
acceptance is contingent on what this measurement names (below).
Also measure the re-posed twin of the fixture (`transform_rigid`
off every axis plane) at the same head — the differential's
baseline.

## The unit

1. **`cylinder_sphere_section` in `crates/geom-brep`** (beside
   `sphere_sphere_section` and `plane_torus_section`; cite by
   anchor, not line — `intersect.rs` is in flux while C5ARMS
   merges). The exact coaxial classification, `T: Decide`-generic,
   atan2-free:
   - **`TwoCircles`** — radius `r` (the cylinder's), centres at
     axial stations `±√((R−r)(R+r))` from the sphere centre — the
     FACTORED form, not `R²−r²`, for interval tightness (the
     `sphere_sphere_section` precedent). Both circles share the
     cylinder axis; the enum carries centre/axis/stations.
   - **`TangentCircle`** at `R = r` — CLASSIFICATION DATA (C7
     lineage), one circle at the equator station. This arm must be
     CONSISTENT with the SSI's own tangency door: `ssi.rs`'s
     `ssi_cs_tangency` trilean already refuses the tangent pose
     toward C7 (`TransversalityBand`; pinned at
     `m5_pr7_ssi.rs:857-885` — survey P12). The section must NOT
     double-adjudicate the same pose differently — same margin
     shape, same verdict class, and the doc says so.
   - **`Empty`** for `R < r` (the sphere never reaches the wall).
   - **Gate: a DECLARED coaxiality evidence parameter** — define
     `CoaxialEvidence` as the `RadiusEvidence` sibling (a
     structural declaration type, never constructed from a measured
     distance; doc cites #1372 as the eventual production carrier).
     **No production caller can supply it yet — acceptable and
     STATED**, per the GERMARMS PR-2 precedent (an arm whose
     declaration channel does not exist refuses typed at the
     consumer; the arm itself is pinned by direct tests).
   - **Named trileans in gate order with per-branch division/sqrt
     arguments** (the house style): the degeneracy guard FIRST —
     and it guards the FULL convention, `r > 0` AND `R > 0`, not
     just the relation between them. This inherits ordinal 111's
     MAJ-1 verbatim: C5ARMS' `pt_ring_guard` decided only `R − r`
     while documenting `R > r > 0`, and the named threat (a
     pre-validate STEP-minted operand) walked through it. Then the
     crossing/tangency/empty trilean on `Margin::of(R − r)` (a
     length), `Zero ⇒ TangentCircle`. The sqrt's factors are
     definitely positive in the `TwoCircles` branch by that
     trilean's `Positive`; say so at the site.
2. **The `pair_section_frame` `(Cylinder, Sphere)` arm** consuming
   item 1. **One frame serves both circles**: the two coaxial
   circles share the cylinder axis, and the arc-facing sense
   `axis·((p−c)×dir)` is invariant to sliding the centre along that
   shared axis — so unlike CYLCYL's crossing ellipse pair (which is
   why `GermFrameCylinderPinch` exists), no pinch door and no
   second frame are needed. State the argument at the arm.
   Non-declared and non-coaxial poses keep `FrameError::NoArm`
   VERBATIM (the general quartic is the marched rung's, and the
   join window for it is the deferred half).
3. **The #974 tangent-locus circle arm — CONDITIONAL.** #974's
   coaxial `TangentLocus::Circle` arm is blocked on its residual
   one-sign story: "residuals one-signed in OPPOSITE orientations
   per direction" (`boolean/rest.rs` contract sentence;
   `boolean/reduce.rs` covered-lane precondition — survey P11).
   IF the opening work shows that story resolves cleanly with the
   declared-coaxial classification in hand, deliver the arm and
   retire #974's blocker sentence. If not: a DECLARED deferral
   quoting the measured story, with #974 left open and cited — not
   a silent skip.

## Refusal-text rows that MUST move (the C5ARMS R2 MAJ-2 lesson)

Ordinal 111's R2 found four in-repo sentences still asserting the
pre-flip world after C5ARMS flipped its flag — the exact failure
mode `memories/refusal-text-is-not-cause.md` instance 7 records.
This unit's equivalents, each of which either MOVES with the change
or is RE-VERIFIED still true with the verification stated in the
PR:

- the route note's "the coaxial circle special case is not
  classified here — it is marched like any other configuration"
  (`intersect.rs`, the `(Cylinder, Sphere)` arm) — retired by item
  1; the replacement note names what IS classified (declared
  coaxial) and what still marches (everything else);
- `CurvedBooleanUnsupported`'s doc + Display naming "no cyl×sphere
  azimuth-window analog" (`boolean/mod.rs`, anchors near the
  `855-870` and `1112-1123` regions at survey time) — the deferral
  keeps these TRUE (the window is still absent), so they are
  re-verified and said so, with the deferral cross-referenced;
- `CurvedPairUnsupported`'s Display "a cyl×sphere fitted-chord
  window has no window analog to read" (`boolean/mod.rs`, anchor
  near `1211`) — same treatment.

## Fences

- `m5_s13_pips.rs`'s cyl×sphere arm and its message stay VERBATIM
  unless the certificate that retires them lands — this is the row
  SPHSPH's spec explicitly protected ("No CYLSPH work: …row 8's").
  If the opening measurement's door turns out to be the extent scan
  and this unit retires it, the row moves WITH the retiring
  certificate and the PR says so; otherwise it does not move.
- No `BlendArm::CylinderSphereTorus` / roster-string changes (the
  fillet family is separate in both directions).
- The Nurbs edge-gate consequence STAYS: a fitted-rung result body
  carrying certified Nurbs edges is not a boolean operand
  (`gate_operand_edges`) — say so, don't fix it.
- No cone or torus work. No changes to `cylinder_sphere_ssi` or its
  suites (the σ₂-sliver and tangency rows keep refusing).
- The SSI tangency refusal stays the C7 door (item 1's consistency
  clause, not a re-adjudication).
- `docs/KERNEL-VERBS.md`'s curved-boolean breadth row syncs if a
  class lands; the register's germ-class sentence likewise.
- The fitted-chord join window is NOT built (Evan's shape ruling);
  the differential names its door honestly.

## STOP conditions (pre-registered)

1. **If the opening measurement shows the coaxial union dies at a
   door this unit's arms cannot retire AND the acceptance presumed
   it — re-cut, don't build.** (The CYLCYL PR-B lesson: the
   falsified premise was exactly "the arms move the walls.")
2. **If the declared-evidence type cannot be defined without
   widening a public type beyond the `RadiusEvidence` shape — that
   is a design widening this spec deliberately does not authorize;
   STOP for adjudication.**

## Acceptance — CONTINGENT on the opening measurement (the GERMARMS PR-2 pattern)

- **If the coaxial union is reachable** once items 1–2 land: it
  completes, validates tier-3, and meters against the closed form
  (sphere-through-cylinder union volume, derived in the PR); census
  pinned; the refusal rows that named the pair retype or retire per
  the refusal-text section.
- **If it is not reachable** (the expected outcome per the SPHSPH
  precedent): the classification and frame arms are pinned by
  DIRECT tests at three poses (crossing / tangent / empty, plus a
  declared-vs-undeclared differential showing `NoArm` without the
  evidence), and the union's refusal is RETYPED to its honest door
  — never a desync, never silence — with the payload in the PR.
  That outcome is a deliverable.
- **Either way the re-posed twin rule binds**: every acceptance row
  has its `transform_rigid` twin asserted to behave identically
  (same answer or same-typed refusal). A green direct-pose row with
  a differing re-posed row is a MAJOR by construction.
- Differential rows: the non-coaxial transversal pose still marches
  (the SSI suites untouched and green); the undeclared coaxial pose
  refuses `NoArm`; the deferred join window's door is named with
  its payload ("what would retire it" stated, the `verbs_shell`
  precedent).

## The bit-identity block

The conservatism and closed-form pins that must not move: the
`verbs_cylcyl_probe` rows (the spec block GERMARMS carried —
coaxial boss, bracket rounds, six-millimetres, the fully-crossing
typed refusal, standing-clear), `m5_pr7_ssi` + its adversarial
suite, `m5_s13_pips` (both sphere-class scan rows), the
`route_inventory` rows for every pair this unit does not touch,
and TORAX's `torax_axial` closed forms. A moved row is a change to
its argument and needs its own adjudication.

## Lane obligations

`docs/prompts/implementer-discipline.md` binds. No Co-Authored-By
trailer (blinding). **Suite evidence via hosted CI verified at the
STEP level** — read the change filter's own output for
LANE/EPS/KLINT_ROW and CONFIG_SOURCE; a green job name is not
evidence; local compute is for targeted suites, probes, and
mutations only (Evan's 2026-08-31 method ruling). Merge origin/main
before opening the PR; watch for the silent-run faces; note
inherited main reds and distinguish them from yours. Opening
measurement in the PR body before code, payloads quoted; every
deviation declared with an argument and a schedule.

**Pre-logged difficulty: M.**
