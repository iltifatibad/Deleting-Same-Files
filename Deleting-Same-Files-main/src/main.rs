use std::{fs::{self, File}, io::Read};

fn main() {
    let mut datavec: Vec<(String, Vec<std::path::PathBuf>)> = Vec::new();
    let paths2 = fs::read_dir("../test/").unwrap();
      for path in paths2{
        println!("{:?}",path);
        if let Ok(path_entry) = path {
          let path = path_entry.path();
          let mut file = File::open(&path).expect("Unable to open");
          let mut data = String::new();
          file.read_to_string(&mut data).expect("Failed to read file");
          let mut founded = false;
          for (data_in_vec, path_in_vec) in &mut datavec{
            if data_in_vec == &mut data{
              path_in_vec.push(path.clone());
              founded = true;
            }
          }
          if !founded{
            datavec.push((data,vec![path]));
          }
          
          
          println!("{:?}",datavec);
            
  
          }
        }
      
        for (_, path_vec) in datavec{
          if path_vec.len() > 1{
            for (i, path) in path_vec.iter().enumerate(){
              if i < path_vec.len()-1{
                fs::remove_file(path);
              }
            }
          }
      } 

}