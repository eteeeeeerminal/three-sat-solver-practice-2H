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

/// Clause を && でつないだもの
pub type Clauses = Vec<Clause>;
pub struct Solver {
    // 探索する論理式
    clauses: Clauses,
    // 変数の数
    size_vars: usize,
    // 見つかった解
    model: Vec<Option<bool>>,

    // 探索に使う変数
    assigns: Vec<Option<bool>>,     // 各変数の暫定的な割り当てを保持, 変数の数と同じ長さ
    root_level: usize,
    levels: Vec<usize>,             // 各変数の決定レベルを保持, 変数の数と同じ長さ
    trail: Vec<Option<Literal>>,    // 探索, 割り当ての履歴を記録, (もしかしたらOption外せるかも)
    trail_tail: usize,              // trail の末尾を保持(いちいちリサイズしていたら大変)
    trail_lim: Vec<usize>,          // 決定変数のtrail上のindexを持つ, 末尾が直近の決定変数
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            clauses: Vec::new(),
            size_vars: 0,
            model: Vec::new(),

            assigns: Vec::new(),
            root_level: 0,
            levels: Vec::new(),
            trail: Vec::new(),
            trail_tail: 0,
            trail_lim: Vec::new(),
        }
    }

    /// 決定レベル, 決定変数の数
    fn dlevel(&self) -> usize {
        self.trail_lim.len()
    }

    /// 入力された節の番号の大きい節を見て, 変数のサイズを調節する
    fn update_size_vars(&mut self, sorted_clause: &Clause) {
        let max_n_var = sorted_clause[sorted_clause.len()-1].var() +1;
        if self.size_vars < max_n_var {
            self.size_vars = max_n_var;
            self.assigns.resize(self.size_vars, None);
            self.levels.resize(self.size_vars, 0);
            self.trail.resize(self.size_vars, None);
        }
    }

    /// 次に使う変数の番号
    fn select_var(&self) -> Option<usize> {
        // はじめに見つかった未割り当て変数
        for (i, assign) in self.assigns.iter().enumerate() {
            match assign {
                Some(_) => { continue; },
                None => { return Some(i); },
            }
        }
        None
    }

    /// 真偽値を割り当てる
    /// # Returns
    /// * `true` - 割当成功
    /// * `false` - 既に割り当てられてかつ真偽値が矛盾していれば, 充足不可
    fn assign_bool(&mut self, lit: Literal) -> bool {
        let var_n: usize = lit.var().try_into().unwrap();
        let assign_now = self.assigns[var_n];
        match assign_now {
            Some(a) => {
                return a != lit.is_pos();
            },
            None => {
                // 未割り当て
                self.assigns[var_n] = Some(lit.is_pos());
                self.levels[var_n] = self.dlevel();
                self.trail[self.trail_tail] = Some(lit);
                self.trail_tail += 1;
            },
        }
        return true;
    }

    /// 値を割り当てる
    fn assume(&mut self, lit: Literal) {
        assert_eq!(self.assigns[lit.var()], None);
        self.trail_lim.push(self.trail_tail);
        self.assign_bool(lit);
    }

    /// 与えられた Clause が現在の割り当てで真かどうか判定する
    /// # Returns
    /// * `true` - 現在の割り当てで Clause が偽になる場合
    /// * `false` - 真になる場合 || 真になるか偽になるか分からない場合
    fn is_falsified_clause(&self, clause: &Clause) -> bool {
        for lit in clause {
            let assign = self.assigns[lit.var()];
            match assign {
                Some(assign) => {
                    // 1つでも満たせば真(偽にならない)
                    if lit.is_pos() == assign {
                        return false;
                    }
                },
                None => {
                    // 真になるか偽になるか分からない
                    return false;
                },
            }
        }
        return true;
    }

    /// # Returns
    /// * `true` - 現在の割り当てで, Clauses が偽になる場合
    /// * `false` - 真になる場合 || 真になるか偽になるか分からない場合
    fn is_falsified(&self) -> bool {
        for clause in &self.clauses {
            if self.is_falsified_clause(clause) {
                // 1つでも偽なら falsifed
                return true;
            }
        }
        false
    }

    fn canceluntil(&mut self, level: usize) {
        if self.dlevel() <= level {
            return;
        }
        let bound = self.trail_lim[level];
        for c in bound..self.trail_tail {
            match self.trail[c] {
                Some(lit) => {
                    self.assigns[lit.var()] = None;
                },
                None => {
                    error!("trail: 1度も初期化されてない部分にアクセス");
                },
            }
        }
        self.trail_tail = bound;
        self.trail_lim.resize(level, 0);
    }

    /// # Returns
    /// * `true` - 成功
    /// * `false` - バックトラック失敗, UNSAT
    fn backtrack(&mut self) -> bool {
        if self.dlevel() <= 0 {
            return false;
        }
        let blevel = self.dlevel() -1;
        let lit = self.trail[self.trail_lim[blevel]].unwrap();
        self.canceluntil(blevel);
        self.assign_bool(lit.not());
        return true;
    }

    /// # Returns
    /// * `Some`
    ///   - `true` - SAT
    ///   - `false` - UNSAT
    /// * `None` - 判定不能
    fn search(&mut self) -> Option<bool> {
        loop {
            if self.is_falsified() {
                if !self.backtrack() {
                    // バックトラックできなくなった
                    // UNSAT
                    return Some(false);
                }
            } else {
                let next = self.select_var();

                match next {
                    Some(next) =>{
                        self.assume(Literal::Neg(next));
                    },
                    None => {
                        // UNSAT にならずに全ての変数を見終わった
                        for assign in &self.assigns {
                            self.model.push(*assign);
                        }
                        self.canceluntil(self.root_level);
                        return Some(true);
                    },
                }
            }
        }
    }

    pub fn add_clause(&mut self, unnormalized_clause: &mut Clause) -> bool {
        let clause= normalize_clause(unnormalized_clause);
        match clause {
            Ok(c) => {
                self.update_size_vars(&c);

                if c.len() == 1 {
                    // 単位節
                    return self.assign_bool(c[0]);
                } else {
                    self.clauses.push(c);
                }
                return true;
            },
            Err(e) => match e {
                NormalizeError::TautologyClause => {
                    info!("Appear TautologyClause: {:?}", unnormalized_clause);
                    return true
                },
                NormalizeError::EmptyClause => {
                    info!("Appear EmptyClause: {:?}", unnormalized_clause);
                    return false;
                },
            },
        }
    }

    pub fn solve(&mut self) -> Option<bool> {
        let mut search_status = None;
        while search_status == None {
            search_status = self.search();
        }

        return search_status;
    }
}