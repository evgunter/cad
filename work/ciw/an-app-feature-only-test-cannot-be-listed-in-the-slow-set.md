---
id: an-app-feature-only-test-cannot-be-listed-in-the-slow-set
kind: issue
title: A test that exists only under --features app cannot be listed in the slow set: the slow-set name check reads default-feature tests only
status: open
opened: 2026-11-02
priority: P4
cost: E
---

Found by VSEAM's PR 6295 lane (2026-11-02); filed by the VSEAM
orchestrator.

`review_6295_e2e_remove_a_filleted_hole` (a whole-app row in
`crates/viewer/src/app.rs`, compiled only under `--features app`) takes
about 0.6–1.2 s locally, over the slow-set bar
(implementer-discipline §2). Listing it in `.config/nextest.toml`'s
slow filter failed CI's `check-slow-set-names.py`, which enumerates
default-feature tests only and so reported the name as nonexistent. The
row now runs in the viewer app-feature job's fast set. Every whole-app
row is in the same position: the slow set cannot hold any of them.

Fix: the name check enumerates each feature set the CI jobs build
(`--features app` for the viewer), or the slow filter allows a
per-feature section.
