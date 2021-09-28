use crate::services::submission::support::{
    ProblemSupport, SubmissionGenerationPayload, SubmissionGenerationResult,
    SubmissionTestGenerationPayload, SubmissionTestGenerationResult, VerificationPayload,
    VerificationResult,
};
use rand::prelude::*;
use rand::{rngs::StdRng, RngCore, SeedableRng};
use rand_distr::{Distribution, Uniform};
use std::collections::{HashMap, HashSet};

pub struct KingPinnedProblemSupport {}

impl KingPinnedProblemSupport {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct KingPinnedInput {
    pub room: Vec<Vec<usize>>,
    pub boss_starting_at: (usize, usize),
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct KingPinnedOutput {
    pub places_to_move_to: Vec<(usize, usize)>,
}

const MAX_INPUTS: i64 = 10000;
pub type IRng = StdRng;

impl ProblemSupport for KingPinnedProblemSupport {
    fn generate_submission_details(
        &self,
        payload: SubmissionGenerationPayload,
    ) -> anyhow::Result<SubmissionGenerationResult> {
        Ok(SubmissionGenerationResult {
            test_count: if payload.sample_index.is_some() { 1 } else { 4 },
        })
    }

    fn generate_submission_test_input(
        &self,
        payload: SubmissionTestGenerationPayload,
    ) -> anyhow::Result<SubmissionTestGenerationResult> {
        todo!();
    }

    fn verify_output(&self, payload: VerificationPayload) -> anyhow::Result<VerificationResult> {
        todo!();
    }
}
