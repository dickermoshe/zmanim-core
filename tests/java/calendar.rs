use j4rs::{Instance, InvocationArg, Jvm};

pub fn create_calendar(jvm: &Jvm, timestamp: i64) -> Instance {
    // Get a Calendar instance using the static getInstance() method
    let calendar_instance = jvm
        .invoke_static("java.util.Calendar", "getInstance", InvocationArg::empty())
        .unwrap();

    // Set the time in milliseconds
    jvm.invoke(
        &calendar_instance,
        "setTimeInMillis",
        &[InvocationArg::try_from(timestamp)
            .unwrap()
            .into_primitive()
            .unwrap()],
    )
    .unwrap();

    let timezone_instance = jvm
        .invoke_static(
            "java.util.TimeZone",
            "getTimeZone",
            &[InvocationArg::try_from("UTC").unwrap()],
        )
        .unwrap();

    jvm.invoke(
        &calendar_instance,
        "setTimeZone",
        &[InvocationArg::try_from(timezone_instance).unwrap()],
    )
    .unwrap();

    calendar_instance
}
