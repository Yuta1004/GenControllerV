mod mymacro;
mod port;
mod wire_man;
mod slv_man;

use std::fs;
use std::fs::File;
use std::io::Write;

use serde::Serialize;
use tinytemplate::TinyTemplate;

pub use port::{Port, PortKind};
use wire_man::WireMan;
use slv_man::SlvMan;

const CONTROLLER_V: &'static str = include_str!("../template/controller.v");
const CONTROLLER_AXI_V: &'static str = include_str!("../template/controller_AXI.v");
const CONTROLLER_AG_V: &'static str = include_str!("../template/controller_auto_generated.v");

#[allow(non_snake_case)]
pub struct Controller {
    name: String,
    controller_v: String,
    controller_AXI_v: String,
    controller_auto_generated_v: String,
}

impl Controller {
    #[allow(non_snake_case)]
    pub fn new(name: String, controller_v: String, controller_AXI_v: String, controller_auto_generated_v: String) -> Controller {
        Controller {
            name,
            controller_v,
            controller_AXI_v,
            controller_auto_generated_v,
        }
    }

    pub fn save(&self, dir: impl Into<String>) -> Result<(), Box<dyn std::error::Error>> {
        let dir = dir.into();
        let _ = fs::create_dir(&dir);

        Self::__save(dir.clone()+"/"+&self.name+"_controller.v", &self.controller_v)?;
        Self::__save(dir.clone()+"/"+&self.name+"_controller_AXI.v", &self.controller_AXI_v)?;
        Self::__save(dir        +"/"+&self.name+"_controller_auto_generated.v", &self.controller_auto_generated_v)?;

        Ok(())
    }

    fn __save(fname: String, body: &String) -> Result<(), Box<dyn std::error::Error>> {
        let mut f = File::create(fname)?;
        write!(f, "{}", body)?;
        f.flush()?;
        Ok(())
    }
}

#[allow(non_snake_case)]
#[derive(Serialize)]
struct Template {
    NAME: String,
    CLOCK: String,
    WIRE_DEFINITIONS: String,
    WIRE_CONNECTIONS: String,
    SLV_REGISTERS: String,
    SLV_REGISTERS_RESET: String,
    SLV_REGISTERS_SET: String,
    SLV_OCACHE_REGISTERS: String,
    SLV_OCACHE_REGISTERS_ASSIGN: String,
    SLV_OCACHE_REGISTERS_RESET: String,
    SLV_OCACHE_REGISTERS_SET: String,
    SLV_ICACHE_REGISTERS: String,
    SLV_ICACHE_REGISTERS_SET: String,
    SLV_ICACHE_REGISTERS_ASSIGN: String,
}

pub fn generate<'a>(name: impl Into<String>, clock_port: impl Into<String>, ports: &'a Vec<Port>) -> Result<Controller, Box<dyn std::error::Error>> {
    let name = name.into();

    let wire_man = WireMan::from(ports);
    let slv_man = SlvMan::from(ports);

    let context = Template {
        NAME: name.clone(),
        CLOCK: clock_port.into(),
        WIRE_DEFINITIONS: wire_man.gen_wire_definitions(),
        WIRE_CONNECTIONS: wire_man.gen_wire_connctions(),
        SLV_REGISTERS: slv_man.gen_slv_registers(),
        SLV_REGISTERS_RESET: slv_man.gen_slv_registers_reset(),
        SLV_REGISTERS_SET: slv_man.gen_slv_registers_set(),
        SLV_OCACHE_REGISTERS: slv_man.gen_slv_ocache_registers(),
        SLV_OCACHE_REGISTERS_ASSIGN: slv_man.gen_slv_ocache_registers_assign(),
        SLV_OCACHE_REGISTERS_RESET: slv_man.gen_slv_ocache_registers_reset(),
        SLV_OCACHE_REGISTERS_SET: slv_man.gen_slv_ocache_registers_set(),
        SLV_ICACHE_REGISTERS: slv_man.gen_slv_icache_registers(),
        SLV_ICACHE_REGISTERS_SET: slv_man.gen_slv_icache_registers_set(),
        SLV_ICACHE_REGISTERS_ASSIGN: slv_man.gen_slv_icache_registers_assign(),
    };

    let mut tt = TinyTemplate::new();
    tt.set_default_formatter(&tinytemplate::format_unescaped);
    tt.add_template("controller.v", CONTROLLER_V)?;
    tt.add_template("controller_AXI.v", CONTROLLER_AXI_V)?;
    tt.add_template("controller_auto_generated.v", CONTROLLER_AG_V)?;

    Ok(Controller::new(
        name,
        tt.render("controller.v", &context)?,
        tt.render("controller_AXI.v", &context)?,
        tt.render("controller_auto_generated.v", &context)?,
    ))
}
