export CARGO_TARGET_DIR=/home/user/review-target
export CARGO_INCREMENTAL=1
export CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1
export CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
FILTER="test(/^injective::/) | test(/rest_facts::/) | test(/extent::/) | test(/^self_intersection_proves_injectivity::/) | test(/review_probe/) | test(/the_sector_slivers_meet/)"
