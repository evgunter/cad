#!/bin/bash
# usage: mut.sh NAME FILE PYTHON_OLD PYTHON_NEW
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/rev6521-target
name=$1; file=$2; old=$3; new=$4
cp "$file" /tmp/claude-0/-home-user-cad/755744d2-59b8-50ff-ab01-211ecd8008f4/scratchpad/backup
python3 - "$file" "$old" "$new" <<'PY'
import sys
p,o,n=sys.argv[1:]
s=open(p).read()
assert s.count(o)==1, ("count", s.count(o))
open(p,'w').write(s.replace(o,n))
PY
[ $? -ne 0 ] && { echo "$name: PATCH FAILED"; exit 1; }
out=$(cargo nextest run -p geom-brep -p sweep --no-fail-fast -E 'test(a_football_revolved_part_way_is_its_lune) | test(a_horns_cusp_revolved_part_way_refuses_at_its_cap_meridians) | test(limb_2_against_a_sheet_reads_that_sheet_up_to_its_apexes) | test(a_spindle_bound_is_this_sheets_distance) | test(a_one_segment_circle_on_or_against_the_axis_refuses) | test(review_6521) | (binary_id(sweep::all) & test(/^the_football_revolves::/)) | (binary_id(geom-brep::all) & test(/^closed_form_limb2::/))' 2>&1)
cp /tmp/claude-0/-home-user-cad/755744d2-59b8-50ff-ab01-211ecd8008f4/scratchpad/backup "$file"
echo "== $name"; echo "$out" | grep -E "^\s+FAIL|Summary|^error" | sort -u
