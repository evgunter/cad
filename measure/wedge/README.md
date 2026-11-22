# Wedge vs departure measurements (`embed/wedge-measure`, not for merge)

Branch cut from `embed/remeter` at 1a640b685 (PR 7329 head).

## Item 1: the sliver through `sign_on`

```sh
export CARGO_TARGET_DIR=/home/user/wm-target CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 \
  CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1
cargo nextest run -p topo --no-capture -E 'test(=injective::wedge_measure_rows::the_sliver_through_sign_on)'
```
