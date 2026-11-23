# Wedge vs departure measurements (`embed/wedge-measure`, not for merge)

Branch cut from `embed/remeter` at 1a640b685 (PR 7329 head).

## Item 1: the sliver through `sign_on`

```sh
export CARGO_TARGET_DIR=/home/user/wm-target CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 \
  CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1
cargo nextest run -p topo --no-capture -E 'test(=injective::wedge_measure_rows::the_sliver_through_sign_on)'
```

## Item 2: the sliver and the plain lens as built bodies

```sh
cargo nextest run -p sweep --features test-support --no-capture --test wedge_measure
```
(`crates/sweep/Cargo.toml` gains a `wedge_measure` test target on this branch.) Rows: `wm2-rows.txt`.

## Items 3-5: the census population and the tour walk

Instrumentation: `crates/topo/src/wedge_measure.rs` and the `departure`
wrapper in `crates/topo/src/injective.rs`, env-switched, default off.
`CAD_WM_LOG=<dir>` logs; `CAD_WM_MUTANT=1` makes the walk take the
mutant's outcome (every near edge a check-4 `Crease`: each levered
departure decided by the sine's sign alone). Rows: `WME` per edge per
ask (class, reach, fold), `WMD` per cell pair that flips, is refused for
good at a crease, or changes verdict between reach and fold, `WMF` each
flipped face pair asked alone in both modes, `WMV` per ask with counts.

```sh
# census: the 1,313 tests of measure/gate/census-c2a0c41f40.json (expr files built from it)
RUNS="P M" ./measure/wedge/run.sh       # paths inside point at /home/user/wm-work
python3 measure/wedge/summ.py P M ; python3 measure/wedge/deep.py
# tour (gate passes so the walk reads every body)
./measure/wedge/tour.sh ; python3 measure/wedge/summ.py tP tM
```
Files: `summ-P0.txt` (all 1,301 tests that ran, PR 7329, unsigned gap),
`summ-P.txt` / `summ-M.txt` (the 580 tests with rows, both modes),
`deep-M.txt`, `item5.txt` (from P0), `summ-tour.txt`, `tour-walk-*.txt`.
