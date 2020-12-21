use i3ipc::{I3EventListener, event::inner::WindowChange};
use i3ipc::Subscription;
use i3ipc::event::Event;


fn main()
{
    let mut listener = I3EventListener::connect().unwrap();

    listener.subscribe(&[Subscription::Window]).unwrap();
    for event in listener.listen()
    {
        match event.unwrap()
        {
            Event::WindowEvent(e) =>
            {
                if e.change==WindowChange::Focus {
                    match e.container.name {
                        Some(value) => println!("{}",value),
                        None => (),
                    }
                }
            },
            _ =>unreachable!()
        }
    }

}
