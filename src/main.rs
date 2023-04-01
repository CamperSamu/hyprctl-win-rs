mod def_not_bsod;
use def_not_bsod::green_screen_of_life;

fn main() {
    println!(
        "usage: hyprctl [(opt)none] [none] [(opt)none]\n\ncommands:\n    ????\nflags:\n    ????"
    );

    std::thread::sleep(std::time::Duration::from_secs(1));
    green_screen_of_life();
}
