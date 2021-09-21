use crate::services::submission::support::{ProblemSupport, SubmissionGenerationPayload, SubmissionGenerationResult, SubmissionTestGenerationPayload, SubmissionTestGenerationResult, VerificationPayload, VerificationResult};
use rand::prelude::*;
use rand::{RngCore, SeedableRng, rngs::StdRng};
use rand_distr::{Uniform, Distribution};
use std::collections::{HashMap, HashSet};

pub struct SanityCheckProblemSupport {}

impl SanityCheckProblemSupport {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SanityCheckInputMeta {
    pub set_length: usize,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct SanityCheckInput {
    pub set: Vec<usize>,
    pub meta: SanityCheckInputMeta,
}

#[derive(serde::Serialize,serde::Deserialize, Debug)]
pub struct SanityCheckOutput {
    pub insane_numbers: Vec<usize>,
}

const MAX_INPUTS: i64 = 10000;
pub type IRng = StdRng;


lazy_static! {
    pub static ref SAMPLES: Vec<SanityCheckInput> = {
        vec![
            SanityCheckInput {
                meta: SanityCheckInputMeta {
                    set_length: 4,
                },
                set: vec![
                    1,
                    233,
                    100,
                    64
                ]
            },
            SanityCheckInput {
                meta: SanityCheckInputMeta {
                    set_length: 12,
                },
                set: vec![
                    424242424242424,
                    42,
                    24242,
                    24,
                    424242424242424,
                    424242,
                    242424,
                    4242,
                    2424,
                    4242,
                    24242,
                    2424
                ]
            },
        ]
    };
}

impl SanityCheckProblemSupport {
    fn rng(seed: i64, skip: i64) -> IRng {
        StdRng::seed_from_u64((seed + skip) as u64)
    }

    fn generate_input(payload: &SubmissionTestGenerationPayload) -> SanityCheckInput {
        let mut rng = Self::rng(payload.seed, 0);

        let mut set = vec![];
        let unf = Uniform::new(10usize.pow(payload.test_index as u32), MAX_INPUTS as usize);
        let set_length = unf.sample(&mut rng);
        let unf_small = Uniform::new(0, 10usize.pow(payload.test_index as u32 * 3u32 + 2u32 ));

        for _ in 0..set_length {
            set.push(unf_small.sample(&mut rng));
        }

        SanityCheckInput {
            meta: SanityCheckInputMeta {
                set_length,
            },
            set,
        }
    }

    pub fn solver(inp_str: &str) -> anyhow::Result<SanityCheckOutput> {
        let inp = serde_json::from_str::<SanityCheckInput>(inp_str)
            .map_err(|_| anyhow::Error::msg("Input couldn't be constructed!"))?;

        let mut res: Vec<(usize, usize)> = vec![];
        let mut max_ones = 0;

        for ii in inp.set.into_iter() {
            let ones_in_binary = format!("{:b}", ii).chars().filter(|ch| ch == &'1').count();
            if max_ones < ones_in_binary {
                max_ones = ones_in_binary;
            }
            res.push((ii, ones_in_binary));
        }

        let mut insane_numbers: Vec<usize> = vec![];

        for (num, ones) in res.into_iter() {
            if ones == max_ones {
                insane_numbers.push(num);
            }
        }

        Ok(SanityCheckOutput {
            insane_numbers,
        })
    }

    fn are_outputs_conforming(correct: &SanityCheckOutput, to_test: &SanityCheckOutput) -> bool {
        let correct_numbers = &correct.insane_numbers;
        let to_test_numbers = &to_test.insane_numbers;

        if correct_numbers.len() != to_test_numbers.len() {
            return false;
        }

        let mut test_hash = HashSet::new();
        for tt in to_test_numbers.into_iter() {
            test_hash.insert(tt.clone());
        }

        for nn in correct_numbers.into_iter() {
            let num_in_test = test_hash.contains(nn);

            if !num_in_test {
                return false;
            }
        }

        true
    }
}

impl ProblemSupport for SanityCheckProblemSupport {
    fn generate_submission_details(&self, payload: SubmissionGenerationPayload) -> anyhow::Result<SubmissionGenerationResult> {
        Ok(SubmissionGenerationResult {
            test_count: if payload.sample_index.is_some() { 1 } else { 4 },
        })
    }

    fn generate_submission_test_input(&self, payload: SubmissionTestGenerationPayload) -> anyhow::Result<SubmissionTestGenerationResult> {
        if let Some(sample_index) = payload.sample_index {
            let sample = SAMPLES.get(sample_index as usize)
                .ok_or_else(|| anyhow::Error::msg("Sample not found!"))?;

            return Ok(SubmissionTestGenerationResult {
                input: serde_json::to_value(sample.clone())
                    .map_err(|_| anyhow::Error::msg("Input couldn't be constructed!"))?,
                test_class: "".to_string(),
            });
        }

        let input = Self::generate_input(&payload);
        let input_val = serde_json::to_value(&input)
            .map_err(|_| anyhow::Error::msg("Input couldn't be constructed!"))?;

        Ok(SubmissionTestGenerationResult {
            input: input_val,
            test_class: "".to_string(),
        })
    }

    fn verify_output(&self, payload: VerificationPayload) -> anyhow::Result<VerificationResult> {
        let expected_output = Self::solver(&payload.test.input)?;
        let got_output = serde_json::from_value::<SanityCheckOutput>(payload.output.clone())
            .map_err(|_|  anyhow::Error::msg("Output is invalid!"))?;

        let correct = Self::are_outputs_conforming(&expected_output, &got_output);

        Ok(VerificationResult {
            correct,
        })
    }
}
