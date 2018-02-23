error_chain! {

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error) #[cfg(unix)];
        TomlDe(::toml::de::Error);
        Diesel(::diesel::result::Error);
    }

    errors {
       InvalidForm(model: String, reason: String) {
           description("the model is invalid")
           display("the model for {0} is invalid due to: {1}", model, reason)
       }

       SettingsIncomplete(config: ::config::Config) {
           description("Not enough settings are set to start")
           display("Not enough settings are set: {:?}", config)
       }
    }
}
