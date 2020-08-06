
fn main() {
    println!("Starting up..");

    let sdl = sdl2::init()
                .expect("Failed to init SDL");
    
    let video_subsystem = sdl.video()
                .expect("Failed to init Video subsystem");
    
    let window = video_subsystem
        .window("Game", 900, 700)
        .resizable()
        .build()
        .expect("Failed to create window");
    
    let mut event_pump = sdl.event_pump()
        .expect("Event pump failed");
        
    'main: loop {
        // Handle user input here
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        // Render window contents here

        std::thread::sleep(std::time::Duration::from_millis(17));
    }

}
