#[derive(Default)]
pub struct ScanStatus {
    pub url: String,
    pub status_code: u16,
    pub status_text: String,
    pub response_time_ms: u128,
    pub server: String,
    pub x_powered_by: String,
    pub x_content_type_options: String,
    pub x_frame_options: String,
    pub strict_transport_security: String,
    pub referrer_policy: String,
}
