---
id: face-walks-outside-mass-properties-are-still-serial
kind: issue
title: two per-face walks in topo::props are still one face at a time
status: open
opened: 2026-09-12
parent: PERF-11
---



## The finding

PERF-8 made `mass_properties_impl`'s face loop and `sign_certified`'s
round loops an indexed parallel map over arena-order slots. Two per-face
walks in the same module were out of that unit's fence and are still
serial, both in `crates/topo/src/props.rs`:

- **`classify_shells_of`'s per-shell face loop** — the same flux per
  face, restricted to one shell's faces, read at the REPORTING level.
  It calls the same `face_flux` through the same `reporting_hook`
  (shared with the whole-body walk since PERF-8, so there is one lane
  and not two), and its inner loop is the shape `decide_faces` already
  has. Nothing structural blocks it; PERF-8's spec named two loops and
  this was not one of them.
- **`SignCertificate::refine_to_target`'s continuation** — and this one
  is NOT the same shape. It resumes each open face in arena order and
  must return at the FIRST refusal it reaches, resumed or already
  outstanding, because that is the reporting walk's own rule (its own
  doc argues why: folding refusals at the end would let a later face's
  hard refusal pre-empt an earlier face's budget one). Mapping it would
  decide faces the serial continuation never reaches — the disclosed
  failure-path cost — which is tolerable for a body that answers and is
  exactly what this door is asked for on a body that does not.

## What a fix is

For `classify_shells_of`: `decide_faces` over the shell's faces, then
the same sequential arena-order fold the loop already does. The
per-shell sum is already arena-ordered, so idiom 2 is unchanged.

For `refine_to_target`: measure first. The question is whether the
continuation's bodies are ones whose faces are open in NUMBERS worth
mapping — a certificate that settled early leaves few faces open, and
`work/perf/quadrature-setup-is-re-derived-per-round-window.md` is the
item about what re-entering an open face costs. If that one lands, this
loop's shape changes with it.

## Not in scope

The funnel door (`geom_core::k_stats::detached` / `splice`) — it exists
and either walk would use it unchanged.
