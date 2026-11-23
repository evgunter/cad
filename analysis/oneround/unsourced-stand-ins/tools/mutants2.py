P='crates/geom-brep/src/pcurve_cache.rs'
D='crates/topo/src/pcurves/decision.rs'
F='crates/topo/src/boolean/fate.rs'
SA="""                Margin::sign_of(chart_axis.dot(axis)),
                Band::enclosure_sign(),"""
JR='decide("loop_windings_joint_room", Margin::of(quarter_chord), ends),'
TL="Margin::levered(n.cross(axis).norm() - r / big, arm),"
BI="Margin::levered(len, start.min(end))"
M={
 'sp_extent': (P,SA,"                Margin::levered(chart_axis.dot(axis), crate::edge_extent_inf(carrier, place.span.0, place.span.1)),\n                band,"),
 'sp_box': (P,SA,"                Margin::levered(chart_axis.dot(axis), geom_core::Lever::session_box()),\n                band,"),
 'sp_oldR': (P,SA,"                Margin::levered(chart_axis.dot(axis), geom_core::Lever::curvature_radius(major_radius)),\n                band,"),
 'sp_tinyarm': (P,SA,"                Margin::levered(chart_axis.dot(axis), geom_core::Lever::curvature_radius(T::from_f64(1e-9))),\n                band,"),
 'sp_flip': (P,SA,"                Margin::sign_of(T::zero() - chart_axis.dot(axis)),\n                Band::enclosure_sign(),"),
 'jr_old': (D,JR,'decide("loop_windings_joint_room", Margin::of(quarter_chord / distance * (distance - T::from_f64(2.0 * band.zero()))), band),'),
 'jr_nopad': (D,JR,'decide("loop_windings_joint_room", Margin::of(quarter_chord), band),'),
 'jr_sphereR': (D,JR,'decide("loop_windings_joint_room", Margin::of(quarter_chord / distance * geom_brep::curvature_lever_arm(chart.surface(), vertex).length()), ends),'),
 'tl_r': (P,TL,"Margin::levered(n.cross(axis).norm() - r / big, geom_core::Lever::curvature_radius(r)),"),
 'tl_tiny': (P,TL,"Margin::levered(n.cross(axis).norm() - r / big, geom_core::Lever::curvature_radius(big * T::from_f64(1e-6))),"),
 'tl_huge': (P,TL,"Margin::levered(n.cross(axis).norm() - r / big, geom_core::Lever::curvature_radius(big * T::from_f64(1e6))),"),
 'tl_x2': (P,TL,"Margin::levered(n.cross(axis).norm() - r / big, geom_core::Lever::curvature_radius(big * T::from_f64(2.0))),"),
 'tl_half': (P,TL,"Margin::levered(n.cross(axis).norm() - r / big, geom_core::Lever::curvature_radius(big * T::from_f64(0.5))),"),
 'bis_unit': (F,BI,"Margin::levered(len, geom_core::Lever::curvature_radius(T::one()))"),
 'bis_far': (F,BI,"Margin::levered(len, start.max(end))"),
 'bis_box': (F,BI,"Margin::levered(len, geom_core::Lever::session_box())"),
 'bis_sector_arm': (F,BI,"Margin::levered(len, s.arm)"),
}
import sys
name=sys.argv[1]; f,o,n=M[name]
p='/home/user/dold/'+f; s=open(p).read()
if sys.argv[2:]==['revert']:
    assert s.count(n)==1,(name,'rev'); s=s.replace(n,o)
else:
    assert s.count(o)==1,(name,s.count(o)); s=s.replace(o,n)
open(p,'w').write(s)
