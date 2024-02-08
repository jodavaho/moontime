mod readme;

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
struct GetEtTime {
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
    #[serde(default = "default_format")]
    f: FormatSpecifier,
}

    async fn get_et_time(
        State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
        Query(GetEtTime {t, f}): Query<GetEtTime>
        )
    -> Result<String, ()>
{
    println!("t: {:?}, f: {:?}", t, f);
    let res = moontime::get_et(sl_mutex, t);
    Ok(moontime::format_as(res,f))
}

#[derive(Serialize, Deserialize, Debug)]
struct EtBody {
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    pub t: DateTime,
    #[serde(default = "default_format")]
    pub f: FormatSpecifier,
}

    async fn post_et_time(
        State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
        Json(EtBody {t, f}): Json<EtBody>
        )
         -> Result<String, ()>
{
    println!("t: {:?}, f: {:?}", t, f);
    let res = moontime::get_et(sl_mutex, t);
    Ok(moontime::format_as(res,f))
}

#[derive(Serialize, Deserialize, Debug)]
struct PostSolarTime {
    #[serde(default = "default_format")]
    f: FormatSpecifier,
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
    #[serde(default = "default_position")]
    p: Position,
}
async fn cadre_post_solar_time( 
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Json(PostSolarTime{f, t, p}): Json<PostSolarTime>
    ) -> Result<String, ()>
{
    println!("t: {:?}, f: {:?}, p: {:?}", t, f, p);
    let result = moontime::solar_time(sl_mutex, t, p.to_radians()).unwrap();
    Ok(moontime::format_as(result, f))
}

#[derive(Serialize, Deserialize, Debug)]
struct SolarTimeQuery{
    #[serde(default = "default_format")]
    f: FormatSpecifier,
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
    #[serde(default = "default_position")]
    p: Position,
}

async fn cadre_get_solar_time( 
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Query(SolarTimeQuery{t, f, p}): Query<SolarTimeQuery>,
    ) -> Result<String, ()>
{
    println!("t: {:?}, f: {:?}, p: {:?}", t, f, p);
    let result = moontime::solar_time(sl_mutex, t, p).unwrap();
    Ok(moontime::format_as(result, f))
}

#[derive(Serialize, Deserialize, Debug)]
struct CadrePostSolarAzel {
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
    #[serde(default = "default_format")]
    f: FormatSpecifier,
    #[serde(default = "default_degrees")]
    u: UnitSpecifier,
    #[serde(default = "default_position")]
    p: Position,

}

async fn cadre_post_solar_azel( 
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Json(CadrePostSolarAzel{t, f, u, p}): Json<CadrePostSolarAzel>
    ) -> Result<String, ()>
{

    println!("time: {:?}", t);
    println!("format: {:?}", f);
    println!("units: {:?}", u);
    println!("pos: {:?}", p);

    let res = moontime::solar_azel(sl_mutex, t, p);
    let res = moontime::translate_to(res, u);
    let res = moontime::format_as(res, f);
    Ok(res)
}

#[derive(Serialize, Deserialize, Debug)]
struct QuerySolarAzel {
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
    #[serde(default = "default_format")]
    f: FormatSpecifier,
    #[serde(default = "default_degrees")]
    u: UnitSpecifier,
    #[serde(default = "default_position")]
    p: Position,
}

async fn cadre_get_solar_azel( 
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Query(QuerySolarAzel{t, f, u, p}): Query<QuerySolarAzel>
    ) -> Result<String, ()>
{

    println!("time: {:?}", t);
    println!("format: {:?}", f);
    println!("units: {:?}", u);
    println!("pos: {:?}", p);
    let res = moontime::solar_azel(sl_mutex, t, p);
    let res = moontime::translate_to(res, u);
    let res = moontime::format_as(res, f);
    Ok(res)
}

