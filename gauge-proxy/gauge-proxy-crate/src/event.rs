use alloc::string::{String, ToString};
use casper_types::Key;

pub enum GaugeProxyEvent {
    CommitAdmins {
        ownership_admin: Key,
        emergency_admin: Key,
    },
    ApplyAdmins {
        ownership_admin: Key,
        emergency_admin: Key,
    },
}

impl GaugeProxyEvent {
    pub fn type_name(&self) -> String {
        match self {
            GaugeProxyEvent::CommitAdmins {
                ownership_admin: _,
                emergency_admin: _,
            } => "commitAdmins",
            GaugeProxyEvent::ApplyAdmins {
                ownership_admin: _,
                emergency_admin: _,
            } => "applyAdmins",
        }
        .to_string()
    }
}
