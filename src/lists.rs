pub fn vec_to_string(content: Vec<String>) -> String {
    let mut output = "".to_string();
    let mut container = None;

    let switch_indicator = "$";

    for segment in content {
        let target_container = (|| {
            if let Some(container) = container {
                if !segment.to_lowercase().contains(container) {
                    return container;
                }
            }

            for char in "[]<>()abcdefghijklmnopqrstuvwxyz".chars() {
                if !segment.to_lowercase().contains(char) {
                    return char;
                }
            }

            panic!("all chars taken :sob:")
        })();

        if Some(target_container) != container || segment.starts_with(&switch_indicator) {
            output += &format!("{switch_indicator}{target_container}");
            container = Some(target_container);
        }

        output += &format!("{segment}{}", container.unwrap());
    }

    output
}
