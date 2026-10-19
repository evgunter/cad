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
and restores it around one capture, with that flaw:

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

The sites that only silence the hook (`set_hook(Box::new(|_| {}))`)
want `caught` and its `None`/`Some`; the two capturing ones want it
too, or `test_utils::own_thread` where the subject panics on a thread
of its own.

**Sweep:** `take_hook` over `crates/**/*.rs` outside `crates/test-utils`
(ten hits in nine files, all above). Blind spot: a capture that
downcasts the `catch_unwind` payload instead of hooking takes no hook;
`scripts/gates/bit-identity-punning.sh` bans that outside
`geom-core/src/bit_identity.rs`, so it cannot hide there.
