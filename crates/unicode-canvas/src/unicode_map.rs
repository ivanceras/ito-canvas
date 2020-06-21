use crate::fragment::Cell;
use crate::fragment::{arc, line, thick, Fragment};
use std::collections::BTreeMap;
use std::iter::FromIterator;

///```ignore
///          C
///          ╷
///          │
///          │
///          │
///     K╶───M───╴O
///          │
///          │
///          │
///          ╵
///          W
///```

pub(crate) fn unicode_map() -> Vec<(char, Vec<Fragment>)> {
    let c = Cell::C;
    let k = Cell::K;
    let m = Cell::M;
    let o = Cell::O;
    let w = Cell::W;
    vec![
        // thin lines
        ('╴', vec![line(k, m)]),
        ('╵', vec![line(c, m)]),
        ('╶', vec![line(m, o)]),
        ('╷', vec![line(m, w)]),
        ('─', vec![line(k, m), line(m, o)]),
        ('│', vec![line(c, m), line(m, w)]),
        ('┌', vec![line(m, o), line(m, w)]),
        ('┐', vec![line(k, m), line(m, w)]),
        //
        ('└', vec![line(c, m), line(m, o)]),
        ('┘', vec![line(c, m), line(k, m)]),
        ('┬', vec![line(k, m), line(m, o), line(m, w)]),
        //
        ('┴', vec![line(k, m), line(m, o), line(c, m)]),
        ('├', vec![line(c, m), line(m, w), line(m, o)]),
        //
        ('┤', vec![line(k, m), line(c, m), line(m, w)]),
        //
        ('┼', vec![line(k, m), line(m, o), line(c, m), line(m, w)]),
        // curves
        ('╭', vec![arc(o, w)]),
        ('╮', vec![line(k, w)]),
        //
        ('╰', vec![line(c, o)]),
        ('╯', vec![line(c, k)]),
        // thick lines
        ('╸', vec![thick(k, m)]),
        ('╹', vec![thick(c, m)]),
        ('╺', vec![thick(m, o)]),
        ('╻', vec![thick(m, w)]),
        //
        ('┛', vec![thick(c, m), thick(k, m)]),
        ('┓', vec![thick(k, m), thick(m, w)]),
        ('━', vec![thick(k, m), thick(m, o)]),
        ('┃', vec![thick(c, m), thick(m, w)]),
        //
        ('┗', vec![thick(c, m), thick(m, o)]),
        ('┻', vec![thick(c, m), thick(k, m), thick(m, o)]),
        ('┳', vec![thick(k, m), thick(m, o), thick(m, w)]),
        //
        ('┣', vec![thick(c, m), thick(m, w), thick(m, o)]),
        ('┏', vec![thick(m, o), thick(m, w)]),
        //
        ('┫', vec![thick(k, m), thick(c, m), thick(m, w)]),
        //
        (
            '╋',
            vec![thick(c, m), thick(m, w), thick(k, m), thick(m, o)],
        ),
        // thin and thick line combination
        ('┍', vec![thick(m, o), line(m, w)]),
        ('┎', vec![thick(m, w), line(m, o)]),
        ('┑', vec![thick(k, m), line(m, w)]),
        ('┒', vec![thick(m, w), line(k, m)]),
        //
        ('┕', vec![thick(m, o), line(c, m)]),
        ('┖', vec![thick(c, m), line(m, o)]),
        ('┙', vec![thick(k, m), line(c, m)]),
        ('┚', vec![thick(c, m), line(k, m)]),
        ('┝', vec![thick(m, o), line(c, m), line(m, w)]),
        //
        ('┞', vec![thick(c, m), line(m, w), line(m, o)]),
        //
        ('┟', vec![thick(m, w), line(c, m), line(m, o)]),
        //
        ('┠', vec![thick(c, m), thick(m, w), line(m, o)]),
        //
        ('┡', vec![thick(c, m), thick(m, o), line(m, w)]),
        //
        ('┢', vec![thick(c, m), thick(m, o), line(c, m)]),
        //
        ('┥', vec![thick(k, m), line(c, m), line(m, w)]),
        //
        ('┦', vec![thick(c, m), line(k, m), line(m, w)]),
        //
        ('┧', vec![thick(m, w), line(k, m), line(c, m)]),
        //
        ('┨', vec![thick(c, m), thick(m, w), line(k, m)]),
        //
        ('┩', vec![thick(k, m), thick(c, m), line(m, w)]),
        //
        ('┪', vec![thick(k, m), thick(m, w), line(c, m)]),
        ('┭', vec![thick(k, m), line(m, w), line(m, o)]),
        ('┮', vec![thick(m, o), line(k, m), line(m, w)]),
        ('┯', vec![thick(k, m), thick(m, o), line(m, w)]),
        ('┰', vec![thick(m, w), line(k, m), line(m, o)]),
        ('┱', vec![thick(k, m), thick(m, w), line(m, o)]),
        ('┲', vec![thick(m, o), thick(m, w), line(k, m)]),
        //
        ('┵', vec![thick(k, m), line(c, m), line(m, o)]),
        ('┶', vec![thick(m, o), line(k, m), line(c, m)]),
        ('┷', vec![thick(k, m), thick(m, o), line(c, m)]),
        ('┸', vec![thick(c, m), line(k, m), line(m, o)]),
        ('┹', vec![thick(c, m), thick(k, m), line(m, o)]),
        ('┺', vec![thick(c, m), thick(m, o), line(k, m)]),
        ('┽', vec![thick(k, m), line(c, m), line(m, w), line(m, o)]),
        //
        ('┾', vec![thick(m, o), line(c, m), line(m, w), line(k, m)]),
        //
        ('┿', vec![thick(k, m), thick(m, o), line(c, m), line(m, w)]),
        //
        ('╀', vec![thick(c, m), line(k, m), line(m, o), line(m, w)]),
        //
        ('╁', vec![thick(m, w), line(c, m), line(k, m), line(m, o)]),
        //
        ('╂', vec![thick(c, m), thick(m, w), line(k, m), line(m, o)]),
        //
        ('╃', vec![thick(k, m), thick(c, m), line(m, o), line(m, w)]),
        //
        ('╄', vec![thick(c, m), thick(m, o), line(k, m), line(m, w)]),
        //
        ('╅', vec![thick(k, m), thick(m, w), line(c, w), line(m, o)]),
        //
        ('╆', vec![thick(m, w), thick(m, o), line(c, m), line(k, m)]),
        //
        ('╇', vec![thick(c, m), thick(k, m), thick(m, o), line(m, w)]),
        //
        ('╈', vec![thick(k, m), thick(m, o), thick(m, w), line(c, m)]),
        //
        ('╉', vec![thick(k, m), thick(c, m), thick(m, w), line(m, o)]),
        //
        ('╊', vec![thick(c, m), thick(m, w), thick(m, o), line(k, m)]),
        ('╼', vec![thick(m, o), line(k, m)]),
        ('╽', vec![thick(m, w), line(c, m)]),
        ('╾', vec![thick(k, m), line(m, o)]),
        ('╿', vec![thick(c, m), line(m, w)]),
    ]
}

// TODO: put this in lazy static
pub(crate) fn fragment_char() -> BTreeMap<Vec<Fragment>, char> {
    BTreeMap::from_iter(unicode_map().iter().map(|(ch, frag)| {
        let mut frag = frag.clone();
        frag.sort();
        (frag, *ch)
    }))
}
