use crate::literal::Literal;

/// Literal を || でつないだもの
pub type Clause = Vec<Literal>;
pub enum NormalizeError {
    TautologyClause, // tautology clause is permanently true
    EmptyClause, // empty clause cannot be satisfied
}

/// # Returns
/// * `Ok(Clause)` - トートロジーでない, 重複を削除した, 変数番号で昇順に整列した Clause
/// * `Err(NormalizeError)` - see NormalizeError
pub fn normalize_clause(clause: &mut Clause) -> Result<Clause, NormalizeError> {
    if clause.len() == 0 {
        return Err(NormalizeError::EmptyClause)
    } else if clause.len() == 1 {
        // 単位節
        return Ok(clause.to_vec());
    }

    clause.sort_unstable();
    let mut ret_clause: Clause = Vec::new();

    let mut clause_i = clause.iter();
    let mut prev = clause_i.next().unwrap();
    ret_clause.push(*prev);
    for l in clause_i {
        if prev.eq(l) {
            // deplicated literal skip
            continue;
        } else if prev.is_same_var(l) {
            // tautology
            return Err(NormalizeError::TautologyClause);
        }
        prev = l;
        ret_clause.push(*l);
    }

    if ret_clause.len() == 0 {
        return Err(NormalizeError::EmptyClause)
    }

    Ok(ret_clause)
}