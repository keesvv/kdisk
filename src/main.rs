use sysinfo::{SystemExt, DiskExt, Disk, RefreshKind};
use partition_identity::{PartitionID, PartitionSource};
use pretty_bytes::converter;

struct Bar {
    value: u64,
    max: u64,
    length: u32
}

impl Bar {
    fn new(value: u64, max: u64) -> Self {
        Bar{
            value,
            max,
            length: 25
        }
    }

    fn get_progress(&self) -> f64 {
        if self.max <= 0 {
            return 0.0
        }
        return self.value as f64 / self.max as f64
    }

    fn get_progress_percent(&self) -> u8 {
        return (self.get_progress() * 100.0).round() as u8
    }

    fn format_str(&self) -> String {
        let value = (self.get_progress() * self.length as f64).round() as u32;

        return format!(
            "\x1b[95m{}\x1b[90m{}\x1b[0m",
            "|".repeat(value as usize),
            "|".repeat((self.length - value) as usize)
        );
    }
}

fn format_disk(d: &Disk) -> String {
    let bar = Bar::new(
        d.get_total_space() - d.get_available_space(),
        d.get_total_space()
    );

    let label = match PartitionID::get_source(
        PartitionSource::Label,
        d.get_name().to_str().unwrap()
    ) {
        Some(l) => l.to_string().replace("LABEL=", ""),
        None => String::from(
            d.get_mount_point().to_str().unwrap_or(
                d.get_name().to_str().unwrap()
            )
        )
    };

    format!(
        "{0} \x1b[1m{1: <3}%\x1b[0m {2: >10} \x1b[37m{3}",
        bar.format_str(),
        bar.get_progress_percent(),
        converter::convert(d.get_available_space() as f64),
        label
    )
}

fn main() {
    let system = sysinfo::System::new_with_specifics(
        RefreshKind::new()
            .with_disks()
            .with_disks_list()
    );

    let disks = system.get_disks();
    let fmt_disks: Vec<String> = disks.iter().map(format_disk).collect();

    for disk in fmt_disks {
        println!("{}", disk);
    }
}
