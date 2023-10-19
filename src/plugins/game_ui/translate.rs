use crate::config::*;

/// Change co-ordinate from transform to UI.
/// Alter the co-ordinates of an (x, y) point from a traditional cartesian
/// graph with an origin in the center of the main window, which is used by
/// the transform.translation system, to a co-ordinate with the origin in
/// the top left of the main window, with +y going down the screen and +x
/// moving right in the screen.
pub fn cartesian_to_ui(x_in: f32, y_in: f32) -> (f32, f32) {
    let x_out = x_in - TILE_SIZE / 2.0 + (WINDOW_HEIGHT * RESOLUTION) / 2.0;
    let y_out = -1.0 * (y_in - TILE_SIZE / 2.0 - WINDOW_HEIGHT / 2.0);

    (x_out, y_out)
}
