# JOIN-2 review, lane r1 — PR 3880 @ `17c254c99d` (base main `66bbdaa6bd`)

**Verdict: APPROVE-WITH-FIXES.** MAJOR 1 · MINOR 2 · NOTE 6. MAJOR-1 must be fixed and re-measured before merge. A fix is demonstrated below (mutant M2e).
No glimpse: I read no other `join/2-zip-reads-segments-review-*` branch and no PR comments. I read only the PR body, through the API.

Instruments (all committed in `crates/sweep/tests/join2_r1_probes.rs`, ignored rows):
- `join2_r1_grid`: **31,103 op lines, a battery of my own shape.** A rounded plate (r = 0.5, 1) or the rounded L under an axis-aligned upper rectangle. The rectangle's sides are drawn from the lower's corners, tangent points, arc midpoints and outside values. It runs U, A∖B, B∖A and A∩B in both operand orders. The oracle is the operands' measured volumes, since the interiors are disjoint, read through `common::differential::outcome` (tier 2, 3′, certificate, operand, volume).
- `join2_r1_tangent_radii`: radii toward 0 and toward the plate size (0.01 to 1.99), the L, and mismatched fillets.
- `join2_r1_like_far_ends`, `join2_r1_cove_corner`, `join2_r1_two_edges_one_pair`: targeted probes.
- `join2_r1_unions_main_builds_sound`: **the regression row. Red at head, green on main** (executed on both).

Main and head were each built release in separate target dirs, and the line files were diffed.

## Findings

**MAJOR-1 — The ring-first order refuses unions main built sound (16 grid lines).** `crates/topo/src/boolean/rest.rs:922-929`. Executed.
- `realize_seam` takes first any unrealized span with *either* end at an unjoined pierce-ring vertex. When a span with BOTH ends unjoined comes first in the join's order, `mint_chord` reaches its `([], [])` arm (`rest.rs:1264`) and refuses `RestZipUnsupported { ChordBetweenIsolatedPierces }`.
- This happens where the contact outline runs through two consecutive ring vertices, e.g. an upper rectangle whose corners sit inside the lower's top face.
- Grid diff, main → head: **16 SOUND→refusal**, all `ChordBetweenIsolatedPierces`, all unions. Plus 412 refusal→SOUND gains and 0 BAD on either tree.
- Traced at head: `grid r=1 x=[0,1] y=[0.29,1]` picks span 0 with `iso=(true,true)` first.
- This falsifies the brief's claim 3 ("pierce-ring-first is … sufficient") and the PR's "0 SOUND→refusal" outside its own batteries.
- **Mutant M2e** picks spans with *exactly one* unjoined end first, then the rest as now. Results:
  - topo + sweep suites all green;
  - grid vs head: +478 refusal→SOUND, 0 losses, 0 BAD;
  - vs main, only MINOR-1's 3 lines remain lost.
- M2e also builds `rect a=3 r=1` (`join2_r1_like_far_ends`), which refuses `ChordBetweenIsolatedPierces` on main and on head alike. That gives `rest-zip-frontier-refusals-reached-by-no-row` an end-to-end fixture.
- Rows: `join2_r1_unions_main_builds_sound` lists the 16 poses.

