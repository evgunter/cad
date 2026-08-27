# Review R1 — PR #1093 (GUI-1: `Bvh::ray` + hit-test service), frozen head 568bda33

**VERDICT: APPROVE-WITH-FIXES** (conditional on hosted green; pinned suites ride the PR gate).
One verified contract refutation at adversarial magnitudes (MAJOR below); everything else survived
attack. The unit is well-built: honest docs, real tests (mutants confirm they gate), clean G1 boundary.

Session note (fair-pair rule): this review was interrupted by a container restart ≈17:36–17:39 UTC
and resumed in the same worktree; no measurements were in flight (the state on disk survived and
every suite/mutant run reported below was completed or re-run after the resume).

## Findings

- **MAJOR (verified refutation of claims 1 and 2): intermediate overflow breaks the conservative
  superset.** `axis_interval`'s rounding argument (`crates/bvh/src/ray.rs:77-95`) assumes the three
  roundings produce small RELATIVE error; when `lo − o` overflows (finite box, finite origin,
  |lo|+|o| > ~1.8e308) the products come back ±∞ with no NaN, the skip-arm never fires, and
  `widen_down(+∞) = MAX` mints a near ≈ 1.8e308 on an axis whose TRUE interval is moderate. Another
  axis's honest `far` then empties the fold and a truly-intersected box is pruned. Reproduced:
  `crates/bvh/tests/review_gui1_r1.rs::overflow_prunes_a_truly_hit_box` (committed RED at the frozen
  head, the regression row for the fix): box [-1.5e308,-1e308]×[0.5,1.6]×[-1,1], ray o=(1.7e308,0,0),
  d=(-1.7e308,1,0) — true hit at t≈1.59, `slab_enter` answers None. The same inflation makes
  `t_enter` EXCEED the true entry, so the `pick_face` early-out's license (claim 2) fails at the same
  magnitudes. Practical exposure ≈ nil for tessellated CAD scenes (needs ~1e308-scale coordinates);
  the fix is local (clamp/handle infinite products from finite inputs), or honestly re-scope the
  contract with a stated finite coordinate domain. The 4-ULP widening itself is sufficient in the
  non-overflow regime — my exact-oracle and dyadic sweeps at effort 50 found no other violation.
- **MINOR (doc): `crates/bvh/src/tree.rs:5` module header** still says queries "return candidates in
  ascending input order" — true of `overlapping`, false of the new `ray` (ascending t_enter). Stale
  one screen above the code that contradicts it.
- **MINOR (doc): the "exact prune" claim for d=0-outside is one-sided.** `ray.rs:66-69` (and the PR
  body) say both products land on "the same infinity … and the verdict prunes — exact". On the
  +∞-product side the prune happens only because some OTHER axis supplies a finite far;
  with none (e.g. a zero-direction point-ray strictly below a slab) the box is KEPT with
  t_enter ≈ f64::MAX (probe output in review-probes/gui1-r1-mutants.md). Conservative, so no
  contract breach; the shipped unit test exercises only the pruning sign.
- **MINOR: `HitTestError::Unnamed` carries an `EntityRef` (arena key) out of the service.**
  pick.rs:9-12 claims "no arena key crosses the layer-2/3 boundary", and `MeshPickError` is carefully
  arena-key-free — but the Unnamed bug arm (inherited verbatim from shipped `resolve::hit`, which has
  the same tension in its own header) hands layer 3 a FaceKey inside the error payload. Not new to
  this PR; reachable only on a naming-totality bug. Adjudication input: either bless the bug arm as
  diagnostics or re-payload it in arena-key-free vocabulary (position/debug string).
- **NOTE: `ray_triangle` can return `t = +∞`** (accepted by `t >= 0` when `e2·q × inv` overflows —
  near-degenerate det with moderate coordinates). The comment at pick.rs:271-272 says "the exact test
  refuses non-finite hits"; it refuses NaN only. A winning t=∞ would put NaN components in
  `PickHit::point` (0·∞). Far corner; worth one guard or one honest comment.
