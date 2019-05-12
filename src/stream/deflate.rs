use core::{
    pin::Pin,
    task::{Context, Poll},
};
use std::io::Result;
use std::marker::Unpin;

use super::flate::{FlateDecoder, FlateEncoder};
use bytes::Bytes;
use flate2::{Compress, Compression, Decompress};
use futures::{stream::Stream, stream::StreamExt};

/// A DEFLATE encoder, or compressor.
///
/// This structure implements a [`Stream`] interface and will read uncompressed data from an
/// underlying stream and emit a stream of compressed data.
pub struct DeflateEncoder<S: Stream<Item = Result<Bytes>> + Unpin> {
    inner: FlateEncoder<S>,
}

/// A DEFLATE decoder, or decompressor.
///
/// This structure implements a [`Stream`] interface and will read compressed data from an
/// underlying stream and emit a stream of uncompressed data.
pub struct DeflateDecoder<S: Stream<Item = Result<Bytes>> + Unpin> {
    inner: FlateDecoder<S>,
}

impl<S: Stream<Item = Result<Bytes>> + Unpin> DeflateEncoder<S> {
    /// Creates a new encoder which will read uncompressed data from the given stream and emit a
    /// compressed stream.
    pub fn new(stream: S, level: Compression) -> DeflateEncoder<S> {
        DeflateEncoder {
            inner: FlateEncoder::new(stream, Compress::new(level, false)),
        }
    }
}

impl<S: Stream<Item = Result<Bytes>> + Unpin> DeflateDecoder<S> {
    /// Creates a new decoder which will read compressed data from the given stream and emit an
    /// uncompressed stream.
    pub fn new(stream: S) -> DeflateDecoder<S> {
        DeflateDecoder {
            inner: FlateDecoder::new(stream, Decompress::new(false)),
        }
    }
}

impl<S: Stream<Item = Result<Bytes>> + Unpin> Stream for DeflateEncoder<S> {
    type Item = Result<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Bytes>>> {
        self.inner.poll_next_unpin(cx)
    }
}

impl<S: Stream<Item = Result<Bytes>> + Unpin> Stream for DeflateDecoder<S> {
    type Item = Result<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Result<Bytes>>> {
        self.inner.poll_next_unpin(cx)
    }
}