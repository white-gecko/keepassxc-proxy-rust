use std::env;
use std::io::{self, Read, Write};

#[cfg(not(windows))]
use std::os::unix::net::UnixStream;

#[cfg(windows)]
use named_pipe::PipeClient;

pub struct ProxySocket<T> {
	inner: T,
}

impl<R: Read> Read for ProxySocket<R> {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.inner.read(buf)
	}
}

impl<W: Write> Write for ProxySocket<W> {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		self.inner.write(buf)
	}

	fn flush(&mut self) -> io::Result<()> {
		self.inner.flush()
	}
}

#[cfg(windows)]
pub fn connect() -> io::Result<ProxySocket<PipeClient>> {
	let temp_path = env::var("TEMP").unwrap();
	let pipe_name = format!("\\\\.\\pipe\\{}\\kpxc_server", temp_path);
	let client = PipeClient::connect(pipe_name)?;
	Ok(ProxySocket { inner: client })
}

#[cfg(not(windows))]
pub fn connect() -> io::Result<ProxySocket<UnixStream>> {
	use std::time::Duration;

	let socket_name = "kpxc_server";
	let socket: String;
	if let Ok(xdg) = env::var("XDG_RUNTIME_DIR") {
		socket = format!("{}/{}", xdg, socket_name);
	} else {
		socket = format!("/tmp/{}", socket_name);
	}
	let s = UnixStream::connect(socket)?;
	let timeout: Option<Duration> = Some(Duration::from_secs(1));
	s.set_read_timeout(timeout)?;
	Ok(ProxySocket { inner: s })
}
