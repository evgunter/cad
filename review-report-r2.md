# PR #1093 (GUI-1) — review R2

Frozen head `568bda3363a0ebc6c6fb4f31ebac0b97aa203ba7` on `gui/gui-1-ray`.
Branch `gui/gui-1-review-r2`. Blinded: no A/B material read.

## Verdict

**APPROVE-WITH-FIXES**, conditional on hosted green (my local runs are unique
signal only; the pinned suites ride the PR gate).

The unit does what the spec asks and does it well. Two of the three findings are
doc-accuracy against claims the PR argues at length; one is a real ergonomic hole
in the service's error posture. Nothing found is merge-blocking on its own.

---

## MAJOR

None.

---

## MINOR

### M1. The conservative-superset contract has an undocumented precondition: IEEE overflow

`crates/bvh/src/ray.rs:82-108` (`axis_interval`). The doc-comment enumerates the
IEEE corners — NaN, `d = ±0`, zero extent, inverted boxes — and then argues the
rounding cover ("≤ 3 roundings per endpoint, relative error `< 4·2⁻⁵³`, 4 ULPs
outward covers it with margin"). The rounding arithmetic is right, and I verified
it: 3 roundings give `(1+u)³ − 1 < 4u`, and 4 ULPs of a finite `v` exceed `4u·|v|`
strictly, so the widening is sufficient *where every intermediate is finite and
normal*. What the analysis never mentions is **overflow**, and overflow is not a
rounding: when `bound − origin` overflows to `±∞` while the exact quotient
`(bound − origin)/d` is finite, the axis reports `near = far = ±∞`, and the fold
then prunes a box the ray truly meets.

Exact witness, reproduced under `Bvh::ray` (committed as an `#[ignore]`d row,
`crates/bvh/tests/ray_r2.rs::overflow_in_the_slab_subtraction_drops_a_true_intersection`,
and as `review/gui1-r2-probes/slab2.rs`):

    origin (1.7e308, 0, 0), dir (−1e300, 1, 0)
    box  [−1.7e308, −1.6e308] × [0, 1e9] × [−1, 1]

At `t = 3.35e8` the ray is inside the box on every axis (`x = −1.65e308`,
computed without overflowing the product; `y = 3.35e8`; `z = 0`). `Bvh::ray`
returns the empty vector. A second, structurally identical hole exists when
`1/d` overflows — any `|d|` below about `5.6e-309` makes `inv = ±∞`, and the
same finite-quotient-reported-as-infinite failure follows.

Unreachable at CAD magnitudes, and I would not block a merge on it. But the
crate's header calls the conservative-superset contract "load-bearing" and the
corner list reads as exhaustive, so the honest disposition is one of: scope the
doc-comment to inputs where `bound − origin` and `1/d` stay finite and normal, or
add the guard. Choosing "scope the doc" is fine; choosing to leave the claim
unqualified is not, because the next consumer will read it as universal.

### M2. `d = 0` with the origin outside the slab prunes on only one side

`crates/bvh/src/ray.rs:68-72`: *"`d = 0` with the origin strictly outside the
slab: both products are the same infinity, so `near = far = ±∞` and the caller's
`t_min ≤ t_max` verdict prunes — the ray is truly parallel to and outside the
slab, so the prune is exact, not just legal."*

Only the `−∞` side prunes. On the `+∞` side (origin *below* the slab, `d = +0`),
`widen_down(+∞) = next_down⁴(+∞) ≈ 1.7976931348623151e308`, `widen_up(+∞) = +∞`,
so the axis contributes `t_min ≈ f64::MAX`, `t_max = +∞`, and the box survives as
a candidate whenever no other axis caps `t_max` below `MAX`. Demonstrated in
`review/gui1-r2-probes/slab3.rs`:

    box [10,20]×[−1,1]×[−1,1], origin (0, 0.5, 0.5), dir (0,0,0)
      -> Some(1.7976931348623151e308)     (below the slab: NOT pruned)
    same box, origin (30, 0.5, 0.5)
      -> None                             (above the slab: pruned)

An extra candidate is legal under the one-sided contract, so this is not a
correctness bug — it is the doc claiming an exactness the code does not have.
The row that guards this corner, `ray.rs::zero_direction_outside_slab_prunes`,
picks the `−∞` side, so its premise excludes the mode that behaves differently
(style-lane Q3).

### M3. `(node, body)`/mesh provenance is unchecked, and a mismatch answers a confident wrong name

`crates/editor-core/src/resolve/pick.rs:214-222` (`PickTarget`): *"The (node,
body) pair must be the one the mesh was tessellated from."* Nothing enforces it.
Sibling extrudes mint face keys in their own arenas, so the keys collide
numerically; pairing body A's `MeshPick` with node B produces a `PickHit` whose
`name` belongs to B's face table — no error, no miss, a plausible answer a
selection consumer cannot tell from a right one. Verified: the `#[ignore]`d row
`gui1_pick_r2::a_mesh_paired_with_the_wrong_node_does_not_answer_a_name` fails
today with exactly that outcome.

