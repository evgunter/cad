#!/bin/bash
# usage: diff.sh <tree> <tgt> <tag> <eps> [extra nextest args]
tree=$1; tgt=$2; tag=$3; eps=$4; shift 4
SP=/tmp/claude-0/-home-user-cad/3e284829-abc8-5122-95cb-dce386300ca9/scratchpad
out=$SP/log-$tag-$eps; rm -rf $out; mkdir -p $out
cd $tree
CAD_FUZZ_SEED=0x7268 RVW_LOG=$out RVW_SITES=pcurve_torus_villarceau_tilt,pcurve_torus_villarceau_tube_rate,pcurve_chart_orientation,pcurve_spiric_chart_axis,pcurve_focal_section_beta,props_cone_circle_off_axis,loop_windings_joint_room,bool_fate_corner \
CAD_TOLERANCE_EPS=$eps CARGO_TARGET_DIR=$tgt cargo nextest run --profile ci -p geom-brep -p topo --no-fail-fast "$@" > $SP/run-$tag-$eps.txt 2>&1
echo "exit $?" >> $SP/run-$tag-$eps.txt
