use super::trace;
use super::Particle;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use egui::Slider;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin).add_system(ui_system);
    }
}

fn ui_system(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut uniforms: ResMut<trace::Uniforms>,
    particle_query: Query<Entity, With<Particle>>,
) {
    egui::Window::new("Settings")
        .anchor(egui::Align2::RIGHT_TOP, [-5.0, 5.0])
        .show(egui_context.ctx_mut(), |ui| {
            ui.collapsing("Rendering", |ui| {
                ui.checkbox(&mut uniforms.show_ray_steps, "Show ray steps");
                ui.checkbox(&mut uniforms.indirect_lighting, "Indirect lighting");
                ui.checkbox(&mut uniforms.shadows, "Shadows");
                ui.add(
                    Slider::new(&mut uniforms.accumulation_frames, 1.0..=100.0)
                        .text("Accumulation frames"),
                );
                ui.checkbox(&mut uniforms.freeze, "Freeze");
            });
            ui.collapsing("Compute", |ui| {
                ui.checkbox(&mut uniforms.enable_compute, "Enable compute");
                if ui.button("spawn particles").clicked() {
                    for _ in 0..10000 {
                        commands.spawn_bundle((
                            Transform::from_xyz(0.0, 0.0, 0.0),
                            Particle { material: 41 },
                        ));
                    }
                }
                if ui.button("destroy particles").clicked() {
                    for particle in particle_query.iter() {
                        commands.entity(particle).despawn();
                    }
                }
                ui.label(format!("Particle count: {}", particle_query.iter().count()));
            });
            ui.checkbox(&mut uniforms.misc_bool, "Misc bool");
            ui.add(Slider::new(&mut uniforms.misc_float, 0.0..=1.0).text("Misc float"));
        });
}
