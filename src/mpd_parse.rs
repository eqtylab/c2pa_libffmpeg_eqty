use serde::Deserialize;
use quick_xml::de::from_str;
use std::fs::File;
use std::io::Read;


//MPD Structure
#[derive(Debug, Deserialize)]
pub struct MPD {
    Period: Period,
}

#[derive(Debug, Deserialize)]

struct Period {
    AdaptationSet: Vec<AdaptationSet>,
}

#[derive(Debug, Deserialize)]
struct AdaptationSet {
    Representation:  Option<Vec<Representation>>,
}

#[derive(Debug, Deserialize)]

struct Representation {
    id: String,
    SegmentTemplate: SegmentTemplate,
}

#[derive(Debug, Deserialize)]

struct SegmentTemplate {
    timescale: Option<u64>,
    initialization: String,
    media: String,
    start_number: Option<u64>,
    SegmentTimeline: Option<SegmentTimeline>,
}

#[derive(Debug, Deserialize)]

struct SegmentTimeline {
    S: Vec<S>,
}

#[derive(Debug, Deserialize)]
struct S {
    t: Option<u64>,
    d: u64,
    r: Option<i64>,
}

//Parse MPD
pub fn parse_mpd(file_path: &str) -> Result<Vec<(String, Vec<String>)>, Box<dyn std::error::Error>> {
    // Read the MPD file
    let mut file = File::open(file_path).expect("error");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("error");

//    println!("content file: {}", content);

    // Parse the MPD XML content
    let mpd: MPD = from_str(&content).expect("error");

    let mut results = Vec::new();

    // Iterate through each adaptation set and representation
    for adaptation_set in &mpd.Period.AdaptationSet {
        if let Some(representations) = &adaptation_set.Representation {
            for representation in representations {
                let timescale = representation.SegmentTemplate.timescale.unwrap_or(1);
                let initialization_segment = representation
                    .SegmentTemplate
                    .initialization
                    .replace("$RepresentationID$", &representation.id);

                let media_pattern = &representation.SegmentTemplate.media;
                let mut start_number = representation.SegmentTemplate.start_number.unwrap_or(1);
                
                let mut segments = Vec::new();
                if let Some(segment_timeline) = &representation.SegmentTemplate.SegmentTimeline {
                    let mut current_time = 0;
                    for s in &segment_timeline.S {
                        let duration = s.d;
                        let repeat_count = s.r.unwrap_or(0);

                        for _ in 0..=repeat_count {
                            let segment_file = media_pattern
                                .replace("$RepresentationID$", &representation.id)
                                .replace("$Number%05d$", &format!("{:05}", start_number));
                            segments.push(segment_file);
                            start_number += 1;
                            current_time += duration;
                        }

                        if let Some(t) = s.t {
                            current_time = t;
                        }
                    }
                }

                // Push the initialization segment and associated segments to the results
                results.push((initialization_segment, segments));
            }
        }
    }

    Ok(results)
}