mod readme;

use core::ffi::CStr;
use std::{
    env::set_var,
    f64::consts::PI,
    sync::{
        Arc, 
        Mutex
    },
};
use spice::{
    cstr,
    SpiceLock,
};
use serde::{Serialize, Deserialize};
use chrono::Utc;
use axum::{
    routing::get,
    extract::State,
    extract::Query,
    Router,
};
use lambda_http::{
    run ,Error
};

#[allow(dead_code)]
async fn get_readme( ) -> Result<String, String>
{
    Ok(readme::README.to_string())
}

const CADRE_LAT:f64 = 7.5;
const CADRE_LAT_RAD:f64 = CADRE_LAT * PI / 180.0;
const CADRE_LON:f64 = -59.0;
const CADRE_LON_RAD:f64 = CADRE_LON * PI / 180.0;

#[tokio::main]
async fn main() -> Result<(), Error>
{

    set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");

    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let cwd = std::env::current_dir().unwrap();
    println!("Current directory: {:?}", cwd);
    let cwd_contents = std::fs::read_dir(cwd).unwrap();
    for entry in cwd_contents {
        let entry = entry.unwrap();
        println!("{:?}", entry.path());
    }


    let in_lambda = std::env::var("LAMBDA_TASK_ROOT").is_ok();

    //let mut tlskernel = spice::furnsh("./latest_leapseconds.tls");
    let sl = SpiceLock::try_acquire().unwrap();
    sl.furnsh("data/latest_leapseconds.tls");
    //sl.furnsh("data/earth_fixed.tf");
    sl.furnsh("data/de440.bsp");
    //sl.furnsh("data/moon_080317.tf");
    //sl.furnsh("data/moon_assoc_me.tf");
    //sl.furnsh("data/moon_assoc_pa.tf");
    //sl.furnsh("data/earth_latest_high_prec.bpc");
    //sl.furnsh("data/moon_pa_de440_200625.bpc");
    sl.furnsh("data/moon_pa_de440_200625.bpc");
    sl.furnsh("data/moon_de440_200625.tf");
    sl.furnsh("data/pck00010.tpc");



    let tlskernel = 
        Arc::new(Mutex::new(sl));

    let _test_date:String = "2017-07-14T19:46:00+00:00".to_string();

    let app : Router
        = Router::new()
        .route("/s/et", get(get_et_time))
        .route("/s/cadre/solartime", get(cadre_get_solar_time))
        .route("/s/cadre/sun/azel", get(cadre_get_solar_azel))
        //.route("/cadre/daylighthours", get(get_daylight_hours))
        .route("/s/readme", get(get_readme))
        .with_state(tlskernel);

    if in_lambda{
        println!("Running in AWS Lambda");
        std::env::set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");
        run(app).await
    } else {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
        match axum::serve(listener,app).await
        {
            Ok(_) => {
                println!("Server started on port 3000");
                Ok(())
            },
            Err(e) => {
                eprintln!("Error starting server: {}", e);
                Err(Error::from(e))
            }
        }
    }

}

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Serialize, Deserialize)]
enum RetFormat{
    #[serde(rename = "json")]
    Json,
}

#[allow(dead_code)]
fn get_time(t:Option<String>) -> String{

    if t.is_none(){
        return Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    }
    println!("get_time t: {:?}", t);

    //match t.unwrap().parse::<chrono::DateTime<chrono::Utc>>(){
    match chrono::DateTime::parse_from_rfc3339(t.unwrap().as_str()){
        Ok(dt) => {
            dt.format("%Y-%m-%dT%H:%M:%S").to_string()
        },
        Err(e) => {
            eprintln!(">>Error parsing time: {}", e);
            Utc::now().format("%Y-%m-%dT%H:%M:%S").to_string()
        }
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
#[allow(dead_code)]
    async fn get_et_time(
        State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
        Query(q): Query<EtQuery>,
        )
         -> Result<String, ()>
{
    println!("et_time: {:?}", q);

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
#[allow(dead_code)]
async fn cadre_get_solar_time( 
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Query(q):Query<SolarTimeQuery> ,
    ) -> Result<String, ()>
{
    println!("solar_time: {:?}", q);
    let lock = sl_mutex.lock().unwrap();

    let dt = get_time(q.t); 
    let et:f64 = lock.str2et(dt.as_str());
    let result:String;
    unsafe
    {
        let et_c = et as f64;
        let body_c = 301 as i32;
        let lon_c = -59_f64.to_radians();
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

#[allow(dead_code)]
async fn cadre_get_solar_azel( 
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Query(q):Query<SolarTimeQuery> ,
    ) -> Result<String, ()>
{
    let lock = sl_mutex.lock().unwrap();
    let time = get_time(q.t);

    let mut radius = [0.0, 0.0, 0.0];

    unsafe{

        /*
           pub fn bodvrd_c(
           body: *mut ConstSpiceChar,
           item: *mut ConstSpiceChar,
           maxn: SpiceInt,
           dim: *mut SpiceInt,
           values: *mut SpiceDouble,
           );
           */
        
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
    let mut rect_coord = lock.georec(CADRE_LON_RAD, CADRE_LAT_RAD, 0.0, re, flat);
    println!("rect_coord: {:?}", rect_coord);

    let method = "ELLIPSOID";
    let abcorr = "NONE";
    let mut azlsta = [0.0;6];
    let mut lt = 0.0;

    unsafe{

        spice::c::azlcpo_c(
            cstr!(method),
            cstr!("SUN"),
            et,
            cstr!(abcorr),
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
    println!("range: {} azimuth: {} elevation: {}", 
             range, 
             azimuth.to_degrees(),
             elevation.to_degrees());
    Ok("".to_string())
}