The PR's error posture is otherwise excellent — node standing is checked up
front, `Unnamed` propagates verbatim, misses are typed. Provenance is the one
mistake in the same family that the types do not catch, and it is also the one a
GUI-2 cache keyed by `(epoch, node, body)` will actually make when the key drifts
by one field. A content hash on `MeshPick`, or a `MeshPick` that carries the
`(node, body)` it was built for, would close it; so would saying at the call site
that it is deliberately unguarded and why. The current state — an invariant
asserted in prose and enforced by nothing — is the weak spot.

---

## NOTE

- **N1.** The documented index tie-break (`ties by ascending input index`) has no
  row that reliably goes red. Deleting `.then(a.item.cmp(&b.item))` from
  `tree.rs:197` left the whole bvh suite green on 2 of 5 mutant runs and red on 3
  (the seed-varying sweep catches it by luck). Cause is structural: the traversal
  is left-child-first over an already-ordered `items` array, so within a tie group
  the pre-sort sequence is normally *already* index-ascending and
  `sort_unstable_by` has nothing to disturb. Two deterministic constructions I
  tried (64 identical boxes; 200 boxes in two exact tie groups) both failed to
  reach the mutant; the surviving one is committed as an explicitly-labelled
  evidence row, not a gate. Runs in `review/gui1-r2-probes/mutate3.sh`.
- **N2.** The early-out's strict `<` (`pick.rs:263`) survives mutation to `<=`
  against all 13 rows. I believe this is an **equivalent mutant, not a test gap**:
  `t_enter` is widened down 4 ULPs, so `best.t == cand.t_enter` is unreachable
  except at the `t = 0` floor, where candidate order (ascending index within a
  `t_enter` tie) already coincides with the tie-break. The strict `<` is still the
  right conservative choice; it simply cannot be distinguished by test.
- **N3.** `pick.rs:322` — the trailing comment on `if det == 0.0 { return None; }`
  says "NaN det also lands here via the comparisons below being false". It does
  not land *here*; a NaN determinant passes this arm and is refused two checks
  later by `(0.0..=1.0).contains(&u)`. The behaviour is right, the sentence points
  at the wrong line, and it is the longest line in the file.
- **N4.** Run 33093540858's single failure is real but its shard stopped at 926 of
  2025 tests (nextest fail-fast, 1099 not run). "Red on exactly the census, every
  other job green" is accurate as written; that run is not evidence about the rest
  of that shard. The two later runs are.
- **N5.** `MeshPick::triangle_count()` has no consumer in the PR and no test.

---

## Style

Questions exercised: Q1, Q2, Q3, Q4, Q5, Q6, Q7, Q8. (Q4 found nothing: no gate
removed, no precondition relaxed, no state split — this PR only adds.)

