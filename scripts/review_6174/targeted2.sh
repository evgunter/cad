#!/bin/bash
FS='test(/^(an_aimed_ray_from_far_reads_a_split_side_right|review_6174_census_on_base)::/) | test(/^a_ray_meets_a_curved_edge_once_for_both_faces::/) | test(/^split_seam_vanishing_tilt::/) | test(/^trimmed_sphere_aims::/) | test(=sphere_aims_reader_ties::sphere_aims_reader_ties) | test(/a_rigid_turn_keeps_a_slivers_volume/) | test(/^curved_edge_corpus::/)'
FE='test(/interval|sym/)'
run() { tree=$1; eps=$2; what=$3; shift 3
  dir=/home/user/cad; [ $tree = base ] && dir=/home/user/base; cd $dir
  CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=3 CARGO_TARGET_DIR=/home/user/tgt-$tree CAD_TOLERANCE_EPS=$eps timeout 3000 \
    cargo nextest run --profile ci --ignore-default-filter --no-fail-fast "$@" > /home/user/t-$tree-$eps-$what.log 2>&1
  echo "$tree $eps $what $?" >> /home/user/targeted.done; }
for eps in 1e-9 1e-6 1e-12; do
  for tree in head base; do
    [ "$tree$eps" = "head1e-9" ] || run $tree $eps topo -p topo -p geom --lib
    [ "$tree$eps" = "head1e-9" ] || run $tree $eps sweep -p sweep --test all --run-ignored all --success-output immediate -E "$FS"
    run $tree $eps ec -p editor-core --test all -E "$FE"
    run $tree $eps ties -p editor-core --test all --run-ignored only --success-output immediate -E 'test(=revalue_corpus::reader_ties_at_the_nominal)'
  done
done
echo ALLDONE >> /home/user/targeted.done
