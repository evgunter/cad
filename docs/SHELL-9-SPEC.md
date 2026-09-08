# SHELL-9 — shell runs the closing pcurve mint

**Status: BINDING at dispatch (SHELL orchestrator, 2026-09-08).** Unit
`SHELL-9`, branch `shell/9-closing-mint`, block SHELL-B3 slot 0.
Closes the sphere half of
`work/shell/void-insertion-refuses-a-cavity-with-a-same-surface-latitude-seam.md`;
the drum half is TOPO's (`work/topo/revert-does-not-mirror-plane-chart-images.md`)
and stays open. Deleted at merge per `docs/DOC-LEDGER.md`; the item
file is the record that survives. Read
`docs/prompts/implementer-discipline.md` in full first. Survey against
main at SHELL-8's merge (2026-09-08); citations by symbol.

## 0. The semantics, stated once

The kernel's pcurve posture table (`crates/topo/src/pcurves.rs`,
`insert_void` / `insert_voids` rows) classifies the void door as
`Transfers`: the cavity is reverted (its cache rows keep their keys
and go stale in CONTENT, like any surgery) and grafted (its rows
remapped onto fresh keys verbatim), and the contract is that "both
producers' final mint passes re-derive every row of the merged body".
The boolean (`crates/topo/src/boolean/ops.rs`) and the revolve
(`crates/sweep/src/revolve/mod.rs`, `tube.rs`) run that pass.
`shell` / `shell_open` (`crates/topo/src/shell.rs`) call the void
door, then `move_shells_to_new_solid`, then the rim surgery, then
`validate_geometric` — and `mint_pcurves` nowhere. So the verb is a
producer that does not keep the producer's half of the contract, and
the assembled body carries the reverted cavity's rows as they were.

Measured (diagnosis lane, `shell/9-probe` @ `0cbb6593c`,
`crates/sweep/tests/shell9_probe.rs` rows 3–4): on a sphere authored
as two cocircular arcs (one sphere in four faces, a same-surface
latitude seam at `v = π/4`), the door's cavity is tier-3 valid; its
reversal alone already fails `Pcurve { LoopDiscontinuity }` on the
lower zone's loop — the stored rows are key-for-key the cavity's, the
forward walk parked the one-period `u` wrap at the loop's closure
(legal there), and the reversed walk meets the same `2π` gap
mid-chain at `v = π/4` where the lever is not zero; the graft copies
the rows verbatim onto the twin, and `shell`'s closing
`validate_geometric` reports it (the pinned `ShellError::NotValid`).
`topo::mint_pcurves` on the assembled body → tier 3 `Ok`, volume
`4/3·π(1 − 0.95³) = 0.5974262029576595`, pad 0.

**The decision: `shell` is a producer and runs the closing mint
itself**, once, on the assembled body, after the last surgery and
before `validate_geometric`. Rejected, logged here: making
`insert_voids` `Maintains` (re-minting inside the door) — that is a
posture-table change in S-BOOL's and TOPO's files, it would make the
void door mint a body it does not finish (the partition and the rim
surgery follow it), and the two existing producers already follow the
producer-mints convention; `shell` follows it too.

## 1. The construction

1. **The site.** One `crate::pcurves::mint_pcurves(&mut out, tol)`
   (or the crate-internal door of that name — find it; the boolean's
   call is the model) after the rim surgery and the partition, before
   the closing `validate_geometric`. Argue in the PR, step by step,
   why ONE pass at the end suffices: list every step between
   `insert_voids` and the validate that creates, keys or re-keys a
   pcurve-bearing entity (`move_shells_to_new_solid` preserves keys;
   the rim surgery of `shell_open` mints the rim faces — measure what
   it does to rows) and show each is covered by a mint that runs
   after it. If a step needs rows fresh BEFORE the end (a reader of
   `pcurves` between the void door and the validate), say which and
   where the pass has to sit instead; the number of passes is the
   minimum the readers need, stated.
