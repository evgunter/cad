# ATREST-9 — point-in-solid's false `Out` inside a re-posed torus shell

**Binds one implementer lane.** Deleted at merge; `work/atrest/ATREST-9.md`
survives. Read `docs/prompts/implementer-discipline.md` in full first.

Branch `atrest/9-pis-torus-pose`. Row carried:
`work/atrest/point-in-solid-reads-out-from-inside-a-re-posed-torus-barrel`
— read it in full; it quotes the point, the pose and the CI evidence.

## The defect

`boolean::solid_contain::point_in_solid_faces`, over the six faces of
the hollowed torus barrel's OUTER shell after a rigid re-pose (rotation
0.7 rad about x through `(1/4, −1/2, 1/8)`), answers **`Ok(Out)`** for a
point strictly inside that shell. The unposed body answers correctly.
That is a false answer from a CERTIFIED walk — not a refusal — and it
reaches every consumer: the boolean's containment fallback, the census's
material test, and ATREST-7's check 10 (parked on this row).

## What to do

1. **Reproduce without check 10.** Build the body through the public
   doors the torax row uses (`crates/sweep/tests/torax_axial.rs`'s
   `torus_barrel()` and its re-pose, then `topo::shell`), and call
   `point_in_solid` / `point_in_solid_faces` directly on that point.
   The repro is the red row; it must not depend on ATREST-7's branch.
2. **Measure before you fix** (`memories/refusal-text-is-not-cause.md`):
   name the ray direction that decided, every crossing it found and
   every one it should have found, and which arm dropped or misread
   the crossing. The row's first suspects — the torus arm's chart
   windows (`torus_chart_trim`, `point_on_torus_in_face`) and the
   ray×torus quartic under a general frame — are the dispatcher's
   hypotheses, not findings.
3. **Assume it is a class** (discipline §5). The defect is
   pose-dependent. Re-pose the other curved walls the walk certifies
   (cylinder, cone, sphere, and the torus at other poses and in its
   other windows) and probe interior points of each, so the fix lands
   against the whole class and not the one pose that surfaced it. A
   table of kind × pose × verdict goes in the PR body.
4. **Fix it at the arm that is wrong**, keeping the walk's existing
   postures: an answer the walk cannot certify stays a typed refusal
   (`KindUnsupported` or an escalation), never a guess. If the only
   honest fix is to refuse where it now answers, that is acceptable and
   must be said plainly.

## Seam

`crates/topo/src/boolean/solid_contain.rs` is CONTACT's ground. The
work is ATREST's (the row is claimed; CONTACT has no orchestrator);
announce the seam in the PR body and name every CONTACT-owned file you
touch.

## What you owe

The repro row (red before, green after); the kind × pose table; any
other kind the table catches, fixed or filed with its own row; the
sweep's hit list and blind spot; out-of-fence findings filed. Set the
carried row to `review` when you open the PR.

## Verification

Hosted CI on the merged head: **six** `test (eps = …, n/2)` jobs and
**five** `k-lint (gate, …)`, read at STEP level. Own `CARGO_TARGET_DIR`
outside the worktree; private scratch; never end a turn with background
work live; **do not hold the machine-wide build slot for a battery** —
targeted runs, one acquisition at a time. Do not touch
`work/atrest/log.md`. Do not merge.

## Review tier

**Dual** (`docs/DUAL-REVIEW-PROTOCOL.md`): numeric soundness of a
certified containment walk that the boolean trusts — especially tricky
logic, and a wrong fix is a wrong answer shipped to every consumer.
Class M / NUMERIC.
