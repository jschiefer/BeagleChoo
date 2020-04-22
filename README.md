# BeagleChoo
BeagleChoo is an experimental model train controller using BeagleBone, more specifically a BeagleBone Blue.


# Random notes
## PRU Processing

* [C programming overview](http://www.righto.com/2016/09/how-to-run-c-programs-on-beaglebones.html)
* Binaries are loaded from /lib/firmware
* Communication with Linux is through RpMsg ([See Quick Start Guide](https://processors.wiki.ti.com/index.php/RPMsg_Quick_Start_Guide))
* [Debugging is possible with CodeComposer Studio](http://software-dl.ti.com/ccs/esd/documents/users_guide/ccs_debug-main.html#configuring-the-debugger), but this requires a JTAG port
* [Debugging presentation from TI](https://training.ti.com/system/files/docs/debug_pru_using_ccs_slides.pdf)
* [More debuggin information](https://www.element14.com/community/community/designcenter/single-board-computers/next-genbeaglebone/blog/2019/05/18/debugging-the-beaglebone-pru-in-ccs)
* [Overview of Programming PRU in C](https://www.element14.com/community/community/designcenter/single-board-computers/next-genbeaglebone/blog/2019/05/14/coding-for-the-beaglebone-pru-with-c-in-2019)

## Beaglebone Blue References
* [U-Boot and device tree overlays](https://elinux.org/Beagleboard:BeagleBoneBlack_Debian#U-Boot_Overlays)
* [Beaglebone Blue repository](https://github.com/beagleboard/beaglebone-blue)
* Probably outdated: [Device tree for robotics cape](https://github.com/StrawsonDesign/librobotcontrol/blob/master/device_tree/dtb-4.14-ti/am335x-boneblue.dts)
