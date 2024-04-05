pub mod types;
pub use types::*;


use core::ffi::CStr;
use std::sync::{ Arc, Mutex };
use spice::{
    cstr,
    SpiceLock,
};

use time::{
    OffsetDateTime,
    format_description::well_known::Rfc3339,
};

fn to_cspice_string(t : OffsetDateTime) -> String {
    let format=Rfc3339;
    t.format(&format).unwrap()
}

pub fn get_et(
    sl_mutex: Arc<Mutex<SpiceLock>>, 
    t: OffsetDateTime,
    ) -> f64 
{
    let lock = sl_mutex.lock().unwrap();
    let dt = to_cspice_string(t);
    lock.str2et(dt.as_str())
}

pub fn solar_time(
    sl_mutex: Arc<Mutex<SpiceLock>>,
    t: OffsetDateTime,
    pos: types::Position,
    )
     -> Result<String, ()>
{
    println!("solar_time");
    let lock = sl_mutex.lock().unwrap();
    println!("lock");
    let dt = to_cspice_string(t);
    println!("dt: {}", dt);
    let et = lock.str2et(dt.as_str());
    println!("et: {}", et);
    let body_code = 301;
    let lon = pos.lon.to_radians();
    let lon_type = "PLANETOCENTRIC";

    println!("et: {}", et);
    let (_,_,_,_,ampm) = lock.et2lst(et, body_code, lon, lon_type);
    println!("ampm: {}", ampm);
    Ok(ampm)
}

#[allow(dead_code)]
pub fn solar_azel( 
    sl_mutex: Arc<Mutex<SpiceLock>>,
    time : OffsetDateTime,
    pos: types::Position,
    ) -> types::RAzEl
{
    let lock = sl_mutex.lock().unwrap();
    let time = to_cspice_string(time);

    let mut radius = [0.0, 0.0, 0.0];

    unsafe{
        //TODO bodvrd_c should not borrow mutable strings here - can we upstream a fix?
        let mut out_dim: i32 = 0;
        let out_dim_p = &mut out_dim as *mut i32;
        spice::c::bodvrd_c(
            cstr!("MOON"),
            cstr!("RADII"),
            3, 
            out_dim_p, 
            radius.as_mut_ptr());
    }

    let re = radius[0];
    println!("re: {}", re);
    let et = lock.str2et(time.as_str());
    println!("et: {}", et);
    let flat = radius[0] - radius[2];
    let flat = flat / radius[0];


    let pos = pos.to_radians();
    println!("pos: {}", serde_json::to_string(&pos).unwrap_or("err".to_string()));
    let mut rect_coord = lock.georec(pos.lon, pos.lat, pos.alt, re, flat);
    println!("rect_coord: {:?}", rect_coord);
    let mut azlsta = [0.0;6];
    let mut lt = 0.0;

    unsafe{

        spice::c::azlcpo_c(
            cstr!("ELLIPSOID"),
            cstr!("SUN"),
            et,
            cstr!("NONE"), //TODO: provide aberration correction
            spice::c::SPICETRUE as i32,
            spice::c::SPICETRUE as i32,
            rect_coord.as_mut_ptr(),
            cstr!("MOON"),
            cstr!("MOON_ME_DE440_ME421"),
            azlsta.as_mut_ptr(),
            &mut lt
            );
    }

    println!("azlsta: {:?} lt: {:?}", azlsta, lt);
    let range = azlsta[0];
    let azimuth = azlsta[1];
    let elevation = azlsta[2];
    println!("range: {} azimuth: {} elevation: {} in degrees", 
             range, 
             azimuth.to_degrees(),
             elevation.to_degrees());
    println!("range: {} azimuth: {} elevation: {} in radians", 
             range, 
             azimuth,
             elevation);

    types::RAzEl{
        r:range, 
        az:azimuth,
        el:elevation,
        units: types::UnitSpecifier::Radians
    }
}


#[allow(dead_code)]
pub fn sun_path(
    sl_mutex: Arc<Mutex<SpiceLock>>,
    t: OffsetDateTime,
    pos: types::Position,
    )
     -> Result<Vec<types::RAzEl>, ()>
{
    let mut razels:Vec<types::RAzEl> = Vec::new();
    for i in -14..14 {
        let ti = t + time::Duration::days(i);
        let this_razel = solar_azel(sl_mutex.clone(), ti, pos);
        razels.push(this_razel);
    }
    for i in -24..24 {
        let ti = t + time::Duration::hours(2*i);
        let this_razel = solar_azel(sl_mutex.clone(), ti, pos);
        razels.push(this_razel);
    }
    Ok(razels)
}
