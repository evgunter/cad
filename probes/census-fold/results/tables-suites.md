
## eps = 1e-09  (5424 distinct f64 bodies passing tier 3)

| stratum | bodies | pass 3' (empty) | fail | of which gated/minted WITH contacts elsewhere | UndeclaredContact | CensusUndecidable | InstanceInterference | CensusEscalated | CensusUnsupported | CensusLaneUnsupported |
|---|---|---|---|---|---|---|---|---|---|---|
| op/other, 1 solid | 2431 | 2412 | 19 | 12 | 19 | 0 | 0 | 1 | 1 | 0 |
| op/other, multi-solid | 141 | 94 | 47 | 7 | 15 | 39 | 4 | 0 | 0 | 0 |
| contact/assembly-subject fixtures | 2852 | 259 | 2593 | 83 | 2570 | 656 | 290 | 10 | 0 | 0 |

Timing (census alone / tier 3; 3' whole / tier 3), median of 3 runs per gate:

| stratum | n | census/t3 median | p90 | max | 3'/t3 median | p90 | max | t3 median s | census median s | census max s |
|---|---|---|---|---|---|---|---|---|---|---|
| all | 5424 | 0.65 | 8.24 | 75.0 | 2.23 | 9.52 | 38.5 | 1.3e-04 | 1.1e-04 | 1.9e-02 |
| 1 solid, planar | 670 | 0.28 | 0.58 | 1.3 | 1.47 | 2.08 | 28.8 | 9.1e-05 | 3.0e-05 | 5.5e-04 |
| 1 solid, curved | 1905 | 0.07 | 0.22 | 0.9 | 1.16 | 1.54 | 15.4 | 1.3e-04 | 1.0e-05 | 9.2e-04 |
| multi-solid | 2848 | 5.53 | 9.61 | 75.0 | 6.84 | 11.06 | 38.5 | 1.3e-04 | 7.3e-04 | 1.9e-02 |
| non-fixture | 2572 | 0.10 | 0.43 | 17.1 | 1.23 | 1.87 | 28.8 | 1.2e-04 | 1.4e-05 | 8.2e-03 |

## eps = 1e-06  (5400 distinct f64 bodies passing tier 3)

| stratum | bodies | pass 3' (empty) | fail | of which gated/minted WITH contacts elsewhere | UndeclaredContact | CensusUndecidable | InstanceInterference | CensusEscalated | CensusUnsupported | CensusLaneUnsupported |
|---|---|---|---|---|---|---|---|---|---|---|
| op/other, 1 solid | 2389 | 2366 | 23 | 12 | 19 | 0 | 0 | 5 | 1 | 0 |
| op/other, multi-solid | 142 | 94 | 48 | 7 | 15 | 39 | 5 | 0 | 0 | 0 |
| contact/assembly-subject fixtures | 2869 | 265 | 2604 | 86 | 2580 | 662 | 292 | 16 | 0 | 0 |

Timing (census alone / tier 3; 3' whole / tier 3), median of 3 runs per gate:

| stratum | n | census/t3 median | p90 | max | 3'/t3 median | p90 | max | t3 median s | census median s | census max s |
|---|---|---|---|---|---|---|---|---|---|---|
| all | 5400 | 0.79 | 8.50 | 38.4 | 2.31 | 9.72 | 127.9 | 1.3e-04 | 1.6e-04 | 2.1e-02 |
| 1 solid, planar | 649 | 0.29 | 0.62 | 1.5 | 1.51 | 2.12 | 5.7 | 9.0e-05 | 3.0e-05 | 6.7e-04 |
| 1 solid, curved | 1887 | 0.07 | 0.21 | 1.0 | 1.15 | 1.50 | 127.9 | 1.2e-04 | 9.8e-06 | 9.6e-04 |
| multi-solid | 2863 | 5.53 | 10.35 | 38.4 | 6.86 | 11.52 | 58.4 | 1.3e-04 | 7.0e-04 | 2.1e-02 |
| non-fixture | 2531 | 0.10 | 0.46 | 17.3 | 1.22 | 1.88 | 127.9 | 1.1e-04 | 1.4e-05 | 7.3e-03 |

## eps = 1e-12  (5417 distinct f64 bodies passing tier 3)

| stratum | bodies | pass 3' (empty) | fail | of which gated/minted WITH contacts elsewhere | UndeclaredContact | CensusUndecidable | InstanceInterference | CensusEscalated | CensusUnsupported | CensusLaneUnsupported |
|---|---|---|---|---|---|---|---|---|---|---|
| op/other, 1 solid | 2422 | 2403 | 19 | 12 | 19 | 0 | 0 | 1 | 1 | 0 |
| op/other, multi-solid | 144 | 96 | 48 | 7 | 15 | 39 | 5 | 0 | 0 | 0 |
| contact/assembly-subject fixtures | 2851 | 262 | 2589 | 83 | 2570 | 654 | 290 | 5 | 0 | 0 |

Timing (census alone / tier 3; 3' whole / tier 3), median of 3 runs per gate:

| stratum | n | census/t3 median | p90 | max | 3'/t3 median | p90 | max | t3 median s | census median s | census max s |
|---|---|---|---|---|---|---|---|---|---|---|
| all | 5417 | 0.69 | 8.41 | 29.1 | 2.27 | 9.81 | 42.4 | 1.3e-04 | 1.2e-04 | 1.9e-02 |
| 1 solid, planar | 663 | 0.28 | 0.59 | 1.4 | 1.50 | 2.05 | 13.2 | 9.1e-05 | 3.1e-05 | 7.4e-04 |
| 1 solid, curved | 1903 | 0.07 | 0.22 | 0.9 | 1.12 | 1.52 | 42.4 | 1.2e-04 | 9.7e-06 | 3.7e-04 |
| multi-solid | 2850 | 5.57 | 10.45 | 29.1 | 6.78 | 11.52 | 23.6 | 1.3e-04 | 6.9e-04 | 1.9e-02 |
| non-fixture | 2566 | 0.11 | 0.46 | 19.5 | 1.19 | 1.89 | 42.4 | 1.1e-04 | 1.3e-05 | 8.9e-03 |

## Verdict changes across eps (same fingerprint)

bodies present at every eps: 5306
pass/fail flips: 5; variant-set changes: 8
- 1fc09931299cb110 s=2 Plane :: 1e-12: {'InstanceInterference': 2}; 1e-09: ok; 1e-06: {'InstanceInterference': 2} :: all-4a2343dc379c82eb::bool4r1_probes::probe_a_both_orderings_decide_and_neither_clears_on_the_first_vertex
- 42ec8d7f54348362 s=1 Plane :: 1e-12: ok; 1e-09: ok; 1e-06: {'CensusEscalated': 130} :: all-ccde0d28663ad546::sf2a_r1::r1e_conditioning_verdict_moves_with_the_offset_alone
- 614e9e7ef8829b37 s=3 Plane :: 1e-12: {'CensusEscalated': 20, 'CensusUndecidable': 2, 'UndeclaredContact': 4}; 1e-09: {'CensusEscalated': 20, 'CensusUndecidable': 2, 'UndeclaredContact': 4}; 1e-06: {'UndeclaredContact': 8} :: all-4a2343dc379c82eb::contact5_gate_and_beam::a_beam_across_two_supports_clears
- a13dc09490cd2d67 s=2 Plane :: 1e-12: {'UndeclaredContact': 8}; 1e-09: {'CensusEscalated': 16, 'CensusUndecidable': 1}; 1e-06: {'UndeclaredContact': 8} :: all-4a2343dc379c82eb::bool4_material_containment::a_witness_at_the_band_edge_refuses_typed_at_this_eps
- bb56f86c84a6e9dc s=2 Plane :: 1e-12: {'InstanceInterference': 1, 'UndeclaredContact': 10}; 1e-09: {'InstanceInterference': 1, 'UndeclaredContact': 10}; 1e-06: {'CensusEscalated': 68} :: all-909d09c92d6bf586::p2_gauge_poses_and_doors::a_declaring_mate_across_gauges_never_certifies_a_real_gap_or_overlap
- c5fd13d63509ef9c s=1 Plane :: 1e-12: ok; 1e-09: ok; 1e-06: {'CensusEscalated': 114} :: all-ccde0d28663ad546::sf2a_r1::r1e_conditioning_verdict_moves_with_the_offset_alone
- c61a6e78de3a3780 s=2 Plane :: 1e-12: ok; 1e-09: ok; 1e-06: {'CensusUndecidable': 1, 'UndeclaredContact': 8} :: all-909d09c92d6bf586::p2_gauge_poses_and_doors::a_declaring_mate_across_gauges_never_certifies_a_real_gap_or_overlap
- c8c8356f649f6e7d s=1 Cylinder+Plane+Sphere :: 1e-12: ok; 1e-09: ok; 1e-06: {'CensusEscalated': 8} :: all-ccde0d28663ad546::blend4_r1_probes::p2_slim_skews_carve_valid_or_refuse_typed_never_worse
