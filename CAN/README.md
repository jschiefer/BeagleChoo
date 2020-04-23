# BeagleBone CAN Bus Configuration

## Hardware setup

Here is an excerpt of the 
[BeagleBone Blue schematic](https://github.com/beagleboard/beaglebone-blue/blob/master/BeagleBone_Blue_sch.pdf), 
which shows the CAN circuitry:

![CAN transceiver on BeagleBone Blue schematic](CAN_sch.png)

As we can see, the [CAN bus transceiver](https://www.ti.com/product/TCAN1051HV) is 
connected to pins `E18` (TX) and `E17` (RX) of the OSD3358 SiP. There is also another
signal called `DCAN1_SILENT`, which is connected to pin 8 of the CAN transceiver.
This signal goes to pin M16 (`S`") of the OSD3358 SiP. The `S` pin is pulled to 
ground internally. Putting it high actives the "silent mode" of the 
transceiver, which prevents communication from the `TXD` pin to the CAN bus.
The receiver remains active; communication from the CAN bus still gets passed
to the `RXD` output pin.

## Pin configuration and device tree overlays
Maybe it's just me, but I find the method of configuring the BeagleBone 
device tree with slots, capes, overlays etc. hard to understand. Much of the
documentation on the web seems to be outdated and referring to methods previously
in use, but abandoned. As far as I can tell, as of this writing (April 2020),
the state of the art appears to be the
[U-Boot Partitioning Layout 2.0](https://elinux.org/Beagleboard:U-boot_partitioning_layout_2.0), 
based on discussions in 2017, partially documented 
[here](https://web.archive.org/web/20170618061856/https://wiki.linaro.org/Platform/DeviceTreeConsolidation).
The hardware is configured by U-Boot, rather than the kernel. The above documents
define the order in which directories are traversed, in order to find a valid
device tree for the hardware.

Device tree overlays are documented (hinted at) 
[here](https://elinux.org/Beagleboard:BeagleBoneBlack_Debian#U-Boot_Overlays)
and 
[here](https://github.com/RobertCNelson/bb.org-overlays).

[Device tree in the 4.19 kernel](https://github.com/torvalds/linux/blob/v4.19/arch/arm/boot/dts/am335x-boneblue.dts)
[Latest version](https://github.com/torvalds/linux/blob/master/arch/arm/boot/dts/am335x-boneblue.dts)

Let's try to make this work. I am starting from a standard Debian Buster "flasher" 
image [distributed by the BeagleBone foundation](http://beagleboard.org/latest-images),
called `AM3358 Debian 10.3 2020-04-06 4GB eMMC IoT Flasher`. I installed this 
to the eMMC, and after booting, `uname -a` reports
```
Linux bbb 4.19.94-ti-r42 #1buster SMP PREEMPT Tue Mar 31 19:38:29 UTC 2020 armv7l GNU/Linux
```
The `/etc/debian_version` is 10.3.

This U-Boot Partitioning Layout 2.0 is configured via `/boot/uEnv.txt`, so 
let's take a look.

BeagleBone Blue does not support the universal cape, 
so comment out this line in  `/boot/uEnv.txt` as follows:
```
#enable_uboot_cape_universal=1
```

This used to be broken, but apparently a fix has been in the kernel since
version 4.16.

## CAN network configuration

```
sudo ip link set can0 up type can bitrate 250000
```

https://developer.ridgerun.com/wiki/index.php/How_to_configure_and_use_CAN_bus
https://electronics.stackexchange.com/questions/195416/beaglebone-black-can-bus-setup
https://www.thomas-wedemeyer.de/beaglebone-canbus-python.html




## CAN testing

