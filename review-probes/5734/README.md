Review probes for PR 5734 (frozen head 09a56169b7). Not for merge.

Env-gated instrumentation in quad_lane.rs (trig_at_end, reported) and quad.rs (Run::stop):
- PROBE5734_FILE=<path> + PROBE5734_LOG=1: log every meet where the vertex reading does not contain the
  parameter bracket, and every lane end with its floored decision.
- PROBE5734_ROUNDS=1: log every round's flux width / removable / area mid.
- PROBE5734_EXTRA=N: no floor stop before round N (drift measurement after the floor).
- PROBE5734_NOFLOOR=1: never stop at the floor.
- PROBE5734=vertex|param|trig1vertex|flip|allfloor: mutants.
Key reproduction (1e-12): split_seam_vanishing_tilt::a_split_through_a_seam_vertex_returns_no_side_tier_three_refuses
logs t=-4.4721359910896466e-4: param cos [0.9999998999999997, 0.9999999000000005] holds cos t = 0.99999990000000005267,
vertex [0.9999999000000003, 0.9999999000000005] does not; the meet is the vertex bracket.
