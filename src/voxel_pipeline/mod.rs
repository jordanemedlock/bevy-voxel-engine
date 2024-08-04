use crate::VoxelSubGraph;

use self::{
    attachments::{AttachmentsNode, AttachmentsPlugin},
    compute::{
        animation::AnimationNode, automata::AutomataNode, clear::ClearNode, physics::PhysicsNode,
        rebuild::RebuildNode, ComputeResourcesPlugin,
    },
    trace::{TraceNode, TracePlugin},
    voxel_world::VoxelWorldPlugin,
    voxelization::VoxelizationPlugin,
};
use bevy::{
    core_pipeline::{fxaa::FxaaNode, tonemapping::TonemappingNode, upscaling::UpscalingNode},
    prelude::*,
    render::{
        extract_resource::{ExtractResource, ExtractResourcePlugin}, graph::CameraDriverLabel, render_graph::{RenderGraph, RenderLabel, ViewNodeRunner}, RenderApp
    },
    ui::UiPassNode,
};

pub mod attachments;
pub mod compute;
pub mod trace;
pub mod voxel_world;
pub mod voxelization;

#[derive(RenderLabel, Debug, Hash, Eq, PartialEq, Clone)]
enum VoxelRenderLabel {
    Attachments,
    Trace,
    Tonemapping,
    FXAA,
    UI,
    Upscaling,
    Rebuild,
    Physics,
    Clear,
    Automata,
    Animation
}


pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RenderGraphSettings::default())
            .add_plugins(ExtractResourcePlugin::<RenderGraphSettings>::default())
            .add_plugins(AttachmentsPlugin)
            .add_plugins(VoxelWorldPlugin)
            .add_plugins(TracePlugin)
            .add_plugins(VoxelizationPlugin)
            .add_plugins(ComputeResourcesPlugin);
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        let render_world = &mut render_app.world;

        // Build voxel render graph
        let mut voxel_graph = RenderGraph::default();

        // Voxel render graph
        let attachments = AttachmentsNode::new(render_world);
        let trace = TraceNode::new(render_world);
        //let bloom = BloomNode::new(&mut render_app.world);
        let tonemapping = TonemappingNode::from_world(render_world);
        let fxaa = FxaaNode::from_world(render_world);
        let ui = UiPassNode::new(render_world);
        let upscaling = UpscalingNode::from_world(render_world);

        voxel_graph.add_node(VoxelRenderLabel::Attachments, attachments);
        voxel_graph.add_node(VoxelRenderLabel::Trace, trace);
        voxel_graph.add_node(
            VoxelRenderLabel::Tonemapping,
            ViewNodeRunner::new(tonemapping, render_world),
        );
        voxel_graph.add_node(VoxelRenderLabel::FXAA, ViewNodeRunner::new(fxaa, render_world));
        voxel_graph.add_node(VoxelRenderLabel::UI, ui);
        voxel_graph.add_node(VoxelRenderLabel::Upscaling, ViewNodeRunner::new(upscaling, render_world));

        voxel_graph.add_node_edge(VoxelRenderLabel::Trace, VoxelRenderLabel::Tonemapping);
        voxel_graph.add_node_edge(VoxelRenderLabel::Tonemapping, VoxelRenderLabel::FXAA);
        voxel_graph.add_node_edge(VoxelRenderLabel::FXAA, VoxelRenderLabel::UI);
        voxel_graph.add_node_edge(VoxelRenderLabel::UI, VoxelRenderLabel::Upscaling);

        voxel_graph.add_slot_edge(VoxelRenderLabel::Attachments, "normal", VoxelRenderLabel::Trace, "normal");
        voxel_graph.add_slot_edge(VoxelRenderLabel::Attachments, "position", VoxelRenderLabel::Trace, "position");

        // Voxel render graph compute
        voxel_graph.add_node(VoxelRenderLabel::Rebuild, RebuildNode);
        voxel_graph.add_node(VoxelRenderLabel::Physics, PhysicsNode);

        voxel_graph.add_node_edge(VoxelRenderLabel::Rebuild, VoxelRenderLabel::Physics);
        voxel_graph.add_node_edge(VoxelRenderLabel::Physics, VoxelRenderLabel::Trace);

        // Main graph compute
        let mut graph = render_world.resource_mut::<RenderGraph>();

        graph.add_node(VoxelRenderLabel::Clear, ClearNode);
        graph.add_node(VoxelRenderLabel::Automata, AutomataNode);
        graph.add_node(VoxelRenderLabel::Animation, AnimationNode);

        graph.add_node_edge(VoxelRenderLabel::Clear, VoxelRenderLabel::Automata);
        graph.add_node_edge(VoxelRenderLabel::Automata, VoxelRenderLabel::Animation);
        graph.add_node_edge(VoxelRenderLabel::Animation, CameraDriverLabel);

        // Insert the voxel graph into the main render graph
        graph.add_sub_graph(VoxelSubGraph, voxel_graph);
    }
}

#[derive(Resource, Clone, ExtractResource)]
pub struct RenderGraphSettings {
    pub clear: bool,
    pub automata: bool,
    pub animation: bool,
    pub voxelization: bool,
    pub rebuild: bool,
    pub physics: bool,
    pub trace: bool,
}

impl Default for RenderGraphSettings {
    fn default() -> Self {
        Self {
            clear: true,
            automata: true,
            animation: true,
            voxelization: true,
            rebuild: true,
            physics: true,
            trace: true,
        }
    }
}
