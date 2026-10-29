---
id: import-normalizes-the-rim-only-cap
kind: issue
title: STEP import re-mints a rim-only sphere cap into the seamed form, as a reported normalization
status: closed
opened: 2026-09-18
refs: [rim-only-sphere-cap-panics-at-census]
priority: P0
cost: M
closed: 2026-10-29
pr: 6007
branch: foreign/pole-vertex
---


Filed by the TESS orchestrator from Ev's ruling on `[ev]` PR 2850
(2026-09-18, in chat): **a chart singularity inside a face is a vertex
of it** — a loop of rims only, with the sphere pole (or cone apex) in
the face's interior, is not a face of this kernel; import re-mints it
and validity refuses it. Deliberately not in DESIGN.md (Ev: provisional,
"too much weight" there); this unit states it in `NormalizationKind`'s
docs, present tense.

A STEP sphere cap stated with one latitude circle and no meridian — as
two half arcs or as ONE closed circle edge — imports today as `Solid`,
unnormalized, and is the face the ruling says is not one. Import owes a
fifth `NormalizationKind` beside `EdgeFreeSphere` and
`DegenerateApexCone`: re-mint as two half-caps on meridians through a
pole vertex (the form revolve and the boolean plane cut mint), reported.
Which pole is the rim's traversal against the face's sense — PR 2741's
`rim_interior_side` is that decision, written once. The cone apex cap
with one rim is the same statement on a cone.

Fixtures: `crates/step-import/tests/fixtures/tess-cap-diag/`
(`rimonly1.step`, `rimonly2.step`, `gen_cap.py`) on branch
`tess/rim-only-cap-diag` at `83833e586`; TESS-1 lands them on main with
`mesh`'s typed refusal, which is what such a body meets until this
lands. Order: this before TOPO's validity rule
(`work/restfront/validity-refuses-an-interior-chart-singularity.md`), or
import starts refusing files it accepts today.

Signed: (TESS orchestrator)

**Fixtures landed (TESS-1's lane, 2026-09-18).** On TESS-1's branch the
two files are `crates/step-import/tests/fixtures/rim-only-cap/`
(`rimonly1.step`, `rimonly2.step`, generator `gen_rim_only_cap.py`),
not the survey branch's `tess-cap-diag/` path, and the row that reads
them is `crates/step-import/tests/meridian_free_cap.rs`. That row
asserts the body is adopted AS STATED (its edge count, tier 3 `Ok`, the
closed-form volume) before it asserts `mesh`'s refusal, so it goes red
the day this normalization lands — which is the row telling this unit
to turn it into the normalization's own.

**Two more notes from TESS-1's fix pass (2026-09-20).** (1) The
survey's third fixture, `seamed.step` (the OCC-style statement: a
closed rim plus one seam meridian to a pole vertex, used twice by the
sphere face), was deliberately NOT lifted to main. It is not a mesh
row: import refuses it today (`TierInvalid`,
`ScaffoldingStrutVertex`, the valence-1 pole), and whether that
statement should be accepted, normalized or stay refused is this
unit's question. It and its generator arm are on branch
`tess/rim-only-cap-diag` at `83833e586`
(`crates/step-import/tests/fixtures/tess-cap-diag/`). (2) The notice
that these rows move by design now sits where this unit's lane will
meet it: the header and the edge-count assert message of
`crates/step-import/tests/meridian_free_cap.rs`, the comment over the
two `rim-only-cap` rows of `tier_gate.rs`'s corpus table, and the
generator's docstring.

**What props reads of the cap now (FLUX `flux/sphere-arm`,
2026-10-10).** The sphere arm reads every face by Gauss–Bonnet, and its
side reader reads a one-loop face as the side of its loop holding no
chart pole. A rim-only cap has a pole on each side, so the reader
answers `Unencoded` and the cap measures under its bit, as before; the
refusal stays validity's
(`work/restfront/validity-refuses-an-interior-chart-singularity.md`).
Which pole the re-minted half-caps meet at no longer needs
`rim_interior_side`, which is gone: it is the pole on the side of the
rim the face's bit names.

## Designed (FOREIGN designer pair, 2026-10-29; two rounds, converged)

What import does is settled; the pole's normal form is not, and is not this row's (below).

