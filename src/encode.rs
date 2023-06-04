use tokio::sync::mpsc;

use crate::frame::Frame;

pub async fn encode_music(mut en_recv: mpsc::Receiver<Vec<Frame>>){
	println!("encodin!");

	loop {
		let frames = en_recv.recv().await.unwrap_or_default();
		println!("recieved {0} frames.", frames.len());
	}
}
