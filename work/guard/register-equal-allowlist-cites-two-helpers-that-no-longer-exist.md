---
id: register-equal-allowlist-cites-two-helpers-that-no-longer-exist
kind: issue
title: guard: register-equal-allowlist.sh says registrations reach the arm table through sweep::swept::handle_registration and profile::handle_registration; neither exists, the one arm table is SymRegistration::handle
status: open
opened: 2026-10-20
---


Found by designer B on PR #5449, outside that PR's question.

`scripts/gates/register-equal-allowlist.sh`'s header says "The sweep's
sites reach that arm table through one helper,
`sweep::swept::handle_registration`, and the profile lowering's
through `profile::handle_registration`; each match is exhaustive by
hand". Neither function exists: `grep -rn handle_registration crates/`
finds nothing. The one arm table is `SymRegistration::handle`
(`crates/geom-core/src/real.rs`), exhaustive by hand, which every
registrant calls (`profile::BuiltArc::register` among them). The
sentence should name that, and the gate could check that a helper its
prose names still exists.
