# BeagleChoo
BeagleChoo is an experimental model train controller using BeagleBone, more specifically a BeagleBone Blue. The idea is to
control a [Märklin 60116 Digital Connector Box](https://www.maerklin.de/en/products/details/article/60116/) through its 
front panel connector, make some of this capability available over the network and enable technical experimentation.

This is work in progress and far from complete. You have been warned!

## Background

I run a Märklin M-track model railroad, and would like digital control with it. The full-blown 
Märklin contollers are more expensive than the budget allows, so I am looking for a more
affordable option. Also, there is a lot of unrealized potential for STEM education in model
trains, particularly if you combine it with Open Source, modern embedded computing and 
building electronic hardware. This project tries to explore some of these areas.

### Previous work

From what I can determine, the original idea of building a controller out of a Märklin 60116 Digital Connector Box
and some external software came from Stefan Krauß [(Homepage in German)](http://www.skrauss.de/modellbahn/index.html),
who proposed "[Gleisbox als Zentrale](http://www.skrauss.de/modellbahn/gbox.html)" and wrote the first 
implementation [GBox2Eth](http://www.skrauss.de/modellbahn/gbox2eth_bin.html)
in 2010. Since then, this idea has become the basis of a lot of good work,
for example by [Michael Bernstein](http://mbernstein.de/modellbahn/can/index.htm) and
[Gerhard Bertelsmann](http://lnxpps.de/bpi/) ([github](https://github.com/GBert/railroad)).


### Märklin 60116 Digital Connector Box

![image of Marklin 60116](https://static.maerklin.de/damcontent/17/17/1717b7e0d1a0ffddbdaf73d730107b291524290466.jpg)

The [Märklin 60116 Digital Connector Box](https://www.maerklin.de/en/products/details/article/60116/) 
provides an attractive and cost-effective interface to a model railroad. It is designed 
to be used in conjunction with a [Märklin 60657 Mobile Station (also known as "MS2")](https://www.maerklin.de/en/products/details/article/60657).
The MS2 is a handheld controller that talks to the 60116 over a [CAN bus](https://en.wikipedia.org/wiki/CAN_bus) link, using a format that
[has been published by Märklin (in German)](https://www.maerklin.de/fileadmin/media/service/software-updates/cs2CAN-Protokoll-2_0.pdf).
The 60116 works as a track protocol processor. The protocol spec calls this a Gleisformat Prozessor,
and I will use GFP as an acronym in this document. The GFP translates CAN bus messages from the
system or GUI processor to one of the communication formats on the track. The formats supported by
the GFP include the MM2, mfx (sometimes also called M4) and DCC formats. Not all of these formats
are documented.

The physical connection to the CAN bus is via one of two available 10-pin mini-DIN connectors
on the front panel. Unfortunately these connectors are really hard to obtain, so I will take
some liberties and solder directly to the PCB.

### BeagleBone Blue
![Image of BeagleBone Blue](https://beagleboard.org/static/images/250px/beagle-blue-pck.png)

The [BeagleBone Blue](http://beagleboard.org/blue) is a member of the BeagleBone family of single board 
computers, and it is focused on robotics. It basically consists of a BeagleBone with an integrated
[Robotics Cape](https://beagleboard.org/capes#robotics). 
The hardware design is [is open-sourced on Github.](https://github.com/beagleboard/beaglebone-blue)
Software support for the rebotics-related
components is available via [librobotcontrol](https://github.com/StrawsonDesign/librobotcontrol).
Just like the BeagleBone Black, the board is based around the Octavo OSD3358-512M-BAS, 
which is a System-in-Package (SiP), which combines a 
Texas Instruments [Sitara ARM® Cortex®-A8 AM3358 Processor](http://www.ti.com/processors/sitara-arm/am335x-cortex-a8/overview.html),
512MB DDR3L RAM, 
a [TPS65217C Power Management IC (PMIC)](https://www.ti.com/product/TPS65217) 
and a [TL5209 Voltage Regulator](https://www.ti.com/product/TL5209).

Compared to other members of the BeagleBoard family, the BeagleBone Blue is unfortunately somewhat 
underdocumented, particularly in its differences in pinout and connectivity. The reason I am 
using the BeagleBone Blue for this project is that I had one laying around that wasn't used for
anything else at the time, and it conveniently has a CAN interface chip already available 
on the board. It also has a lot of hardware drivers and I/O brought out to connectors, and has
a lot of potential for other related experiments. But in principle, all of this could be adapted
to other (most likely cheaper) members of the BeagleBone family.

While the BeagleBone Blue would be capable enough to also include the GFP functionality,
I will make no attempt to do so. The Märklin 60116 is sold at a fair price for the functionality
provided; there would be very little point in replicating all these capabilities. 

## System Overview
The system is connected as follows: The GFP is connected to power (18 V DC, 2-3 A), the railroad track,
and to the BeagleBone Blue via the CAN connector.

## Implementation ideas

Here is a loose collection of ideas of where this project may go. Then again, it may not.
* [Configure BeagleBone for CAN bus](CAN/README.md)
* Write Wireshark disector for Märklin CS2 protocol [in Lua](https://wiki.wireshark.org/Lua)
* Port/adapt/integrate control software
* Integrate with [JMRI]()
* Rewrite control software in Rust, maybe?
* Build capture interface for tracing track protocols with Wireshark
* Write appropriate disectors
* Add user interface hardware (RPG, display, buttons and knobs)
* Drive servos for train gates, etc. 

## Resources
### BeagleBone
* [BeagleBone Black System Reference manual](https://github.com/beagleboard/beaglebone-black/wiki/System-Reference-Manual0). 
Please note that this is for the BeagleBone _Black, not Blue_, so there are important differences.
Regrettably, [there is no System Reference Manual for the BeagleBone Blue](https://github.com/beagleboard/beaglebone-blue/wiki/System-Reference-Manual).
* Probably outdated: [Device tree for robotics cape](https://github.com/StrawsonDesign/librobotcontrol/blob/master/device_tree/dtb-4.14-ti/am335x-boneblue.dts)
* The Amp Hour Podcast [Episode #378: An Interview with Jason Kridner and Robert Nelson](https://theamphour.com/378-an-interview-with-jason-kridner-and-robert-nelson/)
contains a lot of good information about BeagleBone.
* Device tree overlays are documented (hinted at) 
[here](https://elinux.org/Beagleboard:BeagleBoneBlack_Debian#U-Boot_Overlays)
and 
[here](https://github.com/RobertCNelson/bb.org-overlays).


### PRU Programming
* [Beagleboard.org PRU page](http://beagleboard.org/pru)
* [A recent overview of Programming PRU in C](https://www.element14.com/community/community/designcenter/single-board-computers/next-genbeaglebone/blog/2019/05/14/coding-for-the-beaglebone-pru-with-c-in-2019)
* [Another PRU C programming overview](http://www.righto.com/2016/09/how-to-run-c-programs-on-beaglebones.html)
* The "new way" of communication between the PRU and Linux is through RpMsg ([See Quick Start Guide](https://processors.wiki.ti.com/index.php/RPMsg_Quick_Start_Guide))
* [Debugging is possible with CodeComposer Studio](http://software-dl.ti.com/ccs/esd/documents/users_guide/ccs_debug-main.html#configuring-the-debugger), but this requires a JTAG port
* [Debugging presentation from TI](https://training.ti.com/system/files/docs/debug_pru_using_ccs_slides.pdf)
* [More debugging information](https://www.element14.com/community/community/designcenter/single-board-computers/next-genbeaglebone/blog/2019/05/18/debugging-the-beaglebone-pru-in-ccs)
* [Collection of PRU programs](https://processors.wiki.ti.com/index.php/PRU_Projects)
