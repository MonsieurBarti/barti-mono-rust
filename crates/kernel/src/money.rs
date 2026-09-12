use crate::currency::intern;
use std::cmp::Ordering;

const MAX_AMOUNT: i64 = (1_i64 << 53) - 1;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Money {
    amount: i64,
    currency: &'static str,
}

impl Money {
    pub fn create(amount: i64, currency: &str) -> Option<Self> {
        if !(-MAX_AMOUNT..=MAX_AMOUNT).contains(&amount) {
            return None;
        }
        Some(Self {
            amount,
            currency: intern(currency)?,
        })
    }

    pub fn add(&self, other: &Self) -> Option<Self> {
        if self.currency != other.currency {
            return None;
        }
        Self::create(self.amount.checked_add(other.amount)?, self.currency)
    }

    pub fn subtract(&self, other: &Self) -> Option<Self> {
        if self.currency != other.currency {
            return None;
        }
        Self::create(self.amount.checked_sub(other.amount)?, self.currency)
    }

    pub fn compare(&self, other: &Self) -> Option<Ordering> {
        (self.currency == other.currency).then(|| self.amount.cmp(&other.amount))
    }

    pub fn allocate(&self, weights: &[u64]) -> Option<Vec<Self>> {
        if weights.is_empty() {
            return None;
        }
        let total = weights.iter().copied().map(u128::from).sum::<u128>();
        let total = i128::try_from(total).ok()?;
        if total == 0 {
            return None;
        }
        let amount = i128::from(self.amount);
        let mut parts: Vec<i64> = weights
            .iter()
            .map(|&w| i64::try_from(amount * i128::from(w) / total).ok())
            .collect::<Option<_>>()?;
        let mut rem = self.amount - parts.iter().copied().sum::<i64>();
        let step = rem.signum();
        let mut i = 0;
        while rem != 0 {
            parts[i] += step;
            rem -= step;
            i += 1;
            if i == parts.len() {
                i = 0;
            }
        }
        Some(
            parts
                .into_iter()
                .map(|amount| Self {
                    amount,
                    currency: self.currency,
                })
                .collect(),
        )
    }

    pub fn to_plain(&self) -> (i64, String) {
        (self.amount, self.currency.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::Money;
    use std::cmp::Ordering;

    const MAX: i64 = 9_007_199_254_740_991;

    fn usd(amount: i64) -> Money {
        Money::create(amount, "USD").expect("USD")
    }

    #[test]
    fn create_accepts_zero_usd() {
        assert_eq!(usd(0).to_plain(), (0, "USD".to_owned()));
    }

    #[test]
    fn create_rejects_unknown_currency() {
        assert!(Money::create(1, "ZZZ").is_none());
    }

    #[test]
    fn create_rejects_lowercase_currency() {
        assert!(Money::create(1, "usd").is_none());
    }

    #[test]
    fn create_rejects_amount_over_json_safe_max() {
        assert!(Money::create(MAX + 1, "USD").is_none());
    }

    #[test]
    fn create_rejects_amount_under_json_safe_min() {
        assert!(Money::create(-MAX - 1, "USD").is_none());
    }

    #[test]
    fn create_accepts_json_safe_bounds() {
        assert_eq!(usd(MAX).to_plain(), (MAX, "USD".to_owned()));
        assert_eq!(usd(-MAX).to_plain(), (-MAX, "USD".to_owned()));
    }

    #[test]
    fn add_same_currency() {
        assert_eq!(
            usd(2).add(&usd(3)).unwrap().to_plain(),
            (5, "USD".to_owned())
        );
    }

    #[test]
    fn add_rejects_currency_mismatch() {
        let eur = Money::create(1, "EUR").unwrap();
        assert!(usd(1).add(&eur).is_none());
    }

    #[test]
    fn add_rejects_out_of_range_sum() {
        assert!(usd(MAX).add(&usd(1)).is_none());
    }

    #[test]
    fn subtract_same_currency() {
        assert_eq!(
            usd(5).subtract(&usd(3)).unwrap().to_plain(),
            (2, "USD".to_owned())
        );
    }

    #[test]
    fn compare_same_currency() {
        assert_eq!(usd(1).compare(&usd(2)), Some(Ordering::Less));
        assert_eq!(usd(2).compare(&usd(2)), Some(Ordering::Equal));
        assert_eq!(usd(3).compare(&usd(2)), Some(Ordering::Greater));
    }

    #[test]
    fn compare_rejects_currency_mismatch() {
        let eur = Money::create(1, "EUR").unwrap();
        assert!(usd(1).compare(&eur).is_none());
    }

    #[test]
    fn allocate_sends_remainder_pennies_to_the_first_recipients() {
        let parts = usd(5).allocate(&[1, 1, 1]).unwrap();
        let plains: Vec<(i64, String)> = parts.iter().map(Money::to_plain).collect();
        assert_eq!(
            plains,
            vec![
                (2, "USD".to_owned()),
                (2, "USD".to_owned()),
                (1, "USD".to_owned()),
            ]
        );
    }

    #[test]
    fn allocate_parts_sum_to_the_original() {
        let parts = usd(-5).allocate(&[1, 1, 1]).unwrap();
        let sum: i64 = parts.iter().map(|p| p.to_plain().0).sum();
        assert_eq!(sum, -5);
        assert_eq!(
            parts.iter().map(Money::to_plain).collect::<Vec<_>>(),
            vec![
                (-2, "USD".to_owned()),
                (-2, "USD".to_owned()),
                (-1, "USD".to_owned()),
            ]
        );
    }

    #[test]
    fn allocate_rejects_empty_or_zero_weights() {
        assert!(usd(5).allocate(&[]).is_none());
        assert!(usd(5).allocate(&[0, 0]).is_none());
    }
}
