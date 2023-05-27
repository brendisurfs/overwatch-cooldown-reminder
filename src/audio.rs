// list of damage audio:
// 4:14 Uptime and Damage
// probably should only use stuff on yt.
use std::io::BufReader;

// plays audio, duh.
use rodio::Sink;
pub fn play_audio_idk() {
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    let file = std::fs::File::open("assets/hello.mp3").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());
    sink.play();
    sink.sleep_until_end();
}
