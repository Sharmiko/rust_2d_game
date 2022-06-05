use ggez::graphics::Rect;


pub enum SideCollided {
    Top,
    Bottom,
    Left,
    Right
}


pub fn rect_collision(r1: &Rect, r2: &Rect) -> Option<SideCollided> {
    let dx = (r1.x + r1.w / 2.) - (r2.x + r2.w / 2.);
    let dy = (r1.y + r1.h / 2.) - (r2.y + r2.h / 2.);
    let w = (r1.w + r2.w) / 2.;
    let h = (r1.h + r2.h) / 2.;
    let cross_width = w * dy;
    let cross_height = h * dx;

    if dx.abs() <= w && dy.abs() <= h {
        if cross_width > cross_height {
            if cross_width > -cross_height {
                return Some(SideCollided::Bottom);
            } else {
                return Some(SideCollided::Left);
            }
        } else {
            if cross_width > -cross_height {
                return Some(SideCollided::Right);
            } else {
                return Some(SideCollided::Top);
            }
        }
    }

    return None;
}