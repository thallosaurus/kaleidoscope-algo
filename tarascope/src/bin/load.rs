use serde_json::Value;
use tarascope::shader::KaleidoArgs;

fn main() {
    let data = r#"
    {
    "composite": {
        "composite_hue": 0.3856428861618042,
        "composite_lens_dispersion": -0.3,
        "composite_lens_distortion": -0.1,
        "composite_saturation": 1.172764778137207
    },
    "frames": {
        "_frames_max": 300,
        "_frames_start": 1
    },
    "id": "32a93579-7b4b-43ac-b43b-a8ccab30ac62",
    "output_directory": "/media",
    "pingpong": 1.3844809532165527,
    "repetition": 11,
    "rotation": 0.0,
    "scaling": 7.262486934661865,
    "texture": {
        "noise_detail": 1.1247533559799194,
        "noise_distortion": 5.857261657714844,
        "noise_lacunarity": 7.278568744659424,
        "noise_roughness": 0.6839921474456787,
        "noise_scale": 2.931267499923706
    },
    "texture_index": 4
}
    "#;
    
    let v: Value = serde_json::from_str(&String::from(data)).unwrap();
    let data = KaleidoArgs::from_json(v).unwrap();
    println!("{:?}", data);
}
