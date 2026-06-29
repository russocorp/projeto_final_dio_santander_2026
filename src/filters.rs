use askama::{Result, Values};
use bigdecimal::{BigDecimal, ToPrimitive, Zero};

#[askama::filter_fn]
pub fn formato_brl(valor: &BigDecimal, _values: &dyn Values) -> Result<String> {
    let inteiro = valor.with_scale(0).to_i64().unwrap_or(0);
    let decimais = ((valor - valor.with_scale(0)).abs() * BigDecimal::from(10000))
        .with_scale(0)
        .to_u64()
        .unwrap_or(0);

    let inteiro_str = inteiro
        .abs()
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(|chunk| std::str::from_utf8(chunk).unwrap())
        .collect::<Vec<_>>()
        .join(".");

    let sinal = if *valor < BigDecimal::zero() { "-" } else { "" };

    Ok(format!("{}{},{:04}", sinal, inteiro_str, decimais))
}
