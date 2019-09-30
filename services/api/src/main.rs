use dotenv::dotenv;
use fetch::error::{Error, Result};
use fetch::{config, data,  routes};

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
    let db_conn = match config.database_url {
        Some(conntion_string) => data::pgsql::Connection::new(conntion_string)?,
        None => data::pgsql::Connection::new_environment()?
    };

    println!("Starting server on {}", addr);
    warp::serve(routes::routes(db_conn.into_warp())).run(addr);

    Ok(())
}
