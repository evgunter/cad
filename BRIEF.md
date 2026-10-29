# Confirmation brief: SECT PR 6011 fix pass (components by foot)

You are a single confirmation reviewer on a fix pass. Review tier: FULL on
the delta. The unit had a dual review on `b4a0d865` (both APPROVE-WITH-FIXES,
no MAJOR); the fix pass is head `5d84e965a13c004a32a51bc9bedbf1fc95de8aa0`
of https://github.com/evgunter/cad/pull/6011 (branch `sect/components-by-foot`):
`git fetch origin sect/components-by-foot && git checkout --detach 5d84e965a13c004a32a51bc9bedbf1fc95de8aa0`.
Read the PR body (its "Fix pass" section) through the GitHub MCP tools via
ToolSearch. Do not read the PR's comments or reviews. Post nothing, push
nothing.

**Read first:** `docs/prompts/reviewer-style-lane.md`; `CLAUDE.md`; the
C2, C5 and C7 clauses of `crates/geom-brep/README.md`;
`work/sect/a-general-torus-pose-lists-no-components.md` (`## Decided`).
Review `git diff b4a0d865 5d84e965 -- crates` net of the main merges
(`git diff $(git merge-base ...)` as needed; the fix-pass commits are the
lane's own, not the merged mains).

**Builds:** `CARGO_TARGET_DIR` outside the checkout, `CARGO_INCREMENTAL=0`;
foreground; anything past 600 s `setsid`-detached and polled; nothing left
running. Use opt-level 2 (`--profile ci` or `--release`) for the marching
rows. **Budget: about 2 hours.**

## What the fix pass changed that needs falsifying

1. **The site reading widened.** `CarriedComponent::reads` (and so
   `CarriedSection::foot` and the cross-check `holds`) now reads a site
   within the band padded by the loop's uniqueness tube (measured tubes
   8.6e-4..2e-2 at the default ε), where `foot` used the bare band. Can a
   site now be assigned to a component it is not on, or a site that is on no
   component read as on one? Is the argument (limb-3 uniqueness keeps any
   other component out of a loop's tube; the exhaustiveness receipt puts
   every section point in some branch's tube) right as written at
   `CarriedComponent::tube`?
2. **`distinct`.** Each branch's witness must lie on that branch alone.
   Does it catch a doubled loop, and can it refuse two genuinely distinct
   loops (two loops closer than a tube)? Do the landed reviewer rows
   (`a_doubled_loop_among_witnessless_components_is_a_desync`, the reworked
   doubled-ring row via `PairSection::listed`) fail on `b4a0d865` and pass
   here? Is witnessless matching by augmenting paths order-independent?
3. **W3 pads by twice the pad**, argued as: the foot moves at most twice as
   far as the witness because the witness is within the limb-2 bound of a
   surface whose radius is guarded past the band. Check the argument and try
   to break it near high curvature (small tube radius, near-horn torus).
4. **The offer rows now refuse.** `transverse_frame_reach_*` refuse
   `GermSectionUnbuilt` at the withdrawn tolerance (read at real section
   points through the march; the march's tube refuses at the near-node
   tangency). Is that the right reading of those rows, or did the rows lose
   what they were named for?
5. **ε coverage the lane did not run:** after the foot change it did not
   re-run the geom-brep and topo rows at 1e-6 and 1e-12 (except the offer
   rows). Run `pair_section`, `torus_doors_ssi`, `torus_gap`,
   `intersect_table`, topo `section_cert`, `replace_face`, and the sweep rows
   `germ_torus_components`, `cylinder_sphere_frame`, `germ_radial_hole` at
   1e-6 and 1e-12 (opt-level 2), and report failures.
6. **Everything else in the fix pass** (along_facing's check restored,
   `Branch::Component` in frame readers, the served `replace_face` row,
   exact pins, `Band::padded`, constants): a read for correctness.

## Report

Final message, ≤120 lines: verdict (APPROVE · APPROVE-WITH-FIXES ·
NOT-MERGEABLE-AS-IS · REJECT); findings MAJOR/MINOR/NOTE with `file:line`,
evidence, executed or by inspection; `## Style` briefly; tests worth keeping
as code (commit nothing).
