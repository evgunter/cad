M16b summary (probe lane). Report `probe/M16b-REPORT.md` (144 lines) + tables `probe/m16b/` on `boxfinish/probe-side-reading`, head c8aafb799d; runs on `boxfinish/probe-switch-off` (arm off, today's main), head b7c925b9c0. Measured unless marked inferred.

VERDICT FIRST (item 6): with the extended reading ON the tour's first drive DOES NOT MOVE. Per-leaf verdicts compared by box, each run to the drive's summary:
- reading off: certified 211 / refused 301 / splits 511, 1,023 leaves, leaf time 1,546 s (sum), wall 404 s, numeric decisions 2,541,916
- on, steps as you specified (trig + Select + hull + depth 32): 211 / 301 / 511, same 1,023 boxes, same verdict on each; 1,635 s; wall 429 s; decisions 2,541,958
- on, plus frozen nodes opened (my addition): 211 / 301 / 511, same boxes, same verdicts; 3,106 s; wall 807 s; decisions 2,562,874
No certified leaf turns, none newly certified, none newly refused. The 1,964 asks the widest reading decides let their leaves ask ~21,000 more decisions and stop at another.

WHY (items 1–4):
1. The 15,200 "no bracket, no atom record" indeterminates are FROZEN RING NODES standing as their own indeterminate, plus hulls: frozen Powi 6,648 (e.g. curve_face_pose_disc), hull 3,596 (period_near_mark), frozen Mul 2,500 (curve_carrier_gap), frozen Add 2,430 (azimuth_near_cut_side). No bracket-less param, opaque, free, literal, or atom recorded elsewhere (0 unmapped). M16's "no frozen node" read the leaf's own `budget_frozen`, which is empty; the freezes come from the drive's table. Entering the atoms only surfaces more of them: at step 15 every remaining decline is one (Mul 14,514; Powi 14,358; Add 2,692).
2/3. Cumulative on the drive's 37,218 undecided asks (sign / straddles / declines; zero-in-band 0, sliver 0, EMPTY 0 at every step):
 - shipped walk: 0 / 812 / 36,406 (19 µs/ask)
 - + Sin/Cos/Tan/Asin/Acos/Atan/Atan2: 0 / 812 / 36,406
 - + Select: 0 / 812 / 36,406
 - + Hull: 0 / 812 / 36,406
 - + depth 32: 42 / 5,612 / 31,564 (46 µs/ask)
 - + frozen nodes opened (op over children's forms): 1,964 / 22,812 / 12,442 (39 ms/ask in this probe; declines: depth 6,164, quotient over a zero-holding denominator 3,336, Atan2 over the cut 2,942)
 - [alt, 1,465 sampled] node lane values from a probe side table at mint: 180 / 919 / 366
 Today's ladder on the same asks: 29.3 ms/ask. On 552,815 ladder-Zero asks with a non-zero plain form the reading agrees every time. No atom kind was refused as unsound to enter.
4. Where it straddles at the widest step with members sampled, the intersection IS the value's own enclosure in every traced representative: the form opened through frozen nodes by interval arithmetic is no tighter than the value channel. curve_carrier_clear (7,776 straddle): 240 sampled, members all Positive 3.87e-3…3.93e-3 against [0, 1.39e-2] → dependency inside the form. curve_face_pose_disc: 264 all Positive 4.8e-4…5.0e-4 against ±1.38e-3 → dependency. side_at_infinity_winding: 230 all Positive 6.55e-3…6.61e-3 against [−4.8e-3, 2.1e-2] → dependency. period_near_mark: 70 evaluable, one-signed ±0.344 against [−1.33, 0.49] → dependency; 170 not evaluable. assert_bound: 174 one-signed, 67 really cross the band. azimuth_near_cut_side, curve_carrier_gap, curve_pair_span_end_side: members not evaluable (hull/free in the graph) → not determined. Inferred: a centred / mean-value evaluation is what these need; none tried.

OTHER POPULATIONS
- Verb-built tilted body: its 8 refusing asks decline at the shipped walk (Cos 6, Sin 2); trig alone decides 6; all 8 with frozen opened. With the reading ON (either step set) ±0.05 BUILDS (6 faces, census 0, battery 0); ±0.1 and ±0.25 pass the join and are refused later, `OrientationUndecided` on the volume ([−0.41, 3.49], [−4.27, 8.04]); ±0.45 still refused (`curve_pair_span_end_chord`; `sector_arm` at the widest).
- Slab rows (172 undecided; on this tree the door finishes at the door): shipped 94 sign / 78 decline (depth); depth 32: 136 / 32 straddle / 4; frozen opened 136 / 36 straddle. Door refuses at 0.488, 0.49, 0.6, 0.9 with the reading off; only at 0.9 on.

5. SOUNDNESS: step 15: 1,808 asks × 5 members, 0 differ; widest step: 3,400 asks, 0 differ; verb-built 120 and slab 136, 0 differ. M16's 17 (`pm_census_ef_residual_side`; M16's rows, not re-run): `k_stats::prove` uses `Band::least()` = (5e-324, 1e-323). Forms vs f64 members: [6.5813e-19] vs +5.55e-17 / −5.55e-17 / −1.67e-16; [1.3163e-18] vs +1.11e-16 / −9.71e-17; [−2.6645e-17] vs +5.55e-17 / 0 / −1.11e-16; [−4.2188e-17] vs 0 / −1.67e-16; [−6.8834e-17] vs ±1.11e-16. 23 asks, 17 with a member of another sign: the form is exact, a member's f64 rounding (1e-16) is larger than it, at a band of 5e-324.

NOT MEASURED: whether the residual folds on the verb-built body over ±0.05 (builds only with the reading on; forms not printed there); a centred/mean-value form; node values beyond the sample; the tour past its first drive; members on the box diagonal only. Standing by.
