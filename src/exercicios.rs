use rand::{Rng, thread_rng, prelude::SliceRandom, seq::IteratorRandom};
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::fs::{read_dir, read_to_string};
use std::path::{Path, PathBuf};

fn template(dir: &Path) ->  Result<String, Error> {
    let mut rng = thread_rng();
    // Read directory, early return for possible error
    let files = read_dir(dir)?;
    // Choose a file at random
    let file = files.choose(&mut rng)
        // Choice is an Option
        // So change it into a Result
        .ok_or_else(
            || Error::new(ErrorKind::NotFound, "Directory is empty"))??;
    // Read into a string
    let text = read_to_string(file.path())?;
    // String is returned inside an OK() for the 
    // function return compatibility
    return Ok(text)
}

#[derive(Debug)]
pub struct ExMru {
    // Omiting some vars to make recalc func easier:
    // dx can be calculated by v*dt
    // xf can be calculated by xi+dx
    // tf can be calculated by ti+dt
    text: String,
    obj: String,
    xi: f32,
    dt: f32,
    ti: f32,
    v : f32
}

impl ExMru {
    // Creating a Exercice object, with the text and the
    // variables to be replaced
    pub fn new(dir: &Path, xmax: i32, dtmax: i32, vmax: i32) -> ExMru {
        let text = template(dir).unwrap();
        let obj = "test".to_string();
        let mut rng = thread_rng();
        let xi = rng.gen_range(-xmax..xmax) as f32;
        let dt: f32 = (rng.gen_range(1..dtmax*4) as f32)/4.0;
        let ti: f32 = (rng.gen_range(0..18*4) as f32)/4.0;
        let v = rng.gen_range(-vmax..vmax) as f32;
        return ExMru { 
            text,
            obj,
            xi,
            dt,
            ti,
            v 
        }
    }

    // Substitute the variables into the text
    // and return the resulting string
    pub fn subs(&self) -> String {
        let dx = self.v*self.dt;
        let replacements = HashMap::from([
            ("{obj}", self.obj.to_string()),
            ("{deltax}", dx.to_string()),
            ("{x0}", self.xi.to_string()),
            ("{x1}", (self.xi+dx).to_string()),
            ("{deltat}", self.dt.to_string()),
            ("{t0}", self.ti.to_string()),
            ("{t1}", (self.ti +self.dt).to_string()),
            ("{v}", self.v.to_string())
        ]);
        let mut out_text = self.text.clone().to_string();
        for (key, value) in replacements.into_iter() {
            out_text = out_text.replace(key, &value);
        };
        return out_text
    }

    pub fn recalc(&mut self, xmax: i32, dtmax: i32, vmax: i32) {
        let mut rng = thread_rng();
            self.obj= "new obj".to_string();
            self.xi = rng.gen_range(-xmax..xmax) as f32;
            self.dt = (rng.gen_range(1..dtmax*4) as f32)/4.0;
            self.ti = (rng.gen_range(0..18*4) as f32)/4.0;
            self.v = rng.gen_range(-vmax..vmax) as f32;
    }
}
