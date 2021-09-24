const HUN_SLURS: &str = include_str!("./slurs.txt");
const EN_SLURS: &str = include_str!("./slursen.txt");

fn calc_block_list() -> Vec<&'static str> {
    let mut res = vec![];

    for ii in HUN_SLURS.split('\n') {
        res.push(ii);
    }

    for ii in EN_SLURS.split('\n') {
        res.push(ii);
    }

    res
}

lazy_static! {
    pub static ref BLOCK_LIST: Vec<&'static str> = { calc_block_list() };
}
