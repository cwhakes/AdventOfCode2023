use super::*;

#[test]
fn test_answer1() {
	let buf = fs::read_to_string("../../input/08/test1").unwrap();
	let answer = get_answer1(&buf).to_string();

	let goal = "6".to_string();

	assert_eq!(answer, goal);
}

#[test]
fn test_answer2() {
	let buf = fs::read_to_string("../../input/08/test2").unwrap();
	let answer = get_answer2(&buf).to_string();

	let goal = "6".to_string();

	assert_eq!(answer, goal);
}
