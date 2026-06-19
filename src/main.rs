mod daemon;
fn main() {
    println!("GNOME Auto-Dark Mode by isaachhk02");
    daemon::start_daemon();
}
