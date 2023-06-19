use gencontroller_v::generate;
use gencontroller_v::{Port, PortKind};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let ports = vec![
        Port::new(PortKind::Input, "CRST", 1, 0x0000),
        Port::new(PortKind::Input, "CEXEC", 1, 0x0004),
        Port::new(PortKind::Input, "CMEM_ADDR", 32, 0x0008),
        Port::new(PortKind::Output, "CSTAT", 1, 0x000C),
    ];
    let controller = generate("simple", "CCLK", &ports)?;
    controller.save("./simple")
}
