pub fn print_usage(program: &str) {
    println!("Find lyrics from https://lrclib.net");
    print!("Usage:");
    println!("\t{program} search -t <track_name> -a <artist> [-b album] [-d duration]");
    println!("Options:");
    println!("  -t, --track\t\tSet the track name");
    println!("  -a, --artist\t\tSet the artist name");
    println!("  -b, --album\t\tSet the album name");
    println!("  -d, --duration\tSet the duration (in seconds)");
}
