use crate::port_maps;
use crate::Port;

pub struct WireMan<'a> {
    ports: Vec<(i32, &'a Port)>,
}

impl<'a> From<&'a Vec<Port>> for WireMan<'a> {
    fn from(ports: &'a Vec<Port>) -> WireMan<'a> {
        let mut slv_man = WireMan { ports: vec![] };
        for (idx, port) in ports.iter().enumerate() {
            slv_man.ports.push((idx as i32, port));
        }
        slv_man
    }
}

impl<'a> WireMan<'a> {
    #[allow(unused_parens)]
    pub fn gen_wire_definitions(&self) -> String {
        port_maps!(
            self.ports,
            (|(_, p)| format!("{} wire {} {}", p.kind, p.fmt_width(), p.name)),
            ",\n"
        ) + ","
    }

    #[allow(unused_parens)]
    pub fn gen_wire_connctions(&self) -> String {
        port_maps!(
            self.ports,
            (|(_, p)| format!(".{}({})", p.name, p.name)),
            ",\n"
        ) + ","
    }
}
