use nipper::Document;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


fn highlight(text: &str, portion: f32) -> String {
  text.split(" ").map(|word| {
    let place = (word.chars().count() as f32 * portion) as usize;
    let left = word.chars().take(place).collect::<String>();
    let right = word.chars().skip(place).collect::<String>();

    format!("<strong>{}</strong>{} ", left, right)
  }).collect::<String>()
}

fn main() -> std::io::Result<()> {
  let doc = File::open("in.epub")?;
  let buf_reader = BufReader::new(doc);
  let mut in_zip = zip::ZipArchive::new(buf_reader)?;

  let out = File::create("out.epub")?;
  let mut out_zip = zip::ZipWriter::new(out);

  let options = zip::write::FileOptions::default()
    .compression_method(zip::CompressionMethod::Stored);

  for i in 0..in_zip.len() {
    let mut file = in_zip.by_index(i)?;
    match file.name().get(..11) {
      Some("OEBPS/Text/") => {
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let parsed = Document::from(&content);

        parsed.select("p").iter().for_each(|mut node| {
          node.set_html(highlight(&node.text(), 0.5));
        });

        out_zip.start_file(file.name(), options)?;
        out_zip.write(parsed.html().as_bytes())?;
      }, 
      _ => {out_zip.raw_copy_file(file)?;}
    }
  }

  out_zip.finish()?;
  Ok(())
}
