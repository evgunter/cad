---
id: attach-postconditions-validate-the-whole-body-and-panic
kind: issue
title: set_face_surface / set_edge_curve run validate(&self) as a postcondition: a whole-body tier-1 walk per write, and a panic on a malformed body reachable through public doors under the release profile
status: open
opened: 2026-09-08
---



Measured by SHELL-10's lane and both its reviewers (PR #2229,
2026-09-08; probe rows on `shell/10-r1-probes` and
`shell/10-r2-probes`, merged into the unit) and placed here by the
SHELL orchestrator, who agrees with both reviewers that it is TOPO's
finding on its own rather than a paragraph of a SHELL item.
`crates/topo/src/attach.rs` — `set_face_surface` (`:92-97`) and
`set_edge_curve` (`:331-336`) — each run `validate(&self)` as a
postcondition after the write. Two consequences. (1) Every `&mut
Body` door that writes N surfaces and M curves pays N + M whole-body
tier-1 walks: counted with a temporary counter, 18 per
`offset_planes_together` call on one solid of a two-box body (6 faces
+ 12 edges) and 16 per `offset_charts_together` (6 + 10), whichever
solids the scope names; 90 on `shell` of a twice-hollowed box opened
once, 101 on a box beside a vessel opened on the vessel's void
ceiling. This is why SHELL-10's narrowing of the simultaneous doors
to their scope is invisible in cost: the direct door on one of N
solids scales linearly with N on both sides of that unit (0.20 →
0.30 → 0.51 → 1.00 ms for 1-of-1/2/4/8 vessels, release, medians),
and the clone explains 0.004 ms of it. (2) The postcondition is an
assertion, and `[profile.release] debug-assertions = true`
(`Cargo.toml:302-303`, verified by execution) makes it a PANIC in
this workspace's release profile: a scoped door call on a body whose
OTHER solid has a torn loop panics inside the first setter
("set_face_surface postcondition…") — a panic on invalid input
reachable through a public door, which D9 forbids where a refusal is
owed, and the reason SHELL-10's structural acceptance row could only
pin the scope constructor, not the door (the door cannot build past
it; `shell10_r1_probes` carries the `#[should_panic]` row). What
would close it: the postcondition either becomes a debug-only check
that the release profile does not carry, or a per-write local check
(the written entity and its incident loops) with the whole-body walk
left to the door's own closing validate; either way a malformed body
met by a setter refuses typed. `work/shell/doors-still-read-the-whole-body-for-tier1.md`
keeps the tier-2 half (tier 1's passes cannot be restricted to a
shell subset) and points here for the setters. Signed (SHELL
orchestrator).

## Ruled conditionally (2026-09-14, PR 2527) — the D9 reading

Ev: close it "if your answer complies with the panic vs error
strategy in DESIGN.md". The reading, against D9: the setters' tier-1
postcondition asserts a state a WRITE cannot create from a tier-1
body; it can fail only on a body torn BEFORE the write. D9's closure
property says every public mutation path preserves tier 1 (checked
against the real surface by
`review_m1_pr5_internal::every_public_mutation_path_preserves_tier1`),
so a torn body at a setter is a kernel-bug state — D9 row 5 ("kernel
bug, detectable only by re-derivation": `debug_assert`), and under
D9's second half such a state MUST panic; downgrading it to a typed
refusal would launder a bug into a supported outcome. The one public
path D9 itself names outside the property is `instance`'s raw graft
(a spent, partially written destination after a discarded
`JoinDesync`), and D9 routes that state class to the open ruling S14
(`work/pipe/S14.md`, Ev's) — this row does not re-decide it. The cost
half is PERF-4's. That the workspace's release profile keeps debug
assertions on until the publish step is D9's own note. So the
postcondition complies as it stands and the row closes on this
reading; the close lands when Ev confirms on the PR.
