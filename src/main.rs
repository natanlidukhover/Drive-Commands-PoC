use std::env;
use std::fs::File;
use std::path::Path;
use std::ffi::c_void;
use std::os::unix::io::{AsRawFd, RawFd};

const INQ_REPLY_LEN: i32 = 96;
const INQ_CMD_CODE: i32 = 0x12;
const INQ_CMD_LEN: i32 = 6;

extern "C" {
	fn ioctl(fd: i32, request: u32, ...) -> i32;
}

#[repr(C)]
struct sg_io_hdr {
	interface_id:		i32,			// [i] 'S' for SCSI generic (required)
	dxfer_direction:	i32,			// [i] data transfer direction
	cmd_len:			u8,		// [i] SCSI command length ( <= 16 bytes)
	mx_sb_len:			u8,		// [i] max length to write to sbp
	iovec_count:		u16,		// [i] 0 implies no scatter gather
	dxfer_len:			u32,			// [i] byte count of data transfer
	dxferp:				*mut c_void,	// [i], [*io] points to data transfer memory or scatter gather list
	cmdp:				*const u8,	// [i], [*i] points to command to perform
	sbp:				*mut u8,	// [i], [*o] points to sense_buffer memory
	timeout:			u32,			// [i] MAX_UINT->no timeout (unit: millisec)
	flags:				u32,			// [i] 0 -> default, see SG_FLAG...
	pack_id:			i32,			// [i->o] unused internally (normally)
	usr_ptr:			*mut c_void,	// [i->o] unused internally
	status:				u8,		// [o] scsi status
	masked_status:		u8,		// [o] shifted, masked scsi status
	msg_status:			u8,		// [o] messaging level data (optional)
	sb_len_wr:			u8,		// [o] byte count actually written to sbp
	host_status:		u16,		// [o] errors from host adapter
	driver_status:		u16,		// [o] errors from software driver
	resid:				i32,			// [o] dxfer_len - actual_transferred
	duration:			u32,			// [o] time taken by cmd (unit: millisec)
	info:				u32,			// [o] auxiliary information
}

fn main() {
	std::process::exit(match run_app() {
        Ok(_) => {
			println!("GREAT SUCCESS!");
			0
		},
        Err(err) => {
            eprintln!("ERROR :: {:?}", err);
            1
        }
    });
}

fn run_app() -> Result<(), ()> {
	// Collect command-line arguments
	let args: Vec<String> = env::args().collect();
	let drive_path = Path::new(&args[1]);
	// Set up variables
	let k: i32;
	let inq_cmd_blk : [u8; INQ_CMD_LEN as usize] = [INQ_CMD_CODE as u8, 0, 0, 0, INQ_REPLY_LEN as u8, 0];
	let inq_buffer : [u8; INQ_REPLY_LEN as usize];
	let sense_buffer : [u8; 32];
	// Open the file given for desired drive
    let mut sg = match File::open(&drive_path) {
        Err(err) => panic!("Could not open {}: {}", drive_path.display(), err),
        Ok(file) => file,
	};
	let sg_fd : i32 = sg.as_raw_fd();
	// Check if sg device
	match ioctl(sg_fd, SG_GET_VERSION_NUM, &k) {
		Err(err) => panic!("{} is not an sg device, or old sg driver", &args[1]),
        Ok(file) => file,
	}
	if k < 30000 {
        panic!("{} is not an sg device, or old sg driver", &args[1]);
        Err()
    }
	// Prepare inquiry command
	struct sg_io_hdr io_hdr;
	
    memset(&io_hdr, 0, sizeof(struct sg_io_hdr));
    io_hdr.interface_id = 'S';
    io_hdr.cmd_len = sizeof(turCmbBlk);
    io_hdr.mx_sb_len = sizeof(sense_b);
    io_hdr.dxfer_direction = SG_DXFER_NONE;
    io_hdr.cmdp = turCmbBlk;
    io_hdr.sbp = sense_b;
    io_hdr.timeout = DEF_TIMEOUT;

    if (ioctl(fd, SG_IO, &io_hdr) < 0) {
	}
	Ok(())
}