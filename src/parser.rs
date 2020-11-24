pub fn scsi_parse_inquiry(inq_buffer : &[u8]) {
    // Parse inquiry response
    let vendor_id = std::str::from_utf8(&inq_buffer[8..16]).unwrap().to_string();
    let model_number = std::str::from_utf8(&inq_buffer[16..32]).unwrap().to_string();
    let firmware_rev = std::str::from_utf8(&inq_buffer[32..36]).unwrap().to_string();
    let serial_number = std::str::from_utf8(&inq_buffer[36..44]).unwrap().to_string();
    // Print parsed response
    println!("======SCSI INQUIRY Command Response======");
    println!();
    println!("Vendor ID: {}", vendor_id);
    println!("Model Number: {}", model_number);
    println!("Firmware Revision: {}", firmware_rev);
    println!("Serial Number: {}", serial_number);
}