**MINOR-1 — The filed residue is reachable, and reaching it loses a union main built.** `sectors.rs:958` (`germ_loci`'s `Equal` arm). Executed.
- Fixture: a plate with a convex fillet into a notch whose east wall returns to y = 0, under an upper L whose reflex corner sits on the fillet's far end (`ell a=… r=…` in `join2_r1_like_far_ends`).
- Instrumented: `ta=Boundary tb=Boundary paired=false` four times per pose. This is exactly `two-tangent-edges-parting-with-like-far-end-touches-stay-on-edge-both`.
- The issue body says "No row in the suites or JOIN-1's batteries reaches the open shape", and the PR says 0 hits. Both are true of those batteries only.
- Outcome: a refusal (`Join(UnpairedLooseEnds { count: 2 })`), never a wrong body. A∖B, B∖A and A∩B are SOUND or empty. Claim 1's "only a refusal" holds here.
- But main built the QA union SOUND through the zip's own enumeration (3 radii; PA refused on main too), and head refuses both orders. This falsifies claim 4 ("nothing the deleted code decided is lost") for these 3 lines.
- The issue should carry this fixture and the loss.

**MINOR-2 — The sweep misses a new edge identity by vertex pair.** `sectors.rs:941-947`. By inspection; no wrong body found.
- `germ_loci` decides "the two edges coincide" iff their far ends are a recorded `vv` pair. That is an edge identity by vertex pair, added by this PR.
- The PR's sweep pattern (`half_edge_end(..) == Some(..)`, `edges_of_vertex`) cannot match a `contacts.vv` lookup.
- Two tangent edges with the same ends and different carriers (a line and a non-circular curve) would read `OnEdge` both.
- My `join2_r1_two_edges_one_pair` cases have non-tangent edges, so they never enter the branch: all built in the join (SOUND) or refused at the reduction (`CurvedPierceUnsupported`).

**NOTE-1 — Claim 2 holds.** Executed.
- An env-gated instrument compared every new locus with the old reading (`OnEdge` iff a bound reads On).
- Whole topo + sweep suites: the locus moves only in `reach_continuation::a_tangency_in_the_middle_of_an_edge_builds_in_either_operand_order`, 304 times.
- R1's `join1_r1_battery`, `_declared_battery` and `_reflex_battery` (polygonal): 0 moves.

**NOTE-2 — Claim 1 holds on every pose I could get past the reduction.** Executed.
- The grid and the radii sweep: 0 BAD, in both orders and all four ops.
- The far end recorded "deeper" for another reason: the rectangle over a notch (`rect`). Its line fragment's far end re-enters (`Boundary`) while the arc ends on the face (`Face`), and it ranks correctly (SOUND).
- My lemma: on these planar stacks, an edge that leaves the partner can only reach `Boundary` (it must cross to come back), and an edge that stays reaches at least `Boundary`. So a strict mis-rank needs a far end recorded `Face` past a gap.
- That needs a 3-D vertex-on-face tangency. My cove pose (`join2_r1_cove_corner`: a filleted block in an L's inside corner) refuses upstream (`CurvedBooleanUnsupported`, Cylinder), and the V-notch upper refuses `CurvedPierceUnsupported`. So that branch of `germ_locus` (`sectors.rs:907-920`) is reached by no pose I found. It is **unsure** rather than shown.

**NOTE-3 — Claim 6 holds: the tests go red.** Mutants, full topo + sweep suites:

| mutant | killed by |
|---|---|
| M1: locus fix reverted (`OnEdge` whenever a bound reads On) | only the re-pinned tangency row |
| M2a: zip declines (`read_segments` → `None`) | 8 rows: every zip-built pin, the new `a_shaft_off_the_bores_seam_is_built_by_the_zip`, three `contact8` rows |
| M2b: ring-first order removed | the two mate2 rows (needed: confirmed) |
| M2c: `fragment_holding` ignores the lineage | 5 rows; 762 grid lines go SOUND→refusal, 0 BAD |
| M2d: `realize_seam`'s `edge_ends` check off | survives everything, and the grid is byte-identical: the guard is reached by no row |

**NOTE-4 — Claim 3: no wrong fragment found.** Executed.
- The grid puts several chords and pierce rings in one face, in both orders.
- 0 BAD at head; M2c shows the lineage is load-bearing and fails as a refusal.

**NOTE-5 — `joined` + `pair_patches` cannot tell two edges between one pair apart.** By inspection.
- `joined` (`rest.rs:1086`) skips mirroring when *any* edge joins u, v.
- `pair_patches` (`rest.rs:1461`) compares vertex cycles (`cycle_starts`), which are identical for two distinct edges between one pair.
- The exposure is limited to interior patches the glue discards. No pose built.

**NOTE-6 — Head suites green.** topo + sweep, release, no failures, run with my instrumentation compiled in but off. The tour's two-peg mate was not re-run.

## Style

I exercised Q1, Q2, Q3, Q4, Q5, Q6 and Q7. Q8 was only partial: I skimmed `rest.rs` (2,282 lines) for structure and did not read it line by line.

- **S1 (Q1) — Four spellings of "the faces around a vertex", one of them new.** *likely.* They are `sectors.rs:1092` `faces_at` (new; null-site-aware, skips null faces), `rest.rs:1285` `incident_faces`, `body.rs:1498` `faces_of_vertex` and `offset_together.rs:673` `faces_at_vertex`. `tangent_face`'s "the one common face" also re-spells the host search that `mirror_edges` and `fragment_holding` each do. It is plausibly a class: look in `splitting/` too.
- **S2 (Q1) — "An edge's two vertices" is written three times.** *sure.* `rest.rs:967-971` `edge_ends` (new), `rest.rs:1026` and `sectors.rs:1055`, each with `start(he_plus)` / `start(he_minus)`.
- **S3 (Q1) — The operand→contact-list selection is repeated.** *likely.* The `Operand::A => a_on_b / B => b_on_a` match and the `vv` side swap appear in both `touch` (`sectors.rs:998-1020`) and `tangent_face` (`sectors.rs:1027-1075`).
- **S4 (Q2/Q7) — `tangent_face`'s `None` silently becomes the sector's face.** *unsure.* The fallback is `f.unwrap_or(side.sector.face)` in `germ_loci`'s `tangent` closure. In a fail-loud kernel, a germ whose host face could not be decided is still given a cell. The zip's `fragment_holding` may then refuse it, but the join reads it unguarded.
- **S5 (Q3) — The locus fix has one row.** *sure.* NOTE-1 and M1 show a single suite row exercises a moved locus; every other tangent shape in this review lives only in my ignored probes. The like-far-ends residue had no row until mine.
- **S6 (Q5) — Stale docs left by the deletion.** *sure.*
  - `arcs.rs:1-4` says the module is shared with the declared-REST zip; the zip no longer calls it.
  - `reach_aligned_half_rods.rs:7-14` says the stack builds through the zip via `arcs_along`; the PR body says it now builds in the join.
  - `docs/predicate-dimension-audit.md:457-459` still list `rest.rs`'s `bool_join_chord/facing/nearest` rows.
  - `rest.rs:171-172`'s step-2 comment still says "A-side geometry — the site points are bitwise-shared", which is a relic of the deleted enumeration.
- **S7 (Q6) — The residue's deferral rests on a false premise.** *sure.* The PR body and the issue both say nothing reaches the residue; MINOR-1 reaches it. The P2 ranking holds no schedule beyond the issue.
- **S8 (Q2) — The ring-first comment claims more than the code does.** *likely.* `realize_seam`'s doc (`rest.rs:900-906`) says ring chords go first "each joining its ring into the face's boundary before any chord divides that face". The code does not ensure that: a span with both ends on rings joins two rings to each other, or refuses (MAJOR-1).
- **S9 (Q4) — The PR's sweep reads producers, not consumers.** *likely.* It lists locus producers but not the consumers that turn a locus into an edge identity: `insert.rs:817` `own_locus_edge` (strut facing), `join.rs:997` and `join.rs:1050`. These are exactly the readers whose input this unit changed.
- **S10 (Q7) — Not how I'd order the spans.** *unsure.* `realize_seam` rescans every span per pick with a closure rebuilt each iteration (O(n²)), and the order is a priority heuristic. A walk outward from already-joined vertices would make "never chord two unjoined rings" structural.
- **S11 (Q3) — The pins accept any join refusal.** *likely.* They assert `matches!(join, Ok(Some(_)))`, so a join that refuses for a new, wrong reason still passes them. Only the tangency row pins a shape (`SectionInvariant`).

REVIEW COMPLETE
