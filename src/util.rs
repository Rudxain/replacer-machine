/// Returns `true` if `pat` matches at least 1 occurrence in `s`.
///
/// If `pat.is_empty()`, inserts `repl` before each element.
/// This behavior might change, such that it also appends `repl`.
///
/// See [`str::replace`].
fn replace_all<T: Clone + Eq>(s: &mut Vec<T>, pat: &[T], repl: &[T]) -> bool {
	let mut is_match = false;
	if pat.is_empty() {
		is_match = true;
		let mut out = Vec::with_capacity(s.len() + repl.len() * s.len());
		for x in &*s {
			out.extend_from_slice(repl);
			out.push(x.clone());
		}
		*s = out;
		return is_match;
	}
	if pat.len() > s.len() {
		return is_match;
	}
	// let caller decide
	let mut out = Vec::with_capacity(s.capacity());
	let mut i = 0;
	while let Some(ss) = s.get(i..(pat.len() + i)) {
		if ss == pat {
			is_match = true;
			out.extend_from_slice(repl);
			i += pat.len();
		} else {
			out.push(s[i].clone());
			#[expect(clippy::arithmetic_side_effects)]
			{
				i += 1;
			}
		}
	}
	out.extend_from_slice(&s[i..]);
	*s = out;
	is_match
}

#[derive(Debug, Copy, Clone)]
pub enum ReplKind {
	All,
	First,
	Last,
}

#[derive(Debug, Copy, Clone)]
pub struct Op<'a, T> {
	/// print memory to stdout
	dump: bool,
	repl_ty: ReplKind,
	/// map entry
	rule: (&'a [T], &'a [T]),
}

#[derive(Debug, Clone)]
pub struct Machine<'o, 'i, T> {
	/// instruction pointer
	i: usize,
	/// program instructions
	ops: &'o [Op<'i, T>],
	/// "turing tape"
	s: Vec<T>,
}
impl<'o, 'i, T> Machine<'o, 'i, T> {
	pub const fn new(ops: &'o [Op<'i, T>], s: Vec<T>) -> Self {
		Self { i: 0, ops, s }
	}
}

impl<T: Clone + Eq> Machine<'_, '_, T> {
	/// poor person's lending Iterator
	pub fn next(&mut self) -> Option<&[T]> {
		loop {
			let (pat, repl) = self.ops.get(self.i)?;
			if replace_all(&mut self.s, pat, repl) {
				self.i = 0;
				return Some(&self.s);
			}
			#[expect(
				clippy::arithmetic_side_effects,
				reason = "`get` makes overflow unreachable"
			)]
			{
				self.i += 1;
			}
		}
	}
}
