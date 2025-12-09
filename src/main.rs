use std::io::IsTerminal;
use std::num::ParseIntError;
use std::process::ExitCode;
use std::str::FromStr;

use rust_decimal::Decimal;

const ONE_KIB: i64 = 1024i64;
const ONE_MIB: i64 = ONE_KIB * ONE_KIB;
const ONE_GIB: i64 = ONE_MIB * ONE_KIB;
const ONE_TIB: i64 = ONE_GIB * ONE_KIB;
const ONE_PIB: i64 = ONE_TIB * ONE_KIB;
const ONE_EIB: i64 = ONE_PIB * ONE_KIB;
// const ZiBn: i64 = EiBn * KiBn;
// const YiBn: i64 = ZiBn * KiBn;

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

impl From<&i64> for Unit {
	fn from(value: &i64) -> Self {
		match value {
			bytes if *bytes < ONE_KIB => Unit::Bytes,
			kib if *kib < ONE_MIB => Unit::KiB,
			mib if *mib < ONE_GIB => Unit::MiB,
			gib if *gib < ONE_TIB => Unit::GiB,
			tib if *tib < ONE_PIB => Unit::TiB,
			pib if *pib < ONE_EIB => Unit::PiB,
			_ => Unit::EiB,
			// zib if *zib < YiBn => Unit::ZiB,
			// _ => Unit::YiB
		}
	}
}

impl From<i64> for Unit {
	fn from(value: i64) -> Self {
		Unit::from(&value)
	}
}

fn main() -> ExitCode {
	let args: Vec<String> = std::env::args().collect();
	// println!("{:?}", args);
	let number: Result<i64, ParseIntError> = get_number(&args);

	match number {
		Ok(result) => {
			let unit: Unit = get_unit(&args, result);
			println!("{}", convert(&unit, result));
			ExitCode::SUCCESS
		}
		Err(e) => {
			eprintln!("error: {e}");
			ExitCode::FAILURE
		}
	}
}

fn get_number(args: &Vec<String>) -> Result<i64, ParseIntError> {
	if std::io::stdin().is_terminal() {
		get_number_from_args(args)
	} else {
		match get_number_from_stdin() {
			Some(value) => value,
			None => get_number_from_args(args),
		}
	}
}

fn get_number_from_args(args: &Vec<String>) -> Result<i64, ParseIntError> {
	match args.as_slice() {
		[_, number] => i64::from_str(number),
		[_, _, number] => i64::from_str(number),
		_ => panic!("usage: number"),
	}
}

fn get_number_from_stdin() -> Option<Result<i64, ParseIntError>> {
	let mut buffer: String = String::new();
	match std::io::stdin().read_line(&mut buffer) {
		Ok(size) => {
			if size > 0 {
				let without_newlines = buffer.trim_end_matches(['\n', '\r']);
				Some(i64::from_str(without_newlines))
			} else {
				None
			}
		}
		Err(_) => None,
	}
}

fn get_unit(args: &Vec<String>, number: i64) -> Unit {
	match args.as_slice() {
		[_, unit] => Unit::try_from(unit).unwrap_or(Unit::from(number)),
		[_, unit, _] => Unit::try_from(unit).unwrap_or(Unit::from(number)),
		_ => Unit::from(number),
	}
}

fn convert(unit: &Unit, number: i64) -> String {
	let divisor: i64 = get_divisor(unit);
	let decimal = Decimal::new(number, 0) / Decimal::new(divisor, 0);
	let decimal_places = if number > divisor { 2 } else { 3 };
	format!("{} {}", decimal.round_dp(decimal_places), unit)
}

fn get_divisor(unit: &Unit) -> i64 {
	match unit {
		Unit::Bytes => 1i64,
		Unit::KiB => ONE_KIB,
		Unit::MiB => ONE_MIB,
		Unit::GiB => ONE_GIB,
		Unit::TiB => ONE_TIB,
		Unit::PiB => ONE_PIB,
		Unit::EiB => ONE_EIB,
		_other => panic!("cannot handle unit: {_other}"),
	}
}
