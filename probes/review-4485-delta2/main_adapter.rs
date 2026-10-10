fn green(k:&KnotVector,u:&[Interval],v:&[Interval],a:f64,b:f64,pieces:usize)->Result<Interval,geom_brep::props::PropsError>{ bspline_green_integral(k,u,v,&vec![1.0;u.len()],a,b,pieces) }
