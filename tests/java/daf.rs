use j4rs::{Instance, InvocationArg, Jvm};

#[allow(dead_code)]
pub struct JavaDaf<'a> {
    jvm: &'a Jvm,
    pub instance: Instance,
}

#[allow(dead_code)]
impl<'a> JavaDaf<'a> {
    #[allow(dead_code)]
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
            .invoke(&self.instance, "getMasechtaNumber", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
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
            .invoke(&self.instance, "getDaf", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
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
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    pub fn get_masechta(&self) -> String {
        let result = self
            .jvm
            .invoke(&self.instance, "getMasechta", InvocationArg::empty())
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    pub fn get_yerushalmi_masechta_transliterated(&self) -> String {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getYerushalmiMasechtaTransliterated",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }

    pub fn get_yerushalmi_masechta(&self) -> String {
        let result = self
            .jvm
            .invoke(
                &self.instance,
                "getYerushalmiMasechta",
                InvocationArg::empty(),
            )
            .unwrap();
        self.jvm.to_rust(result).unwrap()
    }
}
