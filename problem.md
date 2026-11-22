# The problem (design fork `embed-wedge`)

Handed to every designer as written. It states a problem and where to look. It proposes no answer: the options are yours.

## What needs deciding

A finished solid passes **tier 3** and then **the embedding question**.

- Tier 3 is a set of per-entity checks. One of them, check 4, reads each edge's **wedge**: the angle the two faces make along the edge, levered at how far the faces reach, against the band K·ε (`crates/topo/src/edge_wedge.rs`, `crates/topo/src/wedge_reach.rs`, `geom_brep::edge_wedge_over_as`, `classify_dihedral`, `folded_lever_arm` in `crates/geom-brep/src/dihedral.rs`).
- The embedding question (`topo::clearance::embedded`; the certificate is `crates/topo/src/injective.rs`) establishes that the solid's boundary does not meet itself. Where two faces share an edge it must show they do not cross near it. It subdivides the two faces into cells, signs a separator on each face's part of each cell, and asks that the faces **depart** from what they share by more than the band.

**What should the embedding question's local reading be where faces meet, and what is its relation to the wedge tier 3 has already read?**

## Why it is open

`docs/DESIGN.md` `[topology.tiers]`, the finished-body paragraph, says of the embedding question:

> The question is asked inside the gate once tier 3 has passed, and reads tier 3's facts from its verdict, never re-deciding one over a cell. […] Where faces do meet, or a face is read against itself, the local reading (how the faces leave their shared edge or vertex, or a face's own turning) passes by more than the band. That reading is the edge's wedge as tier 3 reads it, one reading for both: an angle metered at the reach of the faces it governs ([tolerance.epsilon]), never at a cell of the question's own subdivision. A cell holds none of a face only where it lies off the face. A reading inside the band refuses as ambiguous, never as Holds or Violated.

The sentence beginning "That reading is the edge's wedge…" is Ev's ruling on PR 7095. Before it, the certificate's margin was the separator's rate times the **cell's** span, and a concession (the "sliver") declared a face absent from a cell where its part was a strip no wider than ε. Ev's words on that PR:

- "is 'rate times face size' a length? this sounds like it could be using eps for something other than a maximum deviation from true geometry … definitely agree with getting rid of the shortcut though"
- "the margin is needed/used for things that must tolerate the geometry being eps off from real. is this one of those cases, where geometry that's eps off could cause issues?"
- (to "the margin is kept, metered at the faces' reach, as one reading with tier 3's wedge check") "yes i think so, presumably unified with the other case of this that you mentioned"

PR 7329 (`origin/embed/remeter`, reviewed, in its fix pass) builds that ruling like this: in every cell near a shared edge, `sign_on` decides `sine × arm` against K·ε, where the sine is the separator's normalised rate enclosed over the cell's rectangle and the arm is `arm_of`: the reach by the recipe check 4 is handed (`wedge_reach::reach_over`); for two faces sharing only a vertex, both faces' reach from that vertex; at a pole or apex (a collapsed side of a chart) the sine's limit. The sliver is removed.

Its two reviews both approve it and both say the following. Their reports are in this branch, `reviews/review-1.md` and `reviews/review-2.md`.

1. **The engine and check 4 can class one edge differently.** Check 4 folds the faces' curvature arms into the lever (`folded_lever_arm`); the engine uses the reach alone. Measured on one edge with one 1 m reach, a cylinder of radius 1e-3 against a plane through its ruling:

   | sine | check 4 | the engine |
   |---|---|---|
   | 1e-4 | Corner | Positive |
   | 5e-6 | InBand | Positive |
   | 5e-7 | Tangent (smooth class) | Positive |
   | 5e-9 | Tangent | None |

   One reviewer: under Ev's test (can geometry ε off turn the angle?) the reach alone fails on a curved face whose radius is far below its reach. No wrong body passes today only because check 4 gates first. Folding the curvature arm in the engine was tried in the unit's design step: it refuses every cone that owns its apex (a cell holding the apex has a curvature arm reaching 0; tier 3 passes that edge only because its stations are interior to the edge).
2. **The engine re-decides the wedge in every cell near the edge**, where the paragraph says the question reads tier 3's facts from its verdict and never re-decides one over a cell. Main did the same before PR 7329, with the cell's span as lever.
3. **Tier 3 reads the wedge at stations interior to the edge; the engine reads the whole edge, cell by cell.** (`work/restfront/check-4-reads-an-edges-wedge-at-stations-where-the-embedding-question-reads-the-same-margin-over-the-whole-edge.md`, on `origin/embed/remeter`.)
4. **Some of what the engine must read has no edge**, so tier 3's check 4 says nothing of it: two faces that share only a vertex; a face against itself (its own turning); a pole or an apex, where a chart's side collapses to a point.
5. **Five more margins in the certificate are still levered at something of the subdivision** (`work/embed/five-more-injectivity-margins-are-levered-at-a-cell-of-the-sweep.md`, on `origin/embed/remeter`; one review corrects two of its five).

## Not part of this question

The **site** the reach is measured from. Measured from the edge's middle it counts a face's run ALONG its edge as lever (two strips 4 long and 0.05 wide, folded so their far edges part by 0.9ε, are read as opening 72ε). That is `work/honeone/the-wedge-reach-counts-a-faces-run-along-its-edge.md`, and another program (ONEROUND) has a fork on it going to Ev, with "the line the datum turns about" as its designers' answer (`work/oneround/log.md`, 2026-11-21). Take it as given that the reach's site will be settled there; say if your answer depends on how.

## Where to look

- `docs/DESIGN.md`: `[topology.tiers]` (the whole finished-body paragraph), `[tolerance.epsilon]`, `[tolerance.recourse]`, `[one-way.places]`, `[unrepresentable]`; `crates/topo/README.md`, tier 3.
- On `origin/embed/remeter` (`git show origin/embed/remeter:<path>`): `crates/topo/src/injective.rs` (`sign_on`, `Margin`, `arm_of`, `reach_from_vertex`, `departure`), `crates/topo/src/wedge_reach.rs`, the PR's rows named above, and `work/embed/the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause.md` (the measurements, the earlier fork, Ev's ruling, what landed).
- On main: `crates/topo/src/edge_wedge.rs`, `crates/topo/src/validate.rs` (check 4 and what the verdict keeps: "the verdict keeps tier 3's per-entity facts beside its certificate, and a reading of such a fact takes the bundle"), `crates/geom-brep/src/dihedral.rs`, `crates/topo/src/clearance.rs` (`embedded`, the walk).
- `docs/DESIGN-FORK-LOG.md` row 285 (the fork behind PR 7095).

Check every statement above against the tree; the orchestrator has been wrong about causes several times this week.
