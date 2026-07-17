# M1 Work Order — Topology + Euler Operators

**Status: DRAFT — awaiting Evan's ratification.** No implementation
until this plan is signed off (it is itself a design PR, like M0-PLAN).

Read `DESIGN.md` first — it is the ratified contract; M1 leans on D1
(arenas + Euler operators, manifold-first), D5 (provenance from birth),
D9 (determinism, no panics), and Q1's genericity boundary (`Body<T>` =
scalar-free topology + `T`-valued geometry). M1's goal (Roadmap): the
topology layer becomes real — half-edge adjacency plus the
Euler-operator construction discipline; **build a cube by hand;
watertightness and Euler–Poincaré checks pass.**

Primary source: Mäntylä, *An Introduction to Solid Modeling*, ch. 9–11
(read 2026-07-16). Detailed reading notes live in
`<main-checkout>/references/notes/mantyla-ch{9,10,11}-*.md`
(git-ignored, persistent) — implementer and reviewer prompts cite the
notes, never the scan.

## Source grounding (the facts the plan stands on)

- **Euler–Poincaré (Mäntylä eq. 9.2):** `v − e + f = 2(s − h) + r`,
  with s = shells, h = holes (genus), r = rings (interior loops of
  faces). Necessary, not sufficient, for validity.
- **Operator catalog** (effect on `(v, e, f, h, r, s)`):

  | Op | Δ | Semantics (one line) |
  |---|---|---|
  | `mvfs` | (+1, 0, +1, 0, 0, +1) | skeletal body: face with one empty loop holding a lone vertex |
  | `mev`  | (+1, +1, 0, 0, 0, 0) | split a vertex's edge fan; new edge joins old→new vertex |
  | `mef`  | (0, +1, +1, 0, 0, 0) | split a loop by joining two of its vertices; new face |
  | `kemr` | (0, −1, 0, 0, +1, 0) | kill an edge occurring twice in one loop; split-off piece becomes a ring |
  | `kfmrh` | (0, 0, −1, +1, +1, 0) | kill face f₂, demote its bounding loop to a ring of f₁ (same shell ⇒ +1 genus; cross-shell ⇒ merges shells instead) |
  | `kvfs`, `kev`, `kef`, `mekr`, `mfkrh` | negated | exact inverses of the above |

- **Algebra (§9.4.2):** the make-direction five are a basis of the
  hyperplane 9.2; minimal counts for a connected solid are
  `v−1` mev, `f+h−1` mef, 1 mvfs, `h` kfmrh, `r−h` kemr. Connected
  solids never *need* kev/kef/kvfs/mfkrh, but solids with `h > r`
  strictly require `mekr`. Soundness (§9.1.3, §9.4.1): every operator
  sequence from `mvfs` preserves 9.2 and cannot reach nonorientable or
  otherwise topologically invalid structures; completeness: every valid
  structure is reachable.
- **Acceptance-test sequences (§9.3):** plain cube = 1 mvfs + 7 mev +
  5 mef (v8 e12 f6, provably minimal). Box with a through-hole =
  1 mvfs + 15 mev + 10 mef + 1 kemr + 1 kfmrh (v16 e24 f10 r2 h1,
  minimal).
- **Half-edge structure (ch. 10):** Edge + two antiparallel half-edges;
  mate is *computed*, not stored; loop = circular chain of half-edges;
  the degenerate states are load-bearing and legal mid-construction:
  **empty loops** (a loop holding only a lone vertex), **struts**
  (`mev` from a lone vertex), **self-loop edges**, **laminae** (two
  faces sharing all edges — the cube passes through one). Only
  vertices carry geometry through Euler ops; face/edge geometry
  attaches later (M2).
- **Implementation (ch. 11):** low-level ops address topology by
  half-edge handles (no searching); `addhe`/`delhe` encode the
  empty-loop placeholder convention; `lmef` does **not** reclassify
  rings (a separate non-Euler `ringmv` helper does); in `lkemr` which
  component becomes the ring is argument-order-dependent. GWB's C code
  reads freed nodes (blessed by its allocator) — a Rust port must
  reorder reads before frees.

## Working agreements (inherited from M0, plus M1 specifics)

- Small PRs, each a design conversation where marked; design PRs carry
  a full writeup and **wait for Evan's sign-off**; work continues
  stacked on top, accepting rework risk. Non-design PRs self-merge
  after adversarial e2e review + green CI.
- One implementer + one adversarial e2e reviewer + one fix pass per PR;
  reviewers write and run real construction programs, not just read
  diffs.
- Branches `ev/m1-<n>-<slug>`, stacked serially, merge commits only.
- Orchestrator log: `docs/M1-LOG.md` (L-numbering continues from M0's
  L7).
