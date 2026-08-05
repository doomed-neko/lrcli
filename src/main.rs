use std::{
    env::{self, Args},
    process::{ExitCode, exit},
};

use reqwest::{Method, blocking::Client};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const API_URL: &'static str = "https://lrclib.net/api";
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Lyric {
    id: i64,
    name: String,
    track_name: String,
    artist_name: String,
    album_name: String,
    duration: f64,
    instrumental: bool,
    plain_lyrics: String,
    synced_lyrics: String,
    #[serde(alias = "lyricsfile")]
    lyrics_file: String,
}

impl Lyric {
    pub fn by_song(
        track_name: String,
        artist: String,
        album: Option<String>,
        duration: Option<String>,
    ) -> Result<Lyric, LrcLibError> {
        let client = Client::new();
        let mut url = format!("{API_URL}/get?track_name={track_name}&artist_name={artist}");
        if let Some(album) = album {
            url += &album;
        }
        if let Some(duration) = duration {
            url += &duration;
        }
        let res = client.request(Method::GET, url).send()?;
        if res.status().as_u16() == 404 {
            return Err(LrcLibError::TrackNotFound {
                name: track_name.into(),
            });
        }
        serde_json::from_reader::<_, Lyric>(res).map_err(|err| err.into())
    }
}

#[derive(Error, Debug)]
enum LrcLibError {
    #[error("Could not find track with name {name:?}")]
    TrackNotFound { name: String },
    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Serde error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

fn print_usage_and_exit(program: &str) -> ! {
    println!("Find lyrics from https://lrclib.net");
    print!("Usage:");
    println!("\t{program} search -t <track_name> -a <artist> [-b album] [-d duration]");
    println!("Options:");
    println!("  -t, --track\t\tSet the track name");
    println!("  -a, --artist\t\tSet the artist name");
    println!("  -b, --album\t\tSet the album name");
    println!("  -d, --duration\tSet the duration (in seconds)");
    exit(1)
}

fn cmd_search(program: &str, args: Args) -> Result<(), ()> {
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
                let Some(current) = current_arg else {
                    eprintln!("ERRROR: Invalid arguments");
                    print_usage_and_exit(&program);
                };
                *current = Some(i);
                current_arg = None;
            }
        }
    }
    if track_name.is_none() || artist.is_none() {}
    let track_name = track_name.unwrap_or_else(|| {
        eprintln!("ERRROR: Track name must be provided");
        print_usage_and_exit(&program);
    });
    let artist = artist.unwrap_or_else(|| {
        eprintln!("ERRROR: Artist name must be provided");
        print_usage_and_exit(&program);
    });
    let lyrics =
        Lyric::by_song(track_name, artist, album, duration).map_err(|err| eprintln!("{err}"))?;
    println!("{}", lyrics.plain_lyrics);
    Ok(())
}

fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().unwrap_or("lrcli".into());
    let action = args
        .next()
        .unwrap_or_else(|| print_usage_and_exit(&program));
    match action.as_str() {
        "search" => {
            cmd_search(&program, args)?;
        }
        _ => {
            print_usage_and_exit(&program);
        }
    }

    Ok(())
}

fn main() -> ExitCode {
    match entry() {
        Ok(_) => ExitCode::SUCCESS,
        Err(_) => ExitCode::FAILURE,
    }
}
