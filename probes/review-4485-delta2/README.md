Delta-2 review probe for PR 4485 (head 2ac0994f8). Copy `zz_delta2_diff.rs`
into `crates/geom-brep/tests/`, add a `[[test]] name = "zz_delta2"` entry, and
run with `PROBE_OUT=<dir> NFACE=200 cargo test --release -p geom-brep --test zz_delta2 -- --nocapture`.
On main (489f9fa22) swap the `green` adapter for `main_adapter.rs`.
Results: patch.txt bit-identical on main / c5d1c7aed / head (400 surfaces, 200 faces);
ladder.txt identical c5 vs head; main excludes the truth on 1288/6000 constant-u cases, head on 0.
