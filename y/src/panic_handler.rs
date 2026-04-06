// Panic handler
//
// For Embassy, and maybe more generally, we *really* want our own panic handler.
//
//  - 'esp_idf_sys' feature "panic_handler" would only work in "abort" mode, and cause device
//    to reboot. We wish to have control over that.
//

//      - 'esp_println' doesn't fit with a 'ldproxy' project that's not 'std'
//      - 'esp_backtrace' did not work
//
// The solution below does print enough information in the case of a panic,
// to help solve the case.
//
// Expects:
//  - in '.cargo/config.toml': no "panic_abort" in '[unstable] build-std'
//  - in 'Cargo.toml': '[profile.release] panic = "unwind"'
//
// NOTE: We could also just use 'log::error!'. It does work.
//
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // ROM 'print' is slightly safer to use than 'log::error!'; both should work.
    {
        let mut buf = [0u8; 512];
        let mut w = BufWriter { buf: &mut buf, pos: 0 };

        let _ = write!(w, "PANIC: {}\n\0", info);

        unsafe {
            esp_idf_sys::esp_rom_printf(w.buf.as_ptr() as *const _);
        }
    }
    #[cfg(false)]
    {
        log::error!("*** PANIC ***\n{}", info);
    }

    // Do not reboot
    loop {
        unsafe { esp_idf_sys::vTaskDelay(1000) };
    }
}

use core::fmt::{self, Write};

struct BufWriter<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> Write for BufWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let space = self.buf.len().saturating_sub(self.pos);    // remaining bytes, if any
        let n = bytes.len().min(space); // truncate to 'buf'
        self.buf[self.pos..self.pos + n].copy_from_slice(&bytes[..n]);
        self.pos += n;
        Ok(())
    }
}
