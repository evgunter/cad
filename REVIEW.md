# Review of #4004: "JOIN: the fan end has one home; strut facing has one rule"

Frozen head `a4181ca0`, base `11d9c7a7`. Full reviewer lane.

**Verdict: APPROVE-WITH-FIXES.** MAJOR 0 · MINOR 2 · NOTE 4.

No wrong body anywhere I could reach; the change is confined to the `_` arm, as the PR says, and every PR measurement
I re-ran holds. The MINORs: an unpinned behaviour change on an unreached path, and stale rule prose.

## How it was checked

- **Probe rows** (`crates/sweep/tests/fan_end_review_probes.rs`, on this branch):
  - `fan_end_review_prism_battery`: corners of five prisms touch a SIDE=10 cube's face on the plane through the corner,
    with `m` over a 400-point Fibonacci sphere. The corners are the L's reflex and convex corners, the 315° corner, a
    200° notch and a ~350° slot, each at the top and the bottom of its prism. Ops ∪ ∩ ∖, both operand orders.
  - `fan_end_review_rim_battery`: a revolved tube's rim vertex (the lone vertex of a closed circle edge) on the face,
    with the same sphere plus 24 normals that hold the rim tangent.
  - Every line is `differential::outcome`: tier 2, tier 3′, the certificate, a legal operand and the volume.
- **The volume oracle is independent of the PR's**: the slab on `m`'s side is `∫_R clamp(L,0,1) dA = ∫L⁺ − ∫(L−1)⁺`,
  from half-plane clips and first moments, with a disc's in closed form. The tube measures π(r²−b²) to 12 digits.
- **Instrumentation** (a scratch copy of head, not pushed): at every strut it logs the `strut_faces_first` step and
  answer, beside `vtxfac`'s old facing and main's `mint_directed` angular `strut_order` (at `arm.min`).

## Claims

**1. The facing rule is right everywhere it is read: HOLDS.**
- Prism battery, release build, 29 087 lines on both builds. main → head moves exactly **5 628** lines, every one a
  `vtxfac` unvoted bare-arm pose.
  - Main refuses all of them `JoinDesync`. On head 2 814 are `OK SOUND` and 2 814 refuse typed (938 `Euler`, 1 876
    `JoinDesync`).
  - The split is exact on geometry, in all 4 reflex profiles, top and bottom. SOUND ⇔ the corner's vertical edge reads
    In (one bare run). Refused ⇔ it also reads Out: two Out runs at one vertex, the PR's filed
    `a-pierce-with-two-out-runs-at-one-vertex-refuses-every-op`.
  - No `BAD` line moved. The 24 `BAD` lines are byte-identical on main (NOTE 4).
- Unvoted, the whole orbit was reached 2 814 times and the bare arm 5 628 times, over reflex sectors from 200° to 350°,
  both operand orders, ∪ ∩ ∖. The angular `strut_order` equals the new walk order on all 8 442 calls. Two independent
  readings of "the germ the corner meets first" agree on every pose that exercises the change.
- Convex sectors cannot hold a `vtxfac` strut, because its two germs are the two rays of one line. Two germs never share
  a `vtxfac` entry either: `build_sectors` bisects every sector of 180° or more (`sectors.rs:220`). So the convex and
  "several germs in one entry" cases are empty on this path, and the `strut_order` step is reachable from
  `mint_directed` only (claim 2).
- Closed edge's lone vertex: 0 `StrutFacing::ClosedEdge` calls in the rim battery (rim-tangent normals included), the
  batteries and the suites. See MINOR 1.

**2. `run_site` is the same strut every old copy minted: HOLDS.**
- By reading: `orbit_step` (`body.rs:1600`) is exactly the old `get_half_edge(mate(last)).next`, and
  `WholeOrbit.mev_site()` is `Fan{first, first}`, the old `he2 == first` site. The splitter keeps its flags `(true, true)`
  (`splitting/insert.rs:106`). `run_degenerates` and `mint_run` keep their semantics; only one error string merged.
- By running the instrumented build over the PR's 7 batteries, the sweep suite (2 026 passed) and the topo suite (890
  passed): about 42 000 unvoted `strut_faces_first` calls. Every one equals main's `strut_order`, including the
  `mint_directed` calls with germs in different entries, where head now reads `precedes`.
- The only disagreements are the 6 bare-arm calls of the PR's new row. The topo suite's bare-arm mints are 59 voted
  plus those 6, in line with the PR's "60, all settled by the vote".
- I did not re-run the PR's 132 251-line diff byte for byte. My own-shape battery diff is above, and the facing
  comparison over the same batteries covers its premise.

**3. The new rows can go red: HOLDS.** Release build; the `join*` rows (30) plus the `run_site` unit row.
- **M1–M4** reproduce the PR's kill sets exactly; M1 also kills `r2_multi_spike_corner_meet_is_tier_three`, and M3
  kills only `a_bare_bisector_strut…`, the `_` arm's pin.
- **M5**, mine (`precedes` anchors on `start_edge` instead of `end_edge`): red on the whole-orbit row and
  `the_reflex_315…`.
