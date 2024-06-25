mod ai_handler;
mod audio_processor;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let audio_file = "audio_samples/crushed.wav";
    match audio_processor::process_audio(audio_file).await {
        Ok(transcript) => {
            ai_handler::handle_transcript(transcript).await;
        }
        Err(e) => eprintln!("Audio processing failed: {}", e),
    }
}
