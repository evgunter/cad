# base c2c004c41e vs head fb6800c5cd (release, ignored batteries)
## eps 1e-9
diff9_germ_plane_torus_frame::.log: 1 lines; old:  1 ok; new:  1 ok;
diff9_join3_r2_probes::.log: 1 lines; old:  new: 
diff9_join_pierce_runs_sweep::near_tangent_battery.log: 126 lines; old:  126 ERR Escalated; new:  126 OK SOUND;
diff9_join_rc_probes::.log: 180 lines; old:  180 ERR Escalated; new:  180 OK SOUND;
## eps 1e-6
diff6_curved_notch_pinch::.log: 12 lines; old:  12 ERR Escalated; new: 
diff6_germ_plane_torus_frame::.log: 1 lines; old:  1 ok; new:  1 ok;
diff6_join_pierce_runs_sweep::corner_pairs_battery.log: 252 lines; old:  252 ERR Escalated; new:  252 OK SOUND;
diff6_join_pierce_runs_sweep::dbl_battery.log: 12 lines; old:  12 ERR Escalated; new:  12 OK SOUND;
diff6_join_pierce_runs_sweep::four_pairs_battery.log: 54 lines; old:  54 ERR Escalated; new:  4 OK BAD; 50 OK SOUND;
diff6_join_pierce_runs_sweep::pinch_runs_battery.log: 18 lines; old:  18 ERR Escalated; new:  18 OK SOUND;
diff6_join_rc_probes::.log: 4 lines; old:  4 ERR Escalated; new:  4 OK SOUND;
## in-band-only reversal (probe), outcome / matching moves
eps6 curved_notch_pinch::.log outcome_moved=0 matching_moved=0 inband_mutual_rounds=18
eps6 join_pierce_runs_sweep::four_pairs_battery.log outcome_moved=0 matching_moved=0 inband_mutual_rounds=79
eps6 join_rc_probes::.log outcome_moved=12 matching_moved=12 inband_mutual_rounds=4
eps9 join_pierce_runs_sweep::near_tangent_battery.log outcome_moved=0 matching_moved=0 inband_mutual_rounds=198
ALLDONE
rc_probes 1e-6, stdout/stderr split:
outcome: 0
matching: 0
inband: 4 picks_moved: 4
ALLDONE
