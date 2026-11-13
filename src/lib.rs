pub fn simplify_report(input: &str, redact_pii: bool) -> String {
    let mut output_lines = Vec::new();
    let mut found_terms: Vec<(&str, &str)> = Vec::new();

    // Comprehensive clinical dictionary covering labs, vitals, diagnoses, and prescriptions
    let glossary = [
        ("BP", "Blood Pressure (force of blood against artery walls)"),
        ("HR", "Heart Rate (beats per minute)"),
        ("SpO2", "Peripheral Oxygen Saturation (percentage of oxygen in the blood)"),
        ("BMI", "Body Mass Index (body fat estimate based on height and weight)"),
        ("HTN", "Hypertension (high blood pressure)"),
        ("DM", "Diabetes Mellitus (chronic high blood sugar condition)"),
        ("T2DM", "Type 2 Diabetes Mellitus (adult-onset insulin resistance)"),
        ("COPD", "Chronic Obstructive Pulmonary Disease (long-term lung airflow blockage)"),
        ("MI", "Myocardial Infarction (heart attack)"),
        ("CVA", "Cerebrovascular Accident (stroke)"),
        ("SOB", "Shortness of Breath (dyspnea)"),
        ("Hx", "Medical History (past health background)"),
        ("Dx", "Diagnosis (identified medical condition)"),
        ("Rx", "Prescription (ordered medication or treatment)"),
        ("Tx", "Treatment (medical management plan)"),
        ("CBC", "Complete Blood Count (broad panel measuring blood cells)"),
        ("WBC", "White Blood Cells (immune cells that fight infection)"),
        ("RBC", "Red Blood Cells (cells that carry oxygen throughout the body)"),
        ("HGB", "Hemoglobin (oxygen-carrying protein in red blood cells)"),
        ("PLT", "Platelets (blood cells responsible for clotting)"),
        ("MCV", "Mean Corpuscular Volume (average size of red blood cells)"),
        ("HbA1c", "Glycated Hemoglobin (3-month average blood sugar level)"),
        ("LDL", "Low-Density Lipoprotein ('bad' cholesterol that can build up in arteries)"),
        ("HDL", "High-Density Lipoprotein ('good' cholesterol that helps clear arteries)"),
        ("TSH", "Thyroid-Stimulating Hormone (regulates thyroid gland function)"),
        ("ALT", "Alanine Aminotransferase (liver enzyme checked for liver health)"),
        ("AST", "Aspartate Aminotransferase (enzyme checked for liver or muscle injury)"),
        ("eGFR", "Estimated Glomerular Filtration Rate (measures kidney filtering efficiency)"),
        ("CRP", "C-Reactive Protein (blood marker indicating inflammation)"),
        ("ECG", "Electrocardiogram (test recording electrical signals in the heart)"),
        ("EKG", "Electrocardiogram (test recording electrical signals in the heart)"),
        ("CXR", "Chest X-Ray (imaging scan of the heart, lungs, and chest bones)"),
        ("MRI", "Magnetic Resonance Imaging (detailed magnetic scan of soft tissues)"),
        ("CT", "Computed Tomography (cross-sectional X-ray imaging scan)"),
        ("PO", "Per Os (medication taken by mouth)"),
        ("IV", "Intravenous (medication delivered directly into a vein)"),
        ("BID", "Bis in Die (take twice a day)"),
        ("TID", "Ter in Die (take three times a day)"),
        ("QID", "Quater in Die (take four times a day)"),
        ("PRN", "Pro Re Nata (take only as needed)"),
    ];

    for line in input.lines() {
        let trimmed = line.trim();

        // 1. Redact explicit patient header lines
        if redact_pii && is_sensitive_header(trimmed) {
            let label = trimmed.split(':').next().unwrap_or("Field");
            output_lines.push(format!("{}: [REDACTED]", label));
            continue;
        }

        // 2. Redact inline emails, dates (DD.MM.YYYY or YYYY-MM-DD), and phone/ID numbers
        let processed_line = if redact_pii {
            redact_inline_tokens(line)
        } else {
            line.to_string()
        };

        // 3. Whole-word scan for medical abbreviations
        for &(term, explanation) in &glossary {
            if contains_whole_word(&processed_line, term)
                && !found_terms.iter().any(|&(t, _)| t == term)
            {
                found_terms.push((term, explanation));
            }
        }

        output_lines.push(processed_line);
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

fn is_sensitive_header(trimmed: &str) -> bool {
    let lower = trimmed.to_lowercase();
    let prefixes = [
        "patient:", "name:", "dob:", "birth:", "id:", "mrn:", "address:", "phone:", "insurance:",
    ];
    prefixes.iter().any(|p| lower.starts_with(p))
}

fn redact_inline_tokens(line: &str) -> String {
    let tokens: Vec<String> = line
        .split_whitespace()
        .map(|token| {
            let clean = token.trim_matches(|c: char| c == ',' || c == '.' || c == ';' || c == ')');
            if clean.contains('@') && clean.contains('.') {
                "[REDACTED-EMAIL]".to_string()
            } else if is_date_like(clean) {
                "[REDACTED-DATE]".to_string()
            } else if is_phone_or_id(clean) {
                "[REDACTED-ID]".to_string()
            } else {
                token.to_string()
            }
        })
        .collect();
    tokens.join(" ")
}

fn is_date_like(s: &str) -> bool {
    let parts_dot: Vec<&str> = s.split('.').collect();
    let parts_dash: Vec<&str> = s.split('-').collect();
    let is_digits = |arr: &[&str]| arr.len() == 3 && arr.iter().all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()));
    is_digits(&parts_dot) || is_digits(&parts_dash)
}

fn is_phone_or_id(s: &str) -> bool {
    let digit_count = s.chars().filter(|c| c.is_ascii_digit()).count();
    (s.starts_with('+') && digit_count >= 7) || (s.starts_with("PAT-") && digit_count >= 4)
}

fn contains_whole_word(line: &str, word: &str) -> bool {
    line.split(|c: char| !c.is_alphanumeric())
        .any(|token| token.eq_ignore_ascii_case(word))
}
