use regex::Regex;
use roxmltree::{Document, Node};
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref NON_WORD: Regex = Regex::new(r"\W+").unwrap();
}

/// Parse GB XML files into genome and its peptides.
#[derive(Debug, StructOpt)]
pub struct Config {
    /// Path to input GB XML file
    #[structopt(parse(from_os_str))]
    pub xml: PathBuf,

    /// Path to genome output
    #[structopt(parse(from_os_str))]
    pub genomes: PathBuf,

    /// Path to peptide output
    #[structopt(parse(from_os_str))]
    pub peptides: PathBuf,
}

fn get_descendant_tag_text<'a>(node: &'a Node, tag_name: &str) -> &'a str {
    node.descendants()
        .find(|n| n.has_tag_name(tag_name))
        .unwrap()
        .text()
        .unwrap()
}

fn get_qualifier_value<'a>(qualifiers: &'a [Node], name: &str) -> &'a str {
    let result = qualifiers
        .iter()
        .find(|node| get_descendant_tag_text(&node, "GBQualifier_name") == name)
        .expect("couldn't find the qualifier you were looking for");

    get_descendant_tag_text(&result, "GBQualifier_value")
}

fn is_cds_feature(node: &Node) -> bool {
    node.has_tag_name("GBFeature")
        && node
            .descendants()
            .find(|child| child.has_tag_name("GBFeature_key"))
            .expect("no 'GBFeature_key' node")
            .text()
            .expect("no text for 'GBFeature_key' node")
            == "CDS"
}

fn parse_genomes(xml: &Document) -> Vec<String> {
    // There can be many if the eutils query was run like this: https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=nuccore&id=MF417837,MF417838,MF417839&rettype=gb&retmode=xml
    xml.descendants()
        // There should only be a single GBSet, so just take the first one with `find`.
        .find(|node| node.has_tag_name("GBSet"))
        .expect("couldn't find the 'GBSet' node")
        .descendants()
        // There can be many GBSeq's however.
        .filter(|node| node.has_tag_name("GBSeq"))
        .map(|gbseq| {
            let acc = get_descendant_tag_text(&gbseq, "GBSeq_accession-version");

            let organism = get_descendant_tag_text(&gbseq, "GBSeq_organism");

            // We have stuff like: 'Blah; Silly; Thing; apple pie'...need to join them on __
            let taxonomy = get_descendant_tag_text(&gbseq, "GBSeq_taxonomy")
                .split("; ")
                .collect::<Vec<&str>>()
                .join("__");

            let sequence = get_descendant_tag_text(&gbseq, "GBSeq_sequence");

            format!(
                ">{} ~~ {} ~~ {}\n{}",
                acc,
                NON_WORD.replace_all(organism, "_"),
                NON_WORD.replace_all(&taxonomy, "_"),
                // Sequences should only have ascii codes
                sequence.to_ascii_uppercase()
            )
        })
        .collect()
}

fn parse_peptides(xml: &Document) -> Vec<String> {
    let mut peptides = Vec::new();

    let gbseqs = xml
        .descendants()
        // There should only be a single GBSet, so just take the first one with `find`.
        .find(|node| node.has_tag_name("GBSet"))
        .expect("couldn't find the 'GBSet' node")
        .descendants()
        // There can be many GBSeq's however.
        .filter(|node| node.has_tag_name("GBSeq"));

    for gbseq in gbseqs {
        let genome_acc = get_descendant_tag_text(&gbseq, "GBSeq_accession-version");

        let cds_features = gbseq
            .descendants()
            .find(|node| node.has_tag_name("GBSeq_feature-table"))
            .expect("no 'GBSeq_feature-table' node")
            .descendants()
            // There can be many GBFeature nodes
            .filter(|node| is_cds_feature(node));

        for cds_feature in cds_features {
            let qualifiers = cds_feature
                .descendants()
                .find(|node| node.has_tag_name("GBFeature_quals"))
                .expect("no 'GBFeature_quals' node")
                .descendants()
                // There are many GBQualifier nodes
                .filter(|node| node.has_tag_name("GBQualifier"))
                // Just collect it here so it's easier to pass around.
                .collect::<Vec<Node>>();

            let product = get_qualifier_value(&qualifiers, "product");
            let acc = get_qualifier_value(&qualifiers, "protein_id");
            let translation = get_qualifier_value(&qualifiers, "translation");

            peptides.push(format!(
                ">{} ~~ {} ~~ {}\n{}",
                acc,
                genome_acc,
                NON_WORD.replace_all(product, "_"),
                translation.to_ascii_uppercase()
            ))
        }
    }

    peptides
}

fn parse_xml(xml: String) -> (Vec<String>, Vec<String>) {
    let xml = Document::parse(&xml).expect("couldn't parse XML file");

    let genomes = parse_genomes(&xml);
    let peptides = parse_peptides(&xml);

    (genomes.clone(), peptides)
}

fn write_strings(strings: &[String], path: &Path) {
    let file = File::create(path).expect("couldn't create the file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "{}", strings.join("\n")).expect("couldn't write to file");
}

pub fn run(config: Config) {
    eprintln!("DEBUG -- config: {:?}", &config);

    let xml = fs::read_to_string(&config.xml).expect("couldn't read XML file");

    let (genomes, peptides) = parse_xml(xml);

    write_strings(&genomes, &config.genomes);
    write_strings(&peptides, &config.peptides);
}
