# MSOLVE-14 — The mate solve runs at the evaluation's own scalar (spec)

Unit of the `msolve` program. Item: `work/msolve/MSOLVE-14.md`.

It builds Ev's ruling on `[ev]` PR 3679 (2026-10-01), which `ASSEMBLY.md` A11 (5) now states in place:
- the solve's inputs are read at the evaluation's own scalar and over its own parameters;
- a pattern's count and a `Part`'s index are read at the nominal, because no box or seed binds them.

It answers two rows:
- `from-face-frame-under-an-analysis-lane-refuses-unpinned` (P1);
- `a-mate-through-a-parametric-placer-is-solved-at-the-nominal-in-box-and-seed-runs` (P2, including its "Widened" section on `check_offsets`).

Read both rows, and the two designers' reports on #3679 (in the PR body), in full.

**Track:** a large mechanical generification with a correctness core. **Review:** dual, under `docs/DUAL-REVIEW-PROTOCOL.md`. The unit changes how every analysis run reads an assembly, the memo key's inputs, and a public payload (`Unpinned`'s producer goes). **Dispatches after MSOLVE-13 merges**, because both touch `mate/member.rs` and `mate/solve.rs`.

## What the tree says now

1. **The solve runs once, at the nominal, for every lane.** `evaluate` calls `mate::solve_with_env(doc, &nominal_env, &reach, tol)` (`eval/mod.rs`) before any node's bracket opens. Instances lift that `f64` answer into the lane as a constant, with three consequences:
   - **Face frames refuse on analysis lanes.** A `FromFace` side crosses the solve through `SectionScalar::pinned_f64` (`face_pose_over_cache`), and on an analysis lane it refuses `FacePoseRefusal::Unpinned`.
   - **Parametric placers are silently held at the nominal.** A `Transform` or `Pattern` placer whose slots the run binds stays at the nominal pose.
   - **`check_offsets` is decided at the nominal too.**
2. **The structure and the placement are mixed in one `f64` value.** The structure (members, groups, the spanning tree, roles, faults) depends only on the recipe. The placement (the solved relative poses) is geometry that depends on the parameters. `SolvedPoses` carries both as one `f64` value.

## What the unit builds

**1. One solve, generic over the scalar.**
- `solve_with_env<T>` takes `ParamEnv<T>` and a `MateReach<T>`, and the evaluation calls it once with the lane environment it has already built.
- `solve_document`, used by the edit door, the viewer and Python, is the `f64` instantiation. At `f64` it is bit-for-bit today's solve. Pin that with a fence row in the manner of `m10_p_fence`: the solve's value channel at `Dual64` equals `solve::<f64>`, and at `f64` the poses are byte-identical to `main`'s on the mate corpus.

**2. What goes generic, and what stays nominal.**

