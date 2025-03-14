use notify_rust::Notification;

pub(crate) fn send_notification(content: &String) {
    Notification::new()
        .summary("HyprDisplay")
        .body(content)
        .show().expect("Error sending notification");
}