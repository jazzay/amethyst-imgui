# Usage:
1. In your `main.rs`, do
```rust
// could be whatever, but must take a lifetime argument and impl SystemData
type ImguiPassData<'a> = (
	Option<Read<'a, amethyst::renderer::ScreenDimensions>>,
	Option<Read<'a, amethyst::core::timing::Time>>,
);

// ui is &mut imgui::Ui, data is &ImguiPassData
// the closure arguments are defined by the shitty macro, it's not _really_ a closure
define_pass!(ImguiPass, |ui, data: ImguiPassData| {
	if let (Some(dimensions), Some(time)) = data {
		ui.window(im_str!("TEST WINDOW WOOO")).build(|| {
			ui.text(im_str!("{}x{}, {}", dimensions.width(), dimensions.height(), time.delta_seconds()));
		});
		ui.show_demo_window(&mut true);
	}
});
```
2. In your `Stage`, do
```rust
	.with_pass(ImguiPass::default())
```
3. Add this to your `handle_event`:
```rust
	amethyst_imgui::handle_imgui_events(std::borrow::Borrow::<Resources>::borrow(data.world), &event);
```
