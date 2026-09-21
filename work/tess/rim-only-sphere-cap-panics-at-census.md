---
id: rim-only-sphere-cap-panics-at-census
kind: issue
title: mesh: tessellate panics at the issue-897 census on a rim-only sphere cap the shape door admits (even at f = 0)
status: open
opened: 2026-09-03
github: 1615
refs: [897, 1571]
priority: P0
cost: H
---

## From GitHub issue 1615

Opened 2026-09-03; 0 comments.

**Found by:** MESH-12's measurement pass (issue 1588's rim-continuation witness), filed by the S-MESH orchestrator as a forward observation. MESH-12 does NOT touch the walk; this is out of its fence.

**What was measured.** A sphere face bounded by a single rim-only loop (a latitude circle, no meridian edges, no pole crossing) passes the shape door — `require_iso_rectangle` admits it, and `require_one_chart_branch` has nothing to refuse since no traversed arc crosses a pole — and then `tessellate` panics in the cross-face identification census (`crates/mesh/src/tessellate.rs`, the issue-897 census) for every such cap tried, including the trivial one at f = 0 (the rim at the equator). The panic is the census doing its job: the emission for a rim-only cap is not a manifold patch by the census's count. So this is the same class as MESH-11's half-cap and bow-tie findings (a debug build panicked at the census at δ = 0.5), except that here the door does NOT refuse the input, so the walk reaches a shape it cannot emit.

**Pre-existing.** MESH-12's branch changes nothing on this path; the measurement was taken to locate the rim-continuation witness (issue 1588) and found that the only on-surface construction reaching `RimContinuation` is the Euler-door two-level rim, which constructs and which `examine_chart_coherence` reports on at 1.5ε / 1.9ε. The import route to the same state is dead at all ε (`pcurve_loop_continuity`, `crates/topo/src/pcurves.rs`, refuses at R·Δv ≥ ε before props decides).

**What the unit that takes this has to decide (design surface, not a fix-in-place):**
1. Is a rim-only sphere cap a shape the mesh lane admits at all? If yes, the walk needs a cap emission (the pole is interior to the face and no meridian edge names it) and the census must pass on it. If no, the shape door must cite a predicate that refuses it structurally — never by inspecting values — and the refusal must be a `NotIsoRectangle`-style typed door, not the census panic.
2. Whichever way, CERT-1's rows and `mass_properties` are untouched: this is emission, not measurement.
3. A debug-only census panic reaching a door-admitted shape is a fail-loud gap in the release build too: the release build emits the non-manifold patch silently. The unit should measure what the release build emits for the f = 0 cap and record it.

**Fixture.** MESH-12's PR (branch `mesh/12-saturated-span`) discloses the exact construction it used; the unit taking this should lift the fixture from there rather than rebuild it.

Band: S-MESH (1200–1299). Not scheduled; parked behind MESH-12.

## Home

`work/mesh/` — the panic is in `crates/mesh/src/tessellate.rs`, an S-MESH territory glob, and the issue names the S-MESH band and parks itself behind MESH-12.

## Re-homed at S-MESH's exit (2026-09-16)

Moved from `work/mesh/` to TESS (opened at this exit as S-MESH's successor for the tessellation kernel) when S-MESH closed (`docs/S-MESH-EXIT-WALK.md`); the item's content, id and history are unchanged.

## Measured, 2026-09-18 (`tess/rim-only-cap-diag` at `83833e586`) — and three corrections to the text above

Survey lane, default ε, local; rows in `crates/mesh/tests/tess_cap_diag.rs`
and `crates/step-import/tests/tess_cap_diag_import.rs` on that branch.

**Corrections.** (1) "f = 0" above is MESH-12's gap factor `R·Δv = f·ε`
(one rim circle), not the equator; the panic has no latitude or pole
dependence. (2) "The import route is dead at all ε" is stale since PROPS'
sphere-pole-side (PR 2741, 2026-09-16): a STEP rim-only cap + disc —
stated as two half arcs or as ONE closed circle edge — imports as
`Solid`, passes tiers 1–3, measures the exact closed-form volume, and
then panics at the census. `pcurve_loop_continuity` refuses only a rim
v-JUMP. (3) "The release build emits the non-manifold patch silently":
this workspace's `[profile.release]` sets `debug-assertions = true`, so
in-repo release PANICS too; with assertions off `tessellate` returns
`Ok` with **zero triangles for the face** — a hole, not a non-manifold
patch (`check_mesh` = `Err(BoundaryEdge)` beside a disc, `Ok(())` on a
two-cap sphere: `check-mesh-passes-the-empty-mesh`).

**Mechanism.** `walk::loop_polygon` classifies every arc `Rim`; with no
meridian the polygon has one v value bitwise (n = 32, `area2 = 0`,
`poles = 0`), `require_swept_rectangle` passes (every entry is on its
zero-height box), `grid_counts` gives `nv = 1`, the CDT gets collinear
points, `inner_faces = 0`, and `tessellate_curved` returns an empty
patch. `walk_anchor`'s doc calls the meridian-free loop "unreachable
through `traversals`"; it is reached. The census is correct; its
message's stated cause ("the faces meeting on that edge did not
identify it") is not the cause here.

**It is a class**: any curved face whose single loop is rims only.
Measured the same on a cone apex cap and a one-rim cylinder face (both
door-admitted; props refuses both `DegenerateFace`, so neither is
tier-3 valid — `work/props/cone-apex-cap-refuses-degenerateface.md`).

**No native verb mints the face.** Revolve (ball, dome), `boolean_op_with`
plane cuts and `merge_coplanar_faces` all keep the seamed form — two
half-caps on meridians meeting at a valence-2 pole vertex (DESIGN.md's
V2/E2/F2 minimal sphere) — and `merge_coplanar_faces` refuses
`PeriodClosure` rather than make the full-wrap loop. The reach is STEP
import and the Euler door. Import already normalizes four seamless
statements into the seamed form (`NormalizationKind::{EdgeFreeSphere,
DegenerateApexCone, FullPeriodTorus, SeamlessPeriodicBand}`); there is
none for the rim-only cap, and the one-face seamed statement is refused
(`ScaffoldingStrutVertex`, the valence-1 pole).

**The rest of the kernel accepts the sphere cap**: tiers 1–3 `Ok`,
`mass_properties` exact, `examine_chart_coherence` silent. The doors
(`geom_brep::props::curved`) decide the pole side structurally —
`rim_interior_side` (σ), `sphere_rim_only_pole_level`,
`require_rim_only_closed` — all private; the public surface is
`boundary_material_sign` → `MaterialSign::Unencoded`.

## Disposition

- **TESS-1** (specced): the class refuses typed at the walk, on the
  structural fact (no meridian traversal). True under either answer
  below — the cone and cylinder members have no other future.
- **The design question this row opened with is Ev's** (an `[ev]` PR
  carries it): does the mesh lane learn the interior pole and emit the
  sphere cap, or does import normalize the statement into the seamed
  form and TESS-1's refusal stand for good? This row stays open on it.

