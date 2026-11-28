# Review probes: boxfinish/wall-roots at 9b843e9910

Results of the probes in this directory and in the `review_wall_roots_probes`
and `review_door_probe` test modules (geom-brep), run 2026-11-28.

| probe | result |
|---|---|
| `quadratic_roots` at `Interval`, 4 seeds, 104,512 boxes kept (43,445 with `b` straddling), 24 members each, exact oracle | 0 member roots outside; 0 unbounded (base spelling: 35,225 unbounded); 0 divisors reaching zero; never wider than a bounded base reading; worst width / members' spread 58x (b touching zero; `disc` is an independent input) |
| `quadratic_roots` at `f64`, 4,088,434 cases incl. ±0, NaN, ±inf, subnormals | bit-identical to the base |
| `wall_quadratic` About::Start (20,000), `sphere_quadratic` About::Start (50,000), cone shift and lead (50,000) | bit-identical to the base |
| wall about the nearest point at `f64`, 20,000, exact oracle | roots inside their own slack: all; root error new vs old same order per family (held-back families: new >4x better 1,690, >4x worse 303); c in units of u·far·near: worst 16.0 both |
| cone B, C at `f64`, 50,000, exact oracle, units of u·far(·|w|) | B worst 5.5 both; C worst new 12.0, old 10.3 (charge: 16); median new about 2x old |
| wall nearest at `Interval`, 5,000 boxes, 9 members each, exact oracle | 0 members outside b, c or the exit root |
| held c's width against the base's over the same 5,000 boxes | wider in 1,825 (worst 5.6x), all where the run holds the approach back; e.g. members' c spans 4.4, base 8.8, held 37.6 |
| the door over `nearest_rows::boxed` at u in [-w, w] | head bounded at every w; base (both spellings) bounded to w = 0.5, (-inf, inf) at w = 0.7, in-band refusal at w = 1 |
| mutants 1-4 and the base's two spellings, 52 rows | as the PR says; the door row is green with both base spellings |
| differential, geom-brep (default, 1e-6, 1e-12) and topo (default), slow set included | same reds but seeded rows (seeds replayed red on both trees); `aims_beside_a_boxed_seams_vertices…` moves exactly as the PR says |

Tour first drive (probe-switch-off + merge base; + f47707e219; + 9b843e9910), release,
4 cores, one run each, `[m11-disc]` tally at `Sym<Interval>` (the "before" run includes
about 27 s of the next cell):

| predicate: undecided of decisions | before | b held | b and c held |
|---|---|---|---|
| curve_carrier_clear | 26,072 of 166,664 | 4,468 of 151,320 | 7,608 of 165,614 |
| curve_carrier_gap | 9,318 of 17,562 | 1,990 of 2,578 | 3,732 of 4,012 |
| curve_face_pose_disc | 1,548 of 16,460 | 1,512 of 16,388 | 216 of 16,378 |
| side_at_infinity_winding | 11,231 (167 s) | 11,537 (291 s) | 12,180 (447 s) |
| verdicts | 211 / 301 / 511 | the same | the same |
| symbolic / all decisions; frozen | 1,832,193 / 4,374,109; 1,490 | 1,832,193 / 4,292,944; 1,514 | 1,832,193 / 4,331,525; 1,688 |
| drive wall | 372 s | 283 s | 337 s |

Wall reads where the clamp may bind over part of the box: 1,074 of 16,378 (after),
over the whole box: 0.
