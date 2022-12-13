use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListOrValue {
    List(Vec<Box<ListOrValue>>),
    Value(u32),
}

impl ListOrValue {
    pub fn parse(s: &[char]) -> Result<Self, String> {
        if s.len() < 2 || s[0] != '[' || s[s.len() - 1] != ']' {
            return Err("root element is not a list".to_string());
        }

        let (_, list) = Self::parse_list(1, s)?;
        Ok(Self::List(list))
    }

    fn parse_list(mut from: usize, s: &[char]) -> Result<(usize, Vec<Box<Self>>), String> {
        let begin = from;
        let mut list = Vec::new();

        'outer: loop {
            if from >= s.len() {
                break;
            }
            match s[from] {
                '[' => {
                    let (index, item) = Self::parse_list(from + 1, s)?;
                    from = index + 1;
                    list.push(Box::new(Self::List(item)));
                }
                ',' => {
                    from += 1;
                }
                ']' => break 'outer,
                c if c.is_ascii_digit() => {
                    let (index, item) = Self::parse_value(from, s)?;
                    from = index;
                    list.push(Box::new(Self::Value(item)));
                }
                e => return Err(format!("unexpected char '{}'", e)),
            };
        }
        Ok((from, list))
    }

    fn parse_value(mut from: usize, s: &[char]) -> Result<(usize, u32), String> {
        let num: String = s[from..]
            .into_iter()
            .take_while(|c| c.is_ascii_digit())
            .collect();

        let end = from + num.len();
        let value = num
            .parse()
            .map_err(|e| format!("failed to parse '{}' as value, err: '{}'", num, e))?;
        Ok((end, value))
    }

    pub fn compare(left: &Self, right: &Self) -> Ordering {
        use ListOrValue::*;
        // println!("comparing:");
        // println!("left:  {}", left);
        // println!("right: {}", right);
        match (left, right) {
            (List(l), List(r)) => {
                let mut l_iter = l.iter();
                let mut r_iter = r.iter();
                loop {
                    let l = l_iter.next();
                    let r = r_iter.next();
                    match match (l, r) {
                        (None, None) => return Ordering::Equal,
                        (None, Some(_)) => return Ordering::Less,
                        (Some(_), None) => return Ordering::Greater,
                        (Some(l), Some(r)) => Self::compare(l, r),
                    } {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => (),
                    }
                }
            }
            (l @ List(_), Value(r)) => Self::compare(l, &List(vec![Box::new(Value(*r))])),
            (Value(l), r @ List(_)) => Self::compare(&List(vec![Box::new(Value(*l))]), r),
            (Value(l), Value(r)) => l.cmp(r),
        }
    }
}

impl Display for ListOrValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        match self {
            ListOrValue::List(l) => write!(
                f,
                "[{}]",
                l.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            ListOrValue::Value(v) => write!(f, "{}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ListOrValue::*;

    fn chars(s: &'static str) -> Vec<char> {
        s.chars().collect()
    }

    #[test]
    fn parse_list_or_value_1() {
        assert_eq!(
            ListOrValue::parse(&chars("[123,1234]")),
            Ok(List(vec![Box::new(Value(123)), Box::new(Value(1234))]))
        );
    }

    #[test]
    fn parse_list_or_value_2() {
        let actual = ListOrValue::parse(&chars("[[1],[1,2,[3]]]")).unwrap();
        let expected = List(vec![
            Box::new(List(vec![Box::new(Value(1))])),
            Box::new(List(vec![
                Box::new(Value(1)),
                Box::new(Value(2)),
                Box::new(List(vec![Box::new(Value(3))])),
            ])),
        ]);

        println!("actual: {}", actual);
        println!("expected: {}", expected);
        assert_eq!(actual, expected);
    }
}
