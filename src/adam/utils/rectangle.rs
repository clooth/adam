use sdl2::rect::Rect as SdlRect;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

impl Rectangle {
    /// Generates and SDL-compatible equal to `self`.
    pub fn to_sdl(self) -> Option<SdlRect> {
        // Reject negative width and height
        assert!(self.w >= 0.0 && self.h >= 0.0);

        // Return Some(SdlRect) as planned.
        Some(SdlRect::new(self.x as i32, self.y as i32, self.w as u32, self.h as u32))
    }

    /// Returns a rectangle of given size with a (0, 0) origin.
    pub fn with_size(w: f64, h: f64) -> Rectangle {
        Rectangle {
            x: 0.0,
            y: 0.0,
            w: w,
            h: h,
        }
    }

    /// Returns a modified version of self that is centered at `center`.
    /// `center.0` = x, `center.1` = y
    pub fn centered_at(self, center: (f64, f64)) -> Rectangle {
        Rectangle {
            x: center.0 - self.w / 2.0,
            y: center.1 - self.h / 2.0,
            ..self
        }
    }

    /// Returns the center (x, y) of self.
    pub fn center(self) -> (f64, f64) {
        (self.x + self.w / 2.0, self.y + self.h / 2.0)
    }

    /// Returns whether self contains another `Rectangle` completely.
    pub fn contains(&self, rect: Rectangle) -> bool {
        let xmin = rect.x;
        let xmax = xmin + rect.w;
        let ymin = rect.y;
        let ymax = ymin + rect.h;

        (xmin >= self.x && xmin <= self.x + self.w) &&
        (xmax >= self.x && xmax <= self.x + self.w) &&
        (ymin >= self.y && ymin <= self.y + self.h) &&
        (ymax >= self.y && ymax <= self.y + self.h)
    }

    /// Returns whether self overlaps another `Reactangle`.
    pub fn overlaps(&self, other: Rectangle) -> bool {
        (self.x < other.x + other.w) && (self.x + self.w > other.x) &&
        (self.y < other.y + other.h) && (self.y + self.h > other.y)
    }

    /// Returns a (perhaps moved) Rectangle which is contained by a `parent` Rectangle.
    /// If it can indeed be moved while still fitting, return `Some(result)`; otherwise `None`.
    pub fn moved_inside(self, parent: Rectangle) -> Option<Rectangle> {
        // It must be smaller than the parent rectangle to fit.
        if self.w > parent.w || self.h > parent.h {
            return None;
        }

        Some(Rectangle {
            x: if self.x < parent.x {
                parent.x
            } else if self.x + self.w >= parent.x + parent.w {
                parent.x + parent.w - self.w
            } else {
                self.x
            },
            y: if self.y < parent.y {
                parent.y
            } else if self.y + self.h >= parent.y + parent.h {
                parent.y + parent.h - self.h
            } else {
                self.y
            },
            ..self
        })
    }
}

#[cfg(test)]
mod tests {
    use sdl2::rect::Rect as SdlRect;
    use super::Rectangle;

    #[test]
    fn test_creation() {
        let rect = Rectangle {
            x: 0.0,
            y: 0.0,
            w: 10.0,
            h: 10.0,
        };

        assert!(rect.x == 0.0);
        assert!(rect.y == 0.0);
        assert!(rect.w == 10.0);
        assert!(rect.h == 10.0);
    }

    #[test]
    fn test_to_sdl() {
        let rect = Rectangle {
                x: 0.0,
                y: 0.0,
                w: 10.0,
                h: 10.0,
            }
            .to_sdl();

        let rect_sdl = SdlRect::new(0, 0, 10, 10);

        assert!(rect.unwrap() == rect_sdl);
    }

    #[test]
    fn test_with_size() {
        let rect = Rectangle::with_size(100.0, 50.0);

        assert!(rect.x == 0.0);
        assert!(rect.y == 0.0);
        assert!(rect.w == 100.0);
        assert!(rect.h == 50.0);
    }

    #[test]
    fn test_centered_at() {
        let rect = Rectangle::with_size(100.0, 100.0).centered_at((200.0, 200.0));

        assert!(rect.x == 150.0);
        assert!(rect.y == 150.0);
        assert!(rect.w == 100.0);
        assert!(rect.h == 100.0);
    }

    #[test]
    fn test_center() {
        let rect_moved = Rectangle::with_size(100.0, 100.0).centered_at((200.0, 200.0));
        let rect_still = Rectangle::with_size(100.0, 100.0);

        assert!(rect_moved.center() == (200.0, 200.0));
        assert!(rect_still.center() == (50.0, 50.0));
    }

    #[test]
    fn test_contains() {
        let parent = Rectangle::with_size(100.0, 100.0);
        let inside_rect = Rectangle::with_size(50.0, 50.0).centered_at((50.0, 50.0));
        let outside_rect = Rectangle::with_size(50.0, 50.0).centered_at((100.0, 100.0));

        assert!(parent.contains(inside_rect) == true);
        assert!(parent.contains(outside_rect) == false);
    }

    #[test]
    fn test_overlaps() {
        let parent = Rectangle::with_size(100.0, 100.0);
        let inside_rect = Rectangle::with_size(50.0, 50.0).centered_at((50.0, 50.0));
        let outside_rect = Rectangle::with_size(50.0, 50.0).centered_at((300.0, 300.0));

        assert!(inside_rect.overlaps(parent) == true);
        assert!(outside_rect.overlaps(parent) == false);
    }

    #[test]
    fn test_moved_inside() {
        let parent = Rectangle::with_size(100.0, 100.0);
        let negative_offset_rect = Rectangle {
            x: -50.0,
            y: -50.0,
            w: 50.0,
            h: 50.0,
        };
        let positive_offset_rect = Rectangle {
            x: 100.0,
            y: 100.0,
            w: 50.0,
            h: 50.0,
        };

        let negative_moved_rect = negative_offset_rect.moved_inside(parent).unwrap();
        let positive_moved_rect = positive_offset_rect.moved_inside(parent).unwrap();

        assert!(negative_moved_rect.x == 0.0);
        assert!(negative_moved_rect.y == 0.0);
        assert!(positive_moved_rect.x == 50.0);
        assert!(positive_moved_rect.y == 50.0);
    }
}