- **S1 — Q1, `crates/bvh/src/tree.rs:104-137` vs `159-198`.** `Bvh::ray`'s
  traversal is a near-line-by-line copy of `Bvh::overlapping`'s: same stack setup,
  same `nodes.get` unreachable arm, same leaf/inner match, same push-right-then-
  left, same `items.iter().skip().take()`. It differs in exactly two places (the
  predicate, and what gets pushed). It is disclosed at the copy site ("Same fixed
  traversal shape as `overlapping`"), which is more honesty than most copies get,
  but the disclosure is the tell: a third query — GUI-2's snapping, or the SSI
  duty the crate docs already bank — makes this three. A traversal parameterised
  by a prune predicate and a collector would have one home for the D9 ordering
  argument instead of two. `sure` that it is a duplicate; `unsure` whether the
  generic version is worth its own weight today.
- **S2 — Q1, `crates/bvh/src/tree.rs:197` vs `crates/bvh/tests/ray.rs:167`.** The
  documented candidate order is written out twice, verbatim, as
  `a.t_enter.total_cmp(&b.t_enter).then(a.item.cmp(&b.item))` — once as the
  implementation and once inside the sweep's `brute` oracle. Nothing in prose
  admits it. This is not merely aesthetic: it is *why* the tie-break mutant
  survives (N1), because both spellings change together in the reviewer's head but
  only one changes in the mutant, and the sort's stability then hides the
  difference. `likely` a real problem.
- **S3 — Q1, `crates/bvh/src/ray.rs:98-105` vs `crates/bvh/src/aabb.rs:131-137`.**
  Two outward-widening idioms in one crate with different constants: `Aabb::padded`
  goes one ULP outward per bound, `axis_interval` goes four. Each argues its own
  constant; neither mentions the other, and a reader who learns "we widen by one
  ULP" from `aabb.rs` will misread `ray.rs`. `likely`.
- **S4 — Q2, `crates/editor-core/src/resolve/pick.rs:24-34`.** "a consumer keys
  its `MeshPick` cache by (`Evaluation::epoch`, node, body) and drops entries whose
  epoch is stale" is an invariant nothing enforces, stated as if it were a
  contract. Same sentence-shape as `PickTarget`'s provenance line. See M3 for what
  it costs. `sure`.
- **S5 — Q2, `crates/bvh/src/ray.rs:75-95`.** The rounding paragraph is 12 lines
  of justification for 4 characters of code (`.next_down()` × 4). The style brief
  says unusual justification length is mild evidence something is worth flagging,
  and here it was: the paragraph is where M1's gap lives — it is careful about
  everything except the case it does not name. `sure` that the length is a signal;
  `likely` that the right response is a shorter claim, not a longer one.
- **S6 — Q3.** `ray.rs::zero_direction_outside_slab_prunes` names a general corner
  and exercises only the half of it that behaves as documented (M2). `sure`.
- **S7 — Q5, `crates/bvh/src/ray.rs:1-11`.** The module header promises the test
  "must never answer 'disjoint' for a box the ray truly intersects" — an
  unqualified universal, refuted by M1. `sure` the sentence needs a qualifier.
- **S8 — Q7, `crates/editor-core/src/lib.rs:117`.** `pub use bvh::Ray;` puts a
  `bvh` type in `editor_core`'s root namespace so layer 3 "needs no direct bvh
  dependency" — but layer 3 will need `bvh` the moment it wants `Bvh` or
  `slab_enter` for anything else, and meanwhile `editor_core::Ray` and `bvh::Ray`
  are two paths to one type with no doc saying they are the same. I would have
  taken the dependency. `unsure` — this is taste.
- **S9 — Q6/Q7, the census family.** `Ray` is dispositioned into the
  `NOT_CARRIED` "hit-test service" family, but it is a `bvh` re-export, not an
  editor-core concept; the family paragraph reads as if all six names are the
  service's own. Harmless, slightly dishonest as a grouping. `likely`.
- **S10 — Q8 (whole-file read: `crates/editor-core/src/resolve/pick.rs`, 346
  lines, end to end).** Nothing accumulated — the file is new and coherent, the
  module header matches its contents, and the private/public split is real. The
  one thing a full read surfaces that a diff read does not: every one of the
  file's four public items carries a paragraph of contract prose, and three of
  those paragraphs (`PickTarget` provenance, the epoch cache, the tie-break) state
  obligations on the *caller*. The tie-break one is enforced by code; the other two
  are not. `sure`.
- **S11 — Q7, `crates/editor-core/Cargo.toml:88-92`.** `mesh` moves from
  `[dev-dependencies]` to `[dependencies]` and the comment that justified the
  dev-dep ("M5 PR 11: the corpus curved documents gain tessellation columns") is
  deleted rather than merged into the new one. The new comment explains the
  service's need; the corpus's need is now undocumented and invisible. `likely`.

---

## Code quality report

**Counts.** MAJOR 0 / MINOR 3 / NOTE 5, plus 11 style findings.
Spec deviations reported by the author: 1 (edge/vertex proximity picking, which
the spec explicitly permits skipping — a scoped exclusion, not a deviation, and
correctly labelled as such; no schedule owed). **Silent deviations found: 0.**
Neither the census disposition nor the manifest changes are deviations.

**Idiom and structure — 4/5.** The layering is right, the private/public split is
real (`PickTri` and the winner struct hold the arena keys and never escape — my
mechanical scan of `pick.rs`'s public surface confirms it), and the fail-safe
directions are chosen consistently (poison never prunes, NaN never witnesses
disjointness, `unwrap_or_else(Aabb::poison)` on an unreachable arm rather than a
panic). One point off for S1/S2/S3: three duplications inside one small diff, one
of which measurably weakens a gate.

**Test quality — 4/5.** The 16 shipped rows pin real contracts and mostly go red:
I mutated eight ways and the suite caught six (leaves accepting on the hull; the
widening removed; the `t ≥ 0` floor removed; NaN witnessing disjointness; open
triangle boundaries — 7 rows red; the tie-break flipped; the up-front standing
check removed). Two survived: the index tie-break (N1, a genuine gap) and the
early-out's strict `<` (N2, which I judge unfalsifiable rather than untested). The
randomized sweep follows `memories/test-suite-cost.md` correctly — counterexample
shape, `fuzz::start`, no fixed seed, `fuzz::scaled` counts, `fuzz::replay()` in
every message. Its one soft spot is that its `brute` oracle calls `Ray::slab_enter`,
so the "realized == idealized" row pins tree-shape independence and nothing about
the slab test itself; the constructed-true-hit assertion is what carries the
contract, and it carries it over one draw per case. Neither shipped sweep has an
anti-vacuity floor, and this repo has `test_utils::vacuity` for exactly that — my
own first draft of the exact-integer sweep drew only 4 true hits in 80 cases and
would have passed green without one.

**Doc/comment honesty — 3/5.** Unusually thorough, and wrong in two named places
(M1, M2) plus one misplaced sentence (N3), with two caller obligations asserted as
contracts that nothing enforces (M3, S4). The PR body is accurate everywhere I
checked it against the tree, including the CI claims. The deduction is for the
pattern: the places where the prose is longest and most confident are the places
where it is not true.

---

## What the end-to-end exercise revealed

I authored documents, evaluated, tessellated and picked entirely through the
public doors (`crates/editor-core/tests/gui1_pick_r2.rs`, 8 rows), and the
ergonomics are good: `MeshPick::build` → `PickTarget` → `pick_face` is three
lines and reads correctly. Three observations:

1. **The scope is right and the answer is right.** 9,000 randomized
   viewport-shaped rays against two bodies, differenced against my own
   plane-and-edge-cross-product ray/triangle test scanning every triangle with no
   BVH and no early-out: 4,208 agreed hits, 4,792 agreed misses, **zero
   disagreements on `t` or on face identity**, and no near-ties at all. Adversarial
   shapes — a ray down the body diagonal into the (1,1,1) corner where three faces
   meet, a ray lying in the `z = 1` face plane, a collinear sliver triangle, a
   collapsed triangle, three-body occlusion peeled in `t` order with the targets
   presented in reverse — all answer correctly and repeatably.
2. **The pncad façade cannot reach the service at all.** `pncad` re-exports `mesh`
   whole, so a Rust consumer can author and tessellate through the façade and then
   has nowhere to go: `pick_face` is `NOT_CARRIED` and `editor_core` is not
   re-exported whole. That is the census disposition working as intended, and it
   means GUI-2's viewer takes a direct `editor-core` dependency. Fine, but it
   should be a decision someone made, not a consequence someone discovered.
3. **`editor-core` had no `test-utils` dev-dependency.** Any randomized sweep in
   that crate needs one added; I added it (dev-only, leaf crate, matching every
   other crate's row). Worth knowing before the fix pass promotes my suite.

---

## Claims to falsify — disposition

1. **Conservative superset — REFUTED at extreme magnitudes, survives everywhere
   else.** Exact-integer differential (9,848 true intersections at
   `CAD_FUZZ_EFFORT=120`): zero drops. Overflow in `bound − origin` and in `1/d`
   breaks it (M1). Rounding count (3) and 4-ULP sufficiency: both correct as
   argued. `d = 0` outside-slab "exact prune": half true (M2).
2. **`t_enter` a conservative lower bound; early-out never skips a true nearest
   hit — SURVIVES.** Checked against the exact rational entry parameter on every
   true hit in the sweep, and the argument holds: any hit `t ≥` true entry `≥
   t_enter`, so `best.t < t_enter` licenses the break. Exact ties are not skipped
   (strict `<`), though no test can show it (N2).
3. **Determinism / tree-shape independence — SURVIVES, strongly.** My permutation
   sweep rebuilds each box set under a random permutation and demands bit-identical
   `t_enter` and identical documented order after remapping — a check that shares
   no code with `slab_enter`. Green at every effort. The realized == idealized row
   asserts set AND order and does go red (mutant M2: leaves accepting on the hull).
   The index half of the order is only probabilistically pinned (N1).
4. **G1 boundary — SURVIVES.** Mechanical scan of `pick.rs`'s public surface finds
   no arena key type (committed as a row). `Ok(None)` is the typed miss;
   Failed/Poisoned/NotEvaluated answer up front in slice order (mutant ME4 red);
   `Unnamed` propagates verbatim by `?` — untested, and I could not construct it
   either. One gap: the boundary holds for *keys*, not for *provenance* (M3).
5. **Möller–Trumbore closed boundaries — SURVIVES.** Opening the boundaries reds 7
   rows. A corner ray (three incident faces) hits; an edge ray hits both faces and
   resolves to the earlier patch; flipping the tie-break reds the shared-edge row.
   The tie-break the code implements is what the test pins.
6. **`MeshPick` self-contained — SURVIVES on the copy, FAILS on the story.**
   `PickTri` holds `Point3<f64>` copies, so no borrow desync is possible. The
   invalidation story is in the module docs, which is where a consumer would find
   it. But it is prose only, and M3 is what that costs.
7. **Census disposition — HONEST, and I judge NOT_CARRIED the right call.** The
   family semantics ("argued by family, in that constant's docs") are satisfied,
   and the precedent is decisive: `HitTestError`, `MeshPatchKey`, `EntityKey`,
   `EntityRef` are already `NOT_CARRIED` — the entire hit-test inversion family is
   interior, and the picking door is the same family one layer up. Carrying it
   through `pncad::select` would put viewport rays and mesh indexes on the Python
   authoring surface for a consumer that does not exist. Adjudication input, not a
   fix. Minor quibble at S9.
8. **Manifests — SURVIVE.** `mesh/interval` and `mesh/probe` both exist on `mesh`
   and are purely additive to `editor-core`'s existing forwarding rows. `bvh` has
   no features to forward. The wasm32 guard is `cargo check --workspace --exclude
   pncad --exclude pncad-py --features interval --target wasm32-unknown-unknown` —
   `bvh` and `mesh` were already inside `--workspace`, so the guarded surface is
   genuinely unchanged, and the step is green on the frozen head. `test-utils`
   stays dev-only (the "test-only features are dev-dependency-only" discipline step
   is green).
9. **CI claims — VERIFIED at the step level.** 33093540858 (c48a58f): exactly one
   failed job, `test (interval, eps = default, 1/2)`, step `run archived tests`;
   log shows the sole FAIL is `pncad::all
   every_document_layer_root_export_is_carried_or_listed` naming the six new names.
   Caveat at N4. 33095274605 (1fc79cb): 17 jobs success, 4 skipped, **zero failed
   steps anywhere**, drawn `eps = 1e-6`, both shards. 33096924504 (568bda3): same
   shape, drawn `eps = 1e-12`, both shards, `run archived tests` success on 1/2 and
   2/2. All three draws are as the PR body states.
10. **Test quality — SURVIVES with two named gaps.** 16 rows, all pinning real
    contracts, 6 of 8 mutants caught. Sweep shape is correct per
    `memories/test-suite-cost.md`. Gaps: no anti-vacuity floor on either sweep, and
    N1.

---

## Lane isolation

No other review lane's branch, scratchpad or artifact was fetched, checked out or
read. One incidental disclosure: `with-build-slot.sh`'s busy messages print the
holder's command line, so I saw `cargo test -p editor-core --test all --
review_gui1_r1 gui1_pick` and `cargo test -p viewer` while queueing for the mutex.
Command lines only — no branch content, no findings, no output. I did not act on
either.

## Interruption

The container restarted mid-review (around 17:43 local). The worktree survived
with the bvh suite uncommitted; nothing was lost, and every measurement taken
before the restart that mattered (the bvh mutation rounds) was re-run afterwards.
Wall-clock gap across the restart: a few minutes; total elapsed wall clock for the
review approximately 2h10m, of which perhaps 25 minutes was spent queueing on the
machine-wide build mutex behind other lanes. Token spend: roughly 260k.

## Artifacts on this branch

- `crates/bvh/tests/ray_r2.rs` — 7 rows (1 ignored): the exact-integer superset
  differential with vacuity floors, permutation invariance, the tie-break evidence
  row, face-plane/edge/corner grazes, empty inputs, and the M1 witness.
- `crates/editor-core/tests/gui1_pick_r2.rs` — 8 rows (1 ignored): the brute-force
  nearest-hit differential with vacuity floors, corner/coplanar/degenerate/
  three-body rows, degenerate rays, the public-surface arena-key scan, and the M3
  witness.
- `crates/editor-core/Cargo.toml` — `test-utils` dev-dependency (needed by the
  above; dev-only).
- `review/gui1-r2-probes/` — the standalone float probes (`slab*.rs.txt` — `.txt` so the pre-push rustfmt gate leaves them alone; compile with `rustc -O -o /tmp/p file.rs.txt --crate-type bin` after copying to a `.rs` name; compiled
  with plain `rustc`) and the four mutation scripts.

Both suites are registered in their crates' `tests/all.rs`, are rustfmt-clean and
clippy-clean, and are green locally (`bvh` 27 passed / 1 ignored; `editor-core`
gui1 13 passed / 1 ignored). High-effort runs: `CAD_FUZZ_EFFORT=120` on bvh,
`CAD_FUZZ_EFFORT=60` on editor-core — both green, evidence lines quoted above.
