#[derive(Debug)]
pub struct Message {
    pub sender: String,
    pub body: String,
    pub date: i64,
    pub thread_id: i64,
    pub unique_id: i32,
    pub device_name: String,
    pub sim_id: i64,
}
