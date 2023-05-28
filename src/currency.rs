pub struct CurrencyInt<const EXPONENT: u8> {
    currency_code: u32,
    amount_main: u32,
    amount_sub: u32,
}

pub trait CurrencyImpl {}
impl CurrencyImpl for Currency {}

pub struct Currency {
    pub symbol: &'static str,
    pub code: u32,
    pub exponent: u8,
}

pub struct CurrencyAmount {
    pub currency: Currency,
    pub amount: u32,
}

impl CurrencyAmount {
    pub fn from_minor(currency: Currency, minor_amount: u32) -> Self {
        Self {
            currency: currency,
            amount: minor_amount,
        }
    }
}

impl std::fmt::Display for CurrencyAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.currency.exponent == 0 {
            write!(f, "{} {}", self.currency.symbol, self.amount)
        } else {
            write!(f, "{} {}", self.currency.symbol, (self.amount as f64) / 10u32.pow(self.currency.exponent as u32) as f64)
        }
    }
}

impl Currency {
    const JPY: Self = Self {
        symbol: "JPY",
        code: 392,
        exponent: 0,
    };
    const USD: Self = Self {
        symbol: "USD",
        code: 840,
        exponent: 2,
    };
    const XTS: Self = Self {
        symbol: "XTS",
        code: 963,
        exponent: 0,
    };
}
