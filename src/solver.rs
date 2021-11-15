use crate::literal::Literal;

pub type Clause = Vec<Literal>;
pub enum NormalizeError {
    TautologyClause, // tautology clause is permanently true
    EmptyClause, // empty clause cannot be satisfied
}

pub fn normalize_clause(clause: &mut Clause) -> Result<Clause, NormalizeError> {
    if clause.len() == 0 {
        return Err(NormalizeError::EmptyClause)
    }

    clause.sort_unstable_by(|a, b| b.cmp(a));
    let mut ret_clause: Clause = Vec::new();

    let mut prev = clause[0];
    for l in clause.iter() {
        if prev.eq(l) {
            // deplicated literal skip
            continue;
        } else if prev.is_same_var(l) {
            // tautology
            return Err(NormalizeError::TautologyClause);
        }
        prev = *l;
        ret_clause.push(*l);
    }

    if ret_clause.len() == 0 {
        return Err(NormalizeError::EmptyClause)
    }

    Ok(ret_clause)
}

pub struct Solver {
    clauses: Vec<Clause>,
}

impl Solver {
    pub fn new() -> Self {
        Solver { clauses: Vec::new() }
    }

    pub fn add_clause(&mut self, unnormalized_clause: &mut Clause) -> bool {
        let clause= normalize_clause(unnormalized_clause);
        match clause {
            Ok(c) => {
                // TODO: 単位節 の場合は割当て
                self.clauses.push(c);
                return true;
            },
            Err(e) => match e {
                NormalizeError::TautologyClause => { return true },
                NormalizeError::EmptyClause => { return false },
            },
        }
    }

    pub fn solver_solve(&mut self) -> bool {
        true
    }
}