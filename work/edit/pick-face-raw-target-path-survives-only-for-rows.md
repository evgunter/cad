---
id: pick-face-raw-target-path-survives-only-for-rows
kind: issue
title: pick_face's raw PickTarget path has no non-test consumer, and its document half is a claim
status: closed
opened: 2026-09-16
closed: 2026-09-17
refs: [2773, 1098]
pr: 2801
branch: edit/raw-target-test-support
---

## What

`pick_face` (`crates/editor-core/src/resolve/pick.rs`) takes a slice of
`PickTarget`s, and a `PickTarget` has two mints: `NodePick::target`,
where the document, the node, the body and the mesh all come from one
tessellation, and `PickTarget::new`, where the caller declares them
over a `MeshPick` of its own.

**The raw mint has no consumer outside tests.** Measured on PR 2773's
tree: `crates/viewer` offers targets only through `NodePick::target`
(`pickindex.rs`, `PickIndex::pick_for`), `crates/pncad-py` does the
same (`py/pick.rs`, `t.inner.target()`), and `crates/pncad` does not
carry `MeshPick` at all, so a façade consumer cannot build one
(`select.rs`'s curation comment, and `pncad/tests/all.rs`'s roster).
Every caller of `PickTarget::new` is a row.

**And on that path the document half is a claim, not a check.** PR
2773 made the fields private, so a target minted by `NodePick::target`
cannot be taken apart and re-stamped (a `compile_fail,E0451` row sits
on the type). What remains is a caller who builds a `MeshPick` over one
document's mesh and declares it to be of another: `pick_face` compares
the declaration against the handed evaluation, not against the mesh,
so the door answers a name out of the other document's tables. The row
that measures it is
`edit_pair_apply_names::a_raw_target_is_a_claim_in_every_half`. This is
the same shape as the NODE half, which cannot be checked even in
principle (arena keys collide across sibling nodes of one document) —
issue #1098's residual raw-assembly class — and `PickTarget`'s docs and
A2a now say so in one voice.

## The question

Whether `NodePick::target` should be the SOLE mint — which would make
"the document half is checked" true of the type rather than of one
path, and would close #1098's raw-assembly class at the API rather than
documenting it.

## What that would cost, measured (PR 2773's fix pass)

Four rows, none re-expressible through `NodePick::target`:

- `review_gui1_r1::dyadic_battery_pins_faces_edges_corners_and_tiebreak`
  pairs a mesh scaled ×2 BY HAND with its node so the ray oracle can run in
  exact integers. A `NodePick` only ever indexes the node's own
  tessellation.
- `gui1_pick::unusable_nodes_surface_typed_errors` builds targets
  naming a failed, a poisoned and an absent node, so that `pick_face`'s
  standing ladder has rows. `NodePick::build` refuses all three and
  mints nothing. (The ladder would stay REACHABLE — a target minted at
  one picture and handed a later evaluation in which the node has since
  failed is admitted by the pairing and refused by standing — so the
  rows could be rebuilt around the later-evaluation admission, at the
  cost of being rows about a different thing.)
- `gui1_pick::node_pick_door_is_prepaired_and_typed` compares the
  door's target against the raw path, which would be gone.
- `gui1_pick_r2::a_mesh_paired_with_the_wrong_node_does_not_answer_a_name`,
  #1098's ignored witness, is about raw assembly by construction.

So it is a change to `pick_face`'s public shape and to four TCOST/TINT
rows, not a one-line narrowing — which is why PR 2773 stopped at the
private fields and the honest contract, and left this here.

## Where it stands

`crates/editor-core/src/resolve/pick.rs` is EDIT's
(`work/edit/program.md` paths); the four rows are TCOST's and TINT's.
Found by lane `nodepick-fix` while taking the style review's N1 on PR
2773.

## Ruled and spec'd (2026-09-17, EDIT orchestrator) — middle tier, branch `edit/raw-target-test-support`

**Ruling.** `NodePick::target` is the only mint a consumer can reach.
`PickTarget::new` (and any other raw constructor of a target or a
`MeshPick` over a declared document) moves behind a `test-support`
cargo feature on `editor-core`, so that "the document half is checked"
is true of every target a non-test consumer can hold, by construction.
The four rows that need the raw mint keep it (the feature is enabled
for `editor-core`'s own integration tests and for the one `pncad`
test that uses it — measure how: a self dev-dependency with the
feature, or `[[test]] required-features`, whichever the workspace's
existing pattern is; `pncad` the façade does not carry it).

