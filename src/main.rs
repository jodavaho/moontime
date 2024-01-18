#![deny(warnings)]

use warp::Filter;
use spice;
use spice::SpiceLock;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() {

    //let mut tlskernel = spice::furnsh("./latest_leapseconds.tls");
    let sl = SpiceLock::try_acquire().unwrap();
    sl.furnsh("./latest_leapseconds.tls");
    let tlskernel = 
        Arc::new(Mutex::new(sl));


    let et_time = warp::path!("et")
        .and(warp::get())
        .and(warp::query::<EtQuery>())
        .and(with_kernel(tlskernel))
        .and_then(get_et_time);

    let readme = warp::path!("readme")
        .and(warp::fs::file("./README.md"));

    let routes = readme
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
    let response :String = match q.f{
        Some(EtFormat::Json) => {
            let er = EtResponse{et};
            serde_json::to_string(&er).unwrap()
        },
        None => {format!("{}", et)},
    };
    Ok(response)
}
