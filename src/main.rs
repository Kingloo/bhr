use std::io::IsTerminal;
use std::process::ExitCode;
use std::str::FromStr;

use bigdecimal::num_bigint::{BigUint, ParseBigIntError};
use bigdecimal::{BigDecimal, FromPrimitive, One};

const ONE_KIB: u128 = 1024u128;
const ONE_MIB: u128 = ONE_KIB * ONE_KIB;
const ONE_GIB: u128 = ONE_MIB * ONE_KIB;
const ONE_TIB: u128 = ONE_GIB * ONE_KIB;
const ONE_PIB: u128 = ONE_TIB * ONE_KIB;
const ONE_EIB: u128 = ONE_PIB * ONE_KIB;
const ONE_ZIB: u128 = ONE_EIB * ONE_KIB;
const ONE_YIB: u128 = ONE_ZIB * ONE_KIB;

enum Unit {
	Bytes,
	KiB,
	MiB,
	GiB,
	TiB,
	PiB,
	EiB,
	ZiB,
	YiB,
}

impl std::fmt::Display for Unit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Unit::Bytes => write!(f, "bytes"),
			Unit::KiB => write!(f, "KiB"),
			Unit::MiB => write!(f, "MiB"),
			Unit::GiB => write!(f, "GiB"),
			Unit::TiB => write!(f, "TiB"),
			Unit::PiB => write!(f, "PiB"),
			Unit::EiB => write!(f, "EiB"),
			Unit::ZiB => write!(f, "ZiB"),
			Unit::YiB => write!(f, "YiB"),
		}
	}
}

impl TryFrom<&str> for Unit {
	type Error = ();

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			"k" => Ok(Unit::KiB),
			"m" => Ok(Unit::MiB),
			"g" => Ok(Unit::GiB),
			"t" => Ok(Unit::TiB),
			"p" => Ok(Unit::PiB),
			"e" => Ok(Unit::EiB),
			"z" => Ok(Unit::ZiB),
			"y" => Ok(Unit::YiB),
			_ => Err(()),
		}
	}
}

impl TryFrom<&String> for Unit {
	type Error = ();

	fn try_from(value: &String) -> Result<Self, Self::Error> {
		Unit::try_from(value.as_str())
	}
}

impl From<&BigUint> for Unit {
	fn from(value: &BigUint) -> Self {
		match value {
			bytes if *bytes < BigUint::from_u128(ONE_KIB).unwrap() => Unit::Bytes,
			kib if *kib < BigUint::from_u128(ONE_MIB).unwrap() => Unit::KiB,
			mib if *mib < BigUint::from_u128(ONE_GIB).unwrap() => Unit::MiB,
			gib if *gib < BigUint::from_u128(ONE_TIB).unwrap() => Unit::GiB,
			tib if *tib < BigUint::from_u128(ONE_PIB).unwrap() => Unit::TiB,
			pib if *pib < BigUint::from_u128(ONE_EIB).unwrap() => Unit::PiB,
			eib if *eib < BigUint::from_u128(ONE_ZIB).unwrap() => Unit::EiB,
			zib if *zib < BigUint::from_u128(ONE_YIB).unwrap() => Unit::ZiB,
			_ => Unit::YiB,
		}
	}
}

impl From<BigUint> for Unit {
	fn from(value: BigUint) -> Self {
		Unit::from(&value)
	}
}

fn main() -> ExitCode {
	let args: Vec<String> = std::env::args().collect();
	// println!("{:?}", args);
	let number: Result<BigUint, ParseBigIntError> = get_number(&args);

	match number {
		Ok(result) => {
			let unit: Unit = get_unit(&args, &result);
			println!("{}", convert(&unit, &result));
			ExitCode::SUCCESS
		}
		Err(e) => {
			eprintln!("error: {e}");
			ExitCode::FAILURE
		}
	}
}

fn get_number(args: &Vec<String>) -> Result<BigUint, ParseBigIntError> {
	if std::io::stdin().is_terminal() {
		get_number_from_args(args)
	} else {
		match get_number_from_stdin() {
			Some(value) => value,
			None => get_number_from_args(args),
		}
	}
}

fn get_number_from_args(args: &Vec<String>) -> Result<BigUint, ParseBigIntError> {
	match args.as_slice() {
		[_, number] => BigUint::from_str(number),
		[_, _, number] => BigUint::from_str(number),
		_ => panic!("usage: number"),
	}
}

fn get_number_from_stdin() -> Option<Result<BigUint, ParseBigIntError>> {
	let mut buffer: String = String::new();
	match std::io::stdin().read_line(&mut buffer) {
		Ok(size) => {
			if size > 0 {
				let without_newlines = buffer.trim_end_matches(['\n', '\r']);
				Some(BigUint::from_str(without_newlines))
			} else {
				None
			}
		}
		Err(_) => None,
	}
}

fn get_unit(args: &Vec<String>, number: &BigUint) -> Unit {
	match args.as_slice() {
		[_, unit] => Unit::try_from(unit).unwrap_or(Unit::from(number)),
		[_, unit, _] => Unit::try_from(unit).unwrap_or(Unit::from(number)),
		_ => Unit::from(number),
	}
}

fn convert(unit: &Unit, number: &BigUint) -> String {
	let divisor: BigUint = get_divisor(unit);
	let decimal = BigDecimal::new(number.clone().into(), 0) / BigDecimal::new(divisor.clone().into(), 0);
	let decimal_places = if number > &divisor { 2 } else { 3 };
	format!("{} {}", decimal.round(decimal_places), unit)
}

fn get_divisor(unit: &Unit) -> BigUint {
	match unit {
		Unit::Bytes => BigUint::one(),
		Unit::KiB => BigUint::from_u128(ONE_KIB).unwrap(),
		Unit::MiB => BigUint::from_u128(ONE_MIB).unwrap(),
		Unit::GiB => BigUint::from_u128(ONE_GIB).unwrap(),
		Unit::TiB => BigUint::from_u128(ONE_TIB).unwrap(),
		Unit::PiB => BigUint::from_u128(ONE_PIB).unwrap(),
		Unit::EiB => BigUint::from_u128(ONE_EIB).unwrap(),
		Unit::ZiB => BigUint::from_u128(ONE_ZIB).unwrap(),
		Unit::YiB => BigUint::from_u128(ONE_YIB).unwrap(),
	}
}
