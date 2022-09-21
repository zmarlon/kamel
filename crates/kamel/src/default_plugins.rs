use kamel_bevy::{
    app::{PluginGroup, PluginGroupBuilder},
    asset::AssetPlugin,
    core::CorePlugin,
    input::InputPlugin,
    log::LogPlugin,
    window::WindowPlugin,
    winit::WinitPlugin
};

pub struct DefaultPlugins;

impl PluginGroup for DefaultPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(LogPlugin::default());
        group.add(CorePlugin::default());
        group.add(AssetPlugin::default());
        group.add(InputPlugin::default());
        group.add(WindowPlugin::default());
        group.add(WinitPlugin::default());
    }
}
