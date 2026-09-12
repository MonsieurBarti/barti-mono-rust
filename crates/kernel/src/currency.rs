/// ISO 4217 table A.1, SIX Group list-one published 2026-01-01.
const CURRENCIES: &[(&str, Option<u8>)] = &[
    ("AED", Some(2)),
    ("AFN", Some(2)),
    ("ALL", Some(2)),
    ("AMD", Some(2)),
    ("AOA", Some(2)),
    ("ARS", Some(2)),
    ("AUD", Some(2)),
    ("AWG", Some(2)),
    ("AZN", Some(2)),
    ("BAM", Some(2)),
    ("BBD", Some(2)),
    ("BDT", Some(2)),
    ("BHD", Some(3)),
    ("BIF", Some(0)),
    ("BMD", Some(2)),
    ("BND", Some(2)),
    ("BOB", Some(2)),
    ("BOV", Some(2)),
    ("BRL", Some(2)),
    ("BSD", Some(2)),
    ("BTN", Some(2)),
    ("BWP", Some(2)),
    ("BYN", Some(2)),
    ("BZD", Some(2)),
    ("CAD", Some(2)),
    ("CDF", Some(2)),
    ("CHE", Some(2)),
    ("CHF", Some(2)),
    ("CHW", Some(2)),
    ("CLF", Some(4)),
    ("CLP", Some(0)),
    ("CNY", Some(2)),
    ("COP", Some(2)),
    ("COU", Some(2)),
    ("CRC", Some(2)),
    ("CUP", Some(2)),
    ("CVE", Some(2)),
    ("CZK", Some(2)),
    ("DJF", Some(0)),
    ("DKK", Some(2)),
    ("DOP", Some(2)),
    ("DZD", Some(2)),
    ("EGP", Some(2)),
    ("ERN", Some(2)),
    ("ETB", Some(2)),
    ("EUR", Some(2)),
    ("FJD", Some(2)),
    ("FKP", Some(2)),
    ("GBP", Some(2)),
    ("GEL", Some(2)),
    ("GHS", Some(2)),
    ("GIP", Some(2)),
    ("GMD", Some(2)),
    ("GNF", Some(0)),
    ("GTQ", Some(2)),
    ("GYD", Some(2)),
    ("HKD", Some(2)),
    ("HNL", Some(2)),
    ("HTG", Some(2)),
    ("HUF", Some(2)),
    ("IDR", Some(2)),
    ("ILS", Some(2)),
    ("INR", Some(2)),
    ("IQD", Some(3)),
    ("IRR", Some(2)),
    ("ISK", Some(0)),
    ("JMD", Some(2)),
    ("JOD", Some(3)),
    ("JPY", Some(0)),
    ("KES", Some(2)),
    ("KGS", Some(2)),
    ("KHR", Some(2)),
    ("KMF", Some(0)),
    ("KPW", Some(2)),
    ("KRW", Some(0)),
    ("KWD", Some(3)),
    ("KYD", Some(2)),
    ("KZT", Some(2)),
    ("LAK", Some(2)),
    ("LBP", Some(2)),
    ("LKR", Some(2)),
    ("LRD", Some(2)),
    ("LSL", Some(2)),
    ("LYD", Some(3)),
    ("MAD", Some(2)),
    ("MDL", Some(2)),
    ("MGA", Some(2)),
    ("MKD", Some(2)),
    ("MMK", Some(2)),
    ("MNT", Some(2)),
    ("MOP", Some(2)),
    ("MRU", Some(2)),
    ("MUR", Some(2)),
    ("MVR", Some(2)),
    ("MWK", Some(2)),
    ("MXN", Some(2)),
    ("MXV", Some(2)),
    ("MYR", Some(2)),
    ("MZN", Some(2)),
    ("NAD", Some(2)),
    ("NGN", Some(2)),
    ("NIO", Some(2)),
    ("NOK", Some(2)),
    ("NPR", Some(2)),
    ("NZD", Some(2)),
    ("OMR", Some(3)),
    ("PAB", Some(2)),
    ("PEN", Some(2)),
    ("PGK", Some(2)),
    ("PHP", Some(2)),
    ("PKR", Some(2)),
    ("PLN", Some(2)),
    ("PYG", Some(0)),
    ("QAR", Some(2)),
    ("RON", Some(2)),
    ("RSD", Some(2)),
    ("RUB", Some(2)),
    ("RWF", Some(0)),
    ("SAR", Some(2)),
    ("SBD", Some(2)),
    ("SCR", Some(2)),
    ("SDG", Some(2)),
    ("SEK", Some(2)),
    ("SGD", Some(2)),
    ("SHP", Some(2)),
    ("SLE", Some(2)),
    ("SOS", Some(2)),
    ("SRD", Some(2)),
    ("SSP", Some(2)),
    ("STN", Some(2)),
    ("SVC", Some(2)),
    ("SYP", Some(2)),
    ("SZL", Some(2)),
    ("THB", Some(2)),
    ("TJS", Some(2)),
    ("TMT", Some(2)),
    ("TND", Some(3)),
    ("TOP", Some(2)),
    ("TRY", Some(2)),
    ("TTD", Some(2)),
    ("TWD", Some(2)),
    ("TZS", Some(2)),
    ("UAH", Some(2)),
    ("UGX", Some(0)),
    ("USD", Some(2)),
    ("USN", Some(2)),
    ("UYI", Some(0)),
    ("UYU", Some(2)),
    ("UYW", Some(4)),
    ("UZS", Some(2)),
    ("VED", Some(2)),
    ("VES", Some(2)),
    ("VND", Some(0)),
    ("VUV", Some(0)),
    ("WST", Some(2)),
    ("XAD", Some(2)),
    ("XAF", Some(0)),
    ("XAG", None),
    ("XAU", None),
    ("XBA", None),
    ("XBB", None),
    ("XBC", None),
    ("XBD", None),
    ("XCD", Some(2)),
    ("XCG", Some(2)),
    ("XDR", None),
    ("XOF", Some(0)),
    ("XPD", None),
    ("XPF", Some(0)),
    ("XPT", None),
    ("XSU", None),
    ("XTS", None),
    ("XUA", None),
    ("XXX", None),
    ("YER", Some(2)),
    ("ZAR", Some(2)),
    ("ZMW", Some(2)),
    ("ZWG", Some(2)),
];

