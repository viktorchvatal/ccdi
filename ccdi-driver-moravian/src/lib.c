static int last_camera_id = -1;

void enumerate_usb_callback(int camera_id) {
    last_camera_id = camera_id;
}

int get_last_camera_id() {
    return last_camera_id;
}

void reset_last_camera_id() {
    last_camera_id = -1;
}