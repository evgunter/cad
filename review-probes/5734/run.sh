#!/bin/bash
cd /home/user/rev5734
export CARGO_TARGET_DIR=/home/user/tgt5734
rm -f /home/user/p5734/DONE
for e in 1e-12 1e-9 1e-6; do
  CAD_TOLERANCE_EPS=$e PROBE5734_LOG=1 PROBE5734_FILE=/home/user/p5734/A_$e.txt cargo nextest run -p sweep -p topo -p step-import --profile ci --no-fail-fast > /home/user/p5734/A_$e.log 2>&1
done
echo A > /home/user/p5734/DONE_A
F='test(a_lofted_lens_blade) | test(a_rim_covering_part_of_its_row) | test(cert5_offgrid) | test(far_thin_disc_sign) | test(m5_s11_concave) | test(m8_3_rational) | test(one_segment_loop) | test(quad_frame_remeasure) | test(split_seam_vanishing_tilt) | test(tcost_k3_certificate) | test(nurbs_import) | test(recognize_pins) | test(a_spiric_cut_far_off)'
for e in 1e-12 1e-9 1e-6; do
  CAD_TOLERANCE_EPS=$e PROBE5734_LOG=1 PROBE5734_FILE=/home/user/p5734/S_$e.txt cargo nextest run -p sweep -p step-import --profile ci --ignore-default-filter --run-ignored all --no-fail-fast -E "$F" > /home/user/p5734/S_$e.log 2>&1
done
echo done > /home/user/p5734/DONE
