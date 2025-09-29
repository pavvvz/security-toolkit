use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub const THREADS: usize = 100;
pub const TIMEOUT_SECS: u64 = 1;

pub fn scan_port(target: &str, port: u16, timeout: Duration) -> bool {
    let address = format!("{}:{}", target, port);
    // uso il tipo con percorso completo per evitare di importarlo se non serve altrove
    match address.parse::<std::net::SocketAddr>() {
        Ok(sock) => TcpStream::connect_timeout(&sock, timeout).is_ok(),
        Err(_) => false,
    }
}

pub fn scan_target(target: &str) -> Vec<u16> {
    let (tx, rx) = mpsc::channel();

    println!("Scansione porte su {}...", target);

    let mut handles = Vec::with_capacity(THREADS);

    for thread_id in 0..THREADS {
        let tx = tx.clone();
        let target = target.to_string();
        let handle = thread::spawn(move || {
            let mut port = (thread_id as u32) + 1; // iniziamo da 1
            let max_port = u32::from(u16::MAX);

            while port <= max_port {
                let p = port as u16;
                if scan_port(&target, p, Duration::from_secs(TIMEOUT_SECS)) {
                    let _ = tx.send(p); // ignoro l'errore se il receiver è chiuso
                }

                port = match port.checked_add(THREADS as u32) {
                    Some(next) => next,
                    None => break,
                };
            }
        });

        handles.push(handle);
    }

    drop(tx); // chiudiamo il tx originale

    let mut open_ports = Vec::new();
    for port in rx {
        println!("port {}: is open", port);
        open_ports.push(port);
    }

    for h in handles {
        let _ = h.join();
    }

    open_ports.sort_unstable();
    open_ports.dedup();
    println!("open ports: {:?}", open_ports);

    open_ports
}
