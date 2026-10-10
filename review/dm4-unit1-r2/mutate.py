#!/usr/bin/env python3
"""r2's mutants for PR 4527 at 3cbd7403ab. Usage: mutate.py <id> applies one
mutant in place (revert with `git checkout -- <file>`)."""
import sys, pathlib
R = pathlib.Path(__file__).resolve().parents[2]
M = {
  # claim 5: the judgement's log always joins the node's
  "M1": ("crates/editor-core/src/eval/wire.rs",
         "    if !matches!(judged, Ok(Some(_))) {\n        geom_core::k_stats::splice(judgement);",
         "    if true {\n        geom_core::k_stats::splice(judgement);"),
  # claim 6: member_of ignores which member holds the name
  "M3": ("crates/editor-core/src/names/emit_union.rs",
         "        .unwrap_or(first))",
         "        .map(|_| first).unwrap_or(first))"),
  # claim 3/DM4: a same-read declared pair is judged as a cross pair
  "M6": ("crates/editor-core/src/eval/wire.rs",
         "        if r1.at == r2.at {\n            continue;\n        }\n        let ((is, n1)",
         "        let ((is, n1)"),
  # claim 6: lift keys a subtract's `from` survivor by the tool's read
  "M7": ("crates/editor-core/src/names/role.rs",
         "            seat(from.read, under(from.read)),",
         "            seat(from.read, under(tool.read)),"),
  # claim 6: the rewriters leave a carry's read unmapped
  "M9": ("crates/editor-core/src/refactor.rs",
         "        self.2.get(&r).copied().ok_or(Unmapped::Read(r))",
         "        let _ = self.2; Ok(r)"),
  # claim 1: a plain read serializes as the struct form
  "M11": ("crates/editor-core/src/operand.rs",
          "        if self.at.is_empty() {\n            return self.read.serialize(ser);\n        }\n        let mut st",
          "        let mut st"),
}
if sys.argv[1] == "check":
    for k, (f, o, n) in M.items(): print(k, (R / f).read_text().count(o))
    sys.exit()
f, old, new = M[sys.argv[1]]
p = R / f
t = p.read_text()
assert t.count(old) == 1, (sys.argv[1], t.count(old))
p.write_text(t.replace(old, new))
print("applied", sys.argv[1], f)
