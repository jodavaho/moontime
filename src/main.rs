mod readme;

use moontime::*;

use std::{
    env::set_var,
    sync::{Arc, Mutex},
};

use axum::{
    extract::{Json, Path, Query, State},
    routing::get,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use spice::SpiceLock;

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum CoordFormat {
    Xyz,
    Spherical,
    Azel,
}

use lambda_http::{run, Error};

async fn get_readme() -> Result<String, String> {
    Ok(format!(
        "\nVersion: {}\nAuthor: {}\nHomepage: {}\n\n{}",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE"),
        readme::README,
    ))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
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

    let tlskernel = Arc::new(Mutex::new(sl));

    let app: Router = Router::new()
        .route("/s/et", get(get_et_time))
        .route("/s/et", post(post_et_time))
        .route("/s/moon/solartime", get(moon_get_solar_time))
        .route("/s/moon/solartime", post(moon_post_solar_time))
        .route("/s/moon/sun", get(moon_get_sun_azel))
        .route("/s/moon/sun", post(moon_post_sun_azel))
        .route("/s/moon/sun/:format", get(moon_get_sun))
        .route("/s/moon/sun/:format", post(moon_post_sun))
        .route("/s/cadre/solartime", get(cadre_get_solar_time))
        .route("/s/cadre/solartime", post(cadre_post_solar_time))
        .route("/s/cadre/sun", get(cadre_get_sun_azel))
        .route("/s/cadre/sun", post(cadre_post_sun_azel))
        .route("/s/cadre/sun/:format", get(cadre_get_sun))
        .route("/s/cadre/sun/:format", post(cadre_post_sun))
        .route("/s/sun/earth", get(get_sun_earth_full))
        .route("/s/sun/earth", post(post_sun_earth_full))
        .route("/s/sun/earth/:format", get(get_sun_earth))
        .route("/s/sun/earth/:format", post(post_sun_earth))
        .route("/s/ecliptic/earth", get(get_ecliptic_earth_full))
        .route("/s/ecliptic/earth", post(post_ecliptic_earth_full))
        .route("/s/ecliptic/earth/:format", get(get_ecliptic_earth))
        .route("/s/ecliptic/earth/:format", post(post_ecliptic_earth))
        //.route("/cadre/daylighthours", get(get_daylight_hours))
        .route("/s/readme", get(get_readme))
        .route("/s/readme", post(get_readme))
        .with_state(tlskernel);

    if in_lambda {
        println!("Running in AWS Lambda");
        std::env::set_var("AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH", "true");
        run(app).await
    } else {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap();
        match axum::serve(listener, app).await {
            Ok(_) => {
                println!("Server started on port 3000");
                Ok(())
            }
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
    Query(GetEtTime { t, f }): Query<GetEtTime>,
) -> Result<String, ()> {
    println!("t: {:?}, f: {:?}", t, f);
    let res = moontime::get_et(sl_mutex, t);
    Ok(moontime::format_as(res, f, Some("et")))
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
    Json(EtBody { t, f }): Json<EtBody>,
) -> Result<String, ()> {
    println!("t: {:?}, f: {:?}", t, f);
    let res = moontime::get_et(sl_mutex, t);
    Ok(moontime::format_as(res, f, Some("et")))
}

#[derive(Serialize, Deserialize, Debug)]
struct MoonSolarTime {
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
    #[serde(default = "default_format")]
    f: FormatSpecifier,
    p: Position,
}
async fn moon_post_solar_time(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Json(MoonSolarTime { f, t, p }): Json<MoonSolarTime>,
) -> Result<String, ()> {
    let result = moontime::solar_time(sl_mutex, t, p.to_radians()).unwrap();
    Ok(moontime::format_as(result, f, Some("solar time")))
}

async fn moon_get_solar_time(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Query(MoonSolarTime { t, f, p }): Query<MoonSolarTime>,
) -> Result<String, ()> {
    println!("t: {:?}, f: {:?}, p: {:?}", t, f, p);
    let result = moontime::solar_time(sl_mutex, t, p).unwrap();
    Ok(moontime::format_as(result, f, Some("solar time")))
}

#[derive(Serialize, Deserialize, Debug)]
struct MoonPostSolarAzel {
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
    #[serde(default = "default_format")]
    f: FormatSpecifier,
    #[serde(default = "default_degrees")]
    u: UnitSpecifier,
    p: Position,
}

async fn moon_post_sun_azel(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Json(MoonPostSolarAzel { t, f, u, p }): Json<MoonPostSolarAzel>,
) -> Result<String, ()> {
    let res = moontime::solar_azel(sl_mutex, t, p);
    let res = moontime::translate_to(res, u);
    let res = moontime::format_as(res, f, Some("moon_sun"));
    Ok(res)
}

async fn moon_post_sun(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Path(coord_format): Path<CoordFormat>,
    Json(MoonPostSolarAzel { t, f, u, p }): Json<MoonPostSolarAzel>,
) -> Result<String, ()> {
    let res = moontime::solar_azel(sl_mutex, t, p);
    match coord_format {
        CoordFormat::Xyz => {
            let xyz: PositionXYZ = res.into();
            Ok(moontime::format_as(xyz, f, Some("moon_sun_xyz")))
        }
        CoordFormat::Spherical => {
            let spherical: PositionSpherical = res.into();
            let spherical = moontime::translate_to(spherical, u);
            Ok(moontime::format_as(spherical, f, Some("moon_sun_spherical")))
        }
        CoordFormat::Azel => {
            let azel = moontime::translate_to(res, u);
            Ok(moontime::format_as(azel, f, Some("moon_sun_azel")))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MoonQuerySolarAzel {
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
    #[serde(default = "default_format")]
    f: FormatSpecifier,
    #[serde(default = "default_degrees")]
    u: UnitSpecifier,
    p: Position,
}

async fn moon_get_sun_azel(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Query(MoonQuerySolarAzel { t, f, u, p }): Query<MoonQuerySolarAzel>,
) -> Result<String, ()> {
    let res = moontime::solar_azel(sl_mutex, t, p);
    let res = moontime::translate_to(res, u);
    let res = moontime::format_as(res, f, Some("moon_sun"));
    Ok(res)
}

async fn moon_get_sun(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Path(coord_format): Path<CoordFormat>,
    Query(MoonQuerySolarAzel { t, f, u, p }): Query<MoonQuerySolarAzel>,
) -> Result<String, ()> {
    let res = moontime::solar_azel(sl_mutex, t, p);
    match coord_format {
        CoordFormat::Xyz => {
            let xyz: PositionXYZ = res.into();
            Ok(moontime::format_as(xyz, f, Some("moon_sun_xyz")))
        }
        CoordFormat::Spherical => {
            let spherical: PositionSpherical = res.into();
            let spherical = moontime::translate_to(spherical, u);
            Ok(moontime::format_as(spherical, f, Some("moon_sun_spherical")))
        }
        CoordFormat::Azel => {
            let azel = moontime::translate_to(res, u);
            Ok(moontime::format_as(azel, f, Some("moon_sun_azel")))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CADREPostSolarTime {
    #[serde(default = "default_format")]
    f: FormatSpecifier,
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
}
async fn cadre_post_solar_time(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Json(CADREPostSolarTime { f, t }): Json<CADREPostSolarTime>,
) -> Result<String, ()> {
    let p = Position::cadre();
    println!("t: {:?}, f: {:?}, p: {:?}", t, f, p);
    let result = moontime::solar_time(sl_mutex, t, p.to_radians()).unwrap();
    Ok(moontime::format_as(result, f, Some("solar time")))
}

#[derive(Serialize, Deserialize, Debug)]
struct CADRESolarTimeQuery {
    #[serde(default = "default_format")]
    f: FormatSpecifier,
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
}

async fn cadre_get_solar_time(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Query(CADRESolarTimeQuery { t, f }): Query<CADRESolarTimeQuery>,
) -> Result<String, ()> {
    let p = Position::cadre();
    println!("t: {:?}, f: {:?}, p: {:?}", t, f, p);
    let result = moontime::solar_time(sl_mutex, t, p).unwrap();
    Ok(moontime::format_as(result, f, Some("solar time")))
}

#[derive(Serialize, Deserialize, Debug)]
struct CADREPostSolarAzel {
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
    #[serde(default = "default_format")]
    f: FormatSpecifier,
    #[serde(default = "default_degrees")]
    u: UnitSpecifier,
}

async fn cadre_post_sun_azel(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Json(CADREPostSolarAzel { t, f, u }): Json<CADREPostSolarAzel>,
) -> Result<String, ()> {
    let p = Position::cadre();
    let res = moontime::solar_azel(sl_mutex, t, p);
    let res = moontime::translate_to(res, u);
    let res = moontime::format_as(res, f, Some("cadre_sun"));
    Ok(res)
}

async fn cadre_post_sun(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Path(coord_format): Path<CoordFormat>,
    Json(CADREPostSolarAzel { t, f, u }): Json<CADREPostSolarAzel>,
) -> Result<String, ()> {
    let p = Position::cadre();
    let res = moontime::solar_azel(sl_mutex, t, p);
    match coord_format {
        CoordFormat::Xyz => {
            let xyz: PositionXYZ = res.into();
            Ok(moontime::format_as(xyz, f, Some("cadre_sun_xyz")))
        }
        CoordFormat::Spherical => {
            let spherical: PositionSpherical = res.into();
            let spherical = moontime::translate_to(spherical, u);
            Ok(moontime::format_as(spherical, f, Some("cadre_sun_spherical")))
        }
        CoordFormat::Azel => {
            let azel = moontime::translate_to(res, u);
            Ok(moontime::format_as(azel, f, Some("cadre_sun_azel")))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CADREQuerySolarAzel {
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
    #[serde(default = "default_format")]
    f: FormatSpecifier,
    #[serde(default = "default_degrees")]
    u: UnitSpecifier,
}

async fn cadre_get_sun_azel(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Query(CADREQuerySolarAzel { t, f, u }): Query<CADREQuerySolarAzel>,
) -> Result<String, ()> {
    let p = Position::cadre();
    let res = moontime::solar_azel(sl_mutex, t, p);
    let res = moontime::translate_to(res, u);
    let res = moontime::format_as(res, f, Some("cadre_sun"));
    Ok(res)
}

async fn cadre_get_sun(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Path(coord_format): Path<CoordFormat>,
    Query(CADREQuerySolarAzel { t, f, u }): Query<CADREQuerySolarAzel>,
) -> Result<String, ()> {
    let p = Position::cadre();
    let res = moontime::solar_azel(sl_mutex, t, p);
    match coord_format {
        CoordFormat::Xyz => {
            let xyz: PositionXYZ = res.into();
            Ok(moontime::format_as(xyz, f, Some("cadre_sun_xyz")))
        }
        CoordFormat::Spherical => {
            let spherical: PositionSpherical = res.into();
            let spherical = moontime::translate_to(spherical, u);
            Ok(moontime::format_as(spherical, f, Some("cadre_sun_spherical")))
        }
        CoordFormat::Azel => {
            let azel = moontime::translate_to(res, u);
            Ok(moontime::format_as(azel, f, Some("cadre_sun_azel")))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SunEarthQuery {
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
    #[serde(default = "default_format")]
    f: FormatSpecifier,
    #[serde(default = "default_degrees")]
    u: UnitSpecifier,
}

async fn get_sun_earth_full(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Query(SunEarthQuery { t, f, u }): Query<SunEarthQuery>,
) -> Result<String, ()> {
    let res = moontime::earth_position_from_sun(sl_mutex, t);
    let res = moontime::translate_to(res, u);
    let res = moontime::format_as(res, f, Some("earth_from_sun"));
    Ok(res)
}

async fn get_sun_earth(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Path(coord_format): Path<CoordFormat>,
    Query(SunEarthQuery { t, f, u }): Query<SunEarthQuery>,
) -> Result<String, ()> {
    let res = moontime::earth_position_from_sun(sl_mutex, t);
    match coord_format {
        CoordFormat::Xyz => {
            let xyz: PositionXYZ = res.into();
            Ok(moontime::format_as(xyz, f, Some("earth_from_sun_xyz")))
        }
        CoordFormat::Spherical => {
            let spherical: PositionSpherical = res.into();
            let spherical = moontime::translate_to(spherical, u);
            Ok(moontime::format_as(spherical, f, Some("earth_from_sun_spherical")))
        }
        CoordFormat::Azel => {
            Err(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SunEarthPost {
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
    #[serde(default = "default_format")]
    f: FormatSpecifier,
    #[serde(default = "default_degrees")]
    u: UnitSpecifier,
}

async fn post_sun_earth_full(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Json(SunEarthPost { t, f, u }): Json<SunEarthPost>,
) -> Result<String, ()> {
    let res = moontime::earth_position_from_sun(sl_mutex, t);
    let res = moontime::translate_to(res, u);
    let res = moontime::format_as(res, f, Some("earth_from_sun"));
    Ok(res)
}

async fn post_sun_earth(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Path(coord_format): Path<CoordFormat>,
    Json(SunEarthPost { t, f, u }): Json<SunEarthPost>,
) -> Result<String, ()> {
    let res = moontime::earth_position_from_sun(sl_mutex, t);
    match coord_format {
        CoordFormat::Xyz => {
            let xyz: PositionXYZ = res.into();
            Ok(moontime::format_as(xyz, f, Some("earth_from_sun_xyz")))
        }
        CoordFormat::Spherical => {
            let spherical: PositionSpherical = res.into();
            let spherical = moontime::translate_to(spherical, u);
            Ok(moontime::format_as(spherical, f, Some("earth_from_sun_spherical")))
        }
        CoordFormat::Azel => {
            Err(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct EclipticEarthQuery {
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
    #[serde(default = "default_format")]
    f: FormatSpecifier,
    #[serde(default = "default_degrees")]
    u: UnitSpecifier,
}

async fn get_ecliptic_earth_full(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Query(EclipticEarthQuery { t, f, u }): Query<EclipticEarthQuery>,
) -> Result<String, ()> {
    let res = moontime::earth_position_ecliptic(sl_mutex, t);
    let res = moontime::translate_to(res, u);
    let res = moontime::format_as(res, f, Some("earth_ecliptic"));
    Ok(res)
}

async fn get_ecliptic_earth(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Path(coord_format): Path<CoordFormat>,
    Query(EclipticEarthQuery { t, f, u }): Query<EclipticEarthQuery>,
) -> Result<String, ()> {
    let res = moontime::earth_position_ecliptic(sl_mutex, t);
    match coord_format {
        CoordFormat::Xyz => {
            let xyz: PositionXYZ = res.into();
            Ok(moontime::format_as(xyz, f, Some("earth_ecliptic_xyz")))
        }
        CoordFormat::Spherical => {
            let spherical: PositionSpherical = res.into();
            let spherical = moontime::translate_to(spherical, u);
            Ok(moontime::format_as(spherical, f, Some("earth_ecliptic_spherical")))
        }
        CoordFormat::Azel => {
            Err(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct EclipticEarthPost {
    #[serde(with = "default_datetime_standard", default = "default_datetime")]
    t: DateTime,
    #[serde(default = "default_format")]
    f: FormatSpecifier,
    #[serde(default = "default_degrees")]
    u: UnitSpecifier,
}

async fn post_ecliptic_earth_full(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Json(EclipticEarthPost { t, f, u }): Json<EclipticEarthPost>,
) -> Result<String, ()> {
    let res = moontime::earth_position_ecliptic(sl_mutex, t);
    let res = moontime::translate_to(res, u);
    let res = moontime::format_as(res, f, Some("earth_ecliptic"));
    Ok(res)
}

async fn post_ecliptic_earth(
    State(sl_mutex): State<Arc<Mutex<spice::SpiceLock>>>,
    Path(coord_format): Path<CoordFormat>,
    Json(EclipticEarthPost { t, f, u }): Json<EclipticEarthPost>,
) -> Result<String, ()> {
    let res = moontime::earth_position_ecliptic(sl_mutex, t);
    match coord_format {
        CoordFormat::Xyz => {
            let xyz: PositionXYZ = res.into();
            Ok(moontime::format_as(xyz, f, Some("earth_ecliptic_xyz")))
        }
        CoordFormat::Spherical => {
            let spherical: PositionSpherical = res.into();
            let spherical = moontime::translate_to(spherical, u);
            Ok(moontime::format_as(spherical, f, Some("earth_ecliptic_spherical")))
        }
        CoordFormat::Azel => {
            Err(())
        }
    }
}

