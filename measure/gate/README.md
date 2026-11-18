# The at-rest gate's measuring harness (`embed/gate-measure`, not for merge)

`embed/gate-measure` is `embed/gate` (the at-rest gate asking
`topo::clearance::embedded` of each solid, after tier 3, in
`AtRestBody::embedded`) plus instrumentation, plus the cavity lane's
per-cell trace (`embed/cavity-trace-measure`, merged in). Every switch is
an environment variable; with none set the build behaves as `embed/gate`.

## Run one named test with the gate on

```sh
export CARGO_TARGET_DIR=/home/user/<lane>-target   # your own, outside the checkout
export CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
export CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1        # CI's build
export RUSTC_WORKSPACE_WRAPPER=$PWD/scripts/rustc-test-crates-opt0.sh     # test crates at opt-0, as CI
mkdir -p /tmp/gate-log
CAD_GATE_LOG=/tmp/gate-log cargo nextest run -p <crate> --profile ci --ignore-default-filter \
  -E 'test(=<module::test_name>)'
cat /tmp/gate-log/*.log
```

The test fails with the gate's finding in its panic: `BoundaryMeetsItself
{ faces, stations, witness }` or `EmbeddingUncertified { refusal, width }`,
where `refusal` is the engine's `EngineRefusal` verbatim (for
`Uncertified`: the two faces, the shared cell and the arm).

## What the switches give

| variable | what it does |
|---|---|
| `CAD_GATE_LOG=<dir>` | one `GATE` row per ask to `<dir>/<pid>.log`: binary, test, scalar, faces, candidates, splits, discharged, seconds, the verdict (`Debug`), the kinds and solids of the faces it names, the records a Boolean's door held (`HELD`), `REFUSED_FIRST` (a crossing found after a refusal was kept), and the finish site (`SITE`) |
| `CAD_GATE_PASS=1` | log and pass: the gate never refuses, so a whole walk (the tour, `demos/wild`) reads every body |
| `CAD_GATE_OFF=1` | the gate asks nothing |
| `CAD_GATE_DECOMPOSE=<n>` | every face pair of each holding ask with at least `n` leaves asked alone (`DECOMP` rows) |
| `CAD_GATE_CARRY=1` | at the deciding doors, each holding ask's pairs classed carried / trimmed / two operands / new (`CARRY` rows) |
| `CAD_GATE_CENSUS=1` | times the tier-3′ census at each Boolean `gate` (`CENSUS` rows) |
| `CV_TRACE=1` | **the refusing cell and its exit**: each `Unpresented::Local` exit in `injective.rs` prints `CVL <site> <line>` to stderr, and the presentation trace prints the cells it read (from the cavity lane) |
| `CV_PAIRS=1` | the sweep's per-pair log (from the cavity lane) |
| `CV_POLE=1` | the pole/singular-vertex reading (from the cavity lane) |
| `CV_CAVITY_F64=1` | the shell's cavity verdict and receipt (from the cavity lane) |

So the refusing cell and exit come from `CV_TRACE`; the gate log gives
the ask, the faces and the payload. Add `--no-capture` to see the trace.

## The census data

`census-c2a0c41f40.json`: the per-solid census on main `c2a0c41f40`,
every failing workspace test (1,313) with its class (`1-not-embedded?`,
`2-refused`, `3-in-solid`, `4-other`), the refusal shape, the faces'
kinds, the scalar, the face count, the gate's first non-embedded payload
as printed, the faces it names with their solids, the records the door
held, and the test's time. Tables and method: the gate row
(`work/embed/the-at-rest-gate-certifies-embedding.md`, "Census, second
take").
