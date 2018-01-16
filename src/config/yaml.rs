/* Read configuration from YAML file */

mod config
pub fn readconfig(input: &str) -> String {
    let mut config = File::open(input)expect("config.yaml not found.");
    println!("Reading config from file: {:?}", config);

    let mut configcontents = String::new();
    config.read_to_string(&mut configcontents)
        .expect("Something went wrong while reading the file.");

    let docs = YamlLoader::load_from_str(&configcontents).unwrap();
    let doc = &docs[0];

    println!("{:?}", docs); // Debug support

    let mut out_str = String::new(); {
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap();
    }
    println!("{}", out_str); // Debug support
    println!("{:?}", doc["Token"].as_str()); // Debug support

    let token = doc["Token"].as_str()
        .expect("Expected a token in config file.");
}
