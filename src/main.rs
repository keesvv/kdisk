use sysinfo::{SystemExt, DiskExt, DiskType, RefreshKind};
use partition_identity::{PartitionID, PartitionSource};

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

fn main() {
    let system = sysinfo::System::new_with_specifics(
        RefreshKind::new()
            .with_disks()
            .with_disks_list()
    );

    let disks = system.get_disks();

    for d in disks {
        if d.get_type() == DiskType::Unknown(-1) {
            continue;
        }

        let bar = Bar::new(
            d.get_total_space() - d.get_available_space(),
            d.get_total_space()
        );

        let label = match PartitionID::get_source(
            PartitionSource::Label,
            d.get_name().to_str().unwrap()
        ) {
            Some(l) => l.to_string().replace("LABEL=", ""),
            None => String::from(d.get_name().to_str().unwrap())
        };

        println!(
            "\x1b[37m{0: <10}\t{1} \x1b[1m{2}%\x1b[0m",
            label,
            bar.format_str(),
            bar.get_progress_percent()
        );
    }
}