**What lands.** The feature; the mint gated and `#[doc(hidden)]`-free
(it is documented as the test-support door, not hidden); `PickTarget`'s
and A2a's prose say the document half is CHECKED for every reachable
target and the raw door is test support; #1098's raw-assembly class is
closed at the API and its residual sentence updated; the
`compile_fail` row on the private fields stays; a new `compile_fail`
row (or a `cargo check` of a consumer crate without the feature —
measure which is honest) shows the mint is unreachable without the
feature.

**Rows.** The four rows unchanged; the unreachability row above; the
`a_raw_target_is_a_claim_in_every_half` row's doc re-read (it is now
about the test-support door).

**Territory.** `crates/editor-core/{Cargo.toml, src/resolve/pick.rs}`,
`crates/editor-core/ASSEMBLY.md` A2a (a clause re-worded because the
door moved, not a new decision — say so in the PR body and cite the
CLAUDE.md test) (EDIT); `crates/editor-core/tests/*`,
`crates/pncad/tests/all.rs` (TCOST/TINT/LIB — mechanical). Middle tier.

## Built (2026-09-17, lane `rawtarget`)

**Landed.** `editor-core` gains a `test-support` feature (`[features]`,
enabled by a self dev-dependency — the pattern `profile` and `topo`
already take; nothing in the workspace uses `[[test]]
required-features`). Both raw mints are behind it: `PickTarget::new`
and `MeshPick::build`, each in a `#[cfg(any(test, feature =
"test-support"))] impl` block, so in a build that does not ask for the
feature neither EXISTS. `MeshPick::build`'s body moved to
`MeshPick::build_every_table`, `pub(crate)` in both arms, which is what
`NodePick::build` calls — so a consumer reaches an index only by
holding the `NodePick` that built it. Neither mint is
`#[doc(hidden)]`: both are documented as the test-support door.
`PickTarget`'s docs, the module header, `ASSEMBLY.md` A2a, the façade
curation comment (`pncad/src/select.rs`), the `pncad` roster's
reasoning and `pncad-py`'s `py/pick.rs` all now say the same thing —
every half of every reachable target is true by construction, and
#1098's raw-assembly class lives where the feature does. The
`compile_fail,E0451` row on the private fields is untouched.

The feature also FORWARDS `profile/test-support`, which the first CI
run found and a local green could not:
`crates/profile/tests/raw_door_census.rs`'s
`every_crate_that_names_the_door_reaches_it` reds a crate whose `src/`
names `profile::RawLoop` and declares its own `test-support` without
forwarding profile's, and editor-core's `#[cfg(test)]` modules do name
it. `crates/sweep/Cargo.toml` carries the same forward for the same
reason.

**The four rows are unchanged** and green: they live in
`editor-core`'s own test binary, which the self dev-dependency compiles
with the feature on.

**Not landed as specified: the unreachability row.** A `compile_fail`
doctest cannot carry this claim in this workspace — `cargo test --doc
--workspace` selects `editor-core`, which activates its own
dev-dependency, so the feature is unified ON for every doctest in the
run and the row would fail for the wrong reason. The honest instrument
is the `cargo check` of a consumer, and CI already runs two: `cargo
nextest run -p viewer --features app` and the wheel build both compile
`editor-core` with the feature off. Measured locally on this branch: a
line naming `editor_core::MeshPick::build` in `crates/pncad/src`
fails `cargo check -p pncad` with `E0599`. Residue, stated not filed
(it is a property of the repo's test-support convention, not of this
change): a workspace TEST file could still name a mint, because
`--workspace` unifies the feature on.

**Premise correction.** No `pncad` test uses the raw mint — the spec's
"the one `pncad` test that uses it" has no referent on this tree.
`crates/pncad/tests/all.rs` carries only the curation ARGUMENT about
it, which is re-worded here.

**Verified.** Hosted CI run 35183311553 on `8de95d14a`: green, 39 jobs
(36 success, 3 skipped), no failed step, twelve `test (…)` and five
`k-lint (gate, …)`, the python suite's wheel build and unittest step
green. PR #2801.

### Fix pass (2026-09-17, lane `rawtarget-fix`) — the review's findings built

The review's verdict was MERGEABLE; nothing was re-baselined. Per
finding:

