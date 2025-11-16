pub mod types;
pub use types::*;

use core::ffi::CStr;
use spice::{cstr, SpiceLock};
use std::sync::{Arc, Mutex};

use time::{format_description::well_known::Rfc3339, OffsetDateTime};

fn to_cspice_string(t: OffsetDateTime) -> String {
    let format = Rfc3339;
    t.format(&format).unwrap()
}

pub fn get_et(sl_mutex: Arc<Mutex<SpiceLock>>, t: OffsetDateTime) -> f64 {
    let lock = sl_mutex.lock().unwrap();
    let dt = to_cspice_string(t);
    lock.str2et(dt.as_str())
}

#[allow(dead_code)]
pub fn solar_time(
    sl_mutex: Arc<Mutex<SpiceLock>>,
    t: OffsetDateTime,
    pos: types::Position,
) -> Result<String, ()> {
    let lock = sl_mutex.lock().unwrap();
    let lon = pos.to_radians().lon;
    let dt = to_cspice_string(t);
    let et: f64 = lock.str2et(dt.as_str());

    println!("et: {}", et);
    println!("lon: {}", lon);
    let result: String;
    unsafe {
        let et_c = et as f64;
        let body_c = 301 as i32;
        let lon_c = lon as f64;
        let type_of_coord = "PLANETOCENTRIC";
        let type_c = cstr!(type_of_coord);
        const TIMLEN_C: i32 = 256 as i32;
        const AMPMLEN_C: i32 = 256 as i32;
        let mut hr_c: i32 = 0;
        let hr_cp = &mut hr_c as *mut i32;
        let mut mn_c: i32 = 0;
        let mn_cp = &mut mn_c as *mut i32;
        let mut sc_c: i32 = 0;
        let sc_cp = &mut sc_c as *mut i32;
        //make a buffer for the time string
        let mut time_c = [0i8; TIMLEN_C as usize];
        let mut ampm_c = [0i8; AMPMLEN_C as usize];
        spice::c::et2lst_c(
            et_c,
            body_c,
            lon_c,
            type_c,
            TIMLEN_C,
            AMPMLEN_C,
            hr_cp,
            mn_cp,
            sc_cp,
            time_c.as_mut_ptr(),
            ampm_c.as_mut_ptr(),
        );
        let time = CStr::from_ptr(time_c.as_ptr()).to_str().unwrap();
        let ampm = CStr::from_ptr(ampm_c.as_ptr()).to_str().unwrap();
        println!("solar time: {} {}", time, ampm);
        //cleanup
        result = format!("{}", ampm);
    }
    Ok(result)
}

#[allow(dead_code)]
pub fn solar_azel(
    sl_mutex: Arc<Mutex<SpiceLock>>,
    time: OffsetDateTime,
    pos: types::Position,
) -> types::RAzEl {
    let lock = sl_mutex.lock().unwrap();
    let time = to_cspice_string(time);

    let mut radius = [0.0, 0.0, 0.0];

    unsafe {
        //TODO bodvrd_c should not borrow mutable strings here - can we upstream a fix?
        let mut out_dim: i32 = 0;
        let out_dim_p = &mut out_dim as *mut i32;
        spice::c::bodvrd_c(
            cstr!("MOON"),
            cstr!("RADII"),
            3,
            out_dim_p,
            radius.as_mut_ptr(),
        );
    }

    let re = radius[0];
    println!("re: {}", re);
    let et = lock.str2et(time.as_str());
    println!("et: {}", et);
    let flat = radius[0] - radius[2];
    let flat = flat / radius[0];

    let pos = pos.to_radians();
    println!(
        "pos: {}",
        serde_json::to_string(&pos).unwrap_or("err".to_string())
    );
    let mut rect_coord = lock.georec(pos.lon, pos.lat, pos.alt, re, flat);
    println!("rect_coord: {:?}", rect_coord);
    let mut azlsta = [0.0; 6];
    let mut lt = 0.0;

    unsafe {
        spice::c::azlcpo_c(
            cstr!("ELLIPSOID"),
            cstr!("SUN"),
            et,
            //TODO: provide aberration correction
            cstr!("NONE"),
            spice::c::SPICETRUE as i32,
            spice::c::SPICETRUE as i32,
            rect_coord.as_mut_ptr(),
            cstr!("MOON"),
            cstr!("MOON_ME_DE440_ME421"),
            azlsta.as_mut_ptr(),
            &mut lt,
        );
    }

    println!("azlsta: {:?} lt: {:?}", azlsta, lt);
    let range = azlsta[0];
    let azimuth = azlsta[1];
    let elevation = azlsta[2];
    println!(
        "range: {} azimuth: {} elevation: {} in degrees",
        range,
        azimuth.to_degrees(),
        elevation.to_degrees()
    );
    println!(
        "range: {} azimuth: {} elevation: {} in radians",
        range, azimuth, elevation
    );

    types::RAzEl {
        r: range,
        az: azimuth,
        el: elevation,
        units: types::UnitSpecifier::Radians,
    }
}