- **M6**, mine (`precedes` returns the raw `p.0 < q.0` with no wrap anchor): survives every row and my whole battery (0
  of 29 087 lines move). It is an equivalent mutant: `build_sectors` pushes each sector's own entry before its twin, so
  no physical sector straddles index 0. See NOTE 2.

**4. The sweep is complete: HOLDS.** Shaped by role, not symbol: every production `NewVertexSide` choice, every
`created.he_plus`/`he_minus` binding, every `.prev` in `boolean/`, `splitting/`, `null.rs`, every `vertex_orbit(`.
The only new hits are parallel *side* spellings, not fan ends: the splitter's `whole_orbit → Below`
(`splitting/insert.rs:124`), and the ring struts (`vtxfac.rs:747-759`, which the PR filed). Blind spot: a facing
computed through an index into a precomputed orbit array.

**5. The oracle is exact: HOLDS.**
- `the_prs_simpson_oracle_agrees_with_the_clamp_oracle` (a copy of `prism_beyond`) agrees to <1e-12 on the PR's 2
  poses and 200 normals; `the_clamp_oracle_matches_a_brute_slice_sum` checks the clamp oracle on 4 000 slices.
- The argument holds: the area is quadratic in z between the heights where the cut line passes a profile vertex, which
  are the PR's breakpoints.

## Findings

- **MINOR 1: the closed-edge path changed behaviour with no row** (`vtxfac.rs:627`, `insert.rs:1206`).
  - Main refused it typed (`CurvedBooleanUnsupported`); head decides it by entry order. 0 calls reach it in every run
    above. The PR discloses "That path is unreached" with no row, schedule or claim-site reason (prompt Q6).
  - Entry order is principled, and agreed with the angle wherever measured. But a fail-loud refusal became an unverified
    answer. Pin it with a row, or keep the refusal.
- **MINOR 2: stale rule prose.**
  - `boolean/join.rs:92` still presents `insert::strut_order` as "the angular strut spike order" that ranks a strut's
    germs. It is now step 3 of `strut_faces_first`, reachable from `mint_directed` only.
  - The PR fixed `join.rs:2156` but not :92; the same phrase is in `tests/m3_pr5_boolean_ops.rs:196` and `tests/review_m3_pr5.rs:387`.
- **NOTE 1: two orderings within one entry.**
  - `precedes` orders same-entry cuts itself (`walks_after`, `insert.rs:1086`), but `strut_faces_first` sends that case
    to `strut_order` (`insert.rs:1208-1220`).
  - `mint_directed` reads both on one strut's germs: `from_is_lo` through `precedes`, the facing through `strut_order`.
  - They agree while entries are under π (bisection guarantees it): not a bug, but two spellings of the last step.
- **NOTE 2: dead code in `precedes`.** Its backward walk (`insert.rs:1091-1099`) is equivalent to `p.0 < q.0` under
  `build_sectors`' push order (M6). The layout assumption it relies on is unstated.
- **NOTE 3: sizing for the filed P0.** Half of all bare-arm poses (2 814 lines here) fall in the filed two-run issue.
- **NOTE 4: pre-existing, identical on main, out of scope.**
  - A tube's rim vertex `(1,1,0)` touching a face whose plane holds the rim tangent (`m = (cos θ, sin θ, 0)`): ∪
    builds with the exact volume but `t3p=false`, in both orders (24 `FANREV tube … U` lines). Reported for routing.

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q6 and Q7. Q5 was read against the `RunSite` and `strut_faces_first` docs only. Q8
was **not** done end to end: I read `vtxfac.rs` 440-780 and `insert.rs` 840-1560, not either whole file.

- `vtxfac.rs:628-632` (Q7, **likely**): `vtxfac` destructures the `MevSite` it just built to recover the corner, behind
  an unreachable `ClassificationInvariant` else-arm. `RunSite::WholeOrbit { corner }` already names it, and the bare arm
  knows `after.he`.
- Side-follows-facing is spelled three times (Q1, **likely**): `splitting/insert.rs:124-140` hard-codes `whole_orbit →
  Below`, `vtxfac.rs:648-660` derives `start_on_plus`, `mint_run` swaps on `dangling && !spike_from_first`
  (`insert.rs:1528`, `:1560`). Class: also the ring struts (`vtxfac.rs:747`), which the PR filed.
- `insert.rs:1190` (Q7, **unsure**): `strut_faces_first` takes positional `(usize, Cells, Vec3)` tuples, which `vtxfac`
  builds through a closure, while each caller has its own `Germ<T>` of a different shape.
- `euler.rs:522` (Q7, **unsure**): every caller writes `site @ RunSite::… => (site.mev_site(), flag)`. The enum's
  weight is in its doc rather than in what the callers do with it.
- `insert.rs:1206` (Q2/Q6, **likely**): `StrutFacing::ClosedEdge` is now a variant no caller treats differently from
  `Unnamed`. The type keeps a distinction the rule no longer draws.
- PR body (Q6, **sure**): "That path is unreached" is a measured claim with no guard and no claim-site reason
  (MINOR 1).

REVIEW COMPLETE
