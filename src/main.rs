#[allow(unused_imports)]
use std::ops::Deref;
use std::fs::File;
use std::io::Write;
use valve_rs::*;


fn main() -> Result<(), vbsp::BspError> {
    let mut args = std::env::args();
    let _ = args.next();
    println!("Loading File...");
    let bspname = args.next().expect("No demo file provided");
    let outputname = args.next().expect("No output file provided");
    let data = std::fs::read(bspname)?;

    let bsp = vbsp::Bsp::read(&data)?;
    let mut entdata: std::string::String = "{\n\"ents\": [\n".to_string();
    let mut entstring: std::string::String = "".to_string();
    println!("Converting...");
    // print_type_of(&bsp);
    bsp_to_json_str(&bsp);
    for ent in bsp.entities.iter() {
        entstring = format!("{:#?}",ent);

        entstring.remove(entstring.len()-3);

        entdata = format!("{}{}{}",entdata,entstring,",\n");

        // println!("{:#?}", ent);
    }
    entdata.pop();
    entdata.pop();
    
    // for prop in bsp.entities.iter() {
        // match prop.parse() {
            // Ok(prop) => println!("{:#?}", prop),
            // Err(e) => println!("Failed parsing {:#?}: {:#}", prop, e),
        // }
    // }

    // for prop in bsp.static_props() {
    //     dbg!(prop.deref());
    //     dbg!(prop.model());
    // }

    // let world_model = bsp.models().next().unwrap();
    // println!("{:#?}", &world_model);
    entdata = format!("{}{}",entdata,"\n]\n}");
    let mut f = File::create(outputname).expect("Unable to create file");
    f.write_all(entdata.as_bytes()).expect("Unable to write data");
    Ok(())
}