2. **The refusal.** The mint's error is typed into `ShellError` —
   a new arm carrying `PcurveMintError` (name it; `NotValid` is the
   validator's and is not it) — with `Display`, and the exhaustive
   fold in `crates/editor-core` extended (a LIB seam, as SHELL-5's
   `ShellError` change was; name it in the PR body). A mint that
   refuses on a body every gate before it accepted is a kernel
   finding, surfaced typed, never a panic (D9).
3. **The rows.** Merge `shell/9-probe` (merge, never rebase); its
   four rows become ordinary tests where they are, trimmed of
   diagnosis prints that assert nothing. Flip
   `shell7_seam_corner::a_two_arc_sphere_is_taken_by_the_door_and_stops_at_the_assembly`
   to the `shells_with_one_surface_vertices` shape at
   `4/3·π(r−t)³ = 3.591364001828731` (`r = 1`, `t = 0.05`; the measured cavity volume is `…733`), with the
   seam vertices moved concentrically as its `cavity_at_closed_form`
   half already checks. Keep
   `a_collinear_cap_vertex_drum_is_taken_by_the_door_and_stops_at_void_insertion`
   refusing at `ShellError::Insert` and rewrite its doc to name the
   TOPO item (`revert`'s plane mirror) as the cause, not this verb.
4. **The pinning mutant.** With the mint call deleted, the sphere row
   and the probe's row 4 go red and nothing else does — state the
   count over `-p topo -p sweep`.
5. **Byte-identity of every committed fixture's rows.** A closing
   mint re-derives every cache row of every body `shell` builds. On a
   body whose transferred rows were content-correct, the re-derived
   row must be bit-identical (the mint is deterministic and the rows
   were minted by the same code on the same surfaces); on a body
   whose rows were stale, the mint changes them and the old body was
   carrying a hidden defect that tier 3 did not see. Build the
   instrument: a dump of every face's cache rows (`Body::pcurves`,
   per half-edge: kind, `p0`, `pa`, `pb`, `pl`, the parameter window)
   over the shell corpora that exist (`shell5_r1_dump`, `shell7_dump`,
   `shell8_dump` and `verbs_shell`'s fixtures — reuse their bodies),
   at the merge base (the commit this branch was cut from; name its
   SHA) and at the head, in a detached worktree on a PRIVATE target
   with `Compiling topo` confirmed on both sides, diffed line by
   line. Report the diff exactly: empty except the two-arc sphere
   (expected), or the rows that moved with the reason each was stale.
   A row that moves on a fixture that was tier-3 valid is a finding
   for TOPO's pcurve pass (it accepted a stale row) — report it, do
   not fix it here.
6. **Cost.** `shell-offset-three-followups` item 3 measured the verb
   at 16–23 ms in release on 3–6 charts with per-call whole-body
   mints; one more whole-body mint is the price. Measure the verb
   before and after on that item's fixture shape and state both
   numbers (release, three runs, the median).

## 2. Acceptance

1. The two-arc sphere shells to `4/3·π(r−t)³` to `1e-12`, tier-3
   valid, tessellates watertight (the row's closed-form shape).
2. The mutant of §1.4: exactly the named rows red.
3. Every committed `shell` row green, both eps lanes, interval
   included where the suites use it; the row dump diff of §1.5
   reported verbatim in the PR.
4. The drum row still refuses `ShellError::Insert` and names TOPO's
   item.
5. `shell_open`: an opened result of a hollow operand (SHELL-5's rows)
   and a multi-solid operand (SHELL-8's rows) unchanged in volume and
   valid — the closing mint on a body with rim faces, N solids and
   voids.
6. The `ShellError` arm has a row that reaches it, or the PR says
   truthfully that no committed fixture reaches it and why (the mint
   on a body every earlier gate accepted).

## 3. Stops

STOP and report if: the mint refuses on a fixture that shells today
(the rows were stale AND cannot be re-derived — a kernel finding, not
a fix here); a committed fixture's rows move under the mint on a body
that was tier-3 valid (report which rows; do not "fix" the fixture);
the one-pass argument of §1.1 fails (a reader needs fresh rows
mid-verb); or the verb's cost more than doubles on the followups
fixture.

## 4. Docs and owed

`shell.rs`'s module doc: the sentence on the closing
`validate_geometric` gains the mint that precedes it, and the
posture-table contract is cited by symbol. `crates/topo/src/pcurves.rs`'s
posture table: if it lists producers by name beside the door rows,
add `shell` (a one-line doc edit in TOPO's file — the orchestrator
announces it); otherwise nothing. `docs/KERNEL-VERBS.md`'s shell row:
nothing unless its wording claims the rows. `work/shell/void-insertion-refuses-a-cavity-with-a-same-surface-latitude-seam.md`:
the sphere half closed by this unit in its body; the item stays open
on TOPO's revert item for the drum. Lane rules as every SHELL brief:
own worktree, own `CARGO_TARGET_DIR`, narrow builds (`-p topo -p
sweep`), one heavy cargo job, foreground runs, no `Co-Authored-By`
trailer in lane commits, push after every coherent step, hosted CI is
the gate (nothing narrowed), report ≤ 150 lines.
