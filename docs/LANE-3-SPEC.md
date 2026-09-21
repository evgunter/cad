# LANE-3 — `ShellLane` folds into `AtRestPolicy`: the shell door is a value the policy answers, the verb takes it, the witness is a function

**Status: ratified at dispatch (SCALAR orchestrator, 2026-09-21).** Binds
the implementer of unit LANE-3; deleted at merge per `docs/DOC-LEDGER.md`.
Read `docs/prompts/implementer-discipline.md` in full first. The item is
`work/scalar/lane-3-shell-lane-folded.md`; the ruling is
`work/scalar/H5.md` §RATIFIED, ruling 3 (PR 2701): "`ShellLane` folds
into `AtRestPolicy`". LANE-0 is the precedent for the shape (a `Copy`
door value with one constructor at the bound that certifies, answered
by `AtRestPolicy` as an `Option`, `None` from the `Dual` arm); LANE-1
and LANE-2 for a door taken by name. The survey with every citation is
`/home/user/scalar-briefs/survey-lane3.md` on the box.

## 0. The finding, and what the tree says

`ShellLane: Lane` (`crates/editor-core/src/verbs/shell.rs:156`) has one
required method, `run_shell(verb: &Verb<Self>, operand: &Body<Self>,
tol) -> Option<Result<VerbOut<Self>, VerbError<Self>>>` (`:159-163`),
whose four certifying arms are literally `Some(verb.run_shell(operand,
tol))` and whose `Dual` arm is `None` (`:248-262`, "a dual does not
certify"); and one provided method, `witness(ShellError<Self>) ->
ShellError<f64>` (`:171-173`), `fold_shell_error(error, Self::end)`
over editor-core's `Lane::end`. The plan's sentence cannot be done
literally: `run_shell` names `verbs::{Verb, VerbOut, VerbError}` and
`topo` cannot name `verbs` (the dependency runs `verbs → topo`); the
witness reads `Lane`, one of the six editor-core capabilities ruling 3
leaves alone. What the certifying arms actually reach is
`verbs::Verb::run_shell` (`crates/verbs/src/run.rs:470`, in `impl<T:
Decide + CertifiedBounds + topo::AtRestPolicy> Verb<T>`, `:451`) →
`topo::shell_open` (`crates/topo/src/shell.rs:967`, bounded `Decide +
CertifiedBounds + AtRestPolicy`) — a topo door with topo-nameable
types. The sole caller of the trait is `wire_shell`
(`eval/wire.rs:2397-2419`), `None` ⇒ `NodeErrorKind::ShellLaneUnsupported
{ lane: <T as Lane>::NAME }`; `verb_refused` (`:2207`) is bounded on it
only to reach `witness`; `EvalScalar` carries `ShellLane` as its
eleventh term (`eval/mod.rs:2339`), pinned by the set-equality rows in
`crates/editor-core/tests/e4_dual_door.rs:67-137`; `crates/pncad/tests/all.rs`'s
`NOT_CARRIED: [&str; 92]` roster names it (`:4466`).

## 1. What this unit delivers

**The trait goes.** `ShellLane`, its five impls and docs (`shell.rs:33-57,
145-262`), the re-export at `editor-core/src/lib.rs:222`, the
`EvalScalar` term (`eval/mod.rs:2339, :2354`) and the three literal
bound rows that pin the set (`e4_dual_door.rs:67-137` — the rows
re-written to the new set, their converse-inclusion row kept), the
`NOT_CARRIED` roster entry (92 → 91; the array's length is in its type).

**The door, in LANE-0's shape, on `AtRestPolicy`.** `topo::ShellDoor<T>`
(`Copy`; one private `fn`-pointer field with `topo::shell_open`'s
signature; one constructor `ShellDoor::certified()` in an `impl<T:
Decide + CertifiedBounds + AtRestPolicy> ShellDoor<T>` block, beside
`QuadLane` in `props.rs`); `AtRestPolicy` gains `fn shell_door() ->
Option<ShellDoor<Self>>` — `f64`, `Probe`, `Interval`, `Sym<T:
CertifiedBounds>` → `Some(ShellDoor::certified())`, `Dual` → `None`,
each arm stating its reason in one sentence (the reasons the five
`ShellLane` arms state today, `shell.rs:176-262`, move here). The
`at_rest_policy_tests` roster (`props.rs:2313-2400`) gains the shell
arm in its shape (each certifying arm's door `==` `shell_open` by
`fn_addr_eq`; the `Dual` arm `None`).

**The verb takes the door.** `verbs::Verb::run_shell` moves out of the
`Decide + CertifiedBounds + AtRestPolicy` block into the general block
as `run_shell(&self, operand, tol, door: ShellDoor<T>)` (the value
proves the scalar certifies — no `CertifiedBounds` bound needed; if
the general block's bound cannot admit it, keep the second block and
say why); `wire_shell` (`eval/wire.rs:2397`) reads
`<T as AtRestPolicy>::shell_door()` and calls `built.run_shell(&body,
tol, door)` on `Some`, and on `None` produces the SAME
`ShellLaneUnsupported { lane: <T as Lane>::NAME }` (`Lane` stays for
its `NAME` and `end`; it is not this unit's). `crates/verbs/README.md:208-212`
(the second-`impl`-block argument) is re-worded to the door.

**The witness is a function.** `witness` becomes `pub fn
fold_shell_error_at<T: Lane>(error: ShellError<T>) -> ShellError<f64>`
(or the existing `fold_shell_error` called with `T::end` at the one
site, `wire.rs:2226`), `verb_refused`, `union_refusal` and
`refusal_menu` losing the `ShellLane` bound for `Lane`; the two in-file
witness rows (`shell.rs:507-565`) call the function.

**What must not change:** every evaluation output at every scalar —
the shell verb's results at `f64` bit for bit (the tour's and the
corpus's shell documents; `lib_g17_shell_node.rs`), the `Dual`
refusal typed and alone (`lib_g17_shell_node.rs:626-650`:
`ShellLaneUnsupported { lane: "Dual" }`, no upstream node failing), the
`Interval` witness row's three ends (`shell.rs:519-565`); the pncad-py
tag `shell_lane_unsupported` (`tags.rs:936`) and the error variant's
name and `Display` (the NAME `ShellLaneUnsupported` stays — it names
the refusal, not the trait; say so at the variant); the `Sym` arm's
where-clause facts (`T: CertifiedBounds`); the K roster (no `decide(`
moves); `EvalScalar`'s remaining ten terms and their homes
(`evalscalar-allowlist.sh`'s `SEAM_HOMES`).

## 2. Docs

`docs/DUAL-DESIGN.md:96-100` (DL3's list of certified doors that run
"at scalars with certification rights … and are structurally absent at
`Dual`") gains the shell door beside the offset fit's — the mechanism
DL3 describes, naming-only under CLAUDE.md's carve-out; cite the
sentence and `git log -S` its commit. `eval/mod.rs:2313-2326`
(`EvalScalar`'s doc: `AtRestPolicy` "carries the two lane traits as its
supertraits" — one after LANE-2; and now the shell door) and
`props.rs:2118-2136` (the `offset_fit_lane` doc that says the policy is
"not a lane trait of its own" — extend to the two doors it answers);
`lane.rs:5` (names `MinClearanceLane` and `ShellLane` as `Lane`'s two
consumers — the shell's witness is now a function over `Lane`);
`wire.rs:2384`; `crates/editor-core/tests/corpus/cup.rs:40-50` and
`vessel.rs` (the held-out reason names the refusal, unchanged — check
the wording); `scripts/gates/evalscalar-allowlist.sh:8-18` — the header's
count and `AtRestPolicy`'s supertrait parenthetical re-derived from the
tree (LANE-1's and LANE-2's passes each touch this line; merge main
and write what is true then) — GUARD, naming-only.

## 3. The pin

- `at_rest_policy_tests` extended (above); a `compile_fail` doctest that
  `ShellDoor::<Dual64>::certified()` does not type-check for the bound.
- Red-first: with the `f64` arm's `Some` replaced by `None` and nothing
  else changed, the shell documents' rows go red (`lib_g17_shell_node.rs`,
  the tour's shell scenes if they run in tests, `verbs/tests/run_door.rs`'s
  shell rows) — name them; restored, green.
- The `Dual` refusal row (`lib_g17_shell_node.rs:626-650`) unchanged
  and green; the `e4_dual_door.rs` set-equality rows re-written and
  green — with the converse row proving the new literal IS `EvalScalar`.
- D9: the shell corpus documents' outputs byte for byte base vs head at
  `f64`; the pncad-py tag row.

## 4. Sweep

The class: every site naming `ShellLane` or reaching `run_shell` through
it (the survey: 22 in `shell.rs`, 17 in `eval/wire.rs`, 4 in
`eval/mod.rs`, `lib.rs:222`, `lane.rs:5`, 3 in `e4_dual_door.rs`, 2 in
`pncad/tests/all.rs`, the verbs seat and README) — dispositioned one by
one in the PR body; re-derive at your merge base. The `AtRestPolicy`
bound sites that now imply the shell door (~40 in `topo`/`verbs`,
`demos/tour/src/scalar.rs:117`): compile-only, list the count.

## 5. Fence

This program claims no paths. This unit reaches the unowned
`crates/editor-core/src/verbs/shell.rs` (in no program's `paths:`),
`crates/editor-core/src/lib.rs`, `lane.rs` (prose only); WIRE's
`eval/mod.rs`, `eval/wire.rs`, `crates/verbs/src/run.rs`,
`crates/verbs/README.md`; the unowned `topo/src/props.rs`; PROPS'
`docs/DUAL-DESIGN.md` (DL3's list) and `crates/editor-core/tests/e4_dual_door.rs`
(with TCOST/TINT); TCOST/TINT's `crates/pncad/tests/all.rs` and the
shell test files the pins touch; GUARD's `evalscalar-allowlist.sh`
(header, naming-only). Not `demos/` (compile only), not `pncad-py`
(the tag and variant are unchanged). Announced by the orchestrator in
`work/scalar/log.md` and on the PR. Merge `origin/main` immediately
before opening the PR; `python3 scripts/work.py territory --base
origin/main` in the PR body.

## 6. Verification and report

Local: `cargo nextest run -p editor-core -p verbs -p topo -p pncad` at
default, `-p editor-core -p topo` under `--features probe` and
`--features interval`; `cargo check --workspace --all-targets` at
default and `--all-features`, `cargo check` in `demos/tour`; `cargo
clippy` on the touched crates `--all-targets -- -D warnings` at default
and `--all-features` (narrowed: disk is shared — say so); `cargo fmt
--all --check` plus `--check` in `demos/tour`, `demos/wild`, `benches`;
`scripts/gates/evalscalar-allowlist.sh`, `bounds-allowlist.sh`,
`probe-suite-census.sh`, `check-interval-cfg-additive.py`,
`scripts/doc-gate.sh`; `python3 scripts/work.py lint`. Private
`CARGO_TARGET_DIR`, `CARGO_INCREMENTAL=0`; delete the target before
reporting; scratch under `/home/user/scalar-lane3-scratch/`. Hosted CI
is the verification of record; poll to conclusion in the foreground
and report the run id. Report ≤120 lines: the door and the policy
method as landed, the verb's signature, the witness function, the
`EvalScalar` set before and after with the e4 rows, the sweep table,
the red-first evidence, D9, the docs re-worded with `git log -S`
commits, deviations, rows filed. Commits are plain one-line messages:
no trailers, no model or vendor names anywhere in commits, files or the
PR body (strip any auto-appended footer through the MCP write path and
verify the live body).
