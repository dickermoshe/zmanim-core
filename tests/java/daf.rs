use j4rs::{Instance, InvocationArg, Jvm};

pub struct JavaDaf<'a> {
    jvm: &'a Jvm,
    pub instance: Instance,
}

impl<'a> JavaDaf<'a> {
    pub fn new(jvm: &'a Jvm, masechta_number: i32, daf: i32) -> Self {
        let instance = jvm
            .create_instance(
                "com.kosherjava.zmanim.hebrewcalendar.Daf",
                &[
                    InvocationArg::try_from(masechta_number)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(daf)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                ],
            )
            .unwrap();
        Self { jvm, instance }
    }

    pub fn get_masechta_number(&self) -> i32 {
        let result = self
            .jvm
            .invoke(&self.instance, "getMasechtaNumber", &[] as &[InvocationArg])
            .unwrap();
        self.jvm.to_rust::<i32>(result).unwrap()
    }

    pub fn set_masechta_number(&self, masechta_number: i32) {
        self.jvm
            .invoke(
                &self.instance,
                "setMasechtaNumber",
                &[InvocationArg::try_from(masechta_number)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
    }

    pub fn get_daf(&self) -> i32 {
        let result = self
            .jvm
            .invoke(&self.instance, "getDaf", &[] as &[InvocationArg])
            .unwrap();
        self.jvm.to_rust::<i32>(result).unwrap()
    }

    pub fn set_daf(&self, daf: i32) {
        self.jvm
            .invoke(
                &self.instance,
                "setDaf",
                &[InvocationArg::try_from(daf)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            )
            .unwrap();
    }

    pub fn get_masechta_transliterated(&self) -> String {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getMasechtaTransliterated",
                &[] as &[InvocationArg],
            )
            .unwrap();
        self.jvm.to_rust::<String>(result).unwrap()
    }

    pub fn get_masechta(&self) -> String {
        let result = self
            .jvm
            .invoke(&self.instance, "getMasechta", &[] as &[InvocationArg])
            .unwrap();
        self.jvm.to_rust::<String>(result).unwrap()
    }

    pub fn get_yerushalmi_masechta_transliterated(&self) -> String {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getYerushalmiMasechtaTransliterated",
                &[] as &[InvocationArg],
            )
            .unwrap();
        self.jvm.to_rust::<String>(result).unwrap()
    }

    pub fn get_yerushalmi_masechta(&self) -> String {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getYerushalmiMasechta",
                &[] as &[InvocationArg],
            )
            .unwrap();
        self.jvm.to_rust::<String>(result).unwrap()
    }

    // Note: Static array access is complex in JNI, so we'll test individual masechtos instead
}
