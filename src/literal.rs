use std::cmp;

#[derive(Debug, Clone, Copy, Eq, Hash)]
pub enum Literal {
    Pos(usize),
    Neg(usize),
}

impl Literal {
    pub fn is_same_var(&self, other: &Self) -> bool {
        self.var() == other.var()
    }

    pub fn var(&self) -> usize {
        match self {
            Self::Pos(v) | Self::Neg(v) => *v,
        }
    }

    pub fn is_pos(&self) -> bool {
        match self {
            Self::Pos(_) => true,
            Self::Neg(_) => false,
        }
    }

    pub fn not(&self) -> Self {
        match self {
            Self::Pos(v) => Self::Neg(*v),
            Self::Neg(v) => Self::Pos(*v),
        }
    }
}

impl PartialOrd for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Literal {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match (self, other) {
            (Self::Pos(p1), Self::Pos(p2)) | (Self::Neg(p1), Self::Neg(p2)) => {
                p1.cmp(p2)
            },
            (Self::Pos(p1), Self::Neg(p2)) => {
                if p1 == p2 {
                    cmp::Ordering::Less
                } else {
                    p1.cmp(p2)
                }
            },
            (Self::Neg(p1), Self::Pos(p2)) => {
                if p1 == p2 {
                    cmp::Ordering::Greater
                } else {
                    p1.cmp(p2)
                }
            },
        }
    }
}


impl PartialEq for Literal {
    fn eq(&self, other : &Self) -> bool {
        match (self, other) {
            (Literal::Pos(p1), Literal::Pos(p2)) |
            (Literal::Neg(p1), Literal::Neg(p2)) => p1 == p2,
            (Literal::Pos(_), Literal::Neg(_)) |
            (Literal::Neg(_), Literal::Pos(_)) => false,
        }
    }
}

// テストコード
#[cfg(test)]
mod tests {
    use super::Literal;

    #[test]
    fn sort_literal() {
        let mut unsorted = vec![Literal::Pos(10), Literal::Neg(1), Literal::Neg(5), Literal::Pos(1)];
        let sorted = vec![Literal::Pos(1), Literal::Neg(1), Literal::Neg(5), Literal::Pos(10)];

        unsorted.sort();
        assert_eq!(unsorted, sorted);
    }
}