- D9 charter applies to every line: no panics, typed errors,
  deterministic iteration, no `T`-branching inside topology.
- **Transcription hazard, named once:** GWB's diagrams are
  clockwise-oriented; if we ratify counterclockwise (PR 1), every
  argument-order/figure transcription from the notes must be mirrored
  consciously. Reviewers check orientation by construction tests, not
  by matching the book's pictures.

## PR sequence

1. **Half-edge restructure of `topo`** *(design PR)*. Replace M0's
   placeholder adjacency (`Loop { edges: Vec<EdgeKey> }`,
   `Edge { start, end }`) with the half-edge form: new `HalfEdge` arena
   + typed key; `Edge` becomes its two half-edges; `Loop` holds a
   representative half-edge; `Face` distinguishes its outer loop from
   rings; `Vertex` gains an optional emanating half-edge. Design
   decisions presented in the PR:
   - **Empty-loop representation**: Mäntylä's nullable-edge placeholder
     half-edge vs. a typed `Loop` state (illegal states
     unrepresentable, `mate` total) — recommendation with trade-offs in
     the PR writeup.
   - **Orientation conventions, documented once** (the DESIGN.md
     deferred item): loop direction vs. outward normal (proposal:
     counterclockwise viewed from outside, deviating from GWB —
     mirrored diagrams noted), antiparallel mates, which half-edge
     defines edge direction.
   - Intrusive `next`/`prev` cycles vs. `Vec` loops (proposal:
     intrusive, faithful to the surgery idioms).
   - Which GWB acceleration pointers we keep (§10.5: loop
     back-pointers are load-bearing; per-solid edge/vertex lists are
     redundant given arenas).
   Validator restructured to *structural coherence* of the new shape
   (cycle closure, back-pointer consistency, mate involution);
   fixtures/proptest rebuilt. No Euler ops yet — raw insertion still
   builds fixtures.

