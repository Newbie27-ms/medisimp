pub fn simplify_report(input: &str, redact_pii: bool) -> String {
    let mut output_lines = Vec::new();
    let mut found_terms = Vec::new();

    let glossary = [
        ("BP", "Blood Pressure (force of blood against artery walls)"),
        ("HTN", "Hypertension (high blood pressure)"),
        ("HbA1c", "Glycated Hemoglobin (3-month average blood sugar level)"),
        ("WBC", "White Blood Cells (immune system cells that fight infection)"),
        ("ECG", "Electrocardiogram (test that checks heart electrical activity)"),
    ];

    for line in input.lines() {
        let trimmed = line.trim();

        if redact_pii
            && (trimmed.starts_with("Patient:")
                || trimmed.starts_with("DOB:")
                || trimmed.starts_with("ID:"))
        {
            let label = trimmed.split(':').next().unwrap_or("Field");
            output_lines.push(format!("{}: [REDACTED]", label));
            continue;
        }

        for (term, explanation) in &glossary {
            if line.contains(term) && !found_terms.contains(&(*term, *explanation)) {
                found_terms.push((*term, *explanation));
            }
        }

        output_lines.push(line.to_string());
    }

    let mut result = String::from("## Cleaned Medical Report\n\n");
    result.push_str(&output_lines.join("\n"));

    if !found_terms.is_empty() {
        result.push_str("\n\n## Plain-Language Glossary\n");
        for (term, explanation) in found_terms {
            result.push_str(&format!("- **{}**: {}\n", term, explanation));
        }
    }

    result
}
