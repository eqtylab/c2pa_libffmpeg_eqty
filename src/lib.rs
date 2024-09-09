use std::fs;
use c2pa::{create_signer, SigningAlg, Manifest};
use serde_json::{json, Value};
use std::ffi::{CStr};
use std::os::raw::c_char;
use std::io;
//use tempfile::NamedTempFile;
use serde::Deserialize;
use std::path::PathBuf;
use std::path::Path;
use std::env;
use uuid::Uuid;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt;

#[cfg(windows)]
use std::os::windows::fs::MetadataExt;

mod mpd_parse;


fn is_same_filesystem(old_path: &Path, new_path: &Path) -> io::Result<bool> {
    // Get the device ID of both paths using `metadata`
    let old_metadata = fs::metadata(old_path)?;
    let new_metadata = fs::metadata(new_path.parent().unwrap_or(new_path))?;

    #[cfg(unix)]
    {
        // Compare device IDs on Unix systems
        Ok(old_metadata.dev() == new_metadata.dev())
    }

    #[cfg(windows)]
    {
        // On Windows, you can compare the volume serial number instead
        Ok(old_metadata.volume_serial_number() == new_metadata.volume_serial_number())
    }
}

fn move_file(old_path: &Path, new_path: &Path) -> io::Result<()> {
    // Try to rename if on the same filesystem
    if is_same_filesystem(old_path, new_path)? {
        fs::rename(old_path, new_path)
    } else {
        // Otherwise, copy and remove the original file
        fs::copy(old_path, new_path)?;
        fs::remove_file(old_path)?;
        Ok(())
    }
}

//Helper Functions
fn get_file_name(path: &str) -> Option<String> {
    let path = Path::new(path);
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
}

fn get_extension(filename: &str) -> &str {
    Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())  // Convert OsStr to Option<&str>
        .unwrap_or("")  // Provide a default value if None
}

fn get_directory_path(path: &str) -> Option<String> {
    let path = Path::new(path);
    path.parent()
        .and_then(|parent| parent.to_str())
        .map(|parent| parent.to_string())
}


fn strings_to_pathbufs(strings: &[String], base_path: &Path) -> Vec<PathBuf> {
    strings.iter().map(|s| base_path.join(s)).collect()
}


#[derive(Debug, Default, Deserialize)]
struct ManifestDef {
    #[serde(flatten)]
    manifest: Manifest
}


