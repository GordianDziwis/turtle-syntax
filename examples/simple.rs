use rdf_types::dataset::IndexedBTreeDataset;
use rdf_types::generator::Blank;

use std::fs::File;
use std::io::Read;
use turtle_syntax::meta::InsertTurtle;

fn main() -> std::io::Result<()> {
	let mut dataset = IndexedBTreeDataset::new();
	dataset.insert_turtle_with(
		"<http://s> <http://p> 'xxx' .",
		&mut (),
		&mut (),
		Blank::new(),
	);
	print!("{:?}", dataset);

	let mut args = std::env::args();
	args.next();

	// let mut files = SimpleFiles::new();

	for filename in args {
		let mut file = File::open(&filename)?;

		let mut buffer = String::new();
		file.read_to_string(&mut buffer)?;
		// let file_id = files.add(filename.clone(), buffer);
		// let buffer = files.get(file_id).unwrap();

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
