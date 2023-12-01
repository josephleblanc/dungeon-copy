#![allow(dead_code)]

use crate::config::*;

/// Change co-ordinate from transform to UI.
/// Alter the co-ordinates of an (x, y) point from a traditional cartesian
/// graph with an origin in the center of the main window, which is used by
/// the transform.translation system, to a co-ordinate with the origin in
/// the top left of the main window, with +y going down the screen and +x
/// moving right in the screen.
pub fn trans_to_window(x_tr: f32, y_tr: f32) -> (f32, f32) {
    let x_offset = WINDOW_HEIGHT * RESOLUTION / 2.0;
    let y_offset = WINDOW_HEIGHT / 2.0;
    // let tile_offset = TILE_SIZE / 2.0;

    let x_ui = x_tr + x_offset;
    let y_ui = WINDOW_HEIGHT - (y_tr + y_offset);

    (x_ui, y_ui)
}

pub fn window_to_trans(x_ui: f32, y_ui: f32) -> (f32, f32) {
    let x_offset = WINDOW_HEIGHT * RESOLUTION / 2.0;
    let y_offset = WINDOW_HEIGHT / 2.0;
    // let tile_offset = TILE_SIZE / 2.0;

    let x_tr = x_ui - x_offset;
    let y_tr = y_ui - y_offset;

    (x_tr, y_tr)
}
