#[ic_cdk_macros::query]
fn print() {
    ic_cdk::print("Hello World from DFINITY!");
}

#[export_name = "canister_heartbeat"]
fn canister_heartbeat() {
    ic_cdk::print("Are we running cron");
}