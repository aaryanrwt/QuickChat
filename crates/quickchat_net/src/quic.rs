use anyhow::Result;
use quinn::{ClientConfig, Endpoint, ServerConfig};
use std::net::SocketAddr;
use std::sync::Arc;

pub struct QuicNode {
    pub endpoint: Endpoint,
}

impl QuicNode {
    pub fn new(listen_addr: SocketAddr) -> Result<Self> {
        let _ = rustls::crypto::ring::default_provider().install_default();
        let (server_config, client_config) = Self::configure_certs()?;

        let mut endpoint = Endpoint::server(server_config, listen_addr)?;
        endpoint.set_default_client_config(client_config);

        Ok(Self { endpoint })
    }

    pub async fn connect(&self, addr: SocketAddr, server_name: &str) -> Result<quinn::Connection> {
        let connection = self.endpoint.connect(addr, server_name)?.await?;
        Ok(connection)
    }

    fn configure_certs() -> Result<(ServerConfig, ClientConfig)> {
        // Generate a self-signed certificate for QUIC
        let cert = rcgen::generate_simple_self_signed(vec!["localhost".to_string()])?;

        let cert_der = cert.cert.der().to_vec();
        let priv_key = cert.signing_key.serialize_der();

        let rustls_cert = rustls::pki_types::CertificateDer::from(cert_der);
        let rustls_key = rustls::pki_types::PrivateKeyDer::Pkcs8(
            rustls::pki_types::PrivatePkcs8KeyDer::from(priv_key),
        );

        let server_crypto = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(vec![rustls_cert.clone()], rustls_key)?;

        let quic_server_config = quinn::crypto::rustls::QuicServerConfig::try_from(server_crypto)?;
        let server_config = ServerConfig::with_crypto(Arc::new(quic_server_config));

        // Create a client configuration that accepts any server certificate
        let client_crypto = rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(AcceptAnyCertVerifier))
            .with_no_client_auth();

        let quic_client_config = quinn::crypto::rustls::QuicClientConfig::try_from(client_crypto)?;
        let client_config = ClientConfig::new(Arc::new(quic_client_config));

        Ok((server_config, client_config))
    }
}

// A verifier that accepts any certificate (for P2P TOFU/custom validation)
#[derive(Debug)]
struct AcceptAnyCertVerifier;

impl rustls::client::danger::ServerCertVerifier for AcceptAnyCertVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::pki_types::CertificateDer<'_>,
        _intermediates: &[rustls::pki_types::CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rustls::pki_types::CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::ED25519,
        ]
    }
}
