# SAU-driver-for-TockOS
I created a low level driver for using the Armv8 Cortex-M Secure Attribution Unit with Tock operating system enabling so Arm TrustZone in Tock.

Implementing a low level driver in Rust for the Tock operating system for the use of Arm TrustZone in an embedded environment, the STM32L5 microcontrollers family that uses the Arm-v8 Cortex-M core.

The novelty is that Tock does not support natively the use of Arm TrustZone technology, so I created a low level driver to control the Secure Attribution Unit (SAU), a critical part for the use of TrustZone in Arm for Tock.

The Arm TrustZone for Cortex-M is a security extension for Armv8-M architecture that is optimized for ultra-low power embedded applications. It enables software security domains that restrict access to secure memory and I/O only for trusted software. Arm TrustZone technology for Armv8-M divide hardware into two sections: the secure and the non-secure sections. The division between the two sections is memory-mapped, so there will be addresses associated with both the secure and non-secure parts. Importantly, all resources of a microcontroller, including Flash memory, SRAM, peripherals, interrupts, and others, are allocated either to the secure or the non-secure section. The assignment of a resource to one of the two sections can be determined by a combination of hardware, like the SAU, and software and affects how that resource accesses other resources: a non-secure resource can only access other non-secure resources, while a secure resource can access all resources. 
