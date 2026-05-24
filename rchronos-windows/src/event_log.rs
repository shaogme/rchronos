use crate::Result;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventLogLevel {
    Success = 0x0000,
    Error = 0x0001,
    Warning = 0x0002,
    Information = 0x0004,
}

use windows::Win32::System::EventLog::{
    DeregisterEventSource, REPORT_EVENT_TYPE, RegisterEventSourceW, ReportEventW,
};
use windows::core::PCWSTR;

pub fn report_event_log(level: EventLogLevel, message: &str) -> Result<()> {
    let source_name_w: Vec<u16> = "rchronos"
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect();
    let handle = unsafe {
        RegisterEventSourceW(None, PCWSTR(source_name_w.as_ptr()))
            .map_err(|e| crate::Error::Driver(format!("RegisterEventSource 失败: {e}")))?
    };

    let wide_strings: Vec<u16> = message.encode_utf16().chain(std::iter::once(0)).collect();
    let pcwstrs = [PCWSTR(wide_strings.as_ptr())];

    unsafe {
        if let Err(e) = ReportEventW(
            handle,
            REPORT_EVENT_TYPE(level as u16),
            0,
            1,
            None,
            0,
            Some(&pcwstrs),
            None,
        ) {
            let _ = DeregisterEventSource(handle);
            return Err(crate::Error::Driver(format!("ReportEvent 失败: {e}")));
        }

        DeregisterEventSource(handle)
            .map_err(|e| crate::Error::Driver(format!("DeregisterEventSource 失败: {e}")))?;
    }

    Ok(())
}
