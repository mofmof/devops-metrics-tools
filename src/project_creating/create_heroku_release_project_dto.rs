use cranenum::Cranenum;

use crate::{
    dependencies::write_new_config::interface::WriteConfigData,
    persistence::project_config::ProjectConfig,
    project_creating::validate_working_days_per_week::{
        self, schema::ValidateWorkingDaysPerWeekError,
    },
    project_parameter_validating::{
        validate_developer_count::{self, ValidateDeveloperCountError},
        validate_github_owner_repo::{self, ValidateGitHubOwnerRepoError},
        validate_github_personal_token::{self, ValidateGitHubPersonalTokenError},
        validate_heroku_app_name::{self, ValidateHerokuAppNameError},
        validate_heroku_auth_token::{self, ValidateHerokuApiTokenError},
    },
};

use super::create_heroku_release_project_schema::HerokuReleaseProjectCreated;

pub type HerokuReleaseProjectCreatedDto = WriteConfigData;

#[derive(Cranenum)]
pub enum ToHerokuReleaseProjectCreatedError {
    GitHubPersonalToken(ValidateGitHubPersonalTokenError),
    GitHubOwnerRepo(ValidateGitHubOwnerRepoError),
    DeveloperCount(ValidateDeveloperCountError),
    WorkingDaysPerWeek(ValidateWorkingDaysPerWeekError),
    HerokuAppName(ValidateHerokuAppNameError),
    HerokuApiToken(ValidateHerokuApiTokenError),
}

impl HerokuReleaseProjectCreatedDto {
    pub fn to_heroku_release_project_created(
        dto: &HerokuReleaseProjectCreatedDto,
    ) -> Result<HerokuReleaseProjectCreated, ToHerokuReleaseProjectCreatedError> {
        let github_personal_token =
            validate_github_personal_token::perform(Some(dto.github_personal_token.to_string()))?;
        let heroku_app_name =
            validate_heroku_app_name::perform(dto.project_config.clone().heroku_app_name)?;
        let heroku_api_token =
            validate_heroku_auth_token::perform(dto.project_config.clone().heroku_api_token)?;
        let github_owner_repo = validate_github_owner_repo::perform(format!(
            "{}/{}",
            dto.project_config.github_owner, dto.project_config.github_repo
        ))?;
        let developer_count =
            validate_developer_count::perform(dto.project_config.developer_count.to_string())?;
        let working_days_per_week = validate_working_days_per_week::workflow::perform(
            dto.project_config.working_days_per_week.to_string(),
        )?;
        Ok(HerokuReleaseProjectCreated {
            project_name: dto.project_name.to_string(),
            github_personal_token,
            github_owner_repo,
            heroku_app_name,
            heroku_api_token,
            developer_count,
            working_days_per_week,
        })
    }

    pub fn from_heroku_release_project_created(
        domain_obj: HerokuReleaseProjectCreated,
    ) -> HerokuReleaseProjectCreatedDto {
        let (owner, repo) = domain_obj.github_owner_repo.get_values();
        HerokuReleaseProjectCreatedDto {
            project_name: domain_obj.project_name,
            github_personal_token: domain_obj.github_personal_token.to_string(),
            project_config: ProjectConfig {
                github_personal_token: None,
                heroku_app_name: Some(domain_obj.heroku_app_name.to_string()),
                heroku_api_token: Some(domain_obj.heroku_api_token.to_string()),
                deployment_source: "github_deployment".to_string(),
                github_owner: owner,
                github_repo: repo,
                developer_count: domain_obj.developer_count.to_u32(),
                working_days_per_week: domain_obj.working_days_per_week.to_f32(),
            },
        }
    }
}
