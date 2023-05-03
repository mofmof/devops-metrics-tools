use clap::{Parser, Subcommand};
use chrono::{prelude::*, Duration};
use env_logger;
use modules::config::DeploymentSource;

mod modules;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    action: Action,

    #[clap(short, long, global = true, required = false)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Action {
    Github {
        #[clap(subcommand)]
        sub_action: GithubAction
    },
    Heroku {
        #[clap(subcommand)]
        sub_action: HerokuAction
    },
    Config {
        #[clap(subcommand)]
        sub_action: ConfigAction
    },
    Commits {
        #[clap(subcommand)]
        sub_action: CommitsAction,

        #[clap(short, long, global = false, required = true)]
        project: String,
    },
    Pulls {
        #[clap(subcommand)]
        sub_action: PullsAction,

        #[clap(short, long, global = false, required = true)]
        project: String,
    },
    Deployments {
        #[clap(subcommand)]
        sub_action: DeploymentsAction,

        #[clap(short, long, global = false, required = false)]
        environment: Option<String>,

        #[clap(short, long, global = false, required = true)]
        project: String,
    },
    FourKeys {
        #[clap(subcommand)]
        sub_action: DeploymentFrequenciesAction,

        #[clap(short, long, global = true, required = false)]
        since: Option<String>,

        #[clap(short, long, global = true, required = false)]
        until: Option<String>,

        #[clap(short, long, global = true, required = false)]
        environment: Option<String>,

        #[clap(short, long, global = false, required = true)]
        project: String,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    Get {},
    Set {
        #[clap(long, required = false)]
        project: Option<String>,
    },
}

#[derive(Subcommand)]
enum CommitsAction {
    Get {},
    Compare {
        #[clap(long, required = false)]
        base: String,

        #[clap(long, required = false)]
        head: String,
    },
}

#[derive(Subcommand)]
enum PullsAction {
    Get {},
}

#[derive(Subcommand)]
enum DeploymentsAction {
    Get {},
}

#[derive(Subcommand)]
enum DeploymentFrequenciesAction {
    Get {},
}

#[derive(Subcommand)]
enum GithubAction {
    Login {},
}

