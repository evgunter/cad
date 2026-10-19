---
id: panic-hook-copies-swap-the-process-hook-around-each-capture
kind: issue
title: Nine test sites swap the process-global panic hook around each capture instead of using test_utils::panic_capture::caught
status: open
opened: 2026-10-19
---


Found by ENCL's param-span-doors fix pass (PR 5365), which deleted its
own copy (`geom_brep::recourse::panics_as_a_kernel_bug` now calls
`test_utils::panic_capture::caught`).

`test_utils::panic_capture::caught` (`crates/test-utils/src/panic_capture.rs:24`)
installs its hook once and switches it per thread, and its docs name
why: taking and restoring the process-global hook around each call
races two concurrent rows (one restores while the other is still inside
`catch_unwind`, so its message reaches the default hook), and a hook
that does not chain to the previous one swallows every other thread's
panic during the window. Each site below takes the hook, sets its own,
and restores it around one capture. The first two serialize on
`surgery::tests::PANIC_HOOK`, so they do not race each other, and
`PanicCapture` chains to the previous hook, so it swallows nothing;
the race and the swallowing hold in full at the other seven:

- `topo::surgery`'s `caught` (`crates/topo/src/surgery.rs:577`), a copy
  of `caught` whose own doc names `review_d18::PanicCapture` as its twin;
- `review_d18::PanicCapture::run` (`crates/topo/src/review_d18.rs:230`,
  the hook at `:209`);
- `outcome` (`crates/mesh/tests/mesh11r1_probes.rs:18`);
- `outcome` (`crates/mesh/tests/mesh11r2_probes.rs:155`);
- `tessellate_outcome` (`crates/mesh/tests/mesh8r1_probes.rs:186`);
- `r2r_the_1571_body_through_tessellate` (`crates/mesh/tests/mesh8r2_probes.rs:382`);
- `scanner_fires_on_planted_violations` (`crates/mesh/tests/profile_overrides.rs:140`);
- `the_halfcap_eps7_witness_is_band_shaped` (`crates/step-import/tests/poleguard.rs:231`);
- `d2_no_input_reaches_a_panic` (`crates/sweep/tests/review_d2_adv_probes.rs:444`).

**A hazard inside one binary.** topo's lib test binary holds
`test_utils::panic_capture::caught` (the tier-3 row,
`crates/topo/src/tier3_tests.rs:1429`) beside `surgery::caught` and
`PanicCapture`, and `PANIC_HOOK` does not serialize the first. Under
in-process `cargo test` threads, a tier-3 panic raised inside a
concurrent `surgery::caught` window reaches `surgery`'s hook, which
does not chain, and the tier-3 row reads an empty message. Worse, if
`caught`'s `INSTALL.call_once` first runs inside either window, it
wraps that window's hook, and the window's restore puts back the hook
from before it: the one-time install is gone for the rest of the
process, and every later `caught` in the binary reads its panic
through the default hook. CI runs nextest, one process per test, so it
stays green; a local `cargo test -p topo --lib` can flake.

The sites that only silence the hook (`set_hook(Box::new(|_| {}))`)
want `caught` and its `None`/`Some`; the two capturing ones want it
too, or `test_utils::own_thread` where the subject panics on a thread
of its own.

**Sweep:** `take_hook` over `crates/**/*.rs` outside `crates/test-utils`
(ten hits in nine files, all above). Blind spot: a capture that
downcasts the `catch_unwind` payload instead of hooking takes no hook;
`scripts/gates/bit-identity-punning.sh` bans that outside
`geom-core/src/bit_identity.rs`, so it cannot hide there.

Note (PR 5393, 2026-10-19): the geom-brep test helper `panics_as_a_kernel_bug` that this row cites is deleted. The topo tier-3 row's `test_utils::panic_capture::caught` use, and the hazard it shares with `surgery::caught`, still stand. (ENCL orchestrator)
