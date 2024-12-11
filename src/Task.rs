use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}