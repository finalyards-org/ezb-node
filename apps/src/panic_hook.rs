// Panic hook
//
// We have three requirements for the panic messages:
//  - no restart loop
//  - line number should be shown
//  - proper message should be shown
//
// The default 'panic_abort' handler does 2/3, but keeps reset-looping. To prevent that, we
// attach a hook of our own.
//
// Note: Based on docs, this should work both on "abort" and "unwind" panic policies.
//
use std::panic::PanicHookInfo;
use esp_idf_svc::sys;

pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(panic_hook));
}

fn panic_hook(info: &PanicHookInfo) -> () /* !*/ {
    // ROM 'print' is slightly safer to use than 'log::error!'; both should work.
    {
        let mut buf = [0u8; 512];
        let mut w = BufWriter { buf: &mut buf, pos: 0 };

        let _ = write!(w, "🛑PANIC: {}\n\n\0", info);  // "panicked at {file:line:column}"

        unsafe {
            sys::esp_rom_printf(w.buf.as_ptr() as *const _);
        }
        // ESP-IDF function that prints mere memory addresses; espflash should convert those to
        // 'file:line:column' references.
        unsafe {
            sys::esp_backtrace_print(100);
        }
    }

    // Do not reboot
    loop {
        unsafe { sys::vTaskDelay(1000) };
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
