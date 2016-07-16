// utils.rs - useful bits and pieces
#[macro_use]
pub mod utils {
	// this macro is borrowed from rust-sdl2_ttf/demo.rs
	#[macro_export]
	macro_rules! rect(
		($x:expr, $y:expr, $w:expr, $h:expr) => (
			Rect::new($x as i32, $y as i32, $w as u32, $h as u32)
		)
	);

	#[macro_export]
	macro_rules! point(
		($x:expr, $y:expr) => (
			Point::new($x as i32, $y as i32)
		)
	);
}
