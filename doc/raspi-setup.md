## Set up a Wifi hotspot

Enable network manager
 - `sudo raspi-config`
 - `6 Advanced options`
 - `AA Network config`
 - `Network Manager`
 - reboot

Create wifi hotspot
 - network
 - advanced options
 - create wireless hotspot
 - enter name and password

Enable hotspot auto start
 - network
 - advanced options
 - edit connections
 - (network name)
 - options
 - general
 - connect automatically with priority: 0

## Set up default init target

Textual multi user:
`sudo systemctl set-default multi-user.target`

Graphical:
`sudo systemctl set-default graphical-target`

## Disable automatic login
 - `sudo raspi-config`
 - `System`
 - `S5 boot / auto login`
 - `B1 console`

## Enable SSH
 - `sudo raspi-config`
 - `interface options`
 - `SSH`
