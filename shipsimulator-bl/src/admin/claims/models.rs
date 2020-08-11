use shipsimulatorcommon::models::ReturnResult;
use shipsimulatordb::authenticatie::models::Claim;

pub struct ClaimsModel {
    result_code: ReturnResult,
    claims: Vec<ClaimModel>
}

pub struct ClaimModel {
    id: i32,
    name: String
}

impl ClaimsModel {
    pub fn new(succeed: bool, message: String, claims: Vec<Claim>) -> ClaimsModel {
        let result: ReturnResult = ReturnResult::new(succeed, message);
        let claims: Vec<ClaimModel> = ClaimsModel::get_claim_model(claims);

        ClaimsModel {
            result_code: result,
            claims: claims
        }
    }

    pub fn get_claim_model(db_claims: Vec<Claim>) -> Vec<ClaimModel> {
        Vec::new()
    }
}