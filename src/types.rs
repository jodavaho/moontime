#[macro_use]
mod iotype{
    macro_rules! default_fields {
        ( ) => {
            #[serde(default = "default_format")]
            pub f: FormatSpecifier
            #[serde(with = "time::serde::rfc3339", default = "OffsetDateTime::now_utc")]
            pub t: OffsetDateTime,
        };
    }
}

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

//a degree/radian trait
pub trait Angular{
    fn to_radians(&self) -> Self;
    fn to_degrees(&self) -> Self;
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct DateTime {
    #[serde(with = "time::serde::rfc3339", default = "OffsetDateTime::now_utc")]
    pub t: OffsetDateTime
}


pub fn default_datetime() -> OffsetDateTime{
    OffsetDateTime::now_utc()
}

impl DateTime {
    pub fn to_string(&self) -> String {
        let format=Rfc3339;
        self.t.format(&format).unwrap()
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Default for DateTime {
    fn default() -> Self {
        DateTime {
            t: default_datetime()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Format {
    #[serde(default = "default_format")]
    pub f: FormatSpecifier
}

fn default_format() -> FormatSpecifier {
    FormatSpecifier::Txt
}

impl Default for Format {
    fn default() -> Self {
        Format {
            f: default_format()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum FormatSpecifier {
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "txt")]
    Txt,
}

impl Default for FormatSpecifier {
    fn default() -> Self {
        default_format()
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Units{
    #[serde(default = "default_degrees")]
    pub u: UnitSpecifier
}

impl Default for Units {
    fn default() -> Self {
        Units {
            u: default_degrees()
        }
    }
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
pub enum UnitSpecifier{
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
    pub pos: Pos,
    pub units: UnitSpecifier
}

impl Position {
    pub fn cadre() -> Position {
        Position {
            pos: Pos {
                lat: CADRE_LAT,
                lon: CADRE_LON,
                alt: 0.0,
            },
            units: UnitSpecifier::Degrees
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        default_position()
    }
}

fn default_position() -> Position {
    Position {
        pos: Pos {
            lat: default_lat(),
            lon: default_lon(),
            alt: default_alt(),
        },
        units: default_degrees()
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "lat: {}, lon: {}, alt: {}", self.pos.lat, self.pos.lon, self.pos.alt)
    }
}

impl Angular for Position{
    fn to_degrees(&self) -> Position {
        match self.units {
            UnitSpecifier::Degrees => return *self,
            UnitSpecifier::Radians => {
                let lat = self.pos.lat.to_degrees();
                let lon = self.pos.lon.to_degrees();
                let alt = self.pos.alt;
                Position {
                    pos: Pos {
                        lat,
                        lon,
                        alt,
                    },
                    units: UnitSpecifier::Degrees
                }
            }
        }
    }
    fn to_radians(&self) -> Position {
        match self.units {
            UnitSpecifier::Radians => return *self,
            UnitSpecifier::Degrees => {
                let lat = self.pos.lat.to_radians();
                let lon = self.pos.lon.to_radians();
                let alt = self.pos.alt;
                Position {
                    pos: Pos {
                        lat,
                        lon,
                        alt,
                    },
                    units: UnitSpecifier::Radians
                }
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

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "lat: {}, lon: {}, alt: {}", self.lat, self.lon, self.alt)
    }
}


#[derive(Debug, Serialize, Deserialize,Copy,Clone)]
pub struct RAzEl {
    pub az: f64,
    pub el: f64,
    pub r: f64,
    pub units: UnitSpecifier
}

impl std::fmt::Display for RAzEl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "az: {}, el: {}, r: {}, u: {}", self.az, self.el, self.r, self.units)
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
                    units: UnitSpecifier::Degrees
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
                    units: UnitSpecifier::Radians
                }
            }
        }
    }
}

pub fn translate_res<T: Serialize + Angular >(res: T, u: UnitSpecifier) -> T {
    match u {
        UnitSpecifier::Degrees => res.to_degrees(),
        UnitSpecifier::Radians => res.to_radians()
    }
}

pub fn format_res<T: Serialize + std::fmt::Display>(res: T, f: FormatSpecifier) -> String {
    match f {
        FormatSpecifier::Json => serde_json::to_string(&res).unwrap(),
        FormatSpecifier::Txt => format!("{}", res)
    }
}
