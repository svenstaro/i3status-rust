use crate::util;
use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Icons(pub HashMap<String, String>);

impl Default for Icons {
    fn default() -> Self {
        // "none" icon set
        Self(map! {
            "backlight_empty" => "BRIGHT",
            "backlight_full" => "BRIGHT",
            "backlight_1" =>  "BRIGHT",
            "backlight_2" =>  "BRIGHT",
            "backlight_3" =>  "BRIGHT",
            "backlight_4" =>  "BRIGHT",
            "backlight_5" =>  "BRIGHT",
            "backlight_6" =>  "BRIGHT",
            "backlight_7" =>  "BRIGHT",
            "backlight_8" =>  "BRIGHT",
            "backlight_9" =>  "BRIGHT",
            "backlight_10" => "BRIGHT",
            "backlight_11" => "BRIGHT",
            "backlight_12" => "BRIGHT",
            "backlight_13" => "BRIGHT",
            "bat_10" => "BAT",
            "bat_20" => "BAT",
            "bat_30" => "BAT",
            "bat_40" => "BAT",
            "bat_50" => "BAT",
            "bat_60" => "BAT",
            "bat_70" => "BAT",
            "bat_80" => "BAT",
            "bat_90" => "BAT",
            "bat_charging" => "CHG",
            "bat_discharging" => "DCG",
            "bat_empty" => "EMP",
            "bat_full" => "FULL",
            "bat_not_available" => "BAT N/A",
            "bell" => "ON",
            "bell-slash" => "OFF",
            "bluetooth" => "BT",
            "calendar" => "CAL",
            "cogs" => "LOAD",
            "cpu" => "CPU",
            "cpu_boost_on" => "BOOST ON",
            "cpu_boost_off" => "BOOST OFF",
            "disk_drive" => "DISK",
            "docker" => "DOCKER",
            "github" => "GITHUB",
            "gpu" => "GPU",
            "headphones" => "HEAD",
            "joystick" => "JOY",
            "keyboard" => "KBD",
            "mail" => "MAIL",
            "memory_mem" => "MEM",
            "memory_swap" => "SWAP",
            "mouse" => "MOUSE",
            "music" => "MUSIC",
            "music_next" => ">",
            "music_pause" => "||",
            "music_play" => ">",
            "music_prev" => "<",
            "net_bridge" => "BRIDGE",
            "net_down" => "DOWN",
            "net_loopback" => "LO",
            "net_modem" => "MODEM",
            "net_up" => "UP ",
            "net_vpn" => "VPN",
            "net_wired" => "ETH",
            "net_wireless" => "WLAN",
            "notification" => "NOTIF",
            "phone" => "PHONE",
            "phone_disconnected" => "PHONE",
            "ping" => "PING",
            "pomodoro" => "POMODORO",
            "pomodoro_break" => "BREAK",
            "pomodoro_paused" => "PAUSED",
            "pomodoro_started" => "STARTED",
            "pomodoro_stopped" => "STOPPED",
            "resolution" => "RES",
            "tasks" => "TSK",
            "thermometer" => "TEMP",
            "time" => "TIME",
            "toggle_off" => "OFF",
            "toggle_on" => "ON",
            "unknown" => "??",
            "update" => "UPD",
            "uptime" => "UP",
            "volume_empty" => "VOL",
            "volume_full" => "VOL",
            "volume_half" => "VOL",
            "volume_muted" => "VOL MUTED",
            "microphone_empty" => "MIC ",
            "microphone_full" => "MIC",
            "microphone_half" => "MIC",
            "microphone_muted" => "MIC MUTED",
            "weather_clouds" => "CLOUDY",
            "weather_default" => "WEATHER",
            "weather_rain" => "RAIN",
            "weather_snow" => "SNOW",
            "weather_sun" => "SUNNY",
            "weather_thunder" => "STORM",
            "xrandr" => "SCREEN"
        })
    }
}

impl Icons {
    pub fn from_file(file: &str) -> Option<Self> {
        if file == "none" {
            Some(Icons::default())
        } else {
            let file = util::find_file(file, Some("icons"), Some("toml"))?;
            Some(Icons(util::deserialize_toml_file(&file).ok()?))
        }
    }

    pub fn apply_overrides(&mut self, overrides: HashMap<String, String>) {
        self.0.extend(overrides);
    }
}

impl<'de> Deserialize<'de> for Icons {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Name,
            File,
            Overrides,
        }

        struct IconsVisitor;

        impl<'de> Visitor<'de> for IconsVisitor {
            type Value = Icons;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Icons")
            }

            /// Handle configs like:
            ///
            /// ```toml
            /// icons = "awesome"
            /// ```
            fn visit_str<E>(self, file: &str) -> Result<Icons, E>
            where
                E: de::Error,
            {
                Icons::from_file(file)
                    .ok_or_else(|| de::Error::custom(format!("Icon set '{}' not found.", file)))
            }

            /// Handle configs like:
            ///
            /// ```toml
            /// [icons]
            /// name = "awesome"
            /// ```
            fn visit_map<V>(self, mut map: V) -> Result<Icons, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut icons: Option<&str> = None;
                let mut overrides: Option<HashMap<String, String>> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name | Field::File => {
                            if icons.is_some() {
                                return Err(de::Error::duplicate_field("name or file"));
                            }
                            icons = Some(map.next_value()?);
                        }
                        Field::Overrides => {
                            if overrides.is_some() {
                                return Err(de::Error::duplicate_field("overrides"));
                            }
                            overrides = Some(map.next_value()?);
                        }
                    }
                }

                let mut icons = match icons {
                    Some(icons) => Icons::from_file(icons).ok_or_else(|| {
                        de::Error::custom(format!("Icon set '{}' not found", icons))
                    })?,
                    None => Icons::default(),
                };

                if let Some(overrides) = overrides {
                    for icon in overrides {
                        icons.0.insert(icon.0, icon.1);
                    }
                }
                Ok(icons)
            }
        }

        deserializer.deserialize_any(IconsVisitor)
    }
}
