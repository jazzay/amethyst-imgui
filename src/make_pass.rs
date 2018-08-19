use super::*;

#[macro_export]
macro_rules! set_keys {
	($imgui:expr, $($key:ident => $id:expr),+$(,)*) => {
		$($imgui.set_imgui_key($crate::imgui::ImGuiKey::$key, $id);)+
	};
}

#[macro_export]
macro_rules! define_pass {
	($name:ident, |$ui:ident, $data:ident: $data_type:ident| $body:tt) => {
		type FormattedT = ($crate::gfx::format::R8_G8_B8_A8, $crate::gfx::format::Unorm);

		struct RendererThing {
			renderer: $crate::imgui_gfx_renderer::Renderer<$crate::amethyst::renderer::Resources>,
			texture: $crate::gfx::handle::Texture<$crate::amethyst::renderer::Resources, $crate::gfx::format::R8_G8_B8_A8>,
			shader_resource_view: $crate::gfx::handle::ShaderResourceView<$crate::amethyst::renderer::Resources, [f32; 4]>,
			mesh: $crate::amethyst::renderer::Mesh,
		}

		#[derive(Default)]
		struct $name {
			imgui: Option<$crate::imgui::ImGui>,
			renderer: Option<RendererThing>,
		}

		impl<'a> $crate::amethyst::renderer::pipe::pass::PassData<'a> for $name {
			type Data = (
				Option<shred::Read<'a, $crate::amethyst::renderer::ScreenDimensions>>,
				Option<shred::Read<'a, $crate::amethyst::core::timing::Time>>,
				shred::Write<'a, Option<$crate::ImguiState>>,
				$data_type<'a>,
			);
		}

		impl $crate::amethyst::renderer::pipe::pass::Pass for $name {
			fn compile(&mut self, mut effect: $crate::amethyst::renderer::pipe::NewEffect<'_>) -> $crate::amethyst::renderer::error::Result<$crate::amethyst::renderer::pipe::Effect> {
				use $crate::amethyst::{
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
				use $crate::gfx::{memory::Typed, preset::blend, pso::buffer::ElemStride, state::ColorMask};
				use $crate::gfx::traits::Factory;
				use $crate::glsl_layout::{vec2, vec4, Uniform};
				use $crate::imgui::{FontGlyphRange, FrameSize, ImFontConfig, ImGui, ImVec4};
				use $crate::imgui_gfx_renderer::{Renderer as ImguiRenderer, Shaders};

				let mut imgui = $crate::imgui::ImGui::init();
				{
					// Fix incorrect colors with sRGB framebuffer
					fn imgui_gamma_to_linear(col: $crate::imgui::ImVec4) -> $crate::imgui::ImVec4 {
						let x = col.x.powf(2.2);
						let y = col.y.powf(2.2);
						let z = col.z.powf(2.2);
						let w = 1.0 - (1.0 - col.w).powf(2.2);
						$crate::imgui::ImVec4::new(x, y, z, w)
					}

					let style = imgui.style_mut();
					for col in 0..style.colors.len() {
						style.colors[col] = imgui_gamma_to_linear(style.colors[col]);
					}
				}
				imgui.set_ini_filename(None);

				let font_size = 13.;

				let _ = imgui.fonts().add_font_with_config(
					$crate::FONT_THING,
					ImFontConfig::new()
						.oversample_h(1)
						.pixel_snap_h(true)
						.size_pixels(font_size)
						.rasterizer_multiply(1.75),
					&FontGlyphRange::japanese(),
				);

				let _ = imgui.fonts().add_default_font_with_config(
					$crate::imgui::ImFontConfig::new()
						.merge_mode(true)
						.oversample_h(1)
						.pixel_snap_h(true)
						.size_pixels(font_size),
				);

				set_keys![imgui,
					Tab => 0,
					LeftArrow => 1,
					RightArrow => 2,
					UpArrow => 3,
					DownArrow => 4,
					PageUp => 5,
					PageDown => 6,
					Home => 7,
					End => 8,
					Delete => 9,
					Backspace => 10,
					Enter => 11,
					Escape => 12,
					A => 13,
					C => 14,
					V => 15,
					X => 16,
					Y => 17,
					Z => 18,
				];

				let data = vec![
					$crate::amethyst::renderer::PosTex {
						position: [0., 1., 0.],
						tex_coord: [0., 0.],
					},
					$crate::amethyst::renderer::PosTex {
						position: [1., 1., 0.],
						tex_coord: [1., 0.],
					},
					$crate::amethyst::renderer::PosTex {
						position: [1., 0., 0.],
						tex_coord: [1., 1.],
					},
					$crate::amethyst::renderer::PosTex {
						position: [0., 1., 0.],
						tex_coord: [0., 0.],
					},
					$crate::amethyst::renderer::PosTex {
						position: [1., 0., 0.],
						tex_coord: [1., 1.],
					},
					$crate::amethyst::renderer::PosTex {
						position: [0., 0., 0.],
						tex_coord: [0., 1.],
					},
				];

				let (texture, shader_resource_view, target) = effect.factory.create_render_target::<FormattedT>(1024, 1024).unwrap();
				let renderer = $crate::imgui_gfx_renderer::Renderer::init(&mut imgui, effect.factory, $crate::imgui_gfx_renderer::Shaders::GlSl130, target).unwrap();
				self.renderer = Some(RendererThing {
					renderer,
					texture,
					shader_resource_view,
					mesh: $crate::amethyst::renderer::Mesh::build(data).build(&mut effect.factory)?,
				});
				self.imgui = Some(imgui);

				effect
					.simple($crate::VERT_SRC, $crate::FRAG_SRC)
					.with_raw_constant_buffer("VertexArgs", std::mem::size_of::<<$crate::VertexArgs as $crate::glsl_layout::Uniform>::Std140>(), 1)
					.with_raw_vertex_buffer($crate::amethyst::renderer::PosTex::ATTRIBUTES, $crate::amethyst::renderer::PosTex::size() as $crate::gfx::pso::buffer::ElemStride, 0)
					.with_texture("albedo")
					.with_blended_output("color", $crate::gfx::state::ColorMask::all(), $crate::gfx::preset::blend::ALPHA, None)
					.build()
			}

			fn apply<'ui, 'data: 'ui>(
				&'ui mut self,
				encoder: &mut $crate::amethyst::renderer::Encoder,
				effect: &mut $crate::amethyst::renderer::pipe::Effect,
				mut factory: $crate::amethyst::renderer::Factory,
				// (screen_dimensions, time, mut imgui_state, ui_data): <Self as $crate::amethyst::renderer::pipe::pass::PassData<'data>>::Data,
				data: <Self as $crate::amethyst::renderer::pipe::pass::PassData<'data>>::Data,
			) {
				use $crate::amethyst::{
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
				use $crate::gfx::{memory::Typed, preset::blend, pso::buffer::ElemStride, state::ColorMask};
				use $crate::gfx::traits::Factory;
				use $crate::glsl_layout::{vec2, vec4, Uniform};
				use $crate::imgui::{FontGlyphRange, FrameSize, ImFontConfig, ImGui, ImVec4};
				use $crate::imgui_gfx_renderer::{Renderer as ImguiRenderer, Shaders};

				fn run_ui($ui: &mut $crate::imgui::Ui, $data: &$data_type) $body

				let (screen_dimensions, time, mut imgui_state, ui_data) = data;

				let screen_dimensions = match screen_dimensions {
					Some(x) => x,
					None => return,
				};
				let time = match time {
					Some(x) => x,
					None => return,
				};

				let imgui_state = imgui_state.get_or_insert_with(|| $crate::ImguiState {
					imgui: self.imgui.take().unwrap(),
					mouse_state: $crate::MouseState::default(),
					size: (1024, 1024),
				});
				let imgui = &mut imgui_state.imgui;

				let (width, height) = (screen_dimensions.width(), screen_dimensions.height());
				let renderer_thing = self.renderer.as_mut().unwrap();

				let vertex_args = $crate::VertexArgs {
					proj_vec: $crate::amethyst::core::cgmath::vec4(2. / width, -2. / height, 0., 1.).into(),
					coord: [0., 0.].into(),
					dimension: [width, height].into(),
				};

				if imgui_state.size.0 != width as u16 || imgui_state.size.1 != height as u16 {
					let (texture, shader_resource_view, target) = factory.create_render_target::<FormattedT>(width as u16, height as u16).unwrap();
					renderer_thing.renderer.update_render_target(target);
					renderer_thing.shader_resource_view = shader_resource_view;
					renderer_thing.texture = texture;
				}

				encoder.clear(
					&factory
						.view_texture_as_render_target::<FormattedT>(&renderer_thing.texture, 0, None)
						.unwrap(),
					[0., 0., 0., 0.],
				);
				{
					let mut ui = imgui.frame($crate::imgui::FrameSize::new(f64::from(width), f64::from(height), 1.), time.delta_seconds());
					run_ui(&mut ui, &ui_data);
					renderer_thing.renderer.render(ui, &mut factory, encoder).unwrap();
				}

				{
					use $crate::gfx::texture::{FilterMethod, SamplerInfo, WrapMode};
					let sampler = factory.create_sampler(SamplerInfo::new(FilterMethod::Trilinear, WrapMode::Clamp));
					effect.data.samplers.push(sampler);
				}

				effect.update_constant_buffer("VertexArgs", &vertex_args.std140(), encoder);
				effect.data.textures.push(renderer_thing.shader_resource_view.raw().clone());
				effect
					.data
					.vertex_bufs
					.push(renderer_thing.mesh.buffer($crate::amethyst::renderer::PosTex::ATTRIBUTES).unwrap().clone());

				effect.draw(renderer_thing.mesh.slice(), encoder);

				effect.data.textures.clear();
				effect.data.samplers.clear();
			}
		}
	};
}
