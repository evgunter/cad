    use crate::names::EntityKey;
    use ladder::Landing;
    use topo::Operand;

    let resolve_one = |name: &names::StableName| -> Result<(Operand, EntityKey), NodeErrorKind> {
        let refused = |error| NodeErrorKind::DeclareResolve {
            error: Box::new(error),
        };
        // Rung 1 first: a dead minting node refuses NodeGone whatever
        // the tables say, so the side-picking below never preempts it.
        let live = ladder::live(name, doc).map_err(refused)?;
        // Side-picking is this door's own. A name PRESENT in both
        // operands (unique or tied, either counts as present) is not
        // an N5 failure — it is this door declining to guess a side.
        let (op, landing) = match (
            ladder::landing(a_table, name),
            ladder::landing(b_table, name),
        ) {
            (Landing::Absent, b) => (Operand::B, b),
            (a, Landing::Absent) => (Operand::A, a),
            _ => {
                return Err(NodeErrorKind::DeclareBothOperands {
                    name: Box::new(name.clone()),
                });
            }
        };
        // Absent from both operand tables lands here as `Absent`, and
        // the ladder's rung 3 turns it into Vanished.
        Ok((op, ladder::resolve(live, landing).map_err(refused)?.key))
    };