#[no_mangle]
pub extern "C" fn c2pa_sign(
    c_manifest_file: *const c_char,
    c_input_file: *const c_char,
    c_output_file: *const c_char,
    c_cert_file: *const c_char,
    c_key_file: *const c_char) {

    let manifest_file = unsafe { CStr::from_ptr(c_manifest_file).to_str().unwrap_or("") };
    let input_file = unsafe { CStr::from_ptr(c_input_file).to_str().unwrap_or("") };
    let output_file = unsafe { CStr::from_ptr(c_output_file).to_str().unwrap_or("") };
    let cert_file = unsafe { CStr::from_ptr(c_cert_file).to_str().unwrap_or("") };
    let key_file = unsafe { CStr::from_ptr(c_key_file).to_str().unwrap_or("") };
        
    println!("Manifest file: {}", manifest_file);
    println!("Input file: {}", input_file);
    println!("Output file: {}", output_file);
    println!("Certificate file: {}", cert_file);
    println!("Key file: {}", key_file);



    let manifest_content = fs::read_to_string(manifest_file).expect("Error");
    let mut manifest_json: Value = serde_json::from_str(&manifest_content).expect("Error");

    // Define the new assertion to add
    let new_assertion = json!({
        "label": "c2pa.actions",
        "data": {
            "actions": [
                {
                    "action": "c2pa.transcoded",
                    "when": chrono::Utc::now().format("%Y-%m-%d").to_string()
                }
            ]
        }
    });

    // Add the new assertion directly to the JSON object

    if let Some(assertions) = manifest_json["assertions"].as_array_mut() {
        assertions.push(new_assertion);
    } else {
        // If `assertions` is not an array, initialize it as a new array with the new assertion
        manifest_json["assertions"] = json!([new_assertion]);
    }


    // Add the new claim generator info directly to the JSON object
    manifest_json["claim_generator"] = json!("ffmpeg_N-116759-g40dda881d6");
    
    let manifest_json_str = serde_json::to_string_pretty(&manifest_json).expect("Error");
//    println!("{}", manifest_json_str);

    // Create a ps256 signer using certs and key files
    let signer = create_signer::from_files(cert_file, key_file, SigningAlg::Ps256, None).expect("error");

    // Create a temporary file
/*    let temp_file = match NamedTempFile::new() {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create temporary file: {}", e);
            return;
        }
    };
    let temp_path = temp_file.path().to_owned();
    let  temp_file_name: &str = &temp_path.to_str().unwrap();
*/

    // read the manifest information
    let manifest_def: ManifestDef = serde_json::from_slice(manifest_json_str.as_bytes()).expect("error");
    
    
    let mut manifest = manifest_def.manifest;

    let ext = get_extension(input_file);
    //let ext = "mpd"; // replace this with the actual variable
    if ext == "mpd" {
        //Dash Signing
        // Base path of the MPD file
        let base_path = get_directory_path(input_file).unwrap();
        println!("Base Path: {:?}", &base_path);

        // Parse MPD file
        let results = mpd_parse::parse_mpd(input_file).expect ("ERROR");

//        let temporary_output_path: &str="/tmp/out";
        let mut temporary_output_path: PathBuf = env::temp_dir();
        let unique_path = format!("ffmpeg_c2pa-{}", Uuid::new_v4());
        temporary_output_path.push(unique_path);
        if !temporary_output_path.exists() {
            fs::create_dir_all(&temporary_output_path).expect("ERROR");
        }


        // Loop through streams
        for (initial_segment, fragments) in results {
            println!("Initialization Segment: {}", initial_segment);
            let initial_segment_full_path = Path::new(&base_path).join(&initial_segment);

            
            let input_fragment_pathbufs =strings_to_pathbufs(&fragments,&Path::new(&base_path));

            //DEBUG list fragment names
            for fragment in &fragments {
                println!("Fragment: {}", &fragment);
            }
            println!("Fragments: {:?}", &input_fragment_pathbufs);

            
            let temp_pathbuf = PathBuf::from(&temporary_output_path);
            println!("Temp Path: {:?}", &temp_pathbuf);

            println!("Full Path of initial Segment: {:?}", &initial_segment_full_path);


            manifest.embed_to_bmff_fragmented(&initial_segment_full_path, 
                                            &input_fragment_pathbufs, 
                                                &temp_pathbuf, 
                                                        signer.as_ref()
                                            ).expect("ERROR");

            //Replace signed files from temp folder
            for fragment in fragments {
                let old_path = Path::new(&temp_pathbuf).join(&fragment);
                let new_path = Path::new(&base_path).join(&fragment);
                println!("moving from {} to {}", old_path.display(), new_path.display());

                if let Err(e) = move_file(old_path.as_ref(), &new_path.as_ref()) {
                    eprintln!("Failed to move file: {}", e);
                }
            }
            let old_path_init = Path::new(&temp_pathbuf).join(&initial_segment);
            let new_path_init = Path::new(&base_path).join(&initial_segment);
            println!("moving from {} to {}", old_path_init.display(), new_path_init.display());

            if let Err(e) = move_file(old_path_init.as_ref(), &new_path_init.as_ref()) {
                eprintln!("Failed to move file: {}", e);
            } 

            if temporary_output_path.exists() {
                fs::remove_dir_all(&temporary_output_path).expect("Failed to delete directory");
            }

        }
    } else {
        let mut temporary_output_path: PathBuf = env::temp_dir();
        let unique_path = format!("ffmpeg_c2pa-{}.mp4", Uuid::new_v4());
        temporary_output_path.push(unique_path);

        // Convert PathBuf to &str
        let temp_file_name: &str =  temporary_output_path.to_str().expect("ERROR");
        /*  {
            Some(path_str) => path_str,
            None => {
                eprintln!("Failed to convert path to string.");
                return Err("");
            },
        };*/

        //let temp_file_name="/tmp/test.mp4";

        manifest
        .embed(&input_file, &temp_file_name, signer.as_ref()).expect("error");
        move_file(&temporary_output_path.as_ref(), output_file.as_ref()).expect("error");
    }

}