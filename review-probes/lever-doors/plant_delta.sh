#!/usr/bin/env bash
# Delta plants against the fix-pass head (pins are DOOR COUNT FILE). Worktree at 7fd54f87; GATE_BASE the same.
cd ~/.local/share/cad-work/lever-doors-review/wt
run() { name=$1; echo "=== $name"; GATE_BASE=${BASE:-7fd54f87} scripts/gates/lever-doors.sh 2>&1 | grep -v "^SITE" | cut -c1-150 | grep -v "^ERROR" | tail -2; echo "exit=${PIPESTATUS[0]}"; git checkout -q -- . ; git clean -fdq crates scripts; }
F=crates/topo/src/link.rs
N=crates/step-import/src/zz_new.rs
G=crates/geom-brep/src/zz_new.rs
C=scripts/gates/lever-doors.counts
PT='pub fn zz(p: P, q: P) -> Lever<f64> { geom_brep::ExtentBall::point(q).lever_from(p) }'
echo "$PT" > crates/topo/src/zz_new.rs; echo 'ExtentBall::point 1 crates/topo/src/zz_new.rs # from crates/topo/src/link.rs' >> $C; run "H1 new file+pin citing unchanged link.rs"
echo "$PT" >> $F; sed -i 's|^ExtentBall::point 3 crates/topo/src/link.rs|ExtentBall::point 4 crates/topo/src/link.rs # from crates/topo/src/offset_axial.rs|' $C; run "H2 pin raised 3->4 citing offset_axial.rs"
echo "$PT" > crates/topo/src/zz_new.rs; echo 'ExtentBall::point 1 crates/topo/src/zz_new.rs' >> $C; run "H3 new file, new pin, no citation"
echo "$PT" >> $F; sed -i 's|^ExtentBall::point 3 crates/topo/src/link.rs|ExtentBall::point 4 crates/topo/src/link.rs|' $C; run "H4 plain raise"
echo "$PT" >> $F; run "A count above pin"
sed -i '0,/ExtentBall::point/s/ExtentBall::point/ExtentBall::of_cover/' $F; echo 'pub fn zz(len: f64) -> Lever<f64> { Lever::curvature_radius(len) }' >> $F; run "G same-file swap (point -> curvature_radius)"
sed -i '0,/ExtentBall::point/s/ExtentBall::point/ExtentBall::of_cover/' $F; echo 'pub fn zz(len: f64) -> Lever<f64> { Lever::curvature_radius(len) }' >> $F; sed -i 's|^ExtentBall::point 3 crates/topo/src/link.rs|ExtentBall::point 2 crates/topo/src/link.rs\ncurvature_radius 1 crates/topo/src/link.rs|' $C; run "G' swap with both pins edited (new door pin in file)"
sed -i '0,/ExtentBall::point/s/ExtentBall::point/zz_ball/' $F; echo 'pub fn zz_ball<T: Real>(q: Point3<T>) -> ExtentBall<T> { ExtentBall::point(q) }' >> $F; echo 'pub fn zz(p: P, q: P) -> Lever<f64> { topo::link::zz_ball(q).lever_from(p) }' > $N; run "G2 pub wrapper in pinned file (author: open by design)"
echo 'pub fn zz(p: P, q: P) -> Lever<f64> { <geom_brep::ExtentBall<f64>>::point(q).lever_from(p) }' > $N; run "J1 qualified path"
echo 'pub fn zz(p: P, q: P) -> Lever<f64> { <geom_brep::ExtentBall<A<B<C<f64>>>>>::point(q).lever_from(p) }' > $N; run "J1b qualified path, generics nested 3 deep"
echo 'pub fn zz(p: P, q: P) -> Lever<f64> { geom_brep::ExtentBall::<A<B<C<f64>>>>::point(q).lever_from(p) }' > $N; run "J1c turbofish, generics nested 3 deep"
echo 'pub fn zz(p: P, q: P) -> Lever<f64> { geom_brep::ExtentBall::<<f64 as Tr>::Out>::point(q).lever_from(p) }' > $N; run "J1d turbofish with a projection inside"
echo 'pub fn zz(p: P, len: f64) -> Lever<f64> { crate::Reach::Measured { at: p, lever: len }.lever_from(p) }' > $G; run "J2 Reach::Measured literal in geom-brep"
printf 'use crate::Reach::*;\npub fn zz(p: P, len: f64) -> Lever<f64> { Measured { at: p, lever: len }.lever_from(p) }\n' > $G; run "J2b glob import of Reach variants, bare Measured literal (geom-brep)"
printf 'use geom_brep::Reach::{Face};\npub fn zz(p: P, a: f64, l: Lever<f64>) -> R { Face { at: p, below: a, above: a, across: l } }\n' > $N; run "J2c grouped import of Reach::{Face}, bare Face literal"
echo 'pub fn zz(c: &Cover<f64>, p: P, len: f64) -> Lever<f64> { geom_brep::ExtentBall::of_cover(c).lever_from_point_and_plane(p, (p, Vec3::new(len, 0.0, 0.0))) }' > $N; run "J3 lever_from_point_and_plane"
printf 'use geom_brep::points_diameter as pd;\npub fn zz(a: &[P]) -> Lever<f64> { pd(a) }\n' > $N; run "J4 free fn renamed"
printf 'use geom_brep::{\n    points_diameter as pd,\n};\npub fn zz(a: &[P]) -> Lever<f64> { pd(a) }\n' >> crates/step-import/src/build.rs; sed -i '0,/points_diameter(/s/points_diameter(/pd(/' crates/step-import/src/build.rs; run "J4b free fn renamed in a pinned file, a call respelled"
printf 'macro_rules! door { ($t:ident, $f:ident, $x:expr) => { geom_brep::$t::$f($x) }; }\npub fn zz(q: P) -> B { door!(ExtentBall, point, q) }\n' > $N; run "J5 macro (author: open)"
printf 'trait Mk { fn mk(p: P, a: f64, l: geom_core::Lever<f64>) -> Self; }\nimpl Mk for geom_brep::Reach<f64> {\n    fn mk(p: P, a: f64, l: geom_core::Lever<f64>) -> Self { Self::of_face(p, (a, a), l) }\n}\n' > $N; run "J6 Self:: in trait impl for Reach (author: open)"
echo 'pub fn zz(s: &Surface<f64>, p: P) -> Lever<f64> { geom_brep::parallel_curvature_arm(s, p) }' > $N; run "J7 parallel_curvature_arm(any point) (author: open, pivot)"
echo 'pub fn zz(len: f64) -> Lever<f64> { geom_brep::circle_curvature_arm(&geom::Circle { frame: F, radius: len }) }' > $N; run "J8 hand-built Circle (author: open)"
printf 'pub fn zz(q: P) -> B {\n    geom_brep::ExtentBall::<\n        f64,\n    >::point(q)\n}\n' > $N; run "J10 multi-line turbofish"
printf 'pub fn zz(q: P) -> B {\n    geom_brep::ExtentBall\n        ::point(q)\n}\n' > $N; run "J10b path split over lines"
echo 'use geom_brep::ExtentBall as Ball;' >> $F; run "D alias"
printf 'impl<T: Real> ExtentBall<T> {\n    pub fn sneak(q: Point3<T>) -> Self { Self::point(q) }\n}\n' >> crates/geom-brep/src/ssi.rs; run "E impl outside home"
sed -i '0,/ExtentBall::point/s/ExtentBall::point/ExtentBall::pointx/' $F; run "C stale pin"
