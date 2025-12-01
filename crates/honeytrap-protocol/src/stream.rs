/// QUIC Stream Utilities
///
/// Helper functions for working with QUIC streams in honeypots

#[cfg(feature = "quic")]
use quinn::{RecvStream, SendStream};

#[cfg(feature = "quic")]
use std::io;

/// QUIC Stream Reader/Writer Wrapper
pub struct QuicStream {
    #[cfg(feature = "quic")]
    send: SendStream,
    #[cfg(feature = "quic")]
    recv: RecvStream,
}

#[cfg(feature = "quic")]
impl QuicStream {
    /// Neue QuicStream von Bi-Stream
    pub fn new(send: SendStream, recv: RecvStream) -> Self {
        Self { send, recv }
    }

    /// Bytes lesen
    pub async fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.recv.read(buf).await {
            Ok(Some(n)) => Ok(n),
            Ok(None) => Ok(0), // EOF
            Err(e) => Err(io::Error::new(io::ErrorKind::Other, e)),
        }
    }

    /// Bytes schreiben
    pub async fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.send
            .write(buf)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    /// Alle Bytes schreiben
    pub async fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
        self.send
            .write_all(buf)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    /// Flush
    pub async fn flush(&mut self) -> io::Result<()> {
        // QUIC streams sind automatisch gepuffert
        Ok(())
    }

    /// Stream schließen
    pub async fn finish(&mut self) -> io::Result<()> {
        self.send
            .finish()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    /// Send-Stream abrufen (für direkte Operationen)
    pub fn send_stream(&mut self) -> &mut SendStream {
        &mut self.send
    }

    /// Recv-Stream abrufen (für direkte Operationen)
    pub fn recv_stream(&mut self) -> &mut RecvStream {
        &mut self.recv
    }
}

/// Line-based Reader für QUIC Streams (z.B. für SSH, HTTP)
#[cfg(feature = "quic")]
pub struct QuicLineReader {
    stream: RecvStream,
    buffer: Vec<u8>,
}

#[cfg(feature = "quic")]
impl QuicLineReader {
    /// Neuer Line Reader
    pub fn new(stream: RecvStream) -> Self {
        Self {
            stream,
            buffer: Vec::new(),
        }
    }

    /// Line lesen (bis \n)
    pub async fn read_line(&mut self) -> io::Result<String> {
        let mut line_buf = Vec::new();

        loop {
            // Prüfe ob \n im buffer
            if let Some(pos) = self.buffer.iter().position(|&b| b == b'\n') {
                line_buf.extend_from_slice(&self.buffer[..=pos]);
                self.buffer.drain(..=pos);
                break;
            }

            // Mehr Daten lesen
            let mut buf = [0u8; 1024];
            match self.stream.read(&mut buf).await {
                Ok(Some(n)) => {
                    self.buffer.extend_from_slice(&buf[..n]);
                }
                Ok(None) => {
                    // EOF - return remaining buffer
                    if !self.buffer.is_empty() {
                        line_buf.extend_from_slice(&self.buffer);
                        self.buffer.clear();
                        break;
                    } else {
                        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF"));
                    }
                }
                Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
            }
        }

        String::from_utf8(line_buf)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Exakt N bytes lesen
    pub async fn read_exact(&mut self, n: usize) -> io::Result<Vec<u8>> {
        let mut result = Vec::with_capacity(n);

        // Erst aus Buffer
        let from_buffer = self.buffer.len().min(n);
        result.extend_from_slice(&self.buffer[..from_buffer]);
        self.buffer.drain(..from_buffer);

        // Rest aus Stream
        let remaining = n - from_buffer;
        if remaining > 0 {
            let mut buf = vec![0u8; remaining];
            let mut offset = 0;

            while offset < remaining {
                match self.stream.read(&mut buf[offset..]).await {
                    Ok(Some(n)) => offset += n,
                    Ok(None) => {
                        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "EOF"))
                    }
                    Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
                }
            }

            result.extend_from_slice(&buf);
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quic_stream_available() {
        // Test dass die Strukturen kompilieren
        #[cfg(feature = "quic")]
        {
            // QuicStream und QuicLineReader sind verfügbar
        }
    }
}
