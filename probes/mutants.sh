#!/usr/bin/env bash
# Reviewer r1 mutants for PR #3984. Each applies one edit, runs the PR's
# rows + the r1 probes, and reverts. Run from the repo root.
set -u
export CARGO_INCREMENTAL=0
run() {
  cargo nextest run -p topo --lib --no-fail-fast \
    -E 'test(/planar_lane_carrier_rows|each_carrier_kind_lands_on_its_own_lane|offer_rows/)' 2>&1 \
    | grep -E "^\s+FAIL|Summary" | sort -u
}
mutate() { # name file python-replacement
  local name=$1 file=$2 old=$3 new=$4
  python3 - "$file" "$old" "$new" <<'P'
import sys; f,o,n=sys.argv[1:]; s=open(f).read(); assert o in s, o; open(f,'w').write(s.replace(o,n,1))
P
  echo "== $name"; run; git checkout -q -- "$file"
}
R=crates/topo/src/boolean/reduce.rs
C=crates/topo/src/splitting/classify.rs
mutate M1-planar-unlaned-as-line $R \
'                    PlaneCrossingLane::Unlaned => {
                        return Err(BooleanError::CrossingCarrierUnsupported {' \
'                    PlaneCrossingLane::Unlaned => None,
                    #[allow(unreachable_patterns)] PlaneCrossingLane::Unlaned => {
                        return Err(BooleanError::CrossingCarrierUnsupported {'
mutate M2-curved-arm-frontier $R \
'        (geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_), _) => {
            return Err(BooleanError::CrossingCarrierUnsupported {' \
'        (geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_), _) => {
            return Err(frontier());
            #[allow(unreachable_code)] return Err(BooleanError::CrossingCarrierUnsupported {'
mutate M3-lane-spiric-nurbs-as-line $C \
'            return PlaneCrossingLane::Unlaned;
        }
    };' \
'            return PlaneCrossingLane::Line;
        }
    };'
mutate M4-only-nurbs-unlaned $C \
'        geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => {
            return PlaneCrossingLane::Unlaned;' \
'        geom::Curve3::Spiric { .. } => return PlaneCrossingLane::Line,
        geom::Curve3::Nurbs(_) => {
            return PlaneCrossingLane::Unlaned;'
mutate M5-curved-arm-spiric-only-frontier $R \
'        (geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_), _) => {
            return Err(BooleanError::CrossingCarrierUnsupported {' \
'        (geom::Curve3::Spiric { .. }, _) => return Err(frontier()),
        (geom::Curve3::Nurbs(_), _) => {
            return Err(BooleanError::CrossingCarrierUnsupported {'
