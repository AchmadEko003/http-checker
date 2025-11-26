use crate::states::HeaderStatus;

#[derive(Default)]
pub struct ScanStatus {
    pub url: String,
    pub scheme: String,
    pub status_code: u16,
    pub status_text: String,
    pub response_time_ms: u128,
    pub header_info: HeaderStatus,
}
