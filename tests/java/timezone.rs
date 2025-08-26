use j4rs::Jvm;
use j4rs::{Instance, InvocationArg};

#[allow(dead_code)]
pub fn create_timezone(jvm: &Jvm, timezone_id: &str) -> Instance {
    let jvm_timezone = jvm
        .invoke_static(
            "java.util.TimeZone",
            "getTimeZone",
            &[InvocationArg::try_from(timezone_id).unwrap()],
        )
        .unwrap();
    jvm_timezone
}
