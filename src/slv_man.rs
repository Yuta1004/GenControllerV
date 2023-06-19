use crate::port::{Port, PortKind};

macro_rules! slv_regs_map {
    ($self:ident, $kind:path, $closure:tt, $delimiter:expr) => {
        $self.ports
            .iter()
            .filter(|(_, p)| matches!(p.kind, $kind))
            .map($closure)
            .collect::<Vec<String>>()
            .join($delimiter)
    };
}

pub struct SlvMan<'a> {
    ports: Vec<(i32, &'a Port)>,
}

impl<'a> From<&'a Vec<Port>> for SlvMan<'a> {
    fn from(ports: &'a Vec<Port>) -> SlvMan<'a> {
        let mut slv_man = SlvMan { ports: vec![] };
        for (idx, port) in ports.iter().enumerate() {
            slv_man.ports.push((idx as i32, port));
        }
        slv_man
    }
}

impl<'a> SlvMan<'a> {
    // SLV_REGISTERS
    pub fn gen_slv_registers(&self) -> String {
        slv_regs_map!(
            self, PortKind::Input,
            (|(i, _)| format!("slv_reg{}", i)),
            ","
        )
    }

    // SLV_REGISTERS_RESET
    pub fn gen_slv_registers_reset(&self) -> String {
        slv_regs_map!(
            self, PortKind::Input,
            (|(i, _)| format!("slv_reg{} <= 32'b0;", i)),
            "\n"
        )
    }

    // SLV_REGISTERS_SET
    pub fn gen_slv_registers_set(&self) -> String {
        slv_regs_map!(
            self, PortKind::Input,
            (|(i, p)| format!("16'h{:04x}: slv_reg{} <= S_AXI_WDATA;", p.addr, i)),
            "\n"
        )
    }

    // SLV_OCACHE_REGISTERS
    pub fn gen_slv_ocache_registers(&self) -> String {
        slv_regs_map!(
            self, PortKind::Input,
            (|(i, p)| format!("reg {} ocache_slv_reg{} [0:1];", p.fmt_width(), i)),
            "\n"
        )
    }

    // SLV_OCACHE_REGISTERS_ASSIGN
    pub fn gen_slv_ocache_registers_assign(&self) -> String {
        slv_regs_map!(
            self, PortKind::Input,
            (|(i, p)| format!("assign {} = ocache_slv_reg{}[1];", p.name, i)),
            "\n"
        )
    }

    // SLV_OCACHE_REGISTERS_RESET
    pub fn gen_slv_ocache_registers_reset(&self) -> String {
        slv_regs_map!(
            self, PortKind::Input,
            (|(i, p)| format!("ocache_slv_reg{}[0] <= {}'b0; ocache_slv_reg{}[1] <= {}'b0;", i, p.width, i, p.width)),
            "\n"
        )
    }

    // SLV_OCACHE_REGISTERS_SET
    pub fn gen_slv_ocache_registers_set(&self) -> String {
        slv_regs_map!(
            self, PortKind::Input,
            (|(i, _)| format!("ocache_slv_reg{}[1] <= ocache_slv_reg{}[0]; ocache_slv_reg{}[0] <= slv_reg{};", i, i, i, i)),
            "\n"
        )
    }

    // SLV_ICACHE_REGISTERS_SET
    pub fn gen_slv_icache_registers(&self) -> String {
        slv_regs_map!(
            self, PortKind::Output,
            (|(i, _)| format!("reg icache_slv_reg{} [0:1];", i)),
            "\n"
        )
    }

    // SLV_ICACHE_REGISTERS_RESET
    pub fn gen_slv_icache_registers_reset(&self) -> String {
        slv_regs_map!(
            self, PortKind::Output,
            (|(i, p)| format!("icache_slv_reg{}[0] <= {}'b0; icache_slv_reg{}[1] <= {}'b0;", i, p.width, i, p.width)),
            "\n"
        )
    }

    // SLV_ICACHE_REGISTERS_ASSIGN
    pub fn gen_slv_icache_registers_assign(&self) -> String {
        slv_regs_map!(
            self, PortKind::Output,
            (|(i, p)| format!("16'h{:04x}: reg_data_out <= icache_slv_reg{}[1];", p.addr, i)),
            "\n"
        )
    }
}
