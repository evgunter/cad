#!/usr/bin/env bash
# Review 6430 r1: shapes that put an ending in a Display and pass
# scripts/gates/display-payload-only.sh. Run from the repo root on a clean tree.
set -u
f=crates/topo/src/lib.rs; cp $f /tmp/lib.rs.bak
run() { echo "--- $1"; bash scripts/gates/display-payload-only.sh 2>&1 | grep -E 'DISPLAY|ROSE|FELL|OK' | head -2; }
# 0. A write() body (ended_by_write! type) that ends at a build when at is None: count unchanged.
git apply review/6430-r1/gate-write-body.diff && run "write(None) renders ApproxRecertify at a build" ; git checkout -q crates/topo/src/transform.rs
F="core::fmt::Formatter<'_>"
for m in \
 "impl core::fmt::Display for Zz1 { fn fmt(&self, f: &mut $F) -> core::fmt::Result { f.write_str(&geom_brep::recourse::Ended::ended(&self.0, self.1)) } }" \
 "impl core::fmt::Display for Zz2 { fn fmt(&self, f: &mut $F) -> core::fmt::Result { write!(f, \"{}\", geom_brep::recourse::read_at(&self.0, Some(self.1))) } }" \
 "impl core::fmt::Display for Zz3 { fn fmt(&self, f: &mut $F) -> core::fmt::Result { f.write_str(geom_brep::recourse::defect_ending(self.1)) } }" \
 "impl core::fmt::Display for Zz4 { fn fmt(&self, f: &mut $F) -> core::fmt::Result { self.write(f, Some(self.1)) } }" \
 "impl core::fmt::Display for Zz6 { fn fmt(&self, f: &mut $F) -> core::fmt::Result { f.write_str(&self.0.ending(self.1)) } }"; do
  cp /tmp/lib.rs.bak $f; echo "$m" >> $f; run "${m:32:60}"
done
cp /tmp/lib.rs.bak $f
