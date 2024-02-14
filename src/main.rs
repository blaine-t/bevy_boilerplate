use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            // Register the Yarn Spinner plugin using its default settings, which will look for Yarn files in the "dialogue" folder.
            // If this app should support Wasm or Android, we cannot load files without specifying them, so use the following instead.
            // YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file("dialogue/hello_world.yarn")),
            YarnSpinnerPlugin::new(),
            // Initialize the bundled example UI
            ExampleYarnSpinnerDialogueViewPlugin::new(),
        ))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(
            Update,
            // Spawn the dialogue runner once the Yarn project has finished compiling
            spawn_dialogue_runner.run_if(resource_added::<YarnProject>()),
        )
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    // Create a dialogue runner from the project.
    let mut dialogue_runner = project.create_dialogue_runner();
    // Immediately start showing the dialogue to the player
    dialogue_runner.start_node("HelloWorld");
    commands.spawn(dialogue_runner);
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}
