---
id: reconcile-solves-with-no-resolver
kind: issue
title: The edit door's cluster-record maintenance solves the prior document with no resolver, and the lever now needs the parts
status: open
opened: 2026-09-07
needs_ev: true
---



Found by the MSOLVE-6 lane (PR 2116, draft) as the spec's stop
clause (iii): a caller of `solve_document` with no resolver in hand
that cannot be given one without a design change. Filed by the
orchestrator from the lane's finding; the `[ev]` PR carries the fork.

## What the tree says

`crates/editor-core/src/mate/solve.rs::reconcile` is the keying
maintenance the edit door runs after any edit that can move the mate
graph (`crates/editor-core/src/edit.rs` `apply`, the `maintenance`
branch): it SOLVES THE PRIOR DOCUMENT (`solve_document(before, tol)`)
to read each surviving cluster's new gauge's relative pose under the
old mate graph, and re-mints the cluster frame from it so the gauge's
world pose is preserved bit for bit across a split or a gauge
rewrite. `apply(doc, edit, tol)` and `Doc::replay` are pure over the
document: no resolver, no part store.

After MSOLVE-6 (Ev's ruling B on PR 2086) the solve's lever is the
mated parts' own extent, read from each part's evaluated body through
a reach the evaluation implements over its `PartCache`. The edit door
has no body to read, so `reconcile` can only solve through a reach
that refuses (`NoResolver`): every mate on a part faults
`Unleverable`, every relative pose reads as the identity, and a split
or gauge rewrite keeps the prior frame instead of composing the solved
pose.

MEASURED on the lane's branch:
`asm_r2a_mate_solve::row4b_a_mate_delete_splits_and_re_mints_from_the_solved_pose`
and `row4c_deleting_the_gauge_rewrites_the_key_and_holds_world_poses`
go red — the re-minted frame lands at z = 4 where the solved pose put
it at z = 5. Every other mate row is green.

Pre-existing and related: `reconcile` reads a refused solve as the
identity silently (`before_poses.relative(gauge).unwrap_or_default()`),
so a prior document whose maintenance solve faults records a frame
nothing decided.

## The fork (Ev's)

The options are in the `[ev]` PR. In short: (a) the edit door and
replay take a reach, so assembly edits need the parts in hand and a
refused maintenance solve refuses the edit typed; (b) the maintenance
leaves the edit door for the doors that hold a resolver; (c) a
verdict-free solve shape for the maintenance; (h) the part's extent
is STAMPED beside its content pin when the reference is pinned, as
recipe data the solve reads like the pin itself.
