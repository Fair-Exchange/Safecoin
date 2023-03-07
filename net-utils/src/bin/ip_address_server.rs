use {
    clap::{Arg, Command},
    std::net::{SocketAddr, TcpListener},
};

fn main() {
    solana_logger::setup();
    let matches = Command::new("safecoin-ip-address-server")
        .version(safecoin_version::version!())
        .arg(
            Arg::new("port")
                .index(1)
                .required(true)
                .help("TCP port to bind to"),
        )
        .get_matches();

    let port = matches.value_of("port").unwrap();
    let port = port
        .parse()
        .unwrap_or_else(|_| panic!("Unable to parse {}", port));
    let bind_addr = SocketAddr::from(([0, 0, 0, 0], port));
    let tcp_listener = TcpListener::bind(bind_addr).expect("unable to start tcp listener");
    let _runtime = safecoin_net_utils::ip_echo_server(tcp_listener, /*shred_version=*/ None);
    loop {
        std::thread::park();
    }
}
