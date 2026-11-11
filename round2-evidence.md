# Round 2: the build of the agreed rule hit two counterexamples

All three of you agreed in your first reports: pair each crossing pair along the In arcs of its lead vertex (the shared vertex, else A's), take the op out of the insertion, and retire the finish's vertex-set check. The orchestrator decided that, and an implementer built it. The build is on branch `gauge/in-arc-pairing` (PR 6832, head `0fd699740c`, not merged). It works on everything you measured: the battery gives 5,190 OK SOUND (main: 4,208 OK, 982 ERR SharedVertexCrossings), with 0 topology differences on the lines that built before, and all six cleave-witness ops build. But two families that build on main now fail, and the implementer stopped there.

## Family 1: nested links at the shared vertex (editor-core)
- Tests: `names::emit_topo::touch_reread_rows::a_corner_held_as_several_result_vertices_is_a_fragment_at_each` and `…::a_tied_corners_candidates_are_named_apart_wherever_they_lie` (crates/editor-core).
- Scene: `x − y` of `over_a_void`. `y` is a pyramid less a pyramid nested inside it at the same apex, so near the apex y's material lies between two cones, and y's two vertices there have nested links. x's apex is shared, so x leads, and its In arc against y's inner vertex (material outside the inner cone) holds the outer pair's cuts.
- The build's new invariant fires: `ClassificationInvariant` "a run at a shared vertex holds another pair's cut: two pieces of one operand overlap at the point". Main builds this pose by turning that run.
- Implementer's reading: the premise "the other operand's pieces at a point are disjoint" is false when one operand's vertices at a point bound nested cones.

## Family 2: a tangent ellipse rim in ∪ (sweep slow set)
- Tests (crates/sweep, `a_closed_conic_in_the_partners_face`): `a_boss_over_a_hole_by_pose`, `a_boss_over_a_hole_tangent_to_it_turned`, `a_crescents_bodies_read_right_and_pinch_at_one_vertex_pair`, `ellipse_rims_tangent_inside_build_every_op`. Main passes all four.
- Each refuses `A ∪ B: ERR Join(BooleanJoin(UnpairedLooseEnds { count: 4 }))`.
- A local mutant with the op-chosen start and no run turning passes all 22 tests in that file. Keeping two-crossing pairs in record order does not help.
- Implementer's reading: the join depends on the pairing for a pair of more than two crossings in ∪, so "the op leaves the insertion" is not free there. The implementer did not establish whether that vertex is shared.

## Other facts from the build
- Before removing them, the build measured the dead arms under the old code over the topo and sweep ci profiles: 372 shared-vertex readings, 0 turns, 0 refusals, 0 changed re-reads, 0 cover hits. (Family 1 is in editor-core and family 2 in the slow set, neither of which that probe covered.)
- The finish's vertex-set check dates from PR5. Every recorded firing was a refusal the pre-split zip also gave (`Euler(SelfLoopEdge)`); no test pinned it.
- Evidence logs in `/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/`: `gauge-build-battery-main.log`, `gauge-build-battery-branch.log`, `gauge-build-slow-default.log`, `gauge-build-ci-app.log`, and `gauge-build-mutants.patch`.

## What this round asks
1. Read both families in the model, on the build branch and on main. What should each op produce, and why does the agreed rule fail there? Is it the rule, the lead choice, the disjointness premise, or something else (for family 2, find out whether the vertex is shared and what pairing the join needs)?
2. Revise your recommendation, or defend it with a fix that keeps it. State the rule for pairing at a vertex where one operand's own vertices bound nested cones, and say what the op must or must not contribute.
3. Whatever you recommend must keep the battery at 5,190 SOUND, the cleave witness building, and both families building as on main (or explain, by meaning, why one of them should refuse). Probe it if you can; say what you measured and what you did not.
4. Report in the same shape as your first report (answer, argument, confidence, framing, ratified text, For Ev, For the orchestrator), and say plainly what moved since round 1.
