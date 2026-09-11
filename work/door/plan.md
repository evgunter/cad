# DOOR — the doors whose fix is already written (plan)

**STATUS: OPEN (2026-09-11).** Opened in the tracker cut of 2026-09-11
(`docs/WORK-TRACKS-2026-09.md` addendum 3). Live state is
`work/door/log.md`'s tail and the item files beside this plan.

Branch prefix: **`door/`** — unit branches `door/<unit>-<slug>`.
Away-channel tag `(DOOR orchestrator)`. A/B ordinal band
**DOOR = 3800–3899**.

## Charter

A row belongs here on one test: **reading it tells you the diff.** Not
"the problem is understood" — the *fix* is written, in the row, and a
lane can land it without deciding anything first. Seven of the eleven
are `E` on that test; the four `M` rows are here because the fix is
still written, and what they add is a second file or one small call
(where a shared helper's home goes, what a refusal's signature becomes),
not a question.

This is FIX's idea (the 2026-09-03 survey's third track) applied to what
has accumulated in `work/issues/` and `work/code-quality/` since. It
exists because *cheap* was not a visible property of the board: these
eleven rows sat interleaved with roll-ups and rulings, and nothing about
where they sat said one of them is an afternoon and another is a
quarter.

## Territory — none, and why

This program claims **no paths**. Every row sits on a file some live
program owns, which is why none of them were claimed: each is one small
thing in someone else's house. The rule that replaces a fence is the
one FIX ran on and it is strict:

- **One PR is one row.** A lane that finds itself editing a second row's
  file has left this program's posture and should say so rather than
  widen.
- **Each PR draws its own fence and announces it** to the owning program
  in the PR body, naming the file and the owner.
- **A row that grows a design question stops being this program's.** It
  is re-homed — to the owner, or to the track whose subject it has
  turned into — rather than carried here at the wrong class.

## The slate

| item | class | what it is | where the work lands |
| --- | --- | --- | --- |
| `step-adopt-let-ok-iso-discards` | **E** | Two sites in one file; keep-with-comment or typed refusal, both options spelled out | `crates/step-import/src/adopt.rs:711`, `:877` (and its error enum if converting) |
| `D306` | **E** | Reword one note; collapse two calls into existing bit-identical `ders1_in_span`. | `crates/geom-brep/src/offset_fit.rs` (cert-record note by the fit door), `crates/geom-brep/src/props/loop_area.rs` (~:165-166 eval+deriv pair) |
| `S114` | **E** | Two four-line test helpers in one crate; only question is the shared home. | `crates/geom/src/curves.rs` (~:1212), `crates/geom/src/surfaces.rs` (~:1264) — both `#[cfg(feature="interval")] mod interval` test helpers |
| `S414` | **E** | Move the finiteness check above the `self_loop` early return; one file | `crates/step-import/src/geometry.rs` (`endpoint_params`, conic arm), a refusal test in `crates/step-import/tests/` |
| `run-on-whitespace-in-message-literals` | **E** | Five literal fixes; only cross-fence coordination, no judgement | `crates/viewer/src/pick.rs:1771`, `crates/geom-brep/src/nurbs_iso.rs:112`, `crates/topo/src/boolean/reduce.rs:2126`, `crates/topo/src/chart_region.rs:816`, `crates/topo/src/props.rs:1673`, optionally `crates/viewer/tests/error_display.rs` |
| `boolean-op-has-a-third-hand-written-complete-list` | **E** | Three files; `ContactClass::ALL` is the precedent, so the fix is stated not designed. | `crates/topo/src/boolean/mod.rs`, `crates/editor-core/src/persist/kernel_wire/boolean_op.rs`, `crates/viewer/src/forms.rs`; unswept neighbours `crates/topo/src/{query.rs,param_source.rs,contact.rs}`, `crates/editor-core/src/{checks.rs,node.rs,names/role.rs}` |
| `S190` | **E** | Swap one lookup to by-pair and retire width-1 caveats, one file | `crates/editor-core/src/assembly.rs` (consumption only; kernel half is `crates/topo/src/census.rs`, Track Q) |
| `viewer-grid-pitch-nonfinite-fallback` | **M** | Small refusal-shaped signature change, but caller and tests ripple; unowned, fence drawn in the PR | `crates/viewer/src/datums.rs` (`grid_pitch` + its sole caller), `crates/viewer/tests/datum_draw.rs` |
| `patherror-display-renders-float-noise` | **M** | Decide helper's home once, then mechanical sweep of Display arms | `crates/profile/src/{path.rs,validate.rs}`, helper home likely `crates/geom-core/src` beside `Real`, other crates' `Display` impls with scalar payloads |
| `viewer-pathverb-all-hand-written-seventeen` | **M** | Two fix shapes offered; picking one needs a call on whether seventeen is deliberate. | `crates/viewer/src/forms.rs` (`PathVerb` has moved there from `app.rs`), `crates/viewer/src/app.rs`, reads `crates/profile/src/path/` `Verb::ALL` |
| `viewer-cannot-author-a-part-node` | **M** | New op plumbed through four files; must decide seat and instance args | `crates/viewer/src/session/op.rs`, `crates/viewer/src/session.rs`, `crates/viewer/src/combine.rs`, `crates/viewer/src/tools.rs`, `crates/viewer/tests/*` |

## Order

Any order; there are no dependencies between these rows, which is most
of what makes the track worth having. Two notes:

- **`run-on-whitespace-in-message-literals` touches five crates for five
  literals** and is the one row here that crosses more fences than it
  edits lines. It was parked in code-quality on "tracks empty", a park
  that was about a comment sweep it is not part of; take it as five
  one-line edits announced to five owners, or split it per owner.
- **`S190`'s trigger has fired** (#855, closed) and `work.py lint` has
  been warning about the park since. It is a verify-and-close or a
  one-file consumption change, and it is the first thing to look at.
- **The two `viewer/src/forms.rs` rows collide on one file**
  (`boolean-op-has-a-third-hand-written-complete-list` and
  `viewer-pathverb-all-hand-written-seventeen`). They are in the same
  track for that reason: land them in that order, one at a time.

## Review posture

One style review per unit against `docs/prompts/reviewer-style-lane.md`,
no A/B row — the FIX and CHROME posture. A row that changes a public
signature or a refusal's type (`patherror-display`, `viewer-grid-pitch`,
`assembly`-side consumption) takes a second correctness reviewer.

**Ordering rule 5 applies hardest here**: the fix mints a fresh instance
of the defect it closes, and on rows this small the reviewer is the only
one who has ever caught it.
## How the class column is read

`E` / `M` / `H` is a **dispatch estimate**, made on 2026-09-11 by reading
each row against the tree, and it is the axis this program's order runs
on. It is not a verdict on the finding and it is not in any header: no
field carries it, `work.py` does not parse it, and this table is the only
place it lives. A lane that finds the estimate wrong says so in its PR
and this table is corrected in the same PR.

- **E** — the fix is written in the row or obvious from it: one or a few
  files, no design question, no ruling, small diff.
- **M** — multi-file, or a small design call (where a shared home lives,
  what a door looks like), or a census or instrument to build first.
- **H** — cross-cutting, numeric or algorithmic, gated on a ruling, or
  spanning several programs' territory.

The cut that opened this program is `docs/WORK-TRACKS-2026-09.md`
addendum 3; it is a survey, and this plan supersedes it as the charter.
