# MESH-8 — issue 868: the coherence-detector relocation

**Binding at dispatch** (S-MESH program, `docs/S-MESH-PLAN.md`;
difficulty logged pre-draw: **M/L, recorded numeric M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
Issue 868 is the primary specification, bounded by the S-MESH Q2
ruling (`docs/S-MESH-PLAN.md` §Rulings — Ev, in-chat, 2026-09-01:
**option (d), relocation**). Read the ruling verbatim, including the
kept-as-record mesh-local alternative it declined. SMELL S115(d) is
the disclosure this closes. Read `crates/mesh/src/walk.rs`'s
`gap_is_noise` doc ("What the three DETECTORS are, under D2") and
`closing_column`'s ledger in full; they are the site's own honest
account.

## Situation

`walk.rs` carries three `debug_assert!` DETECTORS that gate nothing
and are reachable by input: `closing_column`'s loop-closure detector
and `loop_polygon`'s two iso-side continuation detectors. Each
measures a gap against a lever arm, in metres, against ε — a fact
about the BODY's data (carrier vs. vertex coherence), not about the
mesh, and computable from the body alone with no mesh state and no δ.
By D9's D2 addendum they are row 1/3 territory wearing row 5's
clothes; the file says so. The ruling: the conditions move to the
body's own examination lane as a NON-GATING findings report, and the
mesh-side asserts are DELETED — the tessellator stops being a lint for
other people's data and `tessellate`'s signature never changes.

Two things have happened since the ruling that this unit inherits:

- MESH-7 (issue 1571) showed `closing_column`'s detector is today the
  only thing in tree that notices a pole-crossing great-circle arc
  (its carrier-midpoint azimuth and its closing vertex disagree by
  π). That is an ARC-PREMISE defect, not a coordinate-quality one,
  and issue 1571 owns fixing it — but the relocated condition must
  still FIRE on that body, or the relocation silently loses a witness.
- The half-cap of issue 723 (`crates/geom-brep/tests/cert1_sphere_polar.rs`
  header) is the other recorded π-rad witness.

## FIRST, before the build — the door decision, reported

The ruling makes the door the unit's first recorded question,
"answered by the dependency graph and the finding's audience". Write
the answer and REPORT it before building:

- Candidates: a tier-adjacent examination in `topo` (beside
  `validate` / `validate_geometric`, the shape `ContactFinding` /
  `FlushFinding` already use: a `pub fn examine_*(body, band) ->
  Vec<Finding>` that never refuses), or `step-import`'s diagnostics
  (the door where defective source coordinates actually arrive).
- Decide by: who can SEE the data (the conditions need the body's
  carriers and vertices, nothing more); who can HEAR the finding (an
  importer's user; a modelling user whose own sweep cannot produce
  the defect); what the dependency graph allows without a new edge
  (`mesh` must not gain a `step-import` dev-dependency; `topo` must
  not learn about STEP); and where a finding lands in `editor-core`'s
  `ChecksReport` / `CheckFinding` shape if it is to reach a GUI.
- Name the payload: one typed struct for all three conditions (a
  gap, a lever arm, the metres it opens, the ε it was judged at, the
  face/edge/vertex keys, which condition), not a string; state its
  determinism in (body, ε) and that it is NOT mesh bytes (D9's
  contract covers the mesh, not the diagnostics — say so where D9 is
  cited).

If the answer needs a new crate edge or a new public surface on a
crate outside this unit's fence, STOP and report.

## Deliverables

1. **The body-side coherence examination**, at the door decided
   above: the three conditions re-derived from the body alone (the
   closure gap between the closing vertex's azimuth and the carrier's
   own; the two iso-side continuation gaps between consecutive
   sub-edges of one side), banded exactly as `gap_is_noise` bands them
   today (spatial bar, per-axis lever arms `Chart::radial` /
   `Chart::v_lever` or their topo-side equivalents — state which
   lever, from where), returning a findings report that GATES NOTHING.
   Bodies that mesh today keep meshing; nothing panics; nothing
   refuses that did not refuse before (the issue's non-scope).
2. **Each relocated condition firing on the same witness the mesh
   assert would have caught** — red-first per condition: (a) a
   synthetic sub-ε closure wobble (the S22 class; the walk's own
   synthetic row is the model); (b) an iso-side continuation wobble
   on each axis; (c) the pole-crossing great-circle arc (issue 1571's
   body, `mesh7r1_probes`' builder) and issue 723's half-cap — the
   examination REPORTS them (a finding naming the π-rad gap) while
   issue 1571 stays open for the fix. Show the mesh assert firing on
   the same body BEFORE the deletion (recorded in the PR) and the
   finding after.
3. **The three mesh-side `debug_assert!`s DELETED**, with their
   ledger text moved to the examination's doc (one home) and
   `walk.rs`'s "What the three DETECTORS are, under D2" section
   rewritten to say where the conditions live now; `gap_is_noise`
   keeps its one remaining consumer (`entries_off_bbox`, the
   walk-consistency check) — its doc's consumer list shrinks to the
   truth. `walk.rs:803`'s ledger (two π-rad witnesses) moves with the
   condition.
4. **D9 / behaviour**: mesh bytes identical (MESH-4's two-build
   digest at the three ε rows over the tour corpus and the suites'
   bodies — the detectors gated nothing, so nothing may move); state
   that the findings report is NOT covered by the byte contract and
   why.
5. **ε posture** (issue 1356): the examination's band is stated per
   band; the mesh crate's ε inventory pin loses the detector reads —
   re-form it honestly; three-ε battery; trailer decision argued (the
   conditions decide at the band; the interval lane can differ — ask
   for it or say why not).
6. **Class sweep** (discipline §5): every other `debug_assert` in
   `crates/mesh` reachable by input (the S65 censuses are row 5 by
   ruling and stay; MESH-3's pole guard is issue 896's and stays —
   disposition each), and every other lane's input-quality
   `debug_assert` the ruling's "would dominate other lanes'
   debug_asserts too" sentence pointed at — enumerate, do not act.
7. **Issue 868 CLOSES at this merge**; SMELL S115(d)'s disclosure is
   the orchestrator's to update at merge — say so in the PR.

## Acceptance

- The examination at the decided door with its typed findings; the
  three conditions each demonstrated on a witness, red-first; the
  mesh asserts gone; D9 digest identical; hosted CI green; gate record
  per head; the door decision reported BEFORE the build.

## Hard rules

- NO `Co-Authored-By`, no model names. "issue 868" spelled out, no
  closing keywords.
- Scope fence: `crates/mesh` (walk.rs, sizing.rs's inventory pin,
  suites); the examination's home crate (`topo` or `step-import` per
  the door decision) — a NEW examination module and its findings type
  only, no change to `validate`'s tiers or to any refusal; the
  `editor-core` checks surface ONLY if the door decision routes a
  finding there and then minimal arms. NOT: `walk.rs` classification
  decisions, the S65 censuses, issue 896's guard, issue 1571's fix
  (report it firing; do not fix the arc premise), `props`. `topo`'s
  boolean is S-BOOL's; Track V rows in `editor-core` — disclose any
  row's file reached.
- Re-merge main before opening the PR.
