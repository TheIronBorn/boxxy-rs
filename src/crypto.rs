use rustls;
use rustls::{Session, ClientSession};

use std::io;
use std::io::prelude::*;
use std::net::TcpStream;


pub mod danger {
    use rustls;
    use rust_crypto::sha2::Sha256;
    use rust_crypto::digest::Digest;
    use base64;
    use webpki;

    use std::iter::repeat;

    pub struct PinnedCertificateVerification {}

    fn verify_fingerprint(trusted: &str, cert: &rustls::Certificate) -> Result<(), ()> {
        let idx = match trusted.find(':') {
            Some(idx) => idx,
            None => return Err(()),
        };

        let (algo, trusted_fp) = trusted.split_at(idx);

        let trusted_fp = base64::decode(&trusted_fp[1..]).unwrap();

        let fingerprint = match algo {
            "SHA256" => {
                let mut h = Sha256::new();
                h.input(&cert.0);

                let mut buf: Vec<u8> = repeat(0).take((h.output_bits()+7)/8).collect();
                h.result(&mut buf);
                buf
            },
            _ => return Err(()),
        };

        if trusted_fp == fingerprint {
            Ok(())
        } else {
            Err(())
        }
    }

    impl rustls::ServerCertVerifier for PinnedCertificateVerification {

        fn verify_server_cert(&self,
                              _roots: &rustls::RootCertStore,
                              presented_certs: &[rustls::Certificate],
                              dns_name: webpki::DNSNameRef,
                              _ocsp: &[u8]) -> Result<rustls::ServerCertVerified, rustls::TLSError> {

            for cert in presented_certs {
                if verify_fingerprint(dns_name.into(), &cert).is_ok() {
                    return Ok(rustls::ServerCertVerified::assertion());
                }
            }

            Err(rustls::TLSError::WebPKIError(webpki::Error::CertNotValidForName))
        }
    }
}


#[derive(Debug)]
pub struct OwnedTlsStream {
    pub sess: rustls::ClientSession,
    pub sock: TcpStream,
}

impl OwnedTlsStream {
    pub fn new(sess: ClientSession, sock: TcpStream) -> OwnedTlsStream {
        OwnedTlsStream { sess, sock }
    }

    fn complete_prior_io(&mut self) -> io::Result<()> {
        if self.sess.is_handshaking() {
            self.sess.complete_io(&mut self.sock)?;
        }

        if self.sess.wants_write() {
            self.sess.complete_io(&mut self.sock)?;
        }

        Ok(())
    }
}

impl Read for OwnedTlsStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.complete_prior_io()?;

        if self.sess.wants_read() {
            self.sess.complete_io(&mut self.sock)?;
        }

        self.sess.read(buf)
    }
}

impl Write for OwnedTlsStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.complete_prior_io()?;

        let len = self.sess.write(buf)?;
        self.sess.complete_io(&mut self.sock)?;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.complete_prior_io()?;

        self.sess.flush()?;
        if self.sess.wants_write() {
            self.sess.complete_io(&mut self.sock)?;
        }
        Ok(())
    }
}
