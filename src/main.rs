use std::fs;
use midly::{Smf, Header, Format, Timing};
use midly::number::u15;

fn main() {
    println!("Hello, world!");
    let data = fs::read("/Users/gulbrand/Desktop/midi/Vocal.mid").unwrap();
    let mut smf = Smf::parse(&data).unwrap();
    println!("{:?}", smf.header.format);
    println!("{:?}", smf.header.timing);
    println!("{} tracks", smf.tracks.len());
    for track in smf.tracks {
        for event in track.to_vec() {
            // println!("{:?}", event);
        }
    }

    let header = Header::new(
        Format::SingleTrack,
        Timing::Metrical(u15(480)));


}
