pub struct Border {
    pub use_thick_border: bool,
    pub has_top: bool,
    pub has_bottom: bool,
    pub has_left: bool,
    pub has_right: bool,

    pub is_top_left_rounded: bool,
    pub is_top_right_rounded: bool,
    pub is_bottom_left_rounded: bool,
    pub is_bottom_right_rounded: bool,
}

impl Border {
    pub fn thin() -> Self {
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

    pub fn thick() -> Self {
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

    pub fn rounded() -> Self {
        Border {
            use_thick_border: false,
            has_top: true,
            has_bottom: true,
            has_left: true,
            has_right: true,
            is_top_left_rounded: true,
            is_top_right_rounded: true,
            is_bottom_left_rounded: true,
            is_bottom_right_rounded: true,
        }
    }
}
