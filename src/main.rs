
fn main() {
    println!("Starting up..");

    let sdl = sdl2::init().unwrap();
               
    let video = sdl.video().unwrap();
            
    let gl_attr = video.gl_attr();
    
    let window = video
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl_ctx = window.gl_create_context().unwrap();
    
    // let ctx 
    
    let mut event_pump = sdl.event_pump().unwrap();        
        
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
