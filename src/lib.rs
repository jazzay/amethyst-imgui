pub extern crate imgui;
pub extern crate amethyst;
pub extern crate gfx;
#[macro_use] pub extern crate glsl_layout;
pub extern crate imgui_gfx_renderer;

#[macro_use] pub mod make_pass;

use amethyst::{
	ecs::shred::FetchMut,
	core::{cgmath},
	renderer::{
		error::Result,
		pipe::{
			pass::{Pass, PassData},
			Effect,
			NewEffect,
		},
		Encoder,
		Mesh,
		PosTex,
		Resources,
		VertexFormat,
	},
};
use gfx::{memory::Typed, preset::blend, pso::buffer::ElemStride, state::ColorMask};
use gfx::traits::Factory;
use glsl_layout::{vec2, vec4, Uniform};
use imgui::{FontGlyphRange, FrameSize, ImFontConfig, ImGui, ImVec4};
use imgui_gfx_renderer::{Renderer as ImguiRenderer, Shaders};

pub const VERT_SRC: &[u8] = include_bytes!("shaders/vertex.glsl");
pub const FRAG_SRC: &[u8] = include_bytes!("shaders/frag.glsl");

#[derive(Copy, Clone, Debug, Uniform)]
#[allow(dead_code)] // This is used by the shaders
#[repr(C)]
pub struct VertexArgs {
	pub proj_vec: vec4,
	pub coord: vec2,
	pub dimension: vec2,
}

pub struct ImguiState {
	pub imgui: ImGui,
	pub mouse_state: MouseState,
	pub size: (u16, u16),
}

#[derive(Copy, Clone, PartialEq, Debug, Default)]
pub struct MouseState {
	pub pos: (i32, i32),
	pub pressed: (bool, bool, bool),
	pub wheel: f32,
}

pub const FONT_THING: &[u8; 1745240] = include_bytes!("../mplus-1p-regular.ttf");

pub fn handle_imgui_events(resources: &amethyst::ecs::Resources, event: &amethyst::renderer::Event) {
	use amethyst::{
		renderer::{
			ElementState,
			Event,
			MouseButton,
			VirtualKeyCode as VK,
			WindowEvent::{self, ReceivedCharacter},
		},
		winit::{MouseScrollDelta, TouchPhase},
	};

	let mut imgui_state: Option<FetchMut<'_, Option<ImguiState>>> = resources.try_fetch_mut::<Option<ImguiState>>();
	let imgui_state: &mut Option<ImguiState> = match imgui_state {
		Some(ref mut x) => x,
		_ => return,
	};
	let imgui_state: &mut ImguiState = match imgui_state {
		Some(ref mut x) => x,
		_ => return,
	};

	let imgui = &mut imgui_state.imgui;
	let mouse_state = &mut imgui_state.mouse_state;

	if let Event::WindowEvent { event, .. } = event {
		match event {
			WindowEvent::KeyboardInput { input, .. } => {
				let pressed = input.state == ElementState::Pressed;
				match input.virtual_keycode {
					Some(VK::Tab) => imgui.set_key(0, pressed),
					Some(VK::Left) => imgui.set_key(1, pressed),
					Some(VK::Right) => imgui.set_key(2, pressed),
					Some(VK::Up) => imgui.set_key(3, pressed),
					Some(VK::Down) => imgui.set_key(4, pressed),
					Some(VK::PageUp) => imgui.set_key(5, pressed),
					Some(VK::PageDown) => imgui.set_key(6, pressed),
					Some(VK::Home) => imgui.set_key(7, pressed),
					Some(VK::End) => imgui.set_key(8, pressed),
					Some(VK::Delete) => imgui.set_key(9, pressed),
					Some(VK::Back) => imgui.set_key(10, pressed),
					Some(VK::Return) => imgui.set_key(11, pressed),
					Some(VK::Escape) => imgui.set_key(12, pressed),
					Some(VK::A) => imgui.set_key(13, pressed),
					Some(VK::C) => imgui.set_key(14, pressed),
					Some(VK::V) => imgui.set_key(15, pressed),
					Some(VK::X) => imgui.set_key(16, pressed),
					Some(VK::Y) => imgui.set_key(17, pressed),
					Some(VK::Z) => imgui.set_key(18, pressed),
					Some(VK::LControl) | Some(VK::RControl) => imgui.set_key_ctrl(pressed),
					Some(VK::LShift) | Some(VK::RShift) => imgui.set_key_shift(pressed),
					Some(VK::LAlt) | Some(VK::RAlt) => imgui.set_key_alt(pressed),
					Some(VK::LWin) | Some(VK::RWin) => imgui.set_key_super(pressed),
					_ => {},
				}
			},
			WindowEvent::CursorMoved { position: pos, .. } => {
				mouse_state.pos = (pos.0 as i32, pos.1 as i32);
			},
			WindowEvent::MouseInput { state, button, .. } => match button {
				MouseButton::Left => mouse_state.pressed.0 = *state == ElementState::Pressed,
				MouseButton::Right => mouse_state.pressed.1 = *state == ElementState::Pressed,
				MouseButton::Middle => mouse_state.pressed.2 = *state == ElementState::Pressed,
				_ => {},
			},
			WindowEvent::MouseWheel {
				delta: MouseScrollDelta::LineDelta(_, y),
				phase: TouchPhase::Moved,
				..
			} | WindowEvent::MouseWheel {
				delta: MouseScrollDelta::PixelDelta(_, y),
				phase: TouchPhase::Moved,
				..
			} => mouse_state.wheel = *y,
			ReceivedCharacter(c) => imgui.add_input_character(*c),
			_ => (),
		}
	}

	imgui.set_mouse_pos(mouse_state.pos.0 as f32, mouse_state.pos.1 as f32);
	imgui.set_mouse_down([mouse_state.pressed.0, mouse_state.pressed.1, mouse_state.pressed.2, false, false]);
	imgui.set_mouse_wheel(mouse_state.wheel);
	mouse_state.wheel = 0.0;
}