- **The trigger is pole incidence, not "the loop has no meridian".** A singular point of a sphere or cone face's chart (a pole, the apex) that lies in the face's closure and is not a vertex of it. "In the closure" is read off the face's region (its loops under the sense bit, Gauss–Bonnet / props' pole-side reading), so a tilted cap and a ball cut through its side and split at the equator (`seam-region/side_cut_equator_split.step`) are in, and a meridian spur that stops short of the pole does not take a face out.
- **Three dispositions, one `NormalizationKind`** (its doc states the rule; it carries which disposition it found), reported, never silent. `DegenerateApexCone` retires into it. `EdgeFreeSphere` stays its own kind (no loop at all is a different defect).
  - (a) the singularity is interior: mint the vertex and the pole form, its meridians run from the singularity to the first boundary edge each meets, that edge split there and the split propagated to every face sharing it (as the band mint does);
  - (b) it lies inside a boundary edge (`halfcap_nosplit.step`): split the edge there;
  - (c) it is a valence-1 vertex on one meridian (OCC's `seamed.step`, `freecad/cone_apex.step`): complete the pole form from it (today, the half-turn twin: `apex_cone`'s algorithm, generalised to the sphere).
- **Refused typed only** where the pole is within ε_in of the loop and decidably neither on nor off it, where a meridian would cross a boundary carrier no lane reads (not Line/Circle), where `chart_direction` cannot read a winding, and on an orientation-inverted face, each naming its lane.
- **The pole form has one home.** Today revolve's `build_wire`, `normalize.rs`'s `apex_cone` and `entities::edge_free_face` each mint it. Import works on `SolidSpec` before a body exists, so the home is a shared description of the form (the azimuths, the meridian carriers) that all three cite, not a `topo` call. The door mints **the kernel's current form, two half-turn meridians**, from it.
- **Not this row: the form itself.** Both designers lean to a one-face form (a pole reached by one wrap meridian, a vertex with one edge), which would change tier 2's valence-1 ban, revolve's two π-bands and the sweep README's minimal sphere. That question is `work/carve/a-full-revolves-curved-wall-carries-two-seam-meridians-nobody-wrote.md`'s, put to Ev with a probe of the consumers a one-face pole meets. If Ev picks it, the shared description flips and import's target flips with revolve's.
- **Fixtures:** a FreeCAD `Part.makeSphere(..).cut(box)` cap through `crates/step-import/tests/fixtures/freecad/gen.py` if FreeCAD is reachable (it shows which statement OCC really writes, including a possible degenerate pole edge, a fourth statement of the same defect); the hand-made `rim-only-cap/` files; `seamed.step` lifted from `tess/rim-only-cap-diag` at `83833e586`. Rows that move by design: `meridian_free_cap.rs`, `tier_gate.rs`'s rim-only and halfcap rows, `poles_inside_a_seam_region.rs` (normalized now, not refused), `loops_the_meridian_guard_admits.rs`'s doc, `MeridianFreeCurvedFace`'s recourse text, and the `entities.rs` face-gate message naming a "sphere-zone band" re-mint that does not exist.
- **Ratified text:** DESIGN `[topology]`'s seam clause gains its second exception, "or where a chart singularity lies inside a face, which is a vertex of it" (the rule is ratified in `crates/geom-brep/README.md` C12 (7), [ev] PR 4549). That sentence is an Ev decision and rides the `[ev]` PR with the form question, not this row's unit.

## Closed (PR 6007, FOREIGN implementer, 2026-10-29)

Built to the `## Designed` spec: `NormalizationKind::ChartSingularityVertex { found }` (`Interior`, `InsideEdge`, `ValenceOneVertex`; `DegenerateApexCone` retired into it), the pass in `crates/step-import/src/normalize/singularity.rs`, the shared description `geom::pole_form`, cited by revolve's `build_wire`, `entities::edge_free_face` and the pass. The row reading it is `crates/step-import/tests/chart_singularity_vertex.rs`. FreeCAD was not runnable; OCC 7.8.1 (through OCP) writes the cap as the seamed statement, a seam to a valence-1 pole with no degenerate pole edge (`fixtures/occ/`). Residue filed: `a-sphere-with-a-hole-stated-as-a-vertex-loop-refuses`, `the-coherence-channel-has-no-shipped-finding`, `the-join-refuses-whole-row-lost-its-import-witness`.
