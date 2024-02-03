#![deny(warnings)]

use core::ffi::CStr;
use std::f64::consts::PI;
use spice::cstr;
use warp::Filter;
use spice;
use spice::SpiceLock;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use chrono::Utc;

async fn readme( ) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!(
"Some spice web services related to my favorite space missions.

/et - returns the ephemeris time for the current time or a time specified in the t parameter.
      * t = e.g., '2021-10-01T12:00:00' is a string in RFC3339 format.
      * f = [json|None] is the format of the response. json returns extra information. If not specified, the response is a string.

/cadre/solar_time - returns the solar time at present, given CADRE's location. Currently, the location is notional. It'll be updated later.
"
    ))
}

#[tokio::main]
async fn main() {

    //let mut tlskernel = spice::furnsh("./latest_leapseconds.tls");
    let sl = SpiceLock::try_acquire().unwrap();
    sl.furnsh("data/latest_leapseconds.tls");
    //sl.furnsh("data/earth_fixed.tf");
    sl.furnsh("data/de440.bsp");
    //sl.furnsh("data/moon_080317.tf");
    //sl.furnsh("data/moon_assoc_me.tf");
    //sl.furnsh("data/moon_assoc_pa.tf");
    //sl.furnsh("data/earth_latest_high_prec.bpc");
    sl.furnsh("data/moon_pa_de440_200625.bpc");
    sl.furnsh("data/moon_de440_220930.tf");
    sl.furnsh("data/pck00010.tpc");



    let tlskernel = 
        Arc::new(Mutex::new(sl));

    //just test them. 
    get_et_time(
        EtQuery{f:Some(EtFormat::Json), t:Some(Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string())},
        Arc::clone(&tlskernel)).await.unwrap();
    get_solar_time(
        Arc::clone(&tlskernel),
        SolarTimeQuery{t:None}
        ).await.unwrap();
    get_daylight_hours(Arc::clone(&tlskernel)).await.unwrap();


    let et_time = warp::path!("et")
        .and(warp::get())
        .and(warp::query::<EtQuery>())
        .and(with_kernel(Arc::clone(&tlskernel)))
        .and_then(get_et_time);

    let solar_time = warp::path!("cadre"/"solartime")
        .and(warp::get())
        .and(with_kernel(Arc::clone(&tlskernel)))
        .and(warp::query::<SolarTimeQuery>())
        .and_then(get_solar_time);

    let daylight_hours = warp::path!("cadre"/"daylight")
        .and(warp::get())
        .and(with_kernel(Arc::clone(&tlskernel)))
        .and_then(get_daylight_hours);

    let readme = warp::path!("readme")
        .and_then(readme);

    let routes = 
        readme
        .or(et_time)
        .or(daylight_hours)
        .or(solar_time)
        ;

    println!("Starting server at 127.0.0.1");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn with_kernel(sl: Arc<Mutex<spice::SpiceLock>>) -> impl Filter<Extract = (Arc<Mutex<spice::SpiceLock>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || sl.clone())
}

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
enum RetFormat{
    #[serde(rename = "json")]
    Json,
}
fn get_time(t:Option<String>) -> String{
    match t{
        Some(t) => {
            let parsed_time = chrono::DateTime::parse_from_rfc3339(t.as_str());
            match parsed_time{
                Ok(dt) => {
                    dt.timestamp().to_string()
                },
                Err(e) => {
                    eprintln!("Error parsing time: {}", e);
                    Utc::now().timestamp().to_string()
                }
            }
        }
        None => Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
    }
}

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Deserialize)]
struct EtQuery
{
    f: Option<RetFormat>,
    t: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct EtResponse
{
    et: f64,
}
/// Returns the ephemeris time for the current time or a time specified in the t parameter.
/// t is a string in RFC3339 format.
/// f is the format of the response. If not specified, the response is a string.
/// If f is specified as json, the response is a json object.
async fn get_et_time(q:EtQuery, sl_mutex: Arc<Mutex<spice::SpiceLock>>) -> Result<impl warp::Reply, warp::Rejection> {
    println!("f: {:?}", q.f);
    println!("f: {:?}", q.t);

    let lock = sl_mutex.lock().unwrap();

    let dt = get_time(q.t); 

    let et:f64 = lock.str2et(dt.as_str());
    println!("et: {}", et);
    let response :String = match q.f{
        Some(RetFormat::Json) => {
            let er = EtResponse{et};
            serde_json::to_string(&er).unwrap()
        },
        None => {format!("{}", et)},
    };
    Ok(response)
}


///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Serialize, Deserialize)]
struct SolarTimeQuery{
    f: Option<RetFormat>,
    t: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct SolarTimeResponse{
    solartime: String,
}

//pub unsafe extern "C" fn et2lst_c( et: f64, body: i32, lon: f64, type_: *mut i8, timlen: i32, ampmlen: i32, hr: *mut i32, mn: *mut i32, sc: *mut i32, time: *mut i8, ampm: *mut i8)
/*     VARIABLE  I/O  DESCRIPTION */
/*     --------  ---  -------------------------------------------------- */
/*     ET         I   Epoch in seconds past J2000 epoch */
/*     BODY       I   ID-code of the body of interest */
/*     LON        I   Longitude of surface point (RADIANS) */
/*     TYPE       I   Type of longitude 'PLANETOCENTRIC', etc. */
/*     HR         O   Local hour on a "24 hour" clock */
/*     MN         O   Minutes past the hour */
/*     SC         O   Seconds past the minute */
/*     TIME       O   String giving local time on 24 hour clock */
/*     AMPM       O   String giving time on A.M./ P.M. scale */
async fn get_solar_time( sl_mutex: Arc<Mutex<spice::SpiceLock>>, q:SolarTimeQuery ) -> Result<impl warp::Reply, warp::Rejection> {
    println!("Getting solar time");
    let lock = sl_mutex.lock().unwrap();

    let dt = get_time(q.t); 
    let et:f64 = lock.str2et(dt.as_str());
    let result:String;
    unsafe
    {
        let et_c = et as f64;
        let body_c = 301 as i32;
        let lon_c = to_rad(-59_f64) as f64;
        let type_of_coord = "PLANETOCENTRIC"; 
        let type_c = cstr!(type_of_coord);
        const TIMLEN_C:i32 = 256 as i32;
        const AMPMLEN_C:i32 = 256 as i32;
        let mut hr_c:i32 = 0;
        let hr_cp = &mut hr_c as *mut i32;
        let mut mn_c:i32 = 0;
        let mn_cp = &mut mn_c as *mut i32;
        let mut sc_c:i32 = 0;
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
			ampm_c.as_mut_ptr()
        );
        let time = CStr::from_ptr(time_c.as_ptr()).to_str().unwrap();
        let ampm = CStr::from_ptr(ampm_c.as_ptr()).to_str().unwrap();
        eprintln!("solar time: {} {}", time, ampm);
        //cleanup
        result = format!("{}", ampm);
    }
    match q.f{
        Some(RetFormat::Json) => {
            let er = SolarTimeResponse{solartime:result};
            Ok(serde_json::to_string(&er).unwrap())
        },
        None => Ok(format!("{}", result))
    }

}

async fn get_daylight_hours( sl_mutex: Arc<Mutex<spice::SpiceLock>> ) -> Result<impl warp::Reply, warp::Rejection> {
    //https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/Tutorials/pdf/individual_docs/29_geometry_finder.pdf
    //or https://naif.jpl.nasa.gov/pub/naif/toolkit_docs/C/cspice/gfsep_c.html
    //spice::c::gfsep_c(
    let _lock = sl_mutex.lock().unwrap();
    Ok("solar angle")

}

fn to_rad(deg:f64) -> f64
{
    deg / 180.0 * PI
}
