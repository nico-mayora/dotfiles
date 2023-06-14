Vim�UnDo� ��0bP	,I��9�2�!��ڦ��6��j  �                                   c_�S    _�                     �       ����                                                                                                                                                                                                                                                                                                                                                             c_ߤ     �              �   mod cli;   mod download;   mod subcommands;       #use anyhow::{anyhow, bail, Result};   #use clap::{CommandFactory, Parser};   Guse cli::{Ferium, ModpackSubCommands, ProfileSubCommands, SubCommands};   'use colored::{ColoredString, Colorize};   $use dialoguer::theme::ColorfulTheme;   use ferinth::Ferinth;   use furse::Furse;   use indicatif::ProgressStyle;   use lazy_static::lazy_static;   use libium::config::{   	    self,   7    structs::{Config, ModIdentifier, Modpack, Profile},   };   use octocrab::OctocrabBuilder;   use online::tokio::check;   
use std::{       env::{var, var_os},       process::ExitCode,       sync::Arc,   };   use tokio::{runtime, spawn};       const CROSS: &str = "×";   lazy_static! {   7    pub static ref TICK: ColoredString = "✓".green();   ?    pub static ref YELLOW_TICK: ColoredString = "✓".yellow();   C    pub static ref THEME: ColorfulTheme = ColorfulTheme::default();   }       #[allow(clippy::expect_used)]   $pub fn style_no() -> ProgressStyle {        ProgressStyle::default_bar()   b        .template("{spinner} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:.cyan}/{len:.blue}")   5        .expect("Progess bar template parse failure")           .progress_chars("#>-")   }       #[allow(clippy::expect_used)]   &pub fn style_byte() -> ProgressStyle {        ProgressStyle::default_bar()           .template(   d            "{spinner} [{bytes_per_sec}] [{wide_bar:.cyan/blue}] {bytes:.cyan}/{total_bytes:.blue}",   	        )   5        .expect("Progess bar template parse failure")           .progress_chars("#>-")   }       fn main() -> ExitCode {       let cli = Ferium::parse();   ;    let mut builder = runtime::Builder::new_multi_thread();       builder.enable_all();   )    builder.thread_name("ferium-worker");   (    if let Some(threads) = cli.threads {   .        builder.max_blocking_threads(threads);       }   :    #[allow(clippy::expect_used)] // No error handling yet   O    let runtime = builder.build().expect("Could not initialise Tokio runtime");   :    if let Err(err) = runtime.block_on(actual_main(cli)) {   6        eprintln!("{}", err.to_string().red().bold());           ExitCode::FAILURE       } else {           ExitCode::SUCCESS       }   }       O#[allow(clippy::future_not_send)] // 3rd party library doesn't implement `Send`   5async fn actual_main(cli_app: Ferium) -> Result<()> {   8    // The complete command should not require a config.   e    // See [#139](https://github.com/gorilla-devs/ferium/issues/139) for why this might be a problem.   A    if let SubCommands::Complete { shell } = cli_app.subcommand {            clap_complete::generate(               shell,   #            &mut Ferium::command(),               "ferium",   #            &mut std::io::stdout(),   
        );           return Ok(());       }           let github = Arc::new(           cli_app               .github_token               .map_or_else(                   || {   4                    var("GITHUB_TOKEN").map_or_else(   3                        |_| OctocrabBuilder::new(),   M                        |token| OctocrabBuilder::new().personal_token(token),                       )                   },   E                |token| OctocrabBuilder::new().personal_token(token),               )               .build()?,       );   )    let modrinth = Arc::new(Ferinth::new(           "ferium",   )        option_env!("CARGO_PKG_VERSION"),   $        Some("theRookieCoder#1287"),           None,       )?);   T    let curseforge = Arc::new(Furse::new(&cli_app.curseforge_api_key.unwrap_or_else(           || {   :            var("CURSEFORGE_API_KEY").unwrap_or_else(|_| {   U                "$2a$10$QbCxI6f4KxEs50QKwE2piu1t6oOA8ayOw27H9N/eaH3Sdp5NTWwvO".into()               })   
        },       )));       +    let mut config_file = config::get_file(           cli_app               .config_file   E            .or_else(|| var_os("FERIUM_CONFIG_FILE").map(Into::into))   /            .unwrap_or_else(config::file_path),       )       .await?;   W    let mut config = config::deserialise(&config::read_file(&mut config_file).await?)?;       B    // Run function(s) based on the sub(sub)command to be executed       match cli_app.subcommand {   7        SubCommands::Complete { .. } => unreachable!(),           SubCommands::Add {               identifier,   $            dont_check_game_version,   "            dont_check_mod_loader,               dependencies,           } => {   ;            let profile = get_active_profile(&mut config)?;   $            check_internet().await?;   ?            if let Ok(project_id) = identifier.parse::<i32>() {   -                subcommands::add::curseforge(                       curseforge,                       project_id,                       profile,   3                    Some(!dont_check_game_version),   1                    Some(!dont_check_mod_loader),   !                    dependencies,                   )                   .await?;   :            } else if identifier.split('/').count() == 2 {   F                let split = identifier.split('/').collect::<Vec<_>>();   )                subcommands::add::github(   5                    github.repos(split[0], split[1]),                       profile,   3                    Some(!dont_check_game_version),   1                    Some(!dont_check_mod_loader),                   )                   .await?;   @            } else if let Err(err) = subcommands::add::modrinth(                   modrinth,                   &identifier,                   profile,   /                Some(!dont_check_game_version),   -                Some(!dont_check_mod_loader),                   dependencies,               )               .await               {                   return Err(   Q                    if err.to_string() == ferinth::Error::NotBase62.to_string() {   6                        anyhow!("Invalid indentifier")                       } else {                           err                       },                   );               }   
        },   4        SubCommands::List { verbose, markdown } => {   ;            let profile = get_active_profile(&mut config)?;   *            check_empty_profile(profile)?;               if verbose {   (                check_internet().await?;   +                let mut tasks = Vec::new();   +                for mod_ in &profile.mods {   !                    if markdown {   0                        match &mod_.identifier {   M                            ModIdentifier::CurseForgeProject(project_id) => {   a                                subcommands::list::curseforge_md(curseforge.clone(), *project_id)   ,                                    .await?;                               },   K                            ModIdentifier::ModrinthProject(project_id) => {   ?                                subcommands::list::modrinth_md(   5                                    modrinth.clone(),   7                                    project_id.clone(),   !                                )   (                                .await?;                               },   K                            ModIdentifier::GitHubRepository(full_name) => {   _                                subcommands::list::github_md(github.clone(), full_name.clone())   ,                                    .await?;                               },                           };                       } else {   <                        let mut mr_ids = Vec::<&str>::new();   0                        match &mod_.identifier {   ]                            ModIdentifier::CurseForgeProject(project_id) => tasks.push(spawn(   _                                subcommands::list::curseforge(curseforge.clone(), *project_id),                               )),   b                            ModIdentifier::ModrinthProject(project_id) => mr_ids.push(project_id),   [                            ModIdentifier::GitHubRepository(full_name) => tasks.push(spawn(   ]                                subcommands::list::github(github.clone(), full_name.clone()),                               )),                           };   Y                        let mr_projects = modrinth.get_multiple_projects(&mr_ids).await?;   7                        let mr_teams_members = modrinth   9                            .list_multiple_teams_members(   ,                                &mr_projects   +                                    .iter()   =                                    .map(|p| &p.team as &str)   9                                    .collect::<Vec<_>>(),                               )   $                            .await?;   6                        for (project, team_members) in   U                            mr_projects.into_iter().zip(mr_teams_members.into_iter())                           {   b                            tasks.push(spawn(subcommands::list::modrinth(project, team_members)));                           }                       }                   }   %                for handle in tasks {   #                    handle.await??;                   }               } else {   +                for mod_ in &profile.mods {                       println!(   #                        "{:45} {}",   )                        mod_.name.bold(),   0                        match &mod_.identifier {   C                            ModIdentifier::CurseForgeProject(id) =>   a                                format!("{:10} {}", "CurseForge".red(), id.to_string().dimmed()),   A                            ModIdentifier::ModrinthProject(id) =>   U                                format!("{:10} {}", "Modrinth".green(), id.dimmed()),   M                            ModIdentifier::GitHubRepository(name) => format!(   +                                "{:10} {}",   2                                "GitHub".purple(),   I                                format!("{}/{}", name.0, name.1).dimmed()                               ),                           },                       );                   }               }   
        },   A        SubCommands::Modpack { subcommand } => match subcommand {   %            ModpackSubCommands::Add {                   identifier,                   output_dir,   "                install_overrides,               } => {   (                check_internet().await?;   C                if let Ok(project_id) = identifier.parse::<i32>() {   :                    subcommands::modpack::add::curseforge(   +                        curseforge.clone(),   $                        &mut config,   #                        project_id,   #                        output_dir,   *                        install_overrides,                       )                       .await?;   M                } else if let Err(err) = subcommands::modpack::add::modrinth(   %                    modrinth.clone(),                        &mut config,                        &identifier,                       output_dir,   &                    install_overrides,                   )                   .await                   {                       return Err(   U                        if err.to_string() == ferinth::Error::NotBase62.to_string() {   :                            anyhow!("Invalid indentifier")                            } else {                               err                           },                       );                   }               },   +            ModpackSubCommands::Configure {                   output_dir,   "                install_overrides,               } => {   0                subcommands::modpack::configure(   5                    get_active_modpack(&mut config)?,                       output_dir,   &                    install_overrides,                   )                   .await?;               },   <            ModpackSubCommands::Delete { modpack_name } => {   I                subcommands::modpack::delete(&mut config, modpack_name)?;               },   )            ModpackSubCommands::List => {   /                if config.modpacks.is_empty() {   g                    bail!("There are no modpacks configured, add a modpack using `ferium modpack add`")                   }   4                subcommands::modpack::list(&config);               },   <            ModpackSubCommands::Switch { modpack_name } => {   I                subcommands::modpack::switch(&mut config, modpack_name)?;               },   ,            ModpackSubCommands::Upgrade => {   (                check_internet().await?;   .                subcommands::modpack::upgrade(   %                    modrinth.clone(),   '                    curseforge.clone(),   5                    get_active_modpack(&mut config)?,                   )                   .await?;               },   
        },   A        SubCommands::Profile { subcommand } => match subcommand {   +            ProfileSubCommands::Configure {                   game_version,                   mod_loader,                   name,                   output_dir,               } => {   (                check_internet().await?;   0                subcommands::profile::configure(   5                    get_active_profile(&mut config)?,   !                    game_version,                       mod_loader,                       name,                       output_dir,                   )                   .await?;               },   (            ProfileSubCommands::Create {                   import,                   game_version,                   mod_loader,                   name,                   output_dir,               } => {   +                if game_version.is_none() {   ,                    check_internet().await?;                   }   -                subcommands::profile::create(                        &mut config,                       import,   !                    game_version,                       mod_loader,                       name,                       output_dir,                   )                   .await?;               },   <            ProfileSubCommands::Delete { profile_name } => {   I                subcommands::profile::delete(&mut config, profile_name)?;               },   )            ProfileSubCommands::List => {   /                if config.profiles.is_empty() {   m                    bail!("There are no profiles configured, create a profile using `ferium profile create`")                   }   4                subcommands::profile::list(&config);               },   <            ProfileSubCommands::Switch { profile_name } => {   I                subcommands::profile::switch(&mut config, profile_name)?;               },   
        },   .        SubCommands::Remove { mod_names } => {   ;            let profile = get_active_profile(&mut config)?;   *            check_empty_profile(profile)?;   5            subcommands::remove(profile, mod_names)?;   
        },   !        SubCommands::Upgrade => {   $            check_internet().await?;   ;            let profile = get_active_profile(&mut config)?;   *            check_empty_profile(profile)?;   O            subcommands::upgrade(modrinth, curseforge, github, profile).await?;   
        },       };       3    config.profiles.iter_mut().for_each(|profile| {           profile               .mods   A            .sort_by_cached_key(|mod_| mod_.name.to_lowercase());       });   5    // Update config file with possibly edited config   9    config::write_file(&mut config_file, &config).await?;       
    Ok(())   }       ./// Get the active profile with error handling   Dfn get_active_profile(config: &mut Config) -> Result<&mut Profile> {   #    if config.profiles.is_empty() {   a        bail!("There are no profiles configured, create a profile using `ferium profile create`")   =    } else if config.profiles.len() < config.active_profile {           println!(               "{}",   P            "Active profile specified incorrectly, please pick a profile to use"                   .red()                   .bold()   
        );   4        subcommands::profile::switch(config, None)?;   7        Ok(&mut config.profiles[config.active_profile])       } else {   7        Ok(&mut config.profiles[config.active_profile])       }   }       ./// Get the active modpack with error handling   Dfn get_active_modpack(config: &mut Config) -> Result<&mut Modpack> {   #    if config.modpacks.is_empty() {   [        bail!("There are no modpacks configured, add a modpack using `ferium modpack add`")   =    } else if config.modpacks.len() < config.active_modpack {           println!(               "{}",   P            "Active modpack specified incorrectly, please pick a modpack to use"                   .red()                   .bold()   
        );   4        subcommands::modpack::switch(config, None)?;   7        Ok(&mut config.modpacks[config.active_modpack])       } else {   7        Ok(&mut config.modpacks[config.active_modpack])       }   }       :/// Check if `profile` is empty, and if so return an error   9fn check_empty_profile(profile: &Profile) -> Result<()> {        if profile.mods.is_empty() {   d        bail!("Your currently selected profile is empty! Run `ferium help` to see how to add mods");       }   
    Ok(())   }       $/// Check for an internet connection   )async fn check_internet() -> Result<()> {   &    if check(Some(1)).await.is_err() {   )        // If it takes more than 1 second   ;        // show that we're checking the internet connection   '        // and check for 4 more seconds   4        eprint!("Checking internet connection... ");   $        match check(Some(4)).await {               Ok(_) => {   &                println!("{}", *TICK);                   Ok(())               },   "            Err(_) => Err(anyhow!(   D                "{} Ferium requires an internet connection to work",                   CROSS               )),   	        }       } else {           Ok(())       }   }5�5�_�                     �   *    ����                                                                                                                                                                                                                                                                                                                                                             c_�R    �              �   mod cli;   mod download;   mod subcommands;       #use anyhow::{anyhow, bail, Result};   #use clap::{CommandFactory, Parser};   Guse cli::{Ferium, ModpackSubCommands, ProfileSubCommands, SubCommands};   'use colored::{ColoredString, Colorize};   $use dialoguer::theme::ColorfulTheme;   use ferinth::Ferinth;   use furse::Furse;   use indicatif::ProgressStyle;   use lazy_static::lazy_static;   use libium::config::{   	    self,   7    structs::{Config, ModIdentifier, Modpack, Profile},   };   use octocrab::OctocrabBuilder;   use online::tokio::check;   
