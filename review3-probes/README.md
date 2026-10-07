# PR 4256 review 3 probes (scratch, not for merge)

Head under review: 1526a82c. All probes are `#[ignore]`d.

- `r3_scenes.rs`: shared scene builder (topo test + editor-core module include it).
  Adversarial Y bodies: 4- and 5-face void apexes, near-flat quad voids
  (delta 1e-3 .. 1e-10, both signs), dart voids, depth-3/4 nests, two voids in one
  arch, two voids apart in a buried block, a void with a ray on the arch face or
  in the top; R3_FALLBACK=1 swaps in the fallback witnesses (arch + dart apart,
  dart void + convex island, dart arch + convex void).
  Probes: 4 standard pyramids, scene nests, R3_RANDOM seeded random pyramids.
- `crates/topo/tests/r3_topo_probe.rs`: germ oracle tally + per-cell sorted
  edge_classes dump (R3_OUT). Run:
  `R3_OUT=out.txt R3_RANDOM=6 cargo nextest run -p topo --run-ignored only -E 'test(r3_probe)' --no-capture`
- `crates/editor-core/src/names/emit_topo.rs` `r3_names::r3_names_probe`: name_boolean on every built cell.
- `crates/topo/src/boolean/sectors.rs` `r3_flat_unit`: wedge_classes on a flat
  three-face corner (every bound On): main reads In/Out, head reads None.
- `cmp.py main.txt head.txt`: per-cell main/head diff. `mut.py` + `runmut.sh`: the mutants.
- The *.summary.txt / *.not-named.txt files are the trimmed outputs.
