use std::collections::{HashMap, HashSet};

use log::{debug, error, info};

use crate::literal::Literal;
use crate::clause::{Clause, NormalizeError, normalize_clause};

/// 所有権の関係で探索により変更される変数を分離
/// Solver から Searcher にある比較的単純な関数を呼んで操作する
struct Searcher {
    pub size_vars: usize,               // 変数の数
    pub assigns: Vec<Option<bool>>,     // 各変数の暫定的な割り当てを保持, 変数の数と同じ長さ
    pub levels: Vec<usize>,             // 各変数の決定レベルを保持, 変数の数と同じ長さ
    pub trail: Vec<Option<Literal>>,    // 探索, 割り当ての履歴を記録, (もしかしたらOption外せるかも)
    pub trail_tail: usize,              // trail の末尾を保持(いちいちリサイズしていたら大変)
    pub trail_lim: Vec<usize>,          // 決定変数のtrail上のindexを持つ, 末尾が直近の決定変数
}

impl Searcher {
    pub fn new() -> Self {
        Searcher {
            size_vars: 0,
            assigns: Vec::new(),
            levels: Vec::new(),
            trail: Vec::new(),
            trail_tail: 0,
            trail_lim: Vec::new(),
        }
    }

    /// 決定レベル, 決定変数の数
    pub fn dlevel(&self) -> usize {
        self.trail_lim.len()
    }

    /// 指定されたリテラルが, 現在の割り当てで成立するか返す
    /// # Returns
    /// * Some(flag) - flag が true なら成立, flag が false なら矛盾
    /// * None - まだ値が割り当てられていない
    pub fn is_satisfied(&self, lit: &Literal) -> Option<bool> {
        if let Some(value) = self.assigns[lit.var()] {
            return Some(value == lit.is_pos());
        } else {
            return None;
        }
    }

    /// 入力された節の番号の大きい節を見て, 変数のサイズを調節する
    pub fn update_size_vars(&mut self, sorted_clause: &Clause) {
        let max_n_var = sorted_clause[sorted_clause.len()-1].var() +1;
        if self.size_vars < max_n_var {
            self.size_vars = max_n_var;
            self.assigns.resize(self.size_vars, None);
            self.levels.resize(self.size_vars, 0);
            self.trail.resize(self.size_vars, None);
        }
    }

    /// 次に使う変数の番号
    pub fn select_var(&self) -> Option<usize> {
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
    pub fn assign_bool(&mut self, lit: Literal) -> bool {
        if let Some(is_satisfied) = self.is_satisfied(&lit) {
            return is_satisfied;
        } else {
            // 未割り当て
            let var_n = lit.var();
            self.assigns[var_n] = Some(lit.is_pos());
            self.levels[var_n] = self.dlevel();
            self.trail[self.trail_tail] = Some(lit);
            self.trail_tail += 1;
        }
        return true;
    }

    /// 値を割り当てる
    pub fn assume(&mut self, lit: Literal) {
        assert_eq!(self.assigns[lit.var()], None);
        self.trail_lim.push(self.trail_tail);
        self.assign_bool(lit);
    }

    /// 与えられた Clause が現在の割り当てで真かどうか判定する
    /// # Returns
    /// * `true` - 現在の割り当てで Clause が偽になる場合
    /// * `false` - 真になる場合 || 真になるか偽になるか分からない場合
    pub fn is_falsified_clause(&self, clause: &Clause) -> bool {
        for lit in clause {
            if let Some(is_satisfied) = self.is_satisfied(lit) {
                // 1つでも満たせば真(偽にならない)
                if is_satisfied {
                    return false;
                }
            } else {
                // 真になるか偽になるか分からない
                return false;
            }
        }
        // 全部偽になる
        return true;
    }

    pub fn canceluntil(&mut self, level: usize) {
        if self.dlevel() <= level {
            return;
        }
        let bound = self.trail_lim[level];
        for c in bound..self.trail_tail {
            if let Some(lit) = self.trail[c] {
                self.assigns[lit.var()] = None;
            } else {
                error!("trail: 1度も初期化されてない部分にアクセス");
            }
        }
        self.trail_tail = bound;
        self.trail_lim.resize(level, 0);
    }

    /// # Returns
    /// * `true` - 成功
    /// * `false` - バックトラック失敗, UNSAT
    pub fn backtrack(&mut self) -> bool {
        if self.dlevel() <= 0 {
            return false;
        }
        let blevel = self.dlevel() - 1;
        let lit = self.trail[self.trail_lim[blevel]].unwrap();
        self.canceluntil(blevel);
        self.assign_bool(lit.not());
        return true;
    }
}

/// 状態出力用
#[derive(Clone, Copy)]
pub struct Stats {
    pub conflicts: usize,
    pub decisions: usize,
    clauses: usize,
    clauses_literals: usize,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            conflicts: 0,
            decisions: 0,
            clauses: 0,
            clauses_literals: 0,
        }
    }
}

/// Clause を && でつないだもの
pub type Clauses = Vec<Clause>;
pub struct Solver {
    // 探索する論理式
    clauses: Clauses,
    // 見つかった解
    pub model: Vec<Option<bool>>,

    // 探索に使う変数
    root_level: usize,
    searcher: Searcher,

    // 監視リテラルによる単位伝播に使う変数
    watched_lit_indices: HashMap<Literal, HashSet<usize>>,  // focused_lit[literal] = ~literalを監視リテラルに持つ, Clauseのclauses上のインデックス

    // ログ等
    pub stats: Stats,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            clauses: Vec::new(),
            model: Vec::new(),

            watched_lit_indices: HashMap::new(),