use std::{       env::{var, var_os},       process::ExitCode,       sync::Arc,   };   use tokio::{runtime, spawn};       const CROSS: &str = "×";   lazy_static! {   7    pub static ref TICK: ColoredString = "✓".green();   ?    pub static ref YELLOW_TICK: ColoredString = "✓".yellow();   C    pub static ref THEME: ColorfulTheme = ColorfulTheme::default();   }       #[allow(clippy::expect_used)]   $pub fn style_no() -> ProgressStyle {        ProgressStyle::default_bar()   b        .template("{spinner} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:.cyan}/{len:.blue}")   5        .expect("Progess bar template parse failure")           .progress_chars("#>-")   }       #[allow(clippy::expect_used)]   &pub fn style_byte() -> ProgressStyle {        ProgressStyle::default_bar()           .template(   d            "{spinner} [{bytes_per_sec}] [{wide_bar:.cyan/blue}] {bytes:.cyan}/{total_bytes:.blue}",   	        )   5        .expect("Progess bar template parse failure")           .progress_chars("#>-")   }       fn main() -> ExitCode {       let cli = Ferium::parse();   ;    let mut builder = runtime::Builder::new_multi_thread();       builder.enable_all();   )    builder.thread_name("ferium-worker");   (    if let Some(threads) = cli.threads {   .        builder.max_blocking_threads(threads);       }   :    #[allow(clippy::expect_used)] // No error handling yet   O    let runtime = builder.build().expect("Could not initialise Tokio runtime");   :    if let Err(err) = runtime.block_on(actual_main(cli)) {   6        eprintln!("{}", err.to_string().red().bold());           ExitCode::FAILURE       } else {           ExitCode::SUCCESS       }   }       O#[allow(clippy::future_not_send)] // 3rd party library doesn't implement `Send`   5async fn actual_main(cli_app: Ferium) -> Result<()> {   8    // The complete command should not require a config.   e    // See [#139](https://github.com/gorilla-devs/ferium/issues/139) for why this might be a problem.   A    if let SubCommands::Complete { shell } = cli_app.subcommand {            clap_complete::generate(               shell,   #            &mut Ferium::command(),               "ferium",   #            &mut std::io::stdout(),   
        );           return Ok(());       }           let github = Arc::new(           cli_app               .github_token               .map_or_else(                   || {   4                    var("GITHUB_TOKEN").map_or_else(   3                        |_| OctocrabBuilder::new(),   M                        |token| OctocrabBuilder::new().personal_token(token),                       )                   },   E                |token| OctocrabBuilder::new().personal_token(token),               )               .build()?,       );   )    let modrinth = Arc::new(Ferinth::new(           "ferium",   )        option_env!("CARGO_PKG_VERSION"),   $        Some("theRookieCoder#1287"),           None,       )?);   T    let curseforge = Arc::new(Furse::new(&cli_app.curseforge_api_key.unwrap_or_else(           || {   :            var("CURSEFORGE_API_KEY").unwrap_or_else(|_| {   U                "$2a$10$QbCxI6f4KxEs50QKwE2piu1t6oOA8ayOw27H9N/eaH3Sdp5NTWwvO".into()               })   
        },       )));       +    let mut config_file = config::get_file(           cli_app               .config_file   E            .or_else(|| var_os("FERIUM_CONFIG_FILE").map(Into::into))   /            .unwrap_or_else(config::file_path),       )       .await?;   W    let mut config = config::deserialise(&config::read_file(&mut config_file).await?)?;       B    // Run function(s) based on the sub(sub)command to be executed       match cli_app.subcommand {   7        SubCommands::Complete { .. } => unreachable!(),           SubCommands::Add {               identifier,   $            dont_check_game_version,   "            dont_check_mod_loader,               dependencies,           } => {   ;            let profile = get_active_profile(&mut config)?;   $            check_internet().await?;   ?            if let Ok(project_id) = identifier.parse::<i32>() {   -                subcommands::add::curseforge(                       curseforge,                       project_id,                       profile,   3                    Some(!dont_check_game_version),   1                    Some(!dont_check_mod_loader),   !                    dependencies,                   )                   .await?;   :            } else if identifier.split('/').count() == 2 {   F                let split = identifier.split('/').collect::<Vec<_>>();   )                subcommands::add::github(   5                    github.repos(split[0], split[1]),                       profile,   3                    Some(!dont_check_game_version),   1                    Some(!dont_check_mod_loader),                   )                   .await?;   @            } else if let Err(err) = subcommands::add::modrinth(                   modrinth,                   &identifier,                   profile,   /                Some(!dont_check_game_version),   -                Some(!dont_check_mod_loader),                   dependencies,               )               .await               {                   return Err(   Q                    if err.to_string() == ferinth::Error::NotBase62.to_string() {   6                        anyhow!("Invalid indentifier")                       } else {                           err                       },                   );               }   
        },   4        SubCommands::List { verbose, markdown } => {   ;            let profile = get_active_profile(&mut config)?;   *            check_empty_profile(profile)?;               if verbose {   (                check_internet().await?;   +                let mut tasks = Vec::new();   +                for mod_ in &profile.mods {   !                    if markdown {   0                        match &mod_.identifier {   M                            ModIdentifier::CurseForgeProject(project_id) => {   a                                subcommands::list::curseforge_md(curseforge.clone(), *project_id)   ,                                    .await?;                               },   K                            ModIdentifier::ModrinthProject(project_id) => {   ?                                subcommands::list::modrinth_md(   5                                    modrinth.clone(),   7                                    project_id.clone(),   !                                )   (                                .await?;                               },   K                            ModIdentifier::GitHubRepository(full_name) => {   _                                subcommands::list::github_md(github.clone(), full_name.clone())   ,                                    .await?;                               },                           };                       } else {   <                        let mut mr_ids = Vec::<&str>::new();   0                        match &mod_.identifier {   ]                            ModIdentifier::CurseForgeProject(project_id) => tasks.push(spawn(   _                                subcommands::list::curseforge(curseforge.clone(), *project_id),                               )),   b                            ModIdentifier::ModrinthProject(project_id) => mr_ids.push(project_id),   [                            ModIdentifier::GitHubRepository(full_name) => tasks.push(spawn(   ]                                subcommands::list::github(github.clone(), full_name.clone()),                               )),                           };   Y                        let mr_projects = modrinth.get_multiple_projects(&mr_ids).await?;   7                        let mr_teams_members = modrinth   9                            .list_multiple_teams_members(   ,                                &mr_projects   +                                    .iter()   =                                    .map(|p| &p.team as &str)   9                                    .collect::<Vec<_>>(),                               )   $                            .await?;   6                        for (project, team_members) in   U                            mr_projects.into_iter().zip(mr_teams_members.into_iter())                           {   b                            tasks.push(spawn(subcommands::list::modrinth(project, team_members)));                           }                       }                   }   %                for handle in tasks {   #                    handle.await??;                   }               } else {   +                for mod_ in &profile.mods {                       println!(   #                        "{:45} {}",   )                        mod_.name.bold(),   0                        match &mod_.identifier {   C                            ModIdentifier::CurseForgeProject(id) =>   a                                format!("{:10} {}", "CurseForge".red(), id.to_string().dimmed()),   A                            ModIdentifier::ModrinthProject(id) =>   U                                format!("{:10} {}", "Modrinth".green(), id.dimmed()),   M                            ModIdentifier::GitHubRepository(name) => format!(   +                                "{:10} {}",   2                                "GitHub".purple(),   I                                format!("{}/{}", name.0, name.1).dimmed()                               ),                           },                       );                   }               }   
        },   A        SubCommands::Modpack { subcommand } => match subcommand {   %            ModpackSubCommands::Add {                   identifier,                   output_dir,   "                install_overrides,               } => {   (                check_internet().await?;   C                if let Ok(project_id) = identifier.parse::<i32>() {   :                    subcommands::modpack::add::curseforge(   +                        curseforge.clone(),   $                        &mut config,   #                        project_id,   #                        output_dir,   *                        install_overrides,                       )                       .await?;   M                } else if let Err(err) = subcommands::modpack::add::modrinth(   %                    modrinth.clone(),                        &mut config,                        &identifier,                       output_dir,   &                    install_overrides,                   )                   .await                   {                       return Err(   U                        if err.to_string() == ferinth::Error::NotBase62.to_string() {   :                            anyhow!("Invalid indentifier")                            } else {                               err                           },                       );                   }               },   +            ModpackSubCommands::Configure {                   output_dir,   "                install_overrides,               } => {   0                subcommands::modpack::configure(   5                    get_active_modpack(&mut config)?,                       output_dir,   &                    install_overrides,                   )                   .await?;               },   <            ModpackSubCommands::Delete { modpack_name } => {   I                subcommands::modpack::delete(&mut config, modpack_name)?;               },   )            ModpackSubCommands::List => {   /                if config.modpacks.is_empty() {   g                    bail!("There are no modpacks configured, add a modpack using `ferium modpack add`")                   }   4                subcommands::modpack::list(&config);               },   <            ModpackSubCommands::Switch { modpack_name } => {   I                subcommands::modpack::switch(&mut config, modpack_name)?;               },   ,            ModpackSubCommands::Upgrade => {   (                check_internet().await?;   .                subcommands::modpack::upgrade(   %                    modrinth.clone(),   '                    curseforge.clone(),   5                    get_active_modpack(&mut config)?,                   )                   .await?;               },   
        },   A        SubCommands::Profile { subcommand } => match subcommand {   +            ProfileSubCommands::Configure {                   game_version,                   mod_loader,                   name,                   output_dir,               } => {   (                check_internet().await?;   0                subcommands::profile::configure(   5                    get_active_profile(&mut config)?,   !                    game_version,                       mod_loader,                       name,                       output_dir,                   )                   .await?;               },   (            ProfileSubCommands::Create {                   import,                   game_version,                   mod_loader,                   name,                   output_dir,               } => {   +                if game_version.is_none() {   ,                    check_internet().await?;                   }   -                subcommands::profile::create(                        &mut config,                       import,   !                    game_version,                       mod_loader,                       name,                       output_dir,                   )                   .await?;               },   <            ProfileSubCommands::Delete { profile_name } => {   I                subcommands::profile::delete(&mut config, profile_name)?;               },   )            ProfileSubCommands::List => {   /                if config.profiles.is_empty() {   m                    bail!("There are no profiles configured, create a profile using `ferium profile create`")                   }   4                subcommands::profile::list(&config);               },   <            ProfileSubCommands::Switch { profile_name } => {   I                subcommands::profile::switch(&mut config, profile_name)?;               },   
        },   .        SubCommands::Remove { mod_names } => {   ;            let profile = get_active_profile(&mut config)?;   *            check_empty_profile(profile)?;   5            subcommands::remove(profile, mod_names)?;   
        },   !        SubCommands::Upgrade => {   $            check_internet().await?;   ;            let profile = get_active_profile(&mut config)?;   *            check_empty_profile(profile)?;   O            subcommands::upgrade(modrinth, curseforge, github, profile).await?;   
        },       };       3    config.profiles.iter_mut().for_each(|profile| {           profile               .mods   A            .sort_by_cached_key(|mod_| mod_.name.to_lowercase());       });   5    // Update config file with possibly edited config   9    config::write_file(&mut config_file, &config).await?;       
    Ok(())   }       ./// Get the active profile with error handling   Dfn get_active_profile(config: &mut Config) -> Result<&mut Profile> {   #    if config.profiles.is_empty() {   a        bail!("There are no profiles configured, create a profile using `ferium profile create`")   =    } else if config.profiles.len() < config.active_profile {           println!(               "{}",   P            "Active profile specified incorrectly, please pick a profile to use"                   .red()                   .bold()   
        );   4        subcommands::profile::switch(config, None)?;   7        Ok(&mut config.profiles[config.active_profile])       } else {   7        Ok(&mut config.profiles[config.active_profile])       }   }       ./// Get the active modpack with error handling   Dfn get_active_modpack(config: &mut Config) -> Result<&mut Modpack> {   #    if config.modpacks.is_empty() {   [        bail!("There are no modpacks configured, add a modpack using `ferium modpack add`")   =    } else if config.modpacks.len() < config.active_modpack {           println!(               "{}",   P            "Active modpack specified incorrectly, please pick a modpack to use"                   .red()                   .bold()   
        );   4        subcommands::modpack::switch(config, None)?;   7        Ok(&mut config.modpacks[config.active_modpack])       } else {   7        Ok(&mut config.modpacks[config.active_modpack])       }   }       :/// Check if `profile` is empty, and if so return an error   9fn check_empty_profile(profile: &Profile) -> Result<()> {        if profile.mods.is_empty() {   d        bail!("Your currently selected profile is empty! Run `ferium help` to see how to add mods");       }   
    Ok(())   }       $/// Check for an internet connection   )async fn check_internet() -> Result<()> {   &    if check(Some(1)).await.is_err() {   )        // If it takes more than 1 second   ;        // show that we're checking the internet connection   '        // and check for 4 more seconds   4        eprint!("Checking internet connection... ");   $        match check(Some(4)).await {               Ok(_) => {   &                println!("{}", *TICK);                   Ok(())               },   "            Err(_) => Err(anyhow!(   D                "{} Ferium requires an internet connection to work",                   CROSS               )),   	        }       } else {           Ok(())       }   }5�5��