use reqwest::{header, Response};
use crate::models::{ClientResult, SnapChallengeClientError, StartSubmissionPayload, StartSubmissionResponse, StartTestPayload, StartTestResponse, SubmitTestPayload, SubmitTestResponse};
use reqwest::blocking::Client;

pub struct SnapChallengeClient {
    client: Client,
    base_url: String,
}

impl SnapChallengeClient {
    pub fn from_api_token(api_token: &str) -> ClientResult<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert("User-Agent", header::HeaderValue::from_static("SnapChallengeClient-rs"));
        headers.insert("Accept", header::HeaderValue::from_static("application/json"));
        headers.insert("X-Api-Token", header::HeaderValue::from_str(api_token).map_err(|_| SnapChallengeClientError::FailedToConstructClient)?);

        let client = Client::builder()
            .default_headers(headers)
            .build().map_err(|_| SnapChallengeClientError::FailedToConstructClient)?;

        Ok(Self {
            client,
            base_url: "http://localhost:3001".to_string(),
        })
    }

    pub fn override_base_url(&mut self, new_url: &str) {
        self.base_url = new_url.to_string();
    }

    pub fn start_submission(&self, payload: StartSubmissionPayload) -> ClientResult<StartSubmissionResponse> {
        self.client.post(format!("{}/{}", self.base_url, "/api/submissions/start-submission"))
            .json(&payload)
            .send()
            .map_err(|_| SnapChallengeClientError::RequestFailed)?
            .json::<StartSubmissionResponse>()
            .map_err(|_| SnapChallengeClientError::FailedDeserializingResponse)
    }

    pub fn start_test<Inp: serde::de::DeserializeOwned>(&self, payload: StartTestPayload) -> ClientResult<StartTestResponse<Inp>> {
        self.client.put(format!("{}/{}", self.base_url, "/api/submissions/test"))
            .json(&payload)
            .send()
            .map_err(|_| SnapChallengeClientError::RequestFailed)?
            .json::<StartTestResponse<Inp>>()
            .map_err(|_| SnapChallengeClientError::FailedDeserializingResponse)
    }

    pub fn submit_test<Out: serde::Serialize>(&self, payload: SubmitTestPayload<Out>) -> ClientResult<SubmitTestResponse> {
        let resp = self.client.post(format!("{}/{}/{}", self.base_url, "/api/submissions/test", payload.test_id))
            .json(&payload)
            .send()
            .map_err(|_| SnapChallengeClientError::RequestFailed)?;


        println!("Sts {}", resp.status());

        resp.json::<SubmitTestResponse>()
            .map_err(|err| {
                println!("errs: {:?}", err);

                SnapChallengeClientError::FailedDeserializingResponse
            })
    }
}