#[derive(Subcommand)]
enum HerokuAction {
    Login {},
    Releases {
        #[clap(short, long, global = true, required = false)]
        project: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let log_level = if args.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    env_logger::builder()
        .filter_level(log_level)
        .init();

    match args.action {
        Action::Github { sub_action } => {
            match sub_action {
                GithubAction::Login {} => {
                    modules::config::set_github_personal_token().await.expect("Could not set config");
                }
            }
        },
        Action::Heroku { sub_action } => {
            match sub_action {
                HerokuAction::Login {} => {
                    modules::config::set_heroku_authorization_token().await.expect("Could not set config");
                },
                HerokuAction::Releases { project } => {
                    let config = modules::config::load_config().await;
                    let project_config = config.projects.get(&project).expect("Could not find project");
                    let heroku_app = project_config.heroku_app.clone().expect("Could not find heroku app");
                    let results = modules::heroku::release::list(&heroku_app, &project_config.github_owner, &project_config.github_repo).await.expect("Could not get releases");
                    println!("{}", serde_json::to_string_pretty(&results).expect("Could not convert releases to string"));
                }
            }
        },
        Action::Config { sub_action } => {
             match sub_action {
                ConfigAction::Get {} => {
                    let path = modules::config::get_config_path().expect("Could not convert path to string");

                    let config = modules::config::load_config().await;
                    println!("File: {:?}\nValues: {}", path, serde_json::to_string_pretty(&config).expect("Could not convert config to string"));
                },
                ConfigAction::Set { project } => {
                    match project {
                        Some(project) => {
                            modules::config::set_project_config(&project).await.expect("Could not set project config");
                        },
                        None => {
                            modules::config::set_github_personal_token().await.expect("Could not set config");
                        }
                    }
                }
            }
        },
        Action::Commits { sub_action, project } => {
            let config = modules::config::load_config().await;
            let project_config = config.projects.get(&project).expect("Could not find project");
            match sub_action {
                CommitsAction::Get {} => {
                    let results = modules::github::commit::list(&project_config.github_owner, &project_config.github_repo).await.expect("Could not get commits");
                    println!("{}", serde_json::to_string_pretty(&results).expect("Could not convert commits to string"));
                },
                CommitsAction::Compare { base, head } => {
                    let results = modules::github::compare::get_first_commit_committer_date(&project_config.github_owner, &project_config.github_repo, &base, &head).await.expect("Could not get commits");
                    println!("{}", serde_json::to_string_pretty(&results).expect("Could not convert commits to string"));
                }
            }
        },
        Action::Pulls { sub_action, project } => {
            let config = modules::config::load_config().await;
            let project_config = config.projects.get(&project).expect("Could not find project");
            match sub_action {
                PullsAction::Get {} => {
                    let results = modules::github::pull::list(&project_config.github_owner, &project_config.github_repo).await.expect("Could not get pulls");
                    println!("{}", serde_json::to_string_pretty(&results).expect("Could not convert pulls to string"));
                }
            }
        },
        Action::Deployments { sub_action, environment, project } => {
            let config = modules::config::load_config().await;
            let project_config = config.projects.get(&project).expect("Could not find project");
            let env = environment.map_or("production".to_string(), |s| if s.is_empty() {
                "production".to_string()
            } else {
                s
            });
            match sub_action {
                DeploymentsAction::Get {} => {
                    let results = modules::github::deployment::list(&project_config.github_owner, &project_config.github_repo, &env).await.expect("Could not get deployments");
                    println!("{}", serde_json::to_string_pretty(&results).expect("Could not convert deployments to string"));
                }
            }
        },
        Action::FourKeys { sub_action, project, since, until, environment } => {
            let config = modules::config::load_config().await;
            let project_config = config.projects.get(&project).expect("Could not find project");
            let time = NaiveTime::from_hms_opt(0, 0, 0).expect("Could not parse time");
            let datetime_since = since.and_then(|s| {
                let naive_since = NaiveDate::parse_from_str(&s, "%Y-%m-%d").expect("Could not parse since date");
                let datetime_since = Utc.from_local_datetime(&naive_since.and_time(time)).unwrap();
                Some(datetime_since)
            }).or_else(|| {
                let three_months_ago = Utc::now() - Duration::days(90);
                Some(three_months_ago)
            }).expect("Could not parse since date");
            let datetime_until = until.and_then(|u| {
                let naive_until = NaiveDate::parse_from_str(&u, "%Y-%m-%d").expect("Could not parse until date");
                let datetime_until = Utc.from_local_datetime(&naive_until.and_time(time)).unwrap();
                Some(datetime_until)
            }).or_else(|| {
                let now = Utc::now();
                Some(now)
            }).expect("Could not parse until date");
            let developers = project_config.developers;
            let working_days_per_week = project_config.working_days_per_week;
            let env = environment.map_or("production".to_string(), |s| if s.is_empty() {
                "production".to_string()
            } else {
                s
            });
            let heroku_app = &project_config.heroku_app;
            match sub_action {
                DeploymentFrequenciesAction::Get {} => {
                    match project_config.deployment_source {
                        DeploymentSource::GitHubDeployment => {
                            let results = modules::metric::with_github_deployments::calculate_metrics(&project_config.github_owner, &project_config.github_repo, datetime_since, datetime_until, developers, working_days_per_week, &env).await.expect("Could not calculate metrics");
                            println!("{}", serde_json::to_string_pretty(&results).expect("Could not convert metrics to string"));
                        },
                        DeploymentSource::HerokuRelease => {
                            let heroku_app = heroku_app.as_ref().expect("Could not find heroku app");
                            let results = modules::metric::with_heroku_releases::calculate_metrics(&project_config.github_owner, &project_config.github_repo, datetime_since, datetime_until, developers, working_days_per_week, heroku_app).await.expect("Could not calculate metrics");
                            println!("{}", serde_json::to_string_pretty(&results).expect("Could not convert metrics to string"));
                        },
                        // _ => {
                        //     println!("No deployment source configured");
                        // }
                    }
                }
            }
        }
    }
}