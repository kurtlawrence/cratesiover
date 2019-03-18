extern crate cratesiover;

fn main() {
	cratesiover::output("cratesiover", "2.1.0").unwrap();

	cratesiover::output_with_term(
		"cratesiover",
		"2.1.0",
		&cratesiover::DefaultTerminal::new().unwrap(),
	);

	cratesiover::output_to_writer("cratesiover", "2.1.0", &mut std::io::stderr()).unwrap();
}