- **MINOR-1.** The manifest's stated reason for forwarding
  `profile/test-support` was false: the `profile` dev-dependency below
  it already carries the feature, so this crate's own test targets
  build either way and `cargo check -p editor-core --all-targets` is
  green without the forward. The comment now says the operative reason
  and only it — the convention
  `crates/profile/tests/raw_door_census.rs`'s
  `every_crate_that_names_the_door_reaches_it` holds (a crate whose
  `src/` names profile's raw door and declares its own `test-support`
  forwards profile's), and that census row is what reds without the
  forward.
- **MINOR-2 + S9.** Three sites asserted "exists in no build", an
  absolute `scripts/gates/test-features-dev-only.sh`'s own header
  retracted (a build COMMAND may ask for any feature by name; the gate
  constrains manifests). `crates/pncad/src/select.rs`,
  `crates/pncad/tests/all.rs` and `ASSEMBLY.md` A2a now state the
  operative claim: no consumer's manifest wires the feature onto an
  edge of its own, which the gate holds across every manifest in the
  repository. `PickTarget`'s heading in `resolve/pick.rs` reads "Every
  half a consumer can reach is paired by construction".
- **S1 + S2.** `MeshPick` has one `impl` block again, with
  `#[cfg(any(test, feature = "test-support"))]` on `fn build` itself.
  The docs are written once on the function, there is no `# Errors` on
  an `impl` block and no "see the impl block's own docs" pointer.
  `PickTarget::new` takes the same shape: the `cfg` moved to the
  function.
- **S4.** The ruling stands (neither mint is `#[doc(hidden)]`) and the
  trade is stated at both sites: `scripts/doc-gate.sh` renders this
  crate at `--all-features`, so there the mints sit among the public
  API; a hidden door is one a reader cannot find, and what keeps a
  consumer out is the feature gate rather than the docs.
- **S5.** The sweep was re-run with bare-identifier patterns
  (`PickTarget\b`, `MeshPick\b`) over `crates/` and every hit is
  dispositioned in the PR body, `crates/viewer/tests/review_gui2_r2.rs`
  included.
- **S6.** `crates/viewer/src/pickindex.rs`'s hedge ("whether the façade
  hands a consumer the raw-assembly lane is a separate question") is
  one sentence now, doc-only, announced on VIEW's ground.
- **S7.** `gui1_pick_r2`'s `#[ignore]` reason says what a forced run
  shows rather than what the row used to document.
- **S8.** The workspace-test residue is one line in the manifest
  comment beside the `--all-targets` caveat that states the same
  unification.
- **S3.** Filed, not built:
  `work/issues/test-support-convention-has-no-prose-home` — four
  manifests restate one convention and it has no prose home.
- **S10.** `build_every_table` keeps its name; weighed and kept.

**Verified (fix pass).** Hosted CI run 35191056430 on `2b2b2ab68`:
green — 39 jobs (36 success, 3 skipped), no step with a conclusion
other than success or skipped, twelve `test (…)` and five
`k-lint (gate, …)`, the python suite green through the wheel build and
the unittest step. A fourth site of MINOR-2's class —
`crates/pncad-py/src/py/pick.rs` — was found by the bare-identifier
sweep and re-worded with the other three.

## Closed (2026-09-17, EDIT orchestrator)

Built and merged as PR #2801 (middle tier: one opus style review with
a correctness arm, then the union fix pass). `PickTarget::new` and
`MeshPick::build` exist only under `cfg(any(test, feature =
"test-support"))`, so no consumer build can mint an unpaired target —
`NodePick::target` is the one door a consumer can reach, and the raw
mints live where the four rows that need them do, through the
workspace's self-dev-dependency pattern. Two premises corrected by the
lane: no `pncad` test used the raw mint (the spec's "one test" had no
referent), and a `compile_fail` unreachability row cannot be honest
here — `--workspace` doc-test runs unify the feature on, so the row's
colour would depend on the run's package scope (the reviewer measured
it tier-flaky); CI's two feature-off compiles and the manifest gate
carry the claim instead. The review (0 MAJOR, 2 MINOR, 10 style)
caught the manifest's false compile-consequence claim for the
`profile/test-support` forward (the census convention is the reason)
and three prose absolutes the repo had already retracted once (a build
COMMAND can still enable the feature; no consumer manifest edge does,
and the gate holds that) — a fourth site found by the fix pass's
bare-identifier sweep. One impl block per mint with the cfg on the
fn; the not-hidden ruling kept with its trade stated at the site.
Filed: `work/issues/test-support-convention-has-no-prose-home` (four
manifests restate one convention). Territory crossed by announcement:
`crates/pncad*` (LIB's) and `crates/viewer/src/pickindex.rs` (VIEW's),
prose only.
