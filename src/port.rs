use std::fmt;
use std::fmt::Display;

pub enum PortKind {
    Clock,
    Input,
    Output,
}

impl Display for PortKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Controllerへのinputは接続先回路から見るとoutputになる（その逆も同じ）
        match self {
            PortKind::Input => write!(f, "output"),
            PortKind::Output => write!(f, "input"),
            PortKind::Clock => write!(f, "input"),
        }
    }
}

pub struct Port {
    pub kind: PortKind,
    pub name: String,
    pub width: u32,
    pub addr: u32,
}

impl Port {
    pub fn new(kind: PortKind, name: impl Into<String>, width: u32, addr: u32) -> Port {
        assert!(width > 0);
        Port {
            kind,
            width,
            addr,
            name: name.into(),
        }
    }

    pub fn fmt_width(&self) -> String {
        if self.width == 1 {
            "".to_string()
        } else {
            format!("[{}:0]", self.width - 1)
        }
    }
}
