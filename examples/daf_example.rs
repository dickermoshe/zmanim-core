use zmanim_core::hebrew_calendar::Daf;

fn main() {
    // Create a new Daf for Berachos (masechta 0), page 2
    let daf = Daf::new(0, 2);

    println!(
        "Daf Yomi: {} {}",
        daf.masechta_transliterated().unwrap().as_str(),
        daf.daf()
    );

    println!("Hebrew: {} {}", daf.masechta().unwrap().as_str(), daf.daf());

    // Show Yerushalmi version
    println!(
        "Yerushalmi: {} {}",
        daf.yerushalmi_masechta_transliterated().unwrap().as_str(),
        daf.daf()
    );

    println!(
        "Yerushalmi Hebrew: {} {}",
        daf.yerushalmi_masechta().unwrap().as_str(),
        daf.daf()
    );

    // Show some static data
    println!("\nAll Bavli Masechtos (first 5):");
    for (i, masechta) in Daf::bavli_masechtos_transliterated()
        .iter()
        .take(5)
        .enumerate()
    {
        println!("{}: {}", i, masechta);
    }
}
