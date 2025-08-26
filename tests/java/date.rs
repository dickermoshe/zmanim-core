use j4rs::{Instance, InvocationArg, Jvm};

#[allow(dead_code)]
pub fn create_date(jvm: &Jvm, timestamp: i64) -> Instance {
    let instance = jvm
        .create_instance(
            "java.util.Date",
            &[InvocationArg::try_from(timestamp)
                .unwrap()
                .into_primitive()
                .unwrap()],
        )
        .unwrap();
    instance
}