pub fn is_known_currency(code: &str) -> bool {
    row(code).is_some()
}

pub fn fraction_digits(code: &str) -> Option<u8> {
    row(code)?.1
}

pub(crate) fn intern(code: &str) -> Option<&'static str> {
    row(code).map(|(ccy, _)| *ccy)
}

fn row(code: &str) -> Option<&'static (&'static str, Option<u8>)> {
    CURRENCIES
        .binary_search_by_key(&code, |(ccy, _)| *ccy)
        .ok()
        .map(|i| &CURRENCIES[i])
}

#[cfg(test)]
mod tests {
    use super::{fraction_digits, is_known_currency};

    #[test]
    fn usd_is_a_known_currency() {
        assert!(is_known_currency("USD"));
    }

    #[test]
    fn lowercase_usd_is_unknown() {
        assert!(!is_known_currency("usd"));
    }

    #[test]
    fn zzz_is_unknown() {
        assert!(!is_known_currency("ZZZ"));
    }

    #[test]
    fn usd_has_two_fraction_digits() {
        assert_eq!(fraction_digits("USD"), Some(2));
    }

    #[test]
    fn jpy_has_zero_fraction_digits() {
        assert_eq!(fraction_digits("JPY"), Some(0));
    }

    #[test]
    fn bhd_has_three_fraction_digits() {
        assert_eq!(fraction_digits("BHD"), Some(3));
    }

    #[test]
    fn clf_has_four_fraction_digits() {
        assert_eq!(fraction_digits("CLF"), Some(4));
    }

    #[test]
    fn gold_is_known_without_fraction_digits() {
        assert!(is_known_currency("XAU"));
        assert_eq!(fraction_digits("XAU"), None);
    }

    #[test]
    fn unknown_code_has_no_fraction_digits() {
        assert_eq!(fraction_digits("ZZZ"), None);
    }
}
