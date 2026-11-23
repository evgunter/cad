The suspects lane's harness (`oneround/conic-suspects-probe` at `236dcf122`,
`analysis/oneround/conic-suspects/`), copied: `suspects_t1.rs` is its
`src/suspects_t1.rs` with the decision log cut to its last 24 entries
(the full log runs to gigabytes at 1e-12), `oracle_t1.py` and
`table_t1.py` its oracle and table. Run by dropping `suspects_t1.rs`
into `crates/geom-brep/examples/`:
`CAD_TOLERANCE_EPS=<ε> cargo run -p geom-brep --example suspects_t1 > t1-<ε>.jsonl`,
then `oracle_t1.py t1-<ε>.jsonl` and `table_t1.py t1-*.judged.json`.
`base-table.txt` is this unit's base (PR 7510's head merged with main),
`head-table.txt` this branch; a cell is r right, R refused, W wrong,
G definite in the band's gap, P a meeting reported past Kε.
