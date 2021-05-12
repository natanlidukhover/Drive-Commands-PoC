* Put Seagate copyright on all files. 
* Create a Documents folder to have architecture level documentation 
* Improve README with at least how to build and run type info. 
* Create a crate with the following structure
  * Transport (Top-level)
    * Operations (Common)
    * Utilities (Common)
    * Main (Common)
    * SCSI (Interface-level)
      * Windows (OS-level)
      * Linux
      * FreeBSD
    * ATA (Interface-level)
      * Windows (OS-level)
      * Linux
      * FreeBSD
    * USB (Interface-level)
      * Windows (OS-level)
      * Linux
      * FreeBSD
    * NVMe (Interface-level) 
      * Windows (OS-level)
      * Linux
      * FreeBSD
* At some point in the code, the interface should be automatically detected
  * Figure out if the drive is SCSI or ATA or anything else by sending an identify (or equivalent)
  * May differ by OS
* Follow the philosophy
  * You **DON'T** want to be everything to everyone 
  * For example, openSeaChest is all about portability, and its philosophy is
    * Use REALLY old compilers to be compatible (C99 in C++ C98) 
    * Don't include things you don't need right now 
    * Don't include third party dependencies when picking libraries 
    * Issuing commands to the drive is sacred 
    * How you open the drive handle has consequences in different OSs
  * What is our philosophy with Rust?
    * Figure it out and stick to it