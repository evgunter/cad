p='/home/user/cad-mut/crates/topo/src/validate.rs'
s=open(p).read()
old="""                    Ok(CurvedPlacement::Trim(Some(FaceContainment::In))) => {
                        errors.push(ValidationError::ChartSingularityNotAVertex {
                            face,
                            point,
                            lies: SingularityLies::Interior,
                        });
                    }
                    Err(ContainError::Escalated { diag, .. }) => {
                        errors.push(ValidationError::ChartSingularityUndecided {
                            face,
                            point,
                            reading: Undecided::Side(SideUnread::Banded(diag)),
                        });
                    }"""
assert s.count(old)==1
s=s.replace(old,"""                    Ok(CurvedPlacement::Trim(Some(FaceContainment::In))) if std::env::var("MUT5").is_err() => {
                        errors.push(ValidationError::ChartSingularityNotAVertex {
                            face,
                            point,
                            lies: SingularityLies::Interior,
                        });
                    }
                    Err(ContainError::Escalated { diag, .. }) if std::env::var("MUT6").is_err() => {
                        errors.push(ValidationError::ChartSingularityUndecided {
                            face,
                            point,
                            reading: Undecided::Side(SideUnread::Banded(diag)),
                        });
                    }
                    Ok(CurvedPlacement::Trim(Some(FaceContainment::In))) | Err(ContainError::Escalated { .. }) => {}""")
open(p,'w').write(s)
