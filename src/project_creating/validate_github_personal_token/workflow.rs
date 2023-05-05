use super::schema::*;

pub fn perform(token: UnvalidatedGitHubPersonalToken) -> Result<ValidateGitHubPersonalTokenEvent, ValidateGitHubPersonalTokenError> {
    ValidatedGitHubPersonalToken::new(token)
}

// PRIVATE

impl ValidatedGitHubPersonalToken {
    pub fn new(token: Option<String>) -> Result<Self, ValidateGitHubPersonalTokenError> {
      if let Some(token) = token {
        if token.starts_with("ghp_") {
          Ok(ValidatedGitHubPersonalToken(token.to_string()))
        } else {
          Err(ValidateGitHubPersonalTokenError::InvalidToken("GitHub personal token is invalid".to_string()))
        }
      } else {
        Err(ValidateGitHubPersonalTokenError::Required("GitHub personal token is empty".to_string()))
      }
    }

    pub fn to_string(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::project_creating::validate_github_personal_token::schema::ValidateGitHubPersonalToken;

    #[test]
    fn verify_perform_type() {
        // 型チェックのために代入する
        let _type_check: ValidateGitHubPersonalToken = super::perform;
    }
}
