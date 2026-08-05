pub use crate::helpers::print_usage;
use crate::lyric::Lyric;

mod error;
mod helpers;
mod lyric;

pub const API_URL: &'static str = "https://lrclib.net/api";

pub fn cmd_search(program: &str, args: Vec<String>) -> Result<(), ()> {
    let mut track_name = None;
    let mut artist = None;
    let mut album = None;
    let mut duration = None;
    let mut current_arg = None;
    for i in args {
        match i.as_str() {
            "-t" | "--track" => current_arg = Some(&mut track_name),
            "-a" | "--artist" => current_arg = Some(&mut artist),
            "-b" | "--album" => current_arg = Some(&mut album),
            "-d" | "--duration" => current_arg = Some(&mut duration),
            _ => {
                let current = current_arg.ok_or_else(|| {
                    eprintln!("ERRROR: Invalid arguments");
                    print_usage(&program);
                })?;
                *current = Some(i);
                current_arg = None;
            }
        }
    }
    if track_name.is_none() || artist.is_none() {}
    let track_name = track_name.ok_or_else(|| {
        eprintln!("ERRROR: Track name must be provided");
        print_usage(&program);
    })?;
    let artist = artist.ok_or_else(|| {
        eprintln!("ERRROR: Artist name must be provided");
        print_usage(&program);
    })?;
    let lyrics =
        Lyric::by_song(track_name, artist, album, duration).map_err(|err| eprintln!("{err}"))?;
    println!("{}", lyrics.plain_lyrics);
    Ok(())
}
