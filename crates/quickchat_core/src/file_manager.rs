use anyhow::Result;
use futures::SinkExt;
use prost::Message;
use quickchat_types::proto::{Envelope, FileChunk, envelope::Payload};
use quinn::SendStream;
use std::io::Read;
use std::path::Path;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_util::codec::{FramedWrite, LengthDelimitedCodec};

pub async fn send_file(send: SendStream, filepath: &Path, file_id: String) -> Result<()> {
    let mut file = tokio::fs::File::open(filepath).await?;
    let mut buffer = [0u8; 1024 * 64]; // 64KB chunks
    let mut chunk_index = 0;

    let mut framed = FramedWrite::new(send, LengthDelimitedCodec::new());

    while let Ok(n) = file.read(&mut buffer).await {
        if n == 0 {
            break;
        }

        let chunk_data = &buffer[..n];
        let compressed = zstd::encode_all(chunk_data, 3)?;

        let chunk = FileChunk {
            id: file_id.clone(),
            chunk_index,
            data: compressed,
        };

        let envelope = Envelope {
            payload: Some(Payload::FileChunk(chunk)),
        };

        let mut encoded = bytes::BytesMut::new();
        envelope.encode(&mut encoded)?;

        framed.send(encoded.freeze()).await?;
        chunk_index += 1;
    }

    let mut send = framed.into_inner();
    send.finish()?;
    Ok(())
}

pub async fn append_chunk(save_path: &Path, chunk: &FileChunk) -> Result<()> {
    use tokio::fs::OpenOptions;
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(save_path)
        .await?;

    let decompressed = zstd::decode_all(&chunk.data[..])?;
    file.write_all(&decompressed).await?;
    Ok(())
}
