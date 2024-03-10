use serde::Deserialize;
use sysinfo::{ProcessExt, SystemExt};
use crate::config::Config;

pub fn deserialize_usize<'de, D>(deserializer: D) -> Result<usize, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    usize::from_str_radix(&s[2..], 16).map_err(serde::de::Error::custom)
}

pub fn get_running_games() -> Vec<String> {
    let mut system = sysinfo::System::new_all();
    system.refresh_all();

    let process_names = vec![
        "東方紅魔郷",
        "th07.exe",
        "th08.exe",
        "th09.exe",
        "th10.exe",
        "th11.exe",
        "th12.exe",
        "th13.exe",
        "th14.exe",
        "th15.exe",
        "th16.exe",
        "th17.exe",
        "th18.exe",
    ];

    let mut running_games = vec![];

    for process_name in process_names {
        let pid = system.processes_by_name(process_name).map(|p| p.pid()).collect::<Vec<_>>();

        if pid.len() > 0 {
            running_games.push(process_name.to_string());
        }
    }

    running_games
}

pub fn get_touhou_game_name(process_name: &str) -> &str {

    match process_name {
        "東方紅魔郷" => "Touhou 06 - Embodiment of Scarlet Devil",
        "th07.exe" => "Touhou 07 - Perfect Cherry Blossom",
        "th08.exe" => "Touhou 08 - Imperishable Night",
        "th09.exe" => "Touhou 09 - Phantasmagoria of Flower View",
        "th10.exe" => "Touhou 10 - Mountain of Faith",
        "th11.exe" => "Touhou 11 - Subterranean Animism",
        "th12.exe" => "Touhou 12 - Undefined Fantastic Object",
        "th13.exe" => "Touhou 13 - Ten Desires",
        "th14.exe" => "Touhou 14 - Double Dealing Character",
        "th15.exe" => "Touhou 15 - Legacy of Lunatic Kingdom",
        "th16.exe" => "Touhou 16 - Hidden Star in Four Seasons",
        "th17.exe" => "Touhou 17 - Wily Beast and Weakest Creature",
        "th18.exe" => "Touhou 18 - Unconnected Marketeers",
        _ => "Unknown"
    }
}
