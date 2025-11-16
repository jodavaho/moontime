//reexport time::serde::iso8601;

pub use time::serde::iso8601 as default_datetime_standard;
pub use time::OffsetDateTime as DateTime;

use serde::{Deserialize, Serialize};

use serde_json::json;

use std::f64::consts::PI;

pub fn default_datetime() -> DateTime {
    DateTime::now_utc()
}

/////////// FORMATTING
pub fn default_format() -> FormatSpecifier {
    FormatSpecifier::Txt
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum FormatSpecifier {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "txt")]
    Txt,
}

/////////// POSITION
pub const CADRE_LAT: f64 = 7.5;
pub const CADRE_LAT_RAD: f64 = CADRE_LAT * PI / 180.0;
pub const CADRE_LON: f64 = -59.0;
pub const CADRE_LON_RAD: f64 = CADRE_LON * PI / 180.0;

pub trait Angular {
    fn to_radians(&self) -> Self;
    fn to_degrees(&self) -> Self;
    fn units(&self) -> UnitSpecifier;
}

pub fn default_degrees() -> UnitSpecifier {
    UnitSpecifier::Degrees
}

impl Default for UnitSpecifier {
    fn default() -> Self {
        UnitSpecifier::Degrees
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum UnitSpecifier {
    #[serde(rename = "radians")]
    Radians,
    #[serde(rename = "degrees")]
    Degrees,
}

impl std::fmt::Display for UnitSpecifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UnitSpecifier::Radians => write!(f, "radians"),
            UnitSpecifier::Degrees => write!(f, "degrees"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Position {
    #[serde(default = "default_lat")]
    pub lat: f64,
    #[serde(default = "default_lon")]
    pub lon: f64,
    #[serde(default = "default_alt")]
    pub alt: f64,
    #[serde(default = "default_degrees")]
    pub units: UnitSpecifier,
}

impl Position {
    pub fn new(lat: f64, lon: f64, alt: f64, units: UnitSpecifier) -> Position {
        Position {
            lat,
            lon,
            alt,
            units,
        }
    }
    pub fn cadre() -> Position {
        Position {
            lat: CADRE_LAT,
            lon: CADRE_LON,
            alt: 0.0,
            units: UnitSpecifier::Degrees,
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        default_position()
    }
}

fn default_lat() -> f64 {
    CADRE_LAT
}

fn default_lon() -> f64 {
    CADRE_LON
}

fn default_alt() -> f64 {
    0.0
}

pub fn default_position() -> Position {
    Position {
        lat: default_lat(),
        lon: default_lon(),
        alt: default_alt(),
        units: default_degrees(),
    }
}

impl Angular for Position {
    fn to_degrees(&self) -> Position {
        match self.units {
            UnitSpecifier::Degrees => return *self,
            UnitSpecifier::Radians => {
                let lat = self.lat.to_degrees();
                let lon = self.lon.to_degrees();
                let alt = self.alt;
                Position {
                    lat,
                    lon,
                    alt,
                    units: UnitSpecifier::Degrees,
                }
            }
        }
    }
    fn to_radians(&self) -> Position {
        match self.units {
            UnitSpecifier::Radians => return *self,
            UnitSpecifier::Degrees => {
                let lat = self.lat.to_radians();
                let lon = self.lon.to_radians();
                let alt = self.alt;
                Position {
                    lat,
                    lon,
                    alt,
                    units: UnitSpecifier::Radians,
                }
            }
        }
    }
    fn units(&self) -> UnitSpecifier {
        self.units
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct RAzEl {
    pub az: f64,
    pub el: f64,
    pub r: f64,
    pub units: UnitSpecifier,
}

impl std::fmt::Display for RAzEl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "az: {}, el: {}, r: {}, u: {}",
            self.az, self.el, self.r, self.units
        )
    }
}

impl Angular for RAzEl {
    fn to_degrees(&self) -> RAzEl {
        match self.units {
            UnitSpecifier::Degrees => return *self,
            UnitSpecifier::Radians => {
                let az = self.az.to_degrees();
                let el = self.el.to_degrees();
                let r = self.r;
                RAzEl {
                    az,
                    el,
                    r,
                    units: UnitSpecifier::Degrees,
                }
            }
        }
    }
    fn to_radians(&self) -> RAzEl {
        match self.units {
            UnitSpecifier::Radians => return *self,
            UnitSpecifier::Degrees => {
                let az = self.az.to_radians();
                let el = self.el.to_radians();
                let r = self.r;
                RAzEl {
                    az,
                    el,
                    r,
                    units: UnitSpecifier::Radians,
                }
            }
        }
    }
    fn units(&self) -> UnitSpecifier {
        self.units
    }
}

pub fn translate_to<T: Serialize + Angular>(res: T, u: UnitSpecifier) -> T {
    match u {
        UnitSpecifier::Degrees => res.to_degrees(),
        UnitSpecifier::Radians => res.to_radians(),
    }
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub struct PositionFull {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub r: f64,
    pub lon: f64,
    pub lat: f64,
    pub units: UnitSpecifier,
}

impl std::fmt::Display for PositionFull {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "x: {} km, y: {} km, z: {} km, r: {} km, lon: {}, lat: {}, u: {}",
            self.x, self.y, self.z, self.r, self.lon, self.lat, self.units
        )
    }
}

impl Angular for PositionFull {
    fn to_degrees(&self) -> PositionFull {
        match self.units {
            UnitSpecifier::Degrees => *self,
            UnitSpecifier::Radians => PositionFull {
                x: self.x,
                y: self.y,
                z: self.z,
                r: self.r,
                lon: self.lon.to_degrees(),
                lat: self.lat.to_degrees(),
                units: UnitSpecifier::Degrees,
            },
        }
    }
    fn to_radians(&self) -> PositionFull {
        match self.units {
            UnitSpecifier::Radians => *self,
            UnitSpecifier::Degrees => PositionFull {
                x: self.x,
                y: self.y,
                z: self.z,
                r: self.r,
                lon: self.lon.to_radians(),
                lat: self.lat.to_radians(),
                units: UnitSpecifier::Radians,
            },
        }
    }
    fn units(&self) -> UnitSpecifier {
        self.units
    }
}

pub fn format_as<T: Serialize + std::fmt::Display>(
    res: T,
    f: FormatSpecifier,
    hint: Option<&str>,
) -> String {
    match (f, hint) {
        (FormatSpecifier::Json, None) => json!(res).to_string(),
        (FormatSpecifier::Json, Some(hint)) => json!({hint: res}).to_string(),
        (FormatSpecifier::Txt, _) => format!("{}", res),
    }
}
