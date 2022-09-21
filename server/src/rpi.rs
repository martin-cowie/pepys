use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;

/// Expose Raspberry Pi hardware details.
#[derive(Debug)]
pub struct RpiProcInfo {
    pub manufacturer: String,
    pub hardware: String,
    pub revision: String,
    pub serial: String,
    pub model: String
}

impl Default for RpiProcInfo {
    fn default() -> Self {
        Self {
            manufacturer: "unknown".to_string(),
            hardware: "unknown".to_string(),
            revision: "unknown".to_string(),
            serial: "unknown".to_string(),
            model: "unknown".to_string()
        }
    }
}

impl RpiProcInfo {
    pub(crate) fn from(file: &File) -> Result<RpiProcInfo, Box<dyn Error>> {
        let lines = io::BufReader::new(file).lines();
        let mut result: RpiProcInfo = Default::default();

        for line in lines.flatten() {
            if let Some((key, value)) = line.split_once(": ") {
                match key.trim() {
                    "Hardware" => result.hardware = value.trim().to_string(),
                    "Revision" => result.revision = value.trim().to_string(),
                    "Serial" => result.serial = value.trim().to_string(),
                    "Model" => {
                        result.model = value.trim().to_string();
                        if result.model.starts_with("Raspberry Pi") {
                            result.manufacturer = "Raspberry Pi Foundation".to_string();
                        }
                    },
                    _ => { /* do nothing */ }
                }
            }
        }

        Ok(result)
    }

    pub fn new() -> Result<RpiProcInfo, Box<dyn Error>> {
        let file = File::open("/proc/cpuinfo")?;
        Self::from(&file)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use tempfile::tempfile;
    use std::io::{Write, Seek, SeekFrom};

    #[test]
    fn test_ok() {
        let mut tmpfile: File = tempfile().unwrap();
        write!(tmpfile, r#"
processor       : 3
model name      : ARMv7 Processor rev 4 (v7l)
BogoMIPS        : 38.40
Features        : half thumb fastmult vfp edsp neon vfpv3 tls vfpv4 idiva idivt vfpd32 lpae evtstrm crc32
CPU implementer : 0x41
CPU architecture: 7
CPU variant     : 0x0
CPU part        : 0xd03
CPU revision    : 4

Hardware        : BCM2835
Revision        : a02082
Serial          : 000000008a7cd411
Model           : Raspberry Pi 3 Model B Rev 1.2
"#).unwrap();
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        println!("Loading temporary file {:#?}", tmpfile);


        let info = RpiProcInfo::from(&tmpfile).unwrap();
        assert_eq!(info.hardware, "BCM2835");
        assert_eq!(info.revision, "a02082");
        assert_eq!(info.serial, "000000008a7cd411");
        assert_eq!(info.model, "Raspberry Pi 3 Model B Rev 1.2");
        assert_eq!(info.manufacturer, "Raspberry Pi Foundation");
    }

}
