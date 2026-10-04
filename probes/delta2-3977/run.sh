#!/bin/bash
# run.sh <tree head|main> <eps> <filter> <logname>
tree=$1; eps=$2; filt=$3; name=$4
cd /home/user/wt-$tree
CAD_TOLERANCE_EPS=$eps CARGO_TARGET_DIR=/home/user/t-$tree timeout 7000 cargo nextest run -p sweep --test all --no-capture --no-fail-fast -E "test($filt)" > /home/user/d2/logs/$name-$tree-$eps.log 2>&1
echo "exit $?" >> /home/user/d2/logs/$name-$tree-$eps.log
