//! A Geometry module containing different structures with SDL compatibility.
use sdl2::rect::Rect as SdlRect;
use sdl2::rect::Point as SdlPoint;


#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    /// Generates and SDL-compatible equal to `self`.
    pub fn to_sdl(self) -> Option<SdlRect> {
        // Reject negative width and height
        assert!(self.width >= 0.0 && self.height >= 0.0);

        // Return Some(SdlRect) as planned.
        Some(SdlRect::new(self.x as i32,
                          self.y as i32,
                          self.width as u32,
                          self.height as u32))
    }

    /// Creates a new Rectangle starting from `origin` with a size of `size`.
    pub fn new(origin: Point, size: Size) -> Rectangle {
        Rectangle {
            x: origin.x,
            y: origin.y,
            width: size.width,
            height: size.height,
        }
    }

    /// Returns a rectangle of given size with a (0, 0) origin.
    pub fn with_size(size: Size) -> Rectangle {
        Rectangle {
            x: 0.0,
            y: 0.0,
            width: size.width,
            height: size.height,
        }
    }

    /// Returns a modified version of self that is centered at `center`.
    pub fn centered_at(self, center: Point) -> Rectangle {
        Rectangle {
            x: center.x - self.width / 2.0,
            y: center.y - self.height / 2.0,
            ..self
        }
    }

    /// Returns the center (x, y) of self.
    pub fn center(self) -> Point {
        Point {
            x: self.x + self.width / 2.0,
            y: self.y + self.height / 2.0,
        }
    }

    /// Returns the size of self.
    pub fn size(self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    /// Returns whether self contains another `Rectangle` completely.
    pub fn contains(&self, rect: Rectangle) -> bool {
        let xmin = rect.x;
        let xmax = xmin + rect.width;
        let ymin = rect.y;
        let ymax = ymin + rect.height;

        (xmin >= self.x && xmin <= self.x + self.width) &&
        (xmax >= self.x && xmax <= self.x + self.width) &&
        (ymin >= self.y && ymin <= self.y + self.height) &&
        (ymax >= self.y && ymax <= self.y + self.height)
    }

    /// Returns whether self overlaps another `Reactangle`.
    pub fn overlaps(&self, other: Rectangle) -> bool {
        (self.x < other.x + other.width) && (self.x + self.width > other.x) &&
        (self.y < other.y + other.height) && (self.y + self.height > other.y)
    }

    /// Returns a (perhaps moved) Rectangle which is contained by a `parent` Rectangle.
    /// If it can indeed be moved while still fitting, return `Some(result)`; otherwise `None`.
    pub fn moved_inside(self, parent: Rectangle) -> Option<Rectangle> {
        // It must be smaller than the parent rectangle to fit.
        if self.width > parent.width || self.height > parent.height {
            return None;
        }

        // TODO: Clean this up somehow using macros or something. It's very ugly.
        Some(Rectangle {
            x: if self.x < parent.x {
                parent.x
            } else if self.x + self.width >= parent.x + parent.width {
                parent.x + parent.width - self.width
            } else {
                self.x
            },
            y: if self.y < parent.y {
                parent.y
            } else if self.y + self.height >= parent.y + parent.height {
                parent.y + parent.height - self.height
            } else {
                self.y
            },
            ..self
        })
    }
}


/// A coordinate point with x and y values.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Returns an SDL-compatible equal to `self`.
    pub fn to_sdl(self) -> Option<SdlPoint> {
        Some(SdlPoint::new(self.x as i32, self.y as i32))
    }
}


/// A size struct with a width and height.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size {
    pub width: f64,
    pub height: f64,
}


/// A vector with delta x and delta y values.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub dx: f64,
    pub dy: f64,
}


#[cfg(test)]
mod tests {
    use float_cmp::ApproxEqUlps;
    use sdl2::rect::Rect as SdlRect;
    use super::{Rectangle, Point, Size};

    #[test]
    fn test_creation() {
        let rect = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        };

