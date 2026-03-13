use super::notification::Toast;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Diff {
    pub new: Vec<Toast>,
    pub remove: Vec<Toast>,
}
