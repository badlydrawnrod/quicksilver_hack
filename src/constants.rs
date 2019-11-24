pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;

pub const VIRTUAL_WIDTH: u32 = WINDOW_WIDTH;
pub const VIRTUAL_HEIGHT: u32 = WINDOW_HEIGHT;

pub const FIXED_UPDATE_HZ: f64 = 60.0;
pub const FIXED_UPDATE_INTERVAL_S: f64 = 1.0 / FIXED_UPDATE_HZ;
pub const FIXED_UPDATE_INTERVAL_MS: f64 = 1000.0 / FIXED_UPDATE_HZ;

pub const MAX_DRAW_RATE_HZ: f64 = 144.0;
pub const DRAW_INTERVAL_MS: f64 = 1000.0 / MAX_DRAW_RATE_HZ;

pub const VSYNC: bool = false;

pub const DRAW_DIAGS: bool = false;
