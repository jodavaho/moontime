use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use serde::{
    Serialize,
    Deserialize,
};
use std::f64::consts::PI;

pub const CADRE_LAT:f64 = 7.5;
pub const CADRE_LAT_RAD:f64 = CADRE_LAT * PI / 180.0;
pub const CADRE_LON:f64 = -59.0;
pub const CADRE_LON_RAD:f64 = CADRE_LON * PI / 180.0;


#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct DateTime {
    #[serde(with = "time::serde::rfc3339", default = "OffsetDateTime::now_utc")]
    pub t: OffsetDateTime
}

impl DateTime {
    pub fn new() -> DateTime {
        DateTime {
            t: OffsetDateTime::now_utc()
        }
    }
    pub fn to_string(&self) -> String {
        let format=Rfc3339;
        self.t.format(&format).unwrap()
    }
}

fn default_format() -> FormatSpecifier {
    FormatSpecifier::Txt
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Format {
    #[serde(default = "default_format")]
    pub f: FormatSpecifier
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum FormatSpecifier {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "txt")]
    Txt,
}

pub fn default_degrees() -> UnitType {
    UnitType::Degrees
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct UnitsSpecifier {
    #[serde(default = "default_degrees")]
    pub u: UnitType,
}

impl std::fmt::Display for UnitType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UnitType::Radians => write!(f, "radians"),
            UnitType::Degrees => write!(f, "degrees"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum UnitType{
    #[serde(rename = "radians")]
    Radians,
    #[serde(rename = "degrees")]
    Degrees,
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Position {
    pub pos: Pos
}
impl Position {
    pub fn cadre() -> Position {
        Position {
            pos: Pos {
                lat: CADRE_LAT,
                lon: CADRE_LON,
                alt: 0.0
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize,Copy,Clone)]
pub struct Pos{
    #[serde(default = "default_lat")]
    pub lat: f64,
    #[serde(default = "default_lon")]
    pub lon: f64,
    #[serde(default = "default_alt")]
    pub alt: f64,
}

#[derive(Debug, Serialize, Deserialize,Copy,Clone)]
pub struct RAzEl {
    pub az: f64,
    pub el: f64,
    pub r: f64,
    pub units: UnitType
}
impl std::fmt::Display for RAzEl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "az: {}, el: {}, r: {}, u: {}", self.az, self.el, self.r, self.units)
    }
}
impl RAzEl {
    pub fn to_degrees(&self) -> RAzEl {
        let az = self.az.to_degrees();
        let el = self.el.to_degrees();
        let r = self.r;
        RAzEl {
            az,
            el,
            r,
            units: UnitType::Degrees
        }
    }
    pub fn to_radians(&self) -> RAzEl {
        let az = self.az;
        let el = self.el;
        let r = self.r;
        RAzEl {
            az,
            el,
            r,
            units: UnitType::Radians
        }
    }
}

