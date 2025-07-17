use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-7-16
*/

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct GeneLength {
    annotype: String,
    idvec: String,
    startvec: usize,
    endvec: usize,
}

pub async fn threadedlengthhuman(generate: &str) -> Result<String, Box<dyn Error>> {
    if generate == "yes" {
        let _ = Command::new("wget").
         arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.chr_patch_hapl_scaff.annotation.gtf.gz")
         .output()
            .expect("command failed");
        let _ = Command::new("wget")
            .arg("gencode.v48.primary_assembly.annotation.gtf.gz")
            .output()
            .expect("Command failed");
        let _ = Command::new("gunzip")
            .arg("*")
            .output()
            .expect("Command failed");

        let fileall =
            File::open("gencode.v48.primary_assembly.annotation.gtf").expect("file not present");
        let filecontent = BufReader::new(fileall);
        let mut genevec: Vec<String> = Vec::new();
        let mut id: Vec<String> = Vec::new();
        let mut start: Vec<usize> = Vec::new();
        let mut end: Vec<usize> = Vec::new();
        for i in filecontent.lines() {
            let line = i.expect("line not present");
            if !line.starts_with("#") {
                let linevec: Vec<_> = line
                    .split("\t")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                for _i in 0..linevec.len() {
                    if linevec[2].clone().to_string() == "gene" {
                        genevec.push(linevec[2].clone());
                        start.push(linevec[3].parse::<usize>().unwrap());
                        end.push(linevec[4].parse::<usize>().unwrap());
                        id.push(
                            linevec[8].split(";").collect::<Vec<_>>()[0]
                                .to_string()
                                .replace(" ", "-")
                                .split("-")
                                .collect::<Vec<_>>()[1]
                                .to_string(),
                        )
                    }
                }
            }
        }
        let mut genevecrwrite: Vec<GeneLength> = Vec::new();
        for i in 0..genevec.len() {
            genevecrwrite.push(GeneLength {
                annotype: genevec[i].clone(),
                idvec: id[i].clone(),
                startvec: start[i],
                endvec: end[i],
            })
        }

        let mut filewrite = File::create("genelength.txt").expect("File not present");
        for i in genevecrwrite.iter() {
            writeln!(
                filewrite,
                "{}\t{}\t{}\t{}",
                i.annotype, i.idvec, i.startvec, i.endvec
            )
            .expect("no file present");
        }

        let _ = Command::new("rm")
            .arg("-rf")
            .arg("gencode.vM37.primary_assembly.annotation.gtf.gz")
            .output()
            .expect("file not found");

        Ok("The gene length have been written".to_string())
    } else {
        Ok("no option supplied".to_string())
    }
}
