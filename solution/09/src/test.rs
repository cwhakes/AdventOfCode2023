use super::*;

#[test]
fn test_answer1() {
	let buf = fs::read_to_string("../../input/09/test").unwrap();
	let answer = get_answer1(&buf).to_string();

	let goal = "114".to_string();

	assert_eq!(answer, goal);
}

#[test]
fn test_answer2() {
	let buf = fs::read_to_string("../../input/09/test").unwrap();
	let answer = get_answer2(&buf).to_string();

	let goal = "2".to_string();

	assert_eq!(answer, goal);
}
