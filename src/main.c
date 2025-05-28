#include <libnotify/notification.h>
#include <libnotify/notify.h>
#include <stdio.h>

#define PROJECT_NAME "notify"

int main(int argc, char **argv) {
    notify_init("notify-desktop-rustffi-example");
    NotifyNotification *notification =
        notify_notification_new("wg-waybar", "An error occured", "The error is ...");
    GError *error = NULL;

    notify_notification_show(notification, &error);
    if (error != NULL) {
        fprintf(stderr, "Error showing notification: %s\n", error->message);
        g_error_free(error);
    }
    notify_uninit();
    return 0;
}