            root_level: 0,
            searcher: Searcher::new(),

            stats: Stats::new(),
        }
    }

    /// # Returns
    /// * `true` - 現在の割り当てで, Clauses が偽になる場合
    /// * `false` - 真になる場合 || 真になるか偽になるか分からない場合
    fn is_falsified(&self) -> bool {
        for clause in &self.clauses {
            if self.searcher.is_falsified_clause(clause) {
                // 1つでも偽なら falsifed
                return true;
            }
        }
        false
    }

    /// watched[0] か [1] のどちらかは false_litであるwatchedを受け取り
    /// watched[1] == false_lit にする
    fn align_clause(watched: &mut Clause, false_lit: Literal) {
        if watched[0] == false_lit {
            watched[0] = watched[1];
            watched[1] = false_lit;
        }
        assert!(watched[1] == false_lit);
    }

    /// 単位伝播を実装する
    /// # Returns
    /// * `true` - backtrack する必要あり (現在の割り当てで, Clauses が偽になる場合)
    /// * `false` - backtrack する必要なし (真になる場合 || 真になるか偽になるか分からない場合)
    fn propagate(&mut self) -> bool {
        let mut c = 0;
        while let Some(lit) = self.searcher.trail[c] {
            let false_lit = lit.not();
            let watcher = self.watched_lit_indices.get(&lit).unwrap().clone();
            'clause: for &i in watcher.iter() {
                let clause= &mut self.clauses[i];
                Solver::align_clause(clause, false_lit);
                if self.searcher.is_satisfied(&clause[0]) == Some(true) {
                    // true の節はスキップ
                    continue;
                }

                // 1番目のリテラルを取替
                for k in 2..clause.len() {
                    // 未割り当てか, true となっているリテラルを探す
                    let is_satisfied = self.searcher.is_satisfied(&clause[k]);
                    if is_satisfied == Some(true) || is_satisfied == None {
                        // 変数の取替操作
                        clause[1] = clause[k];
                        clause[k] = false_lit;
                        self.watched_lit_indices.get_mut(&clause[1].not()).unwrap().insert(i);
                        self.watched_lit_indices.get_mut(&clause[k].not()).unwrap().remove(&i);
                        continue 'clause; // 次の節へ
                    }
                }

                // 1番目のリテラルが false 確定の場合 (取替できなかった)
                if self.searcher.is_satisfied(&clause[0]) == Some(false) {
                    // 矛盾!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
                    // clause[0] も false 確定
                    // backtrack させる
                    return true;

                } else {
                    // clause[0] は未割り当て (true の場合はもう見た)
                    // clause[0] を true に割当
                    self.searcher.assign_bool(clause[0]);
                }
            }

            c += 1;
            if c == self.searcher.trail_tail {
                break;
            }
        }

        // 1度も矛盾が起きなければ backtrack しなくていい
        return false;
    }

    /// # Returns
    /// * `Some`
    ///   - `true` - SAT
    ///   - `false` - UNSAT
    /// * `None` - 判定不能
    fn search(&mut self) -> Option<bool> {
        loop {
            if self.propagate() {
                self.stats.conflicts += 1;
                if !self.searcher.backtrack() {
                    // バックトラックできなくなった
                    // UNSAT
                    return Some(false);
                }
            } else {
                let next = self.searcher.select_var();
                self.stats.decisions += 1;

                if let Some(next) = next {
                    self.searcher.assume(Literal::Neg(next));
                } else {
                    // UNSAT にならずに全ての変数を見終わった
                    for assign in &self.searcher.assigns {
                        self.model.push(*assign);
                    }
                    self.searcher.canceluntil(self.root_level);
                    return Some(true);
                }
            }
        }
    }

    pub fn add_clause(&mut self, unnormalized_clause: &mut Clause) -> bool {
        let clause= normalize_clause(unnormalized_clause);
        match clause {
            Ok(c) => {
                self.searcher.update_size_vars(&c);

                let literal_num = c.len();

                if literal_num == 1 {
                    // 単位節
                    self.watched_lit_indices.entry(c[0]).or_insert(HashSet::new());
                    self.watched_lit_indices.entry(c[0].not()).or_insert(HashSet::new());
                    return self.searcher.assign_bool(c[0]);
                } else {
                    for &lit in c.iter() {
                        self.watched_lit_indices.entry(lit).or_insert(HashSet::new());
                        self.watched_lit_indices.entry(lit.not()).or_insert(HashSet::new());
                    }
                    self.watched_lit_indices.get_mut(&c[0].not()).unwrap().insert(self.stats.clauses);
                    self.watched_lit_indices.get_mut(&c[1].not()).unwrap().insert(self.stats.clauses);

                    self.clauses.push(c);
                }
                self.stats.clauses += 1;
                self.stats.clauses_literals += literal_num;
                return true;
            },
            Err(e) => match e {
                NormalizeError::TautologyClause => {
                    debug!("Appear TautologyClause: {:?}", unnormalized_clause);
                    return true
                },
                NormalizeError::EmptyClause => {
                    debug!("Appear EmptyClause: {:?}", unnormalized_clause);
                    return false;
                },
            },
        }
    }

    pub fn solve(&mut self) -> Option<bool> {
        info!("==========[MINIMUMSAT]==========");
        info!("| Conflicts |     ORIGINAL     |");
        info!("|           | Clauses Literals |");
        info!("================================");

        let mut search_status = None;
        while search_status == None {
            info!("| {:9} | {:7} {:8} |",
                self.stats.conflicts,
                self.stats.clauses,
                self.stats.clauses_literals,
            );

            search_status = self.search();
        }
        info!("================================");

        return search_status;
    }
}