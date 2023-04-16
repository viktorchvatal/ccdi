# Installing dependencies

`apt install libusb-1.0.0-dev llvm-dev libclang-dev clang libcfitsio-dev`

`cargo install --locked trunk`

# Enabling permissions for moravian camera

In case of the following error:
`[gxccd] error: my_libusb_open(): libusb_open failed: -3, LIBUSB_ERROR_ACCESS`

create text file `/etc/udev/rules.d/98-moravian.rules`

And insert content
```
# Moravian camera
ATTRS{idVendor}=="1347", ATTRS{idProduct}=="0ca0", MODE:="0666"
```

Reload UDEV rules and reconnect the device

 * `sudo udevadm control --reload-rules`
 * OR `sudo systemctl restart udev`

# Running web service and web client

`ccdi-web-service$ cargo run --release`
`ccdi-web-client$ trunk serve --release --open`

# Notes

Tool to view FITS images

QFitsView - `sudo apt install qfitsview`

