#[deprecated(
    since = "1.15.0",
    note = "Please use `safecoin_quic_client::nonblocking::quic_client::QuicClientConnection` instead."
)]
pub use safecoin_quic_client::nonblocking::quic_client::QuicClientConnection as QuicTpuConnection;
pub use safecoin_quic_client::nonblocking::quic_client::{
    QuicClient, QuicClientCertificate, QuicLazyInitializedEndpoint,
};
