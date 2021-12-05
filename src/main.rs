use i3ipc::event::Event::WindowEvent;
use i3ipc::{
    event::{inner::WindowChange, Event, WindowEventInfo},
    I3EventListener,
};
use i3ipc::{MessageError, Subscription};

fn needs_to_update(info: &WindowEventInfo) -> bool {
    info.change == WindowChange::Focus
        || (info.change == WindowChange::Title && info.container.focused)
}

fn update(info: &WindowEventInfo) {
    if let Some(name) = &info.container.name {
        println!("{}", name)
    }
}

fn handle_events(event_type: Event) {
    if let WindowEvent(info) = event_type {
        if needs_to_update(&info) {
            update(&info)
        }
    }
}

fn ignore_errors(error: MessageError) {
    eprintln!("Error! {}", error.to_string())
}

fn main() {
    let mut listener = I3EventListener::connect().unwrap();

    listener.subscribe(&[Subscription::Window]).unwrap();
    for event in listener.listen() {
        match event {
            Ok(event_type) => handle_events(event_type),
            Err(err) => ignore_errors(err), //Maybe if i will ignore it, it will go away
        }
    }
}
