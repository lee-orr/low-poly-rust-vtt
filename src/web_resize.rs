use bevy::prelude::*;

pub struct FullViewportPlugin;

impl Plugin for FullViewportPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(target_family = "wasm")]
        {
            resize_functions::setup_plugin(app);
        }
        #[cfg(not(target_family = "wasm"))]
        {
            setup_empty_plugin(app);
        }
    }
}

fn setup_empty_plugin(_: &mut App) {
    println!("Resize Plugin Unneeded");
}

#[cfg(target_family = "wasm")]
mod resize_functions {
    use bevy::prelude::*;
    use std::sync::mpsc::{Receiver, Sender};
    use std::sync::Mutex;

    pub type OnResizeSender = Sender<()>;
    pub type OnResizeReceiver = Receiver<()>;

    pub fn setup_plugin(app: &mut App) {
        let channel = std::sync::mpsc::channel();
        let resize_sender: OnResizeSender = channel.0;
        let resize_receiver: OnResizeReceiver = channel.1;

        app.insert_resource(Mutex::new(resize_sender))
            .insert_resource(Mutex::new(resize_receiver))
            .add_startup_system(setup_viewport_resize_system)
            .add_system(viewport_resize_system);
    }
    pub fn get_viewport_size() -> (f32, f32) {
        let web_window = web_sys::window().expect("could not get window");
        let document_element = web_window
            .document()
            .expect("could not get document")
            .document_element()
            .expect("could not get document element");

        let width = document_element.client_width();
        let height = document_element.client_height();

        (width as f32, height as f32)
    }

    pub fn setup_viewport_resize_system(resize_sender: Res<Mutex<OnResizeSender>>) {
        let web_window = web_sys::window().expect("could not get window");
        let local_sender = resize_sender.lock().unwrap().clone();

        local_sender.send(()).unwrap();

        gloo_events::EventListener::new(&web_window, "resize", move |_event| {
            local_sender.send(()).unwrap();
        })
        .forget();
    }

    pub fn viewport_resize_system(
        mut windows: ResMut<Windows>,
        resize_receiver: Res<Mutex<OnResizeReceiver>>,
    ) {
        if resize_receiver.lock().unwrap().try_recv().is_ok() {
            if let Some(window) = windows.get_primary_mut() {
                let size = get_viewport_size();
                window.set_resolution(size.0, size.1);
            }
        }
    }
}
