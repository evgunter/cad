#!/bin/bash
t=base; cd /home/user/cad-base; T=/home/user/tgt-base
CARGO_TARGET_DIR=$T cargo nextest run --profile ci --ignore-default-filter -p geom-brep -p topo -E 'test(/recourse_roster/)' --status-level fail --final-status-level fail --no-fail-fast > /home/user/runs/c12/roster-$t.log 2>&1; echo "exit $?" >> /home/user/runs/c12/roster-$t.log
CAD_FUZZ_SEED=0x53500b4c590de496 CAD_TOLERANCE_EPS=1e-12 CARGO_TARGET_DIR=$T cargo nextest run --profile ci --ignore-default-filter -p geom-brep -E 'test(=curve_face::ellipse_torus::torus_rows::certified_torus_answers_hold_against_the_true_distance)' --status-level fail --final-status-level fail --no-fail-fast > /home/user/runs/c12/torus-$t.log 2>&1; echo "exit $?" >> /home/user/runs/c12/torus-$t.log
CAD_FUZZ_SEED=0x11467e023845fce7 CAD_TOLERANCE_EPS=1e-12 CARGO_TARGET_DIR=$T cargo nextest run --profile ci --ignore-default-filter -p geom-brep -E 'test(/window::oracle/)' --status-level fail --final-status-level fail --no-fail-fast > /home/user/runs/c12/window-seed-$t.log 2>&1; echo "exit $?" >> /home/user/runs/c12/window-seed-$t.log
echo done > /home/user/runs/c12b.done
