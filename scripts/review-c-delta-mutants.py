import sys,subprocess,os,re
W=sys.argv[1]; name=sys.argv[2]
P=W+'/crates/topo/src/pcurves.rs'; J=W+'/crates/topo/src/joint.rs'
M={
 'j1_refuse':(P,"                Err(_) => (0, true),","                Err(cause) => return Err(PinMiss::Escalated(cause)),"),
 'j1_marks':(P,"                Ok(Sign::Positive) => (\n                    period","                Ok(Sign::Positive) | Err(_) => (\n                    period"),
 'j1_shift0':(P,"                Err(_) => (0, true),","                Err(_) => (0, false),"),
 'j2_v_rate':(P,"Margin::metered_sup(v1 - v0, v_meter(chart))","{ let _ = (v0, v1); Margin::of(v_meter(chart).get()) }"),
 'j2_u_rate':(P,"chart_u_arm(chart, T::zero()).meter(u1 - u0)","{ let _ = (u0, u1); Margin::of(chart_u_arm(chart, T::zero()).magnitude()) }"),
 'c_apply_tau':(J,"periods.v()?)","geom_brep::Period::new(<T as geom_core::Real>::tau()))"),
 'c_open_some':(P,"net_closes(payload, NetChannel::V, band).then(|| VPeriod::Net(Period::new(v1 - v0)))","{ let _ = band; Some(VPeriod::Net(Period::new(v1 - v0))) }"),
}
f,a,b=M[name]
s=open(f).read(); assert s.count(a)==1,(name,s.count(a)); 
if name=='j1_marks':
    s=s.replace(a,b).replace("                Err(_) => (0, true),\n","",1)
else: s=s.replace(a,b)
open(f,'w').write(s)
