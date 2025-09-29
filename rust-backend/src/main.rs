mod portscanner;
fn main() {
    let target = "127.0.0.1"; //example
    let open_ports = portscanner::scan_target(target);
    println!(
        "Port scan completed. Found {} open ports.",
        open_ports.len()
    );
}
