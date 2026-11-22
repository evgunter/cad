# name -> (file, old, new); applied one at a time to /home/user/mut
P='crates/geom-brep/src/pcurve_cache.rs'
D='crates/topo/src/pcurves/decision.rs'
F='crates/topo/src/boolean/fate.rs'
C='crates/geom-brep/src/props/curved.rs'
M={
 'room_old_arm': (D,"geom_brep::parallel_curvature_arm(chart.surface(), vertex),\n            ),\n            ends,",
   "geom_core::Lever::curvature_radius(axis_distance(chart, vertex).unwrap() - T::from_f64(2.0 * band.zero())),\n            ),\n            band,"),
 'room_pad_dropped': (D,"            ),\n            ends,","            ),\n            band,"),
 'room_surface_arm': (D,"geom_brep::parallel_curvature_arm(chart.surface(), vertex)","geom_brep::curvature_lever_arm(chart.surface(), vertex)"),
 'room_pad_2eps': (D,"band\n        .padded(band.zero())","band\n        .padded(0.5 * band.zero())"),
 'bis_unit': (F,"Margin::levered(len, start.min(end))","Margin::levered(len, geom_core::Lever::curvature_radius(T::one()))"),
 'bis_far': (F,"Margin::levered(len, start.min(end))","Margin::levered(len, start.max(end))"),
 'bis_box': (F,"Margin::levered(len, start.min(end))","Margin::levered(len, geom_core::Lever::session_box())"),
 'bis_sector_arm': (F,"Margin::levered(len, start.min(end))","Margin::levered(len, s.arm)"),
 'beta_old_arm': (P,"Margin::sign_of(T::one() - image.beta.abs()),\n        Band::enclosure_sign(),",
   "Margin::levered(T::one() - image.beta.abs(), geom_core::Lever::curvature_radius(arm)),\n        band,"),
 'beta_run_band': (P,"Margin::sign_of(T::one() - image.beta.abs()),\n        Band::enclosure_sign(),","Margin::sign_of(T::one() - image.beta.abs()),\n        band,"),
 'beta_box': (P,"Margin::sign_of(T::one() - image.beta.abs()),\n        Band::enclosure_sign(),",
   "Margin::levered(T::one() - image.beta.abs(), geom_core::Lever::session_box()),\n        band,"),
 'beta_abs_dropped': (P,"Margin::sign_of(T::one() - image.beta.abs())","Margin::sign_of(T::one() - image.beta)"),
 'offaxis_nocos': (C,"Margin::of((w - axis * w.dot(axis)).norm() * cos_a)","Margin::of((w - axis * w.dot(axis)).norm())"),
 'offaxis_sin': (C,"Margin::of((w - axis * w.dot(axis)).norm() * cos_a)","Margin::of((w - axis * w.dot(axis)).norm() * sin_a)"),
 'offaxis_tiny': (C,"Margin::of((w - axis * w.dot(axis)).norm() * cos_a)","Margin::of((w - axis * w.dot(axis)).norm() * cos_a * T::from_f64(1e-3))"),
 'unitsign_minor': (P,"decide_positive(name, Margin::levered(part.abs(), arm), band)","decide_positive(name, Margin::levered(part.abs(), geom_core::Lever::curvature_radius(r)), band)"),
 'unitsign_both_minor': (P,"let arm = crate::circle_curvature_arm(&circle);","let arm = geom_core::Lever::curvature_radius(r);"),
 'tilt_minor': (P,"Margin::levered(n.cross(axis).norm() - r / big, arm),","Margin::levered(n.cross(axis).norm() - r / big, geom_core::Lever::curvature_radius(r)),"),
 'tilt_tiny': (P,"Margin::levered(n.cross(axis).norm() - r / big, arm),","Margin::levered(n.cross(axis).norm() - r / big, geom_core::Lever::curvature_radius(big * T::from_f64(1e-6))),"),
 'tilt_huge': (P,"Margin::levered(n.cross(axis).norm() - r / big, arm),","Margin::levered(n.cross(axis).norm() - r / big, geom_core::Lever::curvature_radius(big * T::from_f64(1e6))),"),
 'spiric_old_R': (P,"Margin::levered(chart_axis.dot(axis), arc),","Margin::levered(chart_axis.dot(axis), geom_core::Lever::curvature_radius(major_radius)),"),
 'spiric_tiny': (P,"Margin::levered(chart_axis.dot(axis), arc),","Margin::levered(chart_axis.dot(axis), geom_core::Lever::curvature_radius(T::from_f64(1e-9))),"),
 'spiric_huge': (P,"Margin::levered(chart_axis.dot(axis), arc),","Margin::levered(chart_axis.dot(axis), geom_core::Lever::session_box()),"),
}
import sys
name=sys.argv[1]; f,o,n=M[name]
p='/home/user/mut/'+f; s=open(p).read()
if sys.argv[2:]==['revert']:
    assert s.count(n)==1,(name,'rev'); s=s.replace(n,o)
else:
    assert s.count(o)==1,(name,s.count(o)); s=s.replace(o,n)
open(p,'w').write(s)
