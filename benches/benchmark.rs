// Copyright (C) 2021 hjiayz
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use quickjs_regex::regex;
use regex::Regex;

const TEXT: &'static str = "bcdeeeeeeeeeeeeee";

fn regex_test(c: &mut Criterion) {
    c.bench_function("regex_test", |b| {
        b.iter(|| Regex::new(r"^bc(d|e)*$").unwrap().is_match(black_box(TEXT)))
    });
}

fn quickjs_regex_test(c: &mut Criterion) {
    c.bench_function("quickjs_regex_test", |b| {
        b.iter(|| uregex!(r"^bc(d|e)*$").test(black_box(TEXT)))
    });
}

criterion_group!(benches, regex_test, quickjs_regex_test);
criterion_main!(benches);
