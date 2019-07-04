pub fn local_ip() -> Option<String> {
    let interfaces = pnet::datalink::interfaces();
    for iterface in interfaces {
        for ip in iterface.ips {
            if !ip.is_ipv4() {
                continue;
            }
            let ip = ip.ip();
            if ip.is_loopback() {
                continue;
            }
            return Some(format!("{}", ip));
        }
    }
    None
}
