#![no_std]
use decoder_lib::*;

const BUF_SIZE: usize = 64 * 1024;

#[no_mangle]
pub unsafe extern "C" fn entry(data_ptr: *const u8, data_len: usize) {
    let mut writer = io::RawBuf::new();

    let tab_len = data_ptr.read() as usize;
    let symbols_end = tab_len * 8 + 1;
    let lens_end = symbols_end + tab_len;
    let symbols_ptr = data_ptr.add(1).cast::<u64>();
    let lens_ptr = data_ptr.add(symbols_end);
    let payload_ptr = data_ptr.add(lens_end);
    let payload_len = data_len - lens_end;
    let mut i = 0;

    loop {
        if payload_len - i < BUF_SIZE {
            break;
        }

        while writer.rem_capacity() >= 8 {
            let b = unsafe { *payload_ptr.add(i) };
            i += 1;

            if b == 255 {
                let b = unsafe { *payload_ptr.add(i) };
                i += 1;
                writer.unsafe_write_unaligned(b, 1);
            } else {
                let len = unsafe { *lens_ptr.add(b as usize) };
                let symbol = unsafe { symbols_ptr.add(b as usize).read() };
                writer.unsafe_write_unaligned(symbol, len as usize);
            }
        }

        writer.flush()
    }

    while i < payload_len {
        let b = unsafe { *payload_ptr.add(i) };
        i += 1;

        if b == 255 {
            let b = unsafe { *payload_ptr.add(i) };
            i += 1;
            writer.write_unaligned(b, 1);
        } else {
            let len = unsafe { *lens_ptr.add(b as usize) };
            let symbol = unsafe { symbols_ptr.add(b as usize).read() };
            writer.write_unaligned(symbol, len as usize);
        }
    }
}
