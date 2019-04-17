use dotenv::dotenv;
use fetch::error::{Error, Result};
use fetch::{config, db, routes};

use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = match config::get_config() {
        Ok(config) => config,
        Err(err) => {
            log::error!("Config error: {}", err);
            std::process::exit(1);
        }
    };

    let ip = IpAddr::from_str(&config.bind).map_err::<Error, _>(|err| err.into())?;
    let addr = SocketAddr::from((ip, config.port));
    let db_conn = db::get_pool(&config.database_url)?;

    println!("Starting server on {}", addr);
    warp::serve(routes::routes(db_conn)).run(addr);

    Ok(())
}