Doc rot found by the survey, in files that are not TESS': the module
header of `crates/topo/tests/mesh12_rim_row_reach.rs` still says the
flux lane refuses the rim-only cap while its own third row measures it;
`crates/step-import/tests/poleguard.rs`'s door enumeration reads as
"every sphere-face boolean cut refuses" while macroscopic `ball ∩ slab`
cuts succeed. TESS-1 fixes the first (the file is MESH-12's row);
the second is filed on its owner's slate by TESS-1's lane.

## The question for Ev (2026-09-18)

**Is a sphere face with a pole in its interior — bounded by one
latitude circle, no meridian, no pole vertex — a face of this kernel?**
Today the kernel answers both ways: tiers 1–3, `mass_properties`
(exact, since PR 2741) and STEP import say yes; every native verb, the
walk's premise and DESIGN's "poles have valence 2" minimal sphere say
the canonical form is the seamed one (two half-caps on meridians
through a pole vertex). TESS-1 makes `mesh` refuse the face typed in
the meantime, so nothing is wrong while this is open.

**(E) It is a face: `mesh` learns the interior pole and emits the cap.**
The lane synthesizes what the loop does not carry — the pole row, and
one interior seam column whose two sides share vertex ids — deciding
which pole and that the rim closes from props' existing structural
predicates (`rim_interior_side`, `props_rim_only_closed`), which PROPS
would make public. The refusal survives for the members with no
interior to emit (one-rim cylinder; the cone apex cap until props
admits it). Every other consumer of a sphere face — booleans, fillet,
shell, offset, export — then owes the same answer sooner or later;
none has been probed.

**(N) It is not: the seamed form is the only sphere-cap face.** STEP
import gains a fifth normalization beside `EdgeFreeSphere` and
`DegenerateApexCone` (re-mint the rim-only cap as two half-caps,
reported), tier 2 or 3 refuses the rim-only statement from the Euler
door by a structural rule, and PR 2741's rim-only arm in props retires
as unreachable. One form, decided at the two doors that can state the
other; no downstream lane ever sees it.

**Recommendation: (N).** "Define what a sphere face is" has one
structural answer already written down — an iso-rectangle whose poles
are vertices — and every native producer keeps it; import's standing
practice is to re-mint seamless statements into the kernel's form
rather than teach each lane a second one; and (E)'s real cost is not
the mesh lane but every other consumer, unprobed, each of which would
otherwise discover the face the way `mesh` did. Against it: (N) deletes
two-day-old PROPS work that is correct and exact, adds a validity rule
(a DESIGN change, tier 2's or 3's), and a seam-free exporter's cap
round-trips with two more faces than it arrived with — reported, but
changed. The half-measure — normalize at import and leave the Euler
door's statement valid-but-unmeshable — is the one I would not take: it
keeps two forms legal and makes one of them a permanent refusal.

Evidence and numbers: §Measured above; `work/tess/log.md`.

## Ruled: (N) (Ev, in-chat, 2026-09-18)

The seamed form is the only one: a chart singularity inside a face is
a vertex of it. Provisional by Ev's own word, and kept out of DESIGN.md
for that reason; the rule is stated on the EXCH and TOPO rows and lands
in those doors' docs. (E) is tabled with a
pointer at the code (N) retires:
`consider-emitting-the-rim-only-cap-instead-of-normalizing-it`. The
work is filed where it lands — import's normalization on EXCH, the
validity rule (and the props arm's retirement) on TOPO — and `mesh`'s
part is TESS-1, whose refusal stands for good. This row closes with
TESS-1.
