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

fn to_rad(deg:f64) -> f64
{
    deg / 180.0 * PI
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
        EtQuery{f:Some(EtFormat::Json), m:Some(Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string())},
        Arc::clone(&tlskernel)).await.unwrap();
    get_solar_time(Arc::clone(&tlskernel)).await.unwrap();


    let et_time = warp::path!("et")
        .and(warp::get())
        .and(warp::query::<EtQuery>())
        .and(with_kernel(Arc::clone(&tlskernel)))
        .and_then(get_et_time);

    let solar_time = warp::path!("sun")
        .and(warp::get())
        .and(with_kernel(Arc::clone(&tlskernel)))
        .and_then(get_solar_time);

    let readme = warp::path!("readme")
        .and(warp::fs::file("./README.md"));

    let routes = readme
        .or(solar_time)
        .or(et_time)
        ;

    println!("Starting server at 127.0.0.1");
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;


}

fn with_kernel(sl: Arc<Mutex<spice::SpiceLock>>) -> impl Filter<Extract = (Arc<Mutex<spice::SpiceLock>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || sl.clone())
}

#[derive(Debug, Deserialize)]
enum EtFormat{
    #[serde(rename = "json")]
    Json,
}
#[derive(Debug, Deserialize)]
struct EtQuery
{
    f: Option<EtFormat>,
    m: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct EtResponse
{
    et: f64,
}
async fn get_et_time(q:EtQuery, sl_mutex: Arc<Mutex<spice::SpiceLock>>) -> Result<impl warp::Reply, warp::Rejection> {
    println!("f: {:?}", q.f);
    println!("f: {:?}", q.m);
    let lock = sl_mutex.lock().unwrap();
    let datestr= chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let et:f64 = lock.str2et(datestr.as_str());
    println!("et: {}", et);
    let response :String = match q.f{
        Some(EtFormat::Json) => {
            let er = EtResponse{et};
            serde_json::to_string(&er).unwrap()
        },
        None => {format!("{}", et)},
    };
    Ok(response)
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


async fn get_solar_time( sl_mutex: Arc<Mutex<spice::SpiceLock>> ) -> Result<impl warp::Reply, warp::Rejection> {
    let lock = sl_mutex.lock().unwrap();
    let datestr= chrono::Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let et:f64 = lock.str2et(datestr.as_str());
    unsafe
    {
        let et_c = et as f64;
        dbg!(et_c);
        let body_c = 301 as i32;
        dbg!(body_c);
        let lon_c = to_rad(-59_f64) as f64;
        dbg!(lon_c);
        let type_of_coord = "PLANETOCENTRIC"; 
        let type_c = cstr!(type_of_coord);
        dbg!(type_c);
        const TIMLEN_C:i32 = 256 as i32;
        dbg!(TIMLEN_C);
        const AMPMLEN_C:i32 = 256 as i32;
        dbg!(AMPMLEN_C);
        let mut hr_c:i32 = 0;
        let hr_cp = &mut hr_c as *mut i32;
        dbg!(hr_c);
        let mut mn_c:i32 = 0;
        let mn_cp = &mut mn_c as *mut i32;
        dbg!(mn_c);
        let mut sc_c:i32 = 0;
        let sc_cp = &mut sc_c as *mut i32;
        dbg!(sc_c);
        //make a buffer for the time string
        let mut time_c = [0i8; TIMLEN_C as usize];
        dbg!(time_c);
        let mut ampm_c = [0i8; AMPMLEN_C as usize];
        dbg!(ampm_c);
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
        eprintln!("time: {} {}", time, ampm);
        eprintln!("DONE!");
        //cleanup
        Ok("done")
    }
}
