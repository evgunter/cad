#!/usr/bin/env bash
cd ~/.local/share/cad-work/lever-doors-review/wt
run() { name=$1; echo "=== $name"; GATE_BASE=${BASE:-a55696a1} scripts/gates/lever-doors.sh 2>&1 | grep -v "^SITE" | cut -c1-200 | tail -2; echo "exit=${PIPESTATUS[0]}"; git checkout -q -- . ; git clean -fdq crates scripts; }
F=crates/topo/src/link.rs
N=crates/step-import/src/zz_new.rs
C=scripts/gates/lever-doors.counts
ap() { cat >> $N; }
# H1 '# from' citing an unchanged pinned file
echo 'pub fn zz(p: P, q: P) -> Lever<f64> { geom_brep::ExtentBall::point(q).lever_from(p) }' > crates/topo/src/zz_new.rs; echo 'crates/topo/src/zz_new.rs 1 # from crates/topo/src/link.rs' >> $C; run "H1 new file, new pin citing '# from link.rs' (link.rs pin unchanged)"
# H2 raise existing pin with # from
echo 'pub fn zz(p: P, q: P) -> Lever<f64> { geom_brep::ExtentBall::point(q).lever_from(p) }' >> $F; sed -i 's|^crates/topo/src/link.rs 3.*|crates/topo/src/link.rs 4 # from crates/topo/src/offset_axial.rs|' $C; run "H2 existing pin raised 3->4 with '# from offset_axial.rs'"
# G per-file swap: remove one, add another
sed -i '0,/ExtentBall::point/s/ExtentBall::point/ExtentBall::of_cover/' $F; echo 'pub fn zz(len: f64) -> Lever<f64> { Lever::curvature_radius(len) }' >> $F; run "G swap: one point site converted, a bare curvature_radius added, same file"
# G2 pub wrapper in pinned file replaces a site (count same)
sed -i '0,/ExtentBall::point/s/ExtentBall::point/zz_ball/' $F; echo 'pub fn zz_ball<T: Real>(q: Point3<T>) -> ExtentBall<T> { ExtentBall::point(q) }' >> $F; echo 'pub fn zz(p: P, q: P) -> Lever<f64> { topo::link::zz_ball(q).lever_from(p) }' > $N; run "G2 site refactored into pub wrapper, wrapper called from an unpinned file"
echo 'pub fn zz(p: P, q: P) -> Lever<f64> { <geom_brep::ExtentBall<f64>>::point(q).lever_from(p) }' > $N; run "J1 qualified-path <ExtentBall<f64>>::point"
echo 'pub fn zz(p: P, len: f64) -> Lever<f64> { geom_brep::Reach::Measured { at: p, lever: len }.lever_from(p) }' > $N; run "J2 Reach::Measured literal"
echo 'pub fn zz(c: &Cover<f64>, p: P, len: f64) -> Lever<f64> { geom_brep::ExtentBall::of_cover(c).lever_from_point_and_plane(p, (p, Vec3::new(len, 0.0, 0.0))) }' > $N; run "J3 lever_from_point_and_plane with scaled normal"
printf 'use geom_brep::points_diameter as pd;\npub fn zz(a: &[P]) -> Lever<f64> { pd(a) }\npub fn zz2(a: &[P]) -> Lever<f64> { pd(a) }\n' > $N; run "J4 free fn renamed by use-as (new file)"
printf 'macro_rules! door { ($t:ident, $f:ident, $x:expr) => { geom_brep::$t::$f($x) }; }\npub fn zz(q: P) -> B { door!(ExtentBall, point, q) }\n' > $N; run "J5 macro"
printf 'trait Mk { fn mk(p: P, a: f64, l: geom_core::Lever<f64>) -> Self; }\nimpl Mk for geom_brep::Reach<f64> {\n    fn mk(p: P, a: f64, l: geom_core::Lever<f64>) -> Self { Self::of_face(p, (a, a), l) }\n}\n' > $N; run "J6 Self::of_face in trait impl for Reach (other crate)"
echo 'pub fn zz(s: &Surface<f64>, p: P) -> Lever<f64> { geom_brep::parallel_curvature_arm(s, p) }' > $N; run "J7 parallel_curvature_arm(surface, any point)"
echo 'pub fn zz(len: f64) -> Lever<f64> { geom_brep::circle_curvature_arm(&geom::Circle { center: O, normal: Z, x_dir: X, radius: len }) }' > $N; run "J8 hand-built Circle"
echo 'pub fn zz(len: f64) -> Lever<f64> { geom_core::Lever::session_box().min(geom_brep::uniform_curvature_arm(&geom::Surface::Sphere { center: O, radius: len, axis: Z, x_dir: X })) }' > $N; run "J9 hand-built Sphere"
printf 'pub fn zz(q: P) -> B {\n    geom_brep::ExtentBall::<\n        f64,\n    >::point(q)\n}\n' > $N; run "J10 multi-line turbofish"
printf 'pub fn zz(q: P) -> B {\n    let f: fn(P) -> geom_brep::ExtentBall<f64> = <_>::point;\n f(q)\n}\n' > $N; run "J11 inferred <_>::point"
echo 'pub fn zz(len: f64) -> crate::Lever<f64> { crate::predicate::Lever(len) }' >> crates/geom-core/src/linalg.rs; run "F3 crate::predicate::Lever( in geom-core"
echo 'pub fn zz(len: f64) -> crate::Lever<f64> { let mut l = crate::Lever::session_box(); l.0 = len; l }' >> crates/geom-core/src/linalg.rs; run "F4 field write l.0 = len in geom-core"
