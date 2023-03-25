use alloc::ffi::CString;

use crate::{antidebug::bail_on_debugger, c, v_strings};

pub unsafe fn get_gvlk() -> CString {
    bail_on_debugger();
    
    let f = libc::popen(v_strings::v_pwsh_gvlk()(), c!("r"));
    let mut buffer = vec![0u8; 30];
    libc::fgets(buffer.as_mut_ptr() as *mut _, 30, f);
    if libc::pclose(f) != 0 { crate::bail!("Getting product key failed"); }
    CString::from_vec_with_nul_unchecked(buffer)
}