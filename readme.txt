# Installing bindgen dependencies

`apt install llvm-dev libclang-dev clang`

# Enabling permissions for moravian camera

[gxccd] error: my_libusb_open(): libusb_open failed: -3, LIBUSB_ERROR_ACCESS

$ lsusb | grep Moravian
Bus 006 Device 003: ID 1347:0ca0 Moravian Instruments

create text file /etc/udev/rules.d/98-moravian.rules

# Moravian camera
ATTRS{idVendor}=="1347", ATTRS{idProduct}=="0ca0", MODE:="0666"

sudo udevadm control --reload-rules
OR
sudo systemctl restart udev

