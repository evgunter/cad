# lever-doors review probes (PR 7143, frozen head a55696a1)

- `plant_listed_conditions.sh`: plants each red condition `scripts/gates/lever-doors.sh`
  lists, in a scratch worktree of the head, with `GATE_BASE` the head. Every one reds
  except a `Lever(…)` literal spelled `crate::Lever(len)` in geom-core.
- `plant_bypasses.sh`: calls that hand a lever a caller's length and leave the gate
  green (a `# from` citation raising a pin, a same-file swap, a pub wrapper in a pinned
  file, a qualified path, a scaled plane normal, a macro, `Self::` in a trait impl for
  `Reach`, a caller's point through `parallel_curvature_arm`, hand-built geometry).
  `Reach::Measured { .. }` and `<_>::point` pass the gate too but do not compile
  outside geom-brep.
- `bypass_probes.rs`: the same calls as a test module that compiles against the head and
  asserts the caller's length arrives; it also prints the curvature readers side by side.
  To run: copy to `crates/geom-brep/tests/`, add `mod bypass_probes;` to `tests/all.rs`,
  `cargo test -p geom-brep --test all bypass_probes -- --nocapture`.

Both scripts expect a detached worktree of the head at
`~/.local/share/cad-work/lever-doors-review/wt`.
