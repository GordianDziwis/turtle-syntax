use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use std::fs::File;
use std::io::Read;
use rdf_types::generator::Blank;
use rdf_types::Object;
use turtle_syntax::{parsing::Parse, Document};

fn main() -> std::io::Result<()> {
	let mut args = std::env::args();
	args.next();

	let mut files = SimpleFiles::new();

	for filename in args {
		let mut file = File::open(&filename)?;

		let mut buffer = String::new();
		file.read_to_string(&mut buffer)?;
		let file_id = files.add(filename.clone(), buffer);
		let buffer = files.get(file_id).unwrap();

		match Document::parse_str(buffer.source().as_str(), |span| span) {
			Ok(doc) => {
				let document = doc.0;

				let triples = document.build_triples(None, Blank::new()).unwrap();

				for triple in triples {
					let object = match triple.0.2.0 {
						Object::Id(id) => id.as_str().to_string(),
						Object::Literal(literal) => literal.value.0.as_str().to_string(),
					};

					println!("{} {} {}", triple.0.0.0.as_str(), triple.0.1.0.as_str(), object);
				}
			}
			Err(error_and_span) => {
				let e = error_and_span.0;
				let span = error_and_span.1;

				let diagnostic = Diagnostic::error()
					.with_message(format!("parse error: {}", e))
					.with_labels(vec![Label::primary(file_id, span)]);

				let writer = StandardStream::stderr(ColorChoice::Auto);
				let config = codespan_reporting::term::Config::default();
				codespan_reporting::term::emit(&mut writer.lock(), &config, &files, &diagnostic)
					.unwrap();
			}
		}
	}

	Ok(())
}
