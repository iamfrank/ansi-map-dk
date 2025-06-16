use reqwest;
use dotenvy::dotenv;
use std::env;
use std::error::Error;
use std::io::Cursor;
use geotiff::GeoTiff;
use geo_types::Coord;

fn read_terrain_geotiff(url: String) -> GeoTiff {
  
  let response_bytes = match reqwest::blocking::get(url) {
    Ok(file) => file.bytes(),
    Err(_) => Err("Problem fetching geotiff.").unwrap(),
  };

  let cursor = match response_bytes {
    Ok(bytes) => Cursor::new(bytes),
    Err(_) => Err("Problem getting byte stream.").unwrap(),
  };
  
  let geotiff = match GeoTiff::read(cursor) {
    Ok(reader) => reader,
    Err(_) => Err("Problem reading GeoTiff.").unwrap(),
  };

  geotiff
}

fn draw_terrain(gt: GeoTiff) {

  let width = gt.raster_width as i32;
  let height = gt.raster_height as i32;
  let step_x = gt.model_extent().width() / width as f64;
  let step_y = gt.model_extent().height() / height as f64;
  let min_x = gt.model_extent().min().x;
  let max_y = gt.model_extent().max().y;

  for h in 1..height {
    let imgy: f64 = max_y - (&step_y * h as f64);
    for w in 1..width {
      let imgx: f64 = min_x + (&step_x * w as f64);
      let c: Coord = Coord { x: imgx, y: imgy};
      match gt.get_value_at::<f32>(&c, 0) {
        Some(value) => {
          //print!("{}", value)
          print_value(value)
        },
        None => eprintln!("GeoTIFF had none value"),
      }
    }
    println!();
  }
}

fn print_value(value: f32) {
  match value {
    1.0..10.0 => {
      print!("\x1b[48;5;35m  \x1b[0m");
    }
    10.0..20.0 => {
      print!("\x1b[48;5;34m  \x1b[0m");
    }
    20.0..30.0 => {
      print!("\x1b[48;5;71m  \x1b[0m");
    }
    30.0..40.0 => {
      print!("\x1b[48;5;70m  \x1b[0m");
    }
    40.0..60.0 => {
      print!("\x1b[48;5;106m  \x1b[0m");
    }
    60.0..80.0 => {
      print!("\x1b[48;5;142m  \x1b[0m");
    }
    80.0..100.0 => {
      print!("\x1b[48;5;178m  \x1b[0m");
    }
    100.0..200.0 => {
      print!("\x1b[48;5;214m  \x1b[0m");
    }
    _ => {
      print!("\x1b[48;5;0m  \x1b[0m");
    }
  }
}

fn main() -> Result<(), Box<dyn Error>> {

  dotenv().ok(); // Loads .env

  let base_url = env::var("API_DHM_WCS_BASEURL").unwrap();
  let token_a = env::var("API_DHM_TOKENA").unwrap();
  let token_b = env::var("API_DHM_TOKENB").unwrap();
  let height_width = env::var("ELEVATION_MODEL_SIZE").unwrap();

  let dhm_request_url = base_url 
  + "?SERVICE=WCS&COVERAGE=dhm_terraen&RESPONSE_CRS=epsg:25832&CRS=epsg:25832&FORMAT=GTiff&REQUEST=GetCoverage&VERSION=1.0.0"
  + "&username=" 
  + &token_a
  + "&password=" 
  + &token_b
  + "&height="
  + &height_width 
  + "&width="
  + &height_width
  + "&bbox=430000,6040000,900000,6413000";

  draw_terrain(read_terrain_geotiff(dhm_request_url));

  Ok(())
}
