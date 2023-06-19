#[macro_export]
macro_rules! port_maps {
    ($target:expr, $closure:tt, $delimiter:expr) => {
        $target
            .iter()
            .map($closure)
            .collect::<Vec<String>>()
            .join($delimiter)
    };

    ($target:expr, $kind:path, $closure:tt, $delimiter:expr) => {
        $target
            .iter()
            .filter(|(_, p)| matches!(p.kind, $kind))
            .map($closure)
            .collect::<Vec<String>>()
            .join($delimiter)
    };
}
