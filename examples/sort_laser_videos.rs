use bitvec::prelude::*;
use clap::Parser;
use itertools::Itertools;
use klv_uas::{klv_packet::KlvPacket, klv_value::KlvValue, tag::Tag};
use ts_analyzer::reader::TSReader;
use std::{fs::{self, File}, io::BufReader, process::ExitCode};
use log::{debug, error, info};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Setup the verbose flag
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// Get what directory to check for
    #[arg(short, long)]
    directory: String,

    /// PID to track. If no PID is provided all PIDs are tracked.
    #[arg(short, long)]
    pid: Option<u16>,
}

fn main() -> ExitCode {
    // Parse the arguments
    let args = Args::parse();
    let directory = &args.directory;
    let pid = &args.pid;
    
    // Initialize the logger
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    info!("Starting laser video sorter");

    // Verify that the given path is valid
    if ! fs::metadata(directory).unwrap().is_dir() {
        eprintln!("Directory [{}] is not valid. Cannot continue.", directory);
        return ExitCode::from(1);
    }

    // Create a folder to store laser video in
    let laser_folder = &format!("{}/LASER_VIDS", directory);
    if let Err(e) = create_subdirectory(laser_folder) {
        return e
    };
    
    // Create a folder to store non-laser video in
    let non_laser_folder = &format!("{}/NON_LASER_VIDS", directory);
    if let Err(e) = create_subdirectory(non_laser_folder) {
        return e
    };

    // Get all of the `.ts` files in the given directory
    let ts_files = fs::read_dir(directory).unwrap().into_iter()
        .map(|dr| dr.unwrap())
        .filter(|f| f.file_name().into_string().unwrap().ends_with(".ts"))
        .collect_vec();
    
    debug!("Transport stream files found: {:?}", ts_files.iter().map(|f| f.file_name()).collect_vec());

    // Run through all of the transport stream files
    for video in ts_files.iter() {
        // Get the filename in string form.
        let filename = video.path().into_os_string();
        let filename = filename.to_str().unwrap();

        info!("Starting to read file {}", filename);
        
        // Set the laser video variable to false. Only move it to the laser folder if the laser is
        // seen firing.
        let mut is_laser_video = false;

        // Boilerplate to create a TSReader object
        let f = File::open(filename).expect("Couldn't open file");
        let buf_reader = BufReader::new(f);
        let mut reader = TSReader::new(filename, buf_reader).expect("Transport Stream file contains no SYNC bytes.");
        
        // Set the PID for the reader to track
        if let Some(pid) = pid {
            reader.add_tracked_pid(*pid);
        }

        loop {
            // Check to see if any of the KLV data indicates that the laser is on
            let payload = match reader.next_payload() {
                Ok(payload) => payload,
                Err(e) => panic!("Could not get payload due to error: {}", e),
            };
    
            // If `None` is returned then we have finished reading the file.
            let Some(payload) = payload else {
                debug!("Finished reading file [{}]", filename);
                break;
            };

            // Get the KLV data from the transport stream payload data.
            let Ok(klv) = KlvPacket::from_bytes(payload) else {
                continue;
            };

            // Check if the KLV packet even has a generic flag field
            let Some(generic_flag) =  klv.get(Tag::GenericFlagData) else {
                continue;
            };

            // Get the value of the generic flag
            let value = match generic_flag.value() {
                KlvValue::Uint8(value) => value,
                _ => panic!("Can't get value for generic flag")
            };

            debug!("Generic flag found: [{:#b}]", value);

            // Get the individual bits for the flag
            let bits: &BitSlice<_, Lsb0> = BitSlice::from_element(value);

            // The laser flag is the in the zero'th position
            if bits[0] {
                info!("Laser is on for file {}", filename);
                is_laser_video = true;
                break;
            }
        }

        let rename_directory = if is_laser_video {
            // If the laser is ever on move the video into the laser-video folder.
            laser_folder
        } else {
            // If no laser is ever seen move the video into the non-laser-video folder.
            non_laser_folder
        };

        match fs::rename(filename, format!("{}/{}", rename_directory, video.file_name().into_string().unwrap())) {
            Ok(_) => (),
            Err(e) => {
                error!("Cannot move video [{}] into the folder [{}] due to error: {}", filename, laser_folder, e)
            }
        };

    }

    return ExitCode::from(0);
}

fn create_subdirectory(dir: &str) -> Result<(), ExitCode> {
    // Create folder to store non-laser videos in
    debug!("Checking for non-laser video directory");
    if fs::metadata(dir).is_ok() {
        debug!("Directory [{}] already exists", dir) 
    } else {
        if ! fs::create_dir(dir).is_ok() {
            eprintln!("Cannot create directory [{}]", dir);
            return Err(ExitCode::from(2));
        }
        debug!("Created directory [{}]", dir);
    }

    Ok(())
}