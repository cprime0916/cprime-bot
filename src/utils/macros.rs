#[macro_export]
macro_rules! delete_message {
    ($ctx:expr) => {
        if let Err(e) = $ctx.channel_id().delete_message(&$ctx, $ctx.id()).await {
            eprintln!("{}", e);
            $ctx.say("How to delete this fucking message?").await?;
        }
    };
}

/// # Walao macro
/// ## Overview
/// The walao macro is used as the debug print macro to handle errors more easily & efficiently.
/// It prints or sends the messages depending on the different arguments
/// ## Usage
/// Parsing file type error: `walao!(parse, YourFileType)`
/// 
/// `NotInRange` error: `walao!(ctx, not_in_range)`
/// 
/// Customisable error message (expect): `walao!(expect, $err)`
/// 
/// Other use cases include a parsing error message for `unwrap()` methods in options/results.
/// If the error is not defined, no argument should be given, like this
/// ```
/// walao!();
/// ```
/// And this outputs the unit error message: ```Walao! This error siao eh, not even defined one```
#[macro_export]
macro_rules! walao {
    (status_code_error) =>  {
        eprintln!("Walao! Status code error, skill issue one");
    };
    (parse, $ft:expr) => {
        eprintln!("Walao! Can't even parse {} files! Skill issue one", $ft);
    };
    ($ctx:expr, parse_config) => {
        let toml_info = fs::read_to_string("config.toml").unwrap();
        let config_toml: config::Config = toml::from_str(&toml_info).unwrap();
        let error_message = format!("# Walao!\n<@{}> skill issue one lah! I can't even parse the configuration", config_toml.discord.user_id);
        $ctx.say(error_message).await?;
        eprintln!("Walao! Parse configuration error");
    };
    ($ctx:expr, not_in_range) => {
        let toml_info = fs::read_to_string("config.toml").unwrap();
        let config_toml: config::Config = toml::from_str(&toml_info).unwrap();
        eprintln!("Walao! Not in range...");
        let error_message = format!("# Walao!\n<@{}> skill issue one lah! Not even in range", config_toml.discord.user_id);
        $ctx.say(error_message).await?;
    };
    (expect, $err:expr) => {
        format!("Walao! {}", $err)
    };
    (unwrap, parse) => {
        String::from("Walao! Parsing Error")
    };
    () => {
        eprintln!("Walao! This error siao eh, not even defined one")
    };
}