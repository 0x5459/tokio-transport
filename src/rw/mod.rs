use tokio::io::{AsyncRead, AsyncWrite};
use tokio_util::codec::Framed as TokioFramed;

#[cfg(feature = "rw-pipe")]
pub mod pipe;

impl<T: ?Sized> ReadWriter for T where T: AsyncRead + AsyncWrite {}

pub trait ReadWriter: AsyncRead + AsyncWrite {}

impl<T: ?Sized> ReadWriterExt for T where T: ReadWriter {}

pub trait ReadWriterExt: ReadWriter {
    fn framed<Codec>(self, frame_codec: Codec) -> TokioFramed<Self, Codec>
    where
        Self: Sized,
    {
        TokioFramed::new(self, frame_codec)
    }

    fn framed_default<Codec>(self) -> TokioFramed<Self, Codec>
    where
        Self: Sized,
        Codec: Default,
    {
        self.framed(Codec::default())
    }
}
