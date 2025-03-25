use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use rdf_types::dataset::{DatasetMut, IndexedBTreeDataset, PatternMatchingDataset};
use rdf_types::generator::Blank;
use rdf_types::interpretation::{
	BlankIdInterpretationMut, IdInterpretation, Interpret, IriInterpretation, IriInterpretationMut,
	LiteralInterpretation, LiteralInterpretationMut, TermInterpretation,
};

use rdf_types::vocabulary::{
	BlankIdVocabulary, BlankIdVocabularyMut, IriVocabulary, IriVocabularyMut, LiteralVocabulary,
	LiteralVocabularyMut,
};
use rdf_types::{Dataset, Generator, Interpretation, Literal, Quad, Term, Triple, Vocabulary};
use std::fs::File;
use std::io::Read;
use turtle_syntax::build::{strip, MetaTriple, RdfVocabulary};
use turtle_syntax::{parsing::Parse, Document};

impl<I, V, T> FromTurtle<I, V> for T
where
	I: Interpretation
		+ IriInterpretationMut<<V as IriVocabulary>::Iri>
		+ BlankIdInterpretationMut<<V as BlankIdVocabulary>::BlankId>
		+ LiteralInterpretationMut<Literal<<V as IriVocabulary>::Iri>>,
	V: RdfVocabulary + IriVocabularyMut + BlankIdVocabularyMut + LiteralVocabularyMut,
	V::Iri: Clone,
	V::BlankId: Clone,
	T: DatasetMut<Resource = I::Resource>,
{
}

trait FromTurtle<I = (), V = ()>
where
	I: Interpretation
		+ IriInterpretationMut<<V as IriVocabulary>::Iri>
		+ BlankIdInterpretationMut<<V as BlankIdVocabulary>::BlankId>
		+ LiteralInterpretationMut<Literal<<V as IriVocabulary>::Iri>>,
	V: RdfVocabulary + IriVocabularyMut + BlankIdVocabularyMut + LiteralVocabularyMut,
	V::Iri: Clone,
	V::BlankId: Clone,
	Self: DatasetMut<Resource = I::Resource>,
{
	fn insert_from_turtle(
		&mut self,
		turtle: &str,
		interpretation: &mut I,
		vocabulary: &mut V,
		mut generator: impl Generator<V>,
	) {
		match Document::parse_str(turtle, |span| span) {
			Ok(document) => {
				let mut triples = document
					.value()
					.build_triples_with(None, vocabulary, generator)
					.unwrap()
					.into_iter()
					.map(|triple| strip::<_, V>(triple))
					// .map(|t| Quad(t.0.interpret(interpretation), t.1, t.2, None))
					.map(|t| {
						Triple(
							t.0.interpret(interpretation),
							interpretation.interpret_iri(t.1),
							t.2.interpret(interpretation),
						)
					})
					.map(|t| t.into_quad(None));
				// IndexedBTreeDataset::from_iter(triples);
				self.insert(triples.next().unwrap());
				// todo!()
				// D::from_iter(triples)
			}
			Err(error_and_span) => {
				// let e = error_and_span.0;
				// let span = error_and_span.1;
				//
				// let diagnostic = Diagnostic::error()
				// 	.with_message(format!("parse error: {}", e))
				// 	.with_labels(vec![Label::primary(file_id, span)]);
				//
				// let writer = StandardStream::stderr(ColorChoice::Auto);
				// let config = codespan_reporting::term::Config::default();
				// codespan_reporting::term::emit(&mut writer.lock(), &config, &files, &diagnostic)
				// 	.unwrap();
			}
		}
		// todo!()
	}
}

fn main() -> std::io::Result<()> {
	let mut dataset = IndexedBTreeDataset::new();
	dataset.insert_from_turtle(
		"<http://s> <http://p> 'xxx' .",
		&mut (),
		&mut (),
		Blank::new(),
	);
	print!("{:?}", dataset);

	let mut args = std::env::args();
	args.next();

	let mut files = SimpleFiles::new();

	for filename in args {
		let mut file = File::open(&filename)?;

		let mut buffer = String::new();
		file.read_to_string(&mut buffer)?;
		let file_id = files.add(filename.clone(), buffer);
		let buffer = files.get(file_id).unwrap();

		// match Document::parse_str(buffer.source().as_str(), |span| span) {
		// 	Ok(doc) => {
		// 		doc.value()
		// 			.build_triples(None, Blank::new())
		// 			.unwrap()
		// 			.into_iter()
		// 			.map(MetaTriple::<_, ()>::strip)
		// 			.for_each(|triple| println!("{}", triple));
		// 	}
		// 	Err(error_and_span) => {
		// 		let e = error_and_span.0;
		// 		let span = error_and_span.1;
		//
		// 		let diagnostic = Diagnostic::error()
		// 			.with_message(format!("parse error: {}", e))
		// 			.with_labels(vec![Label::primary(file_id, span)]);
		//
		// 		let writer = StandardStream::stderr(ColorChoice::Auto);
		// 		let config = codespan_reporting::term::Config::default();
		// 		codespan_reporting::term::emit(&mut writer.lock(), &config, &files, &diagnostic)
		// 			.unwrap();
		// 	}
		// }
	}

	Ok(())
}
