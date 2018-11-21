//! A basic example of ImGui integration.

extern crate amethyst;

use amethyst::{
    input::{is_close_requested, is_key_down},
    core::{frame_limiter::FrameRateLimitStrategy},
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
    winit::VirtualKeyCode,
};

use std::time::Duration;

struct Example;

impl<'a, 'b> SimpleState<'a, 'b> for Example {

    fn handle_event(&mut self, data: StateData<GameData>, event: StateEvent) -> SimpleTrans<'a, 'b> {
        match &event {
            StateEvent::Window(event) => {
                amethyst_imgui::handle_imgui_events(data.world, &event);
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(_ui_event) => {
                Trans::None
            }
        }
    }

    fn update(&mut self, state_data: &mut StateData<GameData>) -> SimpleTrans<'a, 'b> {
        let StateData { world, data } = state_data;
        data.update(&world);

        let ui = amethyst_imgui::open_frame(world);
        if let Some(ui) = ui {
            ui.show_demo_window(&mut true);
        }

    	if let Some(ui) = ui { 
            amethyst_imgui::close_frame(ui);
        }

        Trans::None
    }

}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let path = format!(
        "{}/examples/demo/resources/display_config.ron",
        application_root_dir()
    );
    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new())
            .with_pass(amethyst_imgui::DrawUi::default())
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)))?;

    //let mut game = Application::new("./", Example, game_data)?;

    let mut game = Application::build("./", Example)?
        .with_frame_limit(FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)), 144)
        .build(game_data)?;
        
    game.run();

    Ok(())
}
