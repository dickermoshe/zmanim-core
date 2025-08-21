use crate::java::daf::JavaDaf;
use crate::test_utils::create_jvm;
use zmanim_core::hebrew_calendar::Daf;

#[test]
fn test_daf_creation() {
    let jvm = create_jvm();

    // Test various masechtos and daf numbers
    let test_cases = vec![
        (0, 2),    // Berachos 2
        (1, 156),  // Shabbos 156
        (2, 105),  // Eruvin 105
        (3, 121),  // Pesachim 121
        (10, 32),  // Megillah 32
        (20, 119), // Bava Kamma 119
        (36, 22),  // Kinnim 22
        (39, 73),  // Niddah 73
    ];

    for (masechta_number, daf_number) in test_cases {
        // Create Rust implementation
        let rust_daf = Daf::new(masechta_number, daf_number);

        // Create Java implementation
        let java_daf = JavaDaf::new(&jvm, masechta_number, daf_number);

        // Test masechta number
        assert_eq!(
            rust_daf.get_masechta_number(),
            java_daf.get_masechta_number()
        );

        // Test daf number
        assert_eq!(rust_daf.get_daf(), java_daf.get_daf());

        // Test masechta transliterated name
        assert_eq!(
            rust_daf.get_masechta_transliterated(),
            java_daf.get_masechta_transliterated()
        );

        // Test masechta Hebrew name
        assert_eq!(rust_daf.get_masechta(), java_daf.get_masechta());

        // Test Yerushalmi masechta transliterated name
        assert_eq!(
            rust_daf.get_yerushalmi_masechta_transliterated(),
            java_daf.get_yerushalmi_masechta_transliterated()
        );

        // Test Yerushalmi masechta Hebrew name
        assert_eq!(
            rust_daf.get_yerushalmi_masechta(),
            java_daf.get_yerushalmi_masechta()
        );
    }
}

#[test]
fn test_daf_setters() {
    let jvm = create_jvm();

    // Create initial daf
    let mut rust_daf = Daf::new(0, 2);
    let java_daf = JavaDaf::new(&jvm, 0, 2);

    // Test setting masechta number
    rust_daf.set_masechta_number(5);
    java_daf.set_masechta_number(5);

    assert_eq!(
        rust_daf.get_masechta_number(),
        java_daf.get_masechta_number()
    );
    assert_eq!(
        rust_daf.get_masechta_transliterated(),
        java_daf.get_masechta_transliterated()
    );

    // Test setting daf number
    rust_daf.set_daf(42);
    java_daf.set_daf(42);

    assert_eq!(rust_daf.get_daf(), java_daf.get_daf());
}

#[test]
fn test_masechtos_lists() {
    // Test that all Bavli masechtos are correct
    let bavli_transliterated = Daf::get_bavli_masechtos_transliterated();
    let bavli_hebrew = Daf::get_bavli_masechtos();

    // Should have the same number of masechtos
    assert_eq!(bavli_transliterated.len(), bavli_hebrew.len());

    // Test a few specific masechtos
    assert_eq!(bavli_transliterated[0], "Berachos");
    assert_eq!(bavli_hebrew[0], "ברכות");
    assert_eq!(bavli_transliterated[1], "Shabbos");
    assert_eq!(bavli_hebrew[1], "שבת");
    assert_eq!(bavli_transliterated[23], "Sanhedrin");
    assert_eq!(bavli_hebrew[23], "סנהדרין");

    // Test that all Yerushalmi masechtos are correct
    let yerushalmi_transliterated = Daf::get_yerushalmi_masechtos_transliterated();
    let yerushalmi_hebrew = Daf::get_yerushalmi_masechtos();

    // Should have the same number of masechtos
    assert_eq!(yerushalmi_transliterated.len(), yerushalmi_hebrew.len());

    // Test a few specific masechtos
    assert_eq!(yerushalmi_transliterated[0], "Berachos");
    assert_eq!(yerushalmi_hebrew[0], "ברכות");
    assert_eq!(yerushalmi_transliterated[1], "Peah");
    assert_eq!(yerushalmi_hebrew[1], "פאה");
}

#[test]
fn test_edge_cases() {
    let jvm = create_jvm();

    // Test boundary cases
    let test_cases = vec![
        (0, 1),   // First page of first masechta
        (39, 73), // Last page of last masechta (Niddah)
    ];

    for (masechta_number, daf_number) in test_cases {
        let rust_daf = Daf::new(masechta_number, daf_number);
        let java_daf = JavaDaf::new(&jvm, masechta_number, daf_number);

        assert_eq!(
            rust_daf.get_masechta_number(),
            java_daf.get_masechta_number()
        );
        assert_eq!(rust_daf.get_daf(), java_daf.get_daf());
        assert_eq!(
            rust_daf.get_masechta_transliterated(),
            java_daf.get_masechta_transliterated()
        );
        assert_eq!(rust_daf.get_masechta(), java_daf.get_masechta());
    }
}

#[test]
fn test_comprehensive_masechtos_comparison() {
    let jvm = create_jvm();

    // Test all masechtos indices to ensure our arrays match Java implementation
    for masechta_idx in 0..40 {
        // There are 40 masechtos in Bavli
        let rust_daf = Daf::new(masechta_idx, 2); // Use daf 2 as a standard test page
        let java_daf = JavaDaf::new(&jvm, masechta_idx, 2);

        let rust_transliterated = rust_daf.get_masechta_transliterated();
        let java_transliterated = java_daf.get_masechta_transliterated();

        assert_eq!(
            rust_transliterated, java_transliterated,
            "Masechta transliterated name mismatch at index {}: Rust='{}', Java='{}'",
            masechta_idx, rust_transliterated, java_transliterated
        );

        let rust_hebrew = rust_daf.get_masechta();
        let java_hebrew = java_daf.get_masechta();

        assert_eq!(
            rust_hebrew, java_hebrew,
            "Masechta Hebrew name mismatch at index {}: Rust='{}', Java='{}'",
            masechta_idx, rust_hebrew, java_hebrew
        );
    }
}

#[test]
fn test_yerushalmi_masechtos_comparison() {
    let jvm = create_jvm();

    // Test Yerushalmi masechtos indices
    for masechta_idx in 0..39 {
        // There are 39 masechtos in Yerushalmi
        let rust_daf = Daf::new(masechta_idx, 2);
        let java_daf = JavaDaf::new(&jvm, masechta_idx, 2);

        let rust_yerushalmi_transliterated = rust_daf.get_yerushalmi_masechta_transliterated();
        let java_yerushalmi_transliterated = java_daf.get_yerushalmi_masechta_transliterated();

        assert_eq!(
            rust_yerushalmi_transliterated, java_yerushalmi_transliterated,
            "Yerushalmi masechta transliterated name mismatch at index {}: Rust='{}', Java='{}'",
            masechta_idx, rust_yerushalmi_transliterated, java_yerushalmi_transliterated
        );

        let rust_yerushalmi_hebrew = rust_daf.get_yerushalmi_masechta();
        let java_yerushalmi_hebrew = java_daf.get_yerushalmi_masechta();

        assert_eq!(
            rust_yerushalmi_hebrew, java_yerushalmi_hebrew,
            "Yerushalmi masechta Hebrew name mismatch at index {}: Rust='{}', Java='{}'",
            masechta_idx, rust_yerushalmi_hebrew, java_yerushalmi_hebrew
        );
    }
}
