use google_cloud_speech::v1::speech_client::SpeechClient;
use google_cloud_speech::v1::{RecognitionConfig, RecognitionAudio};
use google_cloud_speech::v1::recognition_config::AudioEncoding;
use std::fs::File;
use std::io::Read;

pub async fn process_audio(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut client = SpeechClient::new().await?;

    let mut audio_file = File::open(file_path)?;
    let mut audio_content = Vec::new();
    audio_file.read_to_end(&mut audio_content)?;

    let audio = RecognitionAudio {
        audio_source: Some(AudioSource::Content(audio_content)),
    };

    let config = RecognitionConfig {
        encoding: AudioEncoding::Linear16 as i32,
        sample_rate_hertz: 16000,
        language_code: "en-US".to_string(),
        model: "phone_call".to_string(),
        ..Default::default()
    };

    let response = client.recognize(config, audio).await?;
    let result = &response.results[0];
    Ok(result.alternatives[0].transcript.clone())
}
