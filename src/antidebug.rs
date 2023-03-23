use core::arch::asm;

use winapi::{um::{debugapi::{CheckRemoteDebuggerPresent, IsDebuggerPresent}, processthreadsapi::GetCurrentProcess, winnt::{EXCEPTION_POINTERS, LONG}, errhandlingapi::{RemoveVectoredExceptionHandler, AddVectoredExceptionHandler}}, vc::excpt::EXCEPTION_CONTINUE_EXECUTION};

#[inline(always)]
fn debugger_present() -> bool {
    unsafe { IsDebuggerPresent() != 0 }
}

#[inline(always)]
fn remote_debugger_present() -> bool {
    let mut present = 0;
    unsafe {
        CheckRemoteDebuggerPresent(GetCurrentProcess(), &mut present as *mut _) != 0 && present != 0
    }
}

#[inline(always)]
fn debugger_present_filter() -> bool {
    unsafe {
        let debugged: i32;

        unsafe extern "system" fn handler(info: *mut EXCEPTION_POINTERS) -> LONG {
            let ctx = (*info).ContextRecord;
            (*ctx).Eip += 3;
            return EXCEPTION_CONTINUE_EXECUTION;
        }
        let handler = AddVectoredExceptionHandler(1, Some(handler));
        asm!(
            "mov {0:e}, 1",
            "int 3", // 0xCC
            "jmp 2f", // 0xEB 0x05
            "mov {0:e}, 0", // 0xB8 0x00 0x00 0x00 0x00
            "2:",
            "nop", // 0x90
            out(reg) debugged,
        );
        RemoveVectoredExceptionHandler(handler);
        debugged == 1
    }
}

#[inline(always)]
pub fn bail_on_debugger() {
    if debugger_present() || remote_debugger_present() || debugger_present_filter() {
        crate::bail!("integrity violated");
    }
}