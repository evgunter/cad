#!/bin/bash
# suite.sh <tree> <eps> <pkgargs> <name> [DMUT]
tree=$1; eps=$2; pk=$3; name=$4; mut=$5
cd /home/user/wt-$tree
env ${mut:+DMUT=$mut} CAD_TOLERANCE_EPS=$eps CARGO_TARGET_DIR=/home/user/t-$tree timeout 7000 cargo nextest run $pk --no-fail-fast --failure-output=final --status-level=fail > /home/user/d2/logs/suite-$name-$tree-$eps${mut:+-$mut}.log 2>&1
echo "exit $?" >> /home/user/d2/logs/suite-$name-$tree-$eps${mut:+-$mut}.log
