#!/bin/bash
cd /home/user/frozen
export CARGO_TARGET_DIR=/home/user/tgt-frozen
[ "$1" != "0" ] && export R2_MUTANT=$1
timeout 1200 cargo nextest run -p sweep -p topo -p editor-core -p step-import --ignore-default-filter --no-fail-fast \
  -E 'test(/session_box|past_the_box|a_copy_whose|tcost_k3|boxed_motion|far_placement|product_finishes_once|cover_reach|a_conical_ended_football|a_flat_frustum|review_m2_pr4|review_m2_pr3|wire_loft_end_profile|long_edges_on_one_line|m9_2b_r2|review_m4_pr2_transform|face_chart_and_signed|intent_e_operation_frame|tier3_tests|geometric_cube|review_m3_pr4|a_moved_body_measures|interval_body|seamless|section_cert_cone_rows|verbs_cylsph_opening|nurbs_faces_refuse|revolve_axis_gaps|a_seamed_slender|approximated/) - test(review_r2)' \
  > /home/user/emut-$1.log 2>&1
echo "== mutant $1"; grep -E "^\s+Summary|^\s+(FAIL|TIMEOUT|SIGABRT)" /home/user/emut-$1.log | sort -u | sed -E 's/\s+FAIL \[[^]]*\] \([^)]*\)/  FAIL/' | cut -c1-200
