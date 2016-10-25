// Hope this source will be useful

// Regex abstract syntax tree

#[derive(Debug, PartialEq)]
enum ReAst {
	Seq(Box<ReAst>, Box<ReAst>),
	Star(Box<ReAst>),
	Pipe(Box<ReAst>, Box<ReAst>),
	Ch(char),
	// The empty tree
	Empty
}

use ReAst::*;

macro_rules! debug {
	($e:expr) => {{
		match std::env::var("RUST_BACKTRACE") {
			Ok(_) => println!("{}", $e),
			_ => {},
		}
	}};
}

fn indent(r: i32) {
	match std::env::var("RUST_BACKTRACE") {
		Ok(_) => {for _ in 0..r { print!("|  "); }},
		_ => {},
	}
}

impl ReAst {
	fn seq(l: ReAst, r: ReAst) -> ReAst {
		Seq(Box::new(l), Box::new(r))
	}

	fn star(l: ReAst) -> ReAst {
		Star(Box::new(l))
	}

	fn pipe(l: ReAst, r: ReAst) -> ReAst {
		Pipe(Box::new(l), Box::new(r))
	}

// puts the rightmost node inside a Star node
fn add_star(ast: ReAst) -> ReAst {
	match ast {
		// base
		ReAst::Ch(c) => return ReAst::star(ReAst::Ch(c)),
			// induction
			ReAst::Seq(l, r) => ReAst::seq(*l, ReAst::add_star(*r)),
			ReAst::Pipe(l, r) => ReAst::pipe(*l, ReAst::add_star(*r)),
			ReAst::Star(l) => ReAst::star(ReAst::add_star(*l)),
			_ => panic!("Unreachable state"),
		}
	}

	fn new(re: &str) -> ReAst {
		let mut buf = re.chars();
		return ReAst::parse(&mut buf, 0);
	}

	fn parse(buf: &mut std::str::Chars, r: i32) -> ReAst {
		let ch = buf.next();
		indent(r);
		debug!(format!("parse() ch={:?}", ch));
		let result = match ch {
			Some('*') => panic!("Unexpected char: `*`"),
			Some('|') => panic!("Unexpected char: `|`"),
			Some(')') => panic!("Unexpected char: `)`"),
			Some('(') => ReAst::parse_next(ReAst::parse_inner(buf, 1), buf, 1),
			Some(c) => ReAst::parse_next(ReAst::Ch(c), buf, 1),
			None => ReAst::Empty,
		};
		indent(r);
		debug!(format!("=> {:?}", result));
		return result;
	}

	fn parse_next(l: ReAst, buf: &mut std::str::Chars, r: i32) -> ReAst {
		let ch = buf.next();
		indent(r);
		debug!(format!("parse_next() ch={:?} l={:?}", ch, l));
		let result = match ch {
			Some(')') => panic!("Unexpected char: `)`"),
			Some('(') =>
				ReAst::parse_next(ReAst::seq(l, ReAst::parse_inner(buf, r+1)), buf, r+1),
			Some('*') => ReAst::parse_next(ReAst::add_star(l), buf, r+1),
			// this should really call another state
			Some('|') => ReAst::pipe(l, ReAst::parse(buf, r+1)),
			Some(c) => ReAst::parse_next(ReAst::seq(l, Ch(c)), buf, r+1),
			None => l,
		};
		indent(r);
		debug!(format!("=> {:?}", result));
		return result;
	}

	fn parse_inner(buf: &mut std::str::Chars, r: i32) -> ReAst {
		let ch = buf.next();
		indent(r);
		debug!(format!("parse_inner() ch={:?}", ch));
		let result = match ch {
			None => panic!("Unexpected end of string"),
			Some('*') => panic!("Unexpected char: `*`"),
			Some('|') => panic!("Unexpected char: `|`"),
			Some(')') => panic!("Unexpected char: `)`"),
			Some('(') => ReAst::parse_next_inner(ReAst::parse_inner(buf, r+1), buf, r+1),
			Some(c) => ReAst::parse_next_inner(ReAst::Ch(c), buf, r+1),
		};
		indent(r);
		debug!(format!("=> {:?}", result));
		return result;
	}

