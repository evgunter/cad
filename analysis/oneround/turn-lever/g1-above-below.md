# G1: the reads where the bound from above fell below the truth

## What was read

G1's build was rerun at the line site with each such read logged, at ε = 1e-9,
over topo, step-import, sweep and editor-core in full (12,157 tests). Each
detail line records:
- the door;
- the body's class;
- the bound from above and the closed-form truth;
- every arc's largest distance off its face's plane, over 17 points of the arc;
- each cover's distance from the line.

## Result

The rerun counts the same 80 reads as G1. They all fall in 16 tests: a
test's first five cases are sampled, so every test with a case appears.

The 46 sampled are all at **`euler::certify_rebased_run`**. Each has an arc
standing **1.0 to 2.7 m off the plane of the face it bounds**: the rebased
run reads the edge before the move lands, so the arc is not on its face. The
34 unsampled cases are in the same 16 tests and were not attributed one by one.

The truth's closed form measures the in-plane distance from the line, so it
holds only for an arc in its face's plane. Off the plane it reads a distance
the face does not have. The cover holds the face as it stands, which does not
include that arc. **So none of these is a cover that is not a cover. They are
G1's truth applied outside its premise.** Nothing to file.

Bodies (one line per test):
- `a_keel_on_the_floor_subtracts_as_a_slit::*`: ten tests, the arc 1.4 to 2.7 m off.
- `intent_e_operation_frame_of_reads::every_corpus_document_reads_alike_in_a_second_computing_frame`: 1.0 m.
- `a_flank_touch_at_a_shared_corner::a_flank_touch_glues_the_half_turn_it_leaves`: 1.45 m.
- `a_ring_crossed_along_its_line_places_its_germs_apart::a_box_at_the_rings_tip_builds_in_every_op`: 1.44 m.
- `a_vertex_through_a_declared_seat::a_tool_of_three_stacked_solids_is_one_material`: 1.44 m.

Typical case: bound from above 1.624, truth as computed 1.652, arc 2.66 m off
the plane. The 1e-12 and 1e-6 rows (78 each) were not rerun. They are the same
door's reads.
