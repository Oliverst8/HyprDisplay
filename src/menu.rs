use hyprland::data::Monitor;
use rustofi::components::ItemList;
use rustofi::RustofiResult;
use crate::config::Config;
use crate::monitors::set_monitor_mode_by_string;


fn simple_app(config: &Config, primary_monitor: &Monitor, secondary_monitor: &Monitor) -> RustofiResult {
    let menu_items = vec![
        "mirror".to_string(),
        "extend_right".to_string(),
        "extend_left".to_string(),
    ];

    // Create a closure that captures the variables we need
    let config_clone = config.clone(); // You'll need to implement Clone for Config
    let primary_clone = primary_monitor.clone(); // These likely implement Clone already
    let secondary_clone = secondary_monitor.clone();

    let callback = move |s: &String| {
        println!("Clicked on item: {}", s);
        set_monitor_mode_by_string(config_clone.clone(), s, &primary_clone, &secondary_clone);
        RustofiResult::Success
    };

    ItemList::new(menu_items, Box::new(callback)).display("Select a monitor mode".to_string())
}

pub(crate) fn show_menu(config: &Config, primary_monitor: &Monitor, secondary_monitor: &Monitor) {
    loop {
        match simple_app(config, primary_monitor, secondary_monitor) {
            RustofiResult::Error => break,
            RustofiResult::Exit => break,
            RustofiResult::Cancel => break,
            RustofiResult::Blank => break,
            RustofiResult::Success => break,
            _ => {}
        }
    }
}