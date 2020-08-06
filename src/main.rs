
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
    
    loop {
        std::thread::sleep(std::time::Duration::from_millis(17));
    }

}
