pub struct Board {
    pub(crate) size: usize,
    pub(crate) box_size: usize,
    pub(crate) cells: Vec<Vec<i32>>,
}

pub struct BorderStyle {
    pub(crate) left: char,
    pub(crate) fill: &'static str,
    pub(crate) cell: char,
    pub(crate) box_junction: char,
    pub(crate) right: char,
}

impl BorderStyle {
    pub(crate) const TOP: BorderStyle = BorderStyle {
        left: '╔',
        fill: "═══",
        cell: '╤',
        box_junction: '╦',
        right: '╗',
    };

    pub(crate) const BOTTOM: BorderStyle = BorderStyle {
        left: '╚',
        fill: "═══",
        cell: '╧',
        box_junction: '╩',
        right: '╝',
    };

    pub(crate) const THICK: BorderStyle = BorderStyle {
        left: '╠',
        fill: "═══",
        cell: '╪',
        box_junction: '╬',
        right: '╣',
    };

    pub(crate) const THIN: BorderStyle = BorderStyle {
        left: '╟',
        fill: "───",
        cell: '┼',
        box_junction: '╫',
        right: '╢',
    };
}
