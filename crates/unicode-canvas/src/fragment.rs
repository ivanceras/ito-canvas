#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Cell {
    C,
    K,
    M,
    O,
    W,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Fragment {
    Line(Line),
    Arc(Arc),
    Char(char),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Line {
    start: Cell,
    end: Cell,
    is_thick: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Arc {
    start: Cell,
    end: Cell,
}

pub(crate) fn thick(start: Cell, end: Cell) -> Fragment {
    let line = Line {
        start,
        end,
        is_thick: true,
    };
    Fragment::Line(line)
}

pub(crate) fn line(start: Cell, end: Cell) -> Fragment {
    let line = Line {
        start,
        end,
        is_thick: false,
    };
    Fragment::Line(line)
}

pub(crate) fn arc(start: Cell, end: Cell) -> Fragment {
    let arc = Arc { start, end };
    Fragment::Arc(arc)
}

impl Line {
    #[allow(unused)]
    pub(crate) fn same_line(&self, other: &Self) -> bool {
        (self.start == other.start && self.end == other.end)
            || (self.end == other.start && self.start == other.end)
    }
}

impl Fragment {
    #[allow(unused)]
    pub(crate) fn same_line(&self, other: &Self) -> bool {
        match (self, other) {
            (Fragment::Line(line), Fragment::Line(other)) => {
                line.same_line(other)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_same_line() {
        let k = Cell::K;
        let o = Cell::O;
        let ko = line(k, o);
        let ok = line(o, k);

        assert!(ko.same_line(&ok));
        assert!(ok.same_line(&ko));
    }
}
