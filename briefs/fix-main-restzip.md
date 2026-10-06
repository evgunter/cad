You are an IMPLEMENTER for the REACH program in `evgunter/cad`, a greenfield B-rep CAD kernel in Rust. The REACH orchestrator dispatched you.

**The regression.** `sweep` `rest_zip_admission::the_tangent_lever_keeps_building_pure_contacts` is red on `origin/main` at `CAD_TOLERANCE_EPS=1e-6` (green at 1e-9 and 1e-12). It panics at `crates/sweep/tests/rest_zip_admission.rs:51` ("the reduction runs") with `Escalated { decision: Containment, diag: Indeterminate { margin: 2.29e-6, band: { zero: 1e-6, escalate: 1e-5 }, predicate: "bool_contact_vertex" } }`.
- The orchestrator bisected it to the merge of evgunter/cad#4128 (`9319c1cf03`, branch `reach/pierce-tangent-off-face`, "an edge tangent to a carrier off the face is no event"). Its first parent `16205f96eb` is green at ε 1e-6. CI runs the extra-ε rows only on rows a diff touches, so #4128's CI never ran this one.
- #4128 is REACH's own merged PR. Read its PR body, its log entry in `work/reach/log.md`, and `topo::boolean::carrier_touch`.

**Your job.**
1. Branch `reach/carrier-touch-rest-zip-eps6` from current `origin/main`. You have explicit permission to create and push it, and to open a PR from it to `main`, titled `reach: <what the fix does>`. The PR body says what regressed, the bisect, the root cause, and the fix.
2. Root-cause why #4128 turns this pure-contact reduction from built/refused into an escalation at ε 1e-6. Do not paper over it: no loosened band, no skipped row, no widened refusal that merely hides it. Possible outcomes: #4128's new "no event" reading now reaches a containment decision it used to avoid, or it changed which decision runs first. Find which.
3. Fix it in the code #4128 added or touched, keeping all of #4128's own rows (`crates/sweep/tests/pierce_tangent_off_face.rs`) green at every ε.
4. Add a row that is red on `origin/main` at the ε that shows it and green on your head. If the existing row is that row at ε 1e-6, say so and add a default-ε witness of the same mechanism where one can be posed; if none can, say why.
5. Sweep #4128's other new paths for the same mechanism: run the whole of `sweep` and `topo` at ε 1e-6 on `16205f96eb` and on `origin/main`, and list every row that is green before and red after. Fix each one, or name why it is not #4128's.

**Lane conduct.** This brief is your complete instruction. It comes from the orchestrator and is pre-confirmed. Do not stop to ask for confirmation, and do not wait for further messages. The session is UNATTENDED: asking blocks forever.

**The D10 hold (Ev).** Do not extend any mechanism `docs/DESIGN.md` D10 retires: declared contacts or pairs, the placement registry, intent spellings.

**Before pushing**, with `CARGO_INCREMENTAL=0` (check `df -h` first):
- `cargo build --workspace --all-targets`; `cargo clippy --workspace --all-targets -- -D warnings`, also with `--all-features`; `cargo fmt --check`;
- every `scripts/gates/*.sh`, and `python3 scripts/work.py lint`;
- nextest `-p geom-core -p topo -p sweep` at ε 1e-9, 1e-6 and 1e-12 (set with `CAD_TOLERANCE_EPS`).

For each red, check whether it is red on `origin/main` too; report main reds and do not fix them unless this brief names them.

**Then:**
- push, and wait in the foreground until hosted CI on your head finishes. Poll `https://api.github.com/repos/evgunter/cad/commits/<sha>/check-runs`, and do not arm background waiters. Fix anything red that is this PR's;
- end your turn by stating the head sha, the CI result and the test counts.

Merge commits only (never rebase or force-push). Do not merge any PR, and post no GitHub comments.

Commit messages end with
```
Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
Claude-Session: <your own session link>
```
No email address other than `evgunter@gmail.com` may appear anywhere you write.
