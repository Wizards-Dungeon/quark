use core::ptr::null_mut;
use core::{ffi::*, mem};

use alloc::ffi::CString;

use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::LPVOID;
use winapi::um::fileapi::{CreateDirectoryA, CreateFileA, WriteFile};
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
use winapi::um::processthreadsapi::GetCurrentProcess;
use winapi::um::processthreadsapi::OpenProcessToken;
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::TokenElevation;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::TOKEN_ELEVATION;
use winapi::um::winnt::TOKEN_QUERY;
use winapi::um::winreg::{RegGetValueA, HKEY_LOCAL_MACHINE, RRF_RT_REG_SZ, RRF_SUBKEY_WOW6464KEY};

use crate::antidebug::bail_on_debugger;
use crate::v_strings;

#[macro_export]
macro_rules! c {
    ($s:literal) => {{
        ::core::concat!($s, "\0").as_ptr() as *const _
    }};
}

#[macro_export]
macro_rules! cformat {
    ($($arg:tt)*) => {
        ::alloc::ffi::CString::new(::alloc::format!("{}", ::alloc::fmt::format(::core::format_args!($($arg)*)))).unwrap_unchecked()
    };
}

#[macro_export]
macro_rules! bail {
    ($arg:tt) => {{
        #[allow(unused_unsafe)]
        unsafe {
            ::libc::printf(crate::v_strings::v_activation_failure()(), $crate::c!($arg));
            ::winapi::um::synchapi::Sleep(0xFFFFFFFF);
            ::core::hint::unreachable_unchecked();
        }
    }};
}

pub unsafe fn cleanup(slmgr: &CStr) {
    let slmgr = slmgr.to_str().unwrap_unchecked();
    libc::system(cformat!("cscript.exe //nologo {} -upk>nul 2>nul", slmgr).as_ptr());
    libc::system(cformat!("cscript.exe //nologo {} -cpky>nul 2>nul", slmgr).as_ptr());
    libc::system(cformat!("cscript.exe //nologo {} -ckms>nul 2>nul", slmgr).as_ptr());
}

pub unsafe fn create_ticket() {
    bail_on_debugger();

    // const TICKET: &[u8] = br#"<genuineAuthorization xmlns="http://www.microsoft.com/DRM/SL/GenuineAuthorization/1.0"><genuineProperties origin="sppclient"><properties>SessionId=TwBTAE0AYQBqAG8AcgBWAGUAcgBzAGkAbwBuAD0AMAA7AEcAVgBMAEsARQB4AHAAPQAyADAAMwA4AC0AMAAxAC0AMQA5AFQAMAAzADoAMQA0ADoAMAA3AFoAOwAAAA==;TimeStampClient=0-0-0</properties><signatures><signature name="" method="rsa-sha256" key="BgIAAACkAABSU0ExAAgAAAEAAQARq+V11k+dvHMCaLWVCaSbeQNlOdWTLkkl0hdMh5V3YhLU2R4h0Jd+7k7qfZ4aIo4ussduwGgmyDRikj5L2R77GG2ciHk4i8siK8qg7frOU0KT5rEks3qVj38C3dS1wS6D67shBFrxPlOEP8+JlelgP7Gxmwdao7NF4LXZ3+KdbJ//9jkmN8iAOP0N2XzW0/cJp9P1q6hE7eeqc/3Qn3zMr0q1Dx7vstN98oV17hNYCwumOxxS1rH+3n7ap2JKRSelo8Jvi214jZLBL+hOtYaGpxs7zIL3ofpoaYy5g7pc/DaTvyfpJho5634jK7dXVFMpzJZMn9w0F/3rkquk0Amm">WLbJIWmcesN6v2jP1bLNDG3HcyPlgu/gEmmrmBhDwG9cVDjvCATGnh7JRL7i5rfvlUk68fpZhZ58Oai7LcIFIyWmo4vfEzcfFNS+MRs/WPC2qATZ+hIU3eSzk8tt8A/QrTtgSFZ6T8QOXMVD0IBR9WX9x6owXrkD5Z9y4gOt8ZocZP2fuQXgd2iRD83mZ7cJExHgmsVXoyVbXD7h35o2sAFEM8HoytQm9gdtPeD9o4kwtxdHxn0N9wk4h8uMgepT7nU4pRfxr5U54efMLbqu500e546X5Fv88wPALH4yEwp7ZVGk4ftCADdqAnDvclRtjxG/p9WQnLBhBIjStdyUOQ==</signature></signatures></genuineProperties></genuineAuthorization>"#;
    // const TICKET: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?><genuineAuthorization xmlns="http://www.microsoft.com/DRM/SL/GenuineAuthorization/1.0"><version>1.0</version><genuineProperties origin="sppclient"><properties>OA3xOriginalProductId=;OA3xOriginalProductKey=;SessionId=TwBTAE0AYQBqAG8AcgBWAGUAcgBzAGkAbwBuAD0ANQA7AE8AUwBNAGkAbgBvAHIAVgBlAHIAcwBpAG8AbgA9ADEAOwBPAFMAUABsAGEAdABmAG8AcgBtAEkAZAA9ADIAOwBQAFAAPQAwADsARwBWAEwASwBFAHgAcAA9ADIAMAAzADgALQAwADEALQAxADkAVAAwADMAOgAxADQAOgAwADcAWgA7AEQAbwB3AG4AbABlAHYAZQBsAEcAZQBuAHUAaQBuAGUAUwB0AGEAdABlAD0AMQA7AAAA;TimeStampClient=2022-10-11T12:00:00Z</properties><signatures><signature name="clientLockboxKey" method="rsa-sha256">C52iGEoH+1VqzI6kEAqOhUyrWuEObnivzaVjyef8WqItVYd/xGDTZZ3bkxAI9hTpobPFNJyJx6a3uriXq3HVd7mlXfSUK9ydeoUdG4eqMeLwkxeb6jQWJzLOz41rFVSMtBL0e+ycCATebTaXS4uvFYaDHDdPw2lKY8ADj3MLgsA=</signature></signatures></genuineProperties></genuineAuthorization>"#;
    let program_data = CStr::from_ptr(libc::getenv(c!("PROGRAMDATA")));

    libc::printf(
        c!("[2/4] Using TicketPath=%s\\Microsoft\\Windows\\ClipSVC\\GenuineTicket\n"),
        program_data.as_ptr(),
    );

    CreateDirectoryA(
        cformat!(
            "{}\\Microsoft\\Windows\\ClipSVC\\GenuineTicket",
            program_data.to_str().unwrap_unchecked()
        )
        .as_ptr(),
        null_mut(),
    );

    let handle = CreateFileA(
        cformat!(
            "{}\\Microsoft\\Windows\\ClipSVC\\GenuineTicket\\GenuineTicket.xml",
            program_data.to_str().unwrap_unchecked()
        )
        .as_ptr(),
        0x40000000,
        0x00000001,
        null_mut(),
        0x1,
        0x00000080,
        null_mut(),
    );

    if handle == INVALID_HANDLE_VALUE {
        bail!("couldn't create GenuineTicket.xml")
    }

    WriteFile(
        handle,
        v_strings::v_activation_ticket()() as *const _,
        899,
        null_mut(),
        null_mut(),
    );

    CloseHandle(handle);
}

