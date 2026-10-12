#!/bin/bash
# usage: batt.sh <tree> <eps> <outdir>
tree=$1; eps=$2; out=$3; mkdir -p $out
cd $tree
bin=$(ls -t target/release/deps/all-* | grep -v '\.d$' | head -1)
for t in join_pierce_runs_sweep::pierce_runs_battery join_pierce_runs_sweep::pinch_runs_battery join_pierce_runs_sweep::four_pairs_battery join_pierce_runs_sweep::corner_pairs_battery join_pierce_runs_sweep::dbl_battery join_pierce_runs_sweep::near_tangent_battery join_pierce_runs_sweep::eight_crossings_with_a_pinched_cube_battery join1_r1_probes:: join3_r2_probes:: join_rc_probes:: join_reflex_wedge_probes:: join1_delta_probes:: a_flank_touch_at_a_shared_corner:: pinch_faces_tessellate:: germ_plane_torus_frame:: band_apart_partners_on_a_steep_ellipse:: curved_notch_pinch::; do
  s=$(date +%s)
  CAD_TOLERANCE_EPS=$eps timeout 3000 $bin --include-ignored --nocapture --test-threads 4 $t > $out/$t.log 2>&1
  echo "$t rc=$? $(( $(date +%s)-s ))s" >> $out/summary
done
echo ALLDONE >> $out/summary
