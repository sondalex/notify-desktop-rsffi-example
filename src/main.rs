use std::ffi::CString;
use std::ptr;

const APP_NAME: &str = "notify-desktop-rustffi-example";

#[link(name = "glib-2.0")]
#[link(name = "notify")]
unsafe extern "C" {
    fn notify_init(name: *const i8) -> i32;
    fn notify_notification_new(
        summary: *const i8,
        body: *const i8,
        icon: *const i8,
    ) -> *mut std::os::raw::c_void;
    fn notify_notification_show(
        notification: *mut std::os::raw::c_void,
        error: *mut *mut std::os::raw::c_void,
    ) -> i32;
    fn notify_uninit();
    fn g_error_free(error: *mut std::os::raw::c_void);
}

#[derive(Debug)]
enum NotificationError {
    Initialization,
    Creation,
    Show,
}

impl std::error::Error for NotificationError {}

impl std::fmt::Display for NotificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationError::Initialization => write!(f, "Failed to initialize libnotify"),
            NotificationError::Show => write!(f, "Error showing notification"),
            NotificationError::Creation => write!(f, "Failed to create notification"),
        }
    }
}

fn notify(app_name: &str, summary: &str, body: &str) -> Result<(), NotificationError> {
    let app_name = CString::new(app_name).unwrap();
    unsafe {
        if notify_init(app_name.as_ptr()) == 0 {
            return Err(NotificationError::Initialization);
        }
    }

    let summary = CString::new(summary).unwrap();
    let body = CString::new(body).unwrap();
    let icon = CString::new("").unwrap();
    let notification =
        unsafe { notify_notification_new(summary.as_ptr(), body.as_ptr(), icon.as_ptr()) };

    if notification.is_null() {
        unsafe {
            notify_uninit();
        }
        return Err(NotificationError::Creation);
    }

    let mut error: *mut std::os::raw::c_void = ptr::null_mut();
    unsafe {
        if notify_notification_show(notification, &mut error) == 0 {
            if !error.is_null() {
                g_error_free(error);
                return Err(NotificationError::Show);
            } else {
                return Err(NotificationError::Show);
            }
        }
    }

    unsafe {
        notify_uninit();
    }
    Ok(())
}

fn main() {
    notify(APP_NAME, "An error occured", "The error is ...").unwrap()
}