- **NOTE: the interval-lane CI draw never completed on the new suites at the final head** — run 1
  (interval draw) died at the census with 1099 tests unrun; the two green runs drew default features.
  Job-log tails show all 6 gui1_pick rows and ≥5 of 10 ray rows DID pass under `--features interval`
  in run 1's shards before the kill, and bvh/editor-core's new code is feature-independent, so
  residual risk is a sliver, not a gap.

## Claims-to-falsify disposition

1. Conservative superset — **REFUTED at overflow magnitudes** (MAJOR above); survived everywhere
   else (exact-rational oracle sweep incl. NaN-trap-by-construction, grazes, zero-extent, poison,
   effort 50).
2. t_enter lower bound / early-out safety — holds in the non-overflow regime (dyadic-exact sweep);
   fails with claim 1 at overflow. Strict-`<` tie handling verified safe by analysis: an equal-`t`
   candidate skipped could only lose the (target, flat) tie-break anyway.
3. Determinism/order — survived. Mutants A (sort dropped) and B (leaf-hull t_enter leak) both
   redden the shipped sweep AND the order row; repeats bit-identical in my sweeps.
4. G1 boundary — survived, with the MINOR Unnamed-payload caveat; typed miss/errors verified through
   public doors incl. first-offending-target order (shipped row) and my suites.
5. Möller–Trumbore closed/both-sided — survived: my dyadic battery pins all 12 edges + 8 corners +
   interior origin against an exact i128 oracle; mutant C (tie-break flipped) reddens the shipped
   shared-edge row. Caveat: "resolves to the earlier patch every time" is exact-tie-only — with
   non-dyadic geometry the two faces' computed t's can differ by rounding and the tie-break never
   engages (no seam though: closed boundaries held in every probe).
6. MeshPick self-containment — survived; my battery even picks against a rescaled cloned mesh
   (copies, not borrows). Invalidation story is in the module docs + MeshPick doc where a consumer
   lands; `Evaluation::epoch` exists (eval/mod.rs:56).
7. Census disposition — **NOT_CARRIED is the better call**, and the paragraph is honest. Consistent
   with `HitTestError`/`MeshPatchKey` already interior; but note pncad DOES carry evaluate +
   tessellation, so the "display-side state" argument is half-true — my e2e consumer authored,
   evaluated and tessellated façade-only and needed a second manifest row (editor-core) ONLY for
   picking. "One crate to depend on" bends here; a curated `pncad::select` picking door is a
   plausible future revisit, exactly as the paragraph says.
8. Manifests — verified: mesh promoted dev→real; mesh/interval + mesh/probe forwarding correct
   (both features exist); bvh has no features to forward; wasm32 guard is workspace-wide
   (`--workspace --exclude pncad --exclude pncad-py`), both crates already inside it; Cargo.lock
   delta is exactly the two dep additions.
9. CI — verified at job/step level via the API: run 33093540858 red ONLY on
   `every_document_layer_root_export_is_carried_or_listed` (exactly the 6 names; fail-fast skipped
   1099 tests in that one job — see the NOTE); 33095274605 green, jobs `test (eps = 1e-6, 1/2 & 2/2)`;
   33096924504 (frozen head) green, `test (eps = 1e-12, 1/2 & 2/2)`; interval jobs skipped in both
   green runs as drawn.
10. Test quality — survived: 16 rows counted, each pins a contract; 3 shipped rows proven
    red-capable by mutants; sweep is a proper counterexample search (fuzz::start varying seed,
    replay logged, scaled(60) on the dial, gating-safe direction).

## Style (per docs/prompts/reviewer-style-lane.md)

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8. Confidence vocabulary as marked.

- (Q5, sure) tree.rs:5 stale "ascending input order" module claim — MINOR above.
- (Q2, sure) The 4-ULP rounding justification is the longest comment in the diff and is the one that
  is wrong (at overflow) — the brief's "justified at unusual length" heuristic scored a direct hit.
- (Q1, unsure) Two outward-widening vocabularies now exist in bvh: `Aabb::padded`'s 1-ulp
  `next_down/next_up` and ray.rs's 4-ulp `widen_down/widen_up`. Different rounding budgets, each
  locally documented; no shared home. Where else to look if unified: any future query adding its own
  widening.
