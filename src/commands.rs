use crate::parser;

use std::io;
use std::ptr;
use std::path::Path;
use std::ffi::c_void;

extern "C" {
    fn ioctl(fd: i32, request: u32, ...) -> i32;
}

const SENSE_LEN : u8 = 32;
const SG_GET_VERSION_NUM : u32 = 0x2282;
const SG_IO: u32 = 0x2285;
const SG_DXFER_NONE : i32 = -1;
const SG_DXFER_FROM_DEV : i32 = -3;
const SG_INFO_OK : u32 = 0x0;
const SG_INFO_OK_MASK : u32 =  0x1;

#[repr(C)]
struct sg_io_hdr {
    interface_id:       i32,            // [i] guard field. Current implementations only accept "(int)'S'"
    dxfer_direction:    i32,            // [i] direction of data transfer
    cmd_len:            u8,             // [i] limits command length to 16 bytes
    mx_sb_len:          u8,             // [i] maximum number of bytes of sense data that the driver can output via the sbp pointer
    iovec_count:        u16,            // [i] if not sg driver and greater than zero then the SG_IO ioctl fails with errno set to EOPNOTSUPP
    dxfer_len:          u32,            // [i] number of bytes of data to transfer to or from the device
    dxferp:             *mut c_void,    // [i], [*io] pointer to (user space) data to transfer to (if reading from device) or transfer from (if writing to device)
    cmdp:               *const u8,      // [i], [*i] pointer to SCSI command
    sbp:                *mut u8,        // [i], [*o] pointer to user data area where no more than max_sb_len bytes of sense data from the device will be written if the SCSI status is CHECK CONDITION
    timeout:            u32,            // [i] time in milliseconds that the SCSI mid-level will wait for a response
    flags:              u32,            // [i] block layer SG_IO ioctl ignores this field; the sg driver uses it to request special services like direct IO or mmap-ed transfers
    pack_id:            i32,            // [i->o] unused (for user space program tag)
    usr_ptr:            *mut c_void,    // [i->o] unused (for user space pointer tag)
    status:             u8,             // [o] SCSI command status, zero implies GOOD
    masked_status:      u8,             // [o] logically: masked_status == ((status & 0x3e) >> 1). Old linux SCSI subsystem usage, deprecated.
    msg_status:         u8,             // [o] SCSI parallel interface (SPI) message status (very old, deprecated)
    sb_len_wr:          u8,             // [o] actual length of sense data (in bytes) output via sbp pointer
    host_status:        u16,            // [o] error reported by the initiator (port). These are the "DID_*" error codes in scsi.h
    driver_status:      u16,            // [o] bit mask: error and suggestion reported by the low level driver (LLD)
    resid:              i32,            // [o] (dxfer_len - number_of_bytes_actually_transferred)
    duration:           u32,            // [o] number of milliseconds that elapsed between when the command was injected into the SCSI mid level and the corresponding "done" callback was invoked
    info:               u32,            // [o] bit mask indicating what was done (or not) and whether any error was detected
}

impl Default for sg_io_hdr {
    fn default() -> Self {
        sg_io_hdr {
            interface_id:       'S' as i32,
            dxfer_direction:    0,
            cmd_len:            0,
            mx_sb_len:          0,
            iovec_count:        0,
            dxfer_len:          0,
            dxferp:             ptr::null_mut(),
            cmdp:               ptr::null_mut(),
            sbp:                ptr::null_mut(),
            timeout:            30000,
            flags:              0,
            pack_id:            0,
            usr_ptr:            ptr::null_mut(),
            status:             0,
            masked_status:      0,
            msg_status:         0,
            sb_len_wr:          0,
            host_status:        0,
            driver_status:      0,
            resid:              0,
            duration:           0,
            info:               0,
        }
    }
}

impl sg_io_hdr {
    fn new() -> Self {
        Default::default()
    }
}

pub fn scsi_send_get_version_number(sg_fd : i32, drive_path : &Path) {
    let k: i32 = 0;
    unsafe {
        if ioctl(sg_fd, SG_GET_VERSION_NUM, &k) < 0 || k < 30000 {
            panic!("{} is not an sg device, or old sg driver: {}", drive_path.display(), io::Error::last_os_error());
        }
    }
}

pub fn scsi_send_inquiry(sg_fd : i32, drive_path : &Path) {
    // Constants
    const INQ_REPLY_LEN : u8 = 96;
    const INQ_CMD_CODE : u8 = 0x12;
    const INQ_CMD_LEN : u8 = 6;
    // Set up variables
    let inq_cmd_blk : [u8; INQ_CMD_LEN as usize] = [INQ_CMD_CODE, 0, 0, 0, INQ_REPLY_LEN, 0];
    let mut inq_buffer : [u8; INQ_REPLY_LEN as usize] = [0; INQ_REPLY_LEN as usize];
    let mut sense_buffer : [u8; SENSE_LEN as usize] = [0; SENSE_LEN as usize];
    // Prepare command
    let io_hdr = sg_io_hdr {
        interface_id:       'S' as i32,
        dxfer_direction:    SG_DXFER_FROM_DEV,
        cmd_len:            inq_cmd_blk.len() as u8,
        mx_sb_len:          sense_buffer.len() as u8,
        dxfer_len:          INQ_REPLY_LEN as u32,
        dxferp:             inq_buffer.as_mut_ptr() as *mut c_void,
        cmdp:               inq_cmd_blk.as_ptr(),
        sbp:                sense_buffer.as_mut_ptr(),
        timeout:            20000,
        ..Default::default()
    };
    // Send command
    unsafe {
        if ioctl(sg_fd, SG_IO, &io_hdr) < 0 {
            panic!("{}: Inquiry SG_IO ioctl error: {}", drive_path.display(), io::Error::last_os_error());
        }
    }
    // Error processing
    if (io_hdr.info & SG_INFO_OK_MASK) != SG_INFO_OK {
        if io_hdr.sb_len_wr > 0 {
            print!("INQUIRY sense data: ");
            for k in 0..io_hdr.sb_len_wr {
                if (k > 0) && (0 == (k % 10)) {
                    print!("\n");
                }
                print!("{:X} ", sense_buffer[k as usize]);
            }
            print!("\n");
        }
        if io_hdr.masked_status != 0 {
            print!("INQUIRY SCSI status = {:X}\n", io_hdr.status);
        }
        if io_hdr.host_status != 0 {
            print!("INQUIRY host_status = {:X}\n", io_hdr.host_status);
        }
        if io_hdr.driver_status != 0 {
            print!("INQUIRY driver_status = {:X}\n", io_hdr.driver_status);
        }
    }
    // Assume inquiry response is present
    else {
        parser::scsi_parse_inquiry(&inq_buffer);
        println!();
        println!("INQUIRY Duration = {} ms", io_hdr.duration);
        println!("INQUIRY Resid (non-zero means shortened DMA transfer) = {}", io_hdr.resid);
    }
}