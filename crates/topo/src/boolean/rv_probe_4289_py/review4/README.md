# PR 4289 review 4 probes (scratch, not for merge)

Head 20f6bb26. Binary: `cargo test -p topo --lib --no-run`, then
`RV_CASES=<cases> RV_OUT=<out> RV_EPS=<eps> <bin> --exact boolean::sectors::rv_probe_4289::rv_probe_cases`.

- `oracle_a.py`: exact oracle in the reader's model (plane = the sector's
  normal, held; sector between the bounds' far points), plus band moves
  (probe far point toward each face; a bound far point turned in each
  adjacent plane; joint). NB a bound turned in one plane leaves its
  neighbour's: that cone is cracked, so bound-move "flips" are artefacts
  unless confirmed another way. Only D-moves and `least_flip` count.
- `adv.py gen|check`: directed probes about every bound (in plane past /
  before it at 1.5..200 eps of single-point margin, a hair off the edge,
  over the face), stars with chords 1e-3..10, near-180 faces, graze, fins.
  `inband.py`: decided readings the PR's own `least_flip` calls in band.
- `dart_long.py` + `dl_check.py`: near-flat quad, one edge reflex by g
  (flat within eps at its 1 mm bounds), probed by 50 m / 1 km edges.
  wedge_classes' convex branch reads 133 classes exactly contradicted
  (678..233000 eps from any boundary); identical on main 8d11c126.
- `fac.py`: a 179.997 deg sector probed along its bound, both sector
  orders: Err:bool_cone_facing in one, On in the other.
- `graze.py` (review 2's) + `errs.py`: refusal accounting at 1e-6.
- `mut/mut.py`, `mut/run_mut.sh`, `mut/after.sh`: mutants; results in
  `mut/results/summary.txt`.
