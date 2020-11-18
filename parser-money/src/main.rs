use std::fmt::Formatter;
use std::string::ParseError;
use std::str::FromStr;
use std::num::ParseFloatError;

fn parse_money_v1(input: &str) -> (i32, String){
    let parts: Vec<&str> = input.split_whitespace().collect();
    let maybe_amount = parts[0].parse();
    if maybe_amount.is_err(){
        return (-1, "invalid".to_string());
    }
    let currency = parts[1].to_string();
    return (maybe_amount.unwrap(), currency);
}

// "10 rmb", "20.1 dollar"
// 应该至少支持f64的格式 parse自动根据输出参数调整匹配
fn parse_money_v2(input: &str) -> (f64, String){
    let parts: Vec<&str> = input.split_whitespace().collect();
    let maybe_amount = parts[0].parse();
    if maybe_amount.is_err(){
        return (-1.0, "invalid".to_string());
    }
    let currency = parts[1].to_string();
    return (maybe_amount.unwrap(), currency);
}


// 返回类型修改为更友好的Result类型，除了便于使用外，还可以简化内部的处理逻辑
// 比如可以使用? 来处理可能的错误
fn parse_money_v3(input: &str) -> Result<(f64, String), MoneyError>{
    let parts: Vec<&str> = input.split_whitespace().collect();
    let maybe_amount: f64 = parts[0].parse()?;
    let currency = parts[1].to_string();
    Ok((maybe_amount, currency))
}



// 如果输入类型为"10" 我们需要返回错误信息
// 针对错误类型我们使用Enum管理
#[derive(Debug)]
pub enum MoneyError{
    ParseError,
    ParseFormatting(String),
    ParseCurrency(String)
}

impl std::error::Error for MoneyError{
    fn description(&self) -> &str{
        match *self {
            MoneyError::ParseError => "invalid input",
            _ => "error",
        }
    }
}

impl std::fmt::Display for MoneyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       match *self {
           MoneyError::ParseError => f.write_str("invalid input"),
           _ => f.write_str("error"),
       }
    }
}

impl From<ParseError> for MoneyError{
    fn from(_: ParseError) -> Self {
        MoneyError::ParseError
    }
}

fn parse_money_v4(input: &str) -> Result<(f64, String), MoneyError>{
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2{
        return Err(MoneyError::ParseError);
    }
    let maybe_amount = parts[0].parse()?;
    let currency = parts[1].to_string();
    Ok((maybe_amount, currency))
}


fn parse_money_v5(input: &str) -> Result<(f64, String), MoneyError>{
    let parts: Vec<&str> = input.split_whitespace().collect();

    match parts[..] {
        [amount, currency] => Ok((amount.parse()?, currency.to_string())),
        _ => Err(MoneyError::ParseFormatting("invalid".into()))
    }
}

#[derive(Debug)]
enum Currency{
    Dollar,
    Euro,
}

impl std::str::FromStr for Currency{
    type Err = MoneyError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "dollar" | "$" => Ok(Currency::Dollar),
            "euro" | "eur" => Ok(Currency::Euro),
            _ => Err(MoneyError::ParseCurrency("Unknown currency".into())),
        }
    }
}


// --------- Last Version
#[derive(Debug)]
struct Money{
    amount: f64,
    currency: Currency,
}

impl Money{
    fn new(amount: f64, currency: Currency) -> Money{
        Money{
            amount,
            currency
        }
    }
}

impl From<ParseFloatError> for MoneyError{
    fn from(e: ParseFloatError) -> Self{
        MoneyError::ParseError
    }
}

impl std::str::FromStr for Money{
    type Err = MoneyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();

        match parts[..] {
            [amount, currency] => Ok(Money::new(amount.parse()?, currency.parse()?)),
            _ => Err(MoneyError::ParseFormatting("invalid".into()))
        }
    }
}

impl std::fmt::Display for Money{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?}", self.amount, self.currency)
    }
}

fn main() {
     println!("{:?}", "10.23 dollar".parse::<Money>());
     println!("{:?}", "10 euro".parse::<Money>());
}
