# M-b: every levered scalar decision through the turn-lever doors, at both bounds

## What was computed, and against what

- **Each pair.** A recorder in `geom_core::k_stats::classify`, the one funnel
  every `decide*` door goes through, records `(predicate, outcome)`: `+`, `−`,
  `0`, or `E` for an escalation. Each composite consumer is run three times:
  - at the door's bound from below (today's reading, the one returned);
  - at its bound from above, detached from the escalation log;
  - at the bound from below with each boundary edge's reach added (M-d).
  Both alternate runs read their lever at the same site as today, the span's
  middle. The two decision sequences are paired index by index, up to and
  including the first pair that differs. After it the two runs take different
  paths and their decisions no longer correspond.
- **Consumers wrapped.**
  - In geom-brep: `must_carry_over_edge` (which covers `must_carry_description`,
    `reask_pair`, `reask_chart` and tier 3's re-ask), `edge_wedge_over_as` (WV,
    tier 3's check 4, `topo::edge_wedge`) and `classify_seam`.
  - At topo sites: `classify_shared_rim` (`route_rim`), the singular turn's
    `Period::decide`, `bool_rim_face_opposed` (`stands_on`),
    `bool_touch_sheets_part` (`sheets_part`), `bool_rider_wall_side`
    (`walls_inside`), `edge_join::joinable` and `splitting::rules::edge_wedge`.
  - Certification's own decisions (`run_checks`) are not recorded. It takes the
    bound from below, and the designers' line leaves it there.
- **Coverage.**
  - ε = 1e-9: topo, step-import, sweep and editor-core, all 12,157 tests; 31 hit
    the measurement's 6-minute cap.
  - ε = 1e-12: topo and step-import (4,799 tests, 3 capped); sweep's first 1,354
    of 3,825 (35%, a one-hour budget); editor-core not run.
  - ε = 1e-6: topo and step-import (4,799, 3 capped); sweep in full (3,825,
    6 capped); editor-core not run.
  - Demos: the tour root ran under the instrumentation until it was stopped, and
    7 of its 108 test threads reported (the debug tour at several ε under
    instrumentation runs for hours). The wild root's `cargo test` ran no tests.
    The demos are effectively not covered.
  - Lanes: f64, `Interval`, Dual and Sym, wherever the suites run a consumer.

## Result

About 282 million decision pairs were recorded across the runs.

| Predicate (consumer) | pairs | pairs that differ | of which with the bound from above poison |
|---|---|---|---|
| `dihedral_arm` (WV) | 86,488,639 | 112 | 112 |
| `dihedral_wedge` (WV) | 86,488,478 | 0 | — |
| `dihedral_sign` (WV) | 84,536,823 | 0 | — |
| `pcurve_singular_turn` | 9,488,939 | 82,471 | 82,471 |
| `dihedral_arm` (MC) | 1,928,900 | 36 | 36 |
| `dihedral_wedge` (MC) | 1,928,864 | 0 | — |
| `material_wedge_side`, `tangent_second_order`, `material_cusp_side` (WV) | 1.9 M each | 0 | — |
| `dihedral_arm`, `dihedral_wedge` (seam) | 1.5 M each | 0 | — |
| `tangent_second_order` (MC) | 976,762 | 0 | — |
| `dihedral_arm`, `dihedral_wedge`, `tangent_second_order` (joinable) | 73k, 73k, 3.7k | 0 | — |
| `rules::edge_wedge` (dihedral, `enters_material`) | 10.6k, 9.2k | 0 | — |
| `bool_rider_wall_side` | 9,669 | 0 | — |
| the rim (`dihedral_arm`, `dihedral_wedge`, `tangent_second_order`, `material_*`) | 5,490 each | 0 | — |
| `bool_touch_sheets_part` | 762 | 0 | — |
| `bool_rim_face_opposed` | 384 | 0 | — |
| seam side rows, `material_pairing_*`, `gradient_alignment_lever` | ≤ 116 each | 0 | — |

**(i) Straddles (one bound Zero or in band, the other definite).** There are
none where the bound from above is finite, at any ε or lane run. Every
differing pair has a poison bound from above:
- `(b = +, a = E)` on the arm gate (`dihedral_arm`), where the cover is poison;
- `(b = ±, a = E)` on the turn.
Their sources are M-a's placeholder seeds and unbounded faces mid-construction,
and the site mint.
- **Against the base's verdict:** 0 newly refused where the bound from above
  is finite. All 82,619 differing pairs would be newly refused only because the
  bound from above is poison.
- **Against closed-form truth:** with no finite straddle there is nothing to
  split into a completeness loss or an intended refusal. The poison cases are
  faces with no geometry yet, which no closed form describes.

**(ii) Wrong-direction pairs** (definite below and a different sign or Zero
above; or in band below and Zero above). There are none with finite bounds. So
no recorded margin moved against its lever.

**(iii) The bool sites** (`stands_on`, `sheets_part`). Every pair is equal, so
there is no straddle to split by action.

**Pairing caveat: the poison rows say nothing past the arm gate.** Pairing
stops at the first decision that differs. Where the bound from above is poison,
that decision is the arm gate (`dihedral_arm`, the first decision of WV, MC
and the seam), and the run at the bound from above escalates there. Every
later decision of that composite (`dihedral_wedge`, the bend, the material
reads) is left unpaired. So the zero counts for those predicates cover only
the reads whose bound from above was finite. They say nothing about faces with
no surface (M-a's placeholders and unbounded faces). Pairing past the gate
would need the arm passed from below to the gate and from above to the rest:
a different instrument.

**Rim pairing.** 610 `classify_shared_rim` calls (f64) made 135 decisions at
the bound from below and 45 at the bound from above. The common prefix is
equal and the verdict identical. The most likely cause is a cache the first run
fills: the second run is the detached one. It was not chased.

**Not established.** Whether the lever moved each recorded margin at all. The
folded arm can sit at a face's curvature arm, below both bounds, so an equal
pair may be one the lever never reached. The ratio counts in M-d say how often
the two bounds differ at the door: at 1e-9, 3.46 M of 19.56 M f64 door reads
(17.7%) have above / below > 1.01.
