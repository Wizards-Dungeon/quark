// Add more EditionIDs and their corresponding GLVKs here

pub fn get_gvlk(edition: &str) -> &'static str {
    match edition {
        "Enterprise" => "NPPR9-FWDCX-D2C8J-H872K-2YT43",
        "EnterpriseN" => "DPH2V-TTNVB-4X9Q3-TJR4H-KHJW4",

        "EnterpriseS" => "M7XTQ-FN8P6-TTKYV-9D4CC-J462D",
        "EnterpriseSN" => "92NFX-8DJQP-P6BBQ-THF9C-7CG2H",

        "EnterpriseG" => "YYVX9-NTFWV-6MDM3-9PT4T-4M68B",
        "EnterpriseGN" => "44RPN-FTY23-9VTTB-MP9BX-T84FV",

        "Professional" => "W269N-WFGWX-YVC9B-4J6C9-T83GX",
        "ProfessionalN" => "MH37W-N47XK-V7XM9-C7227-GCQG9",

        "Core" => "TX9XD-98N7V-6WMQ6-BX7FG-H8Q99",
        "CoreN" => "3KHY7-WNT83-DGQKR-F7HPR-844BM",
        "CoreCountrySpecific" => "PVMJN-6DFY6-9CCP6-7BKTT-D3WVR",
        "CoreSingleLanguage" => "7HNRX-D7KGG-3K4RQ-4WPJ4-YTDFH",

        "Education" => "NW6C2-QMPVW-D7KKK-3GKT6-VCFB2",
        "EducationN" => "2WH4N-8QGBV-H22JP-CT43Q-MDWWJ",

        "ProfessionalWorkstation" => "NRG8B-VKK3Q-CXVCJ-9G2XF-6Q84J",
        "ProfessionalWorkstationN" => "9FNHH-K3HBT-3W4TD-6383H-6XYWF",

        "ProfessionalEducation" => "6TP4R-GNPTD-KYYHQ-7B7DP-J447Y",
        "ProfessionalEducationN" => "YVWGF-BXNMC-HTQYQ-CPQ99-66QFC",

        // "EditionID" => "GLVK",

        _ => crate::bail!("Unsupported edition"),
    }
}