pub unsafe fn get_edition() -> CString {
    bail_on_debugger();

    let mut buffer_sz: DWORD = 0;
    if RegGetValueA(
        HKEY_LOCAL_MACHINE,
        c!("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion"),
        c!("EditionID"),
        RRF_RT_REG_SZ | RRF_SUBKEY_WOW6464KEY,
        null_mut(),
        null_mut(),
        &mut buffer_sz as *mut _,
    ) != 0
    {
        bail!("Edition is unreadable");
    }
    let mut buffer = vec![0u8; buffer_sz as usize];
    if RegGetValueA(
        HKEY_LOCAL_MACHINE,
        c!("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion"),
        c!("EditionID"),
        RRF_RT_REG_SZ | RRF_SUBKEY_WOW6464KEY,
        null_mut(),
        buffer.as_mut_ptr() as *mut _,
        &mut buffer_sz as *mut _,
    ) != 0
    {
        bail!("Edition is unreadable");
    }
    buffer.truncate(buffer_sz as usize);
    CString::from_vec_with_nul_unchecked(buffer)
}

pub unsafe fn set_pk(slmgr: &CStr) {
    bail_on_debugger();

    let edition = get_edition();
    let gvlk = crate::editions::get_gvlk();
    libc::printf(
        c!("[1/4] Using EditionID=%s\n      Using ProductKey=%s\n"),
        edition.as_ptr(),
        gvlk.as_ptr(),
    );

    let exit_code = libc::system(
        cformat!(
            "cscript.exe //nologo {} -ipk {}>nul 2>nul",
            slmgr.to_str().unwrap_unchecked(),
            gvlk.to_str().unwrap_unchecked()
        )
        .as_ptr(),
    );
    if exit_code != 0 {
        cleanup(slmgr);
        bail!("Installing product key failed");
    }
}

pub unsafe fn install_license(windir: &CStr, slmgr: &CStr) {
    bail_on_debugger();

    libc::printf(c!("[3/4] Installing license...\n"));
    let exit_code = libc::system(
        cformat!(
            "{0}\\SysWOW64\\cmd.exe /c {0}\\sysnative\\cmd.exe /c ClipUp.exe -v -o>nul 2>nul",
            windir.to_str().unwrap_unchecked()
        )
        .as_ptr(),
    );
    if exit_code != 0 {
        cleanup(slmgr);
        bail!("Installing product license failed");
    }
}

pub unsafe fn show_activation(slmgr: &CStr) {
    libc::printf(c!("[4/4] Checking activation status...\n"));
    libc::system(
        cformat!(
            "cscript.exe //nologo {} -xpr",
            slmgr.to_str().unwrap_unchecked(),
        )
        .as_ptr(),
    );
}