- (Q7, likely) `Bvh::ray` eagerly enumerates and sorts ALL intersected leaves per query, per target;
  the consumer early-out then saves only exact triangle tests, never traversal. A t-ordered
  (priority-queue) traversal is the shape that would let the early-out prune subtrees. Fine for v1
  meshes; the allocation+sort-per-query cost is also taken per target per pick.
- (Q7/Q4, likely) `PickTarget` provenance ("the (node, body) pair must be the one the mesh was
  tessellated from") is held by a doc sentence, while G1 advertises type-level disciplines. A stale
  or wrong (node, body) with a valid mesh surfaces as the Unnamed "THE BUG" arm or, worse, as a
  plausible wrong name — the service cannot detect the mismatch. Same class as `MeshPatchKey`'s
  convention (cited there); if ever fixed, sweep both sites.
- (Q7, unsure) `pick.rs` re-sorts nothing but relies on candidates of ONE target being ascending
  while `best` spans targets; correct (break is per-target), but the cross-target reasoning lives
  only in a comment.
- (Q3, sure) `poison_ray_returns_everything` and `poison_box_is_always_a_candidate` assert exact
  index sets — they can go red. No assertion-free rows shipped in this unit.
- (Q6, sure) The scoped exclusion (edge/vertex picking) is spec-sanctioned and correctly labeled
  "not a deviation"; no unscheduled deviations found. The PR-body t==2.0 dyadic-tessellation
  premise is stated with a loud re-derivation instruction rather than a guard — acceptable as a
  test-local premise; my battery holds the same premise via a loud `int_point` assert.
- (Q8, disclosed) Largest touched file, `crates/pncad/tests/all.rs` (~2950 lines), was NOT read end
  to end — only the census machinery and NOT_CARRIED region; pick.rs/ray.rs/gui1_pick.rs/ray.rs
  tests were read whole, tree.rs ~90%.

## CODE QUALITY REPORT

Counts: 1 MAJOR, 3 MINOR, 2 NOTE. Spec deviations: 1 reported (edge/vertex picking exclusion,
spec-sanctioned), 0 silent found.
- Idiom/structure: **4/5** — clean module seams, arena-key-free vocabularies, fail-safe NaN arms;
  docked for the convention-held PickTarget provenance and eager-enumeration query shape.
- Test quality: **5/5** — rows pin real contracts (three proven red-capable by mutants), the sweep
  is a genuine varying-seed counterexample search with replay logging, and the realized==idealized
  row catches tree-shape leaks including t_enter corruption.
- Doc/comment honesty: **3/5** — unusually thorough and mostly accurate, but the two central
  numerical arguments (4-ULP coverage, exact d=0 prune) both overclaim at the corners, the module
  header order claim went stale, and "refuses non-finite hits" is false for +∞.

## E2E exercise (scope/ergonomics)

Authored documents façade-only (pncad::document apply/evaluate + pncad::mesh::tessellate), picked via
editor_core. Occlusion, cross-target ties, |dir| scaling, silhouette grazes (in-plane face invisible,
side face's closed edge selectable — good for GUI-2), far origins to 1e300, and a 1e-6 sliver body
(wafer cap and 1e-6 side face both pick) all behaved; a 1e-9 sliver is refused upstream by profile
authoring. Ergonomics: the census decision means a headless picking consumer needs a second manifest
row; PickHit's name+node+t+point is exactly what a selection value needs.

## Process disclosures

- Container restart ≈17:36–17:39 UTC mid-review (disclosed above); resumed same worktree, no loss.
- Lane isolation: no other review lane's branches/scratchpads/artifacts were fetched or read. The
  build-slot lock's holder line incidentally displayed other lanes' COMMAND STRINGS
  ("cargo test -p viewer", "cargo test -p editor-core -- gui1") — metadata only; nothing further seen.
- Deliverables on branch gui/gui-1-review-r1: this report, review-probes/gui1-r1-mutants.md,
  crates/bvh/tests/review_gui1_r1.rs (one row deliberately RED at the frozen head — the MAJOR's
  regression row), crates/editor-core/tests/review_gui1_r1.rs (+ test-utils dev-dep).
- Cost: ≈290K tokens; ≈55 min wall clock including the ~3 min restart gap and build-slot queueing.
