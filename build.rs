//! Set the bevy shield in README.md to the current bevy version
//!
//! I know this is completely overengineering things but here I am.
//!
//! this also creates a copy of readme.md in the temp dir with the
//! mock lib added so doctest works.

use std::{env, fs, path::PathBuf, str::FromStr};

use toml::Table;

fn main() {
    let dir = PathBuf::from_str(&env::var("CARGO_MANIFEST_DIR").unwrap()).unwrap();
    let cargo_manifest = fs::read_to_string(dir.join("Cargo.toml"))
        .unwrap()
        .parse::<Table>()
        .unwrap();
    let bevy_ver = cargo_manifest
        .get("dependencies")
        .unwrap()
        .as_table()
        .unwrap()
        .get("bevy")
        .unwrap()
        .as_table()
        .unwrap()
        .get("version")
        .unwrap()
        .as_str()
        .unwrap();

    let readme_path = dir.join("README.md");
    let mut readme = fs::read_to_string(&readme_path).unwrap();
    replace!(readme => "[Bevy version ",bevy_ver,"]");
    replace!(readme => "https://img.shields.io/badge/bevy-",bevy_ver,"-green");
    fs::write(readme_path, &readme).unwrap();

    let my_lib = "
    mod my_lib {
        use bevy_testing::p::*;

        #[derive(Component, Debug, PartialEq)]
        pub struct Countdown(pub u32);

        pub struct CountdownPlugin;

        impl Plugin for CountdownPlugin {
            fn build(&self, app: &mut App) {
                app.add_systems(Update, countdown_sys);
            }
        }

        fn countdown_sys(mut query: Query<&mut Countdown>) {
            for mut countdown in &mut query {
                countdown.0 -= 1;
            }
        }
    }
    ";

    let sanitized_readme = readme.replace("```rust", &("```rust\n".to_owned() + my_lib));
    fs::create_dir_all("temp").unwrap();
    fs::write("temp/readme.md", sanitized_readme).unwrap();
}

#[macro_export]
macro_rules! replace {
    ($string:expr => $start:literal,$replace:expr,$end:literal) => {
        $string.replace_range(
            {
                let start_str = $start;
                let start = $string.find(start_str).unwrap() + start_str.len();
                let end = $string[start..].find($end).unwrap() + start;
                start..end
            },
            $replace,
        )
    };
}