	fn parse_next_inner(l: ReAst, buf: &mut std::str::Chars, r: i32) -> ReAst {
		let ch = buf.next();
		indent(r);
		debug!(format!("parse_next_inner() ch={:?} l={:?}", ch, l));
		let result = match ch {
			Some(')') => l,
			Some('(') => ReAst::parse_next_inner(ReAst::seq(l, ReAst::parse_inner(buf, r+1)), buf, r+1),
			Some('*') => ReAst::parse_next_inner(ReAst::add_star(l), buf, r+1),
			Some('|') => ReAst::pipe(l, ReAst::parse_inner(buf, r+1)),
			Some(c) => ReAst::parse_next_inner(ReAst::seq(l, Ch(c)), buf, r+1),
			None => panic!("Unexpected end of string"),
		};
		indent(r);
		debug!(format!("=> {:?}", result));
		return result;
	}
}

/*
// Nondeterministic finite state automa
struct NFANode {
	friends: Vec<(Option<char>, NFANode)>,
	exit: bool,
}
impl NFANode {
	fn new(ast: ReAst) -> NFANode {
		NFANode { friends: Vec::new(), exit: true }
	}
	fn match_text(&self, text: &str) -> bool {
		false
	}
	fn match_length(&self, text: &str) -> Option<i32> {
		None
	}
}
*/

fn test_re(re: &str, ast: ReAst) {
	println!("testing \"{}\"...", re);
	let result = ReAst::new(re);
	if result != ast {
		println!("Test failed!");
		println!("Ast: {:?}", ast);
		println!("Result: {:?}", result);
		match std::env::var("RUST_BACKTRACE") {
			Ok(_) => panic!("Aborting program"),
			_ => println!("Test suite will continue execution"),
		};
	}
}

// This is the main function
fn main() {
	std::env::set_var("RUST_BACKTRACE", "1");

	let re = "a*a";
	let ast = ReAst::new(re);
	println!("Ast dump: {:?}", ast);

	test_re("", ReAst::Empty);
	test_re("a", ReAst::Ch('a'));
	test_re("a*", ReAst::star(ReAst::Ch('a')));
	test_re("ab", ReAst::seq(Ch('a'), Ch('b')));
	test_re("a*b*", ReAst::seq(ReAst::star(Ch('a')), ReAst::star(Ch('b'))));
	test_re("a*a", ReAst::seq(ReAst::star(Ch('a')), Ch('a')));
	test_re("(a)", ReAst::Ch('a'));
	test_re("(a)*", ReAst::star(ReAst::Ch('a')));
	test_re("(a)b", ReAst::seq(Ch('a'), Ch('b')));
	test_re("((a))", ReAst::Ch('a'));
	test_re("(((a)))", ReAst::Ch('a'));
	test_re("((((a))))", ReAst::Ch('a'));
	test_re("((((a))))b", ReAst::seq(Ch('a'), Ch('b')));
	test_re("((((a*))))b", ReAst::seq(ReAst::star(Ch('a')), Ch('b')));
	test_re("a(b)", ReAst::seq(Ch('a'), Ch('b')));
	test_re("a*(b)", ReAst::seq(ReAst::star(Ch('a')), Ch('b')));
	test_re("a(b)c", ReAst::seq(ReAst::seq(Ch('a'), Ch('b')), Ch('c')));
	test_re("a(b(c))", ReAst::seq(Ch('a'), ReAst::seq(Ch('b'), Ch('c'))));
	test_re("a|(b(c))", ReAst::pipe(Ch('a'), ReAst::seq(Ch('b'), Ch('c'))));
	test_re("a|b|c", ReAst::pipe(Ch('a'), ReAst::pipe(Ch('b'), Ch('c'))));

	/*
	let nfa = NFANode::new(ast);
	nfa.match_length("a");
	*/
	// Print text to the console
	println!("Good. My work here is done");
}
