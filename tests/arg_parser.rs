use lrcli::cmd_search;

#[test]
#[should_panic]
fn test_cmd_search_missing_required_args() {
    let args = vec!["-t".to_string(), "Song Title".to_string()]; // Missing artist
    cmd_search("lrcli", args).unwrap();
}

#[test]
#[should_panic]
fn test_cmd_search_invalid_args() {
    let args = vec![
        "-t".to_string(),
        "-t".to_string(),
        "Song Title".to_string(),
        "-a".to_string(),
        "Artist".to_string(),
    ]; // Missing artist
    cmd_search("lrcli", args).unwrap();
}
