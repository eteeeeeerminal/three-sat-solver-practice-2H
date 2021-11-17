use crate::literal::Literal;
use crate::solver::{Clause, Solver};

fn is_comment_line(line: &str) -> bool {
    for c in line.chars() {
        if c == 'c' {
            return true;
        }else if c != ' ' {
            break;
        }
    }
    false
}

fn is_probrem_line(line: &str) -> bool {
    for c in line.chars() {
        if c == 'p' {
            return true;
        }else if c != ' ' {
            break;
        }
    }
    false
}

fn line_to_clause(line: &str) -> Clause {
    let lit_raw_datum: Vec<i32> = line.split_whitespace().filter_map(|k| k.parse().ok()).collect();
    let mut clause: Clause = Vec::new();
    for lit_raw_data in lit_raw_datum {
        if lit_raw_data == 0 {
            break;
        }
        let lit_var_n: i32 = lit_raw_data.abs();
        let lit_var_n: usize = (lit_var_n - 1).try_into().unwrap();
        if lit_raw_data > 0 {
            clause.push(Literal::Pos(lit_var_n));
        } else {
            clause.push(Literal::Neg(lit_var_n));
        }
    }
    clause
}

/// # Returns
/// * `true` - 読み込み成功
/// * `false` - 解けない
///   - ファイル形式が違っている
///   - empty clause があって解けない場合
///   - x and ¬x があって解けない場合
pub fn parse_dimacs(cnf_data: &mut str, solver: &mut Solver) -> bool {
    let lines: Vec<&str> = cnf_data.split('\n').collect();

    for line in lines {
        if is_comment_line(line) || is_probrem_line(line) {
            continue;
        }
        if line.len() == 0 {
            continue;
        }

        let mut clause = line_to_clause(line);
        if !solver.add_clause(&mut clause) {
            return false;
        }
    }
    return true;
}