pub mod rlib;

fn main() {
    println!("Welcome to the AIM Game!\n\n");
    println!("It's 1622 and some of the first fighting has broken out.");
    let mut command: rlib::Command;
    let mut world = rlib::World::new();
    let mut output: String;

    //
    // Main Loop
    //
    loop {
        command = rlib::get_input();
        output = world.update_state(&command);
        rlib::update_screen(output);

        if matches!(command, rlib::Command::Quit) {
            break;
        }
    }
    println!("I hope you had fun and learned a lot!");
}
