#!/bin/bash
# usage: mut.sh NAME FILE 'perl-subst'
cd /home/user/cad
name=$1; file=$2; sub=$3
cp $file /tmp/mut_backup
perl -0pi -e "$sub" $file
if cmp -s $file /tmp/mut_backup; then echo "MUTANT $name: NO CHANGE"; exit 1; fi
export CARGO_INCREMENTAL=0
out=$(timeout 1500 cargo nextest run -p topo -p sweep --no-fail-fast -E 'test(/review_probe_spread|a_touch_|pinches_and|cylinder_pairs_every_class|sphere_pairs_every_class|extent_scan_off_face_tangency|x4|review_3978_r2_probes/) - test(diag_skew)' 2>&1)
cp /tmp/mut_backup $file
echo "MUTANT $name:"; echo "$out" | grep -E "^\s+(FAIL|SIGABRT)|Summary|error\[" | sort -u | head -30