pub fn earth_position_from_sun(
    sl_mutex: Arc<Mutex<SpiceLock>>,
    time: OffsetDateTime,
) -> types::PositionFull {
    let lock = sl_mutex.lock().unwrap();
    let time_str = to_cspice_string(time);
    let et = lock.str2et(time_str.as_str());

    // Get Earth position from Sun in IAU_SUN frame (rectangular coords)
    let (mut pos, _lt) = lock.spkpos("EARTH", et, "IAU_SUN", "NONE", "SUN");

    let x = pos[0];
    let y = pos[1];
    let z = pos[2];

    // Convert rectangular to spherical (latitudinal) coordinates
    let mut r = 0.0;
    let mut lon = 0.0;
    let mut lat = 0.0;

    unsafe {
        spice::c::reclat_c(pos.as_mut_ptr(), &mut r, &mut lon, &mut lat);
    }

    types::PositionFull {
        x,
        y,
        z,
        r,
        lon,
        lat,
        units: types::UnitSpecifier::Radians,
    }
}

pub fn earth_position_ecliptic(
    sl_mutex: Arc<Mutex<SpiceLock>>,
    time: OffsetDateTime,
) -> types::PositionFull {
    let lock = sl_mutex.lock().unwrap();
    let time_str = to_cspice_string(time);
    let et = lock.str2et(time_str.as_str());

    // Get Earth position from Sun in ECLIPJ2000 frame (rectangular coords)
    let (mut pos, _lt) = lock.spkpos("EARTH", et, "ECLIPJ2000", "NONE", "SUN");

    let x = pos[0];
    let y = pos[1];
    let z = pos[2];

    // Convert rectangular to spherical (latitudinal) coordinates
    let mut r = 0.0;
    let mut lon = 0.0;
    let mut lat = 0.0;

    unsafe {
        spice::c::reclat_c(pos.as_mut_ptr(), &mut r, &mut lon, &mut lat);
    }

    types::PositionFull {
        x,
        y,
        z,
        r,
        lon,
        lat,
        units: types::UnitSpecifier::Radians,
    }
}

pub fn earth_position_galactic(
    sl_mutex: Arc<Mutex<SpiceLock>>,
    time: OffsetDateTime,
) -> types::PositionFull {
    let lock = sl_mutex.lock().unwrap();
    let time_str = to_cspice_string(time);
    let et = lock.str2et(time_str.as_str());

    // Get Earth position from Sun in GALACTIC frame (rectangular coords)
    let (mut pos, _lt) = lock.spkpos("EARTH", et, "GALACTIC", "NONE", "SUN");

    let x = pos[0];
    let y = pos[1];
    let z = pos[2];

    // Convert rectangular to spherical (latitudinal) coordinates
    let mut r = 0.0;
    let mut lon = 0.0;
    let mut lat = 0.0;

    unsafe {
        spice::c::reclat_c(pos.as_mut_ptr(), &mut r, &mut lon, &mut lat);
    }

    types::PositionFull {
        x,
        y,
        z,
        r,
        lon,
        lat,
        units: types::UnitSpecifier::Radians,
    }
}
