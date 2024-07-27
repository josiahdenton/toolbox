use std::path::Path;

use anyhow::Result;
use clap::{arg, command};
use ox_system::files;

/**
---- IDEAS ----

1. list all files in current dir (no recursive, no path option)
2. short mode + long mode (size, permissions)
3. nerd font icons for lang in both short + long mode
4. enable parsing input files for mark (fzf?)
*. crossterm for colors
*/

fn main() -> Result<()> {
    let matches = command!()
        .arg(arg!([path] "path directory to list files"))
        .arg(arg!(-d --depth <DEPTH> "depth to print the files"))
        .get_matches();

    let path = matches.get_one::<String>("path");

    let mut items = files::list_directory(Path::new(path.unwrap_or(&String::from("./"))));
    if let Some(depth) = matches.get_one::<String>("depth") {
        let depth = depth.parse::<u32>().unwrap_or(0);
        if depth > 0 {
            items.iter_mut().for_each(|item| item.with_children(depth));
        }
    }
    items.iter().for_each(|entry| println!("{entry}"));
    Ok(())
}