        assert!(rect.x == 0.0);
        assert!(rect.y == 0.0);
        assert!(rect.width.approx_eq_ulps(&10.0f64, 2));
        assert!(rect.height.approx_eq_ulps(&10.0f64, 2));
    }

    #[test]
    fn test_to_sdl() {
        let rect = Rectangle {
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
            }
            .to_sdl();

        let rect_sdl = SdlRect::new(0, 0, 10, 10);

        assert!(rect.unwrap() == rect_sdl);
    }

    #[test]
    fn test_with_size() {
        let rect = Rectangle::with_size(Size {
            width: 100.0,
            height: 50.0,
        });

        assert!(rect.x == 0.0);
        assert!(rect.y == 0.0);
        assert!(rect.width.approx_eq_ulps(&100.0f64, 2));
        assert!(rect.height.approx_eq_ulps(&50.0f64, 2));
    }

    #[test]
    fn test_centered_at() {
        let rect = Rectangle::with_size(Size {
                width: 100.0,
                height: 100.0,
            })
            .centered_at(Point {
                x: 200.0,
                y: 200.0,
            });

        assert!(rect.x.approx_eq_ulps(&150.0f64, 2));
        assert!(rect.y.approx_eq_ulps(&150.0f64, 2));
        assert!(rect.width.approx_eq_ulps(&100.0f64, 2));
        assert!(rect.height.approx_eq_ulps(&100.0f64, 2));
    }

    #[test]
    fn test_center() {
        let rect_moved = Rectangle::with_size(Size {
                width: 100.0,
                height: 100.0,
            })
            .centered_at(Point {
                x: 200.0,
                y: 200.0,
            });
        let rect_still = Rectangle::with_size(Size {
            width: 100.0,
            height: 100.0,
        });

        let correct_moved = Point {
            x: 200.0,
            y: 200.0,
        };

        let correct_still = Point { x: 50.0, y: 50.0 };

        assert!(rect_moved.center() == correct_moved);
        assert!(rect_still.center() == correct_still);
    }

    #[test]
    fn test_contains() {
        let parent = Rectangle::with_size(Size {
            width: 100.0,
            height: 100.0,
        });

        let inside_rect = Rectangle::with_size(Size {
                width: 50.0,
                height: 50.0,
            })
            .centered_at(Point { x: 50.0, y: 50.0 });

        let outside_rect = Rectangle::with_size(Size {
                width: 50.0,
                height: 50.0,
            })
            .centered_at(Point {
                x: 100.0,
                y: 100.0,
            });

        assert!(parent.contains(inside_rect));
        assert!(!parent.contains(outside_rect));
    }

    #[test]
    fn test_overlaps() {
        let parent = Rectangle::with_size(Size {
            width: 100.0,
            height: 100.0,
        });

        let inside_rect = Rectangle::with_size(Size {
                width: 50.0,
                height: 50.0,
            })
            .centered_at(Point { x: 50.0, y: 50.0 });

        let outside_rect = Rectangle::with_size(Size {
                width: 50.0,
                height: 50.0,
            })
            .centered_at(Point {
                x: 300.0,
                y: 300.0,
            });

        assert!(inside_rect.overlaps(parent));
        assert!(!outside_rect.overlaps(parent));
    }

    #[test]
    fn test_moved_inside() {
        let parent = Rectangle::with_size(Size {
            width: 100.0,
            height: 100.0,
        });
        let negative_offset_rect = Rectangle {
            x: -50.0,
            y: -50.0,
            width: 50.0,
            height: 50.0,
        };
        let positive_offset_rect = Rectangle {
            x: 100.0,
            y: 100.0,
            width: 50.0,
            height: 50.0,
        };

        let negative_moved_rect = negative_offset_rect.moved_inside(parent).unwrap();
        let positive_moved_rect = positive_offset_rect.moved_inside(parent).unwrap();

        assert!(negative_moved_rect.x == 0.0);
        assert!(negative_moved_rect.y == 0.0);
        assert!(positive_moved_rect.x.approx_eq_ulps(&50.0f64, 2));
        assert!(positive_moved_rect.y.approx_eq_ulps(&50.0f64, 2));
    }
}