2. **Euler operator API + the cube: `mvfs`, `mev`, `mef`**
   *(design PR)*. The op-shape conversation:
   - Ops as `&mut Body<T>` methods returning
     `Result<CreatedKeys, EulerOpError>`; **atomic** — all preconditions
     checked before any mutation; every failure a typed error (D9).
   - Addressing: low-level ops take half-edge keys directly (arenas
     make keys stable handles — GWB's id-scan layer is dropped); a
     `find_half_edge(face, v1, v2)` helper serves tests and hand
     construction.
   - **D5 provenance goes typed**: `Provenance::Primordial` placeholder
     replaced by per-operator variants recording the op + argument
     keys.
   - **Deterministic minting order** documented per op (D9 + lineage
     replay: same construction history ⇒ same key sequence).
   - Geometry policy at M1: `mvfs`/`mev` take `Point3<T>`; `mef` mints
     a `Placeholder` curve and copies the parent face's surface — real
     geometry attaches at M2 (signature slots documented now).
   - Debug builds run the validator after every op (D1).
   Lands the **cube-by-hand acceptance test** (1 mvfs + 7 mev + 5 mef,
   validates cleanly, counts verified), plus per-op Euler-delta debug
   assertions.

3. **Rings and genus: `kemr`, `mekr`, `kfmrh`** *(design PR)*. Ring
   conventions: which side of the killed edge becomes the ring
   (argument order, per ch. 11); a `ring_move` helper (documented as
   *not* an Euler op — `mef` deliberately does not reclassify rings);
   `kfmrh` semantics including the cross-shell case (genus vs. shell
   merge — recommendation: support same-shell now, typed error on
   cross-shell until M3's splitting demands it). `mekr` included as
   `kemr`'s inverse (and because `h > r` solids strictly require it).
   Lands the **box-with-through-hole acceptance test** (genus 1,
   minimal sequence per §9.3) and kemr∘mekr roundtrip tests.

4. **Kill-direction duals: `kvfs`, `kev`, `kef`, `mfkrh`**
   *(non-design — semantics forced as exact inverses; preconditions per
   ch. 9: `kev` needs distinct end vertices, `kef` distinct adjacent
   faces)*. Roundtrip property tests: over proptest-generated
   construction sequences, each kill undoes its make and restores the
   E–P tuple; enables teardown-order fuzzing. M3's booleans and
   splitting are the eventual consumers.

5. **Validator completion + validity tiers** *(design PR)*. The
   DESIGN.md "concrete invariant checklist (M1)" item:
   - **Two-tier validity** (ratified in PR #15's conversation): tier 1
     "euler-valid" accepts every Euler-reachable state (empty loops,
     struts, laminae legal — they are mandatory intermediates) and is
     what debug builds check after every op; tier 2 "closed solid"
     additionally bans construction scaffolding: no empty loops, no
     valence-1 vertices (struts). Laminae are deliberately **not**
     banned at tier 2: two faces glued along their entire shared
     boundary is exactly the incidence structure of a legitimate
     two-hemisphere ball, so a zero-volume lamina is a *geometric*
     defect. No bespoke lamina rule exists at the geometric tier
     either (M2+, named now as the taxonomy's third layer, out of M1
     scope): fold-back edges die under the ratified predicate
     discipline — intrinsic variants by their transversality/
     separation margins (D2), conventional variants (whose
     descriptions are true even when degenerate, e.g. a height-0
     extrude's `MappedCurve`s) by the **material wedge-angle
     predicate** — wedge ∈ (0, 2π) bounded away from the ends by the
     derived threshold θ = ε/r; wedge = π is the legal smooth
     `Seam`/conventional-split case — enforced at the operation (D4
     ¶3 typed error) and rechecked by the geometric validator.
     Residual gap, stated: coincident faces sharing *no* edge (e.g.
     two coincident closed shells) are invisible to edge-local checks
     — that is global self-intersection/minimum-clearance, deferred
     to M3 (partial, via booleans) and M6 (interval clearance) per the
     roadmap. Watertightness (every edge exactly two antiparallel
     half-edges, vertex orbits single cycles) is structural — tier 1 —
     in the half-edge representation. Finished bodies must pass
     tier 2; tier-1-only states are visible solely inside operation
     implementations.
   - **Component-aware per-shell Euler–Poincaré** *(corrected in PR 4 —
     the naive form is wrong for tier-1 bodies: `mfkrh` on a detached
     ring disconnects a shell's surface while one shell entity
     remains)*: per shell, partition the incidence complex into
     connected components (faces glue all their loops, outer and rings;
     a cycle loop glues its edges' two sides via mate; an empty loop
     glues its lone vertex; a dartless empty-outer face is its own
     component with its vertex); each component is a closed oriented
     surface piece and must satisfy v − e + f − r = 2(1 − g) with g a
     non-negative integer (parity checked per component); per shell the
     sum reads v − e + f − r = 2(c − Σgᵢ) with c the component count.
     The naive per-body h = s − (v−e+f−r)/2 equals Σgᵢ only when every
     shell has c = 1. Tier 2 additionally requires c = 1 per shell
     (note: reachable disconnections exist with NO empty loops or
     struts — promote a detached cycle ring — so tier 2's existing bans
     do not imply c = 1).
   - **Orphan-vertex rule restated** (M0 deferral): a vertex must be
     referenced by ≥1 half-edge *or* be the lone vertex of an empty
     loop.
   - **Bidirectional D5 provenance check** (M0 deferral): every live
     entity has provenance; no provenance outlives its entity (ops now
     remove arena entries, so `SecondaryMap` leaks become reachable
     bugs).
   - Arity/emptiness rules land here, with the operators that give them
     meaning (per validate.rs's M0 deferral note).
   - **`Body<Interval>` instantiation test** (M0 carry): build the cube
     at `T = Interval` under the `interval` feature and validate.
   - **Raw-insertion builder demoted to `pub(crate)`** — Euler
     operators become the only public construction path (D1:
     "exclusively through Euler operators"); validator tests keep
     in-crate access for malformed fixtures.

6. **M1 exit sweep** *(docs; self-merge)*. Ratify into DESIGN.md:
   orientation/sense conventions (the deferred-list item), the
   half-edge entity in D1's entity list, the operator set + addressing
   scheme, validity tiers; close out M1-LOG; update memories
   (`cad-project-state` → M1 done).

## Deliberately not in M1

- **Sweeping / primitives** (ch. 12) — M2, where real geometry attaches.
- **GWB's high-level id-scan layer** (§11.4–11.5) — dropped; typed
  arena keys are already stable, O(1) handles.
- **Vertex-geometry taxonomy** (D2's vertex generalization) — M3, when
  intersections exist; M1 vertices stay `Point3<T>`.
- **Multi-shell creation inside a solid** (cavities) — arrives with
  M3's splitting/booleans; cross-shell `kfmrh` typed-errors until then.
- **K's numeric value** (M0 carry) — topology is scalar-free and never
  consults a predicate, so M1 generates no new K evidence; the multi-ε
  experiments wait for M2's geometric predicates. Stays on the
  watchlist.

## Exit criteria

All six PRs merged (design PRs with Evan's sign-off); cube and holed
box built by hand through public Euler ops only, passing tier-2
validation with E–P verified; make/kill roundtrip property tests green;
bidirectional provenance check live; `Body<Interval>` cube test green
under the `interval` lane; CI green at ε ∈ {1e-6, 1e-9, 1e-12};
conventions ratified into DESIGN.md.
