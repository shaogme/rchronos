use crate::Result;

use windows::Win32::System::Registry::{
    HKEY, HKEY_LOCAL_MACHINE, KEY_QUERY_VALUE, KEY_SET_VALUE, REG_SZ, REG_VALUE_TYPE, RegCloseKey,
    RegOpenKeyExW, RegQueryValueExW, RegSetValueExW,
};
use windows::core::PCWSTR;

pub fn apply_windows_time_policy(disable_win32_time: bool) -> Result<()> {
    let mut key = HKEY::default();
    let sam_desired = KEY_SET_VALUE | KEY_QUERY_VALUE;

    let subkey_w: Vec<u16> = "SYSTEM\\CurrentControlSet\\Services\\W32Time\\Parameters"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_w.as_ptr()),
            None,
            sam_desired,
            &mut key,
        )
        .ok()
        .map_err(|e| crate::Error::Registry(format!("RegOpenKeyExW 失败: {e}")))?;
    }

    let value = if disable_win32_time { "NoSync" } else { "NTP" };
    let data_w: Vec<u16> = value.encode_utf16().chain(std::iter::once(0)).collect();
    let bytes =
        unsafe { std::slice::from_raw_parts(data_w.as_ptr() as *const u8, data_w.len() * 2) };

    let value_name_w: Vec<u16> = "Type".encode_utf16().chain(std::iter::once(0)).collect();

    unsafe {
        if let Err(e) = RegSetValueExW(
            key,
            PCWSTR(value_name_w.as_ptr()),
            None,
            REG_SZ,
            Some(bytes),
        )
        .ok()
        {
            let _ = RegCloseKey(key);
            return Err(crate::Error::Registry(format!("RegSetValueExW 失败: {e}")));
        }

        RegCloseKey(key)
            .ok()
            .map_err(|e| crate::Error::Registry(format!("RegCloseKey 失败: {e}")))?;
    }

    Ok(())
}

pub fn query_windows_time_policy() -> Result<String> {
    let mut key = HKEY::default();
    let sam_desired = KEY_QUERY_VALUE;

    let subkey_w: Vec<u16> = "SYSTEM\\CurrentControlSet\\Services\\W32Time\\Parameters"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_w.as_ptr()),
            None,
            sam_desired,
            &mut key,
        )
        .ok()
        .map_err(|e| crate::Error::Registry(format!("RegOpenKeyExW 失败: {e}")))?;
    }

    let value_name_w: Vec<u16> = "Type".encode_utf16().chain(std::iter::once(0)).collect();
    let mut val_type = REG_VALUE_TYPE::default();
    let mut buf = vec![0u8; 256];
    let mut cb_data = buf.len() as u32;

    unsafe {
        let res = RegQueryValueExW(
            key,
            PCWSTR(value_name_w.as_ptr()),
            None,
            Some(&mut val_type),
            Some(buf.as_mut_ptr()),
            Some(&mut cb_data),
        );
        let _ = RegCloseKey(key);
        res.ok()
            .map_err(|e| crate::Error::Registry(format!("RegQueryValueExW 失败: {e}")))?;
    }

    // 转换为 String
    let wide_slice =
        unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u16, (cb_data as usize) / 2) };
    // 移除末尾的零
    let len = wide_slice
        .iter()
        .position(|&x| x == 0)
        .unwrap_or(wide_slice.len());
    let value = String::from_utf16(&wide_slice[..len])
        .map_err(|e| crate::Error::Registry(format!("UTF-16 转换失败: {e}")))?;

    Ok(value)
}
