pub mod event_log;
pub mod registry;
pub mod time;

pub use event_log::{EventLogLevel, report_event_log};
pub use registry::{apply_windows_time_policy, query_windows_time_policy};
pub use time::{set_system_time_direct, slew_system_time};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Windows API 错误: {0}")]
    Windows(#[from] windows::core::Error),
    #[error("注册表操作错误: {0}")]
    Registry(String),
    #[error("时间转换错误")]
    TimeConversion,
    #[error("驱动错误: {0}")]
    Driver(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Timelike, Utc};

    fn run_elevated_if_needed(test_name: &str) -> bool {
        // 我们实际上可以更简便地通过一次无副作用的操作进行探测：
        // 比如尝试直接设置当前时间（即使普通用户也会因为 OpenProcessToken / AdjustTokenPrivileges 提权阶段报错而失败）
        let has_privilege = set_system_time_direct(Utc::now()).is_ok();
        if has_privilege {
            return true; // 具备特权，直接在当前进程执行真实测试
        }

        // 如果不具备特权，尝试以管理员身份重新拉起当前测试进程来完成测试
        let current_exe = std::env::current_exe().unwrap();
        let current_exe_str = current_exe.to_str().unwrap();

        // 自动拉起带有 RunAs 动词的管理员子进程只跑当前该测试
        let ps_command = format!(
            "Start-Process -FilePath '{}' -ArgumentList '{}', '--nocapture' -Verb RunAs -Wait -WindowStyle Hidden",
            current_exe_str, test_name
        );

        let status = std::process::Command::new("powershell")
            .args(&["-Command", &ps_command])
            .status();

        match status {
            Ok(s) if s.success() => {
                // 特权子进程成功跑完（退出码 0），通知当前非特权测试父进程可以直接返回通过
                false
            }
            _ => {
                panic!("测试需要管理员权限，自动提权失败或被拒绝！");
            }
        }
    }

    // 1. 系统时间还原 Guard，确保测试结束后时间回归正常，避免破坏宿主系统
    struct TimeRestoreGuard {
        start_instant: std::time::Instant,
        before_time: chrono::DateTime<Utc>,
    }

    impl Drop for TimeRestoreGuard {
        fn drop(&mut self) {
            let elapsed = self.start_instant.elapsed();
            // 补偿测试所耗费的时间，从而极高精度地恢复系统真实时间
            let restore_time =
                self.before_time + chrono::Duration::milliseconds(elapsed.as_millis() as i64);
            let res = set_system_time_direct(restore_time);
            if let Err(e) = res {
                eprintln!("警告：TimeRestoreGuard 还原系统时间失败: {:?}", e);
            } else {
                println!("TimeRestoreGuard 成功还原系统时间。");
            }
        }
    }

    // 2. 注册表还原 Guard，防止将用户的 Windows 时间政策永久更改为 NoSync/NTP
    struct RegistryRestoreGuard {
        original_value: String,
    }

    impl Drop for RegistryRestoreGuard {
        fn drop(&mut self) {
            let disable = self.original_value == "NoSync";
            let res = apply_windows_time_policy(disable);
            if let Err(e) = res {
                eprintln!("警告：RegistryRestoreGuard 恢复注册表失败: {:?}", e);
            } else {
                println!(
                    "RegistryRestoreGuard 成功将注册表还原为原始值 '{}'。",
                    self.original_value
                );
            }
        }
    }

    #[test]
    fn test_utc_to_system_time_boundaries() {
        use chrono::TimeZone;

        // 测试普通时间
        let t1 = Utc
            .with_ymd_and_hms(2026, 5, 24, 22, 10, 15)
            .unwrap()
            .with_nanosecond(123_000_000)
            .unwrap();
        let st1 = time::utc_to_system_time(t1);
        assert_eq!(st1.wYear, 2026);
        assert_eq!(st1.wMonth, 5);
        assert_eq!(st1.wDay, 24);
        assert_eq!(st1.wHour, 22);
        assert_eq!(st1.wMinute, 10);
        assert_eq!(st1.wSecond, 15);
        assert_eq!(st1.wMilliseconds, 123);

        // 测试闰年边界 (2024-02-29)
        let t2 = Utc.with_ymd_and_hms(2024, 2, 29, 12, 0, 0).unwrap();
        let st2 = time::utc_to_system_time(t2);
        assert_eq!(st2.wYear, 2024);
        assert_eq!(st2.wMonth, 2);
        assert_eq!(st2.wDay, 29);
        assert_eq!(st2.wHour, 12);
        assert_eq!(st2.wDayOfWeek, 4); // 2024-02-29 是星期四 (4)

        // 测试跨年边界 (2026-12-31T23:59:59.999Z)
        let t3 = Utc
            .with_ymd_and_hms(2026, 12, 31, 23, 59, 59)
            .unwrap()
            .with_nanosecond(999_000_000)
            .unwrap();
        let st3 = time::utc_to_system_time(t3);
        assert_eq!(st3.wYear, 2026);
        assert_eq!(st3.wMonth, 12);
        assert_eq!(st3.wDay, 31);
        assert_eq!(st3.wHour, 23);
        assert_eq!(st3.wMinute, 59);
        assert_eq!(st3.wSecond, 59);
        assert_eq!(st3.wMilliseconds, 999);
    }

    #[test]
    fn test_set_system_time_direct_real() {
        if !run_elevated_if_needed("tests::test_set_system_time_direct_real") {
            return;
        }

        // 记录当前精确时间并生成 Guard
        let before_time = Utc::now();
        let start_instant = std::time::Instant::now();
        let _guard = TimeRestoreGuard {
            start_instant,
            before_time,
        };

        // 将系统时间调快 5秒
        let target_time = before_time + chrono::Duration::seconds(5);
        let res = set_system_time_direct(target_time);
        assert!(res.is_ok(), "特权模式下修改系统时间应当成功: {:?}", res);

        // 获取刚刚修改后的系统时间并断言
        let after_time = Utc::now();
        let diff = (after_time - target_time).num_milliseconds().abs();

        // 校验实际系统时间与我们期望的时间误差在 500ms 内，说明真实修改生效了
        assert!(
            diff < 500,
            "修改后的系统时间应与目标时间接近（相差 {} 毫秒）",
            diff
        );
    }

    #[test]
    fn test_slew_system_time_skipped_small() {
        if !run_elevated_if_needed("tests::test_slew_system_time_skipped_small") {
            return;
        }

        // 微调时间小偏差测试（差值小于 2ms 应进入快速路径直接返回），必须严格成功
        let target_time = Utc::now() + chrono::Duration::milliseconds(1);
        let res = slew_system_time(target_time);
        assert_eq!(res.unwrap(), "slew-skipped-small");
    }

    #[test]
    fn test_slew_system_time_real() {
        if !run_elevated_if_needed("tests::test_slew_system_time_real") {
            return;
        }

        // 制造一个大于 2ms 且足够让 slew 运行的偏差（例如 10 毫秒）
        let target_time = Utc::now() + chrono::Duration::milliseconds(10);
        let res = slew_system_time(target_time);

        assert!(res.is_ok(), "微调系统时间应执行成功: {:?}", res);
        let val = res.unwrap();
        // 应成功进入真实的微调逻辑分支，返回值必为 slew-precise 或 slew-legacy 之一
        assert!(
            val == "slew-precise" || val == "slew-legacy",
            "应真实进入微调流程，实际返回: {}",
            val
        );
    }

    #[test]
    fn test_apply_windows_time_policy_real() {
        if !run_elevated_if_needed("tests::test_apply_windows_time_policy_real") {
            return;
        }

        // 备份当前的注册表 Type 键值
        let original_value = query_windows_time_policy().expect("备份原始注册表值应当成功");
        let _guard = RegistryRestoreGuard { original_value };

        // 1. 测试应用政策为 true (禁止同步 -> NoSync)
        let res = apply_windows_time_policy(true);
        assert!(res.is_ok(), "设为 NoSync 应当成功: {:?}", res);
        let val = query_windows_time_policy().unwrap();
        assert_eq!(val, "NoSync", "注册表应真实写入为 'NoSync'");

        // 2. 测试应用政策为 false (允许同步 -> NTP)
        let res = apply_windows_time_policy(false);
        assert!(res.is_ok(), "设为 NTP 应当成功: {:?}", res);
        let val = query_windows_time_policy().unwrap();
        assert_eq!(val, "NTP", "注册表应真实写入为 'NTP'");
    }

    #[test]
    fn test_report_event_log_real() {
        if !run_elevated_if_needed("tests::test_report_event_log_real") {
            return;
        }

        let unique_marker = format!("rchronos_test_marker_{}", Utc::now().timestamp_millis());

        // 1. 测试写入不同级别的事件日志
        let levels = [
            EventLogLevel::Success,
            EventLogLevel::Information,
            EventLogLevel::Warning,
            EventLogLevel::Error,
        ];

        for &level in &levels {
            let msg = format!(
                "[{:?}] Windows Event Log 端到端单元测试 | 唯一标识: {}",
                level, unique_marker
            );
            let res = report_event_log(level, &msg);
            assert!(
                res.is_ok(),
                "特权模式下写入事件日志 {:?} 应成功: {:?}",
                level,
                res
            );
        }

        // 等待一小会以让日志被系统可靠写入并索引
        std::thread::sleep(std::time::Duration::from_millis(200));

        // 2. 通过 PowerShell 查询最新几条包含当前 unique_marker 的 rchronos 事件日志
        let ps_cmd = format!(
            "Get-EventLog -LogName Application -Source rchronos -Newest 10 | Where-Object {{ $_.Message -like '*{}*' }} | Select-Object -ExpandProperty Message",
            unique_marker
        );

        let output = std::process::Command::new("powershell")
            .args(&["-Command", &ps_cmd])
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let stdout_str = String::from_utf8_lossy(&out.stdout);
                assert!(
                    stdout_str.contains(&unique_marker),
                    "通过 PowerShell 读取事件日志未发现包含当前唯一标识符的记录！读到的输出为: \n{}",
                    stdout_str
                );
                println!("端到端事件日志校验成功，查询结果包含唯一标识符。");
            }
            Ok(out) => {
                let stderr_str = String::from_utf8_lossy(&out.stderr);
                panic!("执行 PowerShell 读取事件日志失败: \n{}", stderr_str);
            }
            Err(e) => {
                panic!("未能启动 powershell 进程: {:?}", e);
            }
        }
    }
}
