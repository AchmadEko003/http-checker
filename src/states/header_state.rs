#[derive(Default, Debug)]
pub struct HeaderStatus {
    pub server: HeaderField,
    pub x_powered_by: HeaderField,
    pub x_content_type_options: HeaderField,
    pub x_frame_options: HeaderField,
    pub strict_transport_security: HeaderField,
    pub referrer_policy: HeaderField,
}

#[derive(Debug)]
pub enum HeaderField {
    Missing,
    Present {
        raw: String,
        valid: bool,
        note: Option<String>,
    },
}

impl Default for HeaderField {
    fn default() -> Self {
        HeaderField::Missing
    }
}
