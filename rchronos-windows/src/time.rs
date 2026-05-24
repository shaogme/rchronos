use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use chrono::{DateTime, Utc, Datelike, Timelike};
use crate::Result;

thread_local! {
    static SYSTEM_TIME_PRIVILEGE_GRANTED: AtomicBool = const { AtomicBool::new(false) };
}

use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, LUID, SYSTEMTIME},
    Security::{
        AdjustTokenPrivileges, LookupPrivilegeValueW, LUID_AND_ATTRIBUTES, SE_PRIVILEGE_ENABLED,
        TOKEN_PRIVILEGES,
    },
    System::{
        SystemInformation::{
            GetSystemTimeAdjustment, GetSystemTimeAdjustmentPrecise, SetSystemTime,
            SetSystemTimeAdjustment, SetSystemTimeAdjustmentPrecise,
        },
        Threading::{GetCurrentProcess, OpenProcessToken},
    },
};
use windows::core::{BOOL, PCWSTR};

fn ensure_system_time_privilege() -> Result<()> {
    let granted = SYSTEM_TIME_PRIVILEGE_GRANTED.with(|g| g.load(Ordering::Relaxed));
    if granted {
        return Ok(());
    }

    unsafe {
        let process = GetCurrentProcess();
        let mut token_handle = HANDLE::default();
        
        // TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY = 0x0020 | 0x0008 = 0x0028
        let desired_access = windows::Win32::Security::TOKEN_ACCESS_MASK(0x0020 | 0x0008);
        OpenProcessToken(process, desired_access, &mut token_handle)
            .map_err(|e| crate::Error::Driver(format!("OpenProcessToken 失败: {e}")))?;

        let mut luid = LUID::default();
        let name_w: Vec<u16> = "SeSystemtimePrivilege".encode_utf16().chain(std::iter::once(0)).collect();
        LookupPrivilegeValueW(None, PCWSTR(name_w.as_ptr()), &mut luid)
            .map_err(|e| crate::Error::Driver(format!("LookupPrivilegeValue 失败: {e}")))?;

        let token_privileges = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: SE_PRIVILEGE_ENABLED,
            }],
        };

        AdjustTokenPrivileges(token_handle, false, Some(&token_privileges), 0, None, None)
            .map_err(|e| crate::Error::Driver(format!("AdjustTokenPrivileges 失败: {e}")))?;

        CloseHandle(token_handle)
            .map_err(|e| crate::Error::Driver(format!("CloseHandle 失败: {e}")))?;
    }

    SYSTEM_TIME_PRIVILEGE_GRANTED.with(|g| g.store(true, Ordering::Relaxed));
    Ok(())
}

pub fn set_system_time_direct(time: DateTime<Utc>) -> Result<()> {
    ensure_system_time_privilege()?;
    let system_time = utc_to_system_time(time);
    unsafe {
        SetSystemTime(&system_time)
            .map_err(|e| crate::Error::Driver(e.to_string()))?;
    }
    Ok(())
}

pub fn slew_system_time(target: DateTime<Utc>) -> Result<&'static str> {
    ensure_system_time_privilege()?;

    let mut p_adj = 0u64;
    let mut p_inc = 0u64;
    let mut p_dis = BOOL(0);

    // 优先尝试精确渐调 API (Windows 10 2004+)
    let precise_supported = unsafe {
        GetSystemTimeAdjustmentPrecise(&mut p_adj, &mut p_inc, &mut p_dis).is_ok()
    };

    if precise_supported {
        let now = Utc::now();
        let diff_ms = (target - now).num_milliseconds();
        if diff_ms.abs() < 2 {
            return Ok("slew-skipped-small");
        }

        let slew_rate = 0.1;
        let increment = p_inc as f64;
        let adj_delta = increment * slew_rate;
        let new_adj = if diff_ms > 0 {
            increment + adj_delta
        } else {
            increment - adj_delta
        };

        let distance_100ns = diff_ms.abs() as f64 * 10000.0;
        let interrupts_needed = distance_100ns / adj_delta;
        let seconds_to_wait = (interrupts_needed * increment) / 10_000_000.0;

        unsafe {
            SetSystemTimeAdjustmentPrecise(new_adj.round() as u64, false)
                .map_err(|e| crate::Error::Driver(e.to_string()))?;
        }
        
        std::thread::sleep(Duration::from_secs_f64(seconds_to_wait));
        
        unsafe {
            SetSystemTimeAdjustmentPrecise(0, true)
                .map_err(|e| crate::Error::Driver(e.to_string()))?;
        }

        return Ok("slew-precise");
    }

    // 降级使用传统渐调 API
    let mut l_adj = 0u32;
    let mut l_inc = 0u32;
    let mut l_dis = BOOL(0);
    unsafe {
        GetSystemTimeAdjustment(&mut l_adj, &mut l_inc, &mut l_dis)
            .map_err(|e| crate::Error::Driver(e.to_string()))?;
    }

    let now = Utc::now();
    let diff_ms = (target - now).num_milliseconds();
    if diff_ms.abs() < 2 {
        return Ok("slew-skipped-small");
    }

    let slew_rate = 0.1;
    let increment = l_inc as f64;
    let adj_delta = increment * slew_rate;
    let new_adj = if diff_ms > 0 {
        increment + adj_delta
    } else {
        increment - adj_delta
    };

    let distance_100ns = diff_ms.abs() as f64 * 10000.0;
    let interrupts_needed = distance_100ns / adj_delta;
    let seconds_to_wait = (interrupts_needed * increment) / 10_000_000.0;

    unsafe {
        SetSystemTimeAdjustment(new_adj.round() as u32, false)
            .map_err(|e| crate::Error::Driver(e.to_string()))?;
    }
    
    std::thread::sleep(Duration::from_secs_f64(seconds_to_wait));
    
    unsafe {
        SetSystemTimeAdjustment(0, true)
            .map_err(|e| crate::Error::Driver(e.to_string()))?;
    }

    Ok("slew-legacy")
}

pub fn utc_to_system_time(time: DateTime<Utc>) -> SYSTEMTIME {
    SYSTEMTIME {
        wYear: time.year() as u16,
        wMonth: time.month() as u16,
        wDayOfWeek: time.weekday().num_days_from_sunday() as u16,
        wDay: time.day() as u16,
        wHour: time.hour() as u16,
        wMinute: time.minute() as u16,
        wSecond: time.second() as u16,
        wMilliseconds: time.timestamp_subsec_millis() as u16,
    }
}
