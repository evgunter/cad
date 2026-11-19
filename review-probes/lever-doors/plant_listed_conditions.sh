#!/usr/bin/env bash
# usage: plant.sh NAME  (reads plant commands from function), runs gate with GATE_BASE=a55696a1 then resets
cd ~/.local/share/cad-work/lever-doors-review/wt
run() { name=$1; shift; echo "=== $name"; GATE_BASE=${BASE:-a55696a1} scripts/gates/lever-doors.sh 2>&1 | grep -v "^SITE" | cut -c1-260 | tail -${TAILN:-4}; echo "exit=${PIPESTATUS[0]}"; git checkout -q -- . ; git clean -fdq crates scripts; }
F=crates/topo/src/link.rs
C=scripts/gates/lever-doors.counts
# A count above pin
echo 'pub fn zz(p: P, q: P) -> Lever<f64> { geom_brep::ExtentBall::point(q).lever_from(p) }' >> $F; run "A count above pin"
# B pin raised with site
echo 'pub fn zz(p: P, q: P) -> Lever<f64> { geom_brep::ExtentBall::point(q).lever_from(p) }' >> $F; sed -i 's#^crates/topo/src/link.rs 3#crates/topo/src/link.rs 4#' $C; run "B pin above base"
# B2 new file + new pin
echo 'pub fn zz(p: P, q: P) -> Lever<f64> { geom_brep::ExtentBall::point(q).lever_from(p) }' > crates/topo/src/zz_new.rs; echo 'crates/topo/src/zz_new.rs 1' >> $C; run "B2 new file with new pin"
# C stale pin
sed -i '0,/ExtentBall::point/s/ExtentBall::point/ExtentBall::pointx/' $F; run "C stale pin (site removed)"
# D alias
echo 'use geom_brep::Reach as R;' >> $F; run "D alias use-as"
echo 'type B<T> = geom_brep::ExtentBall<T>;' >> $F; run "D2 type alias"
# E impl outside home
printf 'impl<T: Real> ExtentBall<T> {\n    pub fn sneak(q: Point3<T>) -> Self { Self::point(q) }\n}\n' >> crates/geom-brep/src/ssi.rs; run "E impl outside home (geom-brep)"
printf 'trait Mk { fn mk(q: P) -> Self; }\nimpl Mk for geom_brep::ExtentBall<f64> {\n    fn mk(q: P) -> Self { Self::point(q) }\n}\n' >> $F; run "E2 trait impl for ExtentBall in topo"
# F literal
echo 'pub fn zz(len: f64) -> crate::Lever<f64> { crate::Lever(len) }' >> crates/geom-core/src/linalg.rs; run "F Lever( literal in geom-core (crate::Lever path)"
echo 'use crate::Lever; pub fn zz(len: f64) -> Lever<f64> { Lever(len) }' >> crates/geom-core/src/linalg.rs; run "F2 Lever( literal bare"
