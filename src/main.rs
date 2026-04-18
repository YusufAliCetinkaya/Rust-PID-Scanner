use std::mem::{size_of, zeroed};
use windows_sys::Win32::Foundation::{CloseHandle, INVALID_HANDLE_VALUE, GetLastError};
use windows_sys::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW,
    PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};

fn main() {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);

        if snapshot == INVALID_HANDLE_VALUE {
            println!("Snapshot alınamadı. Hata kodu: {}", GetLastError());
            return;
        }

        let mut entry: PROCESSENTRY32W = zeroed();
        entry.dwSize = size_of::<PROCESSENTRY32W>() as u32;

        if Process32FirstW(snapshot, &mut entry) == 0 {
            println!("Process32First başarısız. Hata: {}", GetLastError());
            CloseHandle(snapshot);
            return;
        }

        loop {
            let name_len = entry
                .szExeFile
                .iter()
                .position(|&c| c == 0)
                .unwrap_or(entry.szExeFile.len());

            let process_name = String::from_utf16_lossy(&entry.szExeFile[..name_len]);

            println!("PID: {:<8} | Adı: {}", entry.th32ProcessID, process_name);

            if Process32NextW(snapshot, &mut entry) == 0 {
                break;
            }
        }

        CloseHandle(snapshot);
    }
}