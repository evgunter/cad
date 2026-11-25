---
id: viewer-app-feature-tests-are-timed-by-no-fast-set-check
kind: issue
title: The viewer's app-feature tests are timed by no fast-set check
status: open
opened: 2026-11-25
priority: P2
cost: E
---


The 0.25 s fast-set bar (`docs/prompts/implementer-discipline.md` §2)
is enforced by `scripts/check-fast-set-times.py`, which
`.github/workflows/ci.yml` runs in the `test` job's default-eps step. That
job builds `viewer` without the `app` feature, so no `app::` test is in
the timed pass. The `viewer` job ("viewer app feature (eframe + wgpu,
wasm32)") runs those tests (`cargo nextest run --profile ci -p viewer
--features app`) and calls `check-fast-set-times.py` only with
`--new-since`, to choose the slow set; it never times a row. So every
test under `crates/viewer/src/app.rs` and the other `#[cfg(feature =
"app")]` modules can sit past both the 0.25 s and the 1 s bar unseen.

Evidence (`vgeom/step-focus`, PR 7766):
`app::properties_pane_tests::profile_editor_fields::a_held_step_field_lights_its_wall`
measured 0.880 s in that job's parallel pass and passed the gate. Its
siblings in `profile_editor_fields` run 0.8–2.9 s there, none of them in
`.config/nextest.toml`'s `ci` filter. That PR moves its own row into the
filter; the rest of the class is this row's.

Likely shape: time the `viewer` job's run as the `test` job's is timed,
then move or speed up what it finds.
