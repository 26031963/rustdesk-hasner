use librustdesk::ui;

fn main() {
    let mut args: Vec<String> = std::env::args().skip(1).collect();
    ui::start(&mut args);
}


