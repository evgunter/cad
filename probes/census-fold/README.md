# census-fold — scratch measurement (REACH), not for merging

Findings: `report.md` at the branch root.

Reproduce (each step from the repo root):

    git apply probes/census-fold/kernel-hook.patch probes/census-fold/tour-hook.patch
    probes/census-fold/run_suites.sh 1e-9 /home/user/cf-out/eps-1e-9   # and 1e-6, 1e-12
    (cd demos/tour && CARGO_TARGET_DIR=/home/user/cf-tour-target cargo build --release)
    probes/census-fold/run_tour.sh
    python3 probes/census-fold/tables.py   /home/user/cf-out/eps-1e-9 /home/user/cf-out/eps-1e-6 /home/user/cf-out/eps-1e-12
    python3 probes/census-fold/classify.py /home/user/cf-out/eps-1e-9 /home/user/cf-out/eps-1e-6 /home/user/cf-out/eps-1e-12
    python3 probes/census-fold/arms.py     /home/user/cf-out/eps-1e-9
    git checkout -- crates demos && rm crates/topo/src/census_fold_probe.rs

Repros: copy `repros/census_fold_repros.rs` into `crates/sweep/tests/`,
add `mod census_fold_repros;` to `crates/sweep/tests/all.rs`, run
`cargo nextest run --release -p sweep --test all census_fold_repro --no-capture`.
