use kado_lib_rs::client::SnapChallengeClient;
use kado_lib_rs::models::{StartSubmissionPayload, StartTestPayload, SubmitTestPayload};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SanityCheckInputMeta {
    pub set_length: usize,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SanityCheckInput {
    pub meta: SanityCheckInputMeta,
    pub set: Vec<usize>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SanityCheckOutput {
    pub insane_numbers: Vec<usize>,
}

const API_TOKEN: &'static str = "7HOQGxZYklWLmiPr";

fn sleep(secs: u64) {
    let dur = std::time::Duration::from_secs(secs);
    std::thread::sleep(dur);
}

fn main() {
    println!("Sanity check rust reference impl!");

    let client = SnapChallengeClient::from_api_token(API_TOKEN, "http://localhost:3001")
        .expect("a client");

    let sub_res = client.start_submission(StartSubmissionPayload {
        problem: "sanity-check".to_string(),
        sample_index: None,
    }).expect("created submission");

    for _ in 0..sub_res.submission.test_count {
        println!("Beginning test ...");
        // sleep(2);
        println!("running test ...");

        let test = client.start_test::<SanityCheckInput>(StartTestPayload {
            submission: sub_res.submission.id.to_string(),
        }).expect("test to be generated");

        let mut res: Vec<(usize, usize)> = vec![];
        let mut max_ones = 0;

        for ii in test.input.set.into_iter() {
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

        let test_res = client.submit_test(SubmitTestPayload::<SanityCheckOutput> {
            test_id: test.test_id,
            output: SanityCheckOutput {
                insane_numbers,
            },
        }).expect("to submit test");

        assert!(test_res.correct);
        println!("Test over.");
    }
}
