mod readme;

use time::OffsetDateTime;
use chrono::Utc;

use moontime::*;

use std::{
    env::set_var,
    sync::{
        Arc, 
        Mutex
    },
};

use serde::{
    Serialize,
    Deserialize,
};
use spice::SpiceLock;
use axum::{
    routing::get,
    routing::post,
    extract::{
        State,
        Query,
        Json,
    },
    Router,
};

use lambda_http::{
    run ,Error
};

async fn get_readme( ) -> Result<String, String>
{
    Ok(readme::README.to_string())
}

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
    //sl.furnsh("data/de440.bsp");
    sl.furnsh("data/de440s.bsp");
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
        .route("/s/et", post(post_et_time))
        .route("/s/cadre/solartime", get(cadre_get_solar_time))
        .route("/s/cadre/solartime", post(cadre_post_solar_time))
        .route("/s/cadre/sun/azel", get(cadre_get_solar_azel))
        .route("/s/cadre/sun/azel", post(cadre_post_solar_azel))
        .route("/echo", get(get_echo))
        .route("/echo", post(post_echo))
        //.route("/cadre/daylighthours", get(get_daylight_hours))
        .route("/s/readme", get(get_readme))
        .route("/s/readme", post(get_readme))
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
                println!("Error starting server: {}", e);
                Err(Error::from(e))
            }
        }
    }

}

#[derive(Serialize, Deserialize, Debug)]
struct EchoBody {
    msg: String,
    #[serde(with = "time::serde::rfc3339", default = "OffsetDateTime::now_utc")]
    t: OffsetDateTime,
}

impl Default for EchoBody{
    fn default() -> Self {
        EchoBody {
            msg: "Hello, World!".to_string(),
            t: moontime::default_datetime(),
        }
    }
}

async fn get_echo(
    Query(body): Query<EchoBody>,
    ) -> Result<String, ()>
{
    Ok(body.msg)
}

async fn post_echo(
    Json(e): Json<EchoBody>
    ) -> Result<String, ()>
{
    let t = e.t;
    Ok( format!("{}", t.to_string()))
}

/// Returns the ephemeris time for the current time or a time specified in the t parameter.
/// t is a string in RFC3339 format.
/// f is the format of the response. If not specified, the response is a string.
/// If f is specified as json, the response is a json object.
    async fn get_et_time(
        State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
        Query(t): Query<DateTime>,
        Query(f): Query<Format>,
        )
         -> Result<String, ()>
{
    let res = moontime::get_et(sl_mutex, t);
    Ok(moontime::format_res(res,f.f))
}

#[derive(Serialize, Deserialize, Debug)]
struct EtBody {
    t: Option<DateTime>,
    f: Option<Format>,
}
impl Default for EtBody {
    fn default() -> Self {
        EtBody {
            t: None,
            f: None,
        }
    }
}

    async fn post_et_time(
        State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
        obody: Option<Json<EtBody>>,
        )
         -> Result<String, ()>
{
    let Json(body) = obody.unwrap_or_default();
    let t = body.t.unwrap_or_default();
    let f = body.f.unwrap_or_default();

    let res = moontime::get_et(sl_mutex, t);
    Ok(moontime::format_res(res,f.f))
}

#[derive(Serialize, Deserialize, Debug)]
struct PostSolarTime {
    t: Option<DateTime>,
    f: Option<Format>,
    p: Option<Position>,
}
impl Default for PostSolarTime {
    fn default() -> Self {
        PostSolarTime {
            t: None,
            f: None,
            p: None,
        }
    }
}
async fn cadre_post_solar_time( 
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    obody: Option<Json<PostSolarTime>>,
    ) -> Result<String, ()>
{
    let Json(body) = obody.unwrap_or_default();
    let t = body.t.unwrap_or_default();
    let f = body.f.unwrap_or_default();
    let p = body.p.unwrap_or_default();

    let result = moontime::solar_time(sl_mutex, t, p.to_radians()).unwrap();
    Ok(moontime::format_res(result, f.f))
}

async fn cadre_get_solar_time( 
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    t: Option<Query<DateTime>>,
    f: Option<Query<Format>>,
    ) -> Result<String, ()>
{
    let pos = Position::default();
    let Query(t) = t.unwrap_or_default();
    let Query(f) = f.unwrap_or_default();
    let result = moontime::solar_time(sl_mutex, t, pos).unwrap();
    Ok(moontime::format_res(result, f.f))
}

#[derive(Serialize, Deserialize, Debug)]
struct CadrePostSolarAzel {
    t: Option<DateTime>,
    f: Option<FormatSpecifier>,
    u: Option<UnitSpecifier>,
    pos: Option<Position>,
}
impl Default for CadrePostSolarAzel {
    fn default() -> Self {
        CadrePostSolarAzel {
            t: None,
            f: None,
            u: None,
            pos: None,
        }
    }
}

async fn cadre_post_solar_azel( 
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    obody: Option<Json<CadrePostSolarAzel>>,
    ) -> Result<String, ()>
{
    let body = obody.unwrap_or_default();
    let time = body.t.unwrap_or_default();
    let format = body.f.unwrap_or_default();
    let units = body.u.unwrap_or_default();
    let pos = body.pos.unwrap_or_default();

    println!("time: {:?}", time);
    println!("format: {:?}", format);
    println!("units: {:?}", units);
    println!("pos: {:?}", pos);

    let res = moontime::solar_azel(sl_mutex, time, pos);
    let res = moontime::translate_res(res, units);
    let res = moontime::format_res(res, format);
    Ok(res)
}

async fn cadre_get_solar_azel( 
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    time: Option<Query<DateTime>>,
    format: Option<Query<Format>>,
    units: Option<Query<Units>>,
    ) -> Result<String, ()>
{

    let pos = Position::default();
    let Query(time) = time.unwrap_or_default();
    let format = format.unwrap_or_default().f;
    let units = units.unwrap_or_default().u;

    let res = moontime::solar_azel(sl_mutex, time, pos);
    let res = moontime::translate_res(res, units);
    let res = moontime::format_res(res, format);
    Ok(res)
}

