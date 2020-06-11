use std::{
	env,
	thread,
	boxed::Box,
	option::Option,
	time::Duration,
	convert::Infallible,
	sync::{ Arc, Mutex },
	fmt::{ Display, Result },
};

use std::net::{
	SocketAddr,
	TcpListener,
	TcpStream,
};

use hyper::{
	body::HttpBody as _,
	Client,
};

use hyper::service::{
	make_service_fn,
	service_fn,
};

use tokio::{
	prelude::*,
	io::{
		self, AsyncWrite as _
	},
};

use futures::future;

pub trait Cliental {
	type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

	fn log() -> Result<()>;
	async fn client() -> Result<()>;
}

impl dyn Cliental {
	async fn fetch() -> Result<()> {
		let client = Client::new();

		let mut res = client.get(url).await?;

		println!("Response: {}", res.status());
		println!("Headers: {:#?}\n", res.headers());

		while let Some(next)= res..data().await {
			let chunk = next?;
			tokio::io::stdout().write_all(&chunk?).await?;
		}

		Ok(())
	}

	#[tokio::main]
	fn cli() -> Result<()> {
		let url = match std::env::args().nth(1) {
			Some(url) => url,
			None => {
				println!("Client --> <url>");
				return Ok(());
			}
		};

		let url = url.parse::<hyper::Uri>().unwrap();

		if url.scheme_str() != Some("http") {
			println!("This client logger only works with 'http' URLs.");
			return Ok(());
		}

		fetch(url).await
	}
	//https://raw.githubusercontent.com/hyperium/hyper/master/examples/client.rs
}