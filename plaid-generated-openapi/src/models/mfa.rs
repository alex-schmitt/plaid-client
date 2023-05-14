/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.343.0
 *
 * Generated by: https://openapi-generator.tech
 */

/// Mfa : Specifies the multi-factor authentication settings to use with this test account

#[derive(
    Clone, Debug, PartialEq, Default, Serialize, Deserialize, bincode::Encode, bincode::Decode,
)]
pub struct Mfa {
    /// Possible values are `device`, `selections`, or `questions`.  If value is `device`, the MFA answer is `1234`.  If value is `selections`, the MFA answer is always the first option.  If value is `questions`, the MFA answer is  `answer_<i>_<j>` for the j-th question in the i-th round, starting from 0. For example, the answer to the first question in the second round is `answer_1_0`.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Number of rounds of questions. Required if value of `type` is `questions`.
    #[serde(rename = "question_rounds")]
    pub question_rounds: f32,
    /// Number of questions per round. Required if value of `type` is `questions`. If value of type is `selections`, default value is 2.
    #[serde(rename = "questions_per_round")]
    pub questions_per_round: f32,
    /// Number of rounds of selections, used if `type` is `selections`. Defaults to 1.
    #[serde(rename = "selection_rounds")]
    pub selection_rounds: f32,
    /// Number of available answers per question, used if `type` is `selection`. Defaults to 2.
    #[serde(rename = "selections_per_question")]
    pub selections_per_question: f32,
}

impl Mfa {
    /// Specifies the multi-factor authentication settings to use with this test account
    pub fn new(
        r#type: String,
        question_rounds: f32,
        questions_per_round: f32,
        selection_rounds: f32,
        selections_per_question: f32,
    ) -> Mfa {
        Mfa {
            r#type,
            question_rounds,
            questions_per_round,
            selection_rounds,
            selections_per_question,
        }
    }
}
