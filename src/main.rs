use anyhow::Result;
use std::{fmt::Debug, fs, time::SystemTime};

use chrono::{DateTime, Datelike, Local, NaiveDate};
use clap::Parser;
use humanize_duration::prelude::DurationExt;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    date_format: Option<String>,

    #[arg(short, long)]
    elapsed: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let age = get_system_age()?;

    let datetime: DateTime<Local> = DateTime::from(age);
    let date: NaiveDate = datetime.date_naive();

    let mut date_string = match args.date_format {
        Some(n) => date.format(&n).to_string(),
        None => date.to_string(),
    };

    if args.elapsed {
        let elapsed = (only_date(Local::now().date_naive()) - date).to_std()?;
        date_string.push_str(&format!(
            " ({} ago)",
            elapsed.human(humanize_duration::Truncate::Day)
        ));
    }

    println!("{date_string}");

    Ok(())
}

fn get_system_age() -> Result<SystemTime> {
    let root_folder = fs::metadata("/root")?;

    let created = root_folder.created()?;

    Ok(created)
}

fn only_date(date: NaiveDate) -> NaiveDate {
    NaiveDate::from_ymd_opt(date.year(), date.month0(), date.day0())
        .expect("this should always work")
}
