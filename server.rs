use std::{
    future::Future,
    io,
    net::{Ipv4Addr, SocketAddr, TcpListener},
};

use axum::{
    http::{header, HeaderValue},
    Router,
};


use tower_http::set_header::SetResponseHeaderLayer;

use socket2::{Domain, Socket, Type};

/// Reuse an existing listener, ensuring that the socket `backlog``
/// is set to enable a higher number of pending connections.
fn set_socket_options(addr: SocketAddr) -> io::Result<tokio::net::TcpListener> {
    let socket = match addr {
        SocketAddr::V4(_) => Socket::new(Domain::IPV4, Type::STREAM, None)?,
        SocketAddr::V6(_) => Socket::new(Domain::IPV6, Type::STREAM, None)?,
    };

    socket.set_reuse_port(true)?;
    socket.set_reuse_address(true)?;
    socket.set_nonblocking(true)?;
    socket.set_nodelay(true)?;
    socket.bind(&addr.into())?;
    socket.listen(4096)?;

    let listener: TcpListener = socket.into();
    tokio::net::TcpListener::from_std(listener)
}

/// Build an Axum server with consistent configuration, using the high-level API exposed
/// by Axum 0.8. This is intended for convenience and intentionally does not provide much
/// customisability.
#[allow(dead_code)]
pub async fn serve(app: Router<()>, port: Option<u16>) {
    let port = port.unwrap_or(8000);
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, port));
    let listener = set_socket_options(addr).expect("couldn't bind to address");
    println!("started axum server on port {port}.");

    let server_header_value = HeaderValue::from_static("Axum");
    let app = app.layer(SetResponseHeaderLayer::overriding(
        header::SERVER,
        server_header_value,
    ));

    axum::serve(listener, app.into_make_service()).await.unwrap();
}

/// Start a single-threaded tokio runtime on multiple threads.
#[allow(dead_code)]
pub fn start_tokio<Fut>(f: fn() -> Fut)
where
    Fut: Future<Output = ()> + 'static,
{
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    for _ in 1..num_cpus::get()  {
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(f());
        });
    }
    rt.block_on(f());
}