*Generic over `T`:*
- the side frames, where an `Authored` frame lifts through `from_f64` and a `FromFace` frame is `face_pose` read straight off the part's `T` product;
- the placer maps (`derived_offset`, `pattern_map`, `transform_map`, a gauge's placement);
- `Coset<T>`, `Subgroup<T>`, `Measured<T>` and every levered predicate (decided at `T` through `Decide`);
- `check_offsets`;
- the solved poses (`SolvedPoses<T>`).

*Stays at the nominal:*
- `read_mates`, `check_references`, the groups, `by_pair` and the spanning tree, which are recipe data;
- the pattern counts and `Part` indices read by the two per-reference checks, which no box or seed binds;
- the reach (the lever's extent), which stays an `f64` upper bound. For a face-framed side the lever's `‖origin‖` term is an upper bound at `T` (`.hi()` on an interval).
- The edit door's per-mate admission (`admit_mate`), which stays `f64`: the doors decide edits, and the solve decides states.

**3. What each lane decides.**
- At `Dual64`, a predicate decides on the value channel, so a seed never decides a branch (Q1).
- At `Interval`, a predicate is either definite or `Indeterminate`, and the escalation lands on the deciding mate's log (MSOLVE-11's channel) so the driver can bisect.
- Do not add a refusal for a lane: a face frame on an analysis lane resolves. The `Unpinned` arms (`FacePoseRefusal::Unpinned`, `FaceRefusal`'s carry) lose their producer. Delete them, and move the Python tag and census with them. `SectionScalar::pinned_f64` keeps its DM1c section callers.

**4. The memo key (the highest risk).** The solved pose at `T` must feed the instance's and mate's memo keys through `ContentBits`, with both channels at `Dual64` (DL2). Otherwise a seeded pass reuses a zero-tangent pose:
- `stackup` threads an unseeded dual base through its passes as a memo prior;
- a seeded pass that reuses that pose returns a silent zero sensitivity.

Write that row red first.

**5. What it leaves, stated rather than fixed:**
- **A boxed rotation refuses as not rigid.** A boxed rotation (a tilted face frame, a boxed angle) widens a rotation, which `transform_rigid` refuses `NotRigid` at `Interval`. That is TOPO's open row `a-boxed-rotation-refuses-not-rigid-at-every-placer`, and it applies to every placer today.
- **An identically-zero margin cannot converge.** A margin that is zero across a whole box bisects without converging, which is the interval lane's general limit.
- Name both in the PR. Fix neither.

**6. The box driver.** Read (2) of `drive::classify_replay` now meets mate escalations, which are real box decisions. Any mate fault no box can move (walk, class, dangling head, count, unresolved part, `MovedAbove`) should be terminal at read (1) under its own class: `a-box-independent-mate-fault-bisects-the-whole-leaf-budget` (P2). Close that row in this unit if the build reaches the driver with a resolver. Otherwise, re-measure it and leave it open.

## Acceptance

- **A1:** A face-framed mate on a `Dual64` seed run yields the pose's tangent. On an `Interval` box run it yields an enclosure that contains the pose at every box corner the row samples. `msolve9_from_face::a_face_frame_under_a_dual_evaluation_refuses_unpinned` re-baselines to "resolves".
- **A2:** The parametric-placer red row, written first as the item asks. Under a box over a `Transform` placer's parameter, the mated part's enclosure contains its true pose at the box corners. A seed run reports the true sensitivity. Include the worked example from the designers: a patterned bolt with copy #2 mated, where ∂B/∂s is −2 and ∂copy2/∂s is 0.
- **A3:** The `f64` fence: the corpus poses are byte-identical to `main`'s, and the `Dual64` value channel equals `f64`.
- **A4:** The memo row (§4) is red on a key that omits the tangent and green on the built key.
- **A5:** `stackup::sensitivities` and certified `clearance` run on a face-framed assembly. Pin one row each.
- **A6:** Both items close. `work.py lint` is clean.

## Constraints, binding

- **Discipline:** `docs/prompts/implementer-discipline.md` applies in full. Hosted CI is the verification of record. Work merge-only.
- **Fence:**
  - `crates/editor-core/src/mate/` (all of it);
  - `crates/editor-core/src/eval/mod.rs` (the solve's call, `face_pose_over_cache`, the memo key);
  - `eval/wire.rs` (an instance reading a `T` pose);
  - `stackup.rs` and `clearance.rs` (only their rows and a consumer they need);
  - `drive.rs` (§6 only);
  - `pncad-py` (`Unpinned`'s tag and census);
  - tests and the items.

  `geom-core` and `topo` are outside the fence. If a generic door is missing there, STOP and name it.
- **Stop clause:** STOP and write what you measured into the PR as a draft if any of these holds:
  - the `f64` fence moves a bit;
  - a predicate cannot be decided at `T` without a door outside the fence;
  - the memo key cannot carry both channels without changing a key outside `mate/` and `eval/`.

## Review (dual)

These are the claims to falsify:

- **C1:** At `f64` the solve is bit-identical to `main`'s.
- **C2:** On an analysis lane, a solved pose encloses (at `Interval`) or differentiates (at `Dual64`) the true pose. Check this against an independent computation at sampled parameters.
- **C3:** No branch is decided on a tangent. Every interval decision is definite or escalates on its mate's log.
- **C4:** A reused memo entry never carries a pose from another lane or seed.
- **C5:** The structure (groups, tree, roles) is identical in every lane for one document.
- **C6:** `Unpinned` has no producer left, and its deletion crosses Python consistently.
