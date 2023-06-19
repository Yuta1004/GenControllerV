use std::fmt;
use std::fmt::Display;

pub enum PortKind {
    Input,
    Output,
}

impl Display for PortKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Controllerへのinputは接続先回路から見るとoutputになる（その逆も同じ）
        match self {
            PortKind::Input => write!(f, "output"),
            PortKind::Output => write!(f, "input")
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
            kind, width, addr,
            name: name.into()
        }
    }

    pub fn gen_wire_definition(&self) -> String {
        format!("{} wire {} {},", self.kind, self.fmt_width(), self.name)
    }

    pub fn gen_wire_connection(&self) -> String {
        format!(".{}({}),", self.name, self.name)
    }

    pub fn fmt_width(&self) -> String {
        if self.width == 1 {
            "".to_string()
        } else {
            format!("[{}:0]", self.width-1)
        }
    }
}
