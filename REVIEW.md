# Review of #4004: "JOIN: the fan end has one home; strut facing has one rule"

Frozen head `a4181ca0`, base `11d9c7a7`. Full reviewer lane.

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 2 · NOTE 4.

The facing rule shipped no wrong body anywhere I could reach it. The behaviour change is
confined to the `_` arm, as the PR says, and every one of the PR's measurements I re-ran holds.
The two MINORs are an unpinned behaviour change on an unreached path, and stale rule prose.

## How it was checked

- Probe rows: `crates/sweep/tests/fan_end_review_probes.rs`, on this branch.
  - `fan_end_review_prism_battery`: corners of five prisms, each touching a SIDE=10 cube's face
    on the plane through the corner, with normal `m` over a 400-point Fibonacci sphere.
    - The corners: the L's reflex and convex corners, the 315° corner, a 200° notch and a ~350°
      slot.
    - Each corner at the top and at the bottom of its prism.
    - Ops ∪ ∩ ∖, in both operand orders.
  - `fan_end_review_rim_battery`: a revolved tube's rim vertex (the lone vertex of a closed circle
    edge) on the face. The normals are the same sphere plus 24 that hold the rim tangent.
  - Every line is `differential::outcome`: tier 2, tier 3′, the certificate, a legal operand
    (`assert_legal_operand`'s union) and the volume.
- The volume oracle is independent of the PR's: the slab on `m`'s side is `∫_R clamp(L,0,1) dA`,
  computed as `∫L⁺ − ∫(L−1)⁺` with half-plane clips and first moments, and a disc's in closed form.
  The tube's measured volume equals π(r²−b²) to 12 digits.
- Instrumentation, in a scratch copy of head and not pushed. It logs, at every strut, the
  `strut_faces_first` step and its answer. Beside that answer it logs:
  - `vtxfac`'s old facing;
  - main's `mint_directed` angular `strut_order`, at `arm.min`.

## Claims

**1. The facing rule is right everywhere it is read: HOLDS.**
- Prism battery, release build: 29 087 lines on both builds.
  - main → head moves exactly **5 628** lines, and every moved line is a `vtxfac` unvoted
    bare-arm pose. On main they all refuse `JoinDesync` (4 221 "every chord arc separates…",
    1 407 "B senses agree…").
  - On head, 2 814 are `OK SOUND`. The other 2 814 refuse typed (938 `Euler(SelfLoopEdge)`,
    1 876 `JoinDesync` "…not exactly one kept end").
  - The split is exact on geometry, in all 4 reflex profiles, top and bottom:
    - SOUND ⇔ the corner's vertical edge reads In (one bare run);
    - refused ⇔ the vertical edge also reads Out. That is two Out runs at one vertex, the PR's
      filed `a-pierce-with-two-out-runs-at-one-vertex-refuses-every-op`.
  - No `BAD` line moved; the 24 `BAD` lines are byte-identical on main (NOTE 4).
- Arms reached unvoted: whole orbit 2 814 times and bare arm 5 628 times, over reflex sectors of
  200° to 350°, both operand orders, ∪ ∩ ∖.
  - The angular `strut_order` gives the same answer as the new walk order on all 8 442 calls.
    So two independent readings of "the germ the corner meets first" agree on every pose that
    exercises the change.
- Convex sectors cannot host a `vtxfac` strut: its two germs are the two rays of one line. Nor can
  two germs share a `vtxfac` entry: `build_sectors` bisects every sector of 180° or more
  (`sectors.rs:220`), so every entry is under π. The convex-corner and "several germs in one
  entry" cases are therefore empty on this path. `strut_faces_first`'s `strut_order` step is
  reachable from `mint_directed` only (see claim 2).
- Closed edge's lone vertex: the rim battery, including rim-tangent normals, never reached
  `StrutFacing::ClosedEdge` (0 step-3 calls), and neither did the batteries or the suites below.
  See MINOR 1.

**2. `run_site` is the same strut every old copy minted: HOLDS.**
- By reading:
  - `body.rs:1600`'s `orbit_step` is exactly the old `get_half_edge(mate(last)).next`.
  - `WholeOrbit.mev_site()` is `Fan{first, first}`, the same site the old `he2 == first` built.
  - The splitter keeps its flags `(true, true)` (`splitting/insert.rs:106`).
  - `run_degenerates` and `mint_run` keep their semantics. Only one error string merged: a stale
    mate now reads "run edge without a mate".
- By running the instrumented build over:
  - the PR's 7 batteries (`join1_r1_*` ×6 and `rc_wide`);
  - the sweep suite (2 026 tests passed);
  - the topo suite (890 passed).
- About 42 000 unvoted `strut_faces_first` calls in all. Every one equals main's `strut_order`,
  including the `mint_directed` calls with germs in different entries, where head now reads
  `precedes` instead of the angular order.
- The only disagreements are the 6 bare-arm calls of the PR's own new row. The topo suite's
  bare-arm mints are 59 voted plus those 6, in line with the PR's "60, all settled by the vote".
- My own-shape battery diff is above. I did not re-run the PR's 132 251-line main/head diff
  byte-for-byte; the facing comparison over the same batteries covers its premise.

**3. The new rows can go red: HOLDS.** Release builds, rows `join*` (30) plus the `run_site` unit
row:
- M1 (`run_site` never `WholeOrbit`): the unit row, the new whole-orbit row, the star fixture,
  the 3 `join_whole_orbit_rows` and `r2_multi_spike_corner_meet_is_tier_three` go red.
- M2 (walk order flipped): both new rows and `the_reflex_315…` go red.
- M3 (main's `he_minus` on the bare arm): only `a_bare_bisector_strut…` goes red.
- M4 (whole-orbit facing flipped): the whole-orbit row, `a_merged_rim_vertex…`, the star fixture
  and `the_reflex_315…` go red.
- M5, mine (`precedes` anchors on `start_edge` instead of `end_edge`): the whole-orbit row and
  `the_reflex_315…` go red.
- M6, mine (`precedes` returns the raw `p.0 < q.0`, with no wrap anchor) survives every row and
  my whole battery (0 of 29 087 lines move). It is equivalent: `build_sectors` pushes each
  sector's own entry before its twin, so no physical sector straddles index 0. NOTE 2.

**4. The sweep is complete: HOLDS, with two parallel spellings to note (Style).** My sweep was
shaped by role, not by symbol:
- every `NewVertexSide::{Above,Below}` choice in production `src`;
- every `created.he_plus` / `created.he_minus` facing binding;
- every `.prev` read in `boolean/`, `splitting/` and `null.rs`;
- every `vertex_orbit(` outside `body.rs`.

New hits beyond the PR's list are only parallel *side* spellings, not fan ends:
- the splitter's `whole_orbit → Below`, `dangling → Above` (`splitting/insert.rs:124`);
- the ring struts (`vtxfac.rs:747-759`), which the PR filed.

Blind spot: a facing computed through an index into a precomputed orbit array.

**5. The oracle is exact: HOLDS.**
- `the_prs_simpson_oracle_agrees_with_the_clamp_oracle` copies the PR's `prism_beyond`. Over its
  two poses and 200 sphere normals it agrees with the clamp oracle to <1e-12.
- `the_clamp_oracle_matches_a_brute_slice_sum` checks the clamp oracle against a 4 000-slice sum.
- The argument holds: the clipped area is quadratic in z between the heights where the cut line
  passes a profile vertex, and those heights are the PR's breakpoints.

## Findings

- **MINOR 1: the closed-edge path changed behaviour with no row.** Run every build of
  `vtxfac.rs:627` and `insert.rs:1206`, plus my tube battery: 0 calls reach
  `StrutFacing::ClosedEdge`.
  - Main refused that path typed (`CurvedBooleanUnsupported`). Head now decides it silently by
    entry order.
  - The PR discloses this ("That path is unreached") with no row, no schedule and no claim-site
    reason. Docs prompt Q6 asks for one of those three.
  - Entry order is principled, and agrees with the angle everywhere it was measured. But the old
    fail-loud refusal becomes an unverified answer. Pin it with a row, or keep the refusal there.
- **MINOR 2: stale rule prose.** `boolean/join.rs:92` still presents `insert::strut_order` as
  "the angular strut spike order", the rule that ranks a strut's germs. It is now step 3 of
  `strut_faces_first`, and reachable from `mint_directed` only. `tests/m3_pr5_boolean_ops.rs:196`
  and `tests/review_m3_pr5.rs:387` use the same phrase. The PR updated `join.rs:2156` but not :92.
- **NOTE 1: two orderings remain for germs in one entry.** `precedes` orders two cuts in one
  entry itself (`walks_after`, `insert.rs:1086`). `strut_faces_first` routes that case to
  `strut_order` instead (`insert.rs:1208-1220`). `mint_directed` reads both on one strut's germs:
  `from_is_lo` through `precedes`, and the facing through `strut_order`. They agree while an entry
  is under π (bisection guarantees that), so this is not a bug. But the "one rule" still has two
  spellings of its last step.
- **NOTE 2: `precedes`'s backward walk is dead.** The walk to the sector start
  (`insert.rs:1091-1099`) is equivalent to `p.0 < q.0` under `build_sectors`' push order (M6
  above). Either the walk guards a layout no caller produces, or the layout assumption is
  unstated.
- **NOTE 3: the residual refusals all fall in the filed two-run case.** Every residual head
  refusal on the bare arm is that issue's shape: P0, at 2 814 lines in this battery. That is
  useful sizing for the filed issue: it is half of all bare-arm poses.
- **NOTE 4, pre-existing and identical on main: a tube rim touching a face fails tier 3′.** The
  pose is a revolved tube's rim vertex `(1,1,0)` touching a face whose plane holds the rim
  tangent (`m = (cos θ, sin θ, 0)`, tube entirely outside). Its ∪ builds with the exact volume but
  `t3p=false`, in both orders (24 lines; `FANREV tube … U`). Out of this PR's scope; reported for
  routing.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q6 and Q7. Q5 was read against the `RunSite` and
`strut_faces_first` docs only. Q8 was **not** done end to end: I read `vtxfac.rs` 440-780 and
`insert.rs` 840-1560, not the whole of either file.

- `vtxfac.rs:628-632` (Q7, **likely**): `vtxfac` destructures the `MevSite` it just built to get
  the corner back, with an unreachable `ClassificationInvariant` else-arm. `RunSite::WholeOrbit
  { corner }` already names the corner, and the bare arm knows `after.he`.
- `splitting/insert.rs:124-140` against `vtxfac.rs:648-660` (Q1, **likely**): side-follows-facing
  is spelled three times.
  - The splitter hard-codes `whole_orbit → Below`. It has no germs, but it is the same geometric
    fact.
  - `vtxfac` derives `start_on_plus`.
  - `mint_run` swaps on `dangling && !spike_from_first` (`insert.rs:1528`, `:1560`).
  - Class: look also at the ring struts (`vtxfac.rs:747`), which the PR filed.
- `insert.rs:1190` (Q7, **unsure**): `strut_faces_first` takes positional `(usize, Cells, Vec3)`
  tuples, and `vtxfac` builds them through a local closure. A named germ type exists in each
  caller (`Germ<T>`, twice, with different shapes).
- `euler.rs:522` (Q7, **unsure**): `RunSite` sits beside the public `MevSite` but is
  `pub(crate)`, and its `mev_site()` exists only to be matched straight back by every caller.
  Every caller writes `site @ RunSite::… => (site.mev_site(), flag)`. A `(MevSite, bool)`
  accessor would carry the same information, so the enum's weight is in the doc, not the code.
- `insert.rs:1206` (Q2/Q6, **likely**): `StrutFacing::ClosedEdge` is now a variant no caller
  treats differently from `Unnamed`. The type keeps a distinction the rule no longer draws.
- PR body (Q6, **sure**): "That path is unreached" is a measurement-based claim with no guard
  and no stated reason at the claim site (MINOR 1).

REVIEW COMPLETE
