use crate::{point, Bounds, Pixels, PlatformDisplay, Point, Size, WindowPosition};
use std::rc::Rc;

/// Compute window bounds from a desired size and a semantic position.
pub fn compute_window_bounds(
    size: Size<Pixels>,
    position: &WindowPosition,
    displays: &[Rc<dyn PlatformDisplay>],
    primary_display: Option<&Rc<dyn PlatformDisplay>>,
) -> Bounds<Pixels> {
    match position {
        WindowPosition::Center => {
            if let Some(display) = primary_display {
                center_in(size, display.bounds())
            } else {
                Bounds::new(Point::default(), size)
            }
        }
        WindowPosition::CenterOnDisplay(id) => {
            let display_bounds = displays.iter().find(|d| d.id() == *id).map(|d| d.bounds());
            if let Some(bounds) = display_bounds {
                center_in(size, bounds)
            } else if let Some(display) = primary_display {
                center_in(size, display.bounds())
            } else {
                Bounds::new(Point::default(), size)
            }
        }
        WindowPosition::TrayCenter(tray_bounds) => {
            let x = tray_bounds.origin.x + (tray_bounds.size.width - size.width) * 0.5;
            let y = tray_bounds.origin.y + tray_bounds.size.height;
            Bounds::new(point(x, y), size)
        }
        WindowPosition::TopRight { margin } => {
            corner_position(size, primary_display, *margin, true, false)
        }
        WindowPosition::BottomRight { margin } => {
            corner_position(size, primary_display, *margin, true, true)
        }
        WindowPosition::TopLeft { margin } => {
            corner_position(size, primary_display, *margin, false, false)
        }
        WindowPosition::BottomLeft { margin } => {
            corner_position(size, primary_display, *margin, false, true)
        }
    }
}

fn center_in(size: Size<Pixels>, display: Bounds<Pixels>) -> Bounds<Pixels> {
    let x = display.origin.x + (display.size.width - size.width) * 0.5;
    let y = display.origin.y + (display.size.height - size.height) * 0.5;
    Bounds::new(point(x, y), size)
}

fn corner_position(
    size: Size<Pixels>,
    primary_display: Option<&Rc<dyn PlatformDisplay>>,
    margin: Pixels,
    right: bool,
    bottom: bool,
) -> Bounds<Pixels> {
    if let Some(display) = primary_display {
        let db = display.bounds();
        let x = if right {
            db.origin.x + db.size.width - size.width - margin
        } else {
            db.origin.x + margin
        };
        let y = if bottom {
            db.origin.y + db.size.height - size.height - margin
        } else {
            db.origin.y + margin
        };
        Bounds::new(point(x, y), size)
    } else {
        Bounds::new(Point::default(), size)
    }
}
