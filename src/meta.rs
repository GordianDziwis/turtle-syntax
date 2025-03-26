use rdf_types::dataset::DatasetMut;
use rdf_types::interpretation::{
	BlankIdInterpretationMut, Interpret, IriInterpretationMut, LiteralInterpretationMut,
};
use rdf_types::vocabulary::{
	BlankIdVocabulary, BlankIdVocabularyMut, IriVocabulary, IriVocabularyMut, LiteralVocabularyMut,
};
use rdf_types::{Generator, Id, Literal, Quad, Term, Triple};

use crate::build::{BuildError, MetaTriple};
use crate::parsing::{Parse, ParseError};
use crate::Document;

#[derive(Debug, thiserror::Error)]
pub enum Error<E, M> {
	#[error(transparent)]
	Parse(ParseError<E, M>),

	#[error(transparent)]
	Builder(BuildError<E>),
}

pub trait MetaTuple<A, B> {
	fn map<C, F>(self, f: F) -> (C, B)
	where
		F: FnOnce(A) -> C;

	fn value(&self) -> &A;

	fn into_value(self) -> A;

	fn metadata(&self) -> &B;

	fn borrow(&self) -> (&A, &B);
}

impl<A, B> MetaTuple<A, B> for (A, B) {
	fn map<C, F>(self, f: F) -> (C, B)
	where
		F: FnOnce(A) -> C,
	{
		(f(self.0), self.1)
	}

	fn value(&self) -> &A {
		&self.0
	}

	fn into_value(self) -> A {
		self.0
	}

	fn metadata(&self) -> &B {
		&self.1
	}

	fn borrow(&self) -> (&A, &B) {
		(&self.0, &self.1)
	}
}

pub type RdfTriple<V> = Triple<RdfId<V>, <V as IriVocabulary>::Iri, RdfTerm<V>>;

pub type RdfId<V> = Id<<V as IriVocabulary>::Iri, <V as BlankIdVocabulary>::BlankId>;

pub type RdfTerm<V> = Term<RdfId<V>, Literal<<V as IriVocabulary>::Iri>>;

pub fn strip<M, V>(triple: MetaTriple<M, V>) -> RdfTriple<V>
where
	V: IriVocabularyMut + BlankIdVocabularyMut,
{
	let Triple((s, _), (p, _), (o, _)) = triple.into_value();
	let o = match o {
		Term::Id(id) => Term::Id(id),
		Term::Literal(literal) => Term::Literal(literal.into()),
	};
	Triple(s, p, o)
}

pub trait FromTurtle {
	fn from_turtle_with<I, V>(
		turtle: &str,
		interpretation: &mut I,
		vocabulary: &mut V,
		generator: impl Generator<V>,
	) -> Self
	where
		I: IriInterpretationMut<<V as IriVocabulary>::Iri>
			+ BlankIdInterpretationMut<<V as BlankIdVocabulary>::BlankId>
			+ LiteralInterpretationMut<Literal<<V as IriVocabulary>::Iri>>,
		V: IriVocabularyMut + BlankIdVocabularyMut + LiteralVocabularyMut,
		V::Iri: Clone,
		V::BlankId: Clone,
		Self: FromIterator<Quad<I::Resource>>,
	{
		let quads = to_triples_with(turtle, vocabulary, interpretation, generator)
			.into_iter()
			.map(|triple| triple.into_quad(None));
		Self::from_iter(quads)
	}

	fn from_turtle(turtle: &str, generator: impl Generator<()>) -> Self
	where
		Self: FromIterator<Quad>,
	{
		let quads = to_triples(turtle, generator)
			.into_iter()
			.map(|triple| triple.into_quad(None));
		Self::from_iter(quads)
	}
}

impl<T: DatasetMut> FromTurtle for T {}

pub trait InsertTurtle {
	fn insert_turtle_with<I, V>(
		&mut self,
		turtle: &str,
		interpretation: &mut I,
		vocabulary: &mut V,
		generator: impl Generator<V>,
	) where
		Self: DatasetMut<Resource = I::Resource>,
		I: IriInterpretationMut<<V as IriVocabulary>::Iri>
			+ BlankIdInterpretationMut<<V as BlankIdVocabulary>::BlankId>
			+ LiteralInterpretationMut<Literal<<V as IriVocabulary>::Iri>>,
		V: IriVocabularyMut + BlankIdVocabularyMut + LiteralVocabularyMut,
		V::Iri: Clone,
		V::BlankId: Clone,
	{
		to_triples_with(turtle, vocabulary, interpretation, generator)
			.into_iter()
			.map(|triple| triple.into_quad(None))
			.for_each(|quad| self.insert(quad));
	}

	fn insert_turtle(&mut self, turtle: &str, generator: impl Generator<()>)
	where
		Self: DatasetMut<Resource = Term>,
	{
		self.insert_turtle_with::<(), ()>(turtle, &mut (), &mut (), generator)
	}
}

impl<T: DatasetMut> InsertTurtle for T {}

pub fn to_triples_with<I, V>(
	turtle: &str,
	vocabulary: &mut V,
	interpretation: &mut I,
	generator: impl Generator<V>,
) -> Vec<Triple<I::Resource>>
where
	I: IriInterpretationMut<<V as IriVocabulary>::Iri>
		+ BlankIdInterpretationMut<<V as BlankIdVocabulary>::BlankId>
		+ LiteralInterpretationMut<Literal<<V as IriVocabulary>::Iri>>,
	V: IriVocabularyMut + BlankIdVocabularyMut,
	V::Iri: Clone,
	V::BlankId: Clone,
{
	Document::parse_str(turtle, |span| span)
		.unwrap()
		.value()
		.build_meta_triples_with(None, vocabulary, generator)
		.unwrap()
		.into_iter()
		.map(|triple| strip::<_, V>(triple))
		.map(|t| {
			Triple(
				t.0.interpret(interpretation),
				interpretation.interpret_iri(t.1),
				t.2.interpret(interpretation),
			)
		})
		.collect()
}

pub fn to_triples(turtle: &str, generator: impl Generator<()>) -> Vec<Triple> {
	to_triples_with::<(), ()>(turtle, &mut (), &mut (), generator)
}
