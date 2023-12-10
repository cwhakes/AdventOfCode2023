use super::*;

#[test]
fn test_answer1() {
	let buf = fs::read_to_string("../../input/10/test").unwrap();
	let answer = get_answer1(&buf).to_string();

	let goal = "8".to_string();

	assert_eq!(answer, goal);
}

#[test]
fn test_answer2_2() {
	let buf = fs::read_to_string("../../input/10/test2").unwrap();
	let answer = get_answer2(&buf).to_string();

	let goal = "4".to_string();

	assert_eq!(answer, goal);
}

#[test]
fn test_answer2_3() {
	let buf = fs::read_to_string("../../input/10/test3").unwrap();
	let answer = get_answer2(&buf).to_string();

	let goal = "8".to_string();

	assert_eq!(answer, goal);
}

#[test]
fn test_answer2_4() {
	let buf = fs::read_to_string("../../input/10/test4").unwrap();
	let answer = get_answer2(&buf).to_string();

	let goal = "10".to_string();

	assert_eq!(answer, goal);
}
