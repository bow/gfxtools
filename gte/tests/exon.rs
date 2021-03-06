extern crate bio;
#[macro_use]
extern crate matches;
extern crate multimap;
extern crate gte;

use bio::utils::{self, Interval, Strand};
use multimap::MultiMap;

use gte::{EBuilder, ExonFeature, ExonFeatureKind, ModelError, Error};
use ModelError::{InvalidInterval, InvalidStrandChar};
use ExonFeatureKind::*;

fn make_feat(start: u64, end: u64, kind: ExonFeatureKind) -> ExonFeature {
    ExonFeature::new(Interval::new(start..end).unwrap(), kind)
}

#[test]
fn ebuilder_basic() {
    let mut attribs = MultiMap::new();
    attribs.insert("key1".to_owned(), "value1".to_owned());
    attribs.insert("key2".to_owned(), "value2".to_owned());
    let exonb = EBuilder::new("chrT", 10, 20)
        .strand(Strand::Forward)
        .id("ex1.1")
        .attributes(attribs)
        .feature(make_feat(10, 15, UTR5))
        .build();
    assert!(exonb.is_ok());
    let exon = exonb.unwrap();
    assert_eq!(exon.seq_name(), "chrT");
    assert_eq!(exon.strand(), &Strand::Forward);
    assert_eq!(exon.id(), Some("ex1.1"));
    assert_eq!(exon.attributes().get("key1"), Some(&"value1".to_owned()));
    assert_eq!(exon.attributes().get("key2"), Some(&"value2".to_owned()));
    assert_eq!(exon.attributes().len(), 2);
    assert_eq!(exon.features().to_vec(), vec![make_feat(10, 15, UTR5)]);
}

#[test]
fn ebuilder_alt1() {
    let exonb = EBuilder::new("chrO", 10, 10)
        .strand_char('-')
        .strand(Strand::Reverse)
        .attribute("name", "ex1")
        .build();
    assert!(exonb.is_ok());
    let exon = exonb.unwrap();
    assert_eq!(exon.seq_name(), "chrO");
    assert_eq!(exon.strand(), &Strand::Reverse);
    assert_eq!(exon.id(), None);
    assert_eq!(exon.attributes().get("name"), Some(&"ex1".to_owned()));
    assert_eq!(exon.attributes().len(), 1);
    assert_eq!(exon.features().len(), 0);
}

#[test]
fn ebuilder_interval_invalid() {
    let exonb = EBuilder::new("chrE", 20, 10).build();
    assert!(exonb.is_err());
    assert!(matches!(exonb.unwrap_err(),
                     Error::Model(InvalidInterval(utils::IntervalError::InvalidRange))));
}

#[test]
fn ebuilder_strand_unspecified() {
    let exonb = EBuilder::new("chrT", 20, 30).build();
    assert!(exonb.is_err());
    assert!(matches!(exonb.unwrap_err(), Error::Model(ModelError::UnspecifiedStrand)));
}

#[test]
fn ebuilder_strand_char_unexpected() {
    let exonb = EBuilder::new("chrE", 10, 20)
        .strand_char('w')
        .build();
    assert!(exonb.is_err());
    assert!(matches!(exonb.unwrap_err(),
                     Error::Model(InvalidStrandChar(utils::StrandError::InvalidChar(_)))));
}

#[test]
fn ebuilder_strand_char_conflicting() {
    let exonb = EBuilder::new("chrE", 10, 20)
        .strand_char('-')
        .strand(Strand::Reverse)
        .build();
    assert!(exonb.is_ok());
    let exon = exonb.unwrap();
    assert_eq!(exon.strand(), &Strand::Reverse);
}
