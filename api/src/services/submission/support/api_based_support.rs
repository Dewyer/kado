use crate::models::problem::code_name::CodeName;
use crate::services::submission::support::{ProblemSupport, SubmissionGenerationPayload, SubmissionGenerationResult, SubmissionTestGenerationPayload, SubmissionTestGenerationResult, VerificationPayload, VerificationResult, VerificationPayloadOwned};
use log::{error, info};
use reqwest::blocking::Client;
use reqwest::{header, Method};

pub struct ApiBasedSupportEndpointSettings {
    pub base_url: String,

    pub generate_submission_details_endpoint: String,
    pub generate_submission_test_input_endpoint: String,
    pub verify_output_endpoint: String,
}

pub struct ApiBasedSupport {
    client: Client,
    ep_settings: ApiBasedSupportEndpointSettings,
    code_name: CodeName,
    api_key: String,
}

impl ApiBasedSupport {
    fn create_client(api_key: &str) -> anyhow::Result<Client> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "User-Agent",
            header::HeaderValue::from_static("SnapChallengeServer"),
        );
        headers.insert(
            "Accept",
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "X-Api-Key",
            header::HeaderValue::from_str(api_key)
                .map_err(|_| anyhow::Error::msg("Couldn't construct api support headers!"))?,
        );

        Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|_| anyhow::Error::msg("Couldn't construct api support client!"))
    }

    pub fn new(
        code_name: CodeName,
        ep_settings: ApiBasedSupportEndpointSettings,
        api_key: String,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            code_name,
            client: Self::create_client(&api_key)?,
            ep_settings,
            api_key,
        })
    }

    fn request<Req: serde::Serialize, Res: serde::de::DeserializeOwned>(
        &self,
        method: Method,
        endpoint: &str,
        body: &Req,
    ) -> anyhow::Result<Res> {
        let req = self
            .client
            .request(
                method,
                format!("{}{}", self.ep_settings.base_url, endpoint),
            )
            .json(body)
            .send()
            .map_err(|err| {
                error!(
                    "Requesting support api for {} - failed, err: {:?}",
                    self.code_name.to_string(),
                    err
                );

                anyhow::Error::msg("Requesting support api failed.")
            })?;

        if !req.status().is_success() {
            error!(
                "Requesting api for {} - failed, status-code: {:?}",
                self.code_name.to_string(),
                req.status().to_string()
            );
        }

        req.json::<Res>().map_err(|err| {
            error!(
                "Deserializing support api response for {} - failed, err: {:?}",
                self.code_name.to_string(),
                err
            );
            anyhow::Error::msg("Deserializing support api response failed.")
        })
    }
}

impl ProblemSupport for ApiBasedSupport {
    fn generate_submission_details(
        &self,
        payload: SubmissionGenerationPayload,
    ) -> anyhow::Result<SubmissionGenerationResult> {
        self.request::<SubmissionGenerationPayload, SubmissionGenerationResult>(
            Method::POST,
            &self.ep_settings.generate_submission_details_endpoint,
            &payload,
        )
    }

    fn generate_submission_test_input(
        &self,
        payload: SubmissionTestGenerationPayload,
    ) -> anyhow::Result<SubmissionTestGenerationResult> {
        self.request::<SubmissionTestGenerationPayload, SubmissionTestGenerationResult>(
            Method::POST,
            &self.ep_settings.generate_submission_test_input_endpoint,
            &payload,
        )
    }

    fn verify_output(&self, payload: VerificationPayload) -> anyhow::Result<VerificationResult> {
        self.request::<VerificationPayloadOwned, VerificationResult>(
            Method::POST,
            &self.ep_settings.verify_output_endpoint,
            &VerificationPayloadOwned::from_ref(&payload),
        )
    }
}
