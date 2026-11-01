# R1 probe rows (PR 6266 @ c46bc2d6), per-ε verdict counts
## ε 1e-6
- r1_apex_in_band_of_the_plane:  3 BUILT-UNMESHED ; 432 OK ; 717 REFUSE ;
- r1_apex_pairs_and_curved:  192 REFUSE ;
- r1_grazes_20:  600 OK ; 480 REFUSE ;
- r1_grazes_45:  60 BUILT-UNMESHED ; 660 OK ; 360 REFUSE ;
- r1_grazes_70:  60 BUILT-UNMESHED ; 660 OK ; 360 REFUSE ;
- r1_sweep_15:  261 BUILT-UNMESHED ; 912 OK ; 27 REFUSE ;
- r1_sweep_30:  237 BUILT-UNMESHED ; 780 OK ; 183 REFUSE ;
- r1_sweep_45:  18 BUILT-UNMESHED ; 780 OK ; 402 REFUSE ;
- r1_sweep_60:  720 OK ; 480 REFUSE ;
- r1_sweep_75:  650 OK ; 540 REFUSE ; 10 WRONG ;
- r1_turned_frame:  132 OK ; 12 REFUSE ;
- r1_wedges_general:  396 BUILT ; 120 REFUSE ;
## ε 1e-9
- r1_apex_in_band_of_the_plane:  3 BUILT-UNMESHED ; 432 OK ; 717 REFUSE ;
- r1_apex_pairs_and_curved:  192 REFUSE ;
- r1_apex_touches:  108 REFUSE ;
- r1_cylinder_two_rulings_across:  113 BUILT ; 98 BUILT-UNMESHED ;
- r1_grazes_20:  60 BUILT-UNMESHED ; 780 OK ; 240 REFUSE ;
- r1_grazes_45:  60 BUILT-UNMESHED ; 780 OK ; 240 REFUSE ;
- r1_grazes_70:  72 BUILT-UNMESHED ; 792 OK ; 216 REFUSE ;
- r1_small_box_off_the_apex:  15 BUILT-UNMESHED ; 6 OK ; 51 REFUSE ;
- r1_sweep_15:  261 BUILT-UNMESHED ; 912 OK ; 27 REFUSE ;
- r1_sweep_30:  264 BUILT-UNMESHED ; 780 OK ; 156 REFUSE ;
- r1_sweep_45:  18 BUILT-UNMESHED ; 780 OK ; 402 REFUSE ;
- r1_sweep_60:  720 OK ; 480 REFUSE ;
- r1_sweep_75:  650 OK ; 540 REFUSE ; 10 WRONG ;
- r1_turned_frame:  132 OK ; 12 REFUSE ;
- r1_wedges_general:  396 BUILT ; 120 REFUSE ;
## ε 1e-12
- r1_apex_in_band_of_the_plane:  3 BUILT-UNMESHED ; 432 OK ; 717 REFUSE ;
- r1_apex_pairs_and_curved:  192 REFUSE ;
- r1_grazes_20:  720 OK ; 360 REFUSE ;
- r1_grazes_45:  720 OK ; 360 REFUSE ;
- r1_grazes_70:  720 OK ; 360 REFUSE ;
- r1_sweep_15:  261 BUILT-UNMESHED ; 912 OK ; 27 REFUSE ;
- r1_sweep_30:  264 BUILT-UNMESHED ; 780 OK ; 156 REFUSE ;
- r1_sweep_45:  18 BUILT-UNMESHED ; 780 OK ; 402 REFUSE ;
- r1_sweep_60:  720 OK ; 480 REFUSE ;
- r1_sweep_75:  650 OK ; 540 REFUSE ; 10 WRONG ;
- r1_turned_frame:  132 OK ; 12 REFUSE ;
- r1_wedges_general:  396 BUILT ; 120 REFUSE ;

## Reading the counts

- `OK`: mesh volume within chordal sag of the slice-integrated closed form, and 0 lattice misses by winding number.
- `BUILT-UNMESHED`: the mesher refused (conic trims on a plane, folded triangles); the kernel volume (`pinned_volume`) was checked against the closed form instead. 0 of 1929 differ.
- `BUILT` (wedges, cylinder): the kernel volume against the closed form, rows checked by `r1_wedge.py`.
- The 10 `WRONG` rows per ε at α 75°, θ 177° are the oracle's own slice-integration residual (3e-6 on a truly empty `c ∖ b`). The 9 wedge rows that `r1_wedge.py` flags at n=(90,1.1) are its grid's error: the exact azimuth gives the kernel's value, e.g. (π/2 − 1.1)/2 = 0.235398.
- No PANIC at any ε. No wrong body.
- `rows.tgz`: every raw row, plus main's and mutant M3's rows for the touch, small-box and curved poses.
