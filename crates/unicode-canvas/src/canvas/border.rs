pub(crate) struct Border {
    pub(crate) use_thick_border: bool,

    pub(crate) has_top: bool,
    pub(crate) has_bottom: bool,
    pub(crate) has_left: bool,
    pub(crate) has_right: bool,

    pub(crate) is_top_left_rounded: bool,
    pub(crate) is_top_right_rounded: bool,
    pub(crate) is_bottom_left_rounded: bool,
    pub(crate) is_bottom_right_rounded: bool,
}

impl Border {
    pub(crate) fn thin() -> Self {
        Border {
            use_thick_border: false,
            has_top: true,
            has_bottom: true,
            has_left: true,
            has_right: true,
            is_top_left_rounded: false,
            is_top_right_rounded: false,
            is_bottom_left_rounded: false,
            is_bottom_right_rounded: false,
        }
    }

    pub(crate) fn thick() -> Self {
        Border {
            use_thick_border: true,
            has_top: true,
            has_bottom: true,
            has_left: true,
            has_right: true,
            is_top_left_rounded: false,
            is_top_right_rounded: false,
            is_bottom_left_rounded: false,
            is_bottom_right_rounded: false,
        }
    }
}
