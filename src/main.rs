#![deny(warnings)]

#[tokio::main]
async fn main() {
    let routes = filters::api();
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

mod filters {
    //use super::models::{
    use warp::Filter;

    pub fn api(

        ) ->
        impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        hello()
            .or(et())
    }

    pub fn hello() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("hello")
            .and(warp::get())
            .and_then(handlers::hello_handler)
    }

    pub fn et() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("et")
            .and(warp::get())
            .map(|| format!("ET:") )
    }
}


mod handlers
{
    use std::convert::Infallible;
    pub async fn hello_handler() -> Result<impl warp::Reply, Infallible> {
        Ok(format!("..."))
    }
}
