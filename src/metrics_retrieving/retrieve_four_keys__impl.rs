use super::{retrieve_four_keys__schema::{RetrieveFourKeysExecutionContext, RetrieveFourKeysEvent, RetrieveFourKeysEventError}, retrieve_four_keys__dao::ReadConfig};

// ---------------------------
// Fetch deployments step
// ---------------------------

fn fetch_deployments() {

}

// ---------------------------
// overall workflow
// ---------------------------
pub fn perform(read_config: ReadConfig, context: RetrieveFourKeysExecutionContext) -> Result<RetrieveFourKeysEvent, RetrieveFourKeysEventError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::super::retrieve_four_keys__schema::RetrieveFourKeys;

    #[test]
    fn verify_perform_type() {
        // 型チェックのために代入する
        let _type_check: RetrieveFourKeys = super::perform;
    }
}