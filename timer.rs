#![allow(non_snake_case, non_camel_case_types)]

use std::io::{Seek, Write};
use std::os::raw::*;

type DummyFunction = *const c_void;
type HRESULT = LONG;
type IID = GUID;
type LONG = c_long;
type UINT = c_uint;

#[repr(C)]
struct GUID {
    Data1: c_ulong,
    Data2: c_ushort,
    Data3: c_ushort,
    Data4: [c_char; 8],
}

#[repr(C)]
struct IDXGIFactoryVtbl {
    QueryInterface: DummyFunction,
    AddRef: DummyFunction,
    Release: DummyFunction,
    SetPrivateData: DummyFunction,
    SetPrivateDataInterface: DummyFunction,
    GetPrivateData: DummyFunction,
    GetParent: DummyFunction,
    EnumAdapters: extern "stdcall" fn(
        This: *const IDXGIFactory,
        Adapter: UINT,
        ppAdapter: *mut *const IDXGIAdapter,
    ) -> HRESULT,
    MakeWindowAssociation: DummyFunction,
    GetWindowAssociation: DummyFunction,
    CreateSwapChain: DummyFunction,
    CreateSoftwareAdapter: DummyFunction,
}

#[repr(C)]
struct IDXGIFactory {
    lpVtbl: *const IDXGIFactoryVtbl,
}

#[repr(C)]
struct IDXGIAdapterVtbl {
    QueryInterface: DummyFunction,
    AddRef: DummyFunction,
    Release: DummyFunction,
    SetPrivateData: DummyFunction,
    SetPrivateDataInterface: DummyFunction,
    GetPrivateData: DummyFunction,
    GetParent: DummyFunction,
    EnumOutputs: extern "stdcall" fn(
        This: *const IDXGIAdapter,
        Output: UINT,
        ppOutput: *mut *const IDXGIOutput,
    ) -> HRESULT,
    GetDesc: DummyFunction,
    CheckInterfaceSupport: DummyFunction,
}

#[repr(C)]
struct IDXGIAdapter {
    lpVtbl: *const IDXGIAdapterVtbl,
}

#[repr(C)]
struct IDXGIOutputVtbl {
    QueryInterface: DummyFunction,
    AddRef: DummyFunction,
    Release: DummyFunction,
    SetPrivateData: DummyFunction,
    SetPrivateDataInterface: DummyFunction,
    GetPrivateData: DummyFunction,
    GetParent: DummyFunction,
    GetDesc: DummyFunction,
    GetDisplayModeList: DummyFunction,
    FindClosestMatchingMode: DummyFunction,
    WaitForVBlank: extern "stdcall" fn(This: *const IDXGIOutput) -> HRESULT,
    TakeOwnership: DummyFunction,
    ReleaseOwnership: DummyFunction,
    GetGammaControlCapabilities: DummyFunction,
    SetGammaControl: DummyFunction,
    GetGammaControl: DummyFunction,
    SetDisplaySurface: DummyFunction,
    GetDisplaySurfaceData: DummyFunction,
    GetFrameStatistics: DummyFunction,
}

#[repr(C)]
struct IDXGIOutput {
    lpVtbl: *const IDXGIOutputVtbl,
}

const S_OK: HRESULT = 0;

#[link(name = "dxgi")]
extern "stdcall" {
    fn CreateDXGIFactory(riid: *const IID, ppFactory: *mut *const c_void) -> HRESULT;
}

fn error(msg: &'static str) -> ! {
    eprintln!("fatal error: {}", msg);
    println!("");
    println!("press enter key to close");
    std::io::stdin().read_line(&mut String::new()).unwrap();
    std::process::exit(1);
}

fn write_file(file: &mut std::fs::File, text: &str) {
    match file.set_len(0) {
        Ok(_) => (),
        Err(_) => eprintln!("warning: failed to clear file"),
    }
    match file.seek(std::io::SeekFrom::Start(0)) {
        Ok(_) => (),
        Err(_) => eprintln!("warning: failed to seek file"),
    }
    match file.write_all(text.as_bytes()) {
        Ok(_) => (),
        Err(_) => eprintln!("warning: failed to write file"),
    }
}

fn write_limit(file: &mut std::fs::File, limit: u32) -> String {
    let s = limit / 60;
    let m = s / 60;
    let fmt = format!("{:>02}:{:>02}", m, s % 60);
    write_file(file, &fmt);
    fmt
}

fn main() {
    let iid = IID {
        Data1: 0x7b7166ecu32,
        Data2: 0x21c7u16,
        Data3: 0x44aeu16,
        Data4: [
            0xb2u8 as c_char,
            0x1au8 as c_char,
            0xc9u8 as c_char,
            0xaeu8 as c_char,
            0x32u8 as c_char,
            0x1au8 as c_char,
            0xe3u8 as c_char,
            0x69u8 as c_char,
        ],
    };
    let mut factory = std::ptr::null();
    if unsafe { CreateDXGIFactory(&iid, &mut factory) } != S_OK {
        error("failed to create DXGIFactory");
    }
    let factory = factory as *const IDXGIFactory;

    let mut adapter = std::ptr::null();
    if unsafe { ((*(*factory).lpVtbl).EnumAdapters)(factory, 0, &mut adapter) } != S_OK {
        error("failed to create an instance of IDXGIAdapter");
    }

    let mut output = std::ptr::null();
    if unsafe { ((*(*adapter).lpVtbl).EnumOutputs)(adapter, 0, &mut output) } != S_OK {
        error("failed to create an instance of IDXGIOutput");
    }

    let mut file = match std::fs::File::create("cnt.txt") {
        Ok(n) => n,
        Err(_) => error("failed to open ./cnt.txt"),
    };

    println!("how many minutes?");
    let mut limit_str = String::new();
    std::io::stdin().read_line(&mut limit_str).unwrap();
    let mut limit = match limit_str.trim().parse::<u32>() {
        Ok(n) => n * 60 * 60,
        Err(_) => error("this number is invalid"),
    };

    write_limit(&mut file, limit);

    println!("press enter key to start!");
    std::io::stdin().read_line(&mut String::new()).unwrap();

    while limit > 0 {
        if (limit % 60) % 60 == 0 {
            let fmt = write_limit(&mut file, limit);
            println!("{}", fmt);
        }
        if unsafe { ((*(*output).lpVtbl).WaitForVBlank)(output) } != S_OK {
            error("something happened in IDXGIOutput::WaitForVBlank()");
        }
        limit -= 1;
    }

    write_file(&mut file, "FINAL RUN!");

    println!("press enter key to close");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
