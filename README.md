# GSeq

## TODO

### POSSIBLE EFFECTS
* replication of the object (on the music)
* from one object to another
* Play with the lights

## Usage

### udev rules

To access the FTDI USB device as a regular user on Linux you need to update the udev rules.

Create a file called `/etc/udev/rules.d/99-ftdi.rules` with:

```shell
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6001", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6010", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6011", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6014", MODE="0666"
SUBSYSTEM=="usb", ATTRS{idVendor}=="0403", ATTRS{idProduct}=="6015", MODE="0666"
```

### Unload VCP driver

In Linux, the VCP driver and D2XX driver are incompatible with each other. When a FTDI device is
plugged in, the VCP driver must be unloaded before a D2XX application can be run. Use the
remove module (rmmod) command to do this:

```shell
sudo rmmod ftdi_sio
sudo rmmod usbserial
```

When the FTDI device is power cycled or reset the VCP driver will be reloaded. The rmmod process
must be repeated each time this occurs. It is possible to write a simple script that unloads the VCP
driver before running the D2XX application.
