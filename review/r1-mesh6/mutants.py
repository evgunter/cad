#!/usr/bin/env python3
"""R1 MESH-6 mutation battery: apply one mutant, run the named unit rows, restore.
Files are restored from `git show HEAD:<path>` so the tree is byte-identical afterwards."""
import subprocess, sys, os
LANE="/root/lanes/mesh-6r1"; os.chdir(LANE)
ENV=dict(os.environ, CARGO_TARGET_DIR=f"{LANE}/target")
C="crates/mesh/src/curved.rs"; T="crates/mesh/src/tessellate.rs"
MUT={
 "M1_no_uv_repeat": (C, "            .is_some_and(|p| p != (e.u, e.v))", "            .is_some_and(|_p| false)"),
 "M2_threshold_4":  (C, "    uses.iter().find(|&(_, &n)| n > 2).map(|(&e, &n)| (e, n))", "    uses.iter().find(|&(_, &n)| n > 4).map(|(&e, &n)| (e, n))"),
 "M3_drop_pole_keep": (C, "        if e.pole {\n            identified.insert(e.id);\n        }\n", ""),
 "M4_old_pole_filter": (C, "            if identified.contains(&a) || identified.contains(&b) {", "            if polygon.iter().any(|e| e.pole && (e.id == a || e.id == b)) {"),
 "M5_chord_gt2": (T, "    uses.iter().find(|&(_, &n)| n != 2).map(|(&e, &n)| (e, n))", "    uses.iter().find(|&(_, &n)| n > 2).map(|(&e, &n)| (e, n))"),
 "M6_no_mark": (T, "            if a < shared_below\n                && b < shared_below\n                && let Some(n)", "            if let Some(n)"),
 "M7_bits_compare": (C, "            .is_some_and(|p| p != (e.u, e.v))", "            .is_some_and(|p| (p.0.to_bits(), p.1.to_bits()) != (e.u.to_bits(), e.v.to_bits()))"),
}
FILTER="identified_ids a_seam_vertex the_full_2pi a_repeat_at_the_same unpaired_chord a_boundary_the_second a_shared_chord a_seam_edge_traversed a_segment_no_face ids_at_or_above"
def restore(p): open(p,"w").write(subprocess.run(["git","show",f"HEAD:{p}"],capture_output=True,text=True,check=True).stdout)
for name in (sys.argv[1:] or MUT):
    p,old,new=MUT[name]; restore(p); s=open(p).read()
    assert s.count(old)==1,(name,s.count(old)); open(p,"w").write(s.replace(old,new))
    r=subprocess.run(["local-scripts/with-build-slot.sh","--","cargo","test","-p","mesh","--lib","--"]+FILTER.split(),env=ENV,capture_output=True,text=True)
    out=r.stdout+r.stderr
    failed=[l.split()[1] for l in out.splitlines() if l.startswith("test ") and l.rstrip().endswith("FAILED")]
    comp="COMPILE-ERROR" if "error[" in out or "could not compile" in out else ""
    print(f"{name}: exit={r.returncode} {comp} red_rows={failed or 'NONE'}"); sys.stdout.flush()
    restore(p)
print("restored; git status:"); subprocess.run(["git","status","--short","crates/mesh/src"])
