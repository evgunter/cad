You are a REVIEWER lane on a Rust B-rep CAD kernel (repository evgunter/cad), running in your own cloud session and container. You are NOT an orchestrator: `memories/orchestration-model.md` is not about you. You were dispatched by the TESSLANE orchestrator, session `session_01BJC3B1cF4p5C8uaoTcK91p`.

Read `CLAUDE.md`, then `docs/prompts/reviewer-style-lane.md` IN FULL (the standing second half of every review; it binds you), then `memories/review-and-dependency-policy.md` and `memories/output-stability-as-justification.md`.

This is a FULL review: explicit claims to falsify, alongside the style lane. The claims below are the dispatcher's hypotheses about the change; check them against the tree before you build on them. Correcting the dispatcher is this lane working, not a malfunction.

**A second reviewer is reviewing this same head at the same time on this same brief.** Until your own report is delivered, do not fetch, check out or read any other review lane's branches, scratchpads, CI artifacts or pull-request comments, and do not read any other pull request's review thread; if you glimpse any of it by accident, say so in your report. Do not fetch or read any branch under `analysis/`. Of pull request PRNUM read the description and the check runs only, never its comment thread.

## What to review

Pull request evgunter/cad#PRNUM, branch `tesslane/domain-loops`, at the FROZEN HEAD `FROZEN`. Check it out detached: `git fetch origin tesslane/domain-loops main && git checkout --detach FROZEN`. The unit alone is `git diff origin/main...FROZEN -- crates demos scripts .config`. The unit's spec is `work/tesslane/a-curved-faces-domain-is-read-from-every-loop.md` (its last section, "Unit (2026-11-14, TESSLANE)", above all); `work/tesslane/plan.md`'s section **The chart lane** says where it sits (unit 3). `crates/mesh/README.md` `[mesh.sizing]` is the ratified page for this crate's sizing. The PR description states the implementer's measurement and account.

Background. A curved face is meshed in its chart `(u, v)`: boundary chord points shared with the neighbouring faces, a lattice of interior points, a constrained Delaunay triangulation, a per-triangle certificate. Until this unit the "trimmed" lane read a face's outer loop only, and both curved lanes refused any face holding an inner loop (a ring) with `TessellateError::RingOnCurvedFace`. `topo::Body::face_lift` answers a face's outer loop in its chart and every ring placed in the outer loop's sheet (a ring can be stored a whole period away). The unit makes the lane's domain that one value: the polygon is the outer lift less the rings, inside or outside is the planar lane's even-odd reading, a ring's edges are sized by the chord pass like any boundary, a ringed curved face is routed to this lane, and the ring guards and `RingOnCurvedFace` are gone. A ring placed a period away, counted on the wrong side, or dropped gives a mesh with a hole filled or a wall missing, and where its neighbour's chord points still pair up `check_mesh` can pass it. A face whose mesh moved silently breaks byte-identical replay.

## Claims to falsify

CLAIMS

## How to work

- This container is yours alone; build freely (`-p mesh`, `-p geom-brep`, `-p topo`, `-p sweep`; `demos/tour` is an excluded root with its own `Cargo.toml`). Execute wherever a claim can be run: probes, mutants, adversarial faces, sampled deviations. Say in the report what you executed. Writable disk is a fixed allowance: delete build output you no longer need rather than letting a build fail on space.
- Foreground rule, both halves: never arm waiters or background chains and never end your turn with a job running; a job that could outlive a 600 s foreground call is launched detached with output to a file and its PID recorded, then polled in the foreground.
- Commit nothing to `tesslane/domain-loops`, push nothing, open no pull request and post no GitHub comment. If you write a test worth keeping, give it in the report.
- **Your report does not name your model or vendor**, and neither does anything else you write. No email addresses or personal identifiers.

## Report

Send your report to the orchestrator with the `send_message` tool of the claude-code-remote server (load it with ToolSearch if it is deferred), `session_id` `session_01BJC3B1cF4p5C8uaoTcK91p`, then end your turn. The report, ≤150 lines:

- **Verdict**, one of: APPROVE · APPROVE-WITH-FIXES · NOT-MERGEABLE-AS-IS · REJECT.
- **Each claim**: held / falsified / corrected, with what you ran.
- **Findings** as MAJOR / MINOR / NOTE, each with file and name (a line may ride along), each marked DEMONSTRATED (a red probe, a mutant gone red, a measured wrong value) or BY-INSPECTION.
- **Spec deviations**: reported by the PR vs SILENT.
- **`## Style`**: per `reviewer-style-lane.md` §3, every finding with `sure` / `likely` / `unsure`, no proposed fixes, and the list of questions you actually exercised.
- **Rubric**, each 1–5 with one line of evidence: idiom/structure; test quality (do the tests pin the real contract?); doc/comment honesty.
- What you executed, and what you read only; and whether you saw any other review's material.
