use error_chain::*;

error_chain::error_chain! {

    foreign_links {
        Fmt(std::fmt::Error);
        Io(std::io::Error) #[cfg(unix)];
        AddrParseError(std::net::AddrParseError);
        TomlDe(toml::de::Error);
        Diesel(diesel::result::Error);
        R2D2(r2d2::Error);
    }

    errors {
       InvalidForm(model: String, reason: String) {
           description("the model is invalid")
           display("the model for {0} is invalid due to: {1}", model, reason)
       }

       SettingsIncomplete(config: crate::config::Config) {
           description("Not enough settings are set to start")
           display("Not enough settings are set: {:?}", config)
       }

        ConfigReadFailed {
            description("Failed to read config of request")
           display("Failed to read config of request")
        }
